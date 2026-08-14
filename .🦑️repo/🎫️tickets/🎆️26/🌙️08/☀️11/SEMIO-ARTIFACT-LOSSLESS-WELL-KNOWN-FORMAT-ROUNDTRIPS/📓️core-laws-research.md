# Core Laws and Lossless Artifact Roundtrip Research

## Scope

This discovery maps the existing snapshot, diff, mutation, pack, DSL, and I/O paths for the five requested real-world artifacts. It identifies why the current laws can pass while `export(import(source))` is not byte-identical, and proposes clean ownership boundaries for parallel implementation.

No implementation files were changed and no test suite was run in this discovery track. The source/fixture hashes and byte comparisons below were gathered with read-only commands.

## Source Artifacts

| Source | Bytes | SHA-256 | Initial signature | Existing fixture relationship |
| --- | ---: | --- | --- | --- |
| `temp/📄️bachelor-thesis.pdf` | 6,346,331 | `83ebe31253bfa881ab9478e9e79d1a774e2abee7ddc27fc8ce9613d47d9c9ad3` | `%PDF-1.5` | Byte-identical to the committed bachelor-thesis PDF asset |
| `temp/architectural_example.dwg` | 148,638 | `52d14a7bdb946099d3cf16fd276d19bd8924348fd02b2ddd0003cd4f6b34cce7` | `AC1024` | Byte-identical to the committed architectural DWG asset |
| `temp/artifacts.svg` | 423,414 | `62a1922aad9e06ba7d4a55fe13c360f286eb720873bcf6d8e6ffd1d52e782fc9` | `<?xml version='1.0' encoding='UTF-8'?>` | No exact-byte fixture relationship established |
| `temp/bauen-mit-bestand.mp4` | 16,086,051 | `54b0672cca68a474d44c6096abb6579160b4d33b0f637f588e2e0752373e05c7` | ISO-BMFF `ftyp isom` | No exact-byte fixture relationship established |
| `temp/domai-specific-programmaning-language-for-architects.pptx` | 16,341,544 | `477900b1746139840890bc4edb653c488f3d18f9da34d231332b5db41d4caa8a` | ZIP/OPC | No exact-byte fixture relationship established |

The committed equivalent fixtures are:

- PDF: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf`
- DWG: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🖼️assets/📄️architectural.dwg`

The DWG fixture directory name is `ac1018`, but the bytes declare `AC1024` and the canonical code path is AC1024.

## Shared Architecture

### Snapshot algebra and codecs

`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` defines the core algebra:

- `MutationDiff<P>`: `apply` and `absorb`. Its invariant is that `absorb(d1, d2).apply(base)` equals `d2.apply(d1.apply(base))`; absorption is expected to be associative, structural, total, and independent of a concrete base value.
- `DiffAlgebra<P>`: `inverse`, `between`, and `is_empty`. Its invariants are inverse restoration, `between(a, b).apply(a) == b`, and an empty `between(a, a)`.
- `Mutation<P>`: a mutation produces a diff and an inverse mutation.
- `OpText`: one-line print/parse equality for mutations.
- `OpBinary`: deterministic encode/decode equality for mutations.
- `DiffCodec`: one-line text and deterministic binary equality for diffs.

These contracts operate entirely on the in-memory snapshot. They cannot detect information discarded before the snapshot is constructed.

### Artifact DSL and pack

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` defines:

- `ArtifactDsl`: `parse(print(snapshot)) == snapshot`; hand-authored text may normalize.
- `ArtifactPack`: `decode_pack(encode_pack(snapshot)) == snapshot`; the pack is intended to represent the same structure as the DSL.
- `ArtifactCodec`: a type-erased facade for compile, print, diff, and mutation operations.
- `register_document_codec`: stores a codec in a `HashMap` keyed only by schema; duplicate schema registration silently replaces the prior entry.

Handwritten pack implementations return `record_spec() == None`. Consequently, `ArtifactCodec::of` derives a zero schema hash for these snapshots. This weakens mismatch detection independently of byte preservation.

### Plugin I/O

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` defines the pluggable surfaces:

- `ArtifactSerializer`
- `ArtifactDeserializer`
- `ArtifactComposer`
- `composer_entry_of`
- `serializer_entry_of`
- `deserializer_entry_of`

`🧰️framework/🔨️modules/🚪️io/🦀️component.rs` implements resolution and dispatch with `resolve`, `io_dispatch`, and the two-hop `io_compose_via` path. Erased binary payloads are normally semio `ArtifactPack` bytes, not arbitrary native file bytes.

### Raw-byte versus packed-byte contract mismatch

The current real-format analyzers sniff native bytes, but their binary `analyze` branches decode those same bytes through `<Snapshot as ArtifactPack>::decode_pack`. A native PDF, DWG, MP4, or PPTX is not a semio pack envelope, so sniffing and analysis disagree about the payload contract.

Dedicated leaf functions can perform the native conversion, but they are not registered as serializer/deserializer entries:

- PDF raw import/export leaves call `decode_pdf`/`encode_pdf`; their `register()` bodies are empty.
- DWG raw import/export leaves call `decode_dwg`/`encode_dwg`; their `register()` bodies are empty.
- MP4 and PPTX follow the same pattern.
- SVG raw text happens to work because its `ArtifactDsl` accepts XML text after the preamble is split, but it still canonicalizes lexical form.

The binary helper named `deserialize_bytes` decodes a packed `BinarySnapshot`; it does not directly decode a native file. Native bytes must first be wrapped in `BinarySnapshot` and passed through the proper native deserializer. This ambiguity blocks a trustworthy end-to-end raw dialect route even before exact preservation is considered.

## Existing Eight Laws

The current format tests collectively implement eight relevant laws:

1. `field_sweep`: a fixture pair differs in every mutable snapshot field, and the diff/mutation vocabulary covers those fields in both directions.
2. `mutation_diff_law`: applying a mutation's computed diff to the base equals the actual mutated snapshot.
3. `inverse_law`: mutation inverse and/or diff inverse restores the exact in-memory snapshot.
4. `absorb_law`: a coalesced diff has the same effect as sequential application, is associative, and transports collection indices correctly.
5. `between_roundtrip_law`: `between(a, b).apply(a) == b` and `between(a, a)` is empty.
6. `codec_retention_law`: the existing per-format interpretation is normally decode/encode/decode snapshot equality or equality to a documented normal form.
7. `op_text_binary_roundtrip_law`: mutation text and binary codecs preserve equality; output is deterministic and text is one line.
8. `diff_codec_text_binary_roundtrip_law`: diff text and binary codecs preserve equality; output is deterministic and text is one line.

Representative test regions are embedded in the existing diff, mutation, and I/O files rather than separate test files:

- PDF: `.../📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`, `.../🧬️schema/🧬️mutations/🦀️component.rs`, and the bachelor-thesis example's `🧪️tests/🦀️test.rs`.
- DWG: `.../🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` and `.../🧬️schema/🧬️mutations/🦀️component.rs`.
- SVG: `.../🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` and `.../🧬️schema/🧬️mutations/🦀️component.rs`.
- MP4: `.../🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`, `.../🧬️schema/🧬️mutations/🦀️component.rs`, and `.../🚪️io/🦀️component.rs`.
- PPTX: `.../🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` and `.../🧬️schema/🧬️mutations/🦀️component.rs`.

### Why all eight can be green while export is different

Every law quantifies over a snapshot after import. None asserts:

```text
export(import(source_bytes)) == source_bytes
```

`codec_retention_law` explicitly permits format-specific normalization. `field_sweep` cannot cover fields that do not exist in the snapshot. Mutation and diff laws preserve the modeled snapshot, not bytes discarded by parsing. Text/binary operation laws validate operation serialization, not native artifact serialization.

Exact losslessness therefore requires either strengthening `codec_retention_law` with an exact-source mode or adding an explicit cross-layer native-byte law. The latter is clearer because canonical regeneration after semantic mutation remains a distinct valid mode.

## Format Findings

### PDF

Relevant files:

- Declaration: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🦀️component.rs`
- Snapshot/DSL/pack: `.../📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- Diff: `.../🧬️schema/🔺️diff/🦀️component.rs`
- Mutations: `.../🧬️schema/🧬️mutations/🦀️component.rs`
- Native codec: `.../🚪️io/🦀️component.rs`, notably `decode_pdf` and `encode_pdf`
- Raw binary import: `.../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️component.rs`
- Corresponding raw binary export serializer under the parallel export tree

The root registers a canonical `stdio.pdf.1.7` declaration and a frozen 1.4 declaration for the same artifact kind. A `%PDF-1.5` file routes to the 1.7 implementation.

`PdfSnapshot` stores a parsed COS graph (`objects`, `trailer`) plus typed pages and info. The writer explicitly regenerates a fresh minimal file from pages and info; it does not re-emit the imported object graph and trailer. The DSL hex-encodes regenerated PDF bytes, and the pack wraps regenerated PDF bytes in the semio envelope.

`PdfDiff` covers declared version, info, pages, objects, and trailer. Mutations include set-snapshot, page changes, info changes, and object/trailer changes. This creates a stronger inconsistency than simple normalization: object/trailer mutations alter the snapshot and satisfy diff laws, but `encode_pdf` ignores those fields. The mutation is therefore invisible in exported bytes.

Existing real-fixture assertions describe structural/page-level normal-form retention, not source-byte identity.

Required boundary: retain the complete source PDF bytes for untouched replay. On semantic change, the PDF writer may regenerate only changes it actually represents. An object/trailer mutation must either be encoded faithfully or return an explicit unsupported-export error; it must never silently disappear.

### DWG

Relevant files:

- Declaration: `.../🖊️dwg/🦀️component.rs`
- Snapshot/DSL/pack/native preamble codec: `.../🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- Diff: `.../🧬️schema/🔺️diff/🦀️component.rs`
- Mutations: `.../🧬️schema/🧬️mutations/🦀️component.rs`
- Section I/O: `.../🚪️io/🦀️component.rs`

`DwgSnapshot` already stores `bytes` alongside version, maintenance version, codepage, section names, parsed sections, and decode status. Its documentation treats those bytes as authoritative. Decode copies the complete source, and encode returns that exact byte vector after validating header consistency. Untouched native import/export is consequently exact.

The remaining gap is mutation coherence. Version-info mutations patch the authoritative preamble bytes. Section insert/remove/set mutations update only structural fields, while export still returns the old bytes. These mutations can satisfy snapshot laws while disappearing on export.

Required boundary: align DWG with the shared source-backing contract and preserve its current fast exact replay. Any structural mutation that cannot rebuild valid DWG bytes must fail export explicitly. A stale-byte replay after a semantic mutation is invalid.

### SVG and XML

Relevant files:

- SVG declaration: `.../🎨️svg/🦀️component.rs`
- SVG snapshot/DSL/pack: `.../🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- SVG diff/mutations: sibling `🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs`
- XML model/parser/writer: `.../📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`

`SvgSnapshot` contains only schema metadata and an `XmlDocument`; it has no original text or bytes. The XML model preserves semantic node order, comments, CDATA, processing instructions, and doctype, but not lexical spelling.

The parser trims the whole source, resolves entity spellings, and loses quote and spacing choices. The writer uses double quotes, normalized declaration formatting, canonical entity escaping, its own self-closing form, and inserted newlines. The requested SVG begins with single-quoted XML declaration attributes, so a current writer pass necessarily changes bytes even before deeper differences are considered.

Required boundary: store original UTF-8 bytes, not merely an XML syntax tree. Replay them when the semantic fingerprint is unchanged. The existing XML writer remains the canonical regeneration path after supported semantic edits.

### MP4

Relevant files:

- Declaration: `.../🎥️mp4/🦀️component.rs`
- Snapshot/DSL/pack: `.../🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- Diff/mutations: sibling `🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs`
- Native codec: `.../🚪️io/🦀️component.rs`, notably `decode_mp4` and `encode_mp4`

`Mp4Snapshot` stores `ftyp`, typed video tracks and samples, and unknown top-level boxes. It does not store the complete source bytes. The codec documents exact retention of selected payloads and timing, but whole-file normal form rather than byte identity.

Decode/encode loses or regenerates parts of `moov` metadata such as timestamps, matrices, volume, handler naming, chunk tables/layout, and top-level ordering. The encoder emits its chosen `ftyp`, unknown-box, `mdat`, `moov` ordering. Existing retention tests validate decode/encode/decode snapshot equality and sample containment, not native byte equality.

Required boundary: preserve the complete ISO-BMFF source for untouched replay. After supported edits, use the canonical writer and make its normalization explicit. A modeled mutation must be visible in regenerated output or fail.

### PPTX, OPC, and ZIP

Relevant files:

- PPTX declaration: `.../🎞️pptx/🦀️component.rs`
- Snapshot/DSL/pack: `.../🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- Diff/mutations: sibling `🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs`
- PPTX import: `.../🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🦀️component.rs`, notably `decode_pptx`
- PPTX export: parallel export serializer, notably `encode_pptx`
- OPC codec: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🦀️component.rs`, notably `OpcPackage`, `decode_opc`, and `encode_opc`

`PptxSnapshot` stores an `OpcPackage` plus a typed presentation view. The OPC package preserves decompressed part payloads, content types, and relationships, but not the original ZIP byte stream.

`decode_opc` discards ZIP entry metadata and parses content-type/relationship XML. `encode_opc` regenerates those XML parts, forces entries through its chosen Deflate behavior and default metadata, sorts relationship owners, and clears the archive comment. Its contract is semantic equivalence, not byte equality. PPTX export additionally regenerates presentation parts from the typed view before encoding OPC.

Preserving every decompressed part is insufficient for exact PPTX identity because ZIP local headers, central directory metadata, compression streams, ordering, and comments are observable bytes.

Required boundary: retain the original whole ZIP/PPTX bytes in the PPTX snapshot backing. Use them unchanged while semantics match the imported baseline. OPC and presentation regeneration remain the post-mutation canonical path.

## Systemic Design Recommendation

### Persisted source backing

Introduce one framework-owned, schema-first source-backing concept used by every native artifact snapshot. It should contain at least:

- exact original native bytes;
- native format/dialect identity and version;
- a deterministic semantic baseline fingerprint computed from the modeled snapshot while excluding the backing itself.

The bytes are artifact state, not an ephemeral cache: the required observable behavior includes their exact identity. The backing must survive snapshot equality, pack and DSL roundtrips, history, undo, diff application, and mutation operation codecs.

For the current 6–16 MB examples, directly persisted bytes are the simplest correct representation. Content-addressed deduplication can be an internal optimization later; it must not become an external runtime dependency or weaken portability.

### Derived cleanliness, not scattered dirty flags

Export should make one centralized decision:

1. Compute the current semantic fingerprint, excluding source backing.
2. If it equals the imported baseline, emit the exact backed bytes.
3. If it differs, invoke the format-specific canonical serializer.
4. If the serializer cannot represent the mutated semantics, return an explicit unsupported-export error.
5. After successful regeneration, the newly emitted bytes and their semantic fingerprint can become the refreshed backing/baseline.

Do not add manually maintained `dirty` booleans to every mutation. Such flags are easy to miss in `apply`, `inverse`, `absorb`, `between`, set-snapshot, and future mutations. Cleanliness is a derived relationship between current semantics and the stored baseline.

### Algebra participation

The source backing must be deliberately represented in:

- snapshot equality;
- `DiffAlgebra::between`, `apply`, `inverse`, `absorb`, and `is_empty`;
- set-snapshot mutations;
- mutation text/binary codecs;
- diff text/binary codecs;
- `ArtifactDsl` and `ArtifactPack`.

A shared opaque-backing diff operation is preferable to five ad hoc byte fields. Between/imported snapshots with equal semantics but different bytes must not silently collapse if exact-byte state is part of the contract.

### Native I/O contract

Separate three binary notions in APIs and registration:

- raw native artifact bytes;
- `BinarySnapshot` payload bytes;
- semio `ArtifactPack` envelope bytes.

Analyzer sniffing and analyzer decoding must operate on the same declared notion. Register the existing native serializers and deserializers, or move their behavior behind a single registered composer contract. A native format must never be passed to `ArtifactPack::decode_pack` merely because both are byte arrays.

## End-to-End Acceptance Matrix

Run every row against all five named files and compare length, SHA-256, and byte-for-byte equality:

| Row | Pipeline | Required result |
| --- | --- | --- |
| A | Native import → native export | Exact original bytes |
| B | Import → `ArtifactPack` encode/decode → export | Exact original bytes |
| C | Import → `ArtifactDsl` print/parse → export | Exact original bytes |
| D | Import → no-op mutation and empty diff → export | Exact original bytes |
| E | Import → `between(s, s)` → apply → export | Exact original bytes |
| F | Import → set-snapshot mutation → `OpText`/`OpBinary` roundtrip → apply → export | Exact original bytes |
| G | Import → representative mutation → inverse → export | Exact original bytes |
| H | Import → diff codec roundtrip → apply → inverse → export | Exact original bytes |
| I | Registered `io_dispatch` native dialect → native snapshot → reverse dispatch | Exact original bytes |

The comparison target is the unwrapped external native file, not a `.pack.semio` container. During implementation verification, temporary runtime evidence must use the required `[DEBUG] ` prefix and be removed from implementation code afterward. Permanent test assertions should remain.

For a representative mutation that is supported by a canonical writer, add a second assertion that the exported bytes decode to the mutated snapshot. For unsupported mutations, assert a typed error instead of stale source replay.

The focused project runner is expected to be `bun nx test @semio-tech/stdio-plugin`; confirm the actual target in `project.json` before execution. Full relevant Nx verification and repository policy checks belong to the integration closer.

## Parallel Workforce and Ownership Boundaries

### Wave 0: contract and failing baseline

One shared-contract owner should:

- define the persisted backing schema and semantic fingerprint contract;
- extend existing shared snapshot/diff/mutation/pack test regions with reusable exact-byte assertions;
- pin the five source hashes above;
- demonstrate the current failing pipelines without changing format behavior;
- define raw native versus semio-pack I/O types before format lanes edit their code.

This owner is the only writer for shared algebra/store surfaces during the wave.

### Wave 1: independent format lanes

Run five format lanes in parallel after the shared contract stabilizes:

1. PDF owner: snapshot backing, DSL/pack retention, exact untouched export, mutation/export representability, PDF real-fixture laws.
2. DWG owner: adopt shared backing around existing authoritative bytes and make structural mutation export failures explicit.
3. SVG/XML owner: preserve original UTF-8 bytes, fingerprint XML semantics, retain canonical XML writer for changed documents.
4. MP4 owner: preserve whole ISO-BMFF source and validate supported mutations against regenerated decode.
5. PPTX/OPC/ZIP owner: preserve whole container bytes at the PPTX boundary and validate OPC regeneration only after mutation.

Each lane owns only its artifact snapshot, diff, mutation, native codec, and embedded existing test regions. It should not edit shared registration/glue files.

### Wave 2: I/O registration lane

A dedicated I/O owner should resolve the raw-native/`BinarySnapshot`/`ArtifactPack` mismatch, register the native leaf conversions, and exercise `io_dispatch` both directions. This lane should start after the shared binary contract is fixed and can run alongside later format work.

### Wave 3: single-writer integration

One integration owner should be the sole editor for hot files such as:

- `✏️s/🔌️plugins/🗄️stdio/📦️glue.rs`;
- artifact root declarations;
- catalogs and shared registration tables;
- root `📜️script.ts`, `project.json`, and launch configuration if new permanent commands are required.

The closer runs the full five-by-nine acceptance matrix, relevant Nx targets, runtime verification, and policy checks. It also verifies that no format silently exports stale backing after a semantic change.

## Highest-Risk Gaps

1. The native binary analyzer contract is internally inconsistent: sniff consumes native bytes while analyze consumes semio pack bytes.
2. PDF object/trailer and DWG section mutations can currently satisfy snapshot laws yet disappear on export.
3. PDF, SVG, MP4, and PPTX snapshots discard byte-level source information needed for exact replay.
4. PPTX cannot be made byte-exact by preserving OPC parts alone; the whole original ZIP byte stream is required.
5. The existing eight laws do not cross the snapshot/native-byte boundary.
6. Codec registration keyed only by schema silently overwrites duplicates, and handwritten packs use a zero schema hash.

## Completion Criterion

The work is complete only when every row A–I is green for every named source artifact, supported mutations remain semantically observable after regeneration, unsupported mutations fail explicitly, and the external exported native bytes are exactly equal to the imported bytes whenever the semantic state returns to the imported baseline.
