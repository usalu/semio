# R18 Stdio Residue Split

## Source

Compiler TSV `📝️r17-stdio-pdf-iteration-4-errors.txt`, captured after the PDF lane reached zero library diagnostics and before further non-PDF repairs. It contains 2,447 diagnostics, all outside `🗿️artifacts/📄️pdf`.

## Diagnostic families

| Count | Code | Normalized message |
| ---: | --- | --- |
| 972 | E0308 | mismatched types |
| 678 | E0277 | value is not a future |
| 244 | E0599 | method missing on an opaque future |
| 156 | E0271 | future resolves to the old render type |
| 100 | E0277 | trait bound not satisfied |
| 41 | E0277 | operator requires a non-future value |
| 30 | E0277 | missing formatting/other implementation |
| 27 | E0277 | unsized value |
| 24 | E0277 | error conversion mismatch |
| 24 | E0277 | value is not an iterator |
| 19 | E0271 | implementation returns the old render type |
| 14 | E0308 | match arms incompatible |
| 14 | E0369 | binary operation applied to a future |

The dominant residue is still de-async fallout, plus 175 clean-break renderer return-type migrations. It is not a collection of unrelated defects.

## File-disjoint artifact groups

| Diagnostics | Artifact/group |
| ---: | --- |
| 494 | `🧿️semio` |
| 372 | framework/macro-origin diagnostics |
| 98 | `📐️step` |
| 98 | `🏗️ifc` |
| 93 | `🎨️svg` |
| 89 | `📷️png` |
| 88 | `🖊️dxf` |
| 85 | `📷️jpg` |
| 78 | `🖼️tiff` |
| 76 | `🧊️gltf` |
| 75 | `🎒️zip` |
| 72 | `📜️docx` |
| 64 | `📕️xlsx` |
| 57 | `💬️bcf` |
| 55 | `📰xml` |
| 55 | `🎥️mp4` |
| 52 | `🎞️gif` |
| 51 | `🗜️deflate` |
| 50 | `🔣️json` |
| 44 | `📝️md` |

The remaining smaller artifacts account for 333 diagnostics.

## Recommended parallel wave

When slots are available, split exclusively by artifact subtree:

1. `🧿️semio` only;
2. STEP + IFC only;
3. raster/media group: PNG, JPG, TIFF, SVG, GIF, MP4, AVI, BMP, WAV, MP3;
4. container/text group: DXF, GLTF, ZIP, DOCX, XLSX, BCF, XML, DEFLATE, JSON, Markdown and the smaller leaves.

The coordinator retains framework-origin diagnostics and the mounted renderer/actor gates. Each lane must take a fresh compiler count for its owned paths, use compiler-exact edits, reject no-progress iterations, and avoid any name-keyed replacement. Render migrations use the public `semio_framework_plugin::{ComponentTree, TreeNode, built_to_component_tree}` seam directly; no `UiNode` adapter is permitted.

## Counting method

The TSV was grouped by error code and by the first segment following `/🗿️artifacts/`. Paths without that segment were classified as framework/macro-origin. No source file was changed by this audit.
