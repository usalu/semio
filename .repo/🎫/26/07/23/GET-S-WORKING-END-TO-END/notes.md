# Get S Working End to End

## Root cause
`waiting?: Option<bool>` was added to UI nodes / `WindowMeasure::Slider` (and related) without updating all plugin/module initializers. `dev:s` is studio mode (`isStudioPluginFilter("s")`) so it builds **all** plugin crates — one missing field anywhere blocked the whole OS build.

## Fix
- Added `waiting: None` across framework plugin helpers and all guest plugins/modules that construct UI nodes / sliders.
- Also fixed related struct gaps uncovered by the full studio build: `group_labels` on `AppLabelsOverlay`, `ready`/`loading` on sliders, `diff_view`/`event_feed` on `UiComponentSceneNode`, `drop_overlay` on `UiStackNode`, `SceneQuery` import in `layout/rs` wasm session, and restored corrupted DAG catalogue kinds list.

## Verification
- `SEMIO_PLUGIN=s bun ./script.ts build` in `framework/product/os/dev` completed with exit 0.
- Packaged 35 plugin modules under `framework/product/os/dev/plugin-modules/` including `s/`.
