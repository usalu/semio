# Get Trinity Working End to End

## Root cause

`UiInspectorFieldGroup` gained required `presence: UiPresence` (unified UI element state model).
Jack + rewrite node inspectors in `trinity/plugin/rs/lib.rs` still constructed groups without it → E0063, `dev:trinity:jack` failed at plugin build.

## Fix

Add `presence: UiPresence::default()` to both identity inspector field groups (jack ~614, rewrite ~2329).

## Verification

- `cargo build -p trinity-plugin --target wasm32-unknown-unknown --release` succeeds
- `SEMIO_RENDERER=react bun run dev:trinity:jack` → http://127.0.0.1:6054/
- `SEMIO_RENDERER=react bun run dev:trinity:rewrite` → http://127.0.0.1:6056/
- Playwright jack: title `semio · trinity · jack`, 3 canvases, Document/Catalogue/Jack Query/Results, Nakagin table rows (`floor0-right` / `floor0-left`)
- Playwright rewrite: title `semio · trinity · rewrite`, 9 canvases, Document/Catalogue/LHS/RHS/Jack/Parameters/Before/After
- Headless `NoCompatibleDevice` noise only (no WebGPU in Playwright)
