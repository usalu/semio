# Media and Spatial `PatchSnapshot` Rollout

## Persisted contract

The remaining large-artifact gap should be closed by one opt-in path codec in the framework value layer and one native `PatchSnapshot` mutation leaf in every artifact aggregate. The value derive owns the correspondence between RFC 6901 names and Rust fields, including renamed fields, tagged enums, nullable fields, object keys, array indices, and byte arrays. `DslRecord` cannot provide that mapping because it only knows DSL spelling and would reproduce the current casing and tag loss.

The framework payload must contain one canonical path operation and its typed value when the operation needs one. Applying it traverses the typed snapshot directly and allocates only the containers along the edited path. It must never serialize the whole snapshot to `DslValue`. The same codec must derive an exact inverse against the base snapshot before publication:

- set records the previous value and inverses to set or remove as appropriate;
- insert inverses to remove at the final canonical index;
- remove records the removed value and inverses to insert;
- move records the canonical post-removal indices and returns an exact reverse move;
- rename records the old key and refuses collisions;
- source replacement remains a bounded whole-document operation; large documents expose all fields through path edits and do not advertise an action that transport cannot carry.

Every aggregate retains its domain mutation enum. `PatchSnapshot` is a normal schema-first mutation variant with the framework patch payload, a `MutationLeaf` descriptor with exactly fourteen authority fields, aggregate JSON Schema membership, text and binary codecs, generated language declarations, a unique protocol tag, `Mutation::diff`, and `Mutation::inverse`. Existing semantic leaves remain preferred when they are smaller or carry stronger domain intent. `SetSnapshot` remains only for bounded whole-document replacement.

Admission encodes every forward mutation and every exact inverse against `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES`. An edit is admitted only when both directions fit. Large byte, sample, vertex, point, and frame arrays are edited by leaf or bounded range patches, so sibling payloads never enter the event. A multi-item retained publication must prepare all items before commit and cancellation must leave generation and root identity unchanged.

## Concrete editor roots

All 54 assigned editor roots already parse the same six retained actions and therefore only need their current generic `SetSnapshot` fallback replaced by the derived path codec plus their aggregate's native `PatchSnapshot` wrapper. The exact concrete dialect remains the controller and proof authority.

| Schema owner / concrete roots | Count | Existing compact route to keep | `PatchSnapshot` responsibility |
| --- | ---: | --- | --- |
| PNG 1.2 any | 1 | `PatchPixels` and semantic metadata leaves | All non-pixel paths and metadata combinations not represented by one semantic leaf |
| JPEG JFIF document; baseline | 2 | Header, quality, restart interval, tables, pixel replacement; baseline SOF, precision, arithmetic, sampling | Coefficients, comments, remaining component/header details, and nested collection edits |
| GIF 87a any; 89a base | 2 | Existing GIF semantic leaves | Extension blocks, palettes, frame/image payload leaves, and all remaining nested fields |
| BMP v3 any | 1 | Existing BMP semantic leaves | Header metadata and bounded pixel/opaque byte paths without retaining the bitmap |
| TIFF 6.0 document; baseline | 2 | Byte order, IFD/tag structure, pixels; baseline compression/photometric/bits/strip/tile leaves | Tag values and nested IFD metadata not covered by a semantic leaf |
| SVG 1.1 base; basic; tiny | 3 | Existing XML/SVG semantic leaves | Attributes, namespaces, text, child ordering, and derived-profile snapshot paths |
| WAV RIFF PCM any | 1 | `PatchData`, `SetFmt`, `SetOtherChunks` | Remaining nested format and ancillary chunk fields |
| MP3 MPEG-1 Layer III any | 1 | Existing frame/tag semantic leaves | ID3/frame metadata and bounded frame byte paths without retaining all audio frames |
| MP4 ISO BMFF any | 1 | `SetFtyp`, track dimensions/codec, sample sync, track/sample insert/remove | Remaining box, track, sample metadata, and bounded sample bytes |
| AVI 1.0 hdrl | 1 | Existing RIFF/AVI semantic leaves | Header, stream, index, chunk metadata, and bounded media bytes |
| OBJ 3.0 geometry | 1 | Existing vertex/normal/texcoord/face/object/group leaves | Comments, material references, smoothing data, and remaining collection fields |
| STL ASCII any | 1 | Existing triangle and solid-name leaves | Nested facet details and collection paths not covered semantically |
| PLY 1.0 any | 1 | Existing element/property semantic leaves | Comments, object info, property values, and large vertex/face list leaves |
| glTF 2.0 any | 1 | Existing document/buffer semantic leaves | JSON document metadata and bounded buffer/image byte paths |
| IFC 4 any; IFC 2x3 base, COBie, SAV, CV20 | 5 | Existing header/entity/profile semantic leaves | Entity attributes, headers, relationship collections, and every derived-profile field |
| STEP AP214 base and CC1 through CC6 | 7 | Existing header/entity semantic leaves | Entity parameters, headers, anchors/references, and every conformance-class field |
| DWG AC1018; AC1024 | 2 | Existing header/entity semantic leaves | Version-specific header variables, sections, objects, and bounded opaque payload paths |
| DXF R12 header | 1 | Existing header-variable semantic leaves | Tagged header values and remaining nested map/list paths |
| LAS 1.0 header | 1 | Existing header semantic leaves | Header scalar/string details and bounded variable-length/point data when exposed |
| Semio v1 base, flow, animation, video, model, table, CAD, document, object, presentation, audio, value, text, mesh, graph, drawing, image, brep, kit | 19 | Existing domain semantic leaves, especially mesh/brep direct geometry | Every uncovered typed field in each concrete subset without inheriting the base controller identity |

Total: 54 concrete roots.

## Integration sequence

1. Land the framework path codec against the neutral fixture with escaped object keys, Unicode, tagged enums, nullable fields, byte arrays, insert/remove/move/rename, and exact inverse tests.
2. Define a reusable schema fragment and codec helpers for the native leaf, while keeping each artifact aggregate's protocol tag and mutation enum explicit.
3. Convert PNG and WAV generic fallbacks first. Their 2 MiB retained-store tests already prove large sibling preservation, cancellation, publication, undo, and redo and will guard the shared codec.
4. Convert MP4, JPEG, and TIFF. Their current compact mappings provide representative nested metadata, structural collection, and oversized-inverse cases.
5. Convert GIF/BMP/MP3/AVI/glTF and the geometry/point families. Add one payload-past-1-MiB test per distinct storage shape: pixels, compressed frames, RIFF chunks, external buffers, vertices/faces, and point records.
6. Convert SVG, IFC, STEP, DWG, DXF, LAS, and all Semio subsets. Test exact derived dialect addressing and one typed edit/reopen/undo/redo case per concrete root in the 88-editor catalog.
7. Remove generic whole-snapshot admission from these roots once every action maps to a semantic leaf, a bounded `PatchSnapshot`, or an explicitly size-refused whole-source replacement.

## Acceptance evidence required

The rollout is complete only when each distinct aggregate proves text and binary mutation round trips, exact forward and inverse replay, retained publication, cancellation before commit, undo, redo, Pack reopen, and native format encode/decode where the format has a writer. Binary native tests must preserve opaque sibling payloads byte-for-byte unless the format specification requires canonicalization. Existing third-party format oracles remain the independent evidence for native exports; Pack reopen is separate evidence and cannot substitute for it.
