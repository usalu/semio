# 🎟️ T16 Capability Audit (haiku-capabilities, static)

A = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`, K = `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract`, O = `🧰️framework/🛍️products/💻️os/🔨️modules`. The edit window is `✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs`. "RO" = `window_kind()` with no actions.

## Edit Window Status (36 editors)

**Editable (17):** html, md, txt (text splices); binary (hex splices); deflate (header text only, payload documented non-editable); json, xml, zip (tree); csv, tsv (structural table); epw, bcf (row-revision table); wav (table plus retained chunked work); png (pixel paint commands); docx, pptx (document); semio mesh and brep subsets only.

**RO edit windows despite being editors (17):** las, gif, mp4, svg, mp3, avi, jpg, bmp, tiff, stl, ply, dxf, dwg, obj, gltf, step, ifc 4.

**Unverified (2):** xlsx, pdf (non-1.7-base subsets); also the ifc 2x3 subsets.

## Declared But Unwired

- step: the comment claims `set-vertex`, but there are no actions. The render is a placeholder box per entity (ap214 base main :13-62).
- dwg ac1024: the comment claims `set-vertex`, but it is RO.
- obj: the schema mutation `📍set-vertex` exists but cannot be reached from the RO window.
- gltf: 3 mutations are implemented and registered (`A/🧊️gltf/🦀️.rs:50-54`), but the definition says unimplemented and the window is RO.
- bmp: `paint-index-region` and `paint-direct-region` tools are declared in the editor root (`✏️editor/🦀️.rs:57-58,165-166`) but are missing from the window actions.
- tiff: a `paint-region` command file exists but is not wired.
- json: the definition has `mutations: []`, but i-json has upsert/remove/rename/array/set mutations. Similarly, zip's definition has `mutations: []` while ZipMutation exists.
- deflate: the docs name a `replace-text` action that does not exist.

## Open/Save and Shared Contract

- **NaturalFileCodec.** Declared on 19 editors: html, mp4, svg, mp3, csv, tsv, xlsx, pdf 1.7 base, docx, pptx, md, xml, png, jpg, avi, wav, txt, json, bmp. Mounting is in `O/🔌️plugin/📜️declaration/🧩️app/🗂️natural-file-and-media-artifacts/🦀️.rs:102-116`.
- **Undo/redo.** `mod+z` and `mod+shift+z` route to app history.
- **Copy/paste and shortcuts.** No copy/paste and no artifact keyboard shortcuts are declared anywhere. The Save/Open actions have no chord.
- **Ledger vs codec.** 32 artifacts have codec `implemented` but ledger read/write `unimplemented`; the validator never cross-checks them (`K/🏗️assembly/🦀️.rs:583-598`).
- **Codecs: [] drift.** Eight definitions have `codecs: []`. Six of them (wav, binary, bmp, gif, epw, ifc) have Rust codec registrations.
- **Import/export.** All `ArtifactKindSpec` import/export lists are empty. The OS derives import/export from `NaturalFileCodec`.

## Details Edit Targets for the RO-Main-Window Editors (from T16b; for T06 journey rows)

| Editor | Concrete edit through Details | Rule source |
| --- | --- | --- |
| las | set a point value at `/points/0` (set-point) | `edit-rules:8-10` |
| gif 89a | frame delay at `/frames/0/delay` | `9️⃣89a/…/edit-rules:16-30` |
| gif 87a | image interlace / indices | `7️⃣87a/…/edit-rules:16-25` |
| mp4 | sample sync flag at `/tracks/0/samples/0/sync` | `edit-rules:11-19` |
| avi | stream header field / chunk keyframe | `hdrl/…/edit-rules:9-24` |
| mp3 | `/id3v2` replace (TIT2 title) | `edit-rules:7` |
| svg base | attribute value `/doc/root/children/<n>/attrs/<row>/value` | base `mutation-support:158-222` |
| jpg document | JFIF header field (density) | `document/…/edit-rules:13-24` |
| tiff document | tag value `/ifds/0/entries/<n>/values` | `document/…/edit-rules:12-17` |
| stl | triangle vertex `/solid/triangles/0/vertices/0` | `edit-rules:8-13` |
| ply | insert a header comment | `edit-rules:8-17` |
| dxf | layer field `/tables/layers/<n>` | `edit-rules:8-30` |
| dwg | summary title | `ac1024/…/edit-rules:8-21` |
| obj | vertex `/vertices/0` (set-vertex) | `geometry/…/edit-rules:12-37` |
| gltf | node translation `/nodes/0/translation` | `edit-rules:116-226` |
| step base | entity name `/entities/<row>/name` | `base/…/edit-rules:16-23` |
| ifc 4 | entity name / argument | `4️⃣4/…/edit-rules:16-26` |

bmp, gif (per-frame paint) and tiff document now also have main-window paint actions (batch C). Exact pointers must be confirmed against each editor's Details tree on the first browser run.
