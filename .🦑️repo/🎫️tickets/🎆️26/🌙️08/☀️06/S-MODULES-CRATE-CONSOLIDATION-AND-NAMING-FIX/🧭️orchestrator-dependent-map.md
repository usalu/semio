# Orchestrator scratch — full cross-cutting dependent map (found via exhaustive grep, verified 2026-08-06)

Do NOT delete — working notes for the orchestrating session while the 4 background agents (2d/3d/mindmap/imperative)
finish their merges. Apply these fixes myself once each agent reports its new crate's taxonomy module paths.
None of these are root Cargo.toml edits (those go in the final registrar-handoff block only).

## semio-s-3d dependents (old: kernel_3d_brepkit / kernel_3d_engine / kernel_3d_mesh / kernel_3d_scene / kernel_3d_spatial)
1. `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml` — kernel_3d_brepkit + kernel_3d_engine. Use sites: 🗿️artifacts/📐️cad/⚙️engine/{🦀️component.rs, 🕹️interaction/🦀️component.rs, 🔄️transformation/🦀️component.rs, 📥️geometry-import/🦀️component.rs}, 🎛️apps/📐️cad/🦀️component.rs. Symbols: BrepkitKernel, mesh_data_from_mesh_transfer, ObjSolidExporter/Importer, StlSolidExporter/Importer, StepSolidExporter/Importer, BrepKernel (trait), GeometryHandle, Vec3, block_on.
2. `✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` — kernel_3d_brepkit only. Use site: 🎪️panes/📐️koordinator/🦀️component.rs lines 20-25 (ObjSolidExporter/Importer, StlSolidExporter/Importer, StepSolidExporter/Importer).
3. `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml` — kernel_3d_mesh + kernel_3d_brepkit + kernel_3d_engine. Use sites across 🎛️apps/💠️lowpoly/{🖌️session,🧭️view,🎮️commands/🧲️transform,🎮️commands/🔷️mesh-edit,🎮️commands/🧵️uv}/🦀️component.rs + 🗿️artifacts/💠️lowpoly/{⚙️engine,⚙️engine/🧵️media,🔧️op}/🦀️component.rs. Symbols: Vec3, MirrorAxis, FaceId, EdgeId, VertexId, WeldMode, HalfedgeMesh, MeshKernelError (all mesh); BrepkitKernel, GeometryHandle (brepkit/engine).
4. `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/⚡️implementations/🦀️rust/Cargo.toml` line 27 — kernel_3d_engine dep declared but **UNUSED** (zero .rs use sites found, confirmed via grep of the whole extension dir). Just DELETE this dependency line, don't repoint.
5. `🧰️framework/🛍️products/💻️os/⚡️implementations/🦀️rust/Cargo.toml` lines 26-27 — kernel_3d_brepkit + kernel_3d_engine. Use sites: 📦️lib.rs lines 2735-3450 (SolidExporter/SolidImporter registries, register_solid_exporter/importer, export/import_registered_solid, BrepkitKernel::new, Step/Stl exporters/importers in a test). Also `📦️plugin_bundle_installer_shim.rs` has a COMMENT mentioning kernel_3d_engine (line 4) — check if it needs updating (likely just prose, verify).
6. `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml` — optional dep kernel_3d_scene (feature `dep:kernel_3d_scene`, line ~33/48). Use sites: 📦️lib.rs lines 6404, 8581, 8642 (ScenePass3d, Instance3d, SceneDraw3d) + line 17867 `pub use kernel_3d_scene::{...}` (RE-EXPORT — check exactly what it re-exports, this propagates downstream).
7. `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/⚡️implementations/🦀️rust/🌍️world/Cargo.toml` line ~13 — kernel_3d_scene. Use sites: 📦️lib.rs line 5 (`use kernel_3d_scene::{...}`) + lines 2793-3827 (project_point, screen_segment_distance, ray_segment_distance).
8. `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml` line ~29 — kernel_3d_scene. Use sites: 📦️lib.rs lines 13432-14238 (OrbitController, Camera3d, Vec3).
9. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/⚡️implementations/🦀️rust/🧩️extensions/📐️brep/Cargo.toml` lines 18-19 — kernel_3d_brepkit + kernel_3d_engine. Use sites: 📦️lib.rs (~2300 lines) — BrepkitKernel, mesh_data_from_mesh_transfer, block_on, BrepKernel, GeometryHandle, GeometryKind, ParamDomain, PointClassification, Vec3, BrepError. NOTE this dir still uses OLD ⚡️implementations naming — not my concern, only fix the kernel_3d_* dep.
10. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/⚡️implementations/🦀️rust/Cargo.toml` line 44 — kernel_3d_engine dep declared but **UNUSED** (zero .rs use sites in the whole crate — only file is 📦️lib.rs, grepped clean). DELETE this dependency line, don't repoint.
11. `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml` lines 37-38 — kernel_3d_brepkit + kernel_3d_engine. Use site: 🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs (BrepkitKernel, Obj/Step/Stl Exporter/Importer, SolidExporter/Importer traits, BrepKernel, GeometryHandle, block_on).
12. `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` line 61 — kernel_3d_scene. Use site: 🗿️artifacts/🧊️procedural3d/⚙️engine/🦀️component.rs:576 (aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Mesh3d, Vec3).

Root `Cargo.toml` `[workspace.dependencies]` lines ~314-315 alias `semio-framework-os-kernel-3d-brep` and `semio-framework-os-kernel-3d-brep-engine` — both need removal (→ registrar handoff, not mine to touch).

## semio-s-2d dependents (old: kernel_2d_rs / kernel_2d_engine)
1. `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml` lines 27-28. Use site: 🗿️artifacts/🖍️draw/⚙️engine/🦀️component.rs lines 983-1082 (PathSegment, booleans::boolean_paths_many, trace::trace_bitmap_paths).
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/⚡️implementations/🦀️rust/🧩️extensions/🖍️draw/Cargo.toml` lines 18-19 (crate semio-s-kernel-flow-extension-draw). Use site: 📦️lib.rs (1320 lines) — DrawingStore, DrawingKernel trait, DrawingKind, DrawingError, DrawingHandle, FillStyle, GradientStop, LineCap, LineJoin, StrokeStyle, Vec2, block_on, PathSegment, DrawingNode.

No root `[workspace.dependencies]` aliases exist for these 2 (never were aliased) — nothing to remove there. Consider ADDING new workspace.dependencies entries for semio-s-2d/semio-s-3d/semio-s-mindmap/semio-s-imperative in the handoff (C4 convention) so future dependents can adopt `workspace = true`.

## semio-s-mindmap dependents (old: reasoning_mindmap)
1. `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml` line 81. Use site: 🗿️artifacts/◻2d/⚙️engine/🦀️component.rs:24 — `pub use reasoning_mindmap as mindmap;`
2. `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/Cargo.toml` line 29. Use site: 🗿️artifacts/🔌️wires/⚙️engine/🦀️component.rs:135 — `pub use reasoning_mindmap as mindmap;` (same pattern). Also a prose comment at 🗿️artifacts/🔌️wires/📡️spr/🦀️component.rs:27 mentioning `reasoning_mindmap` — check/update if it names the crate explicitly.

## semio-s-imperative dependents (old: imperative_engine, kernel crate — NOT the plugin)
1. `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/Cargo.toml` line 32. Use site: 🗿️artifacts/🎬️sequence/⚙️engine/🦀️component.rs:12 + several call sites (compile_to_text, imperative_catalogue_json, imperative_module_registry, Executor, Path, RunResult, Step).
2. `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/Cargo.toml` line 36 (the PLUGIN depending on the KERNEL — expected). Use sites: 🗿️artifacts/📜️imperative/⚙️engine/🦀️component.rs:5 (compile_to_text, imperative_catalogue_json, imperative_module_registry, Executor, RunResult) + 🗿️artifacts/📜️imperative/🦀️component.rs:17 (`pub use imperative_engine::{Path, Step};`).

## Root package.json workspaces (lines confirmed via grep, apply directly — NOT root Cargo.toml, this is fair game per task instructions "you may edit just the workspaces array")
Line 31: `"✏️s/🔨️modules/🧊️3d/📐️brep/⚡️implementations/🟦️typescript",` → `"✏️s/🔨️modules/🧊️3d/📦️packages/🟦️typescript",`
Line 32: `"✏️s/🔨️modules/◻2d/⚡️implementations/🟦️typescript",` → `"✏️s/🔨️modules/◻2d/📦️packages/🟦️typescript",`

## Root Cargo.toml (registrar handoff only, I do NOT touch this)
Remove member lines (verified current line numbers, may shift):
- `"✏️s/🔨️modules/🧊️3d/🎬️scene/⚡️implementations/🦀️rust",`
- `"✏️s/🔨️modules/🧊️3d/🥽️mesh/⚡️implementations/🦀️rust",`
- `"✏️s/🔨️modules/💭️mindmap/⚡️implementations/🦀️rust",`
- `"✏️s/🔨️modules/🧊️3d/📐️brep/⚡️implementations/🦀️rust",`
- `"✏️s/🔨️modules/🧊️3d/📐️brep/⚙️engine/⚡️implementations/🦀️rust",`
- `"✏️s/🔨️modules/📜️imperative/⚡️implementations/🦀️rust",`
- `"✏️s/🔨️modules/🧊️3d/🗺️spatial/⚡️implementations/🦀️rust",`
(◻2d's 2 old crates were NEVER explicit members — nothing to remove there, but confirm no glob covers them either.)

Add member lines:
- `"✏️s/🔨️modules/◻2d/📦️packages/🦀️rust",`
- `"✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust",`
- `"✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust",`
- `"✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust",`

Remove from `[workspace.dependencies]`:
- `semio-framework-os-kernel-3d-brep = { path = "✏️s/🔨️modules/🧊️3d/📐️brep/⚡️implementations/🦀️rust" }`
- `semio-framework-os-kernel-3d-brep-engine = { path = "✏️s/🔨️modules/🧊️3d/📐️brep/⚙️engine/⚡️implementations/🦀️rust" }`

Consider adding (C4 convention, optional):
- `semio-s-2d = { path = "✏️s/🔨️modules/◻2d/📦️packages/🦀️rust" }`
- `semio-s-3d = { path = "✏️s/🔨️modules/🧊️3d/📦️packages/🦀️rust" }`
- `semio-s-mindmap = { path = "✏️s/🔨️modules/💭️mindmap/📦️packages/🦀️rust" }`
- `semio-s-imperative = { path = "✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust" }`

## Status log
- 2026-08-06: dependent map built via exhaustive grep (3 passes, caught gaps each time — always re-verify with a broad alias-key grep, not just the obvious plugin names). 4 background agents dispatched (2d=sonnet, 3d=opus, mindmap=sonnet, imperative=sonnet), each sent a correction message with the extra dependents found after initial dispatch. Root workspace transiently broken by unrelated concurrent flow W6 session (missing manifest at ✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/⚡️implementations/🦀️rust) — not my bug, polling.
