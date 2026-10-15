# Store Next Crate Frontier — Read-Only Snapshot

2026-10-15. No builds, Source edits, compiler retries or lock changes. Existing projection and borrowed-reference ownership cuts are excluded from this recommendation. Concurrent edits remain unqualified.

## Current Real Boundary

`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` owns `semio-framework-os-kernel`, whose lib is adjacent `🦀️.rs`. Its declared physical Source bounds are 1,048,576 bytes, 128 files and 12,000 active lines. Lib lines 230–233 mount `../../🔨️modules/🏪️store/🦀️.rs`; that Store file alone currently measures 779,959 bytes. This is a direct physical file measurement, not the full compiler Source census.

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml:74` has an unconditional Cargo edge to OS Kernel; it also depends on `semio-framework` at line 49. Kernel does not declare a Cargo dependency on the plugin crate in its inspected manifest. Instead its lib line 200 physically mounts `🔌️plugin/📇️registry/📦️deployment/🧬️schema/🦀️.rs` as `os_plugin_module_schema`. Thus a physical ownership entanglement exists, but this audit does not falsely label that a literal two-crate Cargo cycle. That deployment schema imports serde and schema-registry, not plugin/Store runtime types; it is another possible canonical schema owner, separate from the Store candidate below.

## Recommended Small Candidate: Blob Identity Schema

The pure `BlobRef` contract is still owned by Store:

- Physical Source: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️blob/🧬️schema/🦀️.rs` (1,053 bytes observed).
- Physical mount: Store lines 8975–8977, private `blob_reference_schema`, exported `BlobRef`.
- Fields: `hash: String`, `size: u64`, `media_type: String`.
- Implementation-only imports: `crate::{FromValue, ToValue}` derive aliases. Other required derive paths are `semio_framework_value::{CanonicalJsonTree, RetainedClone}`, `semio_framework_pack_json`, and serde. No kernel state, plugin, Store, VCS, operation, or UI input is present in the definition.
- Existing language facets at the same schema directory: Rust, TypeScript, JSON Schema, GraphQL and protobuf are included into Store's schema registry at Store lines 11403–11407.
- Retirement impl is currently outside the definition: `🏪️store/♻️retirement/🦀️.rs:9` applies `artifact_retire_struct!(super::BlobRef { hash, size, media_type })`. It must move with canonical ownership rather than remain an orphan external impl.

Give this existing schema directory its own small canonical crate and make Store depend inward on it. Schema consumers can then depend on that owner directly. Preserve the current serde runtime contract: the definition explicitly documents real space serialization, so changing serde to test-only would be dishonest without changing those callers first. This is ownership extraction, not permission to invent a new serialized format or runtime dependency.

Actual callers verified:

- `🏪️store/🔗️link/🧬️schema/🦀️.rs`: `LinkPin::Snapshot { blob: BlobRef }`.
- `🏪️store/🦀️.rs:388`: `LinkState::PinnedOnly { blob: BlobRef }`.
- `🏪️store/🦀️.rs:8984`: `BlobStore::put` returns `BlobRef`; leave this I/O trait with its `VcsError` in Store, not in the pure identity schema.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🦀️.rs:51` stores `ArtifactBody::Blob { blob: store::BlobRef }`. Import/SQLite producers at `…/🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` also construct this identity.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs:312` carries `Vec<(store::BlobRef, Vec<u8>)>` and line 461 accepts `&store::BlobRef` for actual blob import.

## Follow-On Link Schema (Larger, Still Pure Data)

`🏪️store/🔗️link/🧬️schema/🦀️.rs` is 1,689 bytes and defines `ArtifactLink { target: ArtifactRef, pin: LinkPin, role: String }` plus `Head`, `Checkpoint { id }`, `Snapshot { blob }` variants. This is a durable pinned-link contract, distinct from the already extracted borrowed reference/projection cuts. It can depend inward on artifact-reference, blob identity, value and pack-json without any Store or plugin edge. Its `ArtifactLink` retirement impl is in `🏪️store/♻️retirement/🦀️.rs:10`; `LinkPin` derives retirement already.

However, **do not extract only the struct and claim the codec contract is independent**: manual `DslField`/record codecs still reside in `🏪️store/🪆️child/🦀️.rs`, notably `link_pin_to_record` at 523, controlled encoding at 543, controlled decoding at 600, and artifact-link conversion at 713–774. Move the relevant impl ownership with the link schema or retain this as a separate subsequent cut. Existing concrete consumer `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:48,54` has two `Option<store::ArtifactLink>` fields.

## Independent Qualification

For the smallest BlobRef cut, create a language-neutral fixture covering empty/Unicode identity fields, `u64::MAX`, exact camelCase names and unknown-field rejection. Compare the owner crate's first-party value/canonical JSON codecs with its existing serde/serde_json oracle; exercise retained clone and bounded retirement on the same fixture. Run the owner in isolation so its dependency graph demonstrably excludes Kernel, Store and Plugin. This is a proposed test, not a passing result.

For the link follow-on, there is already a useful public law at `🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/🦀️.rs:46–55`: ordinary/controlled `DslField` round trips, three pin variants and maximum-size blob metadata. Relocate ownership of the test alongside the extracted implementation and preserve genuine control admission.

The BlobRef cut removes only a small physical slice from Store and gives metadata-only consumers a clean inward edge; it does **not** alone remove Plugin's real Kernel dependency, eliminate this consumer's full kernel closure, or establish that the 1 MiB Source frontier passes. The link follow-on offers more reusable contract ownership but requires the real codec impls, not a compatibility barrel or duplicate mount.
