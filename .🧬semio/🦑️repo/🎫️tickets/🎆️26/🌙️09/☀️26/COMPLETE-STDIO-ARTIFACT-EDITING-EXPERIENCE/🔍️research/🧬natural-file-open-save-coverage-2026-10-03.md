# Natural File Open/Save Coverage — 2026-10-03

This inventory distinguishes a declared filename from an editor that mounts a real natural encoder and decoder. A schema declaration, document-pack codec, or filename alone does not make a natural file available through Open/Save.

## Mounted codec matrix

The current source has **30 mounted editor implementations** represented by **28 domain-neutral lifecycle rows** across **19 artifact families**. The editor/row difference is deliberate: DOCX base, strict, and transitional each mount a codec, while the neutral lifecycle fixture has one DOCX row because all three share one extension, media type, and native byte contract. XLSX, PPTX, XML, JSON, SVG, and JPEG keep separate rows for each mounted profile because their editor declarations are distinct fixture inputs.

| Artifact/profile | Extension | Media type | Mounted editors | Neutral rows | Native byte/oracle verification | Browser verification |
|---|---|---|---:|---:|---|---|
| CSV RFC 4180 | `.csv` | `text/csv` | 1 | 1 | **Green**: edited bytes, third-party CSV parse, fresh-owner reopen, one mutation (`natural-file-csv-native-4.log`, 1/1) | **Partial**: preview 35 exposed Open/Save; Save dispatched without a console fault. Download bytes and natural-file reopen remain unverified. |
| PNG 1.2 | `.png` | `image/png` | 1 | 1 | Editor law now covers committed 16-bit grayscale, duplicate indexed 2-bit, and Adam7 RGBA through direct editor import/export and whole-document publication; `png` 0.18 independently checks profile, depth, and interlace. Fresh current-source run is pending in the Office-owned native queue. | Not run. |
| BMP v3 | `.bmp` | `image/bmp` | 1 | 1 | Canonical-byte-preserving editor import/export, independent `image` decode, RGBA projection, and fresh `SetSnapshot` witness are authored; native run pending. | Not run. |
| WAV RIFF PCM | `.wav` | `audio/wav` | 1 | 1 | Exact codec, independent oracle, and fresh-owner publication witnesses are source-mounted in the Media lane; native run pending. | Not run. |
| JPEG JFIF document/baseline | `.jpg` | `image/jpeg` | 2 | 2 | Exact document and baseline codecs, independent oracle, and fresh-owner publication witnesses are source-mounted in the Media lane; native run pending. | Not run. |
| MP3 MPEG-1 Layer III | `.mp3` | `audio/mpeg` | 1 | 1 | Exact codec, independent oracle, and fresh-owner publication witnesses are source-mounted in the Media lane; native run pending. | Not run. |
| MP4 ISOBMFF | `.mp4` | `video/mp4` | 1 | 1 | Exact codec, independent oracle, and fresh-owner publication witnesses are source-mounted in the Media lane; native run pending. | Not run. |
| AVI 1.0 | `.avi` | `video/x-msvideo` | 1 | 1 | Exact codec, independent oracle, and fresh-owner publication witnesses are source-mounted in the Media lane; native run pending. | Not run. |
| PDF 1.7 base | `.pdf` | `application/pdf` | 1 | 1 | Canonical encode/decode and a whole-snapshot natural event are source-mounted; two independent `lopdf`/actual-route laws await the coordinated native lane. The other nine declared PDF profiles remain unavailable. | Not run. |
| DOCX ECMA-376 base/strict/transitional | `.docx` | `application/vnd.openxmlformats-officedocument.wordprocessingml.document` | 3 | 1 | Law authored for edited WordprocessingML, independent ZIP/XML inspection, and reopen. The current rerun (`natural-file-docx-native-2.log`) stopped in shared Cargo locking before a compiler/test diagnostic; isolated rerun pending. | Not run. |
| Markdown CommonMark | `.md` | `text/markdown` | 1 | 1 | Law authored for CommonMark bytes, independent parse, and one-mutation reopen; native run pending. | Not run. |
| HTML 5 | `.html` | `text/html` | 1 | 1 | Law authored for HTML bytes, independent parse, and one-mutation reopen; native run pending. | Not run. |
| SVG 1.1 base/basic/tiny | `.svg` | `image/svg+xml` | 3 | 3 | Base canonical XML serialization and Basic/Tiny hard subset admission are mounted. Quick-XML oracle reopen, fresh `SetSnapshot`, and import/export rejection of out-of-profile elements are authored; native run pending. | Not run. |
| TSV IANA | `.tsv` | `text/tab-separated-values` | 1 | 1 | Law authored for tabular bytes, independent parse, and one-mutation reopen; native run pending. | Not run. |
| TXT UTF-8 | `.txt` | `text/plain` | 1 | 1 | Schema-first `SetSnapshot` and a `bstr` byte/line oracle are authored; TypeScript compiler is green (`natural-file-txt-typescript-1.log`). Native law pending. | Not run. |
| XML 1.0 base/valid | `.xml` | `application/xml` | 2 | 2 | Law authored for XML bytes, independent parse, and one-mutation reopen; native run pending. | Not run. |
| JSON RFC 8259 base/I-JSON | `.json` | `application/json` | 2 | 2 | Law authored for JSON bytes, independent parse, and one-mutation reopen; native run pending. | Not run. |
| XLSX ECMA-376 base/strict/transitional | `.xlsx` | `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet` | 3 | 3 | Law authored for edited worksheet XML, independent ZIP/XML inspection, canonical OPC preservation, and one-mutation reopen; native run pending. | Not run. |
| PPTX ECMA-376 base/strict/transitional | `.pptx` | `application/vnd.openxmlformats-officedocument.presentationml.presentation` | 3 | 3 | Law authored for edited slide XML, independent ZIP/XML inspection, canonical OPC preservation, and one-mutation reopen. PPTX package TypeScript check is green (`natural-file-pptx-typescript-2.log`); native run pending. | Not run. |
| **Mounted total** |  |  | **30** | **28** | CSV current-source native acceptance is green; the other format-specific native laws await serial isolated execution. | CSV action exposure only; no natural-file byte/reopen acceptance yet. |

The shared schema/renderer law passed 1/1 with cache disabled before the six Media-lane rows were added (`natural-file-intrinsic-editor-route-contract-1.log`). The current fixture has 28 source-mounted capability rows. It also carries an exact registered-editor byte/count witness for the real `PluginApp::consume_media` route; its native run is active. The neutral route carries raw owned bytes as `MediaPayload::Intrinsic` with `DslValue::Bytes`; it does not introduce a JSON, base64, or Semio pack wrapper.

## Declared formats still unavailable through natural Open/Save

These declarations have no paired natural codec on any editor and therefore do not publish generic Open/Save actions.

| Artifact | Declared extension(s) | Media type(s) | Editor sources | Missing natural boundary |
|---|---|---|---:|---|
| `s.stdio.bcf` | `.bcf` | `application/vnd.bcf+xml` | 1 | encoder + decoder mount |
| `s.stdio.binary` | `.bin` | `application/octet-stream` | 1 | explicit raw-binary document semantics |
| `s.stdio.deflate` | `.zz` | `application/zlib` | 1 | encoder + decoder mount |
| `s.stdio.dwg` | `.dwg` | `image/vnd.dwg` | 2 | encoder + decoder mount |
| `s.stdio.dxf` | `.dxf` | `image/vnd.dxf` | 1 | encoder + decoder mount |
| `s.stdio.epw` | `.epw` | undeclared | 1 | exact media type + encoder + decoder mount |
| `s.stdio.gif` | `.gif` | `image/gif` | 2 | encoder + decoder mount |
| `s.stdio.gltf` | `.gltf`, `.glb` | `model/gltf+json`, `model/gltf-binary` | 1 | representation-specific paired codecs |
| `s.stdio.ifc` | `.ifc` | `application/x-ifc` | 5 | encoder + decoder mount |
| `s.stdio.las` | `.las` | `application/vnd.las` | 1 | encoder + decoder mount |
| `s.stdio.obj` | `.obj` | `model/obj` | 1 | encoder + decoder mount |
| `s.stdio.pdf` | `.pdf` | `application/pdf` | 10 | 1.7 base is mounted; the other nine declared profiles need paired codecs and profile admission |
| `s.stdio.ply` | `.ply` | `model/ply` | 1 | encoder + decoder mount |
| `s.stdio.semio` | `.semio` | `application/vnd.semio` | 19 | natural format authority separate from domain pack |
| `s.stdio.step` | `.step` | `model/step` | 7 | encoder + decoder mount |
| `s.stdio.stl` | `.stl` | `model/stl` | 1 | encoder + decoder mount |
| `s.stdio.tiff` | `.tiff` | `image/tiff` | 2 | encoder + decoder mount |
| `s.stdio.zip` | `.zip` | `application/zip` | 3 | explicit ZIP document semantics + paired codec |

## Browser and scalability acceptance still open

The only current browser evidence is the CSV editor in preview 35: Open/Save were visible and Save produced no console fault. The browser runner did not expose a downloaded file for byte inspection, and no natural file has yet been selected and reopened into a fresh owner. Archive reload evidence is separate and does not establish natural-file persistence.

Natural Open now selects a raw `File`, admits a 512 MiB contiguous owner, and reads it in abortable 256 KiB Blob slices with exact byte progress. Picker cancel settles cleanly, read errors are contained, retry is accepted, and the owner/import function captured before the picker await remains authoritative. The schema-first lifecycle fixture and renderer law are green in `natural-file-raw-picker-contract-3.log` (1/1, cache disabled); aggregate renderer typecheck is recorded in `natural-file-raw-picker-typecheck-2.log`.

The `MediaIn`/`MediaOut` bridge, native codecs, and Save download still transfer one contiguous owned byte buffer. Encoding and decoding cannot yet yield or cancel within a large file, and the browser Save path still assembles all bytes before Blob download. This remains the retained segmented-media frontier for all mounted formats.
