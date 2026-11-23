# Lane `uitests` handoff

The React acceptance tests assert the rectangular frame model. 570 of 570 tests of `🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` pass against the current contract, compositor and elements code.

## What changed (all under `🧰️framework/🔨️modules/🖱️ui/🧪️tests`)

Mechanical renames in every file: `WindowSilhouette*`/`windowSilhouette*` to `WindowFrame*`/`windowFrame*`, `WINDOW_SILHOUETTE_*` to `WINDOW_FRAME_*`, `data-window-silhouette*` to `data-window-frame*`, `window-chrome-silhouette-border` to `window-chrome-frame-border`, `mode-dock-silhouette-border` to `mode-dock-frame-border`, `window-silhouette-*` classes and clearance variables to `window-frame-*`, `data-silhouette-remeasure` to `data-frame-remeasure`. The harness type bag (`🧰️harness/🟦️.ts`) lost `windowSilhouetteOutline` and `windowSilhouetteOutlineViolations`.

Rewritten to the new laws:

- `🧊️shell-components-3`: the polygon path test became "the border path is the inset rectangle whatever the chips are" (all chip set combinations, `M0,0 H200 V100 H0 Z` and `M1,1 H199 V99 H1 Z`). The geometry test asserts `frame`, `borderPath`, `chipRegions` (no `kind`), `safeClearances`, containment of the whole frame including the chip rows, and the absence of `contentClipPath`, `contentPath`, `bodyRegion`, `contentRegions`, `glassRegions`. The pending test asserts a full frame, no chip regions, the clearances and no clip path. The outline-violation test was replaced by an independent `polygon-clipping` oracle: chip overlays lie inside the frame, are pairwise disjoint, and the uncovered chip row is exactly frame area minus chip area (content). A test asserts the old outline and silhouette exports are gone.
- `🛠️control-chrome-2`: chip measurement tests without gap elements (RTL order, bottom chips on the `window-chrome-footer` row, nested frame chips ignored); the panel test asserts the cap row is an absolute pointer-transparent overlay inside the frame and that the content plane carries no clip path.
- `🪟️uiintroduction-appearance-1` and `🎬️.../🔸️uiintroduction-appearance`: chips are glass overlays one level above the content (`data-level="menu"` over a `dialog` content plane), no gap or footer-gap slots, the footer center chip is `window-chrome-footer-center` itself, the footer row is a pointer-transparent absolute overlay. The border tests assert the frame outline as segments: the four full frame edges plus exactly the inner edges of each chip, with no notch when the Next chip is absent.
- `📐️shell-components-4`, `🗂️shell-components-5`, `🎨️shell-components-1`, `🧭️uiintroduction-veil-and-elevation`: no `mode-dock-tab-gap`, tab bar is an absolute pointer-transparent overlay with pointer-active chips, content plane unclipped. The "exposes its top cutout to the canvas" test became "pointer down in the uncovered tab row goes to the window content, never to the floor" (the stack is no longer `pointer-events-none`). The full-span snapping/outline tests are gone.
- `🧲️contextmenu-and-more`: U-gap transparency test replaced by in-frame glass title chip with no gap element or gap CSS; new test that a submenu wing is its own `[data-window-frame]` (`context-menu-submenu`, no chip) next to the primary frame with a rectangular border; the menu scroller is `context-menu-primary` inside a frame clipped by `overflow-clip`.
- `🌳️owned-locale-detector-retirement-and-more`: CSS laws (no gap rules, no `window-silhouette`, `window-frame-content-plane`, `--window-frame-top-clearance`).
- `📦️control-chrome-1`, `🔎️shell-components-10`, `🧱️shell-components-9`, `🪄️tree-helpers-1`, `📚️shell-components-8`: gap assertions inverted to absence, silhouette wording removed.
- `📚️storybook-new-stories/🟦️.ts` (Playwright, not run here): reduced-transparency and forced-colors tests now assert an opaque chip inside the frame, zero gap elements, no content clip path and a content plane covering the frame; a new test asserts translucent backdrop-blurred chips by default.
- `🧫️fixtures/📐️dock-axis-geometry/🔣️.json`: `windowBodyBlockInset` is `frame-clearance`.

## Verified

Native runner (`repo` has no `ui-react` project here, see compositor handoff), from this folder:

- `sh ./compositor-vitest.sh "owned-locale-detector-retirement"`: 1 file, 570 passed, 0 failed.
- `sh ./compositor-vitest.sh "🧪️tests/"` (every file of the stock config whose path contains `🧪️tests/`): 10 failures in 6 files, none from my edits:
  - `🧪️tests/🖼️icon-render-camera` `perspective-fit-zoom` (camera math, nothing to do with frames; not touched by this migration).
  - lane `elements`: `🎨️Canvas/🧪️tests/shared-border-dock.test.tsx` (measured dock outline), `🌳️Tree/🧪️tests/🔤️text-flow` (3), `📑️Tabs/🧪️tests/🧩️component/boundary.tsx` (2), `📐️Layout/🧪️tests/🧩️component` (2), `🥞️LayeredOverview/🧪️tests/🧩️component` (1).
- All 18 changed `.ts`/`.tsx` files parse with esbuild.

## Open

- A recurring `TypeError: view?.removeEventListener is not a function` is logged by jsdom after the run, thrown from `BorderCompositor.dispose` (`🗂️WindowChrome/shared-border.ts:738`) during teardown. Tests still pass; lane `compositor` should guard the call.
- `modeDockChromeGridPlacement` in `🧊️shell-components-3` still returns a `gapCol` flex spacer column; the tests keep asserting it. If lane `elements` removes it, delete those two tests.
- The chip-row `grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]` footer assertion in the introduction tests follows the compositor handoff.
- Playwright story tests are unrun (need Storybook); the story `🖱️ui⚛️react-mode--content-through-glass` must have a chip and a content plane inside one `[data-window-frame]`.
- Rust `🔬️targets-wgpu-*` tests and the vitest config include list belong to other lanes (the config now lists the WindowChrome and WindowFrame tests).
