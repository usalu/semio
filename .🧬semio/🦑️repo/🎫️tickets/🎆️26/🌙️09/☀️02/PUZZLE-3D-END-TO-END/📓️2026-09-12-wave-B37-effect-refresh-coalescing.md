# Wave B37 — the refresh a host effect owes itself, and the one ui-refresh lane

Ticket `26/09/02/PUZZLE-3D-END-TO-END` · 2026-09-12 · wasm #53 on `:6013` (host vite-live).
Everything below ran in the FOREGROUND with its tail quoted.

---

## 0 Verdict summary

| Item | Root cause (`file:line`) | Fix | Law / measurement |
|---|---|---|---|
| the lost refresh | `🛠️ShellHelpers/🟦️.tsx:4563` (`typedOperationCompletionRefreshV1`) answers `{kind:"none"}` for a completion that carries effects but dirtied nothing, and `🏛️ShellHost/🟦️.tsx:5563` (`applyHostEffects`' trailing refresh) forwarded that `none` verbatim to `refreshUi`, which returns at its own `if (scopeArg.kind === "none") return`. **The `SetActiveUtility` pass asked for no pass at all** | `hostEffectRefreshScopeV1` — the declared scope UNIONED with the scope the applied effects earned | law `makes an effect that rewrites a guest render input earn its own pass…`, proven to FAIL without the fix (§4.1); live: the follow-up now carries `utilities:{"puzzle3d-main-perspective":"brush"}` and the guest returns CHANGED bodies (§3.2) |
| the coalescer itself | `🏛️ShellHost/🟦️.tsx` — the hand-rolled `while` loop read its owed slot AFTER `await pass` with no `catch` between it (one rejected pass dropped the follow-up), joiners awaited the RUNNING pass rather than the one covering them, and the in-flight marker was written AFTER the pass was invoked (a re-entrant request started a second concurrent pass) | `createUiRefreshCoalescerV1` — one named lane with three stated properties; `ShellHost` applies a pass's own `requestedEffects` OUTSIDE the pass | 6 laws in the renderer-react engine-contract suite, two proven to FAIL without the fix (§4.2/§4.3) |
| residual red | NOT the refresh any more: the guest re-renders the armed pane (`brushPreview.lane utility=brush preview=281`, 3× inside the earned pass) and the host applies its changed body, yet BOTH panes' `data-interaction-json` keep reading `select` — the world record is assembled under ONE shared `recordKey: puzzle.3d.play.viewport` for both panes | handed over (§5) | `engagement-brush-verb` still FAIL behind `brush-stroke`, PASS isolated (§3.3/§3.4) |

---

## 1 The tap, and the lost refresh

A temporary unconditional tap (all four sites removed before finishing) logged every `refreshUi` ask with
the in-flight/owed state, every coalescing decision, every pass start/request/end/abandon, and every
`applyHostEffects` entry/exit with its effect kinds, owner and session verdicts.

`bun 🔍️browser-probe.ts --only=brush-stroke,engagement-bar --port=6013`
(`🗑️generated/probe-2026-09-12T05-46-23.md`, console tail lines 805-847) — the whole defect in ten lines:

```
805  puzzle3d.utility.publish action=engagementSubmit window=Some("puzzle3d-main-perspective") utility= map_hit=false
809  performInvocation settled {…"actionId":"engagementSubmit"…"historyUpserts":0,"effects":1}
810  b37.effects enter {"kinds":["setActiveUtility"],"scope":"none","owner":true}
811  b37.effects done  {"owner":true,"sessionCurrent":true,"scope":"none"}
        ── NO `b37.refreshUi ask` line here. The lane was never asked. ──
812  puzzle3d.brushPreview.lane utility= preview=0 vortices=2638      ← the pass ALREADY in flight, gen 13
…    (30 s of probe polling; both panes publish `select`)
824  performInvocation {…"actionId":"engagementSubmit"}               ← the probe's NEXT gesture
835  b37.pass end   {"owed":"full"}
836  b37.pass start {"scope":"full","gen":14,"utilities":{"puzzle3d-main-perspective":"brush"}}
```

Three things this settles that reading could not:

1. `owner:true` and `sessionCurrent:true` — the effect pass was NOT dropped by
   `isCurrentEffectOwner`/`shellDialogSessionIsCurrentV1`. It reached its trailing refresh.
2. The trailing refresh's scope is **`none`**, so `refreshUi` returned on its first line. The arm was
   applied to the host's utility map and NOTHING was asked to publish it.
3. The pass that finally carried `brush` (line 836, gen 14) was owed by a LATER, unrelated request. The
   arm rode a stranger's refresh, 30 s late — which is exactly why the same verb passes in an isolated
   lane: with nothing in flight, `refreshUi` runs its pass synchronously in the same turn as the arm, so
   the arm is in the ref by the time the request is built.

`b37.pass threw` never appeared, and every `b37.pass end` was followed by its owed pass — so the owed-slot
loss the handover suspected (B35 §3.3(b), `🏛️ShellHost/🟦️.tsx:4747-4772`) was NOT what fired here. It is
real all the same, and §2.2/§4 fix it.

### 1.1 Why the scope is `none`, and why that is not `typedOperationCompletionRefreshV1`'s fault

`engagementSubmit` is a retained typed operation: the admitting reply carries no outcome, and the
completion frame is the only carrier of the mutation's scope (B27). This completion has
`historyUpserts:0`, no patch and `UiDirtyScope::None` — **correct**, because the GUEST dirtied nothing.
Arming a utility is not a guest edit: `scene.active_utility` becomes `Effect::SetActiveUtility`
(`✏️editor/🦀️.rs:3464-3466`), and the host's own `activeUtilityByWindowId` is the only carrier back into
the next render. So `typedOperationCompletionRefreshV1` keeps answering `{kind:"none"}` (its `requestedEffects
.length > 0` branch, `🛠️ShellHelpers/🟦️.tsx:4563`) and the effect pass is what must earn the render.

---

## 2 The fix

### 2.1 `hostEffectRefreshScopeV1` — what the HOST dirtied by applying the effects

`🛠️ShellHelpers/🟦️.tsx`, next to `typedOperationCompletionRefreshV1`:

```ts
const HOST_EFFECTS_REWRITING_GUEST_RENDER_INPUTS = ["setActiveUtility", "setActiveTool", "setPanel", "loadDocument"] as const;
export function hostEffectRefreshScopeV1(effects, declared, windowBodyKeys): UiDirtyScope
```

The host's per-window utility map, active tool and `panelJson` ride into EVERY `refresh-ui` as
`ViewModel.activeUtilityByWindowId`/`activeToolId`/`panelJson`, so applying one of those four changes what
every later render must produce. `setActiveUtility`/`setActiveTool` earn
`{window bodies, utilities, tools, measures}`; `setPanel`/`loadDocument` name no body of their own and earn
`full`. The declared scope and the earned scope are **unioned, never substituted** — and when nothing was
earned the declared scope is returned by identity, which the law pins with `toBe`.

The no-storm half is the filter: `notify` is host React state, `clipboardWrite`/`downloadMediaExport` never
reach a projection, `dispatchAction`/`replayShellCommand` re-enter shell dispatch with their own scope, and
`navigate`/`spawnPluginInstance`/`openPluginInstance` switch sessions (a switch forces its own full fetch).
None of them upgrades a `none` completion into a pass, so B27's measured 18-completions-per-idle-10 s stay
free.

`🏛️ShellHost/🟦️.tsx:5551-5564` asks with it (both the spawned and the primary branch), and the gated trace
now prints `{declared, scope}` side by side.

### 2.2 `createUiRefreshCoalescerV1` — one lane, three properties

`🛠️ShellHelpers/🟦️.tsx`. `ShellHost`'s hand-rolled in-flight/owed pair is replaced by one built-once lane
(`uiRefreshLaneRef`), reaching the live pass and effect applier through refs; `refreshUi` is now a two-line
`useCallback` with `[]` deps (which also stops the `refreshUi` identity churn every effect in this file
lists in its deps array).

1. **Every request is owed until a pass covers it.** The starter is not a special case — it registers into
   the same owed slot every joiner does and the drain loop takes it from there, so a caller's promise
   settles when the pass that covered ITS request settled. The predecessor let joiners `await` the pass
   that was already running, i.e. one that by construction could not carry what they asked for.
2. **A failed pass never ends the lane.** The rejection reaches exactly the requests that pass covered
   (the boot refresh at `:4893` turns one into the session fault card) and the loop still runs what was
   owed behind it. The predecessor read its owed slot after `await pass`, so a throw skipped that read and
   the follow-up was dropped — then silently cleared by the next unrelated starter.
3. **A pass may ASK for another pass; it may never WAIT for one.** The in-flight marker is raised BEFORE
   the loop is entered (the predecessor assigned it after invoking the pass, so a re-entrant request saw
   an empty lane and started a SECOND concurrent pass into the same guest), and `ShellHost` now stores a
   pass's own `requestedEffects` in `owedPassEffectsRef` and applies them from the lane, OUTSIDE the pass
   and voided. Before, `runUiRefreshPass` did `await applyHostEffects(pendingRefreshEffects, …)` inside
   itself → `applyHostEffects` → `refreshUi` → `await inFlight`, which is the pass awaiting itself: the
   in-flight marker would stay pinned forever and nothing would repaint again.

---

## 3 Measurement

### 3.1 Laws

`bun x nx run @semio-tech/framework-renderer-react:test-long -- ../../../../🧪️tests/🔬️engine-contract/🟦️.ts --run --testNamePattern="ui refresh coalescing lane|earn its own pass|typed operation on its completion"`

```
 Test Files  1 passed (1)
      Tests  8 passed | 562 skipped (570)
```

The seven new ones (six in `describe("ui refresh coalescing lane")`, one beside B27's completion law):

- `turns every refresh asked for during an in-flight pass into exactly ONE follow-up carrying their union`
- `carries a utility armed by a host effect while a pass was in flight into the very next pass`
- `answers a 50-request hover storm during one in-flight pass with exactly one follow-up` (51 requests → 2 passes)
- `runs the owed follow-up after a pass that REJECTED, and rejects only the requests that pass covered`
- `serves a request made from INSIDE a pass with the next pass instead of wedging the lane`
- `asks for nothing at all on a none scope`
- `makes an effect that rewrites a guest render input earn its own pass, and leaves host chrome paying nothing`

### 3.2 The fixed hop, live (`🗑️generated/probe-2026-09-12T06-45-27.md`, tail 688-734)

```
687  performInvocation settled {…"actionId":"engagementSubmit"…"effects":1}
688  b37.effects refresh {"kinds":["setActiveUtility"],"declared":"none","earned":"partial","bodies":["puzzle3d.play.composite"]}
703  b37.pass request {"gen":15,"scope":"full","windows":["puzzle3d-main","puzzle3d-main-top","puzzle3d-main-perspective"],"utilities":{"puzzle3d-main-perspective":"brush"}}
710  puzzle3d.brushPreview.lane utility=brush preview=281 vortices=2639
718  puzzle3d.brushPreview.lane utility=brush preview=281 vortices=2639
724  puzzle3d.brushPreview.lane utility=brush preview=281 vortices=2639
734  b37.sections {"utilities":{"puzzle3d-main-perspective":"brush"},"asked":["puzzle3d-main","puzzle3d-main-top","puzzle3d-main-perspective"],"changed":["puzzle3d-main","puzzle3d-main-perspective"]}
```

`declared:"none"` → `earned:"partial"` is the fix firing. The follow-up pass carries the armed map, the
guest re-renders the armed pane three times with a live preview, and the guest returns CHANGED bodies for
exactly the two sections whose projection moved (`puzzle3d-main-top`'s hash is byte-identical and stays
unchanged, which is correct — it is not the armed pane). In the ISOLATED lane the same chain publishes
`vortices.publish utility=brush brush_or_volume=true` ~1 s after the submit
(`probe-2026-09-12T06-32-54.md` tail 95-107).

### 3.3 Lane verdicts

`bun 🔍️browser-probe.ts --only=brush-stroke,engagement-bar --port=6013` (final, tap-free code —
`🗑️generated/b37-probe-lanes-final.txt`):

```
verdict boot PASS
verdict brush-preview-place PASS
verdict engagement-input-present PASS
verdict engagement-placeholder-has-no-dead-verbs PASS
verdict engagement-brush-verb FAIL activeUtility=select panes=[{"window":"puzzle3d-main-top","surface":"window:puzzle3d-main-top","activeUtility":"select"},{"window":"puzzle3d-main-perspective","surface":"window:puzzle3d-main-perspective","activeUtility":"select"}] heuristic=select waitedMs=30487
verdict engagement-clear-is-a-noop PASS
verdict engagement-fill-verb PASS
verdict engagement-abort FAIL rearmed=false activeUtility=select waitedMs=4
verdict guest-alive-mutate PASS
verdict battery-hard-faults PASS
verdict battery-faults PASS
done booted=true faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0 verdicts=22 → probe-2026-09-12T07-03-20
```

`bun 🔍️browser-probe.ts --only=engagement-bar --port=6013` (isolated, `🗑️generated/b37-probe-engagement.txt`):

```
verdict engagement-brush-verb PASS                                                  (46.5s)
verdict engagement-clear-is-a-noop PASS
verdict engagement-fill-verb PASS
verdict engagement-abort FAIL rearmed=true activeUtility=brush waitedMs=15101
verdict guest-alive-mutate PASS · battery-hard-faults PASS · battery-faults PASS
done booted=true faults=0 hard=0 collateral=0 first-hard-fault-at=none guest-death-faults=0 verdicts=20 → probe-2026-09-12T07-05-37
```

Unchanged verdicts across five runs of the loaded lane (pre-fix, post-fix, two experiments, final):
`boot`, `brush-preview-place`, `engagement-input-present`, `engagement-placeholder-has-no-dead-verbs`,
`engagement-clear-is-a-noop`, `engagement-fill-verb`, `guest-alive-mutate`, `battery-hard-faults`,
`battery-faults` — all PASS, 0 faults, 0 hard faults, 0 guest deaths in every run.

### 3.4 Two experiments that did NOT hold, and were reverted

Both were attempts on the residual of §5, kept here so nobody spends the same hour:

- **`replaceBodies = true` for any earned effect pass** (so `forceReloadLiveUiStoresV1` republishes into the
  live built-node stores the mounted surfaces read). It did not move `engagement-brush-verb`, and it
  REGRESSED `brush-preview-place` to `FAIL [expect-41] instances=1 preview=null`
  (`probe-2026-09-12T06-43-01`) — the whole-cache form re-pushes every key the cache still holds, stale
  siblings included. Reverted; `brush-preview-place` is PASS again in the next run.
- **a narrow twin that force-reloads only the sections THIS response re-serialized.** No regression
  (`brush-preview-place` PASS, `probe-2026-09-12T06-48-40`) and no effect on the verdict either. Reverted
  as an unproven behaviour change — the live store is not the hop.

---

## 4 Real coverage — each law fails without its half of the fix

### 4.1 The earned scope

`hostEffectRefreshScopeV1`'s guard inverted (`rewriting.length >= 0`, i.e. always return the declared scope):

```
FAIL … > makes an effect that rewrites a guest render input earn its own pass, and leaves host chrome paying nothing
AssertionError: arming a utility is a host-owned render input — the pass that applies it owes the render that publishes it:
  expected { kind: 'none' } to deeply equal { kind: 'partial', …(7) }
```

### 4.2 A failed pass may not end the lane

The drain loop's `try`/`catch` removed (predecessor shape: the owed slot is read only after a successful
`await`):

```
FAIL … > runs the owed follow-up after a pass that REJECTED, and rejects only the requests that pass covered
AssertionError: a failed pass may not take the lane down with it: expected [ { scope: { …(2) }, …(2) } ] to have a length of 2 but got 1
```

The law asserts the follow-up BEFORE awaiting either outcome on purpose: with the drop in place both
promises stay pending forever, and the first shape of this law hung the whole suite until the 300 s budget
killed it (`test-long … exceeded 300000ms — killed`). A dropped follow-up must be a failed assertion, not
a hung lane.

### 4.3 The in-flight marker is raised before the pass runs

The marker written after `drain()` is invoked (predecessor shape: `uiRefreshInFlightRef.current = pass`
after `runUiRefreshPass(…)`):

```
FAIL … > serves a request made from INSIDE a pass with the next pass instead of wedging the lane
AssertionError: expected [ { kind: 'full' }, …(1) ] to have a length of 1 but got 2
```

Two passes crossing into the same guest at once, from one request.

---

## 5 Residual, handed over with its evidence

`engagement-brush-verb` behind `brush-stroke` is no longer a refresh defect. In the SAME poll window the
probe reads `select`, the earned pass carried `utilities:{"puzzle3d-main-perspective":"brush"}`, the guest
rendered that pane armed three times, and the host applied its changed body (§3.2). What the probe reads is
`data-interaction-json`, and the guest writes that record from the same `envelope.active_utility` its own
`brushPreview.lane utility=brush` line prints
(`🪟️windows/🧊️main/🦀️.rs:488` `"activeUtility": scene_mode(&envelope.active_utility)`, logged at `:647`).

The discriminator to chase next is in the console of every failing run: the host assembles the world record
under ONE key for BOTH panes —

```
[DEBUG] puzzle3d.brushPreview.assemble {recordKey: puzzle.3d.play.viewport, …}
[DEBUG] puzzle3d.brushPreview.assemble {recordKey: puzzle.3d.play.viewport, …}
```

— and the probe's two panes report the IDENTICAL value in every run (both `select` when it fails, both
`brush` when it passes), which is what a last-writer-wins shared record looks like and not what two
independently projected panes look like. `paneUtilities()` keys by `data-window-instance-id`; the record
behind it does not. That is B20's world-surface identity hop
(`📓️2026-09-12-wave-B20-world-surface-identity.md`), one level below this wave's.

`engagement-abort` stays B31's open red (B35 §3.4), now with the same shape in both lanes.

---

## 6 Gates

| command | tail |
|---|---|
| `bun x nx run @semio-tech/framework-renderer-react:test-long -- …🔬️engine-contract/🟦️.ts --run --testNamePattern="ui refresh coalescing lane\|earn its own pass\|typed operation on its completion"` | `Test Files 1 passed (1)` / `Tests 8 passed \| 562 skipped (570)` |
| the same suite, unfiltered | `Tests  10 failed \| 566 passed (576)`. The suite itself grew 569 → 576 tests DURING this wave (a peer is writing in it), and all 10 reds are that peer's mid-flight work — none in this wave's regions, none touching `refreshUi`/`applyHostEffects`/the lane: 6 × `extension invocation completion ownership` (an uncommitted new block importing `🔁️extension-invocation-wire/🔣️.json` + `💥️extension-evaluate-fault.json`; `expected "vi.fn()" to be called once with arguments: ['evaluate', …]`), `noteShellCommand > buildNoteShellCommandAction …` (`args.commandId` shape), 2 × `framework renderer hosts > … shared Flow browser runtime …`, and `node-graph surface attachment in a hidden tab > retained surface host …` (`4 refreshes must dispatch exactly one attach: expected 4 to be 1` — a test that mounts `FlowGraphCanvasHost` directly and never reaches this file). `git diff HEAD` shows those blocks are working-tree additions, and `tsc` reports type errors inside exactly them (`5217`, `5218`, `7292`, `8538`, `10242`). |
| `bun x tsc --noEmit -p tsconfig.json` (renderer-react) | filtered to the two files this wave edits: `ShellHost/🟦️.tsx(1987,79)`, `(1999,52)`, `(8212,35)`, `(8213,116)`, `(8924,59)` and `🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts(86,5)` — the same pre-existing set B13/B17 reported before peer line drift (they named `1905\|7774\|7775`), **no error in `4540-4800` / `5550-5570`, and none in `🛠️ShellHelpers/🟦️.tsx` at all** |
| `bun 🔍️browser-probe.ts --only=brush-stroke,engagement-bar --port=6013` | §3.3 |
| `bun 🔍️browser-probe.ts --only=engagement-bar --port=6013` | §3.3 |

---

## 7 Files

Product:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx`
  — `createUiRefreshCoalescerV1` + `UiRefreshCoalescerV1`,
  `HOST_EFFECTS_REWRITING_GUEST_RENDER_INPUTS`, `hostEffectsRewriteGuestRenderInputsV1`,
  `hostEffectRefreshScopeV1`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`
  — `refreshUi` on the lane (`uiRefreshLaneRef`, `runUiRefreshPassRef`, `applyHostEffectsRef`),
  a pass's own `requestedEffects` handed to the lane through `owedPassEffectsRef` instead of awaited
  inside the pass, `applyHostEffects` asking with the earned scope, `refreshUi sections` now naming the
  utilities the pass carried.

Laws:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
  — `describe("ui refresh coalescing lane")` (6) plus the effect-scope law beside B27's completion law.

Ticket:
- `🗑️generated/probe-2026-09-12T05-46-23.*` (pre-fix tap), `probe-2026-09-12T06-32-54.*` (isolated,
  post-fix), `probe-2026-09-12T06-34-19.*`, `probe-2026-09-12T06-38-01.*` (sections tap),
  `probe-2026-09-12T06-43-01.*` (reverted experiment 1), `probe-2026-09-12T06-48-40.*` (reverted
  experiment 2), `probe-2026-09-12T06-45-27.*`, and the two final tap-free runs
  `probe-2026-09-12T07-03-20.*` (+ `b37-probe-lanes-final.txt`) and `probe-2026-09-12T07-05-37.*`
  (+ `b37-probe-engagement.txt`).

## 8 For the fleet

1. **A completion's `UiDirtyScope` is what the GUEST dirtied. It is not what the pass owes.** Every host
   effect that rewrites host-owned state the guest reads back (the utility map, the active tool,
   `panelJson`) must earn its own scope, or it is applied and never published. `scope:"none"` on an effect
   pass with `owner:true`/`sessionCurrent:true` is the signature.
2. **"No render for 30 s" was not a dropped follow-up.** Nothing asked. Before blaming a coalescer, tap
   whether the ask happened at all — `applyHostEffects`' trailing `refreshUi` returns on its first line
   for a `none` scope.
3. **A lane whose pass can ask the lane for another pass must never let the pass await it.** Both halves
   of that (the marker raised before the loop, the effects applied outside the pass) are now laws.
4. Two panes reporting the IDENTICAL `activeUtility` is a shared-record signature, not two reads. §5.
