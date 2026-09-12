# Wave B33 — why battery #51 reds pass in a fresh lane: the hover/tick backlog that starves every later command

Ticket 26/09/02/PUZZLE-3D-END-TO-END · wasm #51 on `:6013` (host vite-live) · all probing foreground, `pgrep`
gate checked before every run · **no Rust touched by this wave** (the fix is host TypeScript only, so it needs
no new wasm and rides #51 as-is).

---

## 1 The gap, restated as a measurement

Battery #51 (`🗑️generated/battery-2026-09-12-51-6013.txt`, `probe-2026-09-12T02-44-30.{md,ndjson}`, 83 steps,
one browser, `--reload-between-groups`) burned the full 30 s budget on `volume-brush-add-target-volume`,
`catalogue-add-object-kind`, `duplicate-selection`, `delete-selection`, `gumball-scene-delta`,
`relocate-pose-delta`, and rendered `engagement-input-present input=0`, `add-object-dialog-opens
dialogSurfaces=[]`, `context-menu-opens rows=0 ingressesWhileWaiting=0`, `volume-brush-arm
activeUtility=select`, `export-only download=none` — with FAULTS=0 and a live guest at every group boundary.

Two of the named steps are **not** full-run-only and are NOT this wave's: B29 §4.5 measured
`gumball-scene-delta FAIL waitedMs=30246` and `relocate-pose-delta FAIL waitedMs=30007` in **fresh** single-lane
runs as well (world payload never republishes — B32's completion-refresh / world-lane path). Everything else in
the list is a **state-accumulation** red, and this wave names its one cause, fixes it in the product, and
re-measures every one of them with the polluting predecessor still run first.

---

## 2 The bisect

Canary = the cheapest mutation in the failing region. `volume-brush-add-target-volume` (Alt+click → guest
`addTargetVolume`) for the mutate tail, `catalogue-add-object-kind` for the head. One probe invocation per row,
one browser each, `bun 🔍️browser-probe.ts --only=… --port=6013`.

| # | prefix (`--only=`) | canary | verdict | run |
|---|---|---|---|---|
| L0 | `catalogue-panel` | catalogue-add | **PASS** 13 586 ms | `probe-2026-09-12T03-10-29` |
| L1 | `pick-object,context-menu,catalogue-panel` | catalogue-add | **PASS** 9 947 ms | `03-15-38` |
| L2 | `pick-object,context-menu,tool-category,fill-tab,fill-abort-engagement,fill-wait-ready,fill-apply-max,fill-history,catalogue-panel` | catalogue-add | **PASS** 13 821 ms | `03-16-44` |
| L3 | `frame-perspective,brush-stroke,suggestions-open,volume-brush` | volume-add | **FAIL** 30 039 ms | `03-20-10` |
| L4 | `volume-brush` | volume-add | **PASS** 13 240 ms | `03-22-23` |
| L5 | `suggestions-open,volume-brush` | volume-add | **PASS** 20 762 ms | `03-23-08` |
| L6 | `frame-perspective,brush-stroke,volume-brush` | volume-add | **FAIL** 30 267 ms | `03-24-20` |
| L7 | `frame-perspective,volume-brush` | volume-add | **PASS** 4 776 ms | `03-26-03` |

**The polluting predecessor is `brush-stroke` (§9), and only it.** The example switch to Nakagin is innocent
here (it runs in the LAST group, `replace`, long after every mutate red — candidate (a) is disproved by the #51
timeline itself: `instances=1` through the whole mutate group). The fill suite is innocent (L2). `suggestions-open`
alone is innocent (L5) though it raises latency. `frame-perspective` is innocent (L7). `Agent disconnected` is
constant from t=140 s of the mutate group onward and is present in the `replace` group where example switch, undo,
redo and import all PASS — it is a real presence defect but it is **not** the discriminator (candidate (c) disproved).

---

## 3 Root cause, measured

`brush-stroke` arms Brush and performs a **70-move hover storm** over the 11 rim vortices, then clicks. A
purpose-built helper reproduces and isolates it — `🔍️b33-brush-pollution.ts`, which keeps an **unbounded**
console log so a mark taken before a gesture still addresses that gesture's lines:

```
bun 🔍️b33-brush-pollution.ts --pollute=none  → VERDICT add-target-volume ok=true  before=0 after=1 waitedMs=11785
bun 🔍️b33-brush-pollution.ts --pollute=brush → VERDICT add-target-volume ok=false before=0 after=0 waitedMs=30232
```

Queue census of the polluted run (`b33-pollution-brush-2026-09-12T03-40-08.console.txt`, split at
`[DEBUG] setActiveUtility hop window=puzzle3d-main-perspective next=volumeBrush`):

```
before the volumeBrush arm        after the arm (30 s window)
  72 interactionHover dispatch      1 addTargetVolume dispatch  (seq 201, lane Interactive)
  11 interactionHover settled       0 addTargetVolume settled      ← never reached
  85 suggestionsTick  dispatch     53 interactionHover settled     ← the backlog draining
  10 suggestionsTick  settled      53 suggestionsTick  settled     ← ~3.5 turns/s
```

One 70-move storm enqueues **~136 unsettled guest turns**. `addTargetVolume` is dispatched correctly, lands in the
Interactive lane with a seq, and then waits behind all of them; at the measured drain rate it needs ≈ 39 s, so a
30 s budget reads a *starved* command as a *dropped* one. Same mechanism for `catalogue-add-object-kind`,
`duplicate-selection`, `delete-selection`, and for every "a guest reply never arrived" red (`context-menu-opens
rows=0`, `add-object-dialog-opens dialogSurfaces=[]`, `engagement-input-present input=0`, and even
`volume-brush-arm activeUtility=select` — the arm's own reply starves).

The backlog also **self-feeds**: while it drains, the guest keeps publishing the hover it was told about at the
time each queued turn was made, so `data-interaction-json` cycles `hoveredVortexFullId` v0→v1→…→v10→v0 with the
pointer standing still, and each published change fires another tick. The guest's own
`puzzle3d.brushPreview.hover utility=brush` is still printed minutes after `volumeBrush` was armed — those are
replays of turns queued while Brush was armed, not a guest that ignored the arm.

### 3.1 Why the existing gates did not hold — `file:line`

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`

- `dispatch` (was line 4878) is `(action, args) => { onAction({…}); }` — **it discards `onAction`'s promise**, and
  that promise is the only thing that settles on the guest's `OperationCompleted` frame (ShellHost ~6364:
  `plugin.handleAction(…).then(… await awaitOperationSettle(response.output) …)`).
- `dispatchInstanceHover` / `dispatchVortexHover` (was 5404/5419) wrap `dispatch` in
  `createCoalescingActionDispatcher`, whose gate is `Promise.resolve(dispatch(next)).finally(…)` —
  fed `undefined`, it cleared `inFlight` on the next microtask, so **the "at most one in flight" gate coalesced
  nothing**: 72 dispatches for 70 moves.
- `handleVortexHover` (was 5462) and `handleBrushPlace` (was 5593) each fired their own
  `void dispatch("suggestionsTick")`, and the hover-change effect (was 5262) fired a third — on top of the
  self-gating 120 ms interval, whose `return dispatch("suggestionsTick")` (was 5362) was inert for the same
  reason. Hence 85 ticks for 70 moves.
- `handleReferenceHover` (was 4990) dispatched `interactionHover` **directly on every pointer move**, with no
  coalescing and no gate at all — the same unbounded shape on a second marker lane.

This is the exact shape the fill lane's own law already names ("in-flight skipping interval → gates on exactly
what run returns — a discarded dispatch promise gates nothing", from the 2026-09-09 20:55 fill defect) and that
B22 already fixed for the mesh-page lane with `dispatchBrushMesh`, "dispatch's awaitable twin". Hover and the
ticks were never migrated.

---

## 4 The fix (product, host-side)

1. **`dispatchSettled`** — `dispatch`'s awaitable twin (same envelope, returns `Promise.resolve(onAction(…))`),
   `World3dHost/🟦️.tsx` next to `dispatch`. Every self-gating lane now hands its gate a real awaitable:
   both hover dispatchers, the suggestions interval and the fill interval.
2. **Both hover dispatchers `return` their round trip** → the coalescing dispatcher holds exactly ONE hover
   outstanding and collapses the rest onto the latest target.
3. **One single-flight tick lane** — `suggestionsTickSeqRef` + `sendSuggestionsTick`
   (`createCoalescingActionDispatcher<number>(() => dispatchSettled("suggestionsTick"))`) behind
   `requestSuggestionsTick()`; the three per-gesture tick sites call it. At most one tick outstanding plus at
   most one queued behind it, whatever a gesture storm asks. The sequence number is load-bearing: a constant
   value would dedupe to the first tick and never ask again.
4. **`handleReferenceHover` becomes a coalescing dispatcher** reading `referencesRef`, so a republished scene
   cannot rebuild it and drop its in-flight state.
5. **`createCoalescingActionDispatcher`** (`🛠️ShellHelpers/🟦️.tsx`): `.then(release, release)` instead of a bare
   `.finally` — a refused round trip frees the gate in the same microtask hop and can no longer surface as an
   unhandled rejection now that callers hand over real awaitables.

Nothing else changed: no guest, no plugin, no probe (B31's file), no example/utility reset semantics were needed
once the queue is bounded — an armed utility never blocked anything, its *backlog* did.

### 4.1 After, same storm, same helper

```
--pollute=brush (final code)  VERDICT add-target-volume ok=true before=0 after=1 waitedMs=12619 / 21025
queue census, whole run:  interactionHover 3 dispatch / 3 settled · suggestionsTick 3 / 3
                          addTargetVolume  1 dispatch / 1 settled     (console 594 lines, was 2253)
```
72→3 hover turns, 85→3 ticks, and `hoveredVortexFullId` stops cycling (stays `seed-left-001:v8`).

---

## 5 Laws

All green, run foreground, tails quoted.

**`🧪️tests/🔬️engine-contract/🟦️.ts`** (3 new laws) —
`bun ./📜️script.ts test long ../../../../🧪️tests/🔬️engine-contract/🟦️.ts --testNamePattern='coalescing action dispatcher|in-flight skipping interval|self-gating world lane'`

```
 ✓ in-flight skipping interval > drops overlapping ticks instead of queueing them behind a slow run 3ms
 ✓ in-flight skipping interval > gates on exactly what run returns — a discarded dispatch promise gates nothing 0ms
 ✓ coalescing action dispatcher > dedupes unchanged values and keeps at most one in-flight dispatch 0ms
 ✓ coalescing action dispatcher > gates on exactly what the dispatch returns — a swallowed round trip coalesces nothing 0ms
 ✓ coalescing action dispatcher > keeps flushing after a rejected round trip 0ms
 ✓ registry-derived utilities and activation (P5) > every self-gating world lane dispatches through the awaitable twin 2ms
 Test Files  1 passed (1)      Tests  6 passed | 553 skipped (559)
```

- *"a swallowed round trip coalesces nothing"* — behavioural: the same 5-value storm against the same slow
  dispatch sends 5 turns when the callback drops the promise and 1 when it returns it.
- *"keeps flushing after a rejected round trip"* — a refusal must not wedge the lane.
- *"every self-gating world lane dispatches through the awaitable twin"* — structural, reads
  `World3dHost/🟦️.tsx` (same locate-root shape B31's gumball law uses): the twin returns `onAction`'s promise,
  every gated site reads `return dispatchSettled(…)`, and none of
  `void dispatch("suggestionsTick") | void dispatch("fillBuildTick") | return dispatch("suggestionsTick") |
  return dispatch("fillBuildTick") | dispatch("interactionHover"` may exist; the per-gesture tick must go
  through `sendSuggestionsTick(suggestionsTickSeqRef.current)`.

**Language-neutral oracle + mounted jsdom suite** —
`bun x nx run @semio-tech/framework-renderer-react:world3d-interaction-check`

```
world3d-pointer-gesture-oracle: checks=63 clean
interaction-selection-set-oracle: checks=17 clean
selection-merge-vocabulary-oracle: checks=89 clean
 Test Files  1 passed (1)      Tests  14 passed (14)
 NX   Successfully ran target world3d-interaction-check for project @semio-tech/framework-renderer-react
```
The oracle's pinned hover call site was updated to the awaitable shape (`📜️script.ts` line ~425); the peer's
`🖱️pointer-gestures.json` was left byte-for-byte alone (it belongs to ticket 26/09/09 and is live).

**Full engine-contract suite** — `Tests 1 failed | 558 passed (559)`. The one red is **not** this wave:
`noteShellCommand > buildNoteShellCommandAction …` fails because a peer just added `inverseArgs` /
`inverseCommandId` to the descriptor without updating that law (`expected …(2) to deeply equal …(2)`, the diff is
exactly those two new keys).

**Typecheck** — `bun x tsc --noEmit -p tsconfig.json` in the react target reports 1 100 errors repo-wide from
peers' in-flight refactors (`📜️script.ts` 190, `🧪️owned-locale-detector-retirement` 152, `🧪️docklayoutstore` 84,
`LeftoverWorldSelectionOverlayV1.brushPreviewJson` removed under World3dHost, `pickEnabled` newly required, …).
Zero of them are in my edited regions or name any symbol I introduced (`dispatchSettled`,
`sendSuggestionsTick`, `requestSuggestionsTick`, `referencesRef`, `handleReferenceHover`) — checked by line range
and by symbol. The 1 100 are a peer-churn gate, not a B33 gate.

**Rust gates skipped, with reason**: this wave changed four TypeScript files and no `.rs` at all, so
`cargo test -p semio-s-artifact-puzzle-3d`, `cargo check -p semio-s-artifact-puzzle-3d --features
component-app-assembly` and `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` can only re-measure
peers' live Rust churn (dozens of modified `✏️s/🔌️plugins/**/🦀️.rs` in the tree right now) under load average
7–11. Nothing in B33 rides a wasm build.

---

## 6 Proof: the failing steps pass with the polluter still run first

One browser per row, `brush-stroke` (and `suggestions-open`) still executed before the canaries.

| step | #51 full battery | B33 after fix | run |
|---|---|---|---|
| `volume-brush-add-target-volume` | FAIL `waitedMs=30470` | **PASS** `before=0 after=1 waitedMs=16983` / `18659` | `04-08-00` / `04-23-28` |
| `volume-brush-arm` | FAIL `activeUtility=select` | **PASS** `activeUtility=volumeBrush` | both |
| `catalogue-add-object-kind` | FAIL `waitedMs=30295` | **PASS** `before=1 after=2 waitedMs=17356` / `23348` | `04-08-00` / `04-23-28` |
| `catalogue-drag-drop` | FAIL | **PASS** | both |
| `duplicate-selection` | FAIL `waitedMs=30464` | **PASS** `before=3 after=4 waitedMs=15807` / `23175` | `04-08-00` / `04-23-28` |
| `delete-selection` | FAIL `waitedMs=30852` | **PASS** `before=5 after=4 waitedMs=15363` / `15631` | `04-08-00` / `04-23-28` |
| `focus-selection` | FAIL | **PASS** (camera moved) | both |
| `engagement-input-present` | FAIL `input=0`, pane folded | **PASS** `toggle=1 input=1 placeholder=brush, fill <n>, zoom, clear, pick, rectangle, lasso unfold={"unfolded":true…}` | `04-16-58` |
| `engagement-clear-is-a-noop` / `engagement-fill-verb` / `engagement-abort` | not reached / B29 red | **PASS** / **PASS** `#tool.fill aria-pressed=true waitedMs=4889` / **PASS** | `04-16-58` |
| `context-menu-opens` | FAIL `rows=0 polls=30 ingressesWhileWaiting=0` | **PASS** `rows=8 polls=6 ingressesWhileWaiting=5` | `04-16-58` |
| `outliner-hide-applies` / `outliner-show-restores` | FAIL (B29's #50 regression) | **PASS** / **PASS** `restored=true` | `04-16-58` |
| `add-object-dialog-opens` | FAIL `dialogSurfaces=[]` | **PASS** `dialogSurfaces=["semio-dialog-…=Add Object …"]` | `04-12-22` |
| `add-object-kind-options-are-dynamic` | FAIL | **PASS** `options=["Hexagonal Cut Concrete Forest Left","Base","Bridge","Capital",…]` | `04-12-22` |
| `settings-value-reaches-window-rail` | FAIL `10.5 vs 10` | **PASS** | `04-12-22` |
| `fill-history-entry` (regression check — the fill tick's cadence changed) | PASS `entries=13` | **PASS** `entries=28`, `fill count after End: 0` unchanged | `04-14-34` |

`battery-hard-faults` / `battery-faults` PASS `hard=0 collateral=0 distinct=0` and `guest-alive-*` PASS in every
run above. Absolute latencies (15–23 s) are high because the machine carried load average 7–11 and a second
peer dev server; they are **latency, not starvation** — the console census of `04-05-04` shows every lane at
`dispatch == settled` (`addTargetVolume 1/1`, `interactionHover 4/4`, `setCamera 12/12`, `interactionSelect 12/12`)
with no `suggestionsTick` backlog at all.

### 6.1 Reds that remain, and whose they are

- `brush-preview-place FAIL instances=1 preview=null` — **guest**, rides #52: the guest's own gate says
  `puzzle3d.brushPreview.gate reason=no-free-candidate vortex=… free=0 pending=true index=0`. The preview cache
  has no free candidate slot; nothing host-side can fix that (B24's lane).
- `engagement-brush-verb FAIL activeUtility=select waitedMs=30199` — **guest**, rides #52: the typed `brush`
  verb does not arm the utility (its sibling `fill` verb passes, so the engagement plumbing is fine).
- `catalogue-add-selects-new-object` / `duplicate-reselects-clone` FAIL `selected=[]` — the selection-publication
  family B29 §3.4 already owns; the objects ARE created (`added=1`, `before=3 after=4`).
- `context-menu-object-vocabulary` / `context-menu-zoom-row-action-is-registered` — the menu that opens after
  `brush-stroke` is the *suggestion* menu (`present=["suggestion-0","suggestion-1",…]`), so this row is probe
  ordering: see §7.
- `camera-emits-no-artifact-history`, `projection-repaints-camera`, `locale-control-present switched=false`,
  `add-object-instance-count-increases`, `export-only download=none` — untouched by this wave, unchanged by it.
- `gumball-scene-delta`, `relocate-pose-delta` — B32 (fresh-lane reds, world-lane republication).

Honest caveat on the read group: #51's read-group reds were recorded while **two probes shared the one tab**
(B29 §4.6 documents the overlap at 02:44–02:45). `add-object-dialog-opens`, `add-object-kind-options-are-dynamic`
and `settings-value-reaches-window-rail` are green now in a clean single-probe read-group run, but part of that
flip may be the absence of the second probe rather than this fix. The mutate-tail flips in the table above are
attributed by direct before/after bisect on the same wasm, the same machine and the same command, twice.

---

## 7 For B31 (probe owner) — two recipes, no probe edits made

1. **`consoleBuf` is a 4000-line ring that `shift()`s (`🔍️browser-probe.ts:102`), so every length-based mark is
   invalid once a long run saturates it.** `const mark = consoleBuf.length` → `consoleBuf.slice(mark)` returns
   `[]` forever after saturation. That is the sole reason battery #51 reported `catalogue add console tail=[]`,
   `ingressesWhileWaiting=0`, `guestTaps=[]` and empty `hops=` — the probe measured its own buffer, not the app,
   and those empties read exactly like "the click never dispatched". Fix: keep a monotonic counter next to the
   ring (`consoleSeq += 1` per line, plus `dropped`) and slice by `seq - dropped`; or drop the cap. Verified by
   building `🔍️b33-brush-pollution.ts` with an unbounded log — the same gesture that reported `tail=[]` in #51
   reports 312 matching lines there.
2. **`context-menu-rows` must run before `brush-stroke` or disarm the brush utility first.** With Brush armed and
   a hovered vortex, the right-click opens the *suggestion* menu (`openVortexSuggestions`, `World3dHost` line
   ~5265: `world3dSuggestionsGestureArmed(alt, hover) || brushArmed`), so the object vocabulary can never be
   present. Either order `context-menu-rows` ahead of `brush-stroke` in the mutate group, or have the step
   dispatch `engagementAbort`/re-arm `select` in its precondition and assert `activeUtility=select` before it
   opens the menu.
3. Suggested budget note: with the queue bounded, every mutation above settled in ≤ 24 s on a load-11 machine.
   The 30 s budget is now a real budget rather than a starvation detector, but a loaded machine sits at 60–80 %
   of it — a 45 s budget for the mutation family would remove the remaining flakiness without hiding anything.

## 8 Infrastructure note for the fleet

Twice during this wave `:6013` answered `http=200` while serving a **stale vite transform**: the page died with
`SyntaxError: The requested module '…/🏛️ShellHost/🔀️surface-switch/🟦️.ts' does not provide an export named
'createSessionWorkLedgerV1'` although the export is on disk (line 184), and the probe reported only
`boot FAILED … windows=0 canvases=0` (its console tail carried a bare `500 (Internal Server Error)`).
The served module was 33 755 bytes against 46 267 after a plain `touch` of that file — the watcher missed a
peer's write under the emoji path. **Recipe: `touch` the file named in the SyntaxError and re-fetch
`/@fs/<url-encoded path>`; a boot failure with `console=3` and no app logs is this, not the app.**

## 9 Files

Product:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` — `dispatchSettled`,
  single-flight `requestSuggestionsTick`, both hover dispatchers gated, `handleReferenceHover` coalescing,
  `referencesRef`, both tick intervals on the twin.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` —
  `createCoalescingActionDispatcher` releases on `then(release, release)`.

Laws:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — 3 laws.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts` —
  oracle's pinned hover call site follows the awaitable shape.

Ticket:
- `🔍️b33-brush-pollution.ts` (input, kept) — the isolating repro with an unbounded console log.
- `🗑️generated/b33-L0-catalogue.txt`, `b33-L1.txt`, `b33-L2.txt`, `b33-L3-brushpre-volume.txt`,
  `b33-L4-volume-baseline.txt`, `b33-L5-sugg-volume.txt`, `b33-L6-brush-volume.txt`, `b33-L7-frame-volume.txt`,
  `b33-after-L3.txt`, `b33-after-L8.txt`, `b33-after-L8b.txt`, `b33-after-read-group.txt`, `b33-after-fill.txt`,
  `b33-after-chrome.txt`, `b33-pollution-*.txt` + `*.console.txt`, `probe-2026-09-12T03-10-29 … 04-23-28.*`.

**Wave note:** `World3dHost/🟦️.tsx` was rewritten out from under this wave three times mid-session (a peer tool
re-emits the whole file); two of my hunks had to be re-applied and were verified present by substring count
immediately before and after the final browser run (`PRE/POST sendSuggestionsTick=1 dispatchSettled sites=12`).
Anyone editing that file today should re-check their own hunks the same way.
