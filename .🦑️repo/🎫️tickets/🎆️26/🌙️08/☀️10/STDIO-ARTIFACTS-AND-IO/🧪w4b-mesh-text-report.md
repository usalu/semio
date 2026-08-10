# W4b Report — Mesh, Drawing, and Raster Stdio Artifacts

## Scope

Six roster artifacts (82-file facet tree cloned from `📄txt` / `💾️binary` / `📰xml` as appropriate):

| id | dir | Neutral | IO parents | Codec summary |
|----|-----|---------|------------|---------------|
| obj | `🧊️obj` | MeshData | txt | Wavefront `v` / `f` parse + write |
| ply | `☁️ply` | MeshData | txt | ASCII PLY header + vertices/faces |
| stl | `🟪️stl` | MeshData | txt + binary | ASCII facets + binary STL (80-byte header + triangles) |
| dxf | `🖊️dxf` | DwgDrawing | txt | ENTITIES `LINE` group codes (10–31) |
| svg | `🎨️svg` | DwgDrawing | xml | SVG root via `XmlDocument` + xml codec reuse |
| bmp | `🖼️bmp` | RasterImage | binary | 24-bit BMP encode/decode (`width`/`height`/`pixels` BGR) |

## Wiring

- `📦️glue.rs`: `artifacts::{obj,ply,stl,dxf,svg,bmp}` with `#[path = "."]` on each mod (same pattern as csv/zip).
- `🔌️plugin/🦀️component.rs`: `engine::register()` + `artifact_kind()` for all six.
- `📦️packages/🟦️typescript/📦️index.ts`: exports `obj`, `stl`, `ply`, `dxf`, `svg`, `bmp`.

## Generators (ticket)

- `generators/w4b_scaffold.py` — facet tree copy from txt/xml/binary
- `generators/w4b_fix_codecs.py` — snapshot codecs, schema fields, IO bridges
- `generators/w4b_glue.py` — glue/plugin/index append

## Adjacent fixes (shared crate)

- `📰xml` snapshot: `find().ok_or(...)`, `skip_ws` uses byte index (unblocks svg/xml IO).
- `📷️png` engine: removed stray Python `'''` fragment.
- `☁️las` glue IO: `pub mod binary` (was misnamed `txt`).
- `🧊️gltf` glue IO: `pub mod json` (paths already pointed at `🔣️json` leaves).

## Verification

```text
cargo check -p semio-s-plugin-stdio
→ FAILED (73 errors at time of report)
```

Full log: `🧪w4b-cargo-check.log`

W4b artifact sources compile in isolation; crate failure is dominated by parallel in-flight modules wired in the same `glue.rs` / plugin (`docx`, `xlsx`, `pptx`, `bcf`, `pdf`, `png`, `jpg`, `gif`, `tiff`, `glb`, partial `json`/`zip` engine stubs). Re-run check after those waves land or trim incomplete `pub mod` blocks from glue until their facets exist.

## Examples (on disk)

- `🧊️obj/.../example.obj` — unit cube wireframe
- `☁️ply/.../example.ply` — ASCII triangle
- `🟪️stl/.../example.stl` — ASCII solid
- `🖊️dxf/.../example.dxf` — single LINE entity
- `🎨️svg/.../example.svg` — minimal `<svg>` with rect
- `🖼️bmp/.../example.bmp` — 2×2 BMP (binary asset)
