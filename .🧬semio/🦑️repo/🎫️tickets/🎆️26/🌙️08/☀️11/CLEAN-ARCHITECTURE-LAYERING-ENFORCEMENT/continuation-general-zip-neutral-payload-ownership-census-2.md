# General ZIP Neutral Payload Ownership Census

Read-only current-source review. No source changes or new runtime/dependency/test result is claimed. The exact textual census, matched source lines, file hashes and counts are retained in `🗑️generated/zip-neutral-census-2.json`; the complete file roster follows below. This scan covers repository-visible Rust, TypeScript, JSON, protobuf, GraphQL, EBNF and ANTLR source using normal ripgrep ignore rules. Counts include declarations, imports, comments and tests; they are not counts of compiled call sites. Hidden ticket/generated trees and ignored build outputs are not production consumer evidence. The Layout export module also owns a distinct local `ZipEntry`, which must not be conflated with StdIo payload ownership.

## Concrete Block Boundary

Six Block Rust files use this ZIP boundary: one export and one import leaf for each of 2D, 3D and 5D. Each export constructs the StdIo-owned `ZipSnapshot` containing two StdIo `ZipEntry` objects, names `snapshot.block{2d,3d,5d}.semio` and `snapshot.json`, and calls StdIo `encode_zip`; each import calls StdIo `decode_zip`, searches DSL before JSON and parses the first matching UTF-8 member. Foreign dialect registration remains `s.stdio.zip@2.0/*`. General payload/algorithm extraction must conserve member order, names, DSL preference, error attribution and dialect registration. Removing the Cargo edge alone would leave actual foreign snapshot and codec uses.

The exact 2D export owner is `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` (imports18–20, construction31–38, call48). Its corresponding import owner is the sibling `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` (codec17, member search27–34). Full 3D/5D paths are in the roster.

## Persisted Payload Ownership

The concrete owning Rust definitions are in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs`. That directory contains the JSON-schema, TypeScript, protobuf and GraphQL projections alongside fixtures and shadow tests. Its six records are `ZipExtraField`, `ZipLocalHeaderMetadata`, `ZipCentralHeaderMetadata`, `ZipEntryMetadata`, `ZipEntry` and artifact `ZipSnapshot`.

Neutral payload ownership must retain names, member bytes, compression-method code, separate local/central version and flags, DOS timestamps, ordered extra fields, explicit Unicode-path legacy bytes, central Unicode-comment legacy bytes, central comment, internal/external attributes and descriptor-signature choice. Archive-level comment and its persisted UTF-8/CP437 choice are also required. Metadata defaults are observable: compression8, local version20, central made-by45/version20, UTF-8 flag0x0800, DOS date0x0021, no descriptor signature. Unknown extra-field bytes remain exact; derived0x0001/0x7075/0x6375 fields retain empty marker payloads and regenerate from current sizes/offsets/Unicode text plus explicit legacy bytes. Omitting this metadata would narrow current save/reopen behavior.

General ownership should comprise an independently defined, schema-first archive payload and member/header/extra-field records with all these fields, separate from StdIo artifact identity. `ZipSnapshot.schema`, `STDIO_ZIP_DOCUMENT_SCHEMA`, ArtifactSchema/state annotations and `s.stdio.zip` mutation/editor/composer registration remain artifact ownership. Move the neutral record definitions and their owned schema/projection requirements together; manually update all consumers and projections. A General API must not name or require StdIo snapshot/types, domain dialects, external runtime types or a compatibility adapter. First-party Value/DSL record serialization obligations must remain explicitly owned and tested rather than assumed satisfied by moving Rust declarations alone.

The existing snapshot participates in JSON/Value, DSL, binary/Pack, SQLite, protobuf/GraphQL projections, snapshot mutations, retained editor commands, derived inference/analysis and ISO21320 validation. The complete roster records those consumers. Neutral payload projection schemas and artifact snapshot schemas must each describe their actual owner; the General payload cannot inherit the artifact schema URL or artifact-state metadata. Handcrafted fixtures/projections must be updated as one ownership change; no legacy schema or migration layer is indicated.

## Codec and Conformance Ownership

The current codec owner is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🦀️.rs`: `inspect_zip_central_entry_headers`389, `decode_zip`504, `validate_zip_snapshot_serialization`762 and `encode_zip`845. Header inspection has exactly two textual consumers/files: its definition and the ISO21320 subset IO owner. It performs central-directory inspection without decompressing members; preserve this path and the subset policy separately from algorithm extraction.

The complete codec includes CP437 and UTF-8 text handling, Unicode-extra CRC agreement, ordered unknown extras, ZIP64 read sizes/offsets/end-directory resolution, stored0 and Deflate8 payloads, payload CRC checks, local/central header agreement, data descriptors and regeneration of retained metadata. Existing refusals include encryption/masked headers, unsupported methods, multi-disk and ZIP64 writing. A neutral algorithm must preserve current acceptance/refusal and logical round-trip contracts. Existing `ZipError` carries archive-specific failures and first-party ValueRefusal classification; expose a first-party neutral error owner while conserving meaningful classification and caller attribution.

General already has `🧰️framework/🔨️modules/🗜️deflate/🎒️zip/🦀️.rs`, a separate deterministic minimal archive writer/reader. Its writer fixes timestamps/UTF-8 and omits extra fields; its reader refuses ZIP64. That API is narrower than the StdIo codec and cannot substitute for its complete persisted payload behavior. Consolidation must extend one neutral implementation to the complete retained contract instead of choosing the smaller supported subset. General Deflate is first-party, with only first-party Value runtime dependency and miniz_oxide/zip/serde/serde_json dev oracles (`📦️packages/🦀️rust/Cargo.toml`).

StdIo raw Deflate currently owns tuned dynamic compression routines in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🚪️io/🦀️.rs`: deterministic1318, high-search1323, compact-high-search1328 and inflate1437. Their numeric configurations differ (8/1/4/258/1024,8/4/4/258/4096,7/4/4/258/4096 with sync). General fixed-Huffman compression is not evidence of equal deterministic container output. Any neutral extraction must preserve the tuned algorithms and first-party owned configuration interface and validate outputs against existing independent libraries/corpora.

## Caller Compression Policy

Current StdIo ZIP encode lines920–927 choose compact-high-search for case-insensitive `.bin`, high-search for `.emf`, and standard deterministic otherwise. Keep this document/container filename policy in StdIo. A neutral General ZIP algorithm should receive explicit first-party compression decisions/configuration or a supplied first-party compressor interface; it must not inspect filename suffixes to select Office/media policy. Preserve exact current profile choices in StdIo callers. This separation must not remove any supported encoding branch or replace tuned output with the smaller General writer.

Bounded/progress/cancellation operations are required for expensive General parsing, metadata validation, payload materialization and compression. Current synchronous wrapper signatures are not proof of that requirement. General controlled Deflate/retained inflate provide useful owned mechanisms, but the ZIP lifecycle, cumulative ownership accounting, cancellation frontiers and all affected caller joins need explicit laws and runtime observation before readiness.

## Validation Required Before Retirement

Use a language-neutral corpus for all metadata/encoding/descriptor/ZIP64-read/current refusal cases; compare parsed archives and relevant exact deterministic bytes with existing independent ZIP/Deflate libraries. Preserve current artifact edit/save/reopen, unknown-extra, Unicode, ISO21320, OPC/DOCX/PPTX/BCF, document archive and Block tests. Observe original declaring/native targets with executable compiler identity, full caller/schema/provider closure and actual dependency deletion only after those joins. No such extraction or gate is performed by this report.

## Exact Textual Census

| Symbol | Occurrences | Files |
|---|---:|---:|
| ZipEntry | 163 | 60 |
| ZipExtraField | 70 | 18 |
| ZipEntryMetadata | 52 | 23 |
| ZipLocalHeaderMetadata | 34 | 16 |
| ZipCentralHeaderMetadata | 34 | 16 |
| ZipSnapshot | 450 | 100 |
| encode_zip | 67 | 31 |
| decode_zip | 71 | 29 |
| inspect_zip_central_entry_headers | 2 | 2 |

129 distinct matching files. Paths below are repository-relative and exact; match lines and hashes are in the generated authority.

- `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🔣️.json` — decode_zip, encode_zip
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — ZipEntry, ZipSnapshot, encode_zip
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — decode_zip
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🔬️m5-auto-discovery/🦀️.rs` — decode_zip
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — ZipEntry, ZipSnapshot, encode_zip
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — decode_zip
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — ZipEntry, ZipSnapshot, encode_zip
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — decode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs` — ZipEntry, ZipSnapshot, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🔮️oracles/🦀️.rs` — ZipEntry
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🔮️oracles/🔣️.json` — decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🚪️io/🦀️.rs` — ZipEntry, ZipSnapshot, decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🚪️io/🧪️tests/🔬️unit/🦀️.rs` — ZipEntry, ZipSnapshot, encode_zip
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🧪️tests/🔬️unit/🦀️.rs` — ZipSnapshot, encode_zip
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — decode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧪️tests/🔬️unit/🦀️.rs` — decode_zip
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🔣️.json` — decode_zip, encode_zip
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — decode_zip
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json` — decode_zip, encode_zip
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — ZipEntry, ZipSnapshot, encode_zip
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🧪️tests/🔬️unit/🦀️.rs` — encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🟦️.ts` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🦀️.rs` — ZipEntry, ZipSnapshot, decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🧪️tests/🔬️unit/🦀️.rs` — ZipEntry, ZipSnapshot, decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/✏️editor/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🟦️.ts` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/👁️viewer/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🟦️.ts` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/✍️set-entry-data/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/🗜️add-deflated-entry/🧬️schema/🔣️.json` — ZipEntry
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/🗜️add-deflated-entry/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/🏷️rename-entry/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/➖remove-entry/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/💬set-archive-comment/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/📦add-stored-entry/🧬️schema/🔣️.json` — ZipEntry
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/📦add-stored-entry/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧪️tests/🔀️mutate-zip-2-0-iso21320/🦀️.rs` — ZipSnapshot, decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🔮️oracles/🦀️.rs` — ZipEntry
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🔮️oracles/🔣️.json` — decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🚪️io/🦀️.rs` — ZipEntry, ZipSnapshot, inspect_zip_central_entry_headers
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🚪️io/🧪️tests/🔬️derived-composition-unit/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata, ZipSnapshot, decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🟦️.ts` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/👁️viewer/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🗃️entries/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🗃️entries/🧪️tests/🔬️unit/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🛰️.proto` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🟦️.ts` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🛰️.proto` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts` — ZipEntry, ZipEntryMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs` — ZipEntry, ZipEntryMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔗️.graphql` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔣️.json` — ZipEntry, ZipEntryMetadata
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/✍️set-entry-data/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🛰️.proto` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🟦️.ts` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🏷️rename-entry/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/➖remove-entry/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/➕add-entry/🧬️schema/🔣️.json` — ZipEntry
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/➕add-entry/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔗️.graphql` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/↩️inverse/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/🔺️diff/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/🧪️tests/📖️extends/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/💬set-archive-comment/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🦀️.rs` — ZipEntry, ZipSnapshot, decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛰️.proto` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🔬️shadow/🦀️.rs` — ZipEntry, ZipEntryMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔗️.graphql` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔗️.graphql` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧪️tests/🔀️mutate-zip-2-0/🦀️.rs` — ZipSnapshot, decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🔮️oracles/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🔮️oracles/🔣️.json` — decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📝️text/🔺️diff/🦀️.rs` — ZipEntry, ZipEntryMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📝️text/🧬️mutations/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🔣️.json` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs` — ZipSnapshot, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs` — ZipSnapshot, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🦀️.rs` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata, ZipSnapshot, decode_zip, encode_zip, inspect_zip_central_entry_headers
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🧪️tests/🔬️codec/🦀️.rs` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata, ZipSnapshot, decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/💾️binary/🔺️diff/🦀️.rs` — ZipEntry, ZipEntryMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/💾️binary/🧬️mutations/🦀️.rs` — ZipEntry, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts` — ZipExtraField, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` — ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` — ZipEntry, ZipEntryMetadata, ZipSnapshot, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/📦️public/🟦️.ts` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs` — ZipExtraField, ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🔖️raw/✳️any/🦀️.rs` — ZipSnapshot, decode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗜️deflate/🔖️rfc1950/✳️any/🦀️.rs` — ZipSnapshot, decode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🧩️composition/🔣️.json` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/✏️editor/🟦️.ts` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/✏️editor/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — ZipEntry, ZipSnapshot, decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/✏️editor/🧵️retained/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🦀️.rs` — ZipSnapshot, decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🧪️tests/✏️create-and-edit-archive/🦀️.rs` — decode_zip, encode_zip
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🔮️oracles/🔣️.json` — decode_zip, encode_zip
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — decode_zip
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs` — ZipSnapshot, encode_zip
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs` — ZipSnapshot
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs` — ZipEntry
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/🦀️.rs` — ZipEntry

## Existing General Archive Callers

Separate direct General ZIP search observes two production caller files: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs` (ZipArchiveError278/305–306, writer343, parse372) and `🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🦀️.rs` (imports6, writer226). General Deflate ZIP unit tests are at `🧰️framework/🔨️modules/🗜️deflate/🎒️zip/🧪️tests/🔬️unit/🦀️.rs` and already use third-party ZIP as oracle. These two production callers must also join a consolidated neutral ZIP implementation and retain their deterministic behavior; a StdIo-only extraction cannot ignore existing General writer/reader consumers. This separate search is outside the nine-symbol129-file census above.
