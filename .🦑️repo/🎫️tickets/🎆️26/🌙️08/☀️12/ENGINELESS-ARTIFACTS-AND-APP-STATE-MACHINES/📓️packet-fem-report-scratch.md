# Packet report — `🏗️fem` (workstream A, Tier B)

Scope: `✏️s/🔌️plugins/🏗️fem` only, plus the two new fem-only destination dirs. Did not touch gis, procedural,
or any other plugin.

## Destinations chosen per region

### `🗿️artifacts/◻2d/…/⚙️engine` (5,751 LOC)

| region | destination | note |
|---|---|---|
| `Fem2dEngine` struct + `impl` | **DELETE** | fields were just `artifact`/`snapshot`, 0 external refs, implements no trait — the fossil `*Engine` shape |
| `empty_fem2d_snapshot()` | `🧬️schema/🦀️component.rs`, new region `🌱️DerivedEmpty` | pure document helper, matches exemplar's `empty_block2d_snapshot` |
| `fem2d_io()` / `fem2d_geometry_in_port()` / `fem2d_results_out_port()` | `🎛️apps/◻2d/🦀️component.rs`, new region `🔌️Io` | returns `AppIo`/`MediaPortSpec` — app types |
| `Fem2dError` enum + `build_model`/`fem2d_solve`/`fem2d_solve_all` | `✏️s/🔨️modules/🏗️fem/⚙️engine/◻2d/🦀️component.rs` (NEW) | heavy FE solver entry points — see "numerical placement" below |
| `io_registry` mod (`ComposerEntry` table + export composers) | `🚪️io/🦀️component.rs`, new region `🚪️IoRegistry` | per destination map item 5 |
| `🏗️model`, `🧮️analyses`, `📏️elements2d`, `➗️formulation`, `🔢️sparse` | `✏️s/🔨️modules/🏗️fem/⚙️engine/<name>/` (NEW, crate-root mount names unchanged) | shared cross-artifact kernel, moved verbatim |
| `🕸️meshing` | `✏️s/🔨️modules/🏗️fem/⚙️engine/◻2d/🕸️meshing/` (NEW) | 2D-specific region meshing, pure algorithm |
| `🎵️modal-buckling` | `✏️s/🔨️modules/🏗️fem/⚙️engine/◻2d/🎵️modal-buckling/` (NEW) | pure algorithm (eigen-analysis) |
| `🗺️mesh-preview` | `✏️s/🔨️modules/🏗️fem/⚙️engine/◻2d/🗺️mesh-preview/` (NEW) | pure algorithm (triangulation + stress averaging) |
| Tests | split with subject (see assertion table below) | every assertion accounted for |

### `🗿️artifacts/🧊️3d/…/⚙️engine` (3,672 LOC)

| region | destination | note |
|---|---|---|
| `Fem3dEngine` struct + `impl` | **DELETE** | same fossil shape as `Fem2dEngine`, 0 external refs |
| `empty_fem3d_snapshot()` | `🧬️schema/🦀️component.rs`, new region `🌱️DerivedEmpty` | pure document helper |
| `fem3d_io()` / ports | `🎛️apps/🧊️3d/🦀️component.rs`, new region `🔌️Io` | returns `AppIo`/`MediaPortSpec` |
| `Fem3dError` + `build_model`/`fem3d_solve`/`fem3d_solve_all` | `✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🦀️component.rs` (NEW) | heavy FE solver entry points |
| `SceneRender` region (`fem3d_scene_parts`, `fem3d_camera_json`, quat helpers, `fem3d_structural_instances`, `fem3d_solid_mesh_entries`, `fem3d_deformed_position`, `find_node_3d`, `fem3d_element_endpoints`, `NODE_SIZE_3D`/`MEMBER_THICKNESS_3D`) | `🎛️apps/🧊️3d/🦀️component.rs`, new region `🎬️SceneRender` | references `crate::app_surface` (an app type) directly and is consumed only by the model+results windows — app behaviour, not pure algorithm |
| `io_registry` mod | `🚪️io/🦀️component.rs`, new region `🚪️IoRegistry` | |
| `🧊️elements3d`, `🕸️mesh` | `✏️s/🔨️modules/🏗️fem/⚙️engine/<name>/` (NEW) | shared cross-artifact kernel, moved verbatim (`mesh` is physically the SAME file 2D's mesh-preview/meshing also call via `crate::mesh::…`) |
| `🕸️meshing` | `✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🕸️meshing/` (NEW) | 3D-specific solid meshing, pure algorithm — content genuinely differs from 2D's (confirmed by reading both: 2D meshes `FemRegion` polygons into `Tri3Cst`, 3D meshes `FemSolid` footprints into `Tet4` via extrusion) |
| `🎵️modal-buckling` | `✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🎵️modal-buckling/` (NEW) | pure algorithm |
| `🗺️mesh-preview` | `✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/🗺️mesh-preview/` (NEW) | pure algorithm (tet mesh + boundary + nodal stress) |
| Tests | split with subject | every assertion accounted for |

## fem-specific numerical-code placement reasoning (per subdir, as required)

- **`🏗️model`, `🧮️analyses`, `📏️elements2d`, `🧊️elements3d`, `➗️formulation`, `🔢️sparse`** — exactly the
  subdirs the packet names as the shared cross-artifact kernel. Verified by reading: single physical
  copy each (not duplicated per artifact), consumed by BOTH `fem2d`/`fem3d` via bare `crate::model::…`,
  `crate::analyses::…` etc. Moved verbatim to `✏️s/🔨️modules/🏗️fem/⚙️engine/<name>/`; crate-root mount
  **names** (`model`, `analyses`, …) are unchanged, only the `#[path]` **targets** moved — so every
  existing `crate::model::X`-style call site across the whole plugin (apps, schema, everywhere) needed
  zero edits.
- **`🕸️mesh`** — physically lived under the 3D artifact tree but is the SHARED triangulation/extrusion
  primitive (`crate::mesh::triangulate`/`extrude_tri_mesh`/`split_to_tets`/`boundary_faces`) also called
  directly by 2D's own `mesh-preview`/`meshing`. Moved verbatim to `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/`,
  single copy, same crate-root mount name `mesh`.
- **`🕸️meshing`** — READ BOTH before deciding, per the packet's explicit instruction. They differ: 2D's
  meshes `FemRegion` polygons into `Tri3Cst` plane-stress elements; 3D's meshes `FemSolid` footprints
  into `Tet4` volume elements via extrusion+tet-splitting, and additionally translates `FemLoad::Area`.
  Genuinely artifact-specific pure algorithm (not shared) — kept as two separate subdirs nested under
  new `◻2d/`/`🧊️3d/` groupings inside the module engine (mirrors the taxonomy convention this exact
  plugin already uses at both the artifact and app level for the same 2D/3D split).
- **`🎵️modal-buckling`** — READ per artifact. Both are pure numerical eigenanalysis bridges (modal
  frequencies/shapes, linear buckling load factors/shapes) built directly on the shared `analyses`/
  `sparse` kernel — no `AppIo`, no app types, no snapshot-only-inference shape (they call heavy solver
  machinery, not a cheap derived field). Classified D6 pure algorithm → module engine, per-artifact
  subdir (2D's `fem2d_modal`/`fem2d_buckling` vs 3D's `fem3d_modal`/`fem3d_buckling` — different names,
  genuinely different fixture/geometry resolution, not shared).
- **`🗺️mesh-preview`** — READ per artifact. Judged NOT app-preview rendering despite the name: both
  `fem2d_mesh_preview`/`fem3d_mesh_preview` do real triangulation/tet-meshing (calling `crate::mesh`),
  and `fem2d_nodal_von_mises`/`fem3d_nodal_von_mises` run a real stress-averaging solve
  (`crate::analyses::nodal_averaged_scalar`) — cheap-enough-for-every-render geometry/stress computation,
  not app-facing rendering (no `crate::app_surface`, no scene-JSON, no `AppIo`). Classified D6 pure
  algorithm → module engine, per-artifact subdir. Contrast with fem3d's root-file `SceneRender` region
  (`fem3d_scene_parts` etc.), which DOES reference `crate::app_surface` and DOES build scene JSON for
  the windows — that one went to `🎛️apps/🧊️3d/`, not here.

## Unqualified paths qualified (before → after)

Per the io-registry-shadow-list census, `🏗️fem/🗿️artifacts/◻2d` and `🏗️fem/🗿️artifacts/🧊️3d` were already
listed shadow-present with the `declaration()` call site already qualified — reconfirmed unchanged
(`.composers(crate::artifacts::fem2d::standards::v1::subsets::any::io::io_registry::entries())` /
fem3d equivalent, repointed to the new `io/` location, still fully qualified).

Relocated-body qualifications made in this packet (every one of these was a bare/engine-qualified path
inside a moved function body or `use`, now qualified against its NEW home):

| before | after | where |
|---|---|---|
| `crate::artifacts::fem2d::engine::meshing::{…}` | `crate::fem2d_engine::meshing::{…}` | new `◻2d/🦀️component.rs` (module engine), `◻2d/🎵️modal-buckling`, `◻2d/🗺️mesh-preview` |
| `crate::artifacts::fem2d::engine::Fem2dError` | `crate::fem2d_engine::Fem2dError` | `◻2d/🕸️meshing`, `◻2d/🎵️modal-buckling`, `◻2d/🗺️mesh-preview` |
| `crate::artifacts::fem2d::engine::mesh_preview::…` / `modal_buckling::…` | `crate::fem2d_engine::mesh_preview::…` / `crate::fem2d_engine::modal_buckling::…` | module-engine `◻2d/🦀️component.rs` test; `🎛️apps/◻2d` windows/commands |
| `crate::artifacts::fem2d::engine::fem2d_solve_all` | `crate::fem2d_engine::fem2d_solve_all` | `🎛️apps/◻2d/🦀️component.rs`, results window |
| `crate::artifacts::fem2d::engine::empty_fem2d_snapshot` | `crate::artifacts::fem2d::schema::empty_fem2d_snapshot` | apps root, wasm, windows, commands/example (14 call sites total) |
| `crate::artifacts::fem2d::engine::fem2d_io` | local `fem2d_io()` (now defined in the same file) | apps root (3 call sites) |
| `crate::artifacts::fem3d::engine::{meshing, mesh_preview, Fem3dError}` | `crate::fem3d_engine::{meshing, mesh_preview, Fem3dError}` | module-engine `🧊️3d/*` files |
| `crate::artifacts::fem3d::engine::fem3d_solve_all` | `crate::fem3d_engine::fem3d_solve_all` | `🎛️apps/🧊️3d/🦀️component.rs`, results window |
| `crate::artifacts::fem3d::engine::mesh_preview::fem3d_nodal_von_mises` | `crate::fem3d_engine::mesh_preview::fem3d_nodal_von_mises` | results window |
| `crate::artifacts::fem3d::engine::modal_buckling::{fem3d_modal_mode_values, fem3d_buckling_mode_values}` | `crate::fem3d_engine::modal_buckling::{…}` | results window |
| `crate::artifacts::fem3d::engine::empty_fem3d_snapshot` | `crate::artifacts::fem3d::schema::empty_fem3d_snapshot` | apps root (3), wasm (1) |
| `crate::artifacts::fem3d::engine::{fem3d_camera_json, fem3d_scene_parts}` | `crate::apps::fem3d::{fem3d_camera_json, fem3d_scene_parts}` | model window, results window (×3) |
| `crate::artifacts::fem3d::engine::fem3d_io` | local `fem3d_io()` | apps root (2) |
| `crate::artifacts::fem2d::standards::v1::engine::io_registry::entries()` (declaration composers) | `crate::artifacts::fem2d::standards::v1::subsets::any::io::io_registry::entries()` | artifact-root `declaration()`, both 2D and 3D (fem3d equivalent too) |
| `crate::artifacts::fem2d::standards::v1::engine::io_registry as v1` (artifact root's own shadow wrapper `use`) | `crate::artifacts::fem2d::standards::v1::subsets::any::io::io_registry as v1` | artifact-root `io_registry` wrapper mod, both 2D and 3D |
| `crate::artifacts::fem2d::engine::meshing::build_semio_mesh_snapshot` / fem3d equivalent | `crate::fem2d_engine::meshing::build_semio_mesh_snapshot` / `crate::fem3d_engine::…` | `.obj`/`.stl` export serializers, both artifacts |

**io_registry shadow trap**: the artifact root's own `io_registry` wrapper module (`.iter().collect()`
view, different `&[&ComposerEntry]` type) was NOT touched except its one `use … as v1;` repoint — every
new reference into the relocated `io_registry::entries()` from elsewhere in the crate is written fully
qualified (`crate::artifacts::fem2d::standards::v1::subsets::any::io::io_registry::entries()` in
`declaration()`), so no bare call anywhere resolves to the shadow.

## Assertion-count arithmetic (before → after, `git show HEAD:<path>` vs current)

All counts via `grep -o 'assert!\|assert_eq!\|assert_ne!'` / `grep -o '#\[test\]'`, not eyeballed.

| file (before, HEAD) | asserts | tests |
|---|---:|---:|
| `◻2d/⚙️engine/🦀️component.rs` (root) | 52 | 15 |
| `◻2d/⚙️engine/🎵️modal-buckling` | 12 | 5 |
| `◻2d/⚙️engine/🗺️mesh-preview` | 7 | 2 |
| `◻2d/⚙️engine/🕸️meshing` | 0 | 0 |
| `◻2d/⚙️engine/🏗️model` | 13 | 9 |
| `◻2d/⚙️engine/🧮️analyses` | 56 | 16 |
| `◻2d/⚙️engine/📏️elements2d` | 57 | 32 |
| `◻2d/⚙️engine/➗️formulation` | 16 | 10 |
| `◻2d/⚙️engine/🔢️sparse` | 26 | 15 |
| `🧊️3d/⚙️engine/🦀️component.rs` (root) | 56 | 17 |
| `🧊️3d/⚙️engine/🎵️modal-buckling` | 12 | 5 |
| `🧊️3d/⚙️engine/🗺️mesh-preview` | 8 | 2 |
| `🧊️3d/⚙️engine/🕸️meshing` | 0 | 0 |
| `🧊️3d/⚙️engine/🧊️elements3d` | 54 | 25 |
| `🧊️3d/⚙️engine/🕸️mesh` | 34 | 17 |
| **total moved** | **403** | **170** |

| destination (after) | asserts | tests | delta vs before-total at same path |
|---|---:|---:|---|
| module engine `◻2d/🦀️component.rs` | 37 | 14 | -15/-1 (Io test moved out, matches exactly) |
| module engine `◻2d/🎵️modal-buckling` | 12 | 5 | 0/0 (verbatim) |
| module engine `◻2d/🗺️mesh-preview` | 7 | 2 | 0/0 |
| module engine `◻2d/🕸️meshing` | 0 | 0 | 0/0 |
| module engine `🏗️model` | 13 | 9 | 0/0 |
| module engine `🧮️analyses` | 56 | 16 | 0/0 |
| module engine `📏️elements2d` | 57 | 32 | 0/0 |
| module engine `➗️formulation` | 16 | 10 | 0/0 |
| module engine `🔢️sparse` | 26 | 15 | 0/0 |
| module engine `🧊️3d/🦀️component.rs` | 35 | 12 | -21/-5 (Io test + 4 SceneRender tests moved out, matches exactly) |
| module engine `🧊️3d/🎵️modal-buckling` | 12 | 5 | 0/0 |
| module engine `🧊️3d/🗺️mesh-preview` | 8 | 2 | 0/0 |
| module engine `🧊️3d/🕸️meshing` | 0 | 0 | 0/0 |
| module engine `🧊️elements3d` | 54 | 25 | 0/0 |
| module engine `🕸️mesh` | 34 | 17 | 0/0 |
| **module engine subtotal** | **367** | **164** | |
| `🎛️apps/◻2d/🦀️component.rs` | 56 (was 41) | 18 (was 17) | **+15/+1** |
| `🎛️apps/🧊️3d/🦀️component.rs` | 49 (was 28) | 17 (was 12) | **+21/+5** |
| `🧬️schema/🦀️component.rs` ×2 | 0/0 both | 0/0 both | 0/0 (plain fn, no test of its own, same as before) |
| `🚪️io/🦀️component.rs` ×2 | 0/0 both | 0/0 both | 0/0 (io_registry never had a dedicated test) |
| **apps delta subtotal** | **+36** | **+6** | |
| **GRAND TOTAL after** | **367 + 36 = 403** | **164 + 6 = 170** | **matches 403/170 exactly — zero assertions or tests lost** |

Only deletion: `Fem2dEngine`/`Fem3dEngine` structs + their `new()` — 0 tests, 0 asserts, verified.

## Compiler check

Command (exact, per rule 6):
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES/🎯️target" cargo check -p semio-s-plugin-fem --all-targets
```
Output tail: **see appended section below** — filled in after the run completes (kicked off as a long
background job; the ticket's own guidance says a fresh isolated `CARGO_TARGET_DIR` legitimately takes a
long time on first build and `Blocking waiting for file lock` is normal, not a hang).

## Structural verification

```
$ find ✏️s/🔌️plugins/🏗️fem -path "*🗿️artifacts*" -name "⚙️engine" -type d
(zero results)

$ grep -rn "::engine::\|standards::v1::engine" ✏️s/🔌️plugins/🏗️fem --include="*.rs"
(8 hits, ALL in obj/stl export serializers, ALL referencing semio_s_plugin_stdio's OWN engine —
 e.g. `use semio_s_plugin_stdio::artifacts::obj::standards::v3_0::engine::encode_obj;` — a different
 plugin's real, un-dissolved module, out of scope per the ticket's stdio boundary. Zero hits reference
 fem's own (deleted) engine.)
```

`grep -n "⚙️engine\|::engine::" 📦️glue.rs` (the CRITICAL section's own, stricter, glue.rs-only check):
does **NOT** literally return zero — it returns 18 lines, all `#[path]` string literals / doc-comment
prose pointing at the NEW, legal `✏️s/🔨️modules/🏗️fem/⚙️engine/` module directory (plus the pre-existing,
explicitly-out-of-scope `🎛️apps/◻2d/⚙️engine/🖥️app-surface/` mount, untouched per instruction). This is
a direct, deliberate consequence of the packet's own fem-specific override ("pure algorithms … move up
one level" into a NEW `⚙️engine`), which necessarily still contains the literal substring `⚙️engine` in
its path strings. The narrower, semantically-correct check — `::engine::` as Rust MODULE-PATH syntax
(not a file-path string) — **is** zero in `📦️glue.rs`; confirmed separately below.

```
$ grep -n "::engine::" 📦️glue.rs
(zero results)
```

## Concurrent-churn observations

None observed affecting fem. All edits stayed inside `✏️s/🔌️plugins/🏗️fem` and the two new fem-only
destination dirs named in the dispatch.

## Files touched

**Deleted:**
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/` (whole dir)
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/` (whole dir)

**Created:**
- `✏️s/🔨️modules/🏗️fem/⚙️engine/{🏗️model,🧮️analyses,📏️elements2d,🧊️elements3d,➗️formulation,🕸️mesh,🔢️sparse}/🦀️component.rs`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/◻2d/{🦀️component.rs,🕸️meshing/🦀️component.rs,🎵️modal-buckling/🦀️component.rs,🗺️mesh-preview/🦀️component.rs}`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧊️3d/{🦀️component.rs,🕸️meshing/🦀️component.rs,🎵️modal-buckling/🦀️component.rs,🗺️mesh-preview/🦀️component.rs}`

**Modified:**
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🦀️component.rs`, `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🦀️component.rs`
- `.../🧬️schema/🦀️component.rs` ×2 (2D, 3D)
- `.../🚪️io/🦀️component.rs` ×2 (2D, 3D)
- `.../🚪️io/📤️export/🧵️serializers/🗿️artifacts/{🧊️obj/🔖️3.0,🟪️stl/🔖️ascii}/✳️any/🦀️component.rs` ×4 (2 formats × 2 artifacts)
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🦀️component.rs`, `✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🦀️component.rs`
- `.../🎛️apps/◻2d/🌉️wasm/🦀️component.rs`, `.../🎛️apps/🧊️3d/🌉️wasm/🦀️component.rs`
- `.../🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️component.rs`, `.../📊️results/🦀️component.rs`
- `.../🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️component.rs`, `.../📊️results/🦀️component.rs`
- `.../🎛️apps/◻2d/🎮️commands/📚️example/🦀️component.rs`

## Honest pass/fail

**PENDING compiler run** — see appended tail below once the background check completes. Structural
greps and assertion-count conservation are both verified and green as of this writing.
