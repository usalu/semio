# Staffage Desktop Grid Scroll

Desktop and tablet Staffage levels stay on the card grid (`data-mode="strip"`). Only a phone (`max-width: 767px`) uses `data-mode="swipe"`.

`tracks` takes an optional pixel floor. The root grid passes `ROW_MIN_PX` (150). A category grid passes `CATEGORY_ROW_MIN_PX` (190). `.staffage-grid` scrolls vertically (`overflow-y: auto`, `pointer-events: auto`, `overscroll-behavior: contain`) when those rows exceed the window. Root cells follow `placementOf` and the active column count.

## Verification

- Vitest `@teaching/staffage-react`: 16 files, 104 tests, exit 0. `🗂️grid-scroll` covers 1280×400, 1280×640, tablet 900×400 (strip, 12 root cells, 24 human cells, `minmax(150px` / `minmax(190px`) and 390px (swipe).
- `tsc --noEmit` in `🎯️targets/⚛️react/📦️packages/🟦️typescript`: exit 0.
- Release site build: exit 0, `built in 3.09s`, output `🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/📦️packages/🟦️typescript/dist/release`. The CSS bundle contains `overscroll-behavior:contain` and the script bundle contains the mobile-only swipe ternary.
