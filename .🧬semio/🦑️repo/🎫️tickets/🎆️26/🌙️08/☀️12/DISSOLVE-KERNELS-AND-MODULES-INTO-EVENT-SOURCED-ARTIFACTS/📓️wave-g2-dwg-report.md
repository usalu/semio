# Wave G2 — `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` DWG codec relocation

## Status: **steps 1–3 done and verified; step 4 (deletion) is `blocked` — on a wider set of framework-layer callers than the ticket anticipated**

## 1. Reading Decision #5 / the ac1018 vs ac1024 split

Read in full before touching anything:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🦀️component.rs` (root declaration, Decision #5 provenance)
- `.../🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` (frozen legacy shim: `decode_dwg` scans a generic `AC10xx` sentinel + a couple of header bytes, never determines per-section byte ranges)
- `.../🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` and `.../🚪️io/🦀️component.rs` (the real D1/D2 R2004+ decoder: file-header LCG decrypt, page-directory walk, bespoke LZ77-variant decompressor, validated against the real ~145 KB `architectural.dwg` fixture)

Confirmed: ac1018 is a deliberately frozen legacy shim (its own `DwgSnapshot::section_names` doc says so, citing Decision #5); ac1024 is canonical and has a genuine, non-trivial, fixture-validated real-DWG decode pipeline that locates and decompresses named sections but **never reaches entity/layer bitcode** (D3/D4 out of scope, documented in the io module's own header). Neither ac1018 nor ac1024's real decode path builds anything resembling `DwgDrawing`/`DwgEntity`/`DwgGeometry`.

## 2. The design decision — **(b), with a twist discovered by reading, not guessed**

`🔺️mesh/🦀️component.rs`'s own docstring says the codec writes **`"AC1015"` file magic** with its own **home-grown section-locator/CRC/handle container** — structurally nothing like the real AC1024/R2004+ page-based format ac1024's `🚪️io` decodes. It is a **self-contained, round-trippable format this codec invented for itself**: `dwg_from_bytes` can only decode bytes `dwg_to_bytes` itself produced, never a real AutoCAD file (confirmed: its own docstring says "byte-exact third-party AutoCAD/ODA interop needs follow-up validation against a real DWG viewer").

Crucially, `ac1024/🚪️io/🦀️component.rs` **already established the exact precedent this wave needed**, in its own module doc, for functions in the identical position (`decrypt_r2004_header`, `decompress_r2004_section`): *"Pure byte↔byte algorithms with no `DwgSnapshot` dependency of their own — kept here per ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES rule 6."* The relocated DWG codec has exactly that shape: zero `DwgSnapshot` dependency, pure `bytes ↔ DwgDrawing` (and `MeshData`/`DwgPathSegment` at the edges). So: **option (b)** — land the whole codec (`DwgDrawing`/`DwgLayer`/`DwgColor`/`DwgEntity`/`DwgGeometry`/`DwgPathSegment`, the bit reader/writer, `dwg_to_bytes`/`dwg_from_bytes`, `mesh_to_dwg_drawing`/`dwg_drawing_to_mesh`, `paths_to_dwg_drawing`/`dwg_drawing_to_paths`) inside `ac1024/🪆️subsets/✳️any/🚪️io/🦀️component.rs`, **`DwgSnapshot` completely untouched**. This is the option that requires the fewest lies: it doesn't pretend this hand-rolled codec is real DWG interop, and it doesn't disturb the well-provisioned, Decision-#5-respecting `DwgSnapshot` type or its live consumers.

Independent confirmation this is right, found empirically (not assumed): the pre-existing `raster`/`note`/`layout` DWG deserializer leaves already do exactly this — `decode_dwg(bytes)` (ac1018's generic sentinel scan, into `DwgSnapshot`, for provenance/metadata) **and separately** `dwg_from_bytes(bytes)` (the mesh-module codec, into `DwgDrawing`) **on the same byte buffer**, i.e. `DwgDrawing` was already treated as a structural side-decode of `DwgSnapshot.bytes`, not a replacement for it.

One extraction beyond a pure move: `dwg_drawing_to_paths`'s per-entity match body was factored out into a new `pub fn dwg_geometry_to_path_segments(&DwgGeometry) -> Option<Vec<DwgPathSegment>>` (both old and new call sites produce byte-identical output — covered by the relocated `dwg_path_bridge_round_trips_cubic_control_points_exactly` test). This was necessary because the new `✳️drawing` bridge needs to walk entities one at a time (to keep each path's originating layer), which the original all-flattened `Vec<Vec<DwgPathSegment>>` return shape can't offer (non-path geometry kinds are silently skipped, desyncing any by-index zip against `drawing.entities`).

## 3. Where the code landed

| Old location | New location |
|---|---|
| `DwgDrawing`/`DwgLayer`/`DwgColor`/`DwgEntity`/`DwgGeometry`, bit reader/writer, `dwg_to_bytes`/`dwg_from_bytes`, `DwgPathSegment`, `paths_to_dwg_drawing`, `dwg_drawing_to_paths` (now via new `dwg_geometry_to_path_segments`), `mesh_to_dwg_drawing`, `dwg_drawing_to_mesh` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🚪️io/🦀️component.rs`, new `//#region 🔖️DwgStructuralCodec` (verbatim except the `dwg_geometry_to_path_segments` extraction) |
| — (re-export for convenient callers) | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🦀️component.rs` now `pub use`s all of the above from `standards::v_ac1024::subsets::any::io`, so callers can reach them as `semio_s_plugin_stdio::artifacts::dwg::{DwgDrawing, dwg_to_bytes, ...}` |
| 9 DWG-codec tests (of the file's 29) | same `io/🦀️component.rs`, new `//#region 🔖️RelocatedDwgCodecUnit` inside the existing `mod tests` |
| 20 mesh-format tests (testing `semio_framework_mesh_engine`'s own `mesh_box`/`ObjExporter`/etc., **not** DWG — orphaned in the old file since that module's real content moved to `semio-framework-mesh-engine` in an earlier wave but its tests never followed) | `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs`, new `//#region 🧪️Tests` (that crate had zero prior tests, so this is the file being extended, per the "no new test files" rule — it's the only file in the crate) |

**Test-count accounting: 9 + 20 = 29, all landed, all verified passing in their new homes** (see §6).

New cross-artifact bridges (ticket step 2), built from scratch against `SemioMeshSnapshot`/`SemioDrawingSnapshot` fields directly — **not** by reusing `mesh_to_dwg_drawing`/`paths_to_dwg_drawing` unchanged, because every sibling leaf already in this tree (`SemioMeshToObj`, `SemioMeshToGltf`, `SemioDrawingToDxf`, `SemioCadFromDwg`) does its own field-level mapping against the Snapshot types rather than routing through framework's `MeshData`; templated on `SemioCadFromDwg`/`SemioDrawingToDxf` exactly, per the ticket's "mirror it exactly, do not invent a mechanism" instruction:

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs` — `SemioMeshToDwg` (`SemioMeshSnapshot → DwgSnapshot`; one `PolyfaceMesh` entity per `SemioMesh`, one DWG layer per mesh id; Triangles-only, hard error otherwise)
- `.../✳️mesh/🚪️io/📥️import/🧩️deserializers/.../🦀️component.rs` — `SemioMeshFromDwg` (inverse; one `SemioMesh` per DWG layer carrying `PolyfaceMesh`/`Face3d` entities)
- `.../✳️drawing/🚪️io/📤️export/🧵️serializers/.../🦀️component.rs` — `SemioDrawingToDwg` (`SemioDrawingSnapshot → DwgSnapshot`; walks each `DrawLayer`'s node tree, `PathSegment`→`DwgPathSegment`, reuses the relocated `paths_to_dwg_drawing` per layer then re-tags the resulting entities' `layer` index; `Text` nodes become `DwgGeometry::Text` directly)
- `.../✳️drawing/🚪️io/📥️import/🧩️deserializers/.../🦀️component.rs` — `SemioDrawingFromDwg` (inverse; uses the relocated `dwg_geometry_to_path_segments` per entity, one `DrawLayer` per DWG layer)

Both directions produce `DwgSnapshot{version: "AC1015", bytes: dwg_to_bytes(...), ...}` — honestly labeled with the codec's own file magic, not `"AC1024"`, since that's what the bytes actually are.

Mounted in `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` mirroring the existing `dxf`/`svg`/`pdf` sibling leaf mounts exactly (`pub mod dwg { pub mod v_ac1024 { pub mod any { ... } } }` under both `import`/`export` for both `✳️mesh` and `✳️drawing`).

Cargo dependency added: `semio-framework-mesh-engine` to `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` (needed by the relocated `mesh_to_dwg_drawing`/`dwg_drawing_to_mesh`, which still operate on framework's `MeshData` — see §5 for why they couldn't be rewritten against `SemioMeshSnapshot` instead).

## 4. Consumer census — corrected from the ticket's premise

The ticket said consumers "reach these through `semio_framework::` re-exports." **That's only ⅓ true.** Grepping `semio_framework::Dwg*` alone misses two more re-export surfaces:

- `🧰️framework/🛍️products/💻️os/🦀️component.rs` (`use semio_framework::{DwgColor, DwgDrawing, DwgEntity, DwgGeometry};` then `pub use semio_framework::*;` at its crate root) — so **`semio_framework_os::Dwg*`** is a second valid spelling.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (`pub use semio_framework::*;`) — so **`semio_framework_plugin::Dwg*`** is a third.

All three are pure globs of the same underlying symbols — no independent copies to maintain, so repointing plugin-layer files never needed touching `os`/`host`/`plugin`'s own re-export code. But it means the true census (found via `grep -rln "dwg_to_bytes\|dwg_from_bytes\|DwgDrawing\|DwgEntity\|DwgGeometry\|DwgLayer\|DwgColor\|DwgPathSegment\|dwg_drawing_to_\|_to_dwg_drawing"`) is **23 files**, not ~16, and — more importantly — a real subset of them are **pinned to the framework type by cross-crate function-pointer registration**, which the ticket did not flag at all.

### 4a. Files repointed to the new `semio_s_plugin_stdio::artifacts::dwg::{...}` path (verified safe: never passed as a bare `fn` value across a crate boundary)

| File | Symbols | Verified |
|---|---|---|
| `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` | `DwgDrawing`, `DwgGeometry` | `cargo test -p semio-s-plugin-raster --lib` → 66/66 (incl. `imports_dwg_polyline_into_raster_document`) |
| `.../🖨️raster/.../🧬️schema/🦀️component.rs` | `DwgDrawing`, `DwgEntity`, `DwgColor`, `DwgGeometry` (test-only) | same run |
| `.../🖨️raster/.../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs` | `dwg_from_bytes`, `DwgDrawing` | same run |
| `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` | `DwgDrawing`, `DwgGeometry` (+ test-only `DwgColor`/`DwgEntity`/`DwgLayer`) | `cargo test -p semio-s-plugin-note --lib` → both `*dwg*` tests pass |
| `.../🗒️note/.../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs` | `dwg_from_bytes`, `DwgDrawing` | same run |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` | `DwgDrawing`, `DwgEntity`, `DwgColor`, `DwgGeometry` | `cargo check` shows **zero errors attributable to this file** — see §4c for why a full test run isn't possible right now |
| `.../📏️layout/.../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs` | `dwg_from_bytes`, `DwgDrawing` | same |

### 4b. Files **deliberately left on the framework path** — pinned by a real cross-crate constraint

Found by grepping every `register_dwg_import_handler`/`register_mesh_dwg_import_handler`/`register_mesh_dwg_export_handler` call site repo-wide and checking whether the function being registered is one of the census hits. **This caught a real mistake before it shipped**: I initially repointed `📐️cad`'s `cad_document_from_dwg`/`cad_working_scene_from_dwg` and its app's test module, then found `✏️s/🔌️plugins/🎪️demonstrator/🎪️panes/📐️koordinator/🦀️component.rs:29` does `semio_framework_os::register_dwg_import_handler(CAD_KIND, cad_document_from_dwg);` — a bare function-pointer argument whose type is fixed by `register_dwg_import_handler(from_dwg: fn(&DwgDrawing) -> Result<Value, String>)` in `os/component.rs` (framework type, not mine to change). Repointing `cad_document_from_dwg`'s parameter type would have broken `🎪️demonstrator` at compile time. **Reverted both cad files back to `semio_framework::DwgDrawing` exactly as found** and re-verified (`cargo test -p semio-s-plugin-cad --lib dwg` → 2/2 pass).

Left alone for the same reason (registered, or transitively depending on a framework-pinned helper — `dwg_drawing_to_svg` lives in `os/component.rs` itself):

| File | Why pinned |
|---|---|
| `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🦀️component.rs`, `.../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` | `cad_document_from_dwg` registered in `🎪️demonstrator/🎪️panes/📐️koordinator` |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` | `gis2d_document_json_from_dwg` registered in `🎪️demonstrator/🎪️panes/🗺️verfolgen` |
| `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🦀️component.rs` | `puzzle2d_document_json_from_dwg` registered in that same file (`register_dwg_import_handler("2d.puzzle", ...)`) |
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` | not registered itself, but calls `semio_framework_os::dwg_drawing_to_svg(drawing)`, whose signature is fixed in `os/component.rs` |
| `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` | not currently registered anywhere, but its own doc comment explicitly frames its signature as matching `register_dwg_import_handler`'s contract for future wiring — left alone out of caution rather than gambling on "not registered yet" |
| `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs` | its one DWG usage is self-contained inside a `register_os_media_export_handler_kind` test closure (type-safe either way) — left untouched as out-of-scope/no-value, not because it's unsafe |
| stdio's own `✳️cad`/`✳️drawing` snapshot doc comments (`.../✳️cad/🧬️schema/📸️snapshot/🦀️component.rs`, `.../✳️drawing/🧬️schema/📸️snapshot/🦀️component.rs`) | prose-only mentions of `DwgDrawing`/`DwgEntity`/`DwgGeometry`, no code — left as-is |

### 4c. `semio-s-plugin-layout` — pre-existing, unrelated compile blocker (not mine)

`cargo check -p semio-s-plugin-layout --all-targets` fails with 3 errors, all in `.../📏️layout/.../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs` and its import sibling: `PageDoc`/`page` vs `pages` field-name mismatch against `semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot`. **Confirmed pre-existing and unrelated to DWG**: `stat -f '%Sm' .../🔖️1.4/✳️any/🦀️component.rs` → `Aug 12 10:50:40 2026`, well before this session; `git log` on that path's last commit is ticket `481` (2026-08-10, PNG import/export work), unrelated to this ticket. Per the "concurrent cargo workspace churn" rule, not fixed here. `cargo check`'s error list contains **zero** errors on the two DWG files I edited in that crate — my edits are consistent with the rest of the (broken-elsewhere) crate, but I cannot get a passing `cargo test` for `semio-s-plugin-layout` until that PDF bug is fixed by whoever owns it.

## 5. Why `mesh_to_dwg_drawing`/`dwg_drawing_to_mesh` (the `MeshData`-based bridges) stayed `MeshData`-shaped, not `SemioMeshSnapshot`-shaped

These two specifically operate on `semio_framework_mesh_engine::MeshData` (flat f32 arrays), not any stdio artifact type. Before deciding to keep them exactly as-is (verbatim relocation) I grepped their real call sites and found **all of them are framework-layer, none are stdio-plugin-layer**:

- `🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs` (W3a-owned)
- `🧰️framework/🛍️products/💻️os/🦀️component.rs` and `.../🖥️host/🦀️component.rs` (`register_mesh_dwg_import_handler`/`register_mesh_dwg_export_handler`'s own bodies)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs` (G1a-owned)

A framework crate **cannot** depend on a stdio-plugin crate (dependency direction is plugins → framework, never the reverse — confirmed via `semio-framework-os-kernel`'s own `Cargo.toml`, which has no `semio-s-plugin-*` dependency at all). So relocating these two functions to only exist inside the stdio plugin, as the ticket's step 2 literally describes, would have orphaned every one of their real callers. Kept them landing in `ac1024/🚪️io` too (still zero `DwgSnapshot` dependency, same "rule 6" justification), reachable at `semio_s_plugin_stdio::artifacts::dwg::{mesh_to_dwg_drawing, dwg_drawing_to_mesh}` for any *plugin*-layer caller that wants them (none currently do directly — `📐️cad`'s own use is pinned to the framework copy per §4b) — but this does **not** by itself unblock deleting the framework copy, since the framework-layer callers above still need a framework-reachable copy. See §7.

## 6. Verification — every command run, with real output

```
$ TD=".../DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS/🎯️target"
$ touch <edited file>; RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-s-plugin-stdio --all-targets
    Finished `dev` profile [unoptimized] target(s) in 15.02s      # zero errors

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-stdio --lib
test result: FAILED. 2430 passed; 5 failed; 4 ignored; 0 measured; 0 filtered out; finished in 14.57s
failures:
    artifacts::binary::standards::v_raw::subsets::any::schema::inferences::extent::component::tests::inference_default_law
    artifacts::dwg::standards::v_ac1018::subsets::any::schema::component::tests::conformance_laws::fixture_honesty_law
    artifacts::dxf::standards::v_r12::subsets::any::schema::inferences::bounds::component::tests::bounds_matches_hand_built_entity_extent
    artifacts::ifc::standards::v2x3::subsets::any::io::component::tests::conformance_laws::fixture_honesty_law
    artifacts::zip::standards::v2_0::subsets::any::schema::inferences::entries::component::tests::inference_default_law
```
Identical 5-failure set to the ticket's fresh 2026-08-13 14:08 baseline (`scratch-w0-baseline-failures-sorted.txt`, 2414 passed). **2430 − 2414 = 16 = 9 relocated DWG-codec tests + 7 new bridge tests.** No baseline failure changed character; none of my code is in that failure set.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-framework-mesh-engine --lib
test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
(the 20 relocated non-DWG tests, all pass in their new — first-ever — test module for this crate)

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-framework --lib
test result: ok. 127 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
Matches the ticket's stated baseline (127) exactly — unaffected, as expected (old module untouched).

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-framework-3d --lib
test result: ok. 413 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.68s
```
Matches the ticket's stated baseline (413/0) exactly — `📐️brep/📦️mesh-io` still compiles against the untouched framework module.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-raster --lib
test result: ok. 66 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.76s
```

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-note --lib
test result: FAILED. 81 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
failures:
    apps::note::panels::document::tests::renders_document_tree
    artifacts::note::standards::v1::subsets::any::schema::mutations::component::tests::block_lifecycle_inverse_law_create_delete_duplicate
```
Both DWG-specific tests (`imports_dwg_polyline_and_text_into_note_blocks`, `imports_empty_dwg_drawing_as_valid_empty_note_snapshot`) pass. The 2 failures are a Table/Math block-ordering inverse-law bug, unrelated to DWG — confirmed pre-existing: `.../🧬️mutations/🦀️component.rs` mtime `Aug 13 00:16:20 2026`/last commit ticket `499`, both well before this session.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-s-plugin-layout --all-targets
error[E0432]/E0560/E0609 × 3, all in .../📄️pdf/🔖️1.4/✳️any/🦀️component.rs (pre-existing, see §4c)
```

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-cad --lib dwg
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 138 filtered out; finished in 0.00s
```
(confirms the cad revert in §4b is clean)

## 7. Step 4 — deletion of `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` + its mount + re-export block: **`blocked-on-more-than-G1a`**

The ticket's gate named exactly 2 files (`flow/🌉️wasm`'s `dwg_encode_mesh_json`/`dwg_decode_mesh_json`, `flow/🖍️drawing`'s `export_dwg_sync`/`import_dwg_sync`) plus a soft warning about `📐️brep/📦️mesh-io` (W3a). Re-grepped the whole repo for `dwg_to_bytes|dwg_from_bytes|DwgDrawing|DwgEntity|DwgGeometry|DwgLayer|DwgColor|DwgPathSegment|dwg_drawing_to_|_to_dwg_drawing`, excluding the old module and `target`/`🦑️repo`, **as of this session**:

| File | Owner | Still references old symbols? |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs` | G1a | **yes** — `dwg_encode_mesh_json` still calls `semio_framework::mesh_to_dwg_drawing`/`dwg_to_bytes` (lines 671-679) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs` | G1a | **yes** — `export_dwg_sync`/`import_dwg_sync` and ~15 more call sites, plus 5 tests, still fully on `semio_framework::Dwg*` |
| `🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs` | W3a | **yes** — `export_dwg`/`import_dwg` call `semio_framework::mesh_to_dwg_drawing`/`dwg_drawing_to_mesh` directly |
| `🧰️framework/🛍️products/💻️os/🦀️component.rs` | **not named in this ticket at all — new finding** | **yes** — `svg_to_dwg_bytes`, `dwg_drawing_to_svg`, `register_dwg_import_handler`, `register_mesh_dwg_import_handler`, `register_mesh_dwg_export_handler` are real OS-media-export functionality, not test scaffolding, built directly on `semio_framework::{DwgColor, DwgDrawing, DwgEntity, DwgGeometry}` |
| `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` | **not named in this ticket at all — new finding** | **yes** — same functions, apparently a parallel "host" build target of the same OS surface |

Both `os`/`host` are framework-*product* crates (`semio-framework-os-kernel`/`semio-framework-os`), not plugins — they architecturally **cannot** depend on `semio-s-plugin-stdio` (plugins depend on framework, never the reverse), so their DWG usage cannot be repointed to the new stdio home the way the plugin-layer consumers in §4a were. This is not merely "wait for G1a to land" — even after G1a and W3a both finish, `os`/`host`'s own `svg_to_dwg_bytes`/`dwg_drawing_to_svg`/`register_*_dwg_*_handler` still need *some* framework-reachable `DwgDrawing`/`dwg_to_bytes`/`dwg_from_bytes`. Resolving that is a real design decision (give `os`/`host` their own copy? keep a framework-layer re-export shim pointing at the relocated code, which needs an inverted dependency the build graph doesn't support today? intentionally drop DWG from OS-level media export?) that is outside this wave's authority to make unilaterally, and touches files two other in-flight waves (G1a, W3a) are actively editing.

**Per the ticket's own explicit allowance** ("If they still do, do not delete the module — complete steps 1–3, report step 4 as `blocked-on-G1a` with everything staged and ready, and stop. A correct partial beats a broken tree."): steps 1–3 are complete and verified; step 4 is not attempted. `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs`, its `#[path]` mount (`pub mod mesh;`, framework `📦️glue.rs:22`), and its re-export block (`pub use mesh::{...};`, `📦️glue.rs:59-62`) are **all left completely untouched** — a deliberate, bounded duplication window (the codec now genuinely exists twice, verbatim, in both the old framework location and the new stdio location) that the ticket explicitly sanctions until the wider blocker set above clears.

## 8. `sharedFileRequests` — patches needed from outside this wave's boundary before step 4 can complete

1. **G1a** must finish repointing/removing `flow/🌉️wasm`'s `dwg_encode_mesh_json`/`dwg_decode_mesh_json` and `flow/🖍️drawing`'s `export_dwg_sync`/`import_dwg_sync` (and its ~5 tests) off `semio_framework::Dwg*`.
2. **W3a** must finish repointing/removing `📐️brep/📦️mesh-io`'s `export_dwg`/`import_dwg` off `semio_framework::{mesh_to_dwg_drawing, dwg_drawing_to_mesh}`.
3. **New, previously-unflagged**: whoever owns `🧰️framework/🛍️products/💻️os/🦀️component.rs` + `.../🖥️host/🦀️component.rs` needs to decide what happens to `svg_to_dwg_bytes`/`dwg_drawing_to_svg`/`register_dwg_import_handler`/`register_mesh_dwg_import_handler`/`register_mesh_dwg_export_handler`'s `DwgDrawing`/`MeshData` usage — these are real, load-bearing OS-media-export features (registered by `🎪️demonstrator`, `🏭️process`, `🌀️procedural`, `🧩️puzzle`, `🪐️space` — a wider fan-out than this ticket's own known-consumers list). Recommend a follow-up ticket scoped explicitly to that, since it needs an architectural call (framework-layer DWG stays, forever, distinct from the stdio plugin's copy — probably the honest long-term answer, since `os`/`host` cannot depend on plugins) rather than a mechanical repoint.
4. Independent of DWG: `.../📏️layout/.../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs` (+ its import sibling) has a pre-existing `PageDoc`/`page`-vs-`pages` field mismatch against `semio_s_plugin_stdio::artifacts::pdf::PdfSnapshot`, blocking `semio-s-plugin-layout` from compiling at all (`--all-targets`). Not DWG-related, not touched here; flagged so this wave's inability to get a green `cargo test -p semio-s-plugin-layout` isn't mistaken for my own regression.

## 9. Files touched (created, edited) this wave

Created:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️component.rs`

Edited:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🚪️io/🦀️component.rs` (+codec, +9 tests)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🦀️component.rs` (+re-export block)
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` (+`semio-framework-mesh-engine` dependency)
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` (+4 new leaf mounts under `✳️mesh`/`✳️drawing` `🚪️io`)
- `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs` (+20 relocated tests)
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🦀️component.rs`, `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — **edited then reverted back to their original framework-path content** (see §4b); net diff is zero, kept only because the mistake-and-catch is documented here for the next wave's benefit.

Not edited (left on the framework path, deliberately, see §4b): gismap schema, puzzle2d app, animate/present io, space media command, shooting schema, stdio's `✳️cad`/`✳️drawing` snapshot doc comments.

Not deleted (blocked, see §7): `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs`, its `pub mod mesh;` mount and `pub use mesh::{...};` re-export block in `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`.

Scratch/cleanup note: a shell typo momentarily created `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️标准` (wrong CJK characters instead of `🏅️standards`) via `mkdir -p`; removed with `rm -rf` before any file was written into it. Verified via repo-wide `find -iname "*标准*"` that no such stray directory remains anywhere.
