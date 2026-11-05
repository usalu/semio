# ✏ Acceptance Edit Pin Audit

Read-only current fixture/schema/rule audit; no heavy tests. Runtime owns selectors; editor agent owns row corrections. This snapshot documents defects before their concurrent edits.

All generic Details scalar selectors currently use data-ui-node-key=edit plus aria-label field prefix. Actual controls use Value/Wert, keyed by FNV1a64(pointer) as sd-16hex and suffix -text/-number/-toggle. Objects/arrays have child controls only and no scalar input. Definitions: registry contract/✏️editing/🪟️details/{🏷️labels-and-controls,🌲️detail-tree-and-window}/🦀️.rs. The hash seed is cbf29ce484222325; xor UTF8 byte then multiply100000001b3 modulo64.

| Artifact | Finding |
| --- | --- |
| html | Explicit textEdit verb; text payload plausible; validate natural HTML source and rendered Apply route. |
| json | set-node exists; current main-tree selector name label must be checked against actual TreeWindowKit DOM; sample name renamed-widget. |
| md | Explicit textEdit payload; chosen demo file is empty, but whole-text edit can create content. |
| txt | Explicit textEdit payload; selected sample Hello stdio.txt. |
| xml | set-node exists; only Text nodes are editable; ensure text selector chooses a text leaf in selected catalog document. |
| csv | set-cell exists and selected sample has row0/cell0; current ID suffix requires DOM verification. |
| tsv | set-cell structural path plausible; selected sample contains rows; current ID suffix requires DOM verification. |
| gif | Selected sample is GIF87a but fixture edits 89a frame delay; cannot assume frames0/delay exists. |
| jpg | density is not a snapshot pointer; schema scalar is /image/jfifXDensity or /image/jfifYDensity. |
| tiff | /ifds/0/entries/0/values is an array with no scalar control; choose a safe tag values/0 and avoid structural image-dimension tags. |
| dwg | title is not a full pointer; scalar likely /summary/title; selected sample AC1018, editor owner must follow actual version. |
| dxf | /tables/layers/0 is a layer object with no scalar input; use existing scalar such as /tables/layers/0/name after ensuring layer0 restrictions. |
| svg | Attribute value pointer scalar possible, but selected smallest mouse SVG first root child is likely comment/defs rather than attrs-bearing intended shape; pin exact fixture and node. |
| gltf | Snapshot root is document, so /nodes/0/translation absent; translation itself array. Use existing /document/nodes/0/translation/0 or /document/nodes/0/name. |
| las | /points/0 is a point object, value1 wrong shape; choose numeric /points/0/x or /points/0/intensity. |
| obj | /vertices/0 is vertex object; value0 0 0 wrong shape and no-op sample first vertex. Use /vertices/0/x=0.25. |
| ply | comment isn't a pointer; actual root comments array; selected tetrahedron has /comments/0 string. |
| step | /entities/0/name is entity TYPE, not display name; Matrix may break semantic grammar. Prefer supported /header/fileName/name base edit; selected sample is cc6/AP242, owner/schema mismatch needs pin. |
| stl | /solid/triangles nonexistent; schema root /solidName and /triangles. Safest scalar /solidName=Matrix. |
| ifc | name isn't pointer and likely type name; owner version follows selected2x3 sample, so fixture must specify valid existing scalar argument. |
| avi | keyframe isn't pointer; actual scalar /streams/0/chunks/0/keyframe is toggle, not text input; must toggle opposite current sample value. |
| mp3 | /id3v2 is nullable object, TIT2=Matrix invalid structured value and no scalar input; use existing text frame content values/0. |
| mp4 | /tracks/0/samples/0/sync is boolean toggle; rendered fill false is wrong control interaction; need current-value-aware toggle. |
| idml | edit.mode none explicitly skips editing; now real editor exists this is a placeholder and guarantees journey failure. |
| docx | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| pptx | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| xlsx | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| bmp | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| png | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| pdf | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| bcf | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| epw | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| idf | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| wav | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| binary | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| deflate | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| semio | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |
| zip | mode scan has no explicit verb, target or expected edited value; generic scanner can choose unrelated mutation. Must replace with deterministic artifact-specific edit. |

Current resolveSample chooses smallest accepted manifest fixture, not row.sample. This can cross supported standard/subset ownership: GIF87a, DWGAC1018, IFC2x3, DOCX strict and STEPcc6 fixtures currently win. Deterministic journey edits require exact valid sample and owner matching its schema. ResolveSample and browser ownership selection must honor those pins. Binary magic-only acceptance is not enough to establish a sample can be edited by a different standard's editor.

Verified safe scalar examples were sent to editor owner: STL /solidName, OBJ /vertices/0/x=0.25, LAS /points/0/x, PLY /comments/0, JPG /image/jfifXDensity. MP3 needs actual frame presence; TIFF needs actual tag identity; GLTF needs existing optional node scalar.
