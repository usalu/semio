# Round 2 — glass restored, backdrop pages borderless, device-pixel snapping

## 1. Glass restored, backdrop borders removed at the root
- `🖱️ui/🎨️styling/🖌️ui/🎨️.css`: the opaque `[data-layered-veil]` override is deleted; the veil is the original `ui-veil` again (`blur(var(--veil-blur)) saturate`, alpha `--veil-alpha`; live: `blur(8px) saturate(1.45)`, alpha 0.4).
- New context `data-layered-backdrop` (set by `🥞️LayeredOverview` on `[data-layered-strip]` only while no page is opened, i.e. while the glass exists). In that subtree: every `border-color` / `outline-color` (also `::before`/`::after`) is transparent (`!important`) and compositor overlays / silhouette strokes (`[data-shared-border-overlay]`, `[data-window-silhouette-border]`, `[class*="window-silhouette-border"]`) are `visibility:hidden`. Content still shows, blurred. Result (375 and 1440 screenshots): U-gaps, chip bands and edge hints show only soft blur, no bordered boxes, no side-edge fragments.
- Pressed-card hole: the revealed backdrop page is the same strip, so it is now crisp but borderless (consistent with the rest of the backdrop); the tinted-card rule is unchanged. A test asserts the attribute (`🥞️LayeredOverview/🧪️tests/🧩️component`).

## 2. Device-pixel snapping
- `--device-pixel` (1 / 0.8 / 0.667 / 0.5 / 0.333 px by `min-resolution` buckets) on `:root`. Navbar, Footer heights, `chromeHostedOpenPanelPositionStyle` and `[data-slot=navbar|footer] --panel-header-height` use `round(nearest, var(--navbar|footer-height), var(--device-pixel))` (28.8 px -> 29 px at DPR 2; geometry/shell tests updated).
- `deviceSnappedRect()` (WindowChrome `shared-border.ts`): every edge rounded to a device pixel; used by `measureWindowSilhouetteMetrics` (stack, chips, gaps, caps) and the BorderCompositor (windows, frames, edge collection), so silhouette coordinates and bar edges coincide.
- Course footer is content-height (30.797 px, the actual source of the 781.203 scroller end): `.course-footer { height: calc-size(auto, round(up, size, var(--device-pixel))) }` -> 31 px, top 781, closing line 782, chip bottoms 780/782 (before: 782.2 vs 780.9).
- Concurrent writers had reverted `createSharedBorderSnap` tolerance to 0.5 and `snap.cases.json` to old cases: restored 0.75 and fixed the two "two lines" cases (100/101, 0/0.5/1).

## Live (DPR 2 pane; probe count / overlaps / pending)
| view | 375x812 | 1440x900 |
|---|---|---|
| home | 0/0/false | 0/0/false |
| #energy | 0/0/false | 0/0/false |
| #physics | 0/0/false | 0/0/false |
| run (Medium), top/bottom | 0/0, 0/0 | 0/0 |
Note: the pane's rAF stops when it is hidden/contended by other agents (probe then shows `pending:true`, even transient false violations); a screenshot wakes it, so probes ran after a screenshot. My first two tabs were closed by others.

## Tests
- UI set (LayeredOverview, WindowChrome, Navbar, WindowSilhouette, owned-locale-detector-retirement, styling): 702/702.
- Course config: 1125/1125 (the 4 "failed files" are helper modules without suites).

## Files
`🖱️ui`: `🎨️styling/🖌️ui/🎨️.css`, `🧱️elements/🥞️LayeredOverview/🟦️.tsx` (+ component test), `🧱️elements/🗂️WindowChrome/🟦️.tsx`, `shared-border.ts`, `🧱️elements/🔲️WindowSilhouette/shared-border.ts`, `snap.cases.json`, `🧱️elements/🔝️Navbar/🟦️.tsx`, `🧱️elements/🔚️Footer/🟦️.tsx`, `🔨️modules/⚓️panel-anchor/🟦️.ts`, `🎨️styling/🧪️tests/🧩️suite/📐️geometry/🟦️.ts`, `🧪️tests/🧪️owned-locale-detector-retirement/🎨️shell-components-1/🟦️.tsx`; course `🎯️targets/⚛️react/🎨️styles/⚖️footer-and-forced-colors/🎨️.css`.
