# Stdio Artifact Schema, Codec, And Editing-Coverage Audit

## Scope And Method

Read-only audit of `✏️s/🔌️plugins/🗄️stdio`, completed 2026-09-26. Counts were obtained from the filesystem and source declarations; no product code or tests were modified or run.

`🗿️artifacts` contains 40 direct artifact directories. Four (`🏃️commands`, `📇️inventory`, `🕸️graph`, and `🛂️contract`) have neither a native `🚪️io/🦀️.rs` codec nor an `✏️editor/🦀️.rs`; they are framework/catalog artifacts rather than end-user file formats. The other 36 direct format families each have at least one native I/O codec and one Rust editor root.

The 88 externally-focused subset roots requested by this ticket are a mixture of independently materialized snapshots and derived subsets that reuse a base snapshot/mutation family. A local-path check therefore cannot be used as a pass/fail condition for all 88. There are 58 physical `🧬️schema/📸️snapshot/🦀️.rs` implementations across the complete tree, including the Semio-internal artifacts. 56 derive both `ToValue` and `FromValue`; the two exceptions are:

| Snapshot | Finding | Consequence |
|---|---|---|
| `🧿️semio/.../📦️object/🧬️schema/📸️snapshot/🦀️.rs` | No local `ToValue`/`FromValue` derive | Cannot be a direct generic-detail target unless an explicit value adapter is added. |
| `🖊️dwg/.../4️⃣ac1018/.../📸️snapshot/🦀️.rs` | No local `ToValue`/`FromValue` derive | Same gap; AC1018 needs a value projection or an intentional alias to AC1024. |

All other physical snapshots already expose the value boundary needed for a generic, typed detail editor. Do not serialize arbitrary native bytes to JSON as a substitute: edits must convert a `DslValue` back through `FromValue`, validate the exact snapshot type, then emit its typed mutation.

## Snapshot Commit Coverage

`SetSnapshot` is materially inconsistent. There are 59 operation modules at `🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs` (49 external plus 10 Semio-internal), but only 20 canonical typed mutation leaves at `🧬️schema/🧬️mutations/📸️set-snapshot/🦠️mutation/🦀️.rs`. The external roots with a local operation module are:

- LAS 1.0 header; HTML 5 any; EPW; ZIP 2 base and ISO21320; GIF 87a and 89a base; MP4; MP3; IFC 4 and IFC 2x3 base; binary raw; CSV; STEP AP214 base; TSV; OOXML bases (XLSX, DOCX, PPTX); Markdown; XML valid; JPEG baseline; AVI hdrl; WAV; STL; DWG AC1024; DXF header; TIFF baseline; Deflate; OBJ geometry; PLY; JSON i-json.

The following externally-focused subset roots have no local `SetSnapshot` reference in their own `🧬️schema` tree. Some are intentionally derived from a base operation, but this must be registered explicitly before a universal details editor advertises “apply all fields”; otherwise the action has no typed command target.

- BCF snapshot and viewpoint; AVI movi and idx1; DXF entities, tables, blocks; DWG AC1018; JSON GeoJSON.
- PDF 1.4 base/A/X and PDF 1.7 base/A/UA/VT/X/E/H.
- LAS points and VLR; GIF 89a comment/graphic-control/application.
- glTF animation, buffer, scene, material, mesh, asset, skin, and camera; OBJ material.

Derived standards also need a registration-level `snapshotCommit` mapping to the owning base mutation. It must be an explicit capability, not a guessed path convention. This makes every one of the 88 roots either (a) own a `SetSnapshot`, (b) name its exact base owner, or (c) report that whole-snapshot replacement is unavailable with the reason.

## Native Codec And Preservation Assessment

Native codec roots exist for all 36 file-format families, but that only proves a decoder/encoder path exists. It does **not** prove lossless native save or that each native detail is represented by a mutable field. The codecs generally normalize output; semantic equality after decode → encode is the appropriate contract unless a format intentionally guarantees byte retention.

High-risk preservation families should receive priority tests before their generic detail editor is enabled:

| Family | Why native save can lose material | Required contract |
|---|---|---|
| PDF, OOXML (DOCX/XLSX/PPTX), BCF, ZIP | Package order, extra ZIP fields, relationship/XML part ordering, unknown package parts | Preserve opaque package entries and all unmodelled parts; round-trip each entry’s bytes when untouched. |
| MP4, AVI, GIF, WAV, MP3, LAS | Chunk/box/frame ordering, padding, unknown chunks, index/offset rewrites | Preserve unknown units and order; rebuild offsets/checksums only for touched structures. |
| PNG, JPEG, TIFF, BMP | Ancillary segments/tags, color profiles, compressed image payloads | Preserve every unedited segment/tag/payload byte; isolate pixel re-encoding to an explicit pixel edit. |
| DWG, DXF, IFC, STEP, OBJ, STL, PLY, glTF | Unsupported entities/tables/sections or representation-specific syntax | Represent unsupported native elements as editable opaque records; preserve their original bytes/text if untouched. |
| HTML, XML, SVG, JSON, Markdown, CSV/TSV/TXT, EPW | Whitespace, quoting, declaration/prolog, comments, unknown nodes, line endings | Preserve lexical tokens where the snapshot carries them; state normalization rules where it does not. |

The source does contain `unknown`/`opaque`/raw handling in each sampled complex family, including `PNG`, `JPEG`, `TIFF`, `MP4`, `PDF`, OOXML, LAS, and DWG. That is not a complete proof: a format-level test must demonstrate an untouched unknown item survives native decode → mutate unrelated known field → encode → decode, with native byte equality for the unknown item and semantic equality for the whole snapshot.

## Existing Editing Surface

There is no reusable generic property/details editor in the inspected stdio or framework editor sources. Current artifact editors convert a small hand-written `EditCommand` from actions and render a format-specific main window. Examples include the typed action parsers in `🎨️svg/.../✏️editor/🦀️.rs`, `📖️pdf/.../✏️editor/🦀️.rs`, and `🏗️ifc/.../✏️editor/🦀️.rs`. `ToValue`/`FromValue` is the reusable model boundary, not an existing property UI.

The new shared kit should therefore be implemented once, below per-format editors, with this exact contract:

1. `DetailsSchema`: root identity, a typed snapshot-value projector/parser, exact `snapshotCommit` capability, field metadata, and custom format-control slots.
2. `DetailsPath`: object property, array index, map key, variant arm, and opaque payload range. Edits use immutable `DslValue` path replacement and preserve untouched siblings verbatim.
3. `ValidateDetailsPatch`: runs `FromValue` against the complete candidate snapshot, returns field-path diagnostics, and never emits a mutation for an invalid candidate.
4. `CommitDetailsSnapshot`: emits only the registered typed `SetSnapshot` (or a typed leaf mutation selected by a specialist control). It must expose cancellation/progress for large values/payloads.
5. `OpaqueNativeValue`: displays byte count, checksum, encoding, downloadable/source view, replacement and removal actions. It must not silently discard unknown data.
6. Field controls cover nullable values, booleans, bounded numbers, strings/enums, byte/hex/base64 payloads, objects, arrays with insert/move/remove, maps, discriminated unions, references, colors, durations, dates, matrices/vectors, and recursive values. Labels, descriptions, units, constraints, and English/German localized strings belong in `DetailsSchema`.

The detail editor needs a raw structured-value mode in addition to friendly controls. This is the only credible way to meet “every single detail” before every format gains a specialized panel.

## Implementation Split

Shared Sol agent: build the schema-driven details kit, its type/capability registry, immutable path patcher, validation/diagnostic UI contract, typed commit bridge, opaque-value control, and language-agnostic conformance fixtures. It must not hard-code an artifact family.

Format Sol agent A: document/package/text roots — ZIP, BCF, PDF, DOCX, XLSX, PPTX, HTML, XML, SVG, JSON, Markdown, TXT, CSV, TSV, EPW, binary, Deflate. Own base-to-derived `snapshotCommit` mappings and native preservation tests for package/text syntax.

Format Sol agent B: media/spatial roots — LAS, GIF, MP3, MP4, AVI, WAV, PNG, JPEG, TIFF, BMP, IFC, STEP, DWG, DXF, OBJ, STL, PLY, glTF. Own base-to-derived mappings and native preservation tests for binary units/unsupported geometry.

Both format agents should supply `DetailsSchema` descriptors plus domain controls, while the shared agent owns traversal and generic fields. No format agent should build another bespoke property editor.

## Required Validation

For every registered root, add one language-agnostic fixture that:

1. decodes a native fixture containing known and unknown/native-extension data;
2. converts the snapshot to `DslValue`, changes a nested scalar, converts it back, and commits the registered typed mutation;
3. verifies the changed field and every untouched semantic field after encode/decode;
4. verifies preservation of each untouched opaque item byte-for-byte; and
5. compares the resulting native file with a third-party parser/writer where an existing test dependency is available.

Add a registry test asserting the complete root set has one of `OwnsSetSnapshot`, `UsesSnapshotCommitOwner`, or `WholeSnapshotReplacementUnavailable { reason }`. The test is the enforceable definition of coverage; folder presence is not.
