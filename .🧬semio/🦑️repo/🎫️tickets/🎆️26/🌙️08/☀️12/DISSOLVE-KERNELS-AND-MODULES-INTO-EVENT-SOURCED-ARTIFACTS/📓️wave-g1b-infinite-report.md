# Wave G1b — `semio-framework-os-infinite`: dissolve mesh-engine geometry imports

## Boundary
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🦀️component.rs`

## 1. Dual-copy trap — confirmed real, handled

**Before any edit:** `diff -q` on the two files produced no output (byte-identical, 4092 lines
each). `grep` of `♾️infinite/📦️packages/🦀️rust/📦️glue.rs` confirmed both are path-mounted:
- line 30: `#[path = "../../🦀️component.rs"] mod component;` + `pub use component::*;`
- line 34: `#[path = "../../🌍️world/🦀️component.rs"] pub mod world;` + `pub use world::*;`

So this crate really does compile the same ~4k-LOC file twice under two module names. Every edit
was made to `🌍️world/🦀️component.rs` and then propagated with `cp` onto the sibling path (not
hand-duplicated — `cp` guarantees byte-identity by construction, then verified).

**After all edits:** `diff -q` again produced no output — both files are 4343 lines, byte-identical.
Confirmed a second time as the very last step before writing this report.

## 2. Census of the four named symbols

All from `use semio_framework::{mesh_from_glb, mesh_from_kind, optional_json_to_dsl, MeshData};`
(pre-edit, line 12) — `Mesh3d` is **not** one of these; it comes from `ui_wgpu::wgpu` (the wgpu
renderer crate), not `semio_framework::`, and was out of scope untouched.

### `MeshData` (job 1 — render-buffer field)
- `struct WorldMeshRecord { data: Option<MeshData>, ... }` (pre-edit line 79) — the document-JSON
  record deserialized from `world.meshes_json`.
- `fn mesh_from_data(data: &MeshData) -> Mesh3d` (pre-edit line 1053) — converts to the wgpu
  render mesh.
- `pub fn ingest_glb_mesh(state, url, mesh: MeshData, mesh_id)` (pre-edit line 2482) — glb-decode
  consumer (job 3).
- Never constructed as a struct literal anywhere in this file; `data.colors` is never read despite
  the field existing on the wire type.

### `mesh_from_kind(&str)` (job 2 — placeholder factory)
Call sites and their **actual** kind arguments, censused before porting anything:
| site (pre-edit line) | kind argument | reachable? |
|---|---|---|
| 722 `rebuild_instance_draws` | `&logical_mesh_id` (arbitrary, document-supplied `instance.mesh_id`, defaults to `"box"`) | yes, but falls through to the `_` arm for anything not literally box/plane/vortex-marker/cylinder/cone |
| 1990 | `"box"` | yes |
| 2018 | `"plane"` | yes |
| 2042 | `"plane"` | yes |
| 3038 `ensure_primitive_mesh`, called at 3072–3074 with `"vortex-marker"`, `VORTEX_ARROW_SHAFT_MESH = "cylinder"`, `VORTEX_ARROW_HEAD_MESH = "cone"` | yes |

`"vertex-marker"` (`VERTEX_MARKER_MESH` const, pre-edit line 1154) is looked up as a **mesh-pool
key** at lines 1975–1979 but is **never** passed as a `kind` argument to `mesh_from_kind` anywhere
in this file — so it was never actually reachable and was **not** ported.

Only 5 of the mesh-engine's 9 match arms are reachable from this file: `box` (default fallback),
`plane`, `vortex-marker` (→ `mesh_ico_sphere(0.12, 1)`), `cylinder` (→ `mesh_cylinder(0.5, 1.0,
16)`), `cone` (→ `mesh_cone(0.5, 1.0, 16)`). `sphere`/`uvSphere`, `icoSphere`, `torus`,
`vertex-marker` are **not** reachable and were **not** ported — porting them would have been
scope creep the mission explicitly warned against ("don't port all seven blindly").

### `mesh_from_glb(&[u8]) -> Result<MeshData, String>` (job 3 — real GLB decode)
- Single call site: `apply_glb_bytes` (pre-edit line 3255), invoked from the `#[cfg(not(wasm32))]`/
  `#[cfg(wasm32)]` fetch pipelines that read bytes from a URL and hand them to `ingest_glb_mesh`.
- Exercised by a real test with real asset bytes: `dropped_puzzle_object_glb_becomes_renderable`
  (pre-edit line 3893) decodes
  `🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/🧊️capsule_J.glb` and asserts exact vertex
  (1472) and triangle (1750) counts — this is a real, load-bearing decode path, not a stub.

## 3. What each symbol became, and why

**Job 1 (`MeshData` field).** Replaced with a new infinite-owned `struct WorldMeshBuffers`
(`🌍️world/🦀️component.rs`, region `WorldMeshBuffers`), mirroring the renderer's `WorldMeshData`
type (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/World3dHost/🟦️component.tsx`
line 98) field-for-field: `positions`, `normals`, `indices`, `colors` (kept for wire-schema parity
with the TS twin even though never read on this native path — TS itself notes it's a "react-
renderer-only capability for now"; marked `#[allow(dead_code)]`), `uvs`, `face_ids`, `vertex_ids`,
`edge_positions`, `edge_ids`, `paint_texture_base64`. Dropped `edge_uvs`/`edge_is_seam` from the
old `MeshData` — neither is read anywhere in this file, and neither exists on the TS twin, so
carrying them would have been dishonest padding, not an honest mirror. `WorldMeshRecord.data` and
`mesh_from_data`'s parameter now use this type.

**Job 2 (`mesh_from_kind`).** Replaced with a local `fn placeholder_mesh(kind: &str) ->
WorldMeshBuffers` (region `PlaceholderMesh`) plus `placeholder_box/plane/cylinder/cone/ico_sphere`
+ their shared helpers (`placeholder_push_triangle`, `placeholder_normalize3`,
`placeholder_scale3`, `placeholder_midpoint`), ported verbatim (same vertex math) from
`🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs`'s `mesh_box`/`mesh_plane`/
`mesh_cylinder`/`mesh_cone`/`mesh_ico_sphere`/`push_triangle`/normalize/midpoint helpers — but
only the 5 reachable kinds, per the census above. `WorldMeshBuffers::compute_normals` (inherent
method) replaces `MeshData::compute_normals`.

**Job 3 (`mesh_from_glb`) — left blocked, call site unchanged.** Investigated routing through
stdio's gltf→mesh artifact facet:
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️component.rs`
(`struct SemioMeshFromGltf: ArtifactDeserializer<From = GltfDocument, Into = SemioMesh-snapshot-ish
type via crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::*>`). That trait
(`semio_framework_os_kernel`/plugin's `ArtifactDeserializer`, defined in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` line 450) produces a *structured
document snapshot* (primitives/materials/textures, registered through the CQRS
`io_dispatch`/`io_resolve` machinery) — not the flat `positions`/`normals`/`indices`/`uvs`
render-buffer shape this viewport needs. Routing `mesh_from_glb` through it honestly would require:
(a) confirming/building a GLB-bytes→`GltfDocument` decode step distinct from mesh-engine's own
`gltf`-crate-based decoder, (b) invoking the deserializer to get a mesh snapshot, and (c) a new
snapshot→flat-buffer flattening adapter (interleaving possibly-multiple primitives into one flat
mesh) that has no existing precedent in this codebase. That's a real facet-to-render-buffer
architecture change, not a bounded edit — per the mission's explicit instruction, it was **not**
faked. `apply_glb_bytes` (unchanged) still calls `semio_framework::mesh_from_glb`, and its
`MeshData` result is converted into `WorldMeshBuffers` via a new field-copy-only
`impl From<&MeshData> for WorldMeshBuffers` immediately before `mesh_from_data` — no geometry
logic re-enters the framework dependency, only a plumbing adapter for the still-external decode
result. `semio-s-plugin-stdio` was **not** added as a dependency (Cargo.toml untouched, verified
with `git diff --stat`) since it was never needed for jobs 1/2 and job 3 stayed blocked.

Net result: `semio_framework::{mesh_from_kind, MeshData-as-record-field}` are gone from this file.
`semio_framework::{mesh_from_glb, MeshData-as-glb-result}` remain, scoped to exactly one call
site and one adapter impl, both clearly marked with a `🚧️` doc comment pointing at this report.

## 4. Verification

Ran the mandatory exact form three times: once as a **baseline before any edit**, once
immediately after the code changes, and once as the **final** check after the last edit
(the `#[allow(dead_code)]` tweak on `WorldMeshBuffers::colors`):

```
TD="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS/🎯️target"
touch "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs"
RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-framework-os-infinite --all-targets
```

**All three runs produced the identical result** — `cargo check` never reached
`semio-framework-os-infinite` at all. It fails earlier, while checking `semio-framework-ui`
(a dependency of `ui_wgpu`, which `semio-framework-os-infinite` depends on for `Mesh3d`/`GpuContext`/
etc.):

```
error[E0433]: cannot find module or crate `semio_framework_math` in this scope
 --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/../../../../../🧊️3d/🎬️scene/🦀️component.rs:3:9
  |
3 | pub use semio_framework_math::algebra::{Mat4, Vec3};

error[E0432]: unresolved import `crate::wgpu::Vec3`
   --> 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️widgets.rs:594:45

error[E0689]: can't call method `abs`/`sqrt` on ambiguous numeric type `{float}` (x4)
   --> .../🧊️3d/🎬️scene/🦀️component.rs:842, 861, 864, 867

error: could not compile `semio-framework-ui` (lib) due to 6 previous errors; 5 warnings emitted
```
(7 `error[...]`/`error:` lines total in each of the three run logs — byte-identical error count
and locations across baseline, after-edit, and final.)

**Attribution — this is foreign blocked-churn, not mine:**
- `🧊️3d/🎬️scene/🦀️component.rs` does `pub use semio_framework_math::algebra::{Mat4, Vec3};`, but
  `semio-framework-ui`'s `Cargo.toml` does not declare `semio-framework-math` as a dependency
  (`grep -n "semio-framework-math\|semio_framework_math"` on
  `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` returns nothing).
- Both files are outside my declared boundary (`🧰️framework/🔨️modules/🧊️3d/…` and
  `🧰️framework/🔨️modules/🖱️ui/…`), and `🧊️3d` is literally one of the modules this repo-wide
  mandate says "must cease to exist" — consistent with this being another concurrent wave's
  in-progress dissolution of `🧊️3d`, not a bug in my crate.
- `git status --short` on both files is clean (no uncommitted change), and `stat -f '%Sm %N'`
  shows both last modified ~a week before "today" per the session clock — this is a standing break
  at HEAD, not a transient in-flight edit racing my checks. Per repo convention ("Never fix another
  session's file... report blocked-churn, move on"), I did not touch either file.

**Baseline-vs-after error comparison for `semio-framework-os-infinite` itself: not obtainable.**
Every attempt (before my edit and after) was blocked upstream before reaching this crate, so there
is no cargo-verified before/after error delta for my own file to report — only that the upstream
blocker is unchanged by my edits (present identically before and after). I did not claim compile
success anywhere; the crate's own correctness rests on manual review only (see below), which is an
honest gap, not a claimed pass.

**Manual review performed in lieu of a live compile signal:**
- All `WorldMeshBuffers` field names/types match every read site (`data.face_ids`, `data.uvs`, ...
  in `mesh_from_data`, unchanged field names).
- `placeholder_mesh` call sites still destructure `primitive.positions/normals/indices` — same
  field names as before, so those call sites needed no further edits beyond the function-name swap.
- `ingest_glb_mesh`'s signature is untouched (`mesh: MeshData`); only its body changed to convert
  via `WorldMeshBuffers::from(&mesh)` before calling `mesh_from_data`.
- `Deserialize` was already imported (`use serde::Deserialize;`, pre-edit line 20) — no new import
  needed for `WorldMeshBuffers`'s derive.
- No Cargo.toml edits were made (confirmed via `git diff --stat`), so no
  `RUSTC_WRAPPER="" cargo metadata --no-deps` run was needed for this wave.

## 5. Honest remainders

1. **Job 3 (`mesh_from_glb`) is blocked**, exactly as the mission anticipated as an acceptable
   outcome. `semio_framework::{mesh_from_glb, MeshData}` remain imported, scoped to
   `apply_glb_bytes`/`ingest_glb_mesh`/the new `From<&MeshData> for WorldMeshBuffers` adapter, each
   marked with a `🚧️` comment pointing back here. Real follow-up work (not done here, out of
   bounded-edit scope): confirm whether stdio already has a GLB-container-bytes decoder feeding
   `GltfDocument`, then build the `SemioMesh`-snapshot → flat-buffer flattening adapter.
2. **No live `cargo check -p semio-framework-os-infinite` pass/fail signal was obtainable** for
   this wave, for reasons entirely outside this ticket's boundary (see §4). This should be re-run
   once the `🧊️3d`/`semio-framework-ui` breakage is resolved by whichever session owns it.
3. The pre-existing "~12 errors ... `DslValue` indexing and a missing `🧊️capsule_J.glb` asset"
   noted in the mission brief were **not observed** in any of my three runs — the asset in question
   is in fact present (`dropped_puzzle_object_glb_becomes_renderable` includes it via
   `include_bytes!`) and the build never got far enough to reach that region anyway. Not fixed,
   not counted as mine either way — just noting the mismatch with the mission's stated baseline for
   whoever reconciles these tickets.

## Files touched
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️component.rs` (edited)
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🦀️component.rs` (kept byte-identical via `cp`, verified)

No other files were created or modified. Scratch verification logs:
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS/scratch-g1b-baseline.txt`,
`scratch-g1b-after1.txt`, `scratch-g1b-final.txt`.
