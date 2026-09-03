# `semio-framework`'s `ui_wgpu = { features = ["wgpu"] }` line — recon (no change made)

Scope: `🧰️framework/📦️packages/🦀️rust/Cargo.toml:23`. Investigate-only per task; change only if plugins
provably don't need it. Conclusion: **do not change** — negative result, documented below.

## Part 1 — facts

1. **What `wgpu` (light) enables** in `semio-framework-ui`: `wgpu = ["dep:serde", "dep:serde_json",
   "dep:semio-framework-os-kernel", "dep:semio-framework-value-derive", "dep:bytemuck",
   "dep:semio-framework-job"]`. This is a *different, lighter* feature than `wgpu-engine` (the real
   `wgpu` crate + `winit`/`parley`/`swash`/`taffy`/`wasm-bindgen`/`js-sys`/`web-sys`/`arboard`). Per
   `🎯️targets/🧊️wgpu/🦀️.rs`'s mod table, the `wgpu`-only tier mounts: `component` (the declarative
   `UiNode`/layout/utilities model — 473 serde/`ToValue`/`FromValue` hits, confirmed real, not
   incidental), `draw_types` (`DrawList`, CPU draw accumulation, gizmo math), `geometry`, `minimap`,
   `prepared`, `input`, `action`, `theme`, plus `icon_name`/`ui_axes`/`locale_terminology` generated
   value types, and (feature-gated further on `wgpu` specifically) `presence_bar`. Everything else
   (`arena`, `tree`, `reconcile`, `chrome`, `cursor`, `draw`, `gpu`, `layout`, `flex`,
   `mounted_layout`, `shaders`, `text`, `paint`, `events`, `scene_slots`, `shell`, `engine`,
   `widgets`, `host`, all seven `🧱️elements/*/🎯️targets/🧊️wgpu` submodules) is `wgpu-engine`-only.

2. **Plugin-reachable code DOES use `wgpu`-gated items.** `🧰️framework/📦️packages/🦀️rust/🦀️.rs:15-16`
   re-exports `ui_wgpu::wgpu::IconName` and `{Locale, Terminology}` unconditionally, and its own
   comments (line ~2041) say "the declarative component model (layout/utilities/UiNode) lives in
   `ui_wgpu` now". Confirmed by grep: `component.rs` alone has 473 `serde`/`ToValue`/`FromValue`
   hits — this is the plugin-facing declarative UI tree type, not GPU rendering. Three other
   plugin-adjacent manifests independently confirm the same pattern of depending on `features =
   ["wgpu"]` (never `wgpu-engine`) for runtime: `🔌️plugin/📦️packages/🦀️rust/Cargo.toml:54` (with an
   explicit comment: *"the 'wgpu' feature is declarative types only and stays wasm32-wasip2-safe (it
   never pulls the engine, which lives behind 'wgpu-engine')"*), `🌊️flow/📦️packages/🦀️rust/Cargo.toml:31`,
   and `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml:58` (runtime dep — the same crate's
   *dev*-dependency at line 76 separately upgrades to `wgpu-engine`, for test-only frustum/AABB math,
   never shipped).

3. **Compiles clean for `wasm32-wasip2`.** Grepped all `wgpu`-only-tier files
   (`component.rs`, `draw_types.rs`, `geometry.rs`, `minimap.rs`, `prepared.rs`, `input.rs`,
   `action.rs`, `theme.rs`) for `wgpu::`(crate)/`winit`/`parley`/`swash`/`taffy` — zero hits; every
   `wgpu::` hit found is `crate::wgpu::…`, i.e. this module's own path, not the GPU crate.
   `host.rs` (the one file that does `use winit::…`) is gated `#[cfg(all(feature = "wgpu-engine",
   not(target_os = "wasi")))]` — outside the light tier entirely. Verified by compiling, not just
   reading (see Verification below): both `cargo check -p semio-framework` (host) and `--target
   wasm32-wasip2` (plugin path) are clean, and `cargo check -p semio-framework-ui --features wgpu`
   (the isolated light-tier build) is clean too.

4. **Who needs `wgpu-engine`:** grepped every manifest naming `wgpu-engine` — only
   `📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu` (the actual desktop GPU renderer),
   `♾️infinite/📦️packages/🦀️rust` (native/browser `vello` host-engine path — its one GPU-touching fn
   `render_world_3d` is itself `#[cfg(not(all(target_arch="wasm32", target_env="p2")))]`-gated per
   that crate's own docstring), `🌊️flow` (separate `[target...]`-scoped dep, not the runtime one),
   `🔌️plugin` (module-local dev/build config), and `procedural`'s **dev-dependencies** only. None of
   these are the plugin runtime edge — `semio-framework`'s own dependency (the line under
   investigation) requests only `["wgpu"]`, never `["wgpu-engine"]`.

## Why nothing was changed

The premise in the task ("serde usage lives entirely under `🎯️targets/🧊️wgpu/`, behind a non-default
`wgpu` feature… a plugin component almost certainly does not need it") turns out to describe a
tier split that a prior pass on **this same ticket** already made (see the "✅
RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS (26/09/01)" docstring at the top of
`🎯️targets/🧊️wgpu/🦀️.rs`, and matching docstrings in `flow`'s and `infinite`'s manifests). The crate
already has two features: `wgpu` (light, declarative, serde-bearing, wasm32-wasip2-safe — mounts the
plugin-facing `UiNode`/`IconName`/`Locale`/`Terminology`/`DrawList`/`action`/`input`/`theme` types)
and `wgpu-engine` (heavy, the real `wgpu` crate + winit/parley/swash/taffy, host/desktop-only).
`semio-framework`'s Cargo.toml line 23 already requests the correct, minimal one (`["wgpu"]`, not
`["wgpu-engine"]`) — it is not target-gated because plugins genuinely consume those declarative
types at the type level (re-exported straight through `semio-framework`'s own `🦀️.rs`). The residual
serde pulled in by this feature is load-bearing (ToValue/FromValue for the plugin↔host UI-tree
boundary), not an accident, and is out of this task's scope to remove (would require touching
`semio-framework-ui`'s `component.rs`, explicitly excluded — other agents are recon'ing
`-os-kernel`/`-plugin`/`semio-framework` already per the task's own coordination note).

**No manifest was edited.** This is a precise negative result.

## Verification (both paths), `CARGO_TARGET_DIR=…/scratchpad/iso3`, `RUSTC_WRAPPER=""`

- `cargo check -p semio-framework --message-format short` → exit 0 (warnings only, no errors).
- `cargo check -p semio-framework --target wasm32-wasip2 --message-format short` → exit 0 (warnings
  only, `Finished … in 1m 54s`).
- `cargo check -p semio-framework-ui --features wgpu --message-format short` → exit 0.
- `cargo metadata --no-deps --format-version 1 >/dev/null; echo $?` → `0`.

## Payoff measurement — before/after (unchanged, since no edit was made)

```
cargo tree -p semio-s-plugin-draw --target wasm32-wasip2 --edges normal --prefix none \
  | awk '{print $1}' | sort -u | grep '^serde'
```
Both before and after (identical, no change applied):
```
serde
serde_core
serde_derive
serde_json
```
`cargo tree -p semio-s-plugin-draw --target wasm32-wasip2 -i serde --edges normal` confirms
`semio-framework-ui` is still a **direct** serde-requiring dependent in the graph (it requires
`dep:serde` itself under the `wgpu` feature — not just transitively through `semio-framework`), so
`-ui` is correctly **still** in the runtime-edge list documented in the task. This is expected and
correct, not a miss: `-ui`'s serde is genuine (the plugin-facing declarative UI tree types), and
removing it would require changing `semio-framework-ui`'s own `component.rs`, which is out of this
task's scope.

The separate `wit-component`/`wit-parser` → `wit-bindgen-rust-macro` serde chain is also present in
the `-i serde` output (proc-macro, host-side only, does not ship — consistent with the task's own
note).
