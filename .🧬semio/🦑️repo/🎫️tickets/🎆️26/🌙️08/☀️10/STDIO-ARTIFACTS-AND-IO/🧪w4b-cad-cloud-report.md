# W4b Report — Cad and Cloud Stdio Artifacts

## Scope

End-to-end stdio artifacts (facet trees copied from `📄txt` / `🔣️json` / `💾️binary` references):

| Roster id | Dir | Neutral model | IO parent |
|-----------|-----|---------------|-----------|
| step | `📐️step` | `BrepMesh` (vertices + faces) | txt |
| ifc | `🏗️ifc` | `BrepMesh` | txt |
| las | `☁️las` | `MeshVertex` point cloud | binary |
| gltf | `🧊️gltf` | `MeshVertex` + glTF JSON `document` | json |

**Related but not W4b cad/cloud:** `🧊️glb` exists on disk (office/mesh wave) but is **not** finished here — missing snapshot types (`GlbEntry`), depends on deflate/gltf wiring elsewhere.

**Out of scope:** `png`, `jpg`, `gif`, `tiff`, `pdf` (parallel W4 raster/page waves).

## Facet tree (each format)

For `step`, `ifc`, `las`, `gltf`:

- Root `🦀️component.rs` + `🟦️component.ts`
- `🧬️schema/` (artifact, snapshot, diff, mutations + set-snapshot)
- `⚙️engine/`, `🏗️builder/`, `🪓️decomposer/`
- `🚪️io/` import/export deserializers + serializers to parent IO kind
- `📚️examples/🎬️demo/`

## Codecs

- **step**: ISO-10303-21 `#id=ENTITY(...)` map; `CARTESIAN_POINT` + `POLY_LOOP`; `step_brep_from_text` / `step_brep_to_text`; DSL/pack via `wrap_text` / `wrap_binary`.
- **ifc**: IFC-SPF entity map; `IFCCARTESIANPOINT`, `IFCPOLYLOOP`, `IFCFACE` / `IFCFACEOUTERBOUND` (no cartoon types); `ifc_brep_from_text` / `ifc_brep_to_text`.
- **las**: LAS 1.2 `LASF` header, point format 0, scale/offset XYZ; `las_vertices_from_bytes` / `las_bytes_from_vertices`; pack is raw `.las` bytes in semio envelope.
- **gltf**: glTF 2.0 JSON; first mesh `POSITION` (VEC3, FLOAT); embedded base64 buffer URI; hand-rolled base64; `gltf_vertices_from_value` / `gltf_value_from_vertices`.

## Wiring

- **`📦️glue.rs`**: `#[path = "."] pub mod {step,ifc,las,gltf}` inside `//#region Artifacts` (same region as `zip` … `glb`; single closing `}` before `//#endregion Artifacts`). LAS IO submodule `artifacts::binary`; glTF IO submodule `artifacts::json`.
- **`🔌️plugin/🦀️component.rs`**: `engine::register()` + `artifact_kind()` for step, ifc, las, gltf (among other roster entries from parallel work).
- **`📦️packages/🟦️typescript/📦️index.ts`**: `export * as step|ifc|las|gltf`.

## API alignment (no invented variants)

| Concern | Pattern used |
|---------|----------------|
| Builder | `from_text`, `from_binary` → `DocumentDsl` / `DocumentPack` |
| Decomposer | `DecomposeSource::Text` \| `DecomposeSource::Binary` only |
| Mutations | `OpText` + `OpBinary` (not `OpLas`, `OpZip`, …) |
| Pack | `store::semio_format::wrap_binary` / `unwrap_binary` |
| Text DSL | `wrap_text` / split preamble |

## Generators (ticket folder)

- `generators/w4b_cad_scaffold.py`
- `generators/w4b_cad_fix_codecs.py`
- `generators/w4b_cad_glue.py`

## Examples

- `📐️step/.../example.step` — triangle (`CARTESIAN_POINT`, `POLY_LOOP`)
- `🏗️ifc/.../example.ifc` — triangle (`IFCCARTESIANPOINT`, `IFCPOLYLOOP`)
- `☁️las/.../example.las` — three LAS 1.2 points
- `🧊️gltf/.../example.gltf` — triangle, POSITION accessor, embedded buffer

## Verification

```text
cargo check -p semio-s-plugin-stdio
```

### W4b cad/cloud (step, ifc, las, gltf)

**No rustc diagnostics** on paths under `🗿️artifacts/📐️step`, `🏗️ifc`, `☁️las`, `🧊️gltf`.

### Full crate (2026-08-10) — remaining errors (17)

Not introduced by W4b cad/cloud; block parallel W4 facets:

| Code | Count | Area |
|------|-------|------|
| E0432 | 7 | Missing snapshot types: `PngEntry`, `JpgEntry`, `GifEntry`, `TiffEntry`, `GlbEntry`, `PdfEntry`, `PageDoc` |
| E0061 | 4 | Office zip export: `encode_* (snapshot, true)` — callee takes 1 arg (`docx`, `xlsx`, `pptx`, `bcf` → zip serializer) |
| E0560 / E0609 | 5 | `pdf`: artifact schema expects `page` field; snapshot uses different shape |
| E0599 | 1 | `pdf` engine: `String::extend_from_slice` (should be `Vec<u8>`) |

**Action:** Raster/page/office agents finish snapshot codecs and align `encode_*` signatures; no change required in step/ifc/las/gltf for these failures.

## Notes

- glTF codec requires embedded base64 buffer URI (external `.bin` / `.glb` not in this wave).
- Scaffold replace passes must not strip single-letter Rust field names (`x`, `y`, …) in mesh/CAD parsers.
