# Next Replication Owner Independent Plan

Read-only exploration, 2026-10-15. No production edits, package creation, gates, Originals or compilers. Root reports lower error tests now use canonical Trace HeapWitness; no copied allocator test ownership or cap increase is proposed here.

## Smallest coherent cut: Pack codec identity

The best immediate nominal cut is the **whole** `🧰️framework/🔨️modules/📡️replication/⚙️codec/🆔️ids/🦀️.rs` owner: ContentHash, ChunkId, SegmentKind, CodecId, ByteRange, all exact segment constants and complete Display/Debug traits, together with its two original adjacent Native laws. This is distinct from replication `🆔️ids`, whose live Serde fan-out makes that apparently small move larger.

These are actual runtime nominal types, not test helpers or a facade module. The file calls itself Pack identity and actual Pack facade already reexports it through `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/🦀️.rs` 23/50. A domain-neutral lower Pack identity package is consequently a cleaner owner than a second duplicate Replication identifier type. Choose one canonical domain owner and reexport it explicitly from Pack/Replication codec facades; remove the parent's Source mount. Do not source-mount the same definitions into two crates or create conversion wrappers between duplicate nominal types.

## Exact dependency seam

Today Replication parent glue `📡️replication/📦️packages/🦀️rust/🦀️.rs` 18–21 Source-mounts codec ids, while `⚙️codec/🦀️.rs` 3 consumes ByteRange/ChunkId/CodecId/ContentHash. Format public methods (`📐️format/🦀️.rs` 377/521) expose CodecId. Retarget those actual imports to the lower crate and have the existing named codec ids facade explicitly compose that same owner. Pack and OS Pack facade (`🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs` 122) may continue intentional explicit composition but must declare their real dependency path where they consume it.

Genuine direct downstream clients exist: Server storage `🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs` 39 stores ContentHash; gateway `📡️gateway/🦀️.rs` 2102–2112 parses and constructs it; Server closed-ports and instance tests use it. Retarget those actual imports/Cargo declarations to the lower nominal package instead of leaving every client dependent on the large parent solely for identity. Storage still separately consumes crypto RecordHasher and format Blake3Hasher (40–41), so do not claim its complete Replication dependency disappears.

No production external runtime dependency is necessary for this identity owner: existing Source uses std only. There are no existing serde derives or Value conversion/retirement trait implementations on these five codec types in the scoped search. Preserve that contract; do not invent runtime traits solely to make the extraction look larger. If paid RetireOwned leaf support is required by a concrete existing client, its implementation must live with the nominal type in this owner (or the trait's owner), because the former parent cannot legally implement an external trait for the moved external type. Never leave orphaned retirement impls in parent glue. No such existing codec-id retirement impl was found in the search.

## Source and test boundary

Read-only raw filesystem totals: codec ids production 2334 bytes/1 file; including its two-law test file 3243 bytes/2 files. These are raw counts, **not** the repository's authoritative cfg-aware Source cap proof. Production is structurally a single leaf with no mounted runtime closure. Keep an explicit appropriately small unchanged-policy cap and let the genuine compiled-library Source gate account for package glue, tests and actual dev macro closure; do not infer Native cap green from these raw numbers.

Move both original tests (`content_hash_display_is_lowercase_hex`, `segment_kind_constants_match_contract`) into the actual lower package cfg(test) owner. They currently use async test macros despite synchronous bodies; retain exact assertions or convert to ordinary std tests while preserving the named laws, avoiding unnecessary dev dependency. Add one shared portable corpus covering all 32 hash bytes, lowercase formatting/debug representation and every exact kind constant, with an independent system hex/SQLite oracle and the actual Rust implementation consuming that same corpus. No crypto algorithm correctness is implied: ContentHash Display formats bytes and does not compute BLAKE3.

Before implementation: schema-first owner/dependency/cap law must genuinely fail for missing package/retargets. Then real owned workspace lock/fetch and Native both modes must prove nominal equality across lower→codec→Pack→Server paths and format signatures, all original tests, and unchanged parent facade behavior. Do not launch those while headroom is blocked.

## Why not the other nearby candidates first

Replication `🆔️ids/🦀️.rs` is only 12017 raw bytes, but MutationId, ActorId, ArtifactId and SchemaId still derive Serialize/Deserialize (12–39), with documented kernel/manifest fan-out. Its ToValue/FromValue and HLC semantics must move together; parent retirement currently implements HLC as a leaf (`🧬️retirement/🦀️.rs` 36), creating an orphan-rule move requirement. A lower identifier package retaining Serde would introduce an external runtime dependency in the new owner; removing derives without retargeting all genuine consumer serialization obligations is not an honest cut. Treat it as the next larger coordinated phase, not a quick package move.

Dictionary has real owned retained decoding and two runtime files (15112 raw production,24660 with tests), but retained decode depends on codec RetainedWireByteSource, OriginalWireSourceIdentity and RetainedWireUtf8Validation at `📖️dictionary/🛬️decode/🦀️.rs` 3. Extracting dictionary alone would point upward to the still-large Replication codec package or require a preceding real byte-source ownership cut; do not abstract away its physical source/cancellation/receipt contract.

Conflict's full nominal owner is 32232 raw production across three files and 52410 including tests; it refers to MutationEnvelope, IDs, MutationMessage, MergePolicy and explicit retirement/report state. It is a larger SCC, not the smallest lower nominal leaf. Scalar functions likewise take existing ByteReader/ByteWriter directly and are not a self-contained nominal owner.

## Recommendation

Take the complete Pack codec identity leaf next, with actual direct clients and explicit lower composition. Keep replication actor/document/schema identifier Serde obligations and dictionary byte-source SCC as named later frontiers. This is a concrete plan only; no implementation or Native success is claimed.
