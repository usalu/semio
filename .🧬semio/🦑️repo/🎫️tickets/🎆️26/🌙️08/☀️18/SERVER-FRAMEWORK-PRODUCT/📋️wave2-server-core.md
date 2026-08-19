# Wave 2 — server core

## B3 decision (taken, with the evidence that forced it)

The approved path was "extract db's os deps, then move db". Two of three extractions landed
(pack container B1, `DslValue` B2). The third does not decompose:

- db's last unconditional os edge is `store::pack_rt::encode_wire_value`/`decode_wire_value`
  (4 call sites in `📄️artifact`).
- Those wrap a `DslValue` in a one-field record and call the **schema-driven** `encode_record_body`.
- Promoting that codec byte-identically requires `RecordSpec`/`RecordValue`/`FieldValue`/`Shape`
  (~250 lines, fine) **plus** `parse_expr_text`, which calls `lex`/`Cursor`/`parse_expr` — the DSL
  lexer (1256 lines) and token layer (365), which in turn reference `os_dsl::schema` circularly.
- Changing the format instead is not local either: the TypeScript twin decodes these exact bytes
  (`🟦️backbone-worker.ts:344` `decodePayload` → `decodePackValue` over `envelope.diff.payload`),
  so it is a dual-language format change needing its own parity fixtures.

**Decision: build the server core first.** It is the bulk of Wave 2, it does not depend on how the
db question resolves, and it keeps the tree green. db relocation (B3/B5) stays open as one
well-characterised item rather than being forced through at the end of a long session.

## Landed

`🖥️server/🔨️modules/` now holds four modules, all pure sync Rust over `protocol` + the contract —
no axum, no tokio, no storage driver, no clock (callers pass `HybridLogicalClock`):

- **`🧬️contract`** — envelopes, outcomes, offline policy, actor keys, durable vs ephemeral lane
  records, policy vocabulary, module manifests, instance definition.
- **`🗄️storage`** — the four storage roles as traits + in-memory implementations:
  `AuthorityStore` (receipts/idempotency, per-actor event streams, snapshots, transactional outbox,
  fenced leases), `ProjectionStore` (rebuildable, with checkpoints and `clear`), `BlobStore`
  (content-addressed, caller-supplied hash + a `content_hash` helper over `protocol`'s Blake3),
  `SessionStore`. `StorageProfile::Embedded` is the Wave-2 deployment profile.
- **`🛡️policy`** — `PolicyEngine` with roles as data (`PolicyTemplate`/`PolicyGrant`), closed by
  default, deny-overrides via a `!action` grant, scope-limited assignment; `PrincipalResolver`
  chain (the ladder is generic, the rungs are instance-supplied); `AdminGate`.
- **`🎭️authority`** — the actor turn protocol: `Decider` (pure, rerun by both replica and
  authority), `AuthorityDirectory` (single-process map implementing a contract that already carries
  placement, activation epoch, fenced lease, mailbox seq and passivation), `CommandBus::submit`
  running admit → **idempotency dedup** → policy admission → activation → expected-revision fence →
  decide → atomic event+outbox append → evolve → receipt, and a `Saga`/`SagaRunner` seam.

### Reconciliation applied

The three modules were written concurrently and disagreed at their seam; resolved in favour of the
stronger design rather than the earlier one:
- `append_events(actor, events, outbox)` is now **one atomic call** — a turn can never advance state
  without queueing its publications (the transactional-outbox law).
- `OutboxEntry` carries `kind`/`payload`/`event: Option<EventRecord>` so pure effects queue
  alongside event-derived saga rows.
- `record_receipt` takes references; the bus keeps its receipt to build the outcome.
- `Lease` is the storage shape `{ epoch, holder }`; the directory mints it.

## 📡️gateway — DONE (server core complete)

Layer 3, added with `axum 0.8 (ws)`, `tokio (sync,rt,macros,net,time)`, `dashmap 6`, `futures 0.3`
(hub's own versions; nothing else).

- `ServerModule` (runtime half: manifest/deciders/routes/resolvers/templates) — `routes` lives here
  because it needs axum; the declarative `ModuleManifest` stays in contract.
- **`DocumentAuthority` port** — `welcome` / `submit_frame` in terms of `protocol` bytes. This is how
  a document websocket is bridged without the server product naming any engine. Hub supplies the
  implementation over db; the server never depends on db, and therefore never on os.
- `ServerState`, `Fanout` (per-scope broadcast, channel dropped when empty), `Presence` + 256-slot
  colour leases (lowest free slot, released on last disconnect), `KickMap`, `cors_middleware`,
  credential extraction with a loopback flag, blob `PUT`/`GET`/`HEAD` (hash mismatch → 409),
  **one** `StaticAppHost` (traversal-guarded, content-type table, index.html fallback — this is what
  deletes hub's two duplicated asset servers), `AppRegistry`, the replay-then-live event bridge
  (subscribe first, then replay, then dedupe by seq), and the command/query endpoints.
- `Server::builder(profile).module(..).document_authority(..).app(..).admin_token(..).build()`.

**Verified in the shared workspace build, not just the scratch target dir:**
replication 184 · pack 42 · **server 73** · os-kernel 779 · db 424 · hub compiles ·
plugin-describe compiles · kernel and pack wasm `--lib` clean.

**Invariant check:** `grep -rE "semio-framework-os|\bdb::|os_dsl|os_store|os_pack|os_spr"` across
`🛍️products/🖥️server/` returns **nothing**. The server product does not reference os in any form.

launch.json entries were silently overwritten once by a concurrent session and had to be re-added —
worth re-checking at the end of any session that edits it.
