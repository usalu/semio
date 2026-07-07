# Playground Chrome Reliability — verify log

## Root cause

Window chrome (navbar, dock tab caps, borders, options/command chips) was composited on backdrop draw layers while scene passes (node-graph, world-3d) painted afterward on the same tier, covering tabs and borders. Folded measure/engagement chips used the overlay backdrop tier and could composite incorrectly relative to glass panels.

## Fixes

1. **Chrome layering** (`framework/renderer/wgpu/rs/lib.rs`)
   - `DockState::paint_chrome` re-draws stack chrome on the overlay draw list after window content.
   - Navbar and footer render on the overlay draw list so they stay above scene content.
   - Folded Window Options / Command chips always render on the main `draw` list.

2. **Registration contract** (`framework/plugin/rs/lib.rs`, `framework/core/rs/lib.rs`)
   - `AppBuilder::build_definition()` asserts non-empty unique window kinds, non-empty body keys, and layout `window_kind_id` cross-references.
   - `collect_window_kind_ids_from_layout()` helper for layout validation.
   - `PanelGroup` enum (`Workbench`, `Details`, `Display`, `Settings`) replaces free-form panel group strings.

3. **All 24 plugins** updated to use `PanelGroup::Workbench` / `PanelGroup::Details` in `panel_tab()` registrations.

## Regression fix (2026-07-07 evening)

**Symptom:** procedural3d, lowpoly showed no window content; title tab, focus, and close chips missing.

**Cause:** `paint_chrome` re-drew full stack chrome (including opaque `canvas_clear` body fill) on the **overlay** draw list after window content. Overlay composites after the main draw list, so the body fill covered all scene content. Cap chrome on overlay also failed to appear reliably.

**Fix:** `render_stack` gained a `body_fill` flag. `paint_chrome` now runs on the **main** draw list after window content with `body_fill: false` (tab cap + border strokes only). Navbar/footer restored to the main draw list.


- `cargo test -p semio-framework-core -p semio-framework-plugin --lib` — pass (incl. `app_builder_tests`)
- `cargo test` on all 24 plugin crates (`draw-plugin` … `presentation-plugin`) — pass
- `cargo build -p procedural3d-plugin --target wasm32-unknown-unknown --release` — pass
- `bun ./framework/renderer/wgpu/script.ts wasm` — pass
