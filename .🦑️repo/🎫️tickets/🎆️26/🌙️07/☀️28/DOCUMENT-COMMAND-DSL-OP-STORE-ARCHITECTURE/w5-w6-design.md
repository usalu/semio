# W5 + W6 Design — Binary Causal Envelopes/Wire (M-C) and Kernel Unification (M-E)

(Full design produced by a Plan agent against contract.md, read-only verified against the
working tree. See /Users/ueli/.claude/plans/steady-cuddling-dijkstra.md for the approved
summary; this file is the full detail.)

All paths from `/Users/ueli/Documents/semio`. Assumes W1–W4 landed: `protocol::OpBinary` +
`dsl::op_rt` (format byte `1` | variant-ordinal varint | `pack::encode_record_body` body) exist
on every `DslOps` enum; `DocumentCommand` has text/binary dispatch; history has
backwards+cursor; pack+spr are the authoritative storage artifacts.

---

## W5 — Binary causal envelopes + wire (one un-splittable sitting, 1 agent)

### W5.1 New `protocol_causal` type shapes (`protocol/causal/rs/lib.rs`, region `🔖️Envelope` :12-45)

Replace the three payload types:

```rust
/// Opaque, schema-tagged binary payload: `payload` is the M-A `dsl::op_rt` encoding of one
/// typed operation (format 1), or a producer-defined encoding named by `schema` (e.g. the
/// db pathmap JSON-bytes convention below). serde derives kept: the WIT/backbone JSON seam
/// serializes `Vec<u8>` as a JSON number array — acceptable by design (seam stays JSON).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocumentDiff {
    pub schema: protocol_core::SchemaId,   // was String
    pub payload: Vec<u8>,                  // was serde_json::Value
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InverseOperation {
    pub schema: protocol_core::SchemaId,
    pub payload: Vec<u8>,                  // field renamed from inverse_diff
}

pub struct OperationEnvelope {            // fields unchanged except diff/inverse types
    pub operation_id: protocol_core::OperationId,
    pub document_id: protocol_core::DocumentId,
    pub actor: protocol_core::ActorId,
    pub dependencies: Vec<protocol_core::OperationId>,
    pub diff: DocumentDiff,
    pub inverse: InverseOperation,
    pub timestamp: protocol_core::HybridLogicalTimestamp,
}
```

`operation_envelope_from_edit` (:265-295) — the `std::any::type_name` tag (:264-266) and
`serde_json::to_value` payloads are deleted. New signature adds the `OpBinary` bound and a real
schema id:

```rust
pub fn operation_envelope_from_edit<P, Op>(
    edit: &protocol_command::Edit<Op>,
    document_id: &protocol_core::DocumentId,
    schema: &protocol_core::SchemaId,          // caller-supplied real SchemaId — no type_name
) -> Result<Vec<OperationEnvelope>, protocol_core::ProtocolError>
where Op: protocol_command::Operation<P> + protocol_command::OpBinary
```

Body: `payload = op.encode_op()?`; `inverse.payload =
edit.backwards.get(index).map(OpBinary::encode_op).transpose()?.unwrap_or_default()` (empty
`Vec<u8>` replaces the old `Value::Null` past-end convention — document on the fn). Meta fallback
chain (:272-281) unchanged. Fixture ops in `🧪️Tests` (`CausalAddOp`, :319-331) gain a hand-written
`OpBinary` impl (1-byte format + i64 LE) so tests stay dependency-free.

### W5.2 Binary record codecs — where they live

Decision: shared **primitives** go in `protocol_core` (new region `🔖️WireCodec`), because both
`protocol_causal` (envelope records) and `protocol_wire` (frames) need them and both already dep
`protocol_core`; `protocol_wire`'s private LEB128 twins (:161-194) are deleted in favor of them.

`protocol/core/rs/lib.rs` — new `//#region 🔖️WireCodec`:

```rust
pub fn write_varint_u64(out: &mut Vec<u8>, value: u64);
pub fn read_varint_u64(bytes: &[u8], pos: &mut usize) -> Result<u64, ProtocolError>;
pub fn write_str(out: &mut Vec<u8>, s: &str);                       // varint len + utf8
pub fn read_str(bytes: &[u8], pos: &mut usize) -> Result<String, ProtocolError>;
pub fn write_bytes(out: &mut Vec<u8>, b: &[u8]);                     // varint len + raw
pub fn read_bytes(bytes: &[u8], pos: &mut usize) -> Result<Vec<u8>, ProtocolError>;
pub fn write_hash32(out: &mut Vec<u8>, h: &[u8; 32]);                // 32 raw bytes
pub fn read_hash32(bytes: &[u8], pos: &mut usize) -> Result<[u8; 32], ProtocolError>;
// Option<T>: presence u8 (0/1) then T. Vec<T>: count varint then items. bool: u8 0/1.
// u32: varint. u64: varint. (Document: all integers varint — TS numbers stay < 2^53.)
```

`protocol/causal/rs/lib.rs` — new `//#region 🔖️EnvelopeCodec`:

```rust
pub fn encode_envelope(envelope: &OperationEnvelope, out: &mut Vec<u8>);
pub fn decode_envelope(bytes: &[u8], pos: &mut usize) -> Result<OperationEnvelope, ProtocolError>;
pub fn encode_frontier(f: &FrontierSummary, out: &mut Vec<u8>);
pub fn decode_frontier(bytes: &[u8], pos: &mut usize) -> Result<FrontierSummary, ProtocolError>;
fn encode_hlc / decode_hlc;   // actor varint | physical_ms varint | logical varint
```

Envelope record layout (field declaration order, no tags): `operation_id str | document_id str |
actor str | dependencies vec<str> | diff.schema str | diff.payload bytes | inverse.schema str |
inverse.payload bytes | hlc`. FrontierSummary: `document_id str | head_edit_ordinal varint |
head_edit_id str | last_commit_seq varint | chain_hash 32`.

Rationale for hand-rolled per-frame record specs (not `pack_value::encode_record_body` with a
`RecordSpec` per frame): the TS twin must be byte-identical, and TS has no pack engine — the
record-body codec would force porting symbol tables/tag codecs to TS. Frame fields are fixed
shapes; the primitive-composed layout is the same style as `dsl::op_rt` bodies one level down.
Record this rationale in the `protocol_wire` module doc, replacing the current "permitted
deviation" note (:6-18).

### W5.3 `protocol_wire` frame bodies (`protocol/wire/rs/lib.rs`)

Type changes (frozen shapes amended by this ticket):
- `ClientFrame::Presence { peer: serde_json::Value }` → `Presence { peer_json: String }`;
  `ServerFrame::Presence { peers: Vec<serde_json::Value> }` → `Presence { peers_json: Vec<String>
  }`. Reason: opaque JSON must round-trip byte-exact through decode→re-encode (the canary law);
  `serde_json::Value` re-serialization cannot be guaranteed byte-identical against
  `JSON.stringify`. Everything else keeps its shape.

Codec (region `🔖️Codec` :153-240): delete `encode_frame`/`decode_frame` JSON bodies (:200-219)
and the private varint twins. New layout: `lane u8 | frame tag u8 | fields...` — **no body
length prefix** (frames are self-delimiting; one frame per WS message, matching both current
transports). Tags = variant declaration order, 0-based, per enum (`ClientFrame`: Hello=0 …
Bye=6; `ServerFrame`: Welcome=0 … Error=8). Nested enums: `Bootstrap` tag u8 (None=0, Snapshot=1
{hash32, Option<bytes>}, Tail=2); `ApplyOutcome` tag u8 (Accepted=0, Transformed=1 {envelope},
Rejected=2 {str}); `AckStage` tag u8 (Received=0, Persisted=1, Applied=2 {outcome}).

Public signatures unchanged: `encode_client_frame(&ClientFrame, Lane) -> Vec<u8>`,
`decode_client_frame(&[u8]) -> Result<(Lane, ClientFrame), ProtocolError>`, and the server pair.
All existing round-trip tests (:242-467) keep passing with bodies flipped; add decode-reject
tests for unknown frame tag / truncated field.

`protocol/wire/rs/Cargo.toml`: no new deps needed (`protocol_core` + `protocol_causal` already
present per imports); drop the "no pack_core dep" varint duplication note.

### W5.4 TS twin (`framework/product/os/core/js/index.ts` :320-450)

- `WireOperationEnvelope`'s `diff`/`inverse` become `{ schema: string; payload: readonly
  number[] }` (rename `inverse_diff` → `payload`).
- Rewrite `encodeWireFrame`/`decodeWireFrame` (:406-428): implement the primitive set
  (varintU64, str, bytes, hash32, option, vec, tag bytes) and the per-frame field specs
  byte-for-byte per W5.2/W5.3. Delete the `JSON.stringify` body. `Presence` frames carry
  `peer_json: string` / `peers_json: string[]`.
- Keep `encodeClientFrame`/`decodeClientFrame`/`encodeServerFrame`/`decodeServerFrame`
  signatures (:432-450).
- `backbone-worker.ts` `🔖️WireBridge` (:121-214): `toWireEnvelope` encodes its local JSON edit
  payload as UTF-8 JSON bytes (`new TextEncoder().encode(JSON.stringify(payload))`) tagged with
  the existing schema string; `fromWireEnvelope` decodes with `JSON.parse(new
  TextDecoder().decode(...))`. (Both functions are deleted outright in W6 — this is the one
  permitted single-wave interior shim, contained inside W5's sitting only because W5 and W6 are
  separate waves.)
- Hub handler (:369-418): `Presence` branch parses `frame.Presence.peers_json.map(JSON.parse)`.

### W5.5 `store_sync` WireBridge + both actors (`store/sync/rs/lib.rs`)

- `to_wire_envelope` (:222-232): `payload: serde_json::to_vec(&envelope.diff.payload)`,
  `schema: SchemaId(envelope.diff.schema_id.0.clone())`; same for inverse.
- `from_wire_envelope` (:240-260): `serde_json::from_slice(&envelope.diff.payload)` (malformed →
  skip envelope with a logged conflict event, not panic); `sequenceNumber`/`payload_hash`
  recovery unchanged.
- Native actor: Hello at :911-923; wasm actor twin Hello at :1251, relay at :1283, receive at
  :1344, ack at :1369 — all compile against the new frame types unchanged except Presence
  (`presence_to_json`/`presence_from_json` :284-291 flip to String).
- Hub `Hello.pack_schema_hash` — see W5.7.

### W5.6 db family + hubs + testkit consumer flips

- `db/document/rs/lib.rs` (`🔖️Diff` :110-151, `🔖️Bridge` :154-191): the path-value convention
  keeps existing **inside the opaque payload**. New region constants + codec:
  ```rust
  pub const DB_PATHMAP_SCHEMA: &str = "db.pathmap.v1";
  pub fn encode_pathmap(value: &serde_json::Value) -> Vec<u8>;   // serde_json::to_vec (Value maps are BTreeMap-sorted → deterministic)
  pub fn decode_pathmap(bytes: &[u8]) -> Result<serde_json::Value, DbError>;
  ```
  `entries_from_value` callers (`diff_entries` :122, `inverse_entries` :127) decode first and
  check `schema == DB_PATHMAP_SCHEMA` (foreign schema → the envelope is persisted/relayed but
  produces an empty `TouchedSet`, documented: db never interprets typed op payloads — that is
  `db_document`-and-above's future typed path). `envelope_from_operation` (:161-190) tags
  `SchemaId(DB_PATHMAP_SCHEMA)` and encodes with `encode_pathmap`; `entries_to_value` consumers
  in `undo`/compensating-envelope construction (:765, :896) re-encode. `command_touch`
  (:215-224) uses `envelope.diff.schema.0.as_str()`.
- `db/sync/rs/lib.rs` `🔖️Codec` (:29-51): `encode_command_envelope` =
  `protocol_causal::encode_envelope` (binary record, not JSON); `decode_command_envelope` =
  length-check then `decode_envelope`. WAL command bytes become the binary envelope record —
  this is the "storage" half of M-C for the hub.
- Test fixture builders constructing envelopes with `json!` payloads flip to
  `encode_pathmap(json!({...}))`: `db/sync/rs/lib.rs:359-369`, `db/rs/lib.rs:200-…`,
  `db/engine/rs/lib.rs:928-…`, `db/document` tests, `protocol/testkit/rs/lib.rs:238-260`
  (`generate`) and `:526` (`assert_op_dag_convergence` — only ids matter; use fixed byte
  payloads `vec![seed as u8]`).
- Hubs: `framework/product/os/hub/rs/bin.rs` (`submit_commands` :370-394, `handle_ws` :444-509)
  and `compose/server/hub/rs/bin.rs` (:2735-2745) treat envelopes opaquely — compile-only
  changes plus W5.7 validation.

### W5.7 Hub schema-hash validation flow

1. `store/rs/lib.rs` `DocumentCodec` (:254-267) gains `pub pack_schema_hash: [u8; 32]`, computed
   in `DocumentCodec::of` (:273-315) as `pack::schema_hash(&P::record_spec())`
   (`pack_value::schema_hash` exists at `pack/value/rs/lib.rs:1183`; add the `pack` facade
   forward if missing).
2. Rust actor: in `try_connect_hub` (:900-925) and the wasm twin (:1251), replace `[0u8; 32]`
   with `store::document_codec(&self.schema).map(|c| c.pack_schema_hash).unwrap_or([0u8; 32])`
   (zeros = schema-agnostic client; documented).
3. TS worker: `DocumentActorConfig` (Rust :53-64 + TS `index.ts:456-462`) gains
   `pack_schema_hash: [u8; 32]` / `packSchemaHash?: readonly number[]`; `backbone-worker.ts`
   `connectHub` (:288-302) sends it instead of `new Array(32).fill(0)`. The shell fills it from
   the wasm renderer (which links `store` and the codec registry) — a small `#[wasm_bindgen] pub
   fn document_pack_schema_hash(schema: &str) -> Option<Vec<u8>>` export beside the existing
   renderer exports.
4. Server: `framework/product/os/hub/rs/bin.rs` `handle_ws` (:451) destructures `schema` and
   `pack_schema_hash` from Hello. New `HubState.schema_hashes:
   dashmap::DashMap<String, [u8; 32]>` keyed by `scope_key(studio, document)`: first non-zero
   hash pins; later non-zero mismatch → `error_frame("schema-hash-mismatch", …)` + return before
   Welcome. Zeros skip validation (schema-agnostic clients). Design note in-code: durable pinning
   belongs in the db catalog once it grows a column — in-memory pin is the wave's scope. Mirror
   the same check in `compose/server/hub/rs/bin.rs` Hello arm (:2736).

### W5.8 Fixture regeneration BY CODE + full list

Generator stays the Rust test `wire_fixtures_stay_byte_identical_across_rust_and_ts`
(`store/sync/rs/lib.rs:1924-1974`) — it already deterministically writes
`store/sync/fixtures/wire/*.bin` on every run (fixed constants, no clock/random). Extend it to
cover every variant; verification is the vitest canary
(`framework/product/os/core/js/backbone-worker.ts:727-760`), which decodes each fixture, asserts
fields, re-encodes, and byte-compares. Procedure: `cargo test -p store_sync wire_fixtures`
(writes), then `bun vitest` in os/core js (verifies). Never hand-edit.

Full fixture list (`store/sync/fixtures/wire/`): `client-hello.bin`, `client-commands.bin` (1
envelope, byte payload fixture `dsl::op_rt`-shaped constant), `client-frontier-advertise.bin`,
`client-preview-publish.bin`, `client-presence.bin`, `client-credit-grant.bin`, `client-bye.bin`,
`server-welcome-tail.bin`, `server-welcome-snapshot-inline.bin`, `server-snapshot-chunk.bin`,
`server-snapshot-done.bin`, `server-commands.bin`, `server-ack-accepted.bin`,
`server-ack-transformed.bin`, `server-ack-rejected.bin`, `server-preview.bin`,
`server-presence.bin`, `server-credit-grant.bin`, `server-error.bin`. Delete the old 4 names
(`client-hello/client-commands/server-welcome/server-ack`) or reuse where names coincide; update
the vitest fixture path loop and `store/sync/fixtures/README.md`.

### W5.9 WIT/JSON ↔ binary boundary points (JSON stays by design)

1. Plugin WIT ABI (`framework/plugin/host/rs/lib.rs` calls; jco component bridge in
   `framework/product/os/dev/script.ts:642-789`): invocation/result/context JSON strings —
   unchanged.
2. `PortBackbone` relay across the sandbox (`host-shim.js` `backbonePoll`/`backboneSend`,
   `script.ts:568-577,849-…`): `BackboneMessage` JSON; envelope byte payloads ride as JSON number
   arrays — unchanged encoding, only the payload field type changes (W6).
3. `backbone-worker.ts` ⇄️ main thread `postMessage` (`BackboneWorkerRequest/Response`,
   `index.ts:502-506`): structured-clone JSON — unchanged.
4. Folder dev middleware `/semio-backbone` HTTP (`script.ts:479+`, `backbone-worker.ts:216-275`):
   stays JSON envelope until W4's pack+spr flip covers it; note in report which side landed
   first.
5. Hub WS frames: **binary** (this wave). 6. db WAL command payloads: **binary** (this wave).
7. `.spr`/`.pack` storage: binary (W4).

### W5.10 Ordered edit sequence (single sitting)

1. `protocol/core/rs/lib.rs` — `🔖️WireCodec` primitives + tests.
2. `protocol/causal/rs/lib.rs` — type reshape, `operation_envelope_from_edit` rewrite,
   `🔖️EnvelopeCodec`, tests.
3. `protocol/wire/rs/lib.rs` — Presence type change, frame codec rewrite, module-doc rationale,
   tests.
4. `protocol/rs/lib.rs` — re-export additions (`encode_envelope`, `decode_envelope`,
   `encode_frontier`, `decode_frontier`; :23-28).
5. `db/document`, `db/sync`, `db/rs`, `db/engine`, `protocol/testkit` — pathmap codec + fixture
   flips.
6. Hubs (`framework/product/os/hub/rs/bin.rs`, `compose/server/hub/rs/bin.rs`) — compile flips +
   schema-hash validation.
7. `store/rs/lib.rs` — `DocumentCodec.pack_schema_hash`.
8. `store/sync/rs/lib.rs` — WireBridge byte shim, both actors' Hello hash, fixture generator
   extension.
9. `framework/product/os/core/js/index.ts` + `backbone-worker.ts` — TS twin + config hash +
   canary extension; renderer wasm hash export.
10. Verify: `cargo test -p protocol_core -p protocol_causal -p protocol_wire -p protocol -p
    db_sync -p db_document -p db_engine -p db -p protocol_testkit -p store -p store_sync`;
    regenerate fixtures; os/core vitest; `cargo check --workspace --all-targets`.

---

## W6 — Kernel unification (3 agents: core / os / bridge)

Ordering: **core agent first** (store + framework/core + store_sync are upstream of both
others), then os and bridge in parallel.

### W6-core — delete framework/core twins + store repoint + WireBridge deletion

Deletion list (`framework/core/rs/lib.rs`, exact regions):
- `🔖️Sync` region :6244-6578 entire: `PayloadHash` (:6245-6247), `OperationEnvelope`
  (:6249-6261), `OpDagError`/`OpDag`/`InsertResult` (:6263-6385), `🔖️HubProtocol` (:6387-6490,
  `HubClientFrame`/`HubServerFrame` — zero consumers outside the :8005 re-export;
  `PresencePoint`/`PresenceViewport`/`PresencePeer` :6388-6431 are still consumed by
  store_sync/hubs → **move** them up into the kernel region, don't delete).
- `op_dag_tests` (:6492-6577).
- `🔖️HybridLogicalTimestamp` local struct (:5832-5883) → `pub use
  protocol_core::HybridLogicalTimestamp;` (JSON shape change `physicalMs`→`physical_ms` is this
  wave's sanctioned wire-format reconciliation — the deferral note at :5833-5842 says exactly
  this).
- Kernel `DocumentDiff` (:6121-6126) and `UndoPolicy` (:6128-6138) → `pub use
  protocol_core::UndoPolicy;` and repoint `DocumentDiff` uses to `protocol::DocumentDiff`
  (schema `SchemaId` + `payload: Vec<u8>`). Kernel `InverseOperation` (:6140-6149),
  `KernelOperation` (:6151-6164), `UndoGroup` keep their shells with re-pointed field types.
- Root re-export line :8005 — drop `HubClientFrame, HubServerFrame`, keep names resolving via the
  new `pub use`s.
- Cargo: `framework/core` gains a `protocol` facade dep (it already deps `protocol_core`,
  :5803).

Consumer-flip list for the twins:
- `store/rs/lib.rs` (largest): import line :18-20 flips to `protocol::{…}`.
  `OperationEnvelope` construction `operation_envelope_from_edit` (:1869-1913) is **deleted** —
  replaced by `protocol_causal::operation_envelope_from_edit` (one envelope per op, W5
  signature) called from the outbound path (:1808-1833); undo-policy dispatch (:1306-1350)
  reads `Edit.operation_meta[i].undo_policy` (`protocol::UndoPolicy`) instead of
  `envelope.inverse.undo_policy`; causal ingest (:1674-1730) flips to `protocol::OpDag` API
  (`operation_id`/`dependencies` field names, `seed_applied(OperationId)` single-id, `ready() ->
  Vec<OperationId>`); `edit_from_operation_envelope` (:1916+) decodes ops from payload bytes via
  `OpBinary::decode_op` instead of `serde_json::from_value`. `BackboneMessage::Operations`
  (:1959-1961) carries `protocol::OperationEnvelope`.
- `store/sync/rs/lib.rs`: delete `🔖️WireBridge` region :211-322 **whole** (`to_wire_envelope`,
  `from_wire_envelope`, `rollback_envelope` is rebuilt — see below, `presence_to_json/from_json`
  move beside the Presence handling, `now_ms`/`actor_seed`/`next_timestamp` move into the actor
  region); delete `🧪️WireBridge` tests :1888-1913; `operation_envelope_from_stored_edit`
  (:177-199) deleted with the W4 storage flip's JSON-envelope folder path (its caller :881).
  Actors speak `protocol::OperationEnvelope` natively end to end. `rollback_envelope`
  (:269-281): re-emit `envelope.inverse` as a forward diff — new inverse-of-inverse is the
  original diff; ids/deps logic unchanged, types flipped.
- `framework/plugin/rs/lib.rs`: import list :193-195; the `UndoPolicy`/`HybridLogicalTimestamp`
  match-bridges (:3632-3650) collapse to direct field moves (same type now);
  `KernelOperation.diff` construction (:3651-3670) encodes ops with `OpBinary::encode_op` and a
  real `SchemaId` (the app's document schema string).
- `framework/product/os/core/rs/lib.rs`, `framework/product/os/hub/rs/bin.rs`,
  `framework/renderer/wgpu/rs/lib.rs`: compile-flip imports (`semio_framework_core::CommandScope`
  etc. untouched).
- TS mirrors (`framework/product/os/core/js/index.ts` local `OperationEnvelope`/`UndoPolicy`/
  kernel types, ~lines 200-320): delete the local camelCase envelope type;
  `DocumentActorMsg`/`DocumentEvent`/`backbone-worker.ts` use `WireOperationEnvelope` directly
  (delete `toWireEnvelope`/`fromWireEnvelope`/`placeholderPayloadHash`, :121-214, and their
  tests :691-713).

Ordered sequence: (1) framework/core deletions+re-exports → (2) store repoint → (3) store_sync
WireBridge deletion → (4) framework/plugin → (5) os/core + hub + renderer compile flips → (6) TS
twin cleanup → (7) `cargo check --workspace --all-targets`, `cargo test -p store -p store_sync
-p semio-framework-core -p semio-framework-plugin`, os/core vitest.

### W6-os — delete the OS JSON-patch kernel + DSL fixtures

Deletion list (`framework/product/os/core/rs/lib.rs`):
- `commit_action_result` (:188-201), `invoke_action` (:202-230), `JSON_PATCH_SCHEMA_ID` (:319),
  `extract_patch_operations` (:366), `kernel_operation_from_patch_operation` (:370-…),
  `apply_kernel_patch_operation` (:387-…), tests
  `invoke_action_applies_patch_ops_and_returns_kernel_operations` (:1658-1740).
- Caller enumeration (verified repo-wide grep): **no external callers exist** — only os/core's
  own test. `OsInstanceState.document_json` mutation via patches has no replacement need; every
  OS mutation already flows through `OsStore` (`DocumentStore<OsProjection, OsOperation>`,
  :1077-1135: `dispatch_apply`, `spawn_app_instance`, W3's `dispatch_text/binary` replacing
  `dispatch_json` :1113). Deleting is a pure removal; document in the report that
  `create_instance`'s `document_json` seed (:172-178) remains as the render-input snapshot, now
  only ever produced by store prints.
- Fixtures (G18): delete `register_os_fixture_json`/`os_fixture_json` (:2665-2682) — the only
  registrars are `s/plugin/rs/lib.rs:27-28` which feed DSL text into a JSON registry (silently
  falls back to `json!({})`, comment :22-26). Replace with:
  ```rust
  pub fn register_os_fixture_dsl(slug: &str, schema: &str, dsl: &str);
  pub fn os_fixture_envelope_json(slug: &str) -> Option<String>;  // lazily: store::document_codec(schema)?.parse_dsl(dsl, "")
  ```
  Flip `s/plugin/rs/lib.rs:20-31` to `register_os_fixture_dsl("semio.draw", "draw.document",
  include_str!(…))` / `("jack.writer", "writer.document", …)` and remove the `.json` slugs.
  Update re-export list :5138-5139 and the fixture tests :2745-2760 (register DSL, assert
  codec-parsed envelope JSON).

### W6-bridge — wire `handle-command` on the JS plugin bridge

Existing halves: wasmtime `WasmPluginRuntime::handle_command`
(`framework/plugin/host/rs/lib.rs:418-433`); generated worker + component bridge already route
`handleCommand` (`framework/product/os/dev/script.ts:604-608` in `pluginWorkerSource`,
:699-707 + :767-768 in `pluginComponentBridgeSource` — the template generator is these two
functions, NOT the generated files under `framework/product/os/dev/plugin-modules/*/`);
main-thread client already exposes it (`framework/renderer/wgpu/js/boot.js:406-407,808-810,861,875`,
`boot.ts:24,168`).

Missing piece (the :21928 gap) — `framework/renderer/wgpu/rs/lib.rs` `plugin_bridge` module
(:7472-7770):
1. Add `PluginBridgeEntry::handle_command(&self, instance_id: u32, command_json: &str,
   view_state: &ViewState) -> Result<kernel::InvocationResult, String>` mirroring `handle_action`
   (:7587-7599): wasm arm → new `handle_command_js` (clone of `handle_action_js` :7697-7744
   calling the JS `handleCommand` property — no silent-empty fallback: missing property is an
   `Err`, greenfield); native arm → `runtime.handle_command(...)`.
2. Shell dispatch: in `command_search_items` (:21932-21981) stop skipping arg-carrying
   Plugin/App/Mode commands — emit the staged-form redirect (`dispatch_action` with the
   command's `ActionDescriptor`); route execution through a new `dispatch_scoped_command` on the
   session that builds a `CommandInvocation` JSON (`semio_framework_core::kernel::
   CommandInvocation`, framework/core :5971-5979) and calls `PluginBridgeEntry::handle_command`,
   feeding the returned `InvocationResult` through the same effects/events/ui_scope application
   pass the `handle_action` call sites use (:18154, :18528). Update the stale comment at
   :21928-21931.
3. Ordered sequence: renderer `handle_command_js` + entry method → shell dispatch path → remove
   skip + comment → `cargo check -p semio-framework-renderer-wgpu --target wasm32-unknown-unknown`
   + native, then runtime proof via launch.json os/dev with temporary `[DEBUG]` logs (removed
   after) executing one arg-carrying App-scope command end-to-end in the browser.

Cross-wave laws to cite in reports: laws 2, 5, 6 (op codec round-trip; live==replay; Rust/TS
byte identity). No git mutations; single-file lib.rs regions; scratch as `.txt` in the ticket
folder; AGENTS.md changes → `human-todos.txt`.
