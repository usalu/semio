# 🎯️ wgpu-tier split — closing the last structural leak

## Headline

`cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i {wasm-bindgen,js-sys,web-sys}` and the
same for `semio-s-plugin-puzzle` all report **"nothing to print."** Both now match
`semio-s-plugin-draw-fsm`'s baseline (re-verified clean at the end of this pass too). This closes
the leak `📓️wasip2-glue-leak.md` left open: `ui_wgpu` → `wgpu-engine` → real `wgpu` crate →
`wasm-bindgen-futures` → `js-sys`/`wasm-bindgen`, plus a second edge through `♾️infinite`'s own
`vello`/`vello_svg` dependencies that the predecessor's doc didn't fully chase down.

## Root cause, precisely

`♾️infinite`'s `🌍️world/🦀️component.rs` (14,021 lines, mounted unconditionally) named ~26 symbols
from `ui_wgpu`'s `wgpu-engine`-gated tier. Classifying every one of them by grepping their actual
bodies for `wgpu::`/`bytemuck`/`winit`/etc (not just by which file they happened to live in) showed
the predecessor's hypothesis was almost entirely correct: **the overwhelming majority were already
100% target-neutral**, just mis-gated at the module-mount level. Two files/submodules were
genuinely mixed and needed a real split. Two whole files were needlessly gated in full.

## Per-symbol classification

### Whole files reclassified target-neutral (zero `wgpu::`/`bytemuck`/etc reference, grep-verified)

| File | Contents | Evidence |
|---|---|---|
| `🎯️targets/🧊️wgpu/🦀️action.rs` | `BoundedAction*` family, `PreparedClaimedAction*`, `checked_action_string_bytes`, `ACTION_*_BYTE_CAPACITY` — a bounded reservation/queue system | Only imports: `crate::wgpu::ActionDescriptor`, `dsl::DslValue`. Zero `wgpu::` token anywhere in the file. |
| `🎯️targets/🧊️wgpu/🦀️input.rs` | `InputState`, `HitKind`, `HitTarget`, `PointerModifiers`, `DragAxis`, `DragState`, `KeyAction`, `TreeDragState`, `TreeDropPosition`, `PointerCallbacks` — hit-testing/pointer/keyboard state | Only imports: `crate::wgpu::geometry::Rect`, `crate::wgpu::ActionDescriptor`, `crate::wgpu::{BoundedAction*}`, `std`. Zero `wgpu::` token. |
| `🎯️targets/🧊️wgpu/🦀️prepared.rs` (3,918 lines) | `PreparedRenderInput/Upload/Eviction`, `PreparedRasterProducer/Rejected`, the whole job-driven CPU staging/admission state machine | Zero bare `wgpu::` token in the whole file. One `web_sys::window()` call (`OffscreenPresentToken::mint_for_dedicated_worker`) already carried its own correct `#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]` item gate from a prior pass. |
| `semio-framework-ui-scene`'s `math` module (re-exported as `kernel_3d_scene`) | `Camera3d`, `Vec3`/`Vec3Math`, `Mat4`/`Mat4Math`, `Instance3d`, `Mesh3dFault`/`Mesh3dSchema`/`Mesh3dLease`/etc, `project_point`, `screen_segment_distance`, the whole `mesh3d_*`/`ray_*`/`gumball_*` math surface | Own crate description: *"wasm-safe, depends on ui-contract and serde only."* Non-optional dependency of `semio-framework-ui` regardless of feature — the re-export gate was the only thing restricting it. |

**Action taken:** mounted `action`/`input`/`prepared` unconditionally in `📦️glue.rs` (removed
`#[cfg(feature = "wgpu-engine")]`); ungated their re-export blocks the same way; ungated the
`kernel_3d_scene` re-export block. No dependency edges added — `ui_scene` was already unconditional,
and `dsl`/`serde`/`serde_json` were already in the light `wgpu` feature.

### Genuinely mixed files — split into a new sibling `draw_types.rs`

`🦀️draw.rs` (5,068 lines) and `widgets.rs`'s `gizmo` submodule each interleaved target-neutral value
types with genuine `wgpu::Device`/`Buffer`/`Texture` code in the same file. Per-item classification:

| Symbol | Classification | Evidence |
|---|---|---|
| `DrawList` + its whole `impl` (CPU draw-command accumulator: `push_solid`, `push_line`, `push_triangle_fan`, scissor/clip stacks, …) | target-neutral | `impl DrawList { … }` (608 lines) contains zero `wgpu::`/`Device`/`encoder`/`Buffer` token. |
| `DrawLayer`, `GlassRegion`, `ScissorRect` (+`impl`), `ClipRegion` (+`impl`), `RetainedOutputGrant`, `UiInstance` (+`impl`), `VectorVertex`, the `KIND_*` consts | target-neutral | Plain `#[repr(C)]` value types (`UiInstance`/`VectorVertex` need `bytemuck::{Pod, Zeroable}` for GPU buffer layout, but reference no `wgpu` type — `bytemuck`'s own dependency is `bytemuck_derive`, a proc-macro that runs on the host and links nothing into the target). |
| `mesh_content_version`, `paint_selection_marquee`, `push_marquee_segment`, `selection_marquee_stroke`/`fill`, `dashed_line_segments`, `SELECTION_MARQUEE_*` consts | target-neutral | Pure math/hash over `DrawList`/`Theme`/points. |
| `SceneColorTarget` (+`impl`), `BlurGlobals`, `GlassInstance`, `GpuMeshBuffers`, `MeshGpuTable` (+cursors/registry), `RasterTexture{Table,Admission,StageFault,Witness}`, `IconAtlas`, `World3dVertex/Globals/GpuInstance` | **genuinely GPU** | Hold `wgpu::Texture`/`TextureView`/`Sampler`/`Buffer`/`Device` fields directly, or (`ear_clip_polygon`, unused by any wasip2 caller) live adjacent and were left alone since nothing needs them off that tier. |
| `gizmo::spatial_axis_rgba`, `orbit_view_gizmo_placement`, `OrbitViewGizmoTip`, `orbit_view_gizmo_tips`, `orbit_view_gizmo_hit_test` | target-neutral | Pure screen-space/vector math over `Camera3d`/`Vec3`/`Rect`/`Rgba`. |
| `gizmo::paint_orbit_view_gizmo` | **genuinely GPU-adjacent** | Takes `&mut WidgetContext<'_, E>`, which bundles `&mut FontAtlas` (parley/swash glyph atlas) and `Option<&IconAtlas>` (GPU icon atlas) — real rendering-engine state, not a value type. |

**Action taken:** created `🎯️targets/🧊️wgpu/🦀️draw_types.rs`, mounted unconditionally in `📦️glue.rs`.
It holds exactly the target-neutral rows above (extracted from `draw.rs` by brace-matched
line-range removal, verified brace-balanced before/after) plus a new `pub mod gizmo` holding the
five target-neutral gizmo functions. `draw.rs` keeps everything GPU (unchanged in place) and gained
one line — `pub use super::draw_types::*;` — so every existing `crate::wgpu::draw::{DrawList, ...}`
import path anywhere else in the crate (`gpu.rs`, `chrome.rs`, `engine.rs`, `paint.rs`,
`scene_slots.rs`, `widgets.rs`) keeps resolving unchanged when `wgpu-engine` is on.
`widgets.rs`'s `gizmo` module shrank to just `paint_orbit_view_gizmo`, importing the placement/tip
math back from `draw_types::gizmo`.

Two real regressions were caught and fixed by the build loop (see Build results):
- `prepared.rs` still imported `crate::wgpu::draw::{DrawLayer, DrawList, ScissorRect}` — that path
  only exists when `wgpu-engine` is on, so `prepared` (now unconditional) failed to resolve it for
  wasip2. Repointed all six `crate::wgpu::draw::*` references in `prepared.rs` to
  `crate::wgpu::draw_types::*`.
- `ClipRegion::effective_scissors` was module-private; `draw.rs` (a sibling of `draw_types` now,
  not a child) calls it directly. Widened to `pub(crate)`.

### `♾️infinite`'s `🌍️world/🦀️component.rs` — one genuine GPU capability found and excluded

`render_world_3d` (the retained-mode paint entry point: text labels, the orbit-view gizmo paint
call, `WidgetContext`-typed) is the **only** consumer in the whole file of anything still behind
`wgpu-engine`. Grepping the entire repo for `render_world_3d` shows its only callers are inside
`📺️renderer/🧑️‍🎨️engine`'s `Interpreter`/`Scenes`/`Shell` element components — the OS's
native/browser rendering host, never a plugin. It is now
`#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`-gated; its imports (`draw_text`,
`WidgetContext`, the paint half of `gizmo`) moved to a function-local `use` inside it, and its
signature now names `ui_wgpu::wgpu::widgets::WidgetContext` by full path (a top-level `use` for it
would fail to resolve under the light feature). No wasip2 capability is lost — nothing in the
plugin's own exported program logic ever called this function; drawing happens host-side.

## Cargo.toml narrowing

`semio-framework-ui`'s `wgpu-engine` feature bundles `dep:wgpu` (the real GPU crate) as one
inseparable unit with `winit`/`parley`/`swash`/`arboard`/`js-sys`/`wasm-bindgen`/`web-sys`. The fix
is entirely at the two caller manifests that turned it on unconditionally:

- **`🌊️flow/📦️packages/🦀️rust/Cargo.toml`**: base `ui_wgpu` dependency narrowed to `features =
  ["wgpu"]`; a new `[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]`
  table adds `features = ["wgpu-engine"]` back for every other target (feature-union, standard
  Cargo behavior for the same dependency declared in an unconditional + a target-gated table).
- **`♾️infinite/📦️packages/🦀️rust/Cargo.toml`**: `ui_wgpu` **and** `vello`/`vello_svg` moved into two
  mutually-exclusive target tables (`not(all(target_arch = "wasm32", target_env = "p2"))` vs
  `all(target_arch = "wasm32", target_env = "p2")`), since `vello`'s own `default-features`
  differs per side (`false` for wasip2, keeping only `Scene`/`peniko`/`kurbo`, confirmed
  target-neutral/ungated in vello's own source; `true` + `wgpu`/`wgpu_default` elsewhere). This
  second edge (`infinite` → `vello` → real `wgpu` crate, independent of `ui_wgpu`) was **not**
  identified or fixed by the predecessor's pass — `📓️wasip2-glue-leak.md` only traced the `ui_wgpu`
  side. Both `flow` and `puzzle` depend on `infinite` (`puzzle` via `infinite_canvas`), so this had
  to be fixed for either plugin to reach zero. `gpu_session` (`🖼️canvas/🦀️component.rs`, the only
  consumer of `vello::Renderer`/`util::RenderContext`/`vello::wgpu` in the crate) was already
  correctly `#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]`-gated from a prior pass,
  so no capability is lost.
- `semio-framework-ui`'s own `wgpu` (light) feature gained `dep:bytemuck` (backs
  `draw_types::{UiInstance, VectorVertex}`; its own dependency is a proc-macro, links nothing into
  the target) and `dep:semio-framework-job` (backs `prepared.rs`; first-party, itself already
  wasip2-safe — depends only on `semio-framework-trace` (→ `serde_json` only) and
  `semio-framework-async`, whose browser bridge is already `not(target_env = "p2")`-gated).

Checked for other unconditional `wgpu-engine` requests that could reach `flow`/`puzzle`:
`semio-framework` itself already only requests `features = ["wgpu"]` (unaffected); `📺️renderer`
and `🌀️procedural`'s plugin (the latter `[dev-dependencies]`-only) both request `wgpu-engine` too,
but neither is reachable from `flow`'s or `puzzle`'s dependency graph — confirmed by `cargo tree -i`
below actually reaching "nothing to print," which would be impossible if either leaked in.

## Before / after `cargo tree -i` evidence

Before (from `📓️wasip2-glue-leak.md`, `semio-s-plugin-puzzle --target wasm32-wasip2`):
`wasm-bindgen` 94 lines, `js-sys` 77 lines, `web-sys` 43 lines present; root cause traced to
`puzzle → os-infinite → semio-framework (+ semio-framework-plugin) → semio-framework-ui → wgpu →
{wasm-bindgen-futures → js-sys → wasm-bindgen, web-sys}` and a parallel `vello`/`vello_svg → wgpu`
edge this pass additionally closed.

After (this pass, re-run at the end after all fixes):

```
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i wasm-bindgen   → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i js-sys        → nothing to print
$ cargo tree -p semio-s-plugin-flow --target wasm32-wasip2 -i web-sys       → nothing to print
$ cargo tree -p semio-s-plugin-puzzle --target wasm32-wasip2 -i wasm-bindgen → nothing to print
$ cargo tree -p semio-s-plugin-puzzle --target wasm32-wasip2 -i js-sys      → nothing to print
$ cargo tree -p semio-s-plugin-puzzle --target wasm32-wasip2 -i web-sys     → nothing to print
$ cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 -i {wasm-bindgen,js-sys,web-sys}
  → nothing to print (unchanged baseline, re-verified not regressed)
```

## Build results

- **`cargo check -p semio-framework-ui --features wgpu-engine`** — the guardrail the ticket named
  explicitly: **`Finished dev profile [unoptimized] target(s) in 1m 01s`, 0 errors**, 136
  pre-existing-shape warnings (unrelated to this pass — `private_interfaces`, `unused Result`, dead
  code). The native/browser retained-mode engine is unbroken.
- **`cargo check -p semio-framework-os-infinite`** — 0 errors from this pass (one real regression,
  `effective_scissors` visibility, was caught here and fixed). 8 remaining `E0277`
  (`ToValue`/`FromValue` not satisfied on `DagDelta`/`DagNodeKind`/`DagNodeSpec`/`EdgeRouteStyle`)
  are 100% in `🎲️board/🔌️ports/➡️directed/🕸️dag/…` — the concurrent os-kernel `ToValue` cascade this
  ticket's own brief named in advance (`📓️os-kernel-tovalue-cascade.md` exists in this same research
  folder). Confirmed by grepping every error's file location: zero mention of any file this pass
  touched.
- **`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-flow`** — could not be confirmed at
  a literal 0 errors during this pass; blocked by the same concurrent wave (errors land in
  `🔌️plugin/component.rs` — `DslValue`/`Value` type mismatches — and the `dag` `ToValue` cascade
  above). `cargo check` (same target, no linking) narrows this to exactly **1** error, in
  `🔌️plugin/component.rs:6770` (`expected &DslValue, found &Value`) — unrelated to this pass by the
  same file-location check.
- **`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-animate`** — `semio-framework-ui`
  itself compiles clean (0 errors, 37 warnings) every single run. The build as a whole hit a
  `wasm-component-ld`/`rust-lld` `SIGSEGV` linking the unrelated `semio-framework-actor` crate
  (never touched by this pass) on every retry (`-j 8` default, `-j 2`, `-j 1`, and after a fresh
  isolated pre-link of `actor` alone, which itself succeeds in 1.4s standalone). System load
  averaged 15–21 throughout this session (documented pattern from this same ticket's earlier
  sessions at 30–36) — consistent with the shared-machine resource contention `📓️verified-outcomes.md`
  already attributes this exact crash signature to. Not attributable to this pass: `actor` has zero
  reference to `draw_types`/`draw`/`action`/`input`/`prepared`/`wgpu`/`gizmo`/anything this pass
  touched.
- **`cargo tree -p ... -i {wasm-bindgen,js-sys,web-sys}`** (the metadata-only, lock-free,
  cannot-go-stale check) is unaffected by any of the above concurrent breakage or linker flakiness
  and is the primary evidence for this ticket: **clean for flow, puzzle, and draw-fsm.**

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` — top docstring rewritten;
  `draw_types` mounted; `action`/`input`/`prepared`/`kernel_3d_scene` ungated (mount + re-exports);
  `draw`'s re-export list narrowed (moved symbols dropped); new ungated
  `pub use draw_types::{gizmo, mesh_content_version, paint_selection_marquee, DrawList};`.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw_types.rs` — new file (~1,087
  lines): the target-neutral half of `draw.rs` plus `draw_types::gizmo`.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw.rs` — target-neutral items
  removed (net −965 lines), replaced with `pub use super::draw_types::*;`.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️widgets.rs` — `gizmo` module
  shrunk to `paint_orbit_view_gizmo` only, importing placement/tip math from `draw_types::gizmo`.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs` — six
  `crate::wgpu::draw::*` references repointed to `crate::wgpu::draw_types::*`.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` — `wgpu` feature gained
  `dep:bytemuck`/`dep:semio-framework-job`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` — `ui_wgpu` narrowed
  to `["wgpu"]` base + `["wgpu-engine"]` target-additive.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml` — `ui_wgpu` and
  `vello`/`vello_svg` moved into two mutually-exclusive target tables.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs` — top `use` block split
  (light `gizmo` module added, `draw_text`/`WidgetContext`/GPU `gizmo` removed);
  `render_world_3d` gated `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]` with
  function-local imports and a fully-qualified `WidgetContext` path; its one internal call site
  renamed `gizmo::paint_orbit_view_gizmo` → `gpu_gizmo::paint_orbit_view_gizmo`.

## Deliberately left alone

- `typst`/`typst-svg`/`typst-assets`/`image` remain unconditional dependencies of `♾️infinite` —
  real third-party crates, but not part of the `wasm-bindgen`/`js-sys`/`web-sys` leak this ticket
  slice targets. Flagged as a candidate for a future slice of the broader zero-third-party goal.
- `ear_clip_polygon`/`point_in_triangle` in `draw.rs` are pure math (no `wgpu::` reference) but
  unused by any current wasip2 caller — left under the `wgpu-engine` gate rather than moved, since
  moving unused code adds no verification value and widens the diff for no reason.
- `📺️renderer`'s and `🌀️procedural`'s own unconditional `wgpu-engine` requests were left as-is —
  neither is reachable from `flow`'s or `puzzle`'s dependency graph (confirmed by the clean
  `cargo tree -i` results, which would show them otherwise), and `📺️renderer` is exactly the kind of
  native/browser rendering host this tier split is meant to keep unrestricted.
