# OS Host Dependency Pruning

## Scope

Inspected the twelve requested s-plugin Rust source trees with `rg --glob '*.rs' '\\bsemio_framework_os\\b'`. The word-boundary expression excludes `semio_framework_os_kernel`.

## Manifest Changes

- Removed `semio-framework-os` from writer, procedural, GIS, CAD, and puzzle.
- Removed the target-specific dependency table as well where it became empty (procedural, CAD, and puzzle).
- Puzzle's only two textual matches are documentation/attribute-reason text, not Rust paths or imports; its dependency was removed.

The following dependencies remain because the current live tree has real code-level `semio_framework_os` uses:

- shooting: `rasterize_svg_to_png_base64`
- animate: `title_card_svg`, `dwg_drawing_to_svg`, and `rasterize_svg_to_png_base64`
- layout: `svg_to_dwg_bytes`
- remodel: `OsMediaExportResult`
- space: workflow, space, host, and media APIs (130 textual matches)
- note: `svg_to_dwg_bytes`
- raster: `rasterize_svg_to_png_base64`

## WASI Verification

Each edited crate was checked from the repository root with `RUSTC_WRAPPER="" cargo check -p <crate> --lib --target wasm32-wasip2`, in the foreground. Cargo lock waits were allowed to complete.

| Crate | Result |
| --- | --- |
| `semio-s-plugin-writer` | Failed before linking: `semio-s-plugin-trinity` has 115 existing `ToValue`/`FromValue` and type errors. |
| `semio-s-plugin-procedural` | Failed before linking: `semio-framework-geometry` cannot find `circle_path_elements`. |
| `semio-s-plugin-gis` | Failed before linking: the same `semio-framework-geometry::circle_path_elements` error. |
| `semio-s-plugin-cad` | Failed before linking: existing CAD mutation-module imports and migration errors. |
| `semio-s-plugin-puzzle` | Failed before linking: existing `value` attribute and `ToValue`/`FromValue` migration errors. |

None of the five commands emitted the reported `rust-lld`/`ElemSection::writeBody()` crash. They did not reach a successful final link, so successful WASI linking remains blocked by the unrelated compile errors above.

`git diff --check` passed for the five manifest edits.
