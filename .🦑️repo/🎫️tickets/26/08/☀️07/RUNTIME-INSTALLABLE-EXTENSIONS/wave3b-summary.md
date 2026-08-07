# Wave 3.b — Flow draw extension conversion

## Goal

Migrate flow `🖍️draw` (19 operators, depends on `semio-s-2d`) from a path-module inside `semio-framework-os-flow` into a packaged, runtime-installable extension. Move non-operator Scene APIs onto the flow kernel/core surface so callers (notably procedural2d) do not need the draw extension installed.

## New crate

| Item | Value |
|------|-------|
| Folder | `✏️s/🔌️plugins/🌊️flow/️️extensions/🖍️draw/` |
| Crate | `semio-s-plugin-flow-extension-draw` |
| Component package | `semio:flow-extension-draw` |
| Role | `extension`, `extends = "flow"`, `contributes = ["flow.extension"]` |
| Apps | `flow-play`, `procedural3d-play` (two `Contribution::FlowExtension` entries) |
| Extension id | `draw` |
| Handler | `evaluate` via `evaluate_json` (same pattern as BIM) |

Anatomy:

- Owner `🦀️component.rs` — operator layer + `ExtensionBundle` + `extension_exports!(bundle)`
- `📦️packages/🦀️rust/{📦️glue.rs,Cargo.toml,📜️script.ts,📋️project.json}`
- Root `Cargo.toml` workspace member + `semio-s-plugin-flow-extension-draw` alias

Operators call `flow_extension_sdk::with_drawing_kernel` so they share the host `DrawingStore` when linked in-process.

## Moved APIs → flow core `#region 🖍️DrawingKernel`

Always available via `semio-framework-os-flow` (`flow_core` / `flow_extension_sdk`) without installing the draw extension:

| API | Purpose |
|-----|---------|
| `with_drawing_kernel` | Shared process-wide `DrawingStore` access |
| `retain_drawing_handles` | GC unreferenced drawing handles after eval |
| `render_scene_json` | Flatten drawing → scene JSON |
| `export_svg_json` | SVG export wrapper |
| `export_pdf_json` | Base64 PDF export wrapper |
| `export_dwg_json` | Base64 DWG export wrapper |
| `import_dwg_json` | DWG import → handle |
| `dispose_drawing` | Dispose handle (`#[wasm_bindgen]` on wasm32) |
| `trace_bitmap_json` | Bitmap autotrace → segments |
| `boolean_segments_json` | Planar boolean on segment arrays |

Also: `semio-s-2d` added as a direct dependency of `semio-framework-os-flow`. Wasm wrappers (`render_drawing_scene`, …) call these local functions.

## Callers updated

| Caller | Change |
|--------|--------|
| `procedural2d/⚙️engine` | `use flow_core::render_scene_json` |
| `procedural` Cargo.toml | dep `flow_extension_draw` → `flow_core` (same package path) |
| `procedural` glue | `extern crate flow_core` |
| flow `install_builtin_flow_extensions` | no longer registers draw (empty builtins; packs are extensions) |
| flow `📦️glue.rs` | no `extensions::draw` path-mod / no `flow_extension_draw` alias |
| root `📜️script.ts` | removed `flow_extension_draw` from shared-domain allowlist |

## Removed

- Framework path-module `🌊️flow/️️extensions/🖍️draw/` (deleted)
- Extension `#region 🔖️WasmExt` / `standalone-wasm`
- Scene region from the extension crate (lives in core)

## Verification

| Check | Status |
|-------|--------|
| Workspace member + alias resolve | OK |
| No `flow_extension_draw` residuals in product code | OK |
| `cargo check -p semio-s-plugin-flow-extension-draw` | **Blocked** here: Xcode license (`blake3` C build). Re-run after `sudo xcodebuild -license`. |

```bash
cargo test -p semio-s-plugin-flow-extension-draw
cargo check -p semio-framework-os-flow -p semio-s-plugin-procedural
```

## Coordination

Ran in parallel with Wave 3.a (light packs) and Wave 3.c (brep). Final glue has no draw shim; brep geometry kernel surface is owned by 3.c. Do not reintroduce `extensions::draw` path-mod.
