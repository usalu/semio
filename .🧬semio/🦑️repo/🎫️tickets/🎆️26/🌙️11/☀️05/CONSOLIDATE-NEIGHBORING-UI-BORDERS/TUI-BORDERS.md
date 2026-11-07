# TUI Border Consolidation

Raised and compact tab outlines, neighboring tiled windows, and the navbar/footer shell bands share boundary cells. The window compositor records semantic perimeter writes, merges cardinal connections into light or heavy junctions, and gives hover priority over focused or active state before overlay painting.

The pure and mounted solvers, docking previews and splitter hit testing use the same shared-cell allocation, including zero- and one-cell extents. Splitters enter keyboard focus order; Tab and BackTab select successive splitters, and arrow keys emit one-cell resize signals.

Language-neutral fixtures cover both axes, narrow extents, chrome emphasis and contour geometry. An independent Taffy solver checks docking geometry; tiny-skia supplies raster checks for chrome and junction connectivity. Engine tests cover shell boundaries, pointer dragging and keyboard traversal.

Validation is awaiting the parent task’s coordinated framework test gate.

## Touched Paths

- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧲️docking/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/📏️layout/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🖥️chrome/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🎬️scene/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🔲️cell/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/⚙️engine/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/⚙️engine/🖌️paint-and-gutters/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/⚙️engine/🎯️construction-and-focus/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/⚙️engine/⌨️input-dispatch/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🎯️targets/⌨️tui/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🎯️targets/⌨️tui/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🎯️targets/⌨️tui/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🔬️engine/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🔬️chrome/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🔬️unit/🪟️tui-layout-and-window-laws/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🔬️unit/🎨️tui-widget-paint-and-window-silhouette-laws/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🖼️window-contour/🦀️.rs
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/📏️shared-splitters/🥒️.feature
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🖼️chrome-emphasis/🥒️.feature
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🧫️fixtures/📏️shared-splitters/🔣️.json
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🧫️fixtures/🖼️chrome-emphasis/🔣️.json
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🧫️fixtures/🖼️window-contour/🔣️.json
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧪️tests/🧫️fixtures/🖱️pointer-routing/🔣️.json
- 🧰️framework/🔨️modules/🖱️ui/⌨️tui/🧬️schema/📏️shared-splitters/🔣️.json

## Validation

All changed JSON and schema fixtures parse; pointer golden rows have 80 cells. Source audit covered keyboard focus traversal, box-drawing junction direction and fixture geometry. Native append and prepared-fixture audits were reported to the parent; the owning agent corrected identified release-demand and state-guard problems. The parent owns the coordinated TUI and native cargo gates, which have not yet admitted a compiler due to available-memory limits. No TUI cargo test has run.
