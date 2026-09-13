# 🔁 The wgpu frame loop after a selection — the renderer was QUARANTINING ITSELF (2026-09-13)

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane **wgpu-frame-loop-after-selection** (Opus).
Target: `http://127.0.0.1:6118/?plugin=generation3d` (editor) and `…&role=viewer`, the coordinator's wgpu serve.

Repo MCP was down all session (`repo -32602 invalid initialize params`; `semio CONNECTION_CLOSED`) — this
lane opened, closed and reopened NO ticket, did not touch `📓️status.md` / `🎫️ticket.json`, ran no
git-state-modifying command, started or stopped no dev server, and deleted nothing under `🗑️generated`
that it did not create.

---

## 1. TL;DR

`📓️wgpu-spawn-job-effect-2026-09-13.md` §7.1 left this as "the frame loop stops a few applied selections
in, with no panic, no fault, no console error of any kind, and the main thread fully alive".

It was not stopping. **It was being killed.** The renderer publishes a frame fault, `tick()` answers
`quarantined: true, faultCode: "frame-credits"`, and `BrowserFrameTransport.quarantine()` closes the
surface for good. The fault text, read off the wire (`🗑️generated/wgpu-frame-loop/wire-3`):

```
runtime interaction checkout at frame-deferred outlived its bounded step after 241 apply opportunities
```

It was invisible from the console because the ONLY report was a DOM banner painted over the canvas
(`🗑️generated/wgpu-frame-loop/wire-3/shot-final.png` — "Surface: quarantined · input accepted: no").

| | |
|---|---|
| **Owning defect** | `InteractionCheckoutLedger::admit` aged a LIVE checkout by counting blocked apply opportunities against a fixed `INTERACTION_CHECKOUT_CREDITS = 240`. An apply opportunity is produced by the frame loop and by arriving input — the checkout's VICTIMS, never its holder — so the count measures a rate, not health. |
| **The rate, measured** | 240 opportunities in ≤ 1 986 ms on 6118 (≈ 121/s), i.e. the credit was ≈ 2 s of wall clock. |
| **What it killed** | ONE post-selection settle. `framework.panel.inspection` alone renders for **2 138 ms** inside a single `frame-deferred` checkout, and the shell re-renders all six surfaces per applied selection. Every applied selection therefore overran the credit; the page died 1–4 gestures in. |
| **Fix** | The verdict is now whether an **owner** exists — a reservation in flight, or a ready completion that RESTORES the interaction state. A live owner defers for as long as it lives; no owner is a defect on the first blocked opportunity, with no timer to tune. |
| **6118, editor, 20 gestures** | frame batches **12 → 9 653**, frame builds admitted **2 → 1 206**, render sweeps **15 → 40**, published actions **0 → 31**, quarantines **0**, `frame fault recorded` **0**. |
| **6118, viewer, 20 gestures** | batches **28 → 9 429**, admits **2 → 632**, sweeps **14 → 35**, actions **0 → 28**, quarantines **0**. |
| **Marquee** | `interactionSelect … method:"rectangle"` now APPLIES — §7.2's unresolved item (§6.3). |

---

## 2. Reproduction, and the hop the two known facts straddle

The brief's two facts — the worker's log goes silent, and the page's main thread keeps ticking rAF —
straddle exactly one hop: the `BrowserFrameTransport` ⇄ frame Worker message wire. `T/🐍️wgpu-frame-loop-probe.mjs`
wraps `Worker` with `page.addInitScript` before the boot module loads and records every
`{kind:"batch"}` posted up and every `{kind:"frame"|"wake"|"fault"}` posted down, with its
`sequence`/`generation`, so a stop is attributable:

* last record an UP batch with no matching DOWN frame → the WORKER stopped answering;
* last record a DOWN frame with no UP batch after a pointer move → the UI side parked.

Neither happened. Every batch was answered, and the LAST answer carried the kill:

```
{t:52943, dir:"up",   kind:"batch", sequence:1531, generation:27}
{t:52946, dir:"down", kind:"frame", sequence:1531, generation:27, requestFrame:true,
                      quarantined:true, faultCode:"frame-credits",
                      faultDetail:"runtime interaction checkout at frame-deferred outlived its
                                   bounded step after 241 apply opportunities"}
{t:52946, dir:"up",   kind:"close"}
```

Four pre-fix runs, every one with the same signature and the same last console line
(`wgpu-shell render leave surface=framework.panel.history`):

| run | gestures before the stop | last frame |
|---|---|---|
| `🗑️generated/wgpu-frame-loop/repro-1` | 4 (hover, click, shift-click, empty click) | quarantined |
| `…/wire-1` | 1 (hover) | `frame-credits` |
| `…/wire-2` | 2 (hover, click) | `frame-credits` |
| `…/wire-3` | 2 | `frame-credits`, detail above |

**Why a probe never saw it**: the banner is DOM, `renderFault` wrote nothing to the console, and the
frame worker keeps draining its already-running async settle chain for another ~700 ms after the
`close` — which is why the last line is always the final panel's `render leave`, several hundred
milliseconds AFTER the surface was already dead.

---

## 3. Root cause, file : line

```
🧊️renderer/🦀️.rs:10682   runtime.checkout.admit(head_requires_interaction, available)
📮️runtime-mailbox-core/🦀️.rs   INTERACTION_CHECKOUT_CREDITS = 240
                               admit(): opportunities += 1; if opportunities <= CREDITS { Deferred } else { Stale }
🧊️renderer/🦀️.rs:10693   take_stale_notice() → self.0.frame_fault  (the runtime's UNRECOVERABLE frame lane)
🌐️browser-worker/🦀️.rs:246  take_frame_fault() → fault_code "frame-credits", quarantined: true
🚚️browser-frame-transport/🟦️.ts:716  quarantined → quarantine() → status "quarantined", worker close
```

Three separate things are wrong on that path, and all three are fixed:

**❶ The age is not a measure of the holder.** `admit` counts one opportunity per `apply_pending_step`
that found the queue head blocked. That head is a `DispatchEvents` completion — a pointer move the user
made *while* the shell was busy. So the counter runs at the frame loop's spin rate times the input rate.
Measured on 6118 (`🗑️generated/wgpu-frame-loop/wire-3/console.txt`): the blocked episode opens at
`72906` with `for 1 opportunities` and the last `frame build admitted` before the quarantine is at
`74892` — **240 opportunities in ≤ 1 986 ms, ≈ 121/s**. The module's own doc asserted the opposite
("at most one per frame-build advance"); the browser drive loop advances a build many times per tick.

**❷ Two seconds is less than one settle.** The `frame-deferred` checkout holds the interaction state
for the WHOLE of `flush_deferred_actions` → `settle_ui_chain` → `refresh_ui`, which re-renders all six
surfaces through the guest on every applied selection. From the same run:

```
[DEBUG] boot-phase shell-boot:render:framework.panel.inspection 2138 ms
[DEBUG] wgpu-shell render leave surface=framework.panel.inspection 2149 ms
```

One panel body out-lives the whole credit on its own. The credit did not name a leak; it named a
selection.

**❸ A liveness diagnostic was wired into the fatal lane.** `frame_fault` is the runtime's "this frame
cannot be produced" slot (lost ownership, stale witness, exhausted credits) and the browser answers it
by closing the surface permanently. A slow-but-live checkout is not that — and the proof is that the
settle *finished normally* ~700 ms after the kill, in every run.

---

## 4. The fix — the ledger's verdict is ownership, not age

### 4.1 `📮️runtime-mailbox-core/🦀️.rs`

```rust
pub(crate) struct Completion<T> { …, pub(crate) restores_interaction: bool, … }

impl<T, const CAPACITY: usize> BoundedCompletionQueue<T, CAPACITY> {
    pub(crate) fn interaction_owner_outstanding(&self) -> bool {
        self.in_flight > 0 || self.ready.iter().any(|completion| completion.restores_interaction)
    }
}

pub(crate) enum InteractionCheckoutStep { Admitted, Deferred, Abandoned }

pub(crate) fn admit(&mut self, head_requires_interaction: bool, interaction_available: bool, owner_outstanding: bool) -> InteractionCheckoutStep {
    if interaction_available || !head_requires_interaction { return InteractionCheckoutStep::Admitted }
    self.opportunities = self.opportunities.saturating_add(1);
    if owner_outstanding { return InteractionCheckoutStep::Deferred }
    self.abandoned = true;
    InteractionCheckoutStep::Abandoned
}
```

`INTERACTION_CHECKOUT_CREDITS` is **deleted**, not widened — a bigger number would still have been a
wall-clock proxy, and the whole point is that no number is the right one. `Stale` becomes `Abandoned`
and `take_stale_notice` becomes `take_abandoned_notice`; the AGE survives as a diagnostic only
(`apply_pending_step blocked: … for N opportunities` still prints).

**Why this predicate is exact.** Every checkout is paired with a reservation taken by the same
synchronous block that took it (`start_dispatch`, `start_frame_deferred` — `check_out_interaction` then
`reserve_interaction_future`, with `return_interaction` on every refusal path), and the spawned turn
always `finish`es that reservation with a completion that CARRIES the state. So:

* reservation in flight → the turn is still running, however long;
* reservation finished → the completion is READY and carries the state, until it is applied;
* applied → `return_interaction` → `check_in`.

The only state with neither is a genuine leak, and it is visible on the first blocked opportunity.

### 4.2 `🧊️renderer/🦀️.rs` — one owner for the rule

`restores_interaction` is never spelled at a call site. It is read off the apply:

```rust
fn restores_interaction(&self) -> bool {
    match self {
        Self::ResumeDispatch { interaction, .. } | Self::ResumeFrameDeferred { interaction, .. } => interaction.is_some(),
        #[cfg(not(target_arch = "wasm32"))] Self::RestoreInteraction(interaction) => interaction.is_some(),
        _ => false,
    }
}
```

so `returned_completion(revision, apply)` and `RuntimeMailboxInner::completion(...)` are the only two
constructors and no call site can disagree with the ledger. This matters for one real arm: the native
abandoned-maintenance resume is `ResumeFrameDeferred { interaction: None, .. }` — it carries nothing and
therefore owns nothing. `apply_pending_step` ORs in `runtime.pending_frame_maintenance_refusal.is_some()`
under `cfg(not(wasm32))`, because on native the owner may be a refusal this runtime still holds, which
the mailbox alone cannot see.

### 4.3 A dead surface is now readable

Two smaller defects on the same path, both of which cost earlier lanes days:

* `🚀️browser-boot/🟦️.ts` `renderFault` now also writes `console.error("wgpu renderer fault: …")`. The
  banner was the only report, and a probe, a CI run and a headless browser all read the console.
* `🚚️browser-frame-transport/🟦️.ts` mapped `frame-credits` onto `"worker-step-overrun"` — a frame the
  runtime refused to build was reported as a slow step. It now has its own `"worker-frame-failed"`.

---

## 5. Laws — all run in the FOREGROUND

### 5.1 The shared fixture

`🧑‍🎨engine/🧫️fixtures/🎮️wgpu-runtime-mailbox-admission/🔣️.json`, **10 rows** (was 8), driven by BOTH
halves. `"credits": 240` is gone; `staleNotices` → `abandonedNotices`; `enqueue`/`finish` gain
`restoresInteraction`. Three rows are new and are this lane's law:

| row | what it pins |
|---|---|
| `a-live-owner-keeps-a-long-checkout-out-of-the-fault-lane` | **2 000** blocked opportunities with a live owner are all `deferred`, and publish NO notice. |
| `a-checkout-with-no-owner-to-return-it-is-abandoned-at-once` | no reservation, no restoring completion → `abandoned` on opportunity 1, reported once per episode, cleared by check-in. |
| `a-ready-completion-that-restores-the-state-is-an-outstanding-owner` | the owner is not always a reservation; taking the restoring completion out of the queue is what leaves the checkout with nobody. |

### 5.2 The RED half, kept runnable

`T/🐍️checkout-credits-counterproof.mjs` replays the new fixture against the RETIRED rule (credits = 240)
and reports every row that rule breaks:

```
RED a-live-owner-keeps-a-long-checkout-out-of-the-fault-lane
    step 3 (apply): the retired rule says "stale", the law says "deferred"
    notices [{"site":"frame-deferred","opportunities":241}] vs []
RED a-checkout-with-no-owner-to-return-it-is-abandoned-at-once
RED a-ready-completion-that-restores-the-state-is-an-outstanding-owner
rows=10 broken-by-the-retired-credit-rule=3
```

`opportunities: 241` is byte-identical to the detail 6118 quarantined on. The script exits non-zero if
the fixture ever stops pinning the defect.

### 5.3 Green

| lane | result |
|---|---|
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- admission_laws runtime_mailbox` | **7 passed** (3 fixture-driven + 2 new ledger laws + 3 queue laws, incl. the new owner-predicate law) |
| `bunx vitest … 🧪️tests/🎮️wgpu-runtime-mailbox-admission/🟦️.ts` (independent TS re-derivation, `vitest` oracle) | **17 passed** (was 13; +3 new fixture rows, +2 new hand-written laws) |
| `nx run @semio-tech/framework-renderer-wgpu:test-browser-worker` | **green** (the transport suite, whose fault-code mapping this lane changed) |
| `nx run @semio-tech/framework-renderer-wgpu:check-frame-worker` / `check-browser-worker` | **green** — the generated bundles are fresh |
| `nx run @semio-tech/framework-renderer-wgpu:wasm` | `Successfully ran target`, dist `22:31`; the new fault string is IN the shipped wasm (`has no owner left to return it` ×1, `outlived its bounded step` ×0) |
| `nx run @semio-tech/framework-renderer-wgpu:generate-browser-boot` | green, bundle `22:33`, carries `wgpu renderer fault:` and `worker-frame-failed` |
| `cargo nextest run -p semio-framework-os-renderer-wgpu --lib --no-fail-fast` | 559 tests: **537 passed, 22 failed** — all 22 pre-date this lane, see §7.4 |

The two hand-written Rust laws state the rule directly rather than through the fixture:
`a_live_owner_never_ages_into_a_fault` drives **100 000** blocked opportunities with an owner and
asserts `Deferred` every time and no notice (a credit ceiling fails it at 241), and
`a_missing_owner_is_a_defect_on_the_first_blocked_opportunity` pins the other half. The TS twin carries
the same two plus `counts a ready completion that restores the state as an outstanding owner`.

---

## 6. 6118 — the runtime proof

`T/🐍️wgpu-frame-loop-probe.mjs`, boot + settle, then **20 consecutive gestures** cycling
hover → click → shift-click → empty click → marquee, each followed by a 6 s settle with a pointer nudge
per second (the nudges matter: input arriving DURING a settle is exactly what fed the retired counter).

### 6.1 Editor (`🗑️generated/wgpu-frame-loop/fixed-editor`)

| after | batches | frame builds admitted | render sweeps | published actions | selection lane | quarantines |
|---|---|---|---|---|---|---|
| boot | 12 | 2 | 15 | 0 | — | 0 |
| g1 hover | 1 129 | 99 | 16 | 1 | 172 | 0 |
| g5 marquee | 2 868 | 382 | 22 | 7 | 172 | 0 |
| g10 marquee | 5 036 | 677 | 28 | 15 | 172 | 0 |
| g15 marquee | 7 283 | 939 | 34 | 23 | 172 | 0 |
| **g20 marquee** | **9 590** | **1 203** | **40** | **31** | 172 | **0** |
| final | 9 653 | 1 206 | 40 | 31 | — | 0 |

Every one of the 20 gestures advanced the loop; nothing plateaued. `frame fault recorded` **0**,
`present_step faulted` **0**, `panicked` **0**, `wgpu renderer fault` **0**. Screenshot
`fixed-editor/shot-final.png`: the live workbench — node graph on the left, the shaded hexagonal
column on the right, **no fault banner** (against `wire-3/shot-final.png`, which is the banner).

**The history panel updates.** `renderSurface surface=framework.panel.history` **42 times**, its document
revision reaching `rev=25` — the history document changes on every applied selection, which is what
made the per-surface re-render (and therefore the long checkout) reachable at all.

**Selections apply**: `selected":true` **139** occurrences, the selection lane moving
`172 → 185` (pick) `→ 201` (additive) `→ 188` (empty) with each gesture, and the gumball's three
translucent draws appearing on the frame after an applied select
(`draws=1 translucent=3 instances=4`).

### 6.2 Viewer (`🗑️generated/wgpu-frame-loop/fixed-viewer`)

`?plugin=generation3d&role=viewer`, same ladder: batches **28 → 9 429**, admits **2 → 632**, sweeps
**14 → 35**, actions **0 → 28**, history renders 37 to `rev=21`, quarantines **0**, faults **0**.
In the viewer the marquee's selection also PERSISTS (lane `168 → 185` after each of the four marquees).

### 6.3 The marquee applies

`📓️wgpu-spawn-job-effect-2026-09-13.md` §7.2 could not observe the marquee's apply. All four
`interactionSelect … method:"rectangle"` of the editor run now do, within 730–830 ms of the gesture:

```
69711 world3d interaction … active=MarqueePick …
69742 world3d interaction … active=MarqueePublish[page=0 stage=8 targets=47b]
69742 frame input action … action=interactionSelect … "method":"rectangle"
70475 world3d surface=procedural-preview … draws=1 translucent=3 instances=4 …   ← gumball, i.e. applied
70540 wgpu-bridge spawn-job done instance=1 job=962 kind=framework.reserved.tool input=194B steps=2 outcome=ok bytes=178
```

It was never a marquee defect: the earlier probe's marquee step ran after the surface was already dead.
The apply is then PRUNED again by the pre-existing granularity drift — see §7.2.

---

## 7. What is NOT claimed

1. **The frame-build spin is untouched.** Between an input and a completed build the shell admits a
   build roughly every 50 ms and immediately supersedes it (`frame build admitted generation=N` with N
   incrementing on an idle shell, 20/s). That spin is what made the retired counter run at 121/s. It is
   now harmless, and it is still there — it belongs to the frame-build / deferred-action lane, not the
   mailbox.
2. **The `object` vs `node` granularity drift is untouched** (`📓️wgpu-spawn-job-effect` §7.3). In the
   EDITOR, all four marquee selects and the first two pick selects are followed by
   `interaction selection lost reason=validate-state-pruned
   dispatched=graph=object:extrude@solid;vortex=object:extrude@solid validated=graph=node:extrude@solid`,
   and the selection lane returns to its empty size. The marquee's APPLY is proven (§6.3); its
   PERSISTENCE in the editor is not. In the viewer role it persists.
3. **`Abandoned` was not induced at runtime**, only in the laws. Nothing on 6118 abandons a checkout, so
   the fault lane's new predicate is proven by the Rust and TypeScript laws and the fixture, not by a
   browser run. That is the intended shape: the old rule fired on healthy behaviour, and the new one is
   expected never to fire unless something really leaks.
4. **The 2 138 ms panel render is untouched.** The settle is still slow enough that input queues behind
   it for seconds — it no longer kills the surface, and `flush()`/`inFlight` still deliver every queued
   batch afterwards, but "a selection costs a full six-surface guest re-render" is a performance defect
   this lane only measured.
5. **React's `🔌️PluginRuntime` was not touched**, and the native shell path was changed only by the
   `pending_frame_maintenance_refusal` term in §4.2 — which has no native runtime proof here, only the
   native-compiled laws.
6. **The 22 `cargo nextest` failures are pre-existing.** They sit in twelve modules this lane never
   touched (`deadlines`, `dock`, `engine_canvas`, `scenes`, `shell`, `kernel_runtime`,
   `async_boundary_tests`, …) and include three SIGABRTs. Checked, not assumed:
   `presenter_ack_retirement_source_mutations_are_denied` fails on three source-scan COUNTS
   (`runtime.presentation_witness_for(self.generation.0)` = 2 vs the demanded 1,
   `let expected = self.presentation_authority.current();` = 1 vs 2,
   `self.presentation_authority.mark_scene_changed();` = 1 vs 2) that are **identical in `git show HEAD`**;
   and `runtime_publication_tests::runtime_single_enqueue_reader_cannot_observe_completion_without_its_scene_invalidation`
   fails deterministically (5/5 runs) against `enqueue_runtime_completion`, whose body is byte-identical
   to `HEAD`. All six mailbox tests pass.
7. **`🧪️tests/🧩️package-integration/🟦️.ts` cannot be run through `bunx vitest`** — six cases fail with
   `ReferenceError: Bun is not defined`. That is the runner, not the code; its nx target is
   `test-preview-generated`.

---

## 8. Fix-forward on peers' work

None needed — nothing blocked this lane, and no peer hunk was reverted. A peer added
`[DEBUG] frame fault recorded: …` to `apply_pending_step` mid-session; that line was kept and its
wording updated to the new verdict. The renderer wasm was rebuilt by peers three times during the
session (21:18 → 21:44 → 21:51); the pre-fix measurements in §2 are against the 21:18 build and the
§6 proof against this lane's own 22:31 build.

## 9. Files

| File | What |
|---|---|
| `…/🎯️targets/🧊️wgpu/📮️runtime-mailbox-core/🦀️.rs` | `Completion::restores_interaction`, `interaction_owner_outstanding`, `InteractionCheckoutStep::Abandoned`, `admit(.., owner_outstanding)`, `take_abandoned_notice`; `INTERACTION_CHECKOUT_CREDITS` deleted |
| `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` | `RuntimeApply::restores_interaction`, `returned_completion`, the owner term in `apply_pending_step`, the retyped fault text |
| `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` | `worker-frame-failed`, the `frame-credits` mapping |
| `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` | `renderFault` writes the fault to the console |
| `🧑‍🎨engine/🧫️fixtures/🎮️wgpu-runtime-mailbox-admission/🔣️.json` | **the language-agnostic law** — 10 rows, 3 new, `restoresInteraction`, `abandonedNotices` |
| `🧑‍🎨engine/🧪️tests/🎮️wgpu-runtime-mailbox-admission/🦀️.rs` | replays the new vocabulary; 2 new hand-written ledger laws |
| `🧑‍🎨engine/🧪️tests/🎮️wgpu-runtime-mailbox-admission/🟦️.ts` | the independent TS re-derivation, 17 cases |
| `🧑‍🎨engine/🧪️tests/🔬️wgpu-runtime-mailbox-core-unit/🦀️.rs` | the owner-predicate law at the queue's own level |
| `🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs`, `🖱️ui/…/📥️enqueue/🧪️tests/📥️enqueue/🦀️.rs` | one field each on their `Completion` literals |
| `T/🐍️wgpu-frame-loop-probe.mjs` | the `Worker`-wire probe and the 20-gesture ladder |
| `T/🐍️checkout-credits-counterproof.mjs` | the RED half of the law, kept runnable |

Evidence: `T/🗑️generated/wgpu-frame-loop/{repro-1,wire-1,wire-2,wire-3,fixed-editor,fixed-viewer}`
(`console.txt`, `results.json`, `wire.json`, screenshots).

## 10. Follow-ups

1. **The settle is the real cost.** One applied selection = six guest surface renders, one of which is
   2 138 ms. Until that is incremental, input queues behind a selection for seconds.
2. **The frame-build spin** (§7.1): an idle shell admits and supersedes ~20 builds a second.
3. **The `object`/`node` granularity drift** (§7.2) still prunes the editor's marquee and its first two
   picks — it is the last thing between "the marquee applies" and "the marquee selects".
4. **`enqueue_runtime_completion` publishes before it invalidates** — §7.6's deterministic failure is a
   real ordering defect with a fixture already demanding the opposite, and it has no owner.
