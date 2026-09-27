# Media and Spatial `PatchSnapshot` Rollout

## Persisted contract

The remaining large-artifact gap is closed by one opt-in path codec in the framework value layer and one native `PatchSnapshot` mutation leaf in every artifact aggregate. The value derive owns the correspondence between RFC 6901 names and Rust fields, including renamed fields, tagged enums, nullable fields, object keys, array indices, and byte arrays. `DslRecord` cannot provide that mapping because it only knows DSL spelling and would reproduce the current casing and tag loss.

The landed framework carrier is `SnapshotPatch { edits: Vec<SnapshotValuePatch> }` with at most two sequential operations and at most 128 decoded path segments per operation. Each operation is typed as set, insert, or remove with a `DslValue` only where needed. Move and rename commands compile to remove-plus-insert. Applying it traverses the typed snapshot directly and allocates only the containers along the edited path. It never serializes the whole snapshot to `DslValue`. The same codec derives an exact inverse against the base snapshot before publication:

- set records the previous value and inverses to set or remove as appropriate;
- insert inverses to remove at the final canonical index;
- remove records the removed value and inverses to insert;
- move records the canonical post-removal indices and returns an exact reverse move;
- rename records the old key and refuses collisions;
- source replacement remains a bounded whole-document operation; large documents expose all fields through path edits and do not advertise an action that transport cannot carry.

Every aggregate retains its domain mutation enum. `PatchSnapshot` is a normal schema-first mutation variant with the framework patch payload, a `MutationLeaf` descriptor with exactly fourteen authority fields, aggregate JSON Schema membership, text and binary codecs, generated language declarations, a unique protocol tag, `Mutation::diff`, and `Mutation::inverse`. `SnapshotPatch` implements the framework `DslField` carrier as a structured value, so DslRecord/DslVariants aggregates such as MP4 preserve the typed tree without a JSON string adapter. Existing semantic leaves remain preferred when they are smaller or carry stronger domain intent. `SetSnapshot` remains only for bounded whole-document replacement.

The retained reducer prepares and schema-validates the patch against the exact editor dialect and document schema before it asks the editor for its native mutation. Validation resolves sequential operation paths against each intermediate typed snapshot and validates final context, so a move cannot accidentally validate its destination against the pre-removal shape. Admission encodes every wrapped forward mutation and every exact inverse against `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES`. An edit is admitted only when both directions fit. Large byte, sample, vertex, point, and frame arrays are edited by leaf or bounded range patches, so sibling payloads never enter the event. A multi-item retained publication must prepare all items before commit and cancellation must leave generation and root identity unchanged.

The first five aggregate protocol tags reserved by their next free record are PNG 19, WAV 5, JPEG document 13, TIFF document 9, and MP4 9. Their native leaf diff applies the already validated structural typed patch, and inverse derives a fresh compact patch from the actual publication base. The editor keeps its current semantic mapping first and selects `PatchSnapshot` only for the generic fallback.

## Concrete editor roots

All 54 assigned editor roots already parse the same six retained actions and therefore only need their current generic `SetSnapshot` fallback replaced by the derived path codec plus their aggregate's native `PatchSnapshot` wrapper. The inventory resolves to 43 unique mutation aggregates: GIF's two roots share `GifMutation`; IFC 2x3's four roots share `Ifc2x3Mutation`; STEP base and CC1 through CC6 share `StepMutation`; and both DWG versions share `DwgMutation`. The exact concrete dialect remains the controller and proof authority even where the mutation aggregate is shared.

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

The schema-first implementation therefore adds 43 native leaf wrappers and wires them through 54 editors. A shared aggregate receives one protocol tag and codec surface; its derived editors retain their own controller identity and tests rather than duplicating the mutation vocabulary.

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

## Schema-first pilot leaves authored while native carrier verification is queued

The five pilot roots now have authored, intentionally unmounted `patch-snapshot` leaf modules. Aggregate enums and editor fallbacks remain unchanged until the shared Rust carrier suite passes.

| Pilot | Binary tag | Authored surfaces | Leaf law |
| --- | ---: | --- | --- |
| PNG 1.2 any | 19 | 14-field descriptor, shared-patch JSON Schema reference, Rust leaf, direct text codec, direct binary codec | 2 MiB pixels retained while gamma changes; exact inverse; forward and inverse each below the 1 MiB store item limit; text/binary round-trip |
| WAV RIFF PCM any | 5 | 14-field descriptor, shared-patch JSON Schema reference, Rust leaf | sample past offset 1 MiB changes; all other samples retained; exact inverse; forward and inverse each below the 1 MiB store item limit; tagged text/binary round-trip |
| JPEG JFIF document | 13 | 14-field descriptor, shared-patch JSON Schema reference, Rust leaf, direct text codec, direct binary codec | 2 MiB pixels retained while quality changes; exact inverse; forward and inverse each below the 1 MiB store item limit; text/binary round-trip |
| TIFF 6.0 document | 9 | 14-field descriptor, shared-patch JSON Schema reference, Rust leaf, direct text codec, direct binary codec | one byte within 2 MiB pixels changes; all others retained; exact inverse; forward and inverse each below the 1 MiB store item limit; text/binary round-trip |
| MP4 ISOBMFF any | 9 | 14-field descriptor, shared-patch JSON Schema reference, `DslRecord` Rust leaf using the first-party `SnapshotPatch: DslField` carrier | 2 MiB sample retained while movie title changes; exact inverse; forward and inverse each below the 1 MiB store item limit; derived text/binary round-trip |

All ten new JSON files parse, and every mutation descriptor has exactly the required fourteen authority fields. The tests are staged within the unmounted leaf modules and become executable with the atomic aggregate/codec/editor mount.
