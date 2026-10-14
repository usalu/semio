# Artifact IO Neutral Contract Audit — October 13

Read-only source audit. No producer changes, Git mutations, broad test runs, or runtime pass claims. Root coordinates the existing ticket and snapshot controls. Applicable root, framework products, OS and Repo AGENTS were read. Paths below are repository-relative; OS means `🧰️framework/🛍️products/💻️os/🔨️modules`, Framework means `🧰️framework/🔨️modules`, LIB means `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## Concrete Extraction Seam

`OS/🏪️store/🚪️io/🦀️.rs` is engine-free in its opening claim but not OS-neutral in its signatures. Move the pure representation contracts, not this entire file, into a separately packaged Framework IO owner. Its current `ArtifactPack` combines encoding, authored record metadata, SQLite factory discovery and OS registration.

| Current API | Proposed owner and split | Current consuming witness |
|---|---|---|
| `ArtifactDsl::{EXTENSION,parse_dsl,print_dsl,envelope_id}` | Neutral IO text snapshot contract, retaining first-party `TextError`; envelope identity is representation metadata | Store text `parse_document_*`; authored app text codecs |
| `ArtifactPack::{encode_pack_with,decode_pack_with,encode_pack,decode_pack,record_spec,record_spec_producer,borrowed_record_spec_producer}` | Neutral IO binary snapshot contract; use first-party pack options/errors and DSL record types directly | Store initialization owners/replay, Store sync, Store text print/parse pack |
| `ArtifactPack::native_snapshot_registration` | OS extension trait with explicit authored implementation, outside the neutral trait | `OS/🚪️io/🦀️.rs:1960` calls it when publishing; Run and Infinite DAG binary snapshot owners supply it |
| `ArtifactPack::sqlite_snapshot_codec` | OS snapshot capability extension, separate from binary representation | `OS/🏪️store/🦀️.rs:2400` fills erased codec provider; Run, Workflow, SpaceHistory and MCP workspace declare providers |
| `ArtifactNativeSnapshot` with `NativeSnapshotInput/Encoding` | Neutral controlled native IO bridge; its `NativeDecodeControl`, `NativeEncodeControl`, `ValueError`, `IoPayload` are first-party | `OS/🏪️store/📦️codec/📸️native-snapshot/🦀️.rs`, including the genuine `DslValue` bridge |
| `ArtifactPackReceiving::receive_pack` | OS receiving extension until its owner is proven neutral; currently takes `NativeSnapshotDecodeOwner` | Store native snapshot receiving factory |
| `ArtifactSqliteSnapshot::{to_sqlite_database,from_sqlite_database}` | Neutral relational representation trait using neutral SQLite database/control and first-party `ValueError` | Authored SQLite snapshot implementations |
| SQLite receiving, native decode/encode, preflight, subset validation and `sqlite_codec` methods | Explicit OS extension traits retaining original native custody and retirement, not blanket implementations | Store sqlite workspace/retirement/export/import functions and their existing custody tests |
| `ArtifactCodec`, `ArtifactCodecApplyFuture`, `ArtifactSqliteSnapshotCodec`, `ArtifactNativeSnapshotIdentity` | OS application codec/registration owner | Store codec registry, FolderEndpoint sync and native publication |
| `ArtifactPackFiles`, document pack blob encode/decode | OS document wire IO, because fields include authoritative SPR plus ops mirror and errors are `VcsError` | Store sync and media/WIT hops |

`ArtifactCodec` is explicitly an application table, not a neutral snapshot codec: `apply_ops_binary` receives VCS `EntityIdentityAuthority`; `edit_text_from_envelope` receives SPR `MutationEnvelope`; compile/mirror/replay functions use `VcsError`; its optional SQLite provider uses OS native receiving owners. Renaming/moving it into a neutral package would merely invert the dependency.

The available neutral implementations already include `Framework/🚪️io/🧬️schema` (`semio_framework_io_schema`), `🚪️io/🪶️sqlite-snapshot` (`semio_framework_io_sqlite_snapshot`), binary source and payload schema owners, DSL record and `Framework/🎒️pack/⚠️error/🦀️.rs`. The SQLite package has only first-party Value in its production dependencies; serde_json is development-only. Reuse these owned contracts rather than expose rusqlite/serde_json. `Framework/🌱️value/🔁️codec/🦀️.rs` owns `ToValue` and `FromValue`: semantic conversion to the first-party value tree remains legal under schema and must not be treated as native JSON serialization.

## Implementation Constraints

Remove registration hooks from the binary trait only when every caller has an explicit extension bound and every authored provider has a matching extension impl. Do not supply blanket `None` capability implementations: they overlap actual owners and conceal missed registration. Separate callers that require only pack encoding from callers that require OS registration. Preserve exact TypeId/provider identity checks and optional SQLite capability semantics.

Rust orphan rules become material when moving `ArtifactNativeSnapshot` to a separate neutral crate: OS can no longer implement a foreign neutral trait for foreign `semio_framework_value::DslValue`. That implementation must live with the neutral trait (allowed foreign type implementation), or in the Value owner only if dependency direction permits it. A local wrapper would change the public semantic model and should not be introduced merely to evade ownership. The same check applies to any foreign-tree ArtifactDsl/Pack impls before extraction. Authored snapshots remain local to their implementation crates and may implement neutral traits freely.

Avoid a neutral-to-OS dependency from default SQLite receiving methods. Their existing implementations call `super::sqlite_snapshot_workspace::{project,reconstruct,validate_decoding,validate_encoding}` and `sqlite_snapshot_retirement::{export,import}`. Keep those defaults in the OS extension, preserving all original receiving authorities. Likewise, pack derive expansion currently documents calls through `::semio_framework_os_kernel::os_pack::pack_rt`; change generated imports and consumers together to direct first-party neutral owners, then inspect generated public signatures. Reexports through OS are not extraction evidence.

## Narrow Ownership Checker Extension

`LIB/🚪️io/🏛️architecture/🟦️.ts` uses `rustTokens`, excluding literal strings, to detect direct physical trait implementations. `inspectSchemaSource` covers schema files; `walkOwners` checks a Rust root only for semantic namespaces mounting IO. Therefore a direct root `impl ArtifactPack for Snapshot` currently escapes. `misplacedWireImplementations` recognizes direct impls and `diff_text!/diff_binary!`, but not physical derives and mixed `field_set_mutations!` expansion. Generic lexical references should remain legal in assembly roots: detect implementation/declaration ownership rather than any identifier occurrence.

Add a reusable token-based physical implementation extractor returning trait/macro/derive names. Apply it to actual Rust owner-root files and schema files, and to representation files with expected owner mapping. Parse derive attribute token groups explicitly for qualified/unqualified `CanonicalJsonTree` (and proven physical DslArtifact derives). A macro definition that contains `impl ArtifactPack` is already direct implementation evidence when that file is inspected; invocations require a narrowly authored inventory of macros proven to generate physical implementations. Split `field_set_mutations!` into semantic declaration and explicit text/binary codec invocation instead of simply allowing the macro in schema. Detect root direct impls without rejecting root `pub use` assembly facades. Do not classify `ToValue`/`FromValue`, controlled semantic conversion methods, or `RetireOwned` as physical codecs.

## Meaningful Language-Neutral Test Witnesses

Extend LIB JSON fixture contracts and existing `LIB/🧪️tests/🚪️artifact-io-ownership/🟦️.ts`, which already independently parses Rust using web-tree-sitter/tree-sitter Rust WASM and writes synthetic owners under caller-owned `SEMIO_TEST_ARTIFACT_DIR`. Add source/location/expected-breach witnesses, not a hardcoded current inventory count:

1. Root direct qualified and generic ArtifactDsl/Pack impls reject; root import/reexport facade accepts; root Engine implementation containing physical encode/decode bodies rejects when the exact engine trait/method contract is inventoried.
2. Schema `#[derive(CanonicalJsonTree)]` and qualified multi-derive form reject; strings/comments containing those names accept; semantic ToValue/FromValue plus RetireOwned derives accept.
3. Schema mixed codec-emitting macro invocation rejects; semantic-only successor invocation accepts; explicit text macro accepts only text owner and binary macro only binary owner. Cover macro definitions containing impl tokens and qualified invocations.
4. A snapshot that implements only neutral pack/text builds in an independent crate without OS; an OS registration extension snapshot publishes the exact Dialect/TypeId/provider pair. Preserve absent SQLite capability and mismatching provider refusals.
5. Neutral relational projection roundtrip compares row/schema output to the existing SQLite oracle, while OS extension cancellation/refusal retains original custody and retirement. Existing Store snapshot-capability/native-retirement tests already name success/cancellation/refusal and original-input custody witnesses; move or update bounds without reducing these assertions.

For checker cases, have tree-sitter independently identify impl trait paths, derive attribute names and macro invocation paths; compare those findings and expected ownership decisions with the first-party checker. Never use the first-party extractor as its own oracle. No tests were executed in this audit.
