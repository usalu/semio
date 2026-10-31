# Desktop Card Grid & Scrollable Multi-Row Grids Across UI Systems

## Root Cause
Across the layered overview UI systems (`@teaching/staffage-react`, `@semio-tech/course-react`, `@semio-tech/ui-react`, `🏢️semio-tech/🎡️play`, `♻️mit-bestand/🧺️demonstrator`, and `🎓️teaching/🏛️architecture/⚡️energy`), desktop and tablet viewports were either:
1. Switching from `mode="strip"` (the multi-column card grid) to `mode="swipe"` (the mobile single-item view) whenever the total height of the grid rows exceeded the viewport height (`size.height < rowCount * ROW_MIN_PX` in `RootLevel` and `CategoryPage` of Staffage, and `matched.short[grid]` in `HomeOverview` of Course), or
2. Using `minmax(0, 1fr)` row tracks on a non-scrolling `pointer-events-none` overlay, which squished or clipped cards when there were many rows or a shorter desktop window.

## Changes Applied
1. **Shared `LayeredOverview` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx`)**:
   - Updated the `strip`-mode `[data-layered-overlay]` container to `pointer-events-auto overflow-x-hidden overflow-y-auto` so card grids scroll vertically whenever their rows exceed the viewport height.
2. **Staffage (`🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react`)**:
   - `🔨️modules/🥞️levels/🟦️.tsx`: Added optional `minPx` floor to `tracks(weights, minPx)` producing `minmax(${minPx}px, ${weight}fr)`.
   - `🔨️modules/🏠️overview/🟦️.tsx` (`RootLevel`): Locked `mode` to `breakpoint === "mobile" ? "swipe" : "strip"`, aligned `cells` with `placementOf(id, columns)`, and applied `ROW_MIN_PX` (`150px`) row floor.
   - `🔨️modules/🗃️browse/🟦️.tsx` (`CategoryPage`): Locked `mode` to `breakpoint === "mobile" ? "swipe" : "strip"` and applied `CATEGORY_ROW_MIN_PX` (`190px`) row floor so all 24-card categories stay in the 4-column desktop / 3-column tablet grid and scroll smoothly.
   - `🎨️.css`: Added `overflow-y: auto; overflow-x: hidden; pointer-events: auto; overscroll-behavior: contain;` to `.staffage-grid`.
   - Added unit test suite `🎯️targets/⚛️react/🧪️tests/🗂️grid-scroll/🟦️.tsx` and rebuilt the release site (`🌐️site/📦️packages/🟦️typescript/dist/release`).
3. **Course (`🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react`)**:
   - `🔨️modules/🏠️home/🟦️.tsx`: Updated `homeLayout` so `mode` is `"swipe"` only when `matched.narrow` is true (`<= UI_MOBILE_MAX_WIDTH_PX * scale`), keeping desktop and tablet on `"strip"`, and added a row floor `min(${HOME_GRID_ROW_HEIGHT_PX}px, 100%)` to `homeTrackTemplate` for `--course-home-rows`.
   - `🎨️styles/🏠️chrome-and-home/🎨️.css`: Added `overflow-x: hidden; overflow-y: auto; pointer-events: auto;` to `.course-home-grid`.
   - `🧫️fixtures/🏠️home-grid/🔣️.json`: Updated the desktop/tablet short-window layout vectors (`900×400`, `812×375`, `1280×720` at `scale: 1.5`) to `"mode": "strip"`.
4. **Play, Demonstrator, and Energy Presentation (`🏢️semio-tech/🎡️play/🟦️.tsx`, `♻️mit-bestand/🧺️demonstrator/🟦️.tsx`, `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️.tsx`)**:
   - Updated grid row templates to `minmax(min(10rem, 100%), 1fr)` and card cell wrappers to `min-h-0 overflow-x-hidden overflow-y-auto` so multi-row card grids scroll cleanly on desktop and tablet.
