# Binary Physical Store and Carrier Authority

## Actual Boundaries

The owner `binary/raw/any/snapshot/🦀️.rs` deliberately makes `ArtifactPack` encode/decode the raw byte identity. `ArtifactDsl` emits the declared stdio.binary envelope and hexadecimal body. Both canonical decoders set `schema=STDIO_BINARY_DOCUMENT_SCHEMA`; no schema field exists in raw external bytes or hex. The exact native coordinate is `s.stdio.binary@raw/*`, identical to `CARRIER_BINARY`. Its owning io declaration explicitly states other artifact deserializers accept this raw carrier; it publishes no artificial self-converter.

Framework IO `🧰️framework/🔨️modules/🚪️io/🦀️.rs` lines2522/2536 selects CARRIER_BINARY directly from binary IoPayload. Altering Binary's codec by prepending a Snapshot header would violate that actual file authority and downstream import contracts. No such change is made.

Store has an additional real persistence dependency on each owner's ArtifactPack: `parse_document_spr` reconstructs initial snapshot with P::decode_pack (store line13265); `print_document_pack` emits initial_snapshot.encode_pack (13379); initial digest construction (17179/17450), genesis (21224), and snapshot accessors (24542/24554) also use the actual pack. Its generic ArtifactPack docs assert full equality (10733), while this actual external owner permits arbitrary schema String beyond its raw representation. That is an existing representation-boundary limitation, not a reason to silently replace native raw bytes with a new private persistence codec.

## SQLite Authority

The explicit typed `io_export_sqlite_snapshot` / `io_import_sqlite_snapshot` path projects the complete literal schema and ordered byte entities directly. The new actual-declaration typed file law exercises arbitrary schema and all1024 byte entities, both metadata encoding modes, and asserts native Encode/Decode phases are absent. Erased native factory laws exercise explicitly authored canonical external states. Ordinary native schema normalization is separately asserted as the actual external projection.

## Unmounted Controlled Draft

The Binary snapshot `🚦️native/🦀️.rs` draft preserves raw identity plus the owner's authored enveloped/bare hex grammar. It has no private header and invokes no ordinary parser/encoder fallback: borrowed Text admission, validated hex scanning before allocation, exact row forecast, cumulative NativeDecodeControl, bounded256-byte copy/emission progress, exact output file ceiling and controlled envelope output. It is deliberately unmounted until Root executes the genuine eight-law missing-hook baseline. Compilation/runtime and semantic long-schema copy repairs are not claimed.

No existing carrier or Store production implementation was changed in this audit. All line references are inspected current source, not execution evidence. Source22-law SQLite evidence is retained in the separate staging report.
