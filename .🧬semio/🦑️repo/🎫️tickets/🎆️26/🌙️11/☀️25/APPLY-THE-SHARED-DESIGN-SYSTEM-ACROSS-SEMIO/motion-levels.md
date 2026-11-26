# Motion and Presentation Surfaces

## Implementation

- Reveal owns one opaque window surface on the scaled slide canvas above the base viewport. Ordinary embedded media shares that owner. Artwork still supplies its own pixels. The enlarged view uses the window fill, and overlay controls, page navigation and slide numbering use shared pane glass and chip tint. Existing shared-border groups keep their outline and seams.
- The teaching clip renderer uses the same opaque window swatch for the Manim camera, default content frames and video normalization padding. Annotation plates, subtitle overlays and BIM label chips use the existing pane chip helper. The browser handoff uses one window body throughout the focus animation and lets its title chip overlap that body.
- Standalone Animate HTML separates the base outside the canvas from the actual window and puts the hairline around the canvas. The Vello/WGPU scene and deck video routes use the generated dark window swatch. Animate’s SVG title card uses that same swatch and the corresponding foreground.
- Grasshopper teaching cards use the light window swatch. Depicted Rhino and Grasshopper artwork and the captured external Zukunft Bau site retain their authored colors. The optimization sampling thumbnail color belongs to the room illustration. Base or window-colored masks in authored drawings still serve as masks.

## Verification

- The language-neutral clip cases describe light and dark base, window and chip colors. Manim’s Cairo renderer and Pillow verify visible content pixels and the chip composite over the content frame. Both surface and style suites passed all 21 tests using the managed Python 3.12 environment.
- Nine authored Cairo and geometry checks passed across the Excel reference clip, Grasshopper capsule layout and BIM model counters. The Grasshopper replay fixture now uses the authored named transition helper. The separate Excel annual-result check failed at the same leading-digit threshold with the authored module from HEAD loaded into memory. Its oracle is tracked in the clip audit.
- The full teaching MP4 integration passed after the UI sources settled. It exercised the real Chromium focus and restore transitions, the production encoder, decoded body frame order and pixel tolerances, and the audio handoff.
- A square source ran through the real chapter encoder with a controlled transition stub. PyAV decoded six frames. The letterbox at both edges of every body frame measured exactly `(6, 26, 31)`.
- The authored BIM exchange label and Grasshopper instruction card were rendered through Manim. Pillow measured dark content `(6, 26, 31)`, the BIM chip `(37, 54, 57)`, and the light teaching card `(233, 230, 215)`.
- The production teaching browser geometry test passed using Chromium screenshots and the independent Pillow pixel oracle. The opaque body computed `backdrop-filter: none` and the overlapping chip retained its blur. The adjacent-window oracle also passed. It checks that one active or hover stroke is painted an inset inside its winning clipped owner, then samples the corresponding screen pixels.
- Presentation React’s direct repo-managed Vitest suite passed 143 tests across 15 files, including the new owner and control regression. The normal project target hit `BUILD_SOURCE_UNPARSEABLE` resolving the existing `@semio-tech/presentation-react/🎨️.css` dependency in the story, reference `fcddc746`.
- Chrome mounted the real 33. Projektetage deck via its Vite config and resolved Base `(0, 17, 23)` and Window `(6, 26, 31)`. On its catalogue-labels slide an interaction exposed the pane chip with glass alpha `0.76` and `blur(16px) saturate(1.45)`.
- All modified Rust modules and test modules parse on the repository-pinned nightly. A typed Rust check measured the generated window and foreground colors against the same expected bytes used by the browser and Manim checks.
- Native Animate compiled the production mutations, editor, scene renderer and route libraries with the repository-pinned compiler. All three focused native checks passed for the static HTML bundle, Vello scene MP4 and decoded last-frame window color, and the independent XML title-card oracle. After stale mutation contract fixtures were aligned with the current protocol, the full mutations suite passed 105 tests.
- The production Animate entry points emitted HTML, SVG, PNG and AVC1 MP4. Chrome, XML parsing, Pillow and PyAV checked the output surfaces and frames. The video suite confirmed cache identity for the painted picture and duration, the repainted Metal/Vello center pixel, playable writer packets, exact GIF palette, LRU survival and concurrent publication. The rebuilt video crate passed all 19 tests after correcting frame capture duration. The separate [frame-recorder audit](animation-and-clips.md) records exact frame timestamps and colors.
- The focused native editor proof used the current mutation owner, schema, host and editor slices. The real Add Tile handler minted different IDs for different admitted authoring seeds and preserved the same ID on replay. The second tile appended at index 1. The real export handler emitted one valid three-scene program with 105 frames at 15 fps, Window RGBA `[6, 26, 31, 255]`, and zero document mutations.
- The aggregate editor and v1 suites compiled Rust source. Their full test binaries could not complete linking on this Mac after serial native retries. The mounted acceptance fixture hit the same host link limit. The scoped editor handler proof and complete native video and mutation suites ran successfully.
- Chrome rendered the Animate stylesheet with attributes extracted from the Rust HTML emitter at wide and tall viewport sizes in both appearances. Pillow measured the expected base and window swatches in all four PNGs. The independent Rust tests cover emitted HTML, XML title-card paint, video-program RGBA and a decoded last frame.

The seam fixture previously probed only the exact geometric boundary and used passive windows. The compositor deliberately keeps one hairline inside the winning clipped owner. The updated fixture marks both windows interactive and probes the boundary and its two adjacent inset positions to catch duplicate strokes or a missing winner. The dedicated shared-border suite passed 55 cases using repo-managed Vitest with jsdom.

## Changed Paths

Under `🧰️framework/🛍️products/🎤️presentation`

- `🎯️targets/⚛️react/🔨️modules/🎞️deck/🟦️.tsx`
- `🎯️targets/⚛️react/🔨️modules/🔗️shared-border-chips/🟦️.tsx`
- `🎯️targets/⚛️react/🔨️modules/🫳️interaction/🟦️.tsx`
- `🎯️targets/⚛️react/🔨️modules/🫳️interaction/🧲️interactive-disposition/🟦️.tsx`
- `🎯️targets/⚛️react/🔨️modules/🎭️morph-views/🟦️.tsx`
- `🎯️targets/⚛️react/🔨️modules/🖼️figure-scroll/🟦️.tsx`
- `🎯️targets/⚛️react/🎨️styles/🌱️intro-arrangement-and-reveal-chrome/🎨️.css`
- `🎯️targets/⚛️react/🎨️styles/👻️morph-keyframes-ghosts-and-pdf/🎨️.css`
- `🎯️targets/⚛️react/🎨️styles/🎥️media-teaser-and-figure-scroll/🎨️.css`
- `🎯️targets/⚛️react/🎨️styles/🧱️arrangement-canvas-and-embodiments/🎨️.css`
- `🎯️targets/⚛️react/🎨️styles/🔲️interaction-handles-and-controls/🎨️.css`
- `🎯️targets/⚛️react/🎨️styles/🖱️interaction-selection-and-enlarge/🎨️.css`
- `🧪️tests/🖱️interaction-dom/🟦️.tsx`

Under `🎓️teaching/🏛️architecture`

- `⚡️energy/🏗️authoring/🔤️fonts/🐍️.py`
- `⚡️energy/🏗️authoring/🎨️visuals/🐍️.py`
- `⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🎨️semio-style/🐍️.py`
- `⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🎨️semio-style/🔣️.cases.json`
- `⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🐍️.py`
- `⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🎨️surface/🐍️.py`
- `⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟦️.tsx`
- `⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🧪️tests/seam.mjs`
- `⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🧪️tests/seam.cases.json`
- `⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/📐️geometry/🧪️tests/🐍️.py`
- `🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🐍️.py`
- `🏗️bim/🎬️clip/🎨️visuals/🧾️panels/🐍️.py`
- `🏗️bim/🎬️clip/🔌️exchange/🐍️.py`
- `🏗️bim/🎬️clip/🔗️consistency/🐍️.py`
- `🏗️bim/🎬️clip/🧩️models/🐍️.py`
- `🏗️bim/🎬️clip/👁️representations/🐍️.py`
- `🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py`
- `🦗️grasshopper/🎬️clip/💊️capsule/🧪️tests/📐️layout/🐍️.py`
- `🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py`
- `🎯️optimization/🎬️clip/📉️extrema/🐍️.py`
- `📊️excel/🎬️clip/⚡️energy/🐍️.py`
- `📊️excel/🎬️clip/📍️references/🐍️.py`

Under `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1`

- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`
- `🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs`
- `🪆️subsets/✳️any/✏️editor/⚙️engine/🎨️presentation.css`
- `🪆️subsets/✳️any/✏️editor/⚙️engine/🧪️tests/🔬️compiler-unit/🦀️.rs`
- `🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️export-video-from-deck/🧪️tests/🔬️unit/🦀️.rs`
- `🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs`
- `🪆️subsets/✳️any/🚪️io/🧪️tests/🧫️fixtures/🔣️title-cards.json`
- `🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs`
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲resize-source-frame/🧪️tests/🧪️no/🦀️.rs`
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️replace-source/🧪️tests/🧪️no/🦀️.rs`
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-tile/🧪️tests/🧪️no/🦀️.rs`
- `🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-tiles/🧪️tests/🧪️no/🦀️.rs`

Cargo lockfiles updated for the existing internal styling dependency in the Animate presentation, Animate bridge, hub, and fixture-sweep workspaces. The ticket includes `check-animate-levels.py` to reuse the pinned native build and `replay-animate-mutations.py` to link the mutation test from a frozen native dependency receipt.
