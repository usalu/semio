# Fix Aggregator Empty Windows Plugin Instance Busy

## Symptom

Aggregator Top + Perspective panes empty. Shell shows:

`[object Object] (see error.payload) payload={"tag":"message","val":"plugin instance busy"}`

## Cause chain

1. Boot `setActiveExample(concrete-forest)` runs `drive_precompute` → `precompute_step` → `puzzle3d_now_ms()`.
2. `puzzle3d_now_ms` was gated only on `target_arch = "wasm32"`, so the WASI P2 plugin component called `js_sys::Date::now()`.
3. Panic: `cannot call wasm-bindgen imported functions on non-wasm targets` → wasm trap `unreachable`.
4. Trap skipped `InstanceGuard`/`RefCell` Drop → guard stuck at 1 → every later `refreshUi`/`render` returned `plugin instance busy`.
5. jco surfaces that as `[object Object] (see error.payload)` with `payload.val = "plugin instance busy"`, so message-only busy retries often missed it.

## Fix

- `puzzle/3d/rs/lib.rs`: `puzzle3d_now_ms` uses `Instant` on native + `target_env = "p2"`; `js_sys::Date::now` only for wasm-bindgen web.
- `framework/plugin`: `UnsafeCell` instance store (no poisoned `RefCell` after trap) + WIT `clear-instance-guard` + bridge heal on busy/trap.
- `framework/core/js`: `pluginErrorText` / `isPluginInstanceBusyError` read jco `payload.val`.
- `WindowMeasure::Slider.reveal: None` filled where the new field broke plugin builds.

## Verify

- Playwright `:6023`: 0 busy/unreachable console errors, 2 canvases, Top+Perspective with Abbau Aufbau content (`after-fix.png`).
- Vitest: jco busy detection + serialize handle — 2 passed.
