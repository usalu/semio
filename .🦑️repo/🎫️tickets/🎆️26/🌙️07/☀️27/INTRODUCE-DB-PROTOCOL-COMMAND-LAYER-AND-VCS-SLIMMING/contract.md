# db + protocol + vcs Integration Campaign — Contract (Wave CW0)

Full rationale and design: `/Users/ueli/.claude/plans/introduce-a-new-technology-cuddly-rabbit.md`
(the approved plan). Read it first. This document is the binding cross-crate/cross-wave interface;
deviate only where explicitly marked "your choice", and report any deviation in your wave's `.txt`
report in this ticket folder.

**Relationship to `PROTOCOL-BINARY-OP-LOG-LAYER`**: that ticket's `contract.md` (same directory
tree, `.repo/🎫️/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md`) is the frozen contract for the
full `protocol` crate family (12 crates: core/command/causal/crdt/format/history/materialize/io/
wire/facade/testkit/cli) — this campaign extended it in place (see its "Amendment" section) rather
than forking a second definition. Implement `protocol` strictly against that file. This ticket's
scope is everything downstream: the `db` crate family, the kernel cut-over (vcs slimming +
framework/core extraction), hub rebuilds, wire v2 client integration, and the ~40-crate app sweep.

Repo conventions (identical to every prior rollout — pack, protocol): single-file `lib.rs` per
crate at `<tech>/<part>/rs/lib.rs`; `[lib] path = "lib.rs"`; `edition = "2021"`,
`rust-version = "1.88"`; `[lints] workspace = true`; `//#region 🔖️Name` / `//#endregion 🔖️Name`
blocks; every doc comment starts with an emoji (db family uses 🗄️, distinct from pack's 📦️ and
protocol's 🎞️); tests inline in trailing `//#region 🧪️Tests`, `mod tests { mod quick {} mod long {}
mod exhaustive {} }` only where slow tiers exist; no `unsafe` unless justified inline; no
`std::io::Error` in public signatures (`DbError::Io(String)` pattern); external libs behind traits.

---

## db crate family (24 crates, package prefix `db_`, facade `db`)

Full per-crate responsibility table, WAL record format, actor/mailbox design, persistent
diff-state structures, storage traits, feature flags, testkit contents, CLI subcommands, and the
stable `Database`/`DocumentHandle` API boundary are specified in the approved plan's **Part 2**
(`/Users/ueli/.claude/plans/introduce-a-new-technology-cuddly-rabbit.md`). Implement exactly
against that table. Key points repeated here because they gate correctness across crate
boundaries:

- **No cdylib anywhere in the db family** — native rlib/bin only. Sidesteps the LLVM-22 wasm
  linker `ElemSection::writeBody` crash that forced `[profile.release.package.vcs]
  codegen-units = 16` in root `Cargo.toml`. `db_core`/`db_state`/`db_actor` (core mailbox only)/
  `db_conflict` stay `wasm32-unknown-unknown`-clean regardless (cheap, keeps a future client-side
  reuse door open); everything storage/thread-touching is `#[cfg(not(target_arch = "wasm32"))]`
  module-wrapped.
- **WAL = a `.spr` file.** `db_wal` reuses `protocol::{SprWriter, FrameCursor, ReverseFrameCursor,
  recover}` directly — it does NOT invent new framing. db record kinds live in the SPR extension
  range `0x40..=0x4F` (`WAL_SEGMENT_HEADER, WAL_TX_BEGIN/COMMIT/ABORT, WAL_COMMAND, WAL_PAYLOAD,
  WAL_DIFF, WAL_INVERSE, WAL_EVENT, WAL_OUTBOX, WAL_FRONTIER, WAL_VCS_REF, WAL_SNAPSHOT_PUB,
  WAL_INDEX_CKPT, WAL_LEASE, WAL_MIGRATION` — exact byte values and payload layouts are `db_wal`'s
  own choice within this range, documented in its own `//#region 🔖️RecordKinds`).
- **Snapshots are pack files.** Pages go in `KIND_CHUNK` segments; the snapshot descriptor
  (frontier, protocol version, VCS head, base pack hash, root list) is the payload of the
  previously-reserved **`KIND_SNAPSHOT` (0x07)** segment kind (see `pack/core/rs/lib.rs`'s
  `SegmentKind` consts — this is the first real consumer). Incremental generations use
  `Footer.prev_footer_offset` + `REQUIRED_FOOTER_CHAIN` (also previously reserved-unused).
- **Persistent diff-state structures are hand-rolled** in `db_state` (`PMap` 32-way HAMT, `PVec`
  trie, `PText` rope, `PTree`, `PGraph`) — no `im`/`im-rc`/`rpds`/`imbl` dependency (none exist
  anywhere in `Cargo.lock` today; repo convention is hand-rolled behind traits).
- **Storage substrate is pluggable** (user requirement: db-native/sqlite/postgres/neo4j must all
  be swappable): `db_storage` defines the trait family (`DbStorage`, `WalStorage`,
  `SnapshotStorage`, `PayloadStorage`, `CatalogStorage`, `IndexStorage`, `LeaseStorage`) plus
  `MemoryStorage`/`FsStorage` (zero-touch default, pure files, no new C deps). Three sibling
  crates — `db_storage_sqlite`, `db_storage_postgres`, `db_storage_neo4j` — each implement
  `DbStorage` against the same trait set as separate, independently-optional backends. `db_engine`
  and the `db` facade select a backend via `Arc<dyn DbStorage>` at `Database::open` — never a
  compile-time-only choice. `db_storage_sqlite` must link the SAME bundled rusqlite version vcs
  already uses (single-link-per-workspace constraint).
- **Hard dependency rules** (unchanged from the plan): db crates never depend on
  `semio-framework-core`, `os-hub-storage*`, `pack_value`, or `dsl_*`. Only `db_engine` depends on
  `vcs` (behind Cargo feature `vcs`, through a `db_core::VersionGraph` trait — every other db
  crate is vcs-free). Command payloads are opaque `protocol::OperationEnvelope`/binary bytes below
  `db_document` — no db crate below it interprets operation semantics.
- **Stable API** (frozen, do not change signatures without a ticket note):
  ```rust
  pub struct Database { /* ... */ }
  impl Database {
      pub fn open(config: DbConfig, storage: std::sync::Arc<dyn DbStorage>) -> Result<Database, DbError>;
      pub fn open_at(root: &std::path::Path, profile: Profile) -> Result<Database, DbError>; // FsStorage, zero-touch
      pub fn create_document(&self, spec: DocumentSpec) -> Result<DocumentHandle, DbError>;
      pub fn document(&self, id: &protocol::DocumentId) -> Result<DocumentHandle, DbError>;
      pub fn catalog(&self) -> CatalogView;
      pub fn health(&self) -> DbHealth;
      pub fn shutdown(self, deadline: std::time::Duration) -> Result<(), DbError>;
  }
  #[derive(Clone)] pub struct DocumentHandle { /* mailbox sender + generation */ }
  impl DocumentHandle {
      pub fn submit(&self, batch: CommandBatch, options: SubmitOptions) -> SubmitFuture; // -> Result<CommandReceipt, DbError>
      pub fn query(&self, query: Query, consistency: Consistency) -> Result<QueryStream, DbError>;
      pub fn subscribe(&self, spec: LiveQuerySpec) -> Result<LiveQuery, DbError>;
      pub fn frontier(&self) -> Result<Frontier, DbError>;
      pub fn preview(&self, base: Frontier) -> Result<PreviewHandle, DbError>;
      pub fn history(&self) -> Result<HistoryView, DbError>;
      pub fn snapshot_now(&self, kind: SnapshotKind) -> SnapshotFuture;
  }
  pub struct CommandReceipt { pub command_id: protocol::OperationId, pub frontier: Frontier, pub durability: DurabilityClass, pub conflicts: Vec<ConflictRecord>, pub state_hash: Option<pack_core::ContentHash> }
  pub struct Frontier { pub document: protocol::DocumentId, pub head_seq: u64, pub commit_seq: u64, pub chain_hash: [u8; 32], pub epoch: u64 }
  pub enum Consistency { Canonical, AtLeast(Frontier), Exact(Frontier), Historical(String /* commit id */), Speculative(String /* preview id */), PreviewAugmented(String) }
  pub enum DurabilityClass { Memory, Os, Fsync, Quorum(u8) }
  ```

## Kernel cut-over surface (vcs slimming + framework/core extraction)

See plan **Part 1** ("vcs slimming") for the full stays/moves/dies table.
- vcs keeps: `DocumentVcs*`, `Change`/`Checkpoint`/`Alternative`, `Author`, materialize-by-replay,
  history columns, `DocumentVcsStore`, Backbones, `BlobStore`, Studio layer, `CodecRegistry`,
  `DocumentDsl`/`DocumentPack`/`pack_rt`, folder storages, `test_support`. Its `dag` field
  re-types to `protocol::OpDag`; it drops its `semio-framework-core` dependency entirely.
- vcs fixes: content-addressed checkpoint ids — `Checkpoint.id = format!("ck-{}", hex16(blake3(
  parent_id || ordered_change_content_hashes || message || authors || timestamp)))` using
  `pack_core::ContentHash`; new `merge_base(envelope, a, b)` + ancestor-traversal helpers beside
  `build_history_columns`.
- `dsl/derive/rs`'s `derive_dsl_document` macro flip: `impl ::vcs::OpText` → `impl ::protocol::OpText`
  — this is atomic across every `#[derive(dsl::DslOps)]` crate (~40 crates gain a `protocol` path
  dep in the same wave). Land temporary `pub use protocol::{Operation, OperationDiff, OpText, ...}`
  shims inside vcs during the transition so the tree keeps compiling while the app sweep runs in
  parallel; delete the shims in the closing wave.
- framework/core loses (moves to protocol, per the amended protocol contract):
  `HybridLogicalTimestamp`, `PayloadHash`/`OperationEnvelope`/`OpDag` (+ tests), `UndoPolicy`,
  `MergeStrategyKind`, the operation-flavored id newtypes, and the `🔖️HubProtocol` region
  (`HubClientFrame`/`HubServerFrame`/presence types — superseded by `protocol_wire`). framework/core
  gains a narrow `protocol` dependency for the dual-use ids it still needs in its kernel
  invocation types. `framework/hlc/rs` is deleted outright (zero dependents, verified).

## Hub rebuild surface

`os-hub` (`framework/product/os/hub/rs/bin.rs`) becomes a thin axum shell:
`HubState { db: db::Database, directory: Arc<dyn HubDirectory>, admin_token: Option<String> }`.
`HubStorage` (today's trait in `framework/product/os/hub/storage/rs`) is split: its document/blob
methods die (db owns them); its identity/tenancy methods (users, studios, memberships, auth
sessions, share tokens, vfs nodes, sync sessions) survive as `trait HubDirectory`, implemented by
a sibling set matching db's own backend swappability requirement — `os-hub-directory-sqlite`
(default), `-postgres`, `-neo4j` (repurposed from today's `os-hub-storage-{sqlite,postgres,neo4j}`
crates; their document-persistence code is deleted, their identity-table code survives).
`compose-hub` gets the identical treatment on its own document/session tables.

## Wire v2 surface

`protocol_wire`'s `ClientFrame`/`ServerFrame` (frozen in the amended protocol contract) are the
wire types for both `os-hub`/`compose-hub` (server) and `framework/sync` (client, native + wasm).
`db_sync` implements the server side of frontier exchange/missing-command transfer/snapshot
bootstrap/resume; `framework/sync`'s `SyncSession`/`DocumentHost` actors implement the client side
against the same frame types. No JSON frame format survives past this wave outside the sandboxed
WIT backbone seam (which intentionally stays JSON — see plan Part 3).

## Wave plan for this ticket (CW1 onward — CW0 is this document)

| Wave | Content | Mode |
|---|---|---|
| CW1 | Scaffold protocol's 12 crates (Cargo.toml/project.json/script.ts, root workspace members, launch.json) | 1 agent, shared files |
| CW2 | Implement all 12 protocol crates against the amended contract | parallel batches |
| CW3 | Kernel cut-over: framework/core extraction, vcs slim + shims, dsl_derive flip, ~40-crate dep/import sweep | 1 agent, serial |
| CW4 | Scaffold + implement db's 24 crates in dependency batches | parallel batches |
| CW5 | Wire v2 client integration (framework/sync native+wasm, worker, backbone-worker.ts) | 1 agent |
| CW6 | Hub rebuilds (os-hub, compose-hub) | 2 agents |
| CW7 | App fan-out (all plugin crates + compose client lib) | parallel, disjoint |
| CW8 | Shim removal, policy lints, allowlist assertions | 1 agent |
| CW9 | Verification: build, clippy, leveled tests, verify gate, e2e | 1-2 agents |

## Workspace-wide requirements (identical to every prior rollout)

- Per-crate/per-wave agents write only their own crate's `Cargo.toml` + `lib.rs` (+ `benches/*.rs`
  for testkit crates). A single closing agent per wave owns root `Cargo.toml` members,
  `.vscode/launch.json`, and per-crate `project.json`/`script.ts`.
- Scratch/progress notes go only in this ticket folder
  (`.repo/🎫️/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/`), as `.txt` files
  (never `.log` — gitignored, silently dropped by `ticket_close`).
- Never revert another session's concurrent work; re-read shared files immediately before editing
  them; if a shared-file edit conflicts, re-read and retry rather than force-overwrite.
- AGENTS.md files are never edited by agents (repo rule) — flag needed AGENTS.md changes
  (`playbook/AGENTS.md` stale front-matter, new `protocol/AGENTS.md`/`db/AGENTS.md`) in the
  closing wave's report as human todos.
