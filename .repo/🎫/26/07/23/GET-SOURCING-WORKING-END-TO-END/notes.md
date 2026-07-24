# Get Sourcing Working End to End

## Root cause

`UiPresence` / presence-based UI model landed in framework without updating `sourcing/plugin`:

1. `UiPresence` used on Input/Select/NumberStepper but not imported
2. `UiToggleNode.pressed` removed — pressed state is now `presence: UiPresence::selected(...)`

## Fix

- Import `UiPresence`
- Module filter toggles use `presence: UiPresence::selected(pressed)`
- Regression test: `filter_bar_module_toggles_encode_pressed_state_as_presence_selected`

## Verification

- `cargo build -p sourcing-plugin --target wasm32-wasip2 --release` ✅
- `SEMIO_PLUGIN=sourcing bun ./script.ts build` (framework/product/os/dev) ✅
- `cargo test -p sourcing_curate --lib` → 12 passed
- `cargo test -p sourcing-plugin --lib` → 16 passed (incl. new filter-bar test)
- Playwright smoke on `http://127.0.0.1:6081/` → Pool / Curated / Preview / Grid + Beams/Windows/Slabs, no page errors
