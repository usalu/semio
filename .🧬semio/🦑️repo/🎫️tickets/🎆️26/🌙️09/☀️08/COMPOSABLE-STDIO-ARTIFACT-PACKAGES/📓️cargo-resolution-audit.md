# Resolved Cargo Artifact Closure

Refreshed the local Cargo lockfile with `cargo metadata --offline --format-version 1` after external dependency migration. Traversed actual resolved normal dependency edges for all36 stdio artifact crates. None reach the stdio plugin. This uses Cargo’s resolved graph rather than schema dependency labels or declarations alone; compile/runtime validation remains separate.

| Artifact Package | Resolved Stdio Package Closure |
| --- | --- |
| semio-s-artifact-stdio-avi | semio-s-artifact-stdio-avi, semio-s-artifact-stdio-contract |
| semio-s-artifact-stdio-bcf | semio-s-artifact-stdio-bcf, semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-deflate, semio-s-artifact-stdio-txt, semio-s-artifact-stdio-xml, semio-s-artifact-stdio-zip |
| semio-s-artifact-stdio-binary | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract |
| semio-s-artifact-stdio-bmp | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-bmp, semio-s-artifact-stdio-contract |
| semio-s-artifact-stdio-csv | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-csv, semio-s-artifact-stdio-txt |
| semio-s-artifact-stdio-deflate | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-deflate |
| semio-s-artifact-stdio-docx | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-deflate, semio-s-artifact-stdio-docx, semio-s-artifact-stdio-txt, semio-s-artifact-stdio-xml, semio-s-artifact-stdio-zip |
| semio-s-artifact-stdio-dwg | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-dwg |
| semio-s-artifact-stdio-dxf | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-dxf, semio-s-artifact-stdio-txt |
| semio-s-artifact-stdio-epw | semio-s-artifact-stdio-contract, semio-s-artifact-stdio-epw |
| semio-s-artifact-stdio-gif | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-gif |
| semio-s-artifact-stdio-gltf | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-gltf, semio-s-artifact-stdio-json, semio-s-artifact-stdio-txt |
| semio-s-artifact-stdio-html | semio-s-artifact-stdio-contract, semio-s-artifact-stdio-html |
| semio-s-artifact-stdio-ifc | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-ifc, semio-s-artifact-stdio-step, semio-s-artifact-stdio-txt |
| semio-s-artifact-stdio-jpg | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-jpg |
| semio-s-artifact-stdio-json | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-json, semio-s-artifact-stdio-txt |
| semio-s-artifact-stdio-las | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-las |
| semio-s-artifact-stdio-md | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-md, semio-s-artifact-stdio-txt |
| semio-s-artifact-stdio-mp3 | semio-s-artifact-stdio-contract, semio-s-artifact-stdio-mp3 |
| semio-s-artifact-stdio-mp4 | semio-s-artifact-stdio-contract, semio-s-artifact-stdio-mp4 |
| semio-s-artifact-stdio-obj | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-obj, semio-s-artifact-stdio-txt |
| semio-s-artifact-stdio-pdf | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-deflate, semio-s-artifact-stdio-pdf |
| semio-s-artifact-stdio-ply | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-ply, semio-s-artifact-stdio-txt |
| semio-s-artifact-stdio-png | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-deflate, semio-s-artifact-stdio-png, semio-s-artifact-stdio-txt, semio-s-artifact-stdio-xml, semio-s-artifact-stdio-zip |
| semio-s-artifact-stdio-pptx | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-deflate, semio-s-artifact-stdio-pptx, semio-s-artifact-stdio-txt, semio-s-artifact-stdio-xml, semio-s-artifact-stdio-zip |
| semio-s-artifact-stdio-semio | semio-s-artifact-stdio-avi, semio-s-artifact-stdio-bcf, semio-s-artifact-stdio-binary, semio-s-artifact-stdio-bmp, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-csv, semio-s-artifact-stdio-deflate, semio-s-artifact-stdio-docx, semio-s-artifact-stdio-dwg, semio-s-artifact-stdio-dxf, semio-s-artifact-stdio-gif, semio-s-artifact-stdio-gltf, semio-s-artifact-stdio-ifc, semio-s-artifact-stdio-jpg, semio-s-artifact-stdio-json, semio-s-artifact-stdio-las, semio-s-artifact-stdio-md, semio-s-artifact-stdio-mp3, semio-s-artifact-stdio-mp4, semio-s-artifact-stdio-obj, semio-s-artifact-stdio-pdf, semio-s-artifact-stdio-ply, semio-s-artifact-stdio-png, semio-s-artifact-stdio-pptx, semio-s-artifact-stdio-semio, semio-s-artifact-stdio-step, semio-s-artifact-stdio-stl, semio-s-artifact-stdio-svg, semio-s-artifact-stdio-tiff, semio-s-artifact-stdio-txt, semio-s-artifact-stdio-wav, semio-s-artifact-stdio-xml, semio-s-artifact-stdio-zip |
| semio-s-artifact-stdio-step | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-step, semio-s-artifact-stdio-txt |
| semio-s-artifact-stdio-stl | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-stl, semio-s-artifact-stdio-txt |
| semio-s-artifact-stdio-svg | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-svg, semio-s-artifact-stdio-txt, semio-s-artifact-stdio-xml |
| semio-s-artifact-stdio-tiff | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-tiff |
| semio-s-artifact-stdio-tsv | semio-s-artifact-stdio-contract, semio-s-artifact-stdio-tsv |
| semio-s-artifact-stdio-txt | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-txt |
| semio-s-artifact-stdio-wav | semio-s-artifact-stdio-contract, semio-s-artifact-stdio-wav |
| semio-s-artifact-stdio-xlsx | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-deflate, semio-s-artifact-stdio-txt, semio-s-artifact-stdio-xlsx, semio-s-artifact-stdio-xml, semio-s-artifact-stdio-zip |
| semio-s-artifact-stdio-xml | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-txt, semio-s-artifact-stdio-xml |
| semio-s-artifact-stdio-zip | semio-s-artifact-stdio-binary, semio-s-artifact-stdio-contract, semio-s-artifact-stdio-deflate, semio-s-artifact-stdio-txt, semio-s-artifact-stdio-xml, semio-s-artifact-stdio-zip |

## Selected Normal Dependency Closures

Ran `cargo tree -p <package> --no-default-features --edges normal --prefix none` for each row, both exit0. Stdio without selection has no artifact implementation package in its normal dependency closure. PDF selects only PDF, binary and deflate plus the shared contract.

- Stdio composition with no default features: `semio-s-artifact-stdio-contract`, `semio-s-plugin-stdio`.
- PDF artifact with no default features: `semio-s-artifact-stdio-binary`, `semio-s-artifact-stdio-contract`, `semio-s-artifact-stdio-deflate`, `semio-s-artifact-stdio-pdf`.
