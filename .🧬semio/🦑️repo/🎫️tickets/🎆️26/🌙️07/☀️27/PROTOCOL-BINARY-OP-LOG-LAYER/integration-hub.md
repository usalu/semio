# Wave 2b — Hub binary wire

Single agent, critical section. Prerequisite: Wave 2 merged, full workspace builds green,
`framework/wit/world.wit` already has `document-binary-files`.

Read first: `/Users/ueli/Documents/semio/vcs/rs/lib.rs` region `🔖️Backbone` (~2192+ —
`BackboneMessage`, `trait Backbone`, `PortBackbone`, `MemoryBackbone::pair`, `ChannelBackbone`/
`ChannelBackboneRemote`, `FolderSqliteStorage`, `FolderTextStorage`, `resolve_backbone`),
`/Users/ueli/Documents/semio/framework/core/rs/lib.rs` region `🔖️HubProtocol` (~6382+ —
`HubClientFrame`/`HubServerFrame`/`PresencePeer`) and `🔖️Sync` (~6239+ — `OperationEnvelope`,
`OpDag`), `/Users/ueli/Documents/semio/framework/product/os/hub/rs/bin.rs` (hub server, storage
backends), `/Users/ueli/Documents/semio/framework/sync/rs/lib.rs` (`DocumentActor`,
`PersistenceBinding::Hub`, `SyncSession`), `/Users/ueli/Documents/semio/framework/wit/world.wit`
(`backbone-send`/`backbone-poll` in the `host` interface — currently `string` message-json).

## What to build

1. **`protocol::extract_range`/`verify_slice`/`content_frontier`** already exist (Wave 0) — this
   wave is pure plumbing on top of them, not new format work.

2. **`SliceMeta`** (new type, put it in `vcs/rs/lib.rs`'s `🔖️Backbone` region since it's the
   metadata the hub needs WITHOUT decoding op text):
   ```rust
   pub struct SliceMeta { pub edit_ids: Vec<String>, pub deps: Vec<String>, pub base_version: u64,
       pub document_id: String, pub chain_hash: [u8; 32] }
   ```
   Extracted once, at slice-creation time, from the already-decoded `HistoryEdit`s the sender has
   in memory (or via a lightweight `protocol_history::HistoryReader` scan that reads only header
   fields, not full op text) — never by decoding the shipped slice bytes on the hub side.

3. **`BackboneMessage`** (vcs `🔖️Backbone`): replace `Operations { envelopes: Vec<OperationEnvelope>
   }` (JSON) with a binary variant, e.g. `OperationsBinary { slice: Vec<u8>, meta: SliceMeta }`
   (the `slice` bytes are exactly what `protocol::extract_range` returns — a valid, self-contained
   `.spr` record stream). Replace `Snapshot { envelope_json }` with `Snapshot { pack: Vec<u8>,
   protocol: Vec<u8> }`. `Ack { op_ids }` is unchanged (op ids are still plain strings, cheap in
   JSON — leave control-plane messages as JSON, this is a deliberate scope boundary, don't
   binary-encode Ack/Hello/Welcome-control-metadata/Presence/Bye). Update every `Backbone`
   implementor (`PortBackbone`, `MemoryBackbone`, `ChannelBackbone`/`ChannelBackboneRemote`,
   `FolderSqliteStorage`, `FolderTextStorage`) and `resolve_backbone` accordingly.

4. **`framework/core/rs` 🔖️HubProtocol**: `HubServerFrame::{Welcome{backlog}, Operations}` and
   `HubClientFrame::{PutEnvelope}` carry the new binary payload shape (backlog becomes a Vec of
   `(slice bytes, SliceMeta)` pairs, or one concatenated multi-range slice if
   `protocol::extract_range` supports non-contiguous ranges — check; if it only does contiguous
   ranges, ship one slice per contiguous run and let the client `ingest_slice` them in order).
   `Hello { since_version }` stays a plain control frame (JSON or a tiny fixed binary header — your
   choice, keep it simple, it carries no bulk data). Document in a doc-comment that WS transport
   now sends **binary frames** for `Operations`/`Welcome.backlog`/`PutEnvelope` and text/JSON
   frames for everything else — this is a real protocol-framing decision the hub server and every
   client (including any TS/JS client) must agree on; grep for hub WS client code
   (`framework/product/os/core/js`, `framework/product/os/dev`) and update it to send/receive
   `ArrayBuffer`/`Blob` for the binary frame kinds.

5. **`HubStorage` backends** (`framework/product/os/hub/rs` — sqlite/postgres/neo4j, find the
   exact backend files): schema changes from storing envelope JSON to storing `(document_id,
   slice_bytes BLOB, edit_ids, deps, base_version, chain_hash)` — i.e. `SliceMeta` becomes
   indexed/queryable columns, `slice_bytes` is opaque. OpDag insertion and version-gating logic
   read only `SliceMeta`, never decode `slice_bytes`. No migration script (dev-disposable DBs,
   greenfield).

6. **WIT `backbone-send`/`backbone-poll`**: change from `string` message-json to `list<u8>`
   messages (the wasm sandbox boundary — `PortBackbone` in the plugin SDK queues bytes instead of
   JSON strings now). No dual surface. Update `framework/plugin/host/rs` and `framework/plugin/rs`
   call sites.

7. **`framework/sync` `DocumentActor`/`SyncSession`**: internal message plumbing moves from
   `envelope_json: String` to bytes where it touches `BackboneMessage`; external actor message
   types (`DocumentActorMsg`) can keep their existing shape if they operate above this layer —
   only change what's forced by the `BackboneMessage`/WIT signature changes.

## What NOT to touch this wave

Any app crate. `vcs`'s document codec (`DocumentCodec`, `print_document_binary`/
`parse_document_binary`) — unchanged, this wave is purely about the sync wire, not at-rest storage.

## Verification

`cargo test -p vcs -p semio-framework-sync -p semio-framework-core` passes, including any existing
`MemoryBackbone::pair` convergence tests (now exercising binary slices). Add/extend a test proving
two replicas converge over binary `OperationsBinary` messages: apply edits on replica A, ship via
`extract_range` + `SliceMeta`, `ingest_slice` on replica B, assert projections match. If a hub
integration test harness exists (check `framework/product/os/hub`), run it; note if it requires
infra (postgres/neo4j) you can't start — report that rather than skipping silently.

## Report back

Files touched, the exact new wire message shapes, confirmation of convergence test results, and
any TS/JS client code you updated for binary WS frames (or flag as human todo if you found client
code you're not confident editing correctly, e.g. something requiring browser-specific WS binary
handling you can't verify without a browser).
