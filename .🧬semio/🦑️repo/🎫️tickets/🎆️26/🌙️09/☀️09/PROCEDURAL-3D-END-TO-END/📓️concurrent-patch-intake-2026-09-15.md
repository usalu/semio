# 🧺 One owning intake per instance — the busy surface cell that swallowed Abort and Finalize

Lane `concurrent-patch-intake`, 2026-09-15, React door of 🧊️generation3d on **:6024** and the peer's puzzle 3d
door on **:6013** (read-only). Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.

---

## 0. The headline

> **It was never concurrency. One guest turn legitimately carried TWO patches for the SAME surface, and the
> host admitted every patch of a page to its acknowledgement token before acknowledging any of them — so the
> second patch met the first one's still-open wire and receipt outbox in `OwnedUiInstance.beginPatch`, threw,
> and the throw skipped the page's whole close. That surface's cell stayed busy for the rest of the
> instance's life: every later refresh, every `toolRunAbort` and every `toolRunFinalize` was rejected with
> `plugin-ui.intake-rejected:intake:Foreign or busy instance surface owner`, and the Tool runs panel never
> left `Running`.**

Fixed on both halves, because each must be correct on its own:

| half | change |
|---|---|
| **guest / reactor** | `PendingPatchAuthority::take_one` **defers** a slot whose surface is already in this turn's page. "At most one patch per surface per turn" is now a real wire contract, with a Rust law. The deferred slot keeps its own sequence and rides the very next turn, in order, exactly once — nothing is folded and nothing is lost. |
| **host / PluginRuntime** | a turn's page is split into **admission rounds** (a surface appears at most once per round); each round admits → acknowledges in ONE crossing → installs → closes before the next begins; a fault anywhere **retires** what the round already admitted instead of wedging it; and every path that accepts patches for an instance now enters through **one serialized intake queue** on that instance. |

---

## 1. Root cause, with the two paths named

### 1.1 What the defect report said, and what the measurement said

The peer's reading (INTERACTIVE-TOOLS-VISIBLE-PROCESS, :6013, build 17:54) was *"two host paths — the
ShellHost progress-subscription refresh and the owed/drain settle — accept patches for the same instance
concurrently"*. That hypothesis is **refuted by measurement**, twice:

- semio-91 ran three sessions on :6013 against this lane's own `[DEBUG] intake concurrent entry`
  instrumentation (a re-entrancy counter around `acceptUiPatches`): **`intake concurrent entry` = 0 in every
  run.** Every host path that accepts patches already runs inside `serializeCommandIngressForActor`, which is
  strictly serial per actor — the ShellHost progress refresh reaches the runtime through `refreshUi`, which
  takes that same lease.
- what the same instrumentation DID find, in all three runs: exactly one turn right after `toolRunStart`
  reporting `[DEBUG] intake duplicate surfaces in one turn … duplicates: ["1:framework.panel.toolRun"]`,
  6–8 patches in the page.

### 1.2 The real mechanism, in two named places

**Guest — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🦀️.rs`, `take_one`.**
Pending slots are minted **per reconcile pass and per external push, never per surface**. One `toolRunStart`
therefore leaves two of them for `framework.panel.toolRun`: the action's own panel reconcile
(`Ready to start` → `Running`, `ToolRunLedger::dirty_scope` names `FRAMEWORK_TOOL_RUN_BODY_KEY`) and the
run's first progress reconcile. `take_one`'s slot filter picked the oldest unemitted slot of the carried
instance with **no check that another slot for the same surface was already in this turn's page**, and since
the patch-batching lane removed the `UI_TURN_PATCHES_MAXIMUM = 1` floor on 2026-09-15 a page can hold both.
They left together as `A` (revision n) and `A'` (`base_revision = n`).

**Host — `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`,
`acceptUiPatches`.** Phase 1 ran **every** patch of the page to its acknowledgement token and only then
submitted one `issued-ui-acks` turn:

```ts
for (const [index, patch] of turn.uiPatches.entries()) { … admit to token … admitted.push(…) }
const acknowledged = await submitPluginLifecycleTurn(lease, { kind: "issued-ui-acks", entries: … });
for (…) { … install … await closeIntake(instanceId, intake); }
```

So while `A` sat at phase `ack` — `cell.wire` non-terminal and `cell.ack` set — `A'` called
`OwnedUiInstance.beginPatch` on the same cell and hit
`(cell.wire && !cell.wire.terminalIsEmpty()) || cell.page || cell.ack` → **`Foreign or busy instance surface
owner`** (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts:251`).

**And the throw is what made it permanent.** It escaped before `closeIntake`, so `A`'s intake — and the whole
round's — stayed open and the panel's cell stayed busy forever. That is the peer's whole console in one
sentence: ~20 `typed-operation progress refresh failed` in 40 ms (the ShellHost progress subscription
re-asking), then `[DEBUG] action failed toolRunFinalize … plugin-ui.intake-rejected`, then a panel stuck on
`Running`. Run 1's Abort reaching `Aborted, nothing was changed` ~1 s late with no feedback in between is the
same cell already half-wedged: the refresh that would have painted `Aborting…` was the one being rejected.

### 1.3 The counterproof

The e2e law below fails with **exactly the production error** the moment the round split is disabled
(one-line local edit, reverted):

```
× installs a turn page that names one surface twice, in order, and leaves no surface wedged 345ms
Error: plugin-ui.intake-rejected:intake:Busy instance surface owner
```

---

## 2. Design

### 2.1 Guest — defer, never fold (`take_one`)

```rust
fn turn_page_carries(&self, surface: &ui_contract::SurfaceId) -> bool {
    self.turn_handbacks.iter().any(|cell| cell.published.as_ref().is_some_and(|(published, _)| published.0 == surface.0))
}
fn slot_surface(slot: &PendingPatchSlot) -> Option<&ui_contract::SurfaceId> { … }
```

applied to **both** arms of `take_one` — the retained-cell arm (a cell a previous page could not carry) and
the slot arm. A deferred slot is still `PendingPatchPhase::Queued`, so it still arms the turn
(`guest_owes_turn`) and the very next turn publishes it: no stall, no fold, no lost sequence. Folding `A'`
into `A` was rejected on purpose — the two are a revision CHAIN whose ops the host applies in order, and
merging them would make the guest re-derive a delta it already computed.

This is the half that also protects the **wgpu** door, whose retained host drives the same
`OwnedUiPatchIntake` contract from Rust.

### 2.2 Host — rounds, one ack per round, fault-safe retirement

```ts
export function uiPatchAdmissionRoundsV1(surfaceIds: readonly (string | null)[]): readonly (readonly number[])[]
```

Consecutive rounds in which a surface appears at most once; a repeat opens the next round; the numbers are
the patch's own index in `turn.uiPatches`, which is what `captureUiPatchAuthority(turn.original, index)` is
addressed by, so publication order and wire identity both survive. `[A, B, A'] → [[0, 1], [2]]`.

Per round: `installUiPatchRound` submits ONE `issued-ui-acks` for the round, installs each surface and closes
each intake. A round is a legal acknowledgement unit — `ShardClient.submitInstanceUiAcknowledgements` admits
any non-empty **subset** of a turn's patches (it is the same path the one-patch case took before batching),
verified in source before relying on it. The batching lane's win is kept: N surfaces ready together are still
one round, still two crossings (its own law still passes, §3.1).

`retireAdmittedUiPatches` is the fault arm: on any throw the intake that faulted is closed, and the ones
already admitted in that round are **acknowledged and closed anyway** (a subset batch), so one fault can
never leave a surface holding a wire, an input page or a receipt outbox. This is what removes the
"wedged for the life of the instance" half of the defect independently of its cause.

### 2.3 Host — one serialized intake per instance

```ts
const serializeInstanceUiIntake = <T,>(instanceId: number, run: () => Promise<T>): Promise<T> =>
  serializePerActor(`ui-intake:${pluginId}#${instanceId}`, run);
```

Every path that accepts UI patches for an instance now enters through it: the `settlePluginTurn` accept
callback of a command turn, the owed/drain settle, the ShellHost progress subscription's refresh, job
completions, extension completions, and the hot-swap tear-down (`closeUiOwner`, which must never retire an
instance's surfaces while one of its patches is mid-admission). The recursion over each round's
acknowledgement turn deliberately does **not** re-enter — it is already inside the queue's one admission.

It is keyed by the plugin-qualified instance id (never reused, so a hot swap that replaces the handle keeps
the same queue) and backed by the file's own bounded `serializePerActor` rather than a second mechanism.
Measurement says concurrency was not the cause; the queue is nonetheless the invariant that makes
`beginPatch`'s busy arm unreachable from host code, which is what §2.4 then asserts.

### 2.4 `beginPatch` — two faults, named apart

```ts
if (cell.owner !== this || cell.name !== value.surface || !cell.surface) throw new Error("Foreign instance surface owner");
if (patchIsOpen(cell)) throw new Error("Busy instance surface owner");
```

plus a public query `OwnedUiInstance.surfacePatchIsOpen(facade)`. Identity errors stay faults forever; "busy"
is now a distinct, diagnosable name that a correct host cannot reach. The old message conflated them, which
is why the peer's console could not tell a real foreign owner from a scheduling defect.

---

## 3. Laws, with output

### 3.1 Rust — `cargo test -p semio-framework-plugin --lib turn_patch_batch -- --test-threads=1 --nocapture` (foreground)

```
running 6 tests
test …::a_patch_over_the_turn_byte_budget_travels_on_the_next_turn_and_still_applies_exactly ... ok
test …::a_refused_turn_page_returns_every_patch_to_its_own_slot_in_publication_order ... ok
test …::a_turn_patch_page_never_asks_the_guest_for_more_than_one_contiguous_ceiling ... ok
test …::n_ready_surfaces_converge_in_two_crossings_when_their_patches_fit_the_budget ...
[DEBUG] turn-patch-batch surfaces=1 crossings=2 patch=1 events=1 idle=0
[DEBUG] turn-patch-batch surfaces=3 crossings=2 patch=1 events=1 idle=0
[DEBUG] turn-patch-batch surfaces=6 crossings=2 patch=1 events=1 idle=0
[DEBUG] turn-patch-batch surfaces=8 crossings=2 patch=1 events=1 idle=0
ok
test …::the_first_ready_patch_is_admitted_whatever_the_budget_says ... ok
test …::two_queued_patches_of_one_surface_never_travel_in_the_same_turn_page ...
[DEBUG] turn-patch-batch per-surface deferral: page1=[("82:framework.panel.toolRun", 1), ("82:surface-other", 1)] page2=[(82:framework.panel.toolRun, 2)]
ok

test result: ok. 6 passed; 0 failed; 757 filtered out
```

`two_queued_patches_of_one_surface_never_travel_in_the_same_turn_page` is the new one: three queued
publications `[panel@1, other@1, panel@2]` leave as a page of **two** (the repeat is deferred, every other
ready surface still travels), and the deferred one rides the very next page, in order, exactly once. The
batching lane's own law is re-asserted unchanged above it — deferral costs no crossing for surfaces that are
genuinely distinct.

### 3.2 TypeScript — in-source vitest on `🔌️PluginRuntime/🟦️.tsx` (foreground)

Run with `.🧬semio/…/PROCEDURAL-3D-END-TO-END/🧪️vitest-plugin-runtime.config.ts` (the workspace's
`test long <name>` router is mid-refactor by a peer and no longer accepts a file filter):

```
bun node_modules/vitest/vitest.mjs run --config "<ticket>/🧪️vitest-plugin-runtime.config.ts" -t "owned ui intake admission rounds"
 Test Files  1 passed (1)
      Tests  3 passed | 123 skipped (126)

[DEBUG] intake rounds: [a,b,a] → [[0,1],[2]] — a repeated surface opens the next round, order and wire indices preserved
[DEBUG] beginPatch faults: identity → 'Foreign …', an open previous patch → 'Busy …'; surfacePatchIsOpen tracks the wire, page and receipt outbox
[DEBUG] intake rounds e2e: page [panel@1, other@1, panel@2] installed in rounds, later refreshes still publish — 0 'Busy instance surface owner'
```

| law | asserts |
|---|---|
| `opens the next round at a repeated surface, so one round never names a surface twice` | the pure rule: `[a,b,a] → [[0,1],[2]]`, `[a,b,c] → [[0,1,2]]`, `[a,a,a] → [[0],[1],[2]]`, empty → empty, unnamed patches stay distinct; and for every case, the rounds' flattened indices are the page's own indices in order and no round repeats a surface |
| `names a foreign surface owner and a busy one apart, and answers whether a patch is still open` | on a REAL `OwnedUiInstance` driven through a real `ShardClient`: `surfacePatchIsOpen` false → `beginPatch` → true; a second `beginPatch` throws **`Busy instance surface owner`**; another instance's `beginPatch` throws **`Foreign native instance patch owner`**; after the acknowledgement crosses and the patch closes, `surfacePatchIsOpen` is false again and the SAME second patch is admitted |
| `installs a turn page that names one surface twice, in order, and leaves no surface wedged` | the whole production runtime (`loadPluginModule` → `createApp` → `refreshUi`) against a guest whose open turn hands back `[panel@1, other@1, panel@2]` with `A'.base = A.revision`: every patch installs, the sibling surface is unaffected, and a LATER refresh still publishes and projects (`refresh-3`) — the proof the cell is not wedged. `console.error` is captured for the whole run and asserted to contain no `intake-rejected` and no `Busy instance surface owner` |

Whole-file run: `Tests 2 failed | 124 passed (126)`. **Both reds are peer-owned and reproduce alone, in
files this lane never opened:**

- `reads every request-carrying effect out of its nested WIT params record` — asserts `documentJson` against
  a runtime that now emits `artifactJson` (the in-flight `document`→`artifact` rename). Red before this
  lane's first edit, also red in the predecessor lane's report.
- `keeps the leftover vortex id on an armed brush window` — times out at 5 000 ms importing
  `🌐️World3dHost/🟦️.tsx`; the `Shell` ↔ `ShellHelpers` module cycle a peer's rework flipped. Fails when run
  **alone** with the same timeout, so it is not an interaction with the new laws.

### 3.3 Rust — the wider reactor suite, and what is NOT this lane's

`cargo test -p semio-framework-plugin --lib reactor -- --test-threads=1` currently aborts on two tests:
`reactor_output_fault_returns_real_patch_and_preserves_other_lifecycle_ack`
(`assertion failed: preparation.ui_patches.is_empty()`) and
`one_reactor_turn_pumps_the_envelope_decode_worker_to_its_terminal_poll`
(`the reactor-turn pump left the decode pending after 260 steps`).

**Neither is this lane's.** Verified, not assumed: both were re-run with this lane's two `take_one` filters
neutralized in place (`true || !self.turn_page_carries(…)`, reverted immediately after) and **both fail
identically**. `⚛️reactor/🔄️turn/🦀️.rs` carries an unstaged peer edit in the working tree; this lane touched
only `⚛️reactor/📨️pending/🦀️.rs` (+27/−1) and its own test file.

---

## 4. Files

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📨️pending/🦀️.rs` | `turn_page_carries` / `slot_surface`; both arms of `take_one` defer a slot whose surface is already in this turn's page |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🧺️turn-patch-batch/🦀️.rs` | the per-surface deferral law |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `uiPatchAdmissionRoundsV1`; `serializeInstanceUiIntake`; `acceptUiPatches` → gate + `acceptOwnedUiPatches` round loop; `installUiPatchRound`; `retireAdmittedUiPatches`; `closeUiOwner` → gated `closeOwnedUiInstance` |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts` | `patchIsOpen`; `surfacePatchIsOpen`; `beginPatch`'s foreign/busy faults named apart |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | the three TypeScript laws |
| `<ticket>/🐍️tool-run-intake-probe.mjs` | the browser probe (both doors) |
| `<ticket>/🐍️fixture-menu-recon.mjs` | example-fixture recon (how long a generation3d run lasts) |
| `<ticket>/📜️serve-generation3d-react-6024.sh`, `<ticket>/📜️restage-intake-lane.sh` | this lane's serve and restage |
| `<ticket>/🧪️vitest-plugin-runtime.config.ts` | in-source vitest config for the PluginRuntime laws |

---

## 5. Probe evidence in the browser

### 5.1 THE proof — the peer's own probe on :6013, read-only, with the HOST half only

`:6013` is the peer's serve (not restarted, nothing in their ticket edited); it serves this host module from
source, verified before the run (`curl … /@fs/…🔌️PluginRuntime/🟦️.tsx | grep uiPatchAdmissionRoundsV1` → 3
hits). Its puzzle 3d **guest is NOT restaged** — so the guest still emits the duplicate-surface page, and
what is being measured is the host absorbing it. That is the stronger half of the proof.

```
cd .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS
bun 🔍️w5-abort-finalize-midrun-probe.ts --count=2000 --wait=1500
```

```
== Abort: before start   {"panel":"Ready to start | Start", committed:1, provisional:0}
before Abort  {"t":1597,"panel":"Running · Choosing vortex and object (2/5) | Pause | Step", provisional:40}
Abort clicked
after Abort   {"t":  28,"panel":"Running · Choosing vortex and object (2/5)",  provisional:40}
after Abort   {"t": 420,"panel":"Running · Testing collision (3/5)",           provisional:64}
after Abort   {"t":1190,"panel":"Aborted, nothing was changed · Testing collision (3/5) | Start | Dismiss", provisional:0}

== Finalize: before start (after Dismiss — the SECOND run of the session)
before Finalize {"t":1705,"panel":"Running · Testing collision (3/5)", committed:1, provisional:48}
Finalize clicked
after Finalize  {"t": 551,"panel":"Running · Testing collision (3/5)",            provisional:48}
after Finalize  {"t":1598,"panel":"Finalizing · Placing (4/5)", finalizeDisabled:true, provisional:76}
after Finalize  {"t":2191,"panel":"Finalized · Placing (4/5) | Start | Dismiss",  committed:77, provisional:0}
```

| the peer's measurement (17:58 capture) | before | after |
|---|---|---|
| `plugin-ui.intake-rejected … Busy instance surface owner` | **~20 in 40 ms** | **0** |
| mid-run Finalize (2nd run, after Dismiss) | never left `Running`, 10 s window | **`Finalizing` +1 598 ms → `Finalized` +2 191 ms** |
| objects committed by the partial finalize | 0 (the action faulted) | **77** |
| mid-run Abort | `Aborted` ~1 s after the click, no feedback in between, provisional 37 → 48 → 0 | **terminal at +1 190 ms**, with the run visibly advancing (40 → 64) in between, provisional → 0 |
| page errors | — | **0** |

Copy kept at `<ticket>/🗑️generated/intake-6013-after/abort-finalize-midrun-1789494511118.txt` (the probe also
writes to its own ticket's `🗑️generated`, where it always writes; nothing there was edited).

### 5.2 generation3d on :6024 — Start / mid-run action / Dismiss, twice

`<ticket>/🐍️tool-run-intake-probe.mjs`, `SEMIO_PROBE_CYCLES=2`, the action pressed the instant the panel
arms it (`previewEval` runs ~250–900 ms end to end on this door, so there is no 1.5 s window to wait for).
Output at `<ticket>/🗑️generated/intake-6024-hostonly/`.

| cycle | action | pressed at | panel when pressed | terminal shown | intake faults |
|---|---|---|---|---|---|
| 1 | Abort | mid-run | `Running · Tessellating meshes (2/2)` | `Finalized · Tessellating meshes (2/2)` at **+417 ms** | **0** |
| 1 | Finalize | mid-run | `Running · Tessellating meshes (2/2)` | `Running · Evaluating nodes (1/2)` +538 ms → `Finalized` at **+969 ms** | **0** |
| 2 | Abort | mid-run | `Running · Evaluating nodes (1/2)` (71 %) | **`Aborted, nothing was changed`** at **+877 ms** | **0** |
| 2 | Finalize | — | the re-arm picked `No example`, so no run started (probe artefact) | — | **0** |

`intakeRejected: 0`, `intakeBlocked: 0`, `pageerrors: 0` over the whole 69 s session, four `Dismiss` presses
included. Cycle 1's Abort landing on `Finalized` rather than `Aborted` is the run finishing on its own inside
the ~250 ms it takes — cycle 2 catches it at 71 % and gets the real `Aborted, nothing was changed`.

**Observation for the ticket, not a defect this lane fixes:** after `Dismiss`, generation3d's Tool runs panel
is completely empty — no run group and no *ready* group, so there is no Start to press. `previewEval` is
dispatched by the document, not by an active tool, and `ToolRunLedger::panel` only draws the ready group for
an active run-declaring tool (`ready_tool`). The probe therefore re-arms each later cycle by picking the next
example, which re-dispatches `toolRunStart`. Worth its own lane.

### 5.3 The battery on :6024 — 3/4 green, the red attributed

```
SEMIO_BATTERY_URL=http://127.0.0.1:6024/?plugin=generation3d SEMIO_BATTERY_ROOT=react-intake \
  bun 🐍️react-battery.mjs --only=cancel-preview,status-parity,keyboard-verbs,journey

[DEBUG] battery ■ journey        ok=true  exit=0 121s steps=14/14 pageerrors=0
[DEBUG] battery ■ cancel-preview ok=true  exit=0  16s steps=4/4   pageerrors=0
[DEBUG] battery ■ keyboard-verbs ok=false exit=0 200s steps=17/18 pageerrors=0
[DEBUG] battery ■ status-parity  ok=true  exit=0 201s steps=12/12 pageerrors=0
[DEBUG] BATTERY DONE green=3/4 red=["keyboard-verbs"] 538s pageerrors=0
```

`journey` all 8 examples + generate mode + viewer role, `status-parity` 12/12, `cancel-preview` 4/4 —
including `cancel-preview-eval` inside `keyboard-verbs`, which presses `mod+.` on work genuinely in flight
and gets `toolRunAbort` + `phase: cancelled`: the tool-run abort path this lane is about, green.

The single red row is `keyboard-verbs > edit-arms-history`: typing `8` into the Column Height slider **does**
arm the history (`canUndo: true`, `update-widget input-slider id=height … value=8` on the cursor), but the
preview never settles on a different delivered extent — box and digest stay `…,6` / `365066bd:1681` for the
whole ~154 s the step waits.

**Attributed, not assumed.** The row is reproducible, and it is red with this lane's host change **fully
bypassed** — the round split neutralized *and* `acceptUiPatches` calling `acceptOwnedUiPatches` directly
instead of through the intake queue (both edits reverted immediately after):

| run | round split | intake queue | `edit-arms-history` |
|---|---|---|---|
| `🗑️generated/react-intake/keyboard-verbs` | on | on | red, box `…,6` |
| `🗑️generated/editor-verbs/keys-nosplit` | **off** | on | red, box `…,6` |
| `🗑️generated/editor-verbs/keys-fullbypass` | **off** | **off** | red, box `…,6` |

It was green at 12:37 today (`🗑️generated/react-sweep-closing/keyboard-verbs`, box `…,8`), so it regressed
somewhere in the afternoon's fleet — in the widget-edit → `flowEvalTick` lane, not in patch intake. Handed
back to the coordinator rather than claimed.

### 5.4 The guest half is NOT yet in a browser

`📜️restage-intake-lane.sh` (`NX_SKIP_NX_CACHE=true`) has been running since 19:14 and has not produced a new
`🌀️procedural` module: the shared build dir is queueing 8 `wasm32-wasip2` cargo processes behind one live
`rustc` while peers hold it, swap is at 30.1 GB of 30.7 GB, and the flow plugin's build exceeded its budget
once (the script retries, up to 6 attempts). **Everything measured in §5.1–§5.3 is therefore the HOST half
alone, against guests that still emit the duplicate-surface page** — which is the stronger reading for the
defect, because the host is what had to stop wedging on it. The reactor half is proven by its Rust law
(§3.1) and will reach the browser with the next successful activation.

---

## 6. Not claimed

- **Not claimed: that the two host paths run concurrently.** They do not, and this lane's own instrumentation
  measured it at zero on three runs of the failing scenario. The serialized intake queue is an invariant this
  lane added because the task asked for it and because it makes `beginPatch`'s busy arm unreachable — it is
  not what fixed the defect.
- **Not claimed: that mid-run Abort now shows an `Aborting…` pill on generation3d.** The guest already has
  the state and the label (`ToolRunState::Aborting` → `ToolRunLabel::StateAborting`, and
  `dispatch_tool_run_action` already returns a `Partial` scope naming the tool-run panel), and the rejected
  refresh was what kept it off screen; but generation3d's `previewEval` run is ~250–300 ms end to end, so on
  that door the `Aborting` frame is below one refresh period and the panel legitimately goes straight to its
  terminal state. The visible-feedback claim is made on the long puzzle 3d run in §5.3, not on :6024.
- **Not claimed: any wgpu reading.** wgpu is on hold for this lane; the reactor half protects that door by
  construction, and no wgpu probe was run.
- **Not claimed: the two whole-suite reds in §3.2, the two in §3.3, or `keyboard-verbs > edit-arms-history`
  in §5.3 are fixed.** They are peer-owned; each was reproduced alone, and `edit-arms-history` was
  additionally re-run with this lane's whole host change bypassed and stayed red (§5.3).
- **Not claimed: the reactor half has been seen in a browser.** The generation3d activation had not produced
  a new guest module when this report was written (§5.4). Every browser reading here is the host half.
- **Not claimed: the serve was recycled.** It was not needed: :6024's vite stat-guard had already re-served
  the peer's `__semioTypedOpAckCensus` `defineProperty` fix and this lane's own edits (verified by curling
  the `/@fs` module and grepping for both before every reading).
