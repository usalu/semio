# Window layout unification: Rust TUI migration (not compiled)

## Changed
- `⌨️tui/📏️layout/🦀️.rs`: deleted the TUI-owned `WindowLayoutRoot/Child/AxisNode/StackNode/WindowNode`. `WindowLayout` is now a TUI view struct `{ root: contract WindowLayoutNode, zoomed }` (zoom stays view state). All solver/mutation fns (`solve_window_layout`, `split_window`, `dock_window`, `resize_*`, `compact`, ...) rewritten over the contract tree. Axis typed: `create_default_layout`/`split_window` take `Axis` (row = Horizontal, column = Vertical). New helpers: `window_node`, `stack_node`, `axis_node`, `stack_windows`, `layout_windows_mut`, `window_label`, `window_corner`, `window_corner_set`; re-exports `Axis`, `WindowLayoutNode`, `window_id`, `WindowStackCorner`. `stack_hosting` now returns the stack `WindowLayoutNode`.
- TUI keeps a one-window `Split` from `create_default_layout` (identical mounted splitter chrome/scene tree); it is TUI-internal, never persisted.
- `🧲️docking`: duplicate `WindowStackCorner` deleted, now `pub use semio_ui_contract_presentation::WindowStackCorner`. Contract gained `WindowStackCorner::is_top/is_left` (in `📐️layout/🦀️.rs`).
- Cargo: `semio-ui-contract-presentation` path dep added to tui `🧱️cell-model` and `🏛️frame`.
- `🖥️chrome`: peers, tab states, stack slot, mount tree migrated.
- Dashboard terminal: `🪟️windows` (`Place::Split(Axis)`, `stack_of`, title writer), `⌨️controls`; tests `🧪️tests/🔬️unit/🧲️docking`.
- TUI tests migrated by hand/script: pointer-routing, unit (layout laws, vt-scene, paint), render-equivalence, engine, drag-preview, chrome.

## Check
- `cargo check -p semio-ui-tui-core -p semio-ui-tui-frame -p semio-ui-tui-engine` then tests of `semio-ui-tui-unit` (tui `🧪️tests/🔬️unit` package) + the tui test crates (engine, chrome, pointer-routing, drag-preview, render-equivalence) and the dashboard terminal crate + its `🧲️docking` tests.
- Watch: `window_node` takes `String` (no `impl Into`); `with_stack_mut`/`resize_split` borrow patterns; `matches!(.., Stack { .. })` in docking test.
- Note for contract owner: `create_window_layout(impl Into<String>, ..)` callers passing `"x".into()` will not infer.
