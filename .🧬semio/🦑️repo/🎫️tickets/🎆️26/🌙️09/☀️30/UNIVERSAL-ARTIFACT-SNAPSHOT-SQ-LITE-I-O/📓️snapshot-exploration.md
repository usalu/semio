# Universal Artifact Snapshot SQLite I/O Exploration

Read-only exploration completed on 2026-09-30. Applicable instructions read: root `AGENTS.md`, `🧰️framework/🛍️products/AGENTS.md`, and `🧰️framework/🛍️products/💻️os/AGENTS.md`. No code modified and no tests run.

## Shared Snapshot Foundation

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` defines:

- `ArtifactDsl` at line 5618: canonical text print/parse fixpoint, artifact envelope identity.
- `ArtifactPack` at line 10510: canonical binary encode/decode; derived DSL and pack share `RecordSpec` and `RecordValue`, with stable field IDs.
- `ArtifactPack::record_spec` at line 10531: optional structural schema, populated by derive-based artifacts and absent for synthetic/schema-erased implementations.
- `ArtifactCodec` at line 10627: erased schema identity, extension, structural schema hash, `compile_dsl`, `print_mirror`, and mutation/replay bridges.
- `ArtifactCodec::of` at line 10878: shared generic codec construction seam.
- Document codec registration at lines 11128–11141 and registry lookup at 11154.
- Artifact test snapshot bounds at line 25932 require `ArtifactDsl + ArtifactPack + ToValue + FromValue`.

`rg --files` finds 121 files ending `/🧬️schema/📸️snapshot/🦀️.rs`. This is a path count, not proof that every artifact uses that taxonomy. Generic snapshot transport should therefore use shared native representations rather than maintaining 121 separate encoders.

## Framework I/O

`🧰️framework/🔨️modules/🚪️io/🦀️.rs` has both an older composer registry and the newer `io_mechanism` region. The newer region defines:

- `Serializer<S>` at 2002 and `Deserializer<S>` at 2016, each with dialect and fidelity constants.
- `IoEntry` at 2046, keyed by source/target dialect and storing erased synchronous `run` and sniff function pointers.
- Route resolution at 2234, execution at 2254, identification and registration nearby.
- `serializer_entry` at 2314 and `deserializer_entry` at 2355 bridge typed snapshots through native pack payloads.
- Text-native equivalents at 2332 and 2371 bridge through DSL text.
- Constructors use `resolve_ready` to synchronously drive serializer/deserializer futures. Existing codec bodies are assumed suspension-free; expensive operations need explicit consideration of progress, cancellation, and cooperative execution.

`🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs` supplies canonical `Dialect`, `ArtifactDialect`, `IoFidelity`, and carrier identities. Binary carrier is `s.stdio.binary@raw/*`, text carrier is `s.stdio.txt@utf-8/*`. Targeted searches found no SQLite snapshot dialect or codec in framework I/O or the stdio plugin.

## Automatic Coverage Through Plugin Assembly

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` defines:

- `NativeCodecs` near 38565: snapshot/diff/mutation/inference language pairs and the erased `ArtifactCodec`.
- `IoDeclaration` near 38584: native codecs and static `IoEntry` rows.
- `preflight_io_entries` at 38864.
- `commit_artifact_declarations` near 38920: visits every artifact/standard/subset, collects native codecs and I/O rows, registers codecs and format descriptors, then invokes `io_register` at 38957.

This assembly path can provide universal SQLite routes centrally for every declared native dialect. Any central route should preserve dialect identity and reject importing a database belonging to a different artifact/standard/subset. Transporting canonical native pack bytes avoids requiring external library types or schema-specific serializers in public APIs; whether relational field projection is additionally required is a design decision for the main coordinator.

## Existing SQLite and Database Snapshots

`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs` implements native-only SQLite storage using `rusqlite` internally, behind owned repo database I/O types. Its tables include `snapshot_generation(document, generation, bytes)`, WAL, payload, catalog, index, lease, and staging. Snapshot generation bytes are pack blobs. It does not expose universal snapshot import/export through framework I/O and is disabled on wasm.

`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/🦀️.rs` defines `SnapshotDescriptor` near 73 and pack-file snapshot generations. This is database durability state: frontier, protocol version, roots, new pages, and generation chains. It is distinct from plugin artifact snapshot values and should not be mistaken for universal artifact exchange.

`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs` is another SQLite implementation found by targeted searches; no detailed audit performed because it is directory infrastructure rather than artifact snapshot exchange.

## Verification Limits

Findings are from source inspection and targeted `rg` searches. No runtime behavior was claimed or tested. Line references reflect the inspected working tree and may move while other agents work concurrently.
