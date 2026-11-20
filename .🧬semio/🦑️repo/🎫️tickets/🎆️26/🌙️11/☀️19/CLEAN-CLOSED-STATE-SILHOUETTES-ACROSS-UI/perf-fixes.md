# Performance Fixes: Staffage Overview Main-Thread Blocking

Base `🧰️framework/🔨️modules/🖱️ui/🧱️elements` = `E`, staffage react = `S` (`🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react`).

## 1. Compositor (`E/🗂️WindowChrome/shared-border.ts`)

Before: the observer set `geometric` for every `class`/`style` record anywhere in the subtree and treated every childList record as structural. Animated figure clips therefore caused a full `commit` (4 `querySelectorAll` over the root, `closest(".semio-scope")` per element, measure of every box) per frame.

After:
- `tracked` (line ~215, built at ~441 in `commit`): the set of box elements, all their ancestors and the interactive delegates. Observer (line ~224):
  - attribute records on untracked targets are dropped unless the attribute is a marker attribute (`MARKER_ATTRIBUTES`: `role`, `tabindex`, `data-shared-border-*`) or a `class` change that makes the element a `.shared-border-cell`;
  - childList records are structural only if an added/removed element matches or contains a box-relevant selector (`boxRelevantSelector`; SVG-namespace subtrees are matched only at their root, no `querySelector`); otherwise they are geometric only when the target is tracked, else dropped.
  - Records are filtered before the costly `closest` overlay test.
- `candidates` (line ~216, used at ~394): cached owner/frame/chip/cell lists, invalidated by `schedule()` (structural), so state and geometry commits no longer run `querySelectorAll` over the root.
- `commits` counter (public) for tests.
- `attachSharedBorderRoots`: skips `querySelector` on SVG-namespace nodes and sweeps only when a removed node is or contains a box (before: every removal swept every root).
- Stale-position closure (coordinator follow-up): (1) `scheduleState` now marks geometry dirty, so every hover, focus or marker commit re-measures all boxes (owners and neighbours included) before painting; stated silhouettes are never drawn from stale geometry. (2) After each commit the parent of every box and that parent's element children are ResizeObserver-observed (`watched` set, cleared on structural change, pruned in `restore`), so a sibling's size change that moves a box in flow schedules a geometry commit. A mutation inside an untracked non-box subtree that moves a box without resizing any observed element still waits for the next state commit.

Tests (`E/🗂️WindowChrome/shared-border.test.ts`, "compositor commit economy"): 300 boxes + 20,000+ nodes, 1,000 animated mutations (`d`, `data-time`, `aria-label`, style, innerHTML swaps) => commits stays at 1 and no frame is queued; a box/ancestor mutation commits once without any root `querySelectorAll`; a new box and a state attribute each commit once; "a sibling that grows above a hovered button" (the hover re-reads the button's new rect, parent and siblings are resize-observed). The old test asserting cached geometry on state commits was rewritten to the new contract.

## 2. CardStack interlock (`E/🃏️CardStack/🟦️.tsx`)

Before: every `measure` reset lift, bands and min-block-size (writes) and rewrote them, so `--card-stack-lift` flipped 0px -> 2px each pass.

After:
- `put` (line 22): writes a style property only when its value differs.
- Settled signature (lines ~51-61, ~95): rects of container and cards, row heights and chip spans are read before any write; if equal to the signature stored at the end of the last pass (same card elements), `measure` only syncs the ResizeObserver set and returns: zero writes. A changed layout re-measures as before.
- Test (`🟦️.test.tsx`): "settles: further measure passes of an unchanged stack write no style at all" (3 resize passes, MutationObserver records empty, style attributes identical, measure passes proven by rect reads).

## 3. Staffage clock tick (`S/🔨️modules`)

Profile: one figure frame = `evaluate` + `depict` + `inlineSvg` about 7 ms cold (jsdom measurement); once a loop is cached, the remaining per-frame cost per visible figure is a React render, `JSON.stringify` of options, and HTML re-parsing via `dangerouslySetInnerHTML` (the childList mutations). `tick` time = all subscriber renders, synchronous in the listener.
- `⏱️clock/🟦️.ts` `useHeldTime(clock, active)`: replaces two duplicated hooks (`🧍️figure`, `📍️placed`). An inactive figure (offscreen via IntersectionObserver, or showing a pose which ignores time) does not subscribe at all; it catches up when it becomes active.
- `📍️placed/🟦️.tsx`, `🧍️figure/🟦️.tsx`: `active = visible && query is not a pose`.
- `📍️placed/🟦️.tsx`: depiction options memoised separately from the per-frame query (were rebuilt every frame).
- `🧮️pipeline/🟦️.ts` `keyOf`: `sceneOf` caches the JSON key of its options object (WeakMap).
- `🧮️pipeline/🧬️svg/🟦️.ts` `useSvgMarkup`: replaces `dangerouslySetInnerHTML`; a parsed `<template>` per markup (LRU 1,024) is cloned and swapped with `replaceChildren` in a layout effect, and nothing is written when the element already shows the markup. Looping clips repeat a few markups, so a frame costs a clone instead of an HTML parse. Output is byte-identical (test).
- Animation behaviour unchanged: same clock, same frame quantisation, same queries.

Tests (`S/🧪️tests/⏱️clock-determinism/🟦️.tsx`): held time (no render while idle, exact catch-up), pose figure does not re-render on a running clock, markup of frames 0/4/9/4 equals the parsed `inlineSvg`.

## Runs
- `vitest-ui.config.ts` (`🧱️elements`, `border-presentation`): 723 passed, 4 failed (the known Chromium-only Tree `text-flow` x3 and Layout `component` x1); non-suite helper files fail as before.
- `vitest-teaching.config.ts`: 275 passed, 1 failed (`catalog-loading`: 151 ready, 1 invalid of 152; asset catalog content, none of the touched files).
- Not measured live (coordinator verifies).

## Round 2: live re-check (tick 10 s, commit 3 s, pending never clears)

Profiling note: headless Chrome could not be started in this sandbox (it aborts at launch), so the live profile was not reproduced here. Numbers below are from jsdom micro-measurements plus code reading; the coordinator's live run is the arbiter.

### Clock tick
- Subscribers per tick before: one per visible `PlacedFigure`/`StaffageFigure` (React `useSyncExternalStore`), each doing a synchronous React render in the clock listener (options/JSON keys, `inlineSvg` string compare, `dangerouslySetInnerHTML` HTML parse, style props). Cached scene work is negligible (1.4 microseconds per figure frame measured), so the cost was React plus HTML re-parse, multiplied by every figure on every tick (`AMBIENT_FPS` = 12).
- After: `⏱️clock/🏁️frames/🟦️.ts` `driveFigure`: one clock subscription per clock, one shared IntersectionObserver, visible and animated figures only; a figure that turns visible steps at once. `📍️placed/🟦️.tsx` renders the element once per prop change and paints through `place` + `paint`: markup via `writeSvgMarkup` (template clone, written only when it changed), left/top/width/height/opacity/transform-origin/transform written only when their value changed. No React render per frame (test: Profiler commit count constant over ten ticks, 40 figures, exactly one `clock.subscribe`; offscreen figure frozen and catching up on becoming visible).
- Remaining per-frame cost is DOM: in jsdom one 90-node markup swap = 3.6 ms (jsdom is much slower than Chromium); in Chromium expect tens of microseconds per figure. `StaffageFigure` (used by `RowPreview`) still renders through React with the held-time hook; the overview's previews that use it show poses (static, no subscription).

### Child and style churn on `staffage-card-stack`, DIV and SECTION ancestors
- `staffage-card-stack` is the `[data-overview-card-stack]` window of the archive card; the SECTION is the card, the DIV the grid cell: exactly the elements `useCardInterlock` writes (`min-block-size`, `--card-stack-*`, `margin-block-start`) and the compositor may touch (`position`, overlay svg). The writes are a CardStack re-measure loop fed by ResizeObserver notifications (initial-observation callbacks and sub-pixel noise count as resizes).
- Fix: both `useCardInterlock` and the compositor now ignore ResizeObserver entries whose rounded content size equals the last seen one (first sighting records, never schedules). Together with the settled signature, a settled stack writes nothing. Tests: "a resize notification schedules a commit only when an observed size really changed…" and the earlier CardStack "settles" test.
- Cause not proven live; if churn persists, the next suspect is a size that really changes every frame (check `signature()` inputs: card and chip rects).

### Compositor pending marker
- `data-shared-border-pending` is outside the observer's `attributeFilter` and now covered by a test (set/remove on the root commits nothing). It stayed set because commits kept being queued; the sibling observation added in the last round (animated siblings resize every frame) was a plausible driver and is removed: only the parent of every box is observed, and a stale position of a box moved by an untracked sibling is closed by the state-commit re-measure.

### Runs
- UI suites (WindowChrome, CardStack, elements, border-presentation): only the 4 known Chromium-only failures remain.
- Staffage: all pass except the `catalog-loading` content failure noted before.
