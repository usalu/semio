# Wave B35 — probe console sequence ring, the brush-candidate warm gate, and the typed `brush` verb

Ticket `26/09/02/PUZZLE-3D-END-TO-END` · 2026-09-12 · wasm #53 on `:6013` (host vite-live).
Everything below ran in the FOREGROUND with its tail quoted. The two guest laws ride #54 — the browser
lanes in §4 measure the #53 guest, i.e. the *probe* half of this wave plus the pre-fix guest.

---

## 0 Verdict summary

| Item | Root cause (`file:line`) | Fix | Law / measurement |
|---|---|---|---|
| 1 probe console ring | `🔍️browser-probe.ts:102` — `consoleBuf` is a 4 000-line ring that `shift()`s, so a `consoleBuf.length` mark addresses a dropped line and `slice(mark)` reads `[]` forever | monotonic `consoleSeq` + `consoleCursor()` / `consoleSince()`; all 8 mark sites migrated | every lane below reports real tails (§4.1) |
| 1b `context-menu-rows` after `brush-stroke` | probe registration order; with a utility armed the right-click opens the *suggestion* menu | `STEP_LEADS_ITS_GROUP` hoists the step, AND the step disarms whatever is armed itself | `context-menu-object-vocabulary` **FAIL → PASS** |
| 2 brush preview never computes | `🎮️commands/⏱️suggestions-tick/🦀️.rs:18-25` — the tick resolved its target from the menu and the hover ONLY, while the render also falls through to the latch (`world_brush_preview_target`, `🪟️windows/🧊️main/🦀️.rs:527-534`). With the hover cleared and the cache dropped, the tick ran `target=None` forever | third leg: the latched `brush_live_target` | 2 laws, one proven to FAIL without the fix (§2.3) |
| 3 typed `brush` verb does not arm | **not the guest.** Two independent causes, both measured: (a) the verdict read the WRONG PANE — `dumpBrushPreview` resolves its host by "longest `data-brush-preview-json`", which lands on an unarmed sibling publishing `select` correctly; (b) after a hover-heavy predecessor the post-effect UI refresh does not repaint the world body until the next user gesture (`🏛️ShellHost/🟦️.tsx:4747-4772` coalescing / `:5537`), so the armed map is invisible for the whole budget | probe: pane-scoped `paneUtilities()` + pane-scoped abort reads; guest: a law pinning the effect's window INSTANCE. (b) is host-side and handed over | `engagement-brush-verb` **PASS** in an isolated lane, still FAIL behind `brush-stroke` (§3.3/§4.3); new law `the_engagement_brush_verb_arms_the_utility_of_a_pane_instance_never_the_window_kind` |

---

## 1 The probe console ring

### 1.1 Root cause

`🔍️browser-probe.ts:102` (before):

```ts
if (consoleBuf.length >= 4000) consoleBuf.shift();
consoleBuf.push(`${msg.type()}: ${text}`);
```

Eight readers took `const mark = consoleBuf.length` before a gesture and then `consoleBuf.slice(mark)`.
Once the ring saturates, `length` is pinned at 4 000 and `slice(mark)` is `[]` — permanently. That single
defect produced every `catalogue add console tail=[]`, `ingressesWhileWaiting=0`, `guestTaps=[]` and empty
`hops=` in battery #51, and all of those read exactly like "the click never dispatched" (B33 §7.1).

### 1.2 Fix

A sequence beside the ring, not a length into it:

```ts
const CONSOLE_RING_LINES = 4000;
let consoleSeq = 0;
const consoleCursor = () => consoleSeq;
const consoleSince = (mark: number) => consoleBuf.slice(Math.max(0, mark - (consoleSeq - consoleBuf.length)));
```

`consoleSeq - consoleBuf.length` is the number of dropped lines, so a mark older than the retained window
clamps to the oldest retained line instead of yielding nothing. Migrated marks: the engagement `submit`
tail, `leftoverViewTail`, the selection-surfaces mark, `context-menu` (console tail **and**
`ingressesWhileWaiting`), the zoom tail, the outliner-hide tail, the catalogue-add tail.
`--reload-between-groups` semantics are untouched (the ring and the sequence both survive a reload, which
is exactly what a cross-reboot mark needs).

### 1.3 Group ordering and the armed-utility precondition

Two statements, because `--only=` can defeat either one alone:

- `STEP_LEADS_ITS_GROUP = ["context-menu-rows"]` + `groupEntries(group)` — the step is self-contained (it
  frames, selects through the outliner and disarms), so leading the `mutate` group costs it nothing.
- the step itself reads the armed utility and, since puzzle3d has **no `select` utility** (the default is
  the empty id, published as `select`), disarms by clicking whatever is armed a second time, then polls
  for `select`. `context-menu-opens` now also prints `armedBefore=` / `armedAtRightClick=`.

---

## 2 The brush-candidate warm gate (guest)

### 2.1 What the live console actually says

B33 read `brushPreview.gate reason=no-free-candidate free=0 pending=true` and called the preview dead. The
full console of its own final run (`🗑️generated/b33-pollution-brush-2026-09-12T04-03-47.console.txt`) says
something sharper:

```
384  brushPreview.hover  utility=brush menu=false vortex=Some("seed-left-001:v3")
385  brushPreview.cache  vortex=seed-left-001:v3 free=0 pending=true resume=0      ← cache MISS
387  brushPreview.gate   reason=no-free-candidate … free=0 pending=true index=0
400  suggestionsTick.enter utility=brush menu=false target=Some("seed-left-001:v3")
401  cache tick-before   free=0 pending=true
405  cache tick-after    free=4 pending=true slices=3                              ← ONE tick warmed it
417  brushPreview.compute vortex=seed-left-001:v3 bytes=282
…
459  suggestionsTick.enter utility=  menu=false target=None
464  suggestionsTick.enter utility=brush menu=false target=None                    ← the tick lost the target
```

So the cache warms in **one** tick / three slices, and the gate is a transient — *unless the tick stops
asking*. Lines 459/464 are that state, and 13 renders in the same run printed
`gate reason=no-target … vortex=None`.

### 2.2 Root cause — the tick and the render disagreed about which vortex is live

`🪟️windows/🧊️main/🦀️.rs:527-534` — the render resolves **three** legs:

```rust
pub fn world_brush_preview_target(…) -> Option<String> {
    envelope.runtime.suggestion_menu … .or_else(|| puzzle3d_brush_target_vortex(envelope, interaction))
                                       .or_else(|| session.brush_live_target().map(str::to_string))
}
```

`🎮️commands/⏱️suggestions-tick/🦀️.rs:18-25` resolved only the first two — no latch. The latch is not an
exotic path: it is what answers whenever the leftover `refresh-ui` shape has no guest hover (the law
`leftover_refresh_after_suggestions_tick_still_publishes_latched_brush_preview` exists for exactly it), and
`brush_cache` is dropped by **every** accepted mesh upload (`install_collision_mesh`,
`⏳️precompute/🦀️.rs:1870-1882`) and every document edit (`rebuild_queue`, `:1873`). B22 measured **202
`registerBrushMesh` commands for one example**. So "hover cleared + cache invalidated" is the browser's
normal state, and in it the render asked for a vortex no tick would ever warm again:
`brush_candidates` is read-only and answers a cache miss with `{free: [], unknown_pending: true}`
(`⏳️precompute/🦀️.rs:2674-2675`), which is byte-for-byte the gate line.

### 2.3 Fix and laws

`⏱️suggestions-tick/🦀️.rs` — a third leg, gated on the armed utility so plain select-mode hovering still
costs no slices:

```rust
.or_else(|| brush_armed.then(|| ctx.app.precompute.borrow().brush_live_target().map(str::to_string)).flatten());
```

Two laws in `✏️editor/🧪️tests/🔬️unit/🦀️.rs`, both on a fresh Concrete Forest session
(`app()`'s fixture IS `seed-left-001` / "Hexagonal Cut Concrete Forest Left"), both against the
`render_window_refresh` shape the host actually renders, and both spending the declared budget
`PUZZLE3D_BRUSH_WARM_TICKS = 2` (the browser's measured one tick plus one tick of headroom):

- `one_hover_warms_the_brush_preview_within_the_declared_tick_budget`
- `the_tick_rewarms_the_latched_brush_target_after_an_edit_invalidated_its_candidates`

Both hover the **last** vortex, never the first: the lane's background round-robin enumerates the document
in order, so a law written on vortex 0 cannot tell a targeted warm-up from the enumeration walking past —
and the browser hovers rim vortices (`:v3`, `:v8`) deep in that enumeration, which is why it sees the gate.
The invalidator is the browser's own: one accepted single-page `registerBrushMesh` with geometry no other
law in the binary derives.

`RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 --nocapture one_hover_warms_the_brush_preview the_tick_rewarms_the_latched the_engagement_brush_verb_arms`

```
[DEBUG] brush warm ticks=1 budget=2 preview={"targetVortexFullId":"seed-left-001:v10","objectKindId":"Hexagonal Cut Concrete Forest Left",…}
[DEBUG] engagement brush addressed=["puzzle3d-main-perspective"] fallback=["puzzle3d-main-top"]
[DEBUG] brush relatch ticks=1 budget=2 preview={"targetVortexFullId":"seed-left-001:v10",…}
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 725 filtered out; finished in 0.84s
```

**Real coverage, measured** — with the third leg deleted and nothing else changed:

```
[DEBUG] brush relatch ticks=2 budget=2 preview=null
panicked at …🔬️unit/🦀️.rs:2222:5:
assertion `left == right` failed: a tick with no hover must still warm the vortex the render latched, got null after 2 ticks
test result: FAILED. 1 passed; 1 failed
```

`one_hover_warms_…` stays green without the fix (its hover leg is intact) — that law pins the budget, the
relatch law pins the lane.

---

## 3 The typed `brush` verb — the guest arms correctly; the VERDICT read the wrong pane

### 3.1 The guest chain, hop by hop (all green)

| hop | evidence |
|---|---|
| placeholder derives the verb list | `PUZZLE3D_ENGAGEMENT_VERBS` (`📨️engagement-submit/🦀️.rs:12`) |
| `brush` arm | `📨️engagement-submit/🦀️.rs:23-26` sets `scene.active_utility = brush` |
| the arm becomes a host effect | epilogue `✏️editor/🦀️.rs:3464-3466`: `next_active_utility != active_utility_initial` → `Effect::SetActiveUtility { window_id: wid, utility_id }` |
| `wid` is an INSTANCE, never the kind | `puzzle3d_addressed_window_id` (`✏️editor/🦀️.rs:966`): keyed → `view.window_id` → `view.focused_window_id` (B15) → command arg → roster's first non-kind pane |
| the effect leaves the guest | live: `performInvocation settled {…"actionId":"engagementSubmit"…"effects":1}` |
| the host arms the map | live: the next render prints `utility.publish action=engagementSubmit window=Some("puzzle3d-main-perspective") utility=brush map_hit=true` |
| the guest publishes it | live: `brushPreview.lane utility=brush preview=281 vortices=2638` + `vortices.publish utility=brush brush_or_volume=true` |

(`probe-2026-09-12T04-55-46.md` lines 1212-1255.) `fill` "worked" only because it is a mode-level TOOL
whose verdict reads `#tool.fill aria-pressed` — chrome — while `brush` is a window utility whose verdict
reads a world pane.

### 3.2 Root cause of the red

`dumpBrushPreview` resolves its host as `#puzzle3d-main-perspective [data-brush-preview-json]`, **else the
node with the longest `data-brush-preview-json`, else `#puzzle3d-main-perspective`** — and then reads
`data-interaction-json` off it. The guest publishes one record per pane (`utility=brush` for the armed one
interleaved with `utility=` for the others), so that heuristic can land on an unarmed sibling which is
publishing `select` entirely correctly. `engagement-brush-verb` was measuring the wrong pane.

### 3.3 Fix and measurement

New `paneUtilities()` reads `activeUtility` from every `[data-interaction-json]` keyed by
`data-window-instance-id`; `engagement-brush-verb` now settles on the **perspective** pane and reports all
panes plus the old heuristic's answer side by side. `engagement-abort`'s re-arm and disarm reads are
pane-scoped for the same reason, and its verdict now requires `rearmed` (an unarmed sibling satisfied both
`rearmed=false` and the abort's own `!== "brush"` predicate, i.e. it could pass with no disarm at all).

`bun 🔍️browser-probe.ts --only=engagement-bar --port=6013` (`🗑️generated/b35-probe-engagement.txt`):

```
[39.6s] verdict engagement-brush-verb PASS
[50.6s] verdict engagement-clear-is-a-noop PASS
[53.5s] verdict engagement-fill-verb PASS
[69.7s] verdict engagement-abort FAIL rearmed=true activeUtility=brush waitedMs=15311
```

`engagement-brush-verb` **FAIL → PASS** on the unchanged #53 guest, which is the proof that the verb was
never the defect.

Behind `brush-stroke`, however, it still FAILS — and now with evidence that this second red is NOT a
misread: **both** panes report `select`.

```
verdict engagement-brush-verb FAIL activeUtility=select panes=[{"window":"puzzle3d-main-top",…,"activeUtility":"select"},{"window":"puzzle3d-main-perspective",…,"activeUtility":"select"}] heuristic=select waitedMs=30594
```

The console of that same run (`probe-2026-09-12T05-09-54.md`) pins the hop exactly:

```
1179  utility.publish action=engagementSubmit window=Some("puzzle3d-main-perspective") utility= map_hit=false
1181  performInvocation settled {…"actionId":"engagementSubmit"…"effects":1}
      ── 30 s of polling, no render at all ──
1236  utility.publish action=engagementInput  window=Some("puzzle3d-main-perspective") utility=brush map_hit=true
```

The effect left the guest, the host applied it (`map_hit=true` proves the map holds `brush`), and the next
render happened only when the user typed again. In the ISOLATED lane the same submit is followed
immediately by `brushPreview.lane utility=brush preview=281`. The discriminator is the preceding hover
load, i.e. `applyHostEffects`'s trailing `await refreshUi(nextSession, uiScope, …)`
(`🏛️ShellHost/🟦️.tsx:5537`) against `refreshUi`'s in-flight coalescing
(`:4747-4772` — a pass in flight merges the caller's scope into `uiRefreshOwedRef` and returns after
`await inFlight`). `runUiRefreshPass` re-derives `activeUtilityByWindowId` from the ref (`:4572`), so the
owed pass would carry `brush` if it ran. **Host-side, outside this wave's three items** — handed over with
this recipe: run `--only=brush-stroke,engagement-bar` and `--only=engagement-bar`, and diff whether a
render appears between the `effects:1` settle and the next gesture.

### 3.4 A red this makes honest, and whose it is

`engagement-abort` is now a real measurement in both shapes. Isolated:
`FAIL rearmed=true activeUtility=brush waitedMs=15311` — the re-arm lands on the perspective pane
(`engagement re-armed brush=true activeUtility=brush waitedMs=27`) and Escape does not disarm it inside
15 s. Behind `brush-stroke`: `FAIL rearmed=false activeUtility=select waitedMs=36`, the same missing
repaint as §3.3(b). It had been passing *vacuously* on the unarmed-sibling read. Escape in the isolated
shape takes
`engagement_abort`'s fill branch (`tools=["tool.fill=true"]` from the preceding `fill 5`), which pushes
`SetActiveTool{""}` **and** leaves `scene.active_utility = PUZZLE3D_DEFAULT_UTILITY` for the epilogue to
emit as `SetActiveUtility{wid, ""}`. That pair is B31's own arm (`🛑️engagement-abort/🦀️.rs:22-37`) — not
this wave's — and it is now measurable instead of masked.

### 3.5 Residual noted, not fixed

`[DEBUG] engagement brush addressed=["puzzle3d-main-perspective"] fallback=["puzzle3d-main-top"]` — a verb
that carries no window of its own and reaches a view with **no** `focused_window_id` arms the roster's
FIRST pane, not the focused one. The new law only requires an instance (never the bare kind), because
`focused_window_id` is the host's only carrier of "which pane is the user looking at" and the browser's
engagement line is window chrome that always supplies `window_id`. Worth a law of its own if an app-level
panel ever grows an engagement line.

---

## 4 Browser lanes

### 4.1 `--only=context-menu-rows,brush-stroke,engagement-bar --port=6013` (`🗑️generated/b35-probe-lanes.txt`)

```
plan: [{"group":"read","steps":[]},{"group":"mutate","steps":["context-menu-rows","brush-stroke","engagement-bar"]},{"group":"replace","steps":[]}]
verdict context-menu-selection-precondition PASS
verdict context-menu-opens PASS
verdict context-menu-object-vocabulary PASS
verdict context-menu-zoom-row-action-is-registered PASS
verdict context-menu-zoom-moves-camera PASS
verdict brush-preview-place PASS
verdict engagement-input-present PASS
verdict engagement-placeholder-has-no-dead-verbs PASS
verdict engagement-brush-verb FAIL activeUtility=select waitedMs=30347     ← pre-§3.3 probe
verdict engagement-clear-is-a-noop PASS
verdict engagement-fill-verb PASS
verdict engagement-abort PASS                                             ← vacuous, see §3.4
verdict guest-alive-mutate PASS
verdict battery-hard-faults PASS
verdict battery-faults PASS
```

`context-menu-object-vocabulary` and `context-menu-zoom-row-action-is-registered` were **FAIL** in every
prior battery (B33 §6.1 attributed both to probe ordering) and are PASS here — the ordering + disarm
statement of §1.3 is what flipped them. `brush-preview-place` is **PASS** (B33 §6.1 had it FAIL
`instances=1 preview=null`) on the same #53 guest.

### 4.2 Same three lanes after the pane-scoped reads (`🗑️generated/b35-probe-lanes2.txt`)

```
verdict context-menu-selection-precondition PASS
verdict context-menu-opens PASS
verdict context-menu-object-vocabulary PASS
verdict context-menu-zoom-row-action-is-registered PASS
verdict context-menu-zoom-moves-camera PASS
verdict brush-preview-place PASS
verdict engagement-input-present PASS
verdict engagement-placeholder-has-no-dead-verbs PASS
verdict engagement-brush-verb FAIL activeUtility=select panes=[…"puzzle3d-main-top"…"select"…"puzzle3d-main-perspective"…"select"] heuristic=select waitedMs=30594
verdict engagement-clear-is-a-noop PASS
verdict engagement-fill-verb PASS
verdict engagement-abort FAIL rearmed=false activeUtility=select waitedMs=36
verdict guest-alive-mutate PASS
verdict battery-hard-faults PASS
verdict battery-faults PASS
```

Six context-menu / brush verdicts hold across both runs; the two engagement reds are §3.3(b)'s host
repaint, now reported with every pane's own utility instead of one heuristic answer.

### 4.3 Isolated engagement lane vs. the same lane behind `brush-stroke`

| verdict | `--only=engagement-bar` | `--only=context-menu-rows,brush-stroke,engagement-bar` |
|---|---|---|
| `engagement-brush-verb` | **PASS** (39.6 s) | FAIL `panes=[select, select]` `waitedMs=30594` |
| `engagement-abort` | FAIL `rearmed=true activeUtility=brush` | FAIL `rearmed=false activeUtility=select` |
| `engagement-fill-verb` | PASS | PASS |

### 4.4 Boot

`bun 🔍️browser-probe.ts --only=boot --port=6013` → `verdict boot PASS`,
`battery PASS=3 FAIL=0 FAULTS=0 first-hard-fault-at=none guest-death-faults=0`.

---

## 5 Gates

| command | tail |
|---|---|
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `warning: semio-s-artifact-puzzle-3d (lib) generated 88 warnings` / `Finished dev profile [unoptimized] target(s) in 11.75s` — 0 errors, warnings present (so the expansion really ran) |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `Finished dev profile [unoptimized] target(s) in 1.00s` |
| `RUST_MIN_STACK=134217728 cargo test … --lib -- --test-threads=1 brush engagement` (04:47) | `test result: ok. 61 passed; 0 failed; 0 ignored; 0 measured; 667 filtered out; finished in 2.42s` |
| the same command re-run at 05:25 | `test result: FAILED. 60 passed; 1 failed` — the one red is **not** this wave: `every_advertised_engagement_verb_is_implemented` fails inside `🔬️testkit/🦀️.rs:362` with `retained operation faulted: typed-operation emitted a store lane absent from its exact factory publication contract`, which appeared between the two runs while peers were writing `🔌️plugin/🦀️.rs` (5 min before), `✏️editor/🦀️.rs` (6 min) and `🎮️commands/🌱️add-object-kind/🦀️.rs` (6 min) — the publication-contract files. This wave's only `.rs` product edit is `⏱️suggestions-tick/🦀️.rs` (24 min before, untouched by that contract), and all three new laws are `ok` in the same run (lines 1422/1424/1426). |
| `bun build 🔍️browser-probe.ts --target=bun --external '*'` | `🔍️browser-probe.js 159.88 KB (entry point)` |

---

## 6 Files

Product (guest):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️suggestions-tick/🦀️.rs`
  — the latched third leg.

Laws:
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
  — `PUZZLE3D_BRUSH_WARM_TICKS`, `one_hover_warms_the_brush_preview_within_the_declared_tick_budget`,
  `the_tick_rewarms_the_latched_brush_target_after_an_edit_invalidated_its_candidates`,
  `the_engagement_brush_verb_arms_the_utility_of_a_pane_instance_never_the_window_kind`.

Ticket:
- `🔍️browser-probe.ts` — sequence ring, `STEP_LEADS_ITS_GROUP`/`groupEntries`, the context-menu disarm
  precondition, `paneUtilities()` and the pane-scoped engagement reads. No step or verdict renamed.
- `🗑️generated/b35-cargo-check.txt`, `b35-cargo-check-wasm.txt`, `b35-cargo-test-brush.txt`,
  `b35-cargo-test-brush-engagement.txt`, `b35-probe-boot.txt`, `b35-probe-lanes.txt`,
  `b35-probe-engagement.txt`, `probe-2026-09-12T04-55-34.*`, `probe-2026-09-12T04-55-46.*`,
  `probe-2026-09-12T05-*.*`.

## 7 For the fleet

1. **Any verdict about an armed utility must name its pane.** `dumpBrushPreview`'s longest-attribute
   heuristic is fine for "is there a preview at all" and wrong for "is THIS pane armed" — the guest
   publishes one interaction record per pane and the unarmed ones legitimately say `select`. §3.2.
2. `brush-preview-place`, `context-menu-object-vocabulary` and `context-menu-zoom-row-action-is-registered`
   are green on #53 with the probe fixes alone, in TWO runs; re-measure them in the next full battery
   before attributing them to any guest change.
3. **A guest effect that changes host session state needs a repaint that actually lands.** §3.3(b): the
   `engagementSubmit` → `Effect::SetActiveUtility` → `applyHostEffects` → `refreshUi` chain leaves the
   world body unrepainted for 30 s when a hover-heavy step precedes it, while the same chain repaints
   immediately in an isolated lane. `effects:1` + `map_hit=true` on the NEXT gesture is the signature.
   Host-side (`🏛️ShellHost/🟦️.tsx:4747-4772` / `:5537`), not the guest.
4. `engagement-abort` is a genuine open red now (§3.4) and belongs to B31's arm.
