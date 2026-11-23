# 🧯 Story suite and teaching Python fixes

## 1 Storybook browser suite cannot load
- Root cause: `.storybook/🧪️tests/🧪️browser-runner/📚️suites/🟦️.ts` imported `✏️s/🧪️tests/🎭️storybook-end-to-end/🟦️.ts`. That path never existed in git. It was the old `.storybook/s-end-to-end.spec.ts` (3 tests booting the plugin host shell `s`), deleted in 9869c6e99b6 and never ported. Not a move, a removal, so the repo graph failed with `BUILD_SOURCE_UNPARSEABLE`.
- Fix: dropped the import and the matching entry of the discovery fixture; `minimumTests` 143 to 135 (see below).
- Also fixed: `storybook-new-stories` listed 10 story ids that exist nowhere (MobilePanel x2, ShellDisplayPanel x2, ShellSettingsPanel x2, Providers Transaction, ChromeCompact, ChromeLabelPolicy, GlassTier; no component or story at HEAD or before the migration). Removed them (and `Shell*Panel/Dialog` in its header). Discovery now lists 135 tests in 12 files (was 145 with the 10 dead ids; `repo`-free `--list` through bun).
- Run: `story-playwright.sh` (bun + Playwright, `PLAYWRIGHT_BROWSERS_PATH` = project cache, dev Storybook `ui` scope on 6090 from `os-shell-live-recipe.md`). Filter by `--grep` (file-path arguments do not match the emoji paths).
- Result (ui stories: WindowChrome, Window, Panel, PanelTabBar, CardStack, OverviewCard, ContextMenu, Popover, Select, Tabs, Canvas, Layout, Mode, window-frame glass, reduced-transparency, forced-colors, celebrated icons, panel chrome handle): 66 passed, 2 failed. No failure comes from the window-frame migration.
- Still failing (unrelated):
  - `Mode reserves themed separators before distributing nested React panel extents`: the React dock has flush composition since 3d62b2ef22c (`ResizableHandle` is 0 wide, canvas and stack body inline padding 0). The oracle (`🧪️tests/🧫️fixtures/📐️dock-axis-geometry/🔣️.json`, tokens `--spacing-single`) is also read by the wgpu dock Rust tests, so the fixture needs a cross-language decision; not changed.
  - `TiledMapHost image render`: needs the wasm engine, not served in the `ui` scope.
- Not served by the `ui` scope and therefore not run: puzzle, cad, deck, os hosts, `styling/*` stories (earlier run: they failed with "Expected one Storybook story" or timeouts, scope not loaded).

## 2 Geometry oracle 22.4 vs 24 px
- Root cause: the Python oracle took the title row from `panelHeaderHeightUiSpacing` (7 x 3.2 = 22.4) while the one rule that sizes the row in the product is `--chip-row-height: round(nearest, max(1.5rem, 7 * --ui-spacing), --device-pixel)` = 24 px, and the shell bands are snapped by `--navbar-band-height` / `--footer-band-height` to 29 px at the 1600 px reference. Fixed JSON contracts (`chrome` 28.8/28.8/22.4, focus rect and viewport) and canvas constants in the React page repeated the stale numbers.
- Fix: the CSS token is the truth. `📐️geometry/🐍️.py` reads the floor and the spacing multiple from that one declaration in `🖌️ui/🎨️.css` and snaps like CSS (`geometry_metrics()`: navbar 29, footer 29, title 24). The contract `chrome` (29/29/24), `focus.rect` (y 32.2, h 835.6), `focus.viewport` (y 59.4, h 805.2), schema consts and the interaction `titleHeight` (24) follow it. `🌐️browser/🟦️.tsx` derives canvas top/height and footer top from `contract.chrome` instead of 28.8/842.4/871.2. Fixtures: shell, grid canvas, window and frame cases (chip heights, safe top clearance 24). The test asserts navbar/footer from the oracle (no tolerant wrapper) and that the contract chrome equals `geometry_metrics()`.
- Files: `📐️geometry/🐍️.py`, `📐️geometry/🧪️tests/🐍️.py`, `📐️geometry/🧪️tests/🔣️.cases.json`, `🔣️.json`, `🧬️schema.json`, `🧪️tests/🔣️.cases.json`, `🖱️interaction/🔣️.json`, `🌐️browser/🟦️.tsx` (all under `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation`).
- Verified: geometry module 8/8 OK (was 1 failure); presentation `🧪️tests` module 15 tests OK, interaction 21 tests OK; `🎨️visuals` semio-style 10/10 OK.

## 3 Double `pointerdown` at 4 fps
- Root cause: every frame renders in a fresh page. A press that spans two consecutive sampled frames was replayed in the second frame as a new `mouse.down()` (the held state was restored only when the previous frame was pressed and the current one released), so the second page logged a second `pointerdown`. Dependent on press duration versus sampling, not on geometry.
- Fix in `🌐️browser/🟨️render.mjs`: the held button is restored (down, events cleared) whenever the previous frame was pressed, also when the current one stays pressed; a new press is only issued when the previous frame was not pressed; the early release on a changed opened path stays tied to a released frame. One logical press now logs `pointerdown, pointerup, click` once.
- Verified: `CursorNativeFrameTest.test_four_fps_transitions_capture_current_native_clicks_at_every_required_level` passes; whole interaction module 21/21 OK.

## Files changed outside the teaching folder
`.storybook/🧪️tests/🧪️browser-runner/📚️suites/🟦️.ts`, `.storybook/🧪️tests/🧫️fixtures/🌐️browser-discovery/🔣️.json`, `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-new-stories/🟦️.ts`.
Scripts kept in the ticket: `story-playwright.sh`, `story-probe.mjs`, `story-fixes-measure.py`.
