# Media and Spatial Initial Edit Fixtures

These `setSnapshotValue` cases target fields present in each editor's `initial_snapshot`. Values use the native `DslValue` kind implied by the Rust field. They are deliberately different from the defaults.

| Family | RFC 6901 path | New value | Coverage |
| --- | --- | --- | --- |
| PNG | `/gama` | unsigned `50000` | verified by the PNG history test |
| JPG | `/reEncodeQuality` | unsigned `80` | nullable scalar |
| GIF 89a | `/loopCount` | unsigned `1` | nullable scalar |
| GIF 87a | `/pixelAspectRatio` | unsigned `1` | scalar |
| BMP | `/xPixelsPerMeter` | signed `2835` | scalar |
| TIFF | `/byteOrder` | string `bigEndian` | enum value |
| SVG | `/doc/root/name` | string `svg:svg` | tagged XML element name |
| WAV | `/fmt/sampleRate` | unsigned `48000` | verified reducer path; default is changed |
| MP3 | `/id3v1` | object `{raw:[84,65,71]}` | nullable typed metadata |
| MP4 | `/ftyp/minorVersion` | unsigned `1` | scalar |
| AVI | `/idx1Present` | boolean `true` | scalar |
| OBJ | `/mtllib` | string `materials.mtl` | nullable scalar |
| STL | `/solidName` | string `edited` | scalar |
| PLY | `/comments` | array `["edited"]` | collection replacement |
| glTF | `/document/asset/generator` | string `semio-editor` | nullable scalar |
| IFC 4 | `/header/fileName/0` | string value variant `edited.ifc` | pending exact `IfcValue` tag wrapping |
| IFC 2x3 | `/edmPreamble` | typed object | pending exact preamble fixture |
| STEP | `/header/fileName/name` | string `edited.step` | pending exact `StepFileName` field confirmation |
| DWG | `/maintenanceVersion` | unsigned `1` | scalar |
| DXF | `/headerVars` | one typed header variable | pending exact tagged value fixture |
| LAS | `/header/systemIdentifier` | string `semio-editor` | scalar |

The repository editor-catalog acceptance now owns the executable exact cases for all roots, including all Semio subset overrides. This table remains the family-level rationale for the first meaningful edit in each schema.
