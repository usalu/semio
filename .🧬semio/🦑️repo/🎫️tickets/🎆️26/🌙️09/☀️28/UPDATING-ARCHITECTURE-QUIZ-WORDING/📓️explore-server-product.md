# Exploration: building `teaching/proctor` on the framework `🖥️server` product

Scope: map `🧰️framework/🛍️products/🖥️server` (the generic CQRS/event-sourced server-instance
framework), how `🌎️hub` composes it, how clients talk to it, schema-first conventions, and testing —
then recommend an approach for a small `proctor` quiz server backed by one SQLite database.

All paths are relative to the repo root `C:\git\semio`. Emoji in paths are literal.

---

## 1. `🧰️framework/🛍️products/🖥️server` — the generic CQRS/event-sourcing server product

### 1.1 File tree (`git ls-files`)

```
🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/package.json
🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/📋️project.json
🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/📜️script.ts
🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🟦️.ts
🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/Cargo.toml
🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/📋️project.json
🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/🦀️.rs
🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs                (+ 🧪️tests/🔬️unit/🦀️.rs)
🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs                  (+ 🧪️tests/🔬️unit/🦀️.rs)
🧰️framework/🛍️products/🖥️server/🔨️modules/🔣️.json                          (module registry, see below)
🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs                  (+ 🧪️tests/🔬️unit/🦀️.rs)
🧰️framework/🛍️products/🖥️server/🔨️modules/🛡️policy/🦀️.rs                   (+ 🧪️tests/🔬️unit/🦀️.rs)
🧰️framework/🛍️products/🖥️server/🔨️modules/🧬️contract/🦀️.rs                 (+ 🧪️tests/🔬️unit/🦀️.rs)
🧰️framework/🛍️products/🖥️server/🟦️.ts                                      (façade `pub use crate::contract::*`... wait, this is Rust — see below)
🧰️framework/🛍️products/🖥️server/🦀️.rs                                      (Rust product façade, `pub use crate::contract::*`)
🧰️framework/🛍️products/🖥️server/🧪️tests/🎚️config/🟦️.ts                     (vitest config)
🧰️framework/🛍️products/🖥️server/🧪️tests/🔒️closed-ports/🦀️.rs               (dyn_enum_close cross-crate gate)
🧰️framework/🛍️products/🖥️server/🧪️tests/🔬️conformance/🦀️.rs                (storage-role conformance suite, generic over backend)
🧰️framework/🛍️products/🖥️server/🧪️tests/🔬️wire/🟦️.ts + 🦀️.rs               (cross-language wire gate)
🧰️framework/🛍️products/🖥️server/🧪️tests/🧩️instance/🦀️.rs                   (reference `ServerInstance`: TestInstance)
🧰️framework/🛍️products/🖥️server/🧫️fixtures/🔌️wire/🔣️.json                  (language-agnostic wire vectors + route table)
```

No README in this product (`🌎️hub/README.md` is the operator doc for the *composed instance*, not
for the framework product).

### 1.2 The three layers, and why they never merge

`🧬️contract/🦀️.rs` header (verbatim):

> "Three layers stay strictly separate and are never collapsed into one "message" abstraction: the
> **CQRS dual bus** (`CommandEnvelope`/`QueryEnvelope`) carries application intent and projection
> reads; the **actor turn protocol** (`ActorKey`, `Decision`) is the consistency boundary a command is
> serialized through; the **replication protocol** (`protocol` crate) moves causal state between
> replicas. A UI action is none of the three and never reaches this crate."

- Layer 1 — `contract` (pure data, no axum/storage/clock): envelopes, outcomes, policy vocabulary,
  module manifests.
- Layer 2 — `authority` (turn protocol) + `policy` (authz) + `storage` (4 durability roles).
- Layer 3 — `gateway` (HTTP/WS transport only; "nothing here decides anything").

### 1.3 Command/query envelopes and outcomes (verbatim Rust, `🧬️contract/🦀️.rs`)

```rust
pub struct CommandEnvelope {
    pub command_id: CommandId,
    pub kind: String,
    pub version: u32,
    pub target: ActorKey,
    pub scope: Scope,
    pub principal: Principal,
    pub session: Option<SessionId>,
    pub device: Option<DeviceId>,
    pub payload: Vec<u8>,
    pub causal_frontier: Option<FrontierSummary>,
    pub client_hlc: HybridLogicalClock,
    pub expected_revision: Option<Revision>,
    pub idempotency_key: Option<IdempotencyKey>,
    pub capability_proof: Option<CapabilityProof>,
    pub trace: TraceContext,
}

pub enum CommandOutcome {
    Accepted { receipt: CommandReceipt, events: Vec<EventRecord>, frontier: Option<FrontierSummary> },
    Transformed { receipt: CommandReceipt, canonical_events: Vec<EventRecord>, frontier: Option<FrontierSummary>, notices: Vec<Notice> },
    Rejected { receipt: CommandReceipt, reason: Rejection, notices: Vec<Notice> },
    Pending { receipt: CommandReceipt, process: ProcessId },
}

pub enum OfflinePolicy { Optimistic, Deferred, AuthorityRequired }  // declared per command kind

pub struct QueryEnvelope {
    pub query_id: QueryId, pub kind: String, pub version: u32, pub scope: Scope,
    pub principal: Principal, pub arguments: Vec<u8>, pub consistency: QueryConsistency,
    pub cursor: Option<QueryCursor>,
}
pub enum QueryConsistency { Local, AtFrontier { frontier: FrontierSummary }, Authority }
pub enum QueryResult {
    Snapshot { value: Vec<u8>, frontier: Option<FrontierSummary> },
    Page { items: Vec<Vec<u8>>, next: Option<QueryCursor>, frontier: Option<FrontierSummary> },
    Subscription { subscription: SubscriptionId, initial: Vec<u8>, cursor: Option<QueryCursor>, frontier: Option<FrontierSummary> },
}
```

`Principal` is never a role: `User{id}`, `ServiceAccount{id}`, `Device{id}`, `Anonymous` — roles are
`PolicyTemplate` values assigned to a principal, evaluated at a `PolicyPoint`. This maps directly onto
proctor's "anonymous / pseudonym / name" identity spectrum: identity kind and display handle are a
*domain* concern layered on top of `Principal`, not something the contract dictates.

### 1.4 Actor turn protocol — the CQRS command bus (`🎭️authority/🦀️.rs`, 470 lines)

The `Decider` trait every actor kind implements (pure core, shared between the authority and an
optimistic client replica):

```rust
pub trait Decider: Send + Sync {
    fn actor_kind(&self) -> impl Future<Output = &str> + Send;
    fn decide(&self, state: &ActorState, command: &CommandEnvelope, context: &DecisionContext) -> impl Future<Output = Decision> + Send;
    fn evolve(&self, state: &mut ActorState, event: &EventRecord) -> impl Future<Output = ()> + Send;
}

pub enum Decision {
    Emit { events: Vec<EventRecord>, effects: Vec<Effect> },
    Reject(Rejection),
    Defer(ProcessId),
}
```

`decide` **must be pure** — same state/command/context → same `Decision`, always; the only ambient
input is `DecisionContext { now, principal, scope }`. `evolve` folds a committed event into
`state.bytes` and never touches `state.revision` (the protocol owns revision/optimistic concurrency).

`CommandBus<S: AuthorityStore, D: Decider>::submit` runs the ten-step turn documented and enforced
in source order (admit → deduplicate by `IdempotencyKey` → authorize at
`PolicyPoint::CommandAdmission` → place/activate with a fencing lease → fence `expected_revision` →
decide → commit (events + outbox atomically) → evolve → record receipt → answer). Exactly-once is a
protocol law: a resubmitted idempotency key returns the byte-identical receipt with no second event,
checked *before* policy and placement.

Cross-actor workflows are `Saga` (`fn on_event(&self, event) -> Vec<CommandEnvelope>`), drained from
the transactional outbox by `SagaRunner::drain_outbox` — explicitly **no distributed transactions**;
cross-actor consistency is reached by compensating commands.

### 1.5 Storage: four separate roles, zero implementations (`🗄️storage/🦀️.rs`, 363 lines)

> "Each role has a different truth, a different lifetime and a different recovery story, so
> collapsing them into one trait would force every backend to be good at all four."

```rust
pub trait AuthorityStore: Send + Sync {
    fn receipt(&self, key: &IdempotencyKey) -> impl Future<Output = Result<Option<CommandReceipt>, StorageError>> + Send;
    fn record_receipt(&mut self, key: &IdempotencyKey, receipt: &CommandReceipt) -> impl Future<Output = Result<(), StorageError>> + Send;
    fn append_events(&mut self, actor: &ActorKey, events: &[EventRecord], outbox: &[OutboxEntry]) -> impl Future<Output = Result<u64, StorageError>> + Send;
    fn events_since(&self, actor: &ActorKey, since: u64) -> impl Future<Output = Result<Vec<EventRecord>, StorageError>> + Send;
    fn last_seq(&self, actor: &ActorKey) -> impl Future<Output = Result<u64, StorageError>> + Send;
    fn put_snapshot(&mut self, actor: &ActorKey, revision: Revision, bytes: Vec<u8>) -> impl Future<Output = Result<(), StorageError>> + Send;
    fn snapshot(&self, actor: &ActorKey) -> impl Future<Output = Result<Option<(Revision, Vec<u8>)>, StorageError>> + Send;
    fn enqueue_outbox(&mut self, entries: Vec<OutboxEntry>) -> impl Future<Output = Result<(), StorageError>> + Send;
    fn pending_outbox(&self, limit: usize) -> impl Future<Output = Result<Vec<OutboxEntry>, StorageError>> + Send;
    fn mark_outbox_delivered(&mut self, ids: &[u64]) -> impl Future<Output = Result<(), StorageError>> + Send;
    fn acquire_lease(&mut self, actor: &ActorKey, holder: &str) -> impl Future<Output = Result<Lease, StorageError>> + Send;
    fn validate_lease(&self, actor: &ActorKey, lease: &Lease) -> impl Future<Output = bool> + Send;
}

pub trait ProjectionStore: Send + Sync {  // rebuildable read models — leaderboard lives here
    fn put(&mut self, projection: &str, key: &str, value: Vec<u8>) -> impl Future<Output = Result<(), StorageError>> + Send;
    fn get(&self, projection: &str, key: &str) -> impl Future<Output = Option<Vec<u8>>> + Send;
    fn list(&self, projection: &str, prefix: &str) -> impl Future<Output = Vec<(String, Vec<u8>)>> + Send;
    fn checkpoint(&self, projection: &str) -> impl Future<Output = u64> + Send;
    fn set_checkpoint(&mut self, projection: &str, seq: u64) -> impl Future<Output = Result<(), StorageError>> + Send;
    fn clear(&mut self, projection: &str) -> impl Future<Output = Result<(), StorageError>> + Send;
}

pub trait BlobStore: Send + Sync { /* put/get/has, content-addressed via blake3 content_hash() */ }
pub trait SessionStore: Send + Sync { /* create/get/delete/revoke_principal — deliberately NOT event-sourced */ }
```

`StorageProfile` is two variants only: `Ephemeral` (nothing survives the process) and
`Embedded { data_dir: String }` (one directory owned by one process — "the deployment profile for
hub, zentrale **and a developer's laptop alike**"). No clustered/hosted profile exists yet.

The **conformance suite** (`🧪️tests/🔬️conformance/🦀️.rs`) writes each role's contract once as
`async fn(store: &mut impl XStore)` — e.g. `receipt_round_trips_and_is_idempotent`,
`append_events_rejects_a_sequence_gap_and_writes_nothing` — compiled into the framework only under
`#[cfg(test)]` or the `conformance` feature, so any downstream backend (a SQLite one for proctor) can
enable that dev-dependency feature and be held against the *same* assertions the in-memory reference
backend is, rather than a re-typed copy.

### 1.6 Policy engine (`🛡️policy/🦀️.rs`, 331 lines)

Closed by default, deny overrides allow, roles as data:

```rust
pub struct PolicyEngine { templates: BTreeMap<String, PolicyTemplate>, assignments: BTreeMap<String, Vec<Assignment>>, authenticated: Option<String> }
impl PolicyEngine {
    pub fn register_template(&mut self, template: PolicyTemplate);
    pub fn set_authenticated_template(&mut self, template_name: String);   // auto-applied to every non-anonymous principal
    pub fn assign(&mut self, principal_key: String, template_name: String);
    pub fn assign_scoped(&mut self, principal_key: String, scope: Scope, template_name: String);
    pub fn evaluate(&self, request: &PolicyRequest) -> PolicyDecision;
}
```

Resource patterns: `space:abc/*` prefix wildcard, `*` matches everything, else exact; actions support a
leading `!` for explicit deny (`!write`). `PolicyPoint` has nine values: `CommandAdmission`,
`CommandExecution`, `QueryAccess`, `Subscription`, `EventDelivery`, `BlobRead`, `BlobWrite`, `Effect`,
`Administration`. Authentication is a separate `ResolverChain<R: PrincipalResolver>` ladder (first
match wins, falls through to `Principal::Anonymous`) — exactly the seam proctor's
anonymous/pseudonym/name identity spectrum plugs into (a rung per identity kind).
`AdminGate` guards the admin plane before policy: bearer-token mode if configured, else
loopback-only — "no third mode... in particular no 'no token means open'."

### 1.7 Gateway / transport (`📡️gateway/🦀️.rs`, 1658 lines) — axum, not bun.serve

```rust
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::{Json, Router};
use tokio::sync::{broadcast, Mutex, Notify};
```
`Cargo.toml`: `axum = { version = "0.8", features = ["ws"] }`, `tokio = { features = ["sync","rt","macros","net","time"] }`.
Binding (`Server::run`):
```rust
pub async fn run(self, addr: SocketAddr) -> Result<(), ServerError> {
    let listener = tokio::net::TcpListener::bind(addr).await...;
    axum::serve(listener, self.router.into_make_service_with_connect_info::<SocketAddr>()).await...
}
```

**The composition seam — `ServerInstance`** ("one place every extension point is named"):

```rust
pub trait ServerInstance: Sized + Send + Sync + 'static {
    type Modules: ServerModule<Instance = Self> + 'static;
    type Queries: QueryHandler + 'static;
    type Documents: DocumentAuthority + 'static;        // NoDocumentAuthority if none
    type Deciders: Decider + 'static;
    type Sagas: Saga + 'static;
    type Resolvers: PrincipalResolver + 'static;
    type AuthorityStore: AuthorityStore + 'static;
    type ProjectionStore: ProjectionStore + 'static;
    type BlobStore: BlobStore + 'static;
    type SessionStore: SessionStore + 'static;
    async fn open(profile: &StorageProfile) -> Result<InstanceStores<Self>, StorageError>;
}

pub trait ServerModule: Send + Sync {
    type Instance: ServerInstance;
    async fn manifest(&self) -> ModuleManifest;
    async fn deciders(&self) -> Vec<<Self::Instance as ServerInstance>::Deciders> { Vec::new() }
    async fn sagas(&self) -> Vec<<Self::Instance as ServerInstance>::Sagas> { Vec::new() }
    async fn routes(&self, router: Router<ServerState<Self::Instance>>) -> Router<ServerState<Self::Instance>> { router }
    async fn resolvers(&self) -> Vec<<Self::Instance as ServerInstance>::Resolvers> { Vec::new() }
    async fn templates(&self) -> Vec<PolicyTemplate> { Vec::new() }
}
```

`ServerBuilder<I>` (builder pattern: `.module(...).query(...).saga(...).app(...).admin_token(...).identity(id, version)`)
collects every module's manifest/deciders/sagas/resolvers/templates, opens `I::open(&profile)`, and
produces a `Server<I>` wrapping one shared `ServerState<I>` (all fields `Arc`-wrapped: `authority:
Arc<Mutex<CommandBus<...>>>`, `projections`, `blobs`, `sessions`, `policy: Arc<RwLock<PolicyEngine>>`,
`resolvers`, `admin`, `fanout: Arc<Fanout>` (per-lane `tokio::broadcast`), `presence`, `kicks`, `apps`,
`queries`, `documents: Option<Arc<I::Documents>>`, `profile`, `clock`).

`base_router<I>` — the **twelve fixed routes** every instance gets before a module adds its own
(verbatim from both the Rust source and the language-agnostic fixture
`🧫️fixtures/🔌️wire/🔣️.json`, which pins this table so a TS client drift fails a test):

```
GET  /instance
POST /commands
POST /queries
POST /scopes/{scope}/ephemeral
GET  /actors/{tenant}/{kind}/{id}/events
GET  /actors/{tenant}/{kind}/{id}/events/ws
GET  /blobs/{hash}      HEAD /blobs/{hash}      PUT /blobs/{hash}
GET  /apps              GET /apps/{app}/installs   GET /apps/{app}   GET /apps/{app}/{*rest}
GET  /scopes/{scope}/document/ws     (mounted only if the instance registered a DocumentAuthority)
```

Durable vs. ephemeral lanes never merge: `stream_lane`/`ephemeral_lane`/`document_lane` are distinct
`Fanout` keys; the durable event-stream websocket replays `AuthorityStore::events_since` then
transitions to live, deduplicated by `seq`, "so a reconnecting client sees every fact exactly once" —
this is the built-in mechanism for proctor clients resuming a quiz run on another device or after a
short outage.

### 1.8 Languages, canonical implementation, storage backend

- **Rust is canonical and the only runtime.** `@semio-tech/framework-server-rs`
  (`📦️packages/🦀️rust`) is a `projectType: library` with `build`/`test` targets only (`Cargo.toml`
  description: "the server-instance framework that hub and zentrale compose"). It depends on `axum`,
  `tokio`, `serde`/`serde_json`, and the repo's own `semio-framework-async` / `semio-framework-
  replication` / `semio-framework-dispatch-macros` (the `#[dyn_enum]`/`dyn_enum_close!` machinery that
  replaces `dyn Trait` objects with generated enums, closed at the instance crate).
- **TypeScript is client-only.** `@semio-tech/framework-server`
  (`📦️packages/🟦️typescript/package.json`, description: *"typed client for the authoritative server:
  command submit, projection queries, live subscriptions"*) is a hand-written wire codec + `ServerClient`
  class (`instance()`, `submitCommand()`, `query()`, `publishEphemeral()`, `events()`, `blob()`/
  `hasBlob()`/`putBlob()`, `apps()`, `appInstalls()`, `eventStreamUrl()`, `documentSocketUrl()`).
  There is **no TypeScript server runtime** for this product anywhere in the repo.
- **No Go implementation.**
- **Storage backend of the framework itself: none shipped.** The four traits have zero
  implementations in the product; the only in-memory reference lives with the tests
  (`🧪️tests/🧩️instance/🦀️.rs`, `TestInstance`/`MemoryAuthorityStore`/etc.) explicitly "because a
  reference implementation is a test fixture, not a product surface." A real backend is supplied by
  whoever composes an instance — see §2.3 for how hub does it (SQLite for identity via `rusqlite`, a
  hand-rolled JSON-lines file journal for the four server-product roles).
- **HTTP/WS serving: axum + tokio**, never `bun.serve`. `bun`/Nx only drive the build/test/dev *scripts*
  (§1.9); the runtime binary is a native Rust process.

### 1.9 Cross-language wire gate and reference instance (worked examples)

`🧫️fixtures/🔌️wire/🔣️.json` (`schema: "semio.framework.server.wire/v1"`) carries the route table plus
one canonical JSON vector per wire type; `🧪️tests/🔬️wire/🦀️.rs` and `🧪️tests/🔬️wire/🟦️.ts` each
decode every vector into their own twin and re-encode it, asserting byte-identical JSON — "a rename on
either side fails the other side's suite," which is the *only* mechanism keeping the Rust contract and
the hand-maintained TypeScript twin honest (no codegen — see §4).

`🧪️tests/🧩️instance/🦀️.rs` (`TestInstance`) is explicitly documented as "the worked example a real
instance follows": one module (`CountingModule`) with two deciders (`CounterDecider` exercising all
four `Decision` branches, `MirrorDecider` closed alongside it via `dyn_enum_close!` to prove the macro
dispatches across two variants, not just aliases one) and in-memory stores
(`MemoryAuthorityStore`/`MemoryProjectionStore`/`MemoryBlobStore`/`MemorySessionStore`). **This file is
the best starting template for proctor's own decider(s).**

---

## 2. `🌎️hub` — how it composes the server product

### 2.1 What hub is

`README.md` (verbatim, opening lines): *"The hub is the server half of semio: identity and tenancy
(the directory), the durable event-sourced document store, presence and the collaboration sockets two
browsers meet on, plus the artifact-authority and hub-backed inference lanes. It is a single native
binary, `os-hub`, with no runtime service dependencies beyond a data directory on disk."*

Hub is **much bigger than the generic server product** — it bundles real-time collaborative document
editing (a separate, older replication/CRDT-adjacent system under `🧰️framework/🔨️modules/📡️replication`
and `🧰️framework/🛍️products/💻️os`), an artifact-authority/content-addressed-storage subsystem, a
plugin/inference runtime, an admin SPA, and a full identity/tenancy "directory" with its own
SQLite/Postgres/Neo4j-selectable backend and its **own**, separate event-sourcing scheme
(`hub_directory_event`, `hub_directory_command_receipt`) that predates and sits *beside* the generic
`ServerInstance` framework rather than inside it.

### 2.2 Composing the generic framework — `HubInstance` (`🌎️hub/🗄️stores/🦀️.rs`, verbatim)

```rust
pub struct HubInstance;

impl ServerInstance for HubInstance {
    type Modules = HubModules;
    type Queries = NoQueryHandler;
    type Documents = NoDocumentAuthority;   // hub's real-time documents bypass this framework entirely —
                                             // hub's own router owns /scopes/{scope}/document/ws because
                                             // only its socket handler carries grant admission, presence
                                             // leases and live revocation.
    type Deciders = HubDeciders;
    type Sagas = HubSagas;
    type Resolvers = HubResolvers;
    type AuthorityStore = HubAuthorityStore;
    type ProjectionStore = HubProjectionStore;
    type BlobStore = HubBlobStore;
    type SessionStore = HubSessionStore;

    async fn open(profile: &StorageProfile) -> Result<InstanceStores<Self>, StorageError> {
        let Some(data_dir) = profile.data_dir() else {
            return Ok(InstanceStores { authority: HubAuthorityStore::ephemeral(), ... });
        };
        let root = PathBuf::from(data_dir);
        create_dir(&root).await?;
        Ok(InstanceStores {
            authority: HubAuthorityStore::open(&root.join("authority")).await?,
            projections: HubProjectionStore::open(&root.join("projections")).await?,
            blobs: HubBlobStore::open(&root.join("blobs")).await?,
            sessions: HubSessionStore::open(&root.join("sessions")).await?,
        })
    }
}

pub enum HubModules { Auth(HubAuthModule), Directory(HubDirectoryModule) }
impl ServerModule for HubModules {
    type Instance = HubInstance;
    async fn manifest(&self) -> ModuleManifest { /* per-variant PolicyTemplate, e.g. "authenticated" auto_apply */ }
    // ...
}
```

**Important finding: only two of hub's subsystems (`auth`, `directory`-command-admission) actually run
through the generic `ServerInstance`/`CommandBus` machinery today.** Real-time document collaboration —
hub's flagship feature — is a separate, pre-existing system. `OfflinePolicy` (the contract's declared
per-command offline behaviour) has **zero references** anywhere in `🌎️hub` or
`🧰️framework/🛍️products/💻️os` — it is defined in the contract but not yet consumed by any client-side
"optimistic replica" implementation in this repo. For proctor, a quiz app has no need for
collaborative-document machinery at all — it is a much closer match to hub's *auth/directory* usage
pattern (plain commands → events → projections) than to hub's document sockets.

**The four server-product storage roles for hub are backed by a hand-rolled append-only JSON-lines file
journal, not SQLite** (`🌎️hub/🗄️stores/🦀️.rs`, 927 lines): `HubProjectionStore` holds
`BTreeMap<String, BTreeMap<String, Vec<u8>>>` folded from a `Journal<ProjectionRecord>` (variants
`Put`/`Checkpointed`/`Cleared`), replayed and compacted on open:

```rust
async fn commit(&mut self, record: ProjectionRecord) -> Result<(), StorageError> {
    self.journal.append(&record).await?;   // durable and fsynced before...
    self.fold(record);                     // ...folded into memory — never the other way round
    Ok(())
}
```

SQLite (via `rusqlite`) is used **only** for hub's separate identity/tenancy **directory**
(`📇️directory/🪶️sqlite/🦀️.rs`, 3175 lines) — not for the generic server-product roles.

### 2.3 SQLite usage in hub (verbatim, `📇️directory/🪶️sqlite/🦀️.rs`)

Module doc:
> "`HubDirectory` over SQLite (rusqlite) — the zero-touch default for local dev and single-user
> self-hosting; no external database service required... Uses synchronous `rusqlite` behind
> `Arc<Mutex<Connection>>`, not an async SQLite driver: a Cargo workspace may link only one native
> `sqlite3` (`links = "sqlite3"`), and `rusqlite` is already the sqlite binding used elsewhere in this
> workspace... Trait methods stay `async fn` ... but their bodies are synchronous rusqlite calls:
> queries are short, the mutex guard is never held across an `.await`."

Connection setup:
```rust
pub async fn connect(path: &str) -> DirectoryResult<Self> {
    let conn = Connection::open(path).map_err(backend)?;
    conn.busy_timeout(std::time::Duration::from_secs(2)).map_err(backend)?;
    conn.execute_batch(SCHEMA).map_err(backend)?;
    // ... reads/writes a `hub_directory_format(schema, version)` singleton row to gate cross-version opens
}
```
Schema is a plain `const SCHEMA: &str = "PRAGMA foreign_keys = ON; CREATE TABLE IF NOT EXISTS ..."`
batch (`hub_user`, `hub_space`, `hub_space_membership`, `hub_document_index`,
`hub_document_descriptor`, `hub_auth_session`, `hub_space_invite`, `hub_directory_event`,
`hub_directory_command_receipt`, plus artifact-CAS/checkpoint tables — ~30 tables total). No `WAL`
pragma is set in the code read; default rollback-journal mode with a 2 s busy timeout. `rusqlite`'s
Cargo feature is `bundled` (statically compiles SQLite — zero-touch, no system package needed on any
OS). Crate-level comment states `rusqlite` is already used elsewhere in the workspace (`vcs`,
`db_storage_sqlite`) — it is the established SQLite binding in this repo. **`bun:sqlite` is not used
anywhere in the repo's server-side Rust/TS products** (the few `bun:sqlite` hits elsewhere are
unrelated `.d.ts`/test-runner typings).

`Cargo.toml` (hub rust package, feature wiring):
```toml
sqlite   = ["dep:rusqlite", "db/sqlite"]
postgres = ["dep:sqlx_core", "dep:sqlx_postgres", "db/postgres"]
neo4j    = ["dep:neo4rs", "db/neo4j"]
rusqlite = { version = "0.38.0", features = ["bundled"], optional = true }
```

### 2.4 Build/run — nx targets, `📜️script.ts`, `.vscode/🧩️launch.seed.jsonc` (verbatim)

`🌎️hub/📦️packages/🦀️rust/📋️project.json` targets include `build`, `build-dev`, `build-dev-postgres`,
`publish`, `test`/`test-quick`/`test-long`/`test-exhaustive`/`test-all-features`, `dev`,
`dev-postgres`, `dev-neo4j`, `dev-secure-suite/-native/-mcp/-admin`, and dozens of feature-specific
`*-check` targets — each one a one-line `nx:run-commands` wrapper: `"command": "bun ./📜️script.ts <verb>"`,
`"cwd": "🌎️hub/📦️packages/🦀️rust"`. This matches AGENTS.md's rule verbatim: *"`project.json` MUST only
call `📜️script.ts <command> <subcommand...> <args>`."*

`.vscode/🧩️launch.seed.jsonc` hub entries (verbatim, selected):
```jsonc
{
  "name": "🛠️dev🗄️os-hub",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun nx run os-hub:dev",
  "cwd": "${workspaceFolder}",
  "env": { "OS_HUB_PORT": "8787", "OS_HUB_DATA": "${workspaceFolder}/.🧬semio/🌐hub/hub-dev/" },
  "presentation": { "group": "3_dev", "order": 387 },
  "serverReadyAction": {
    "action": "openExternally",
    "pattern": "(http://(?:127\\.0\\.0\\.1|localhost|0\\.0\\.0\\.0):8787)",
    "uriFormat": "%s/admin"
  }
}
```
```jsonc
{ "name": "📦️build🗄️os-hub", "command": "bun nx run os-hub:build", "presentation": { "group": "4_build", "order": 206.159 } }
{ "name": "📦️build-dev🗄️os-hub", "command": "bun nx run os-hub:build-dev", "presentation": { "group": "4_build", "order": 206.16 } }
{ "name": "🚚️publish🗄️os-hub", "command": "bun nx run os-hub:publish", "presentation": { "group": "4_build", "order": 206.1601 } }
```
(Plus `🛠️dev🗄️os-hub🐘️postgres`, `🛠️dev🗄️os-hub🕸️neo4j`, `🛠️dev🗄️os-hub🛡️admin`,
`📦️test🗄️os-hub`, `📦️test🗄️os-hub♾️all-features`, compound launches combining hub with `s`, etc. — the
same naming/grouping/ordering convention AGENTS.md requires proctor to follow.)

### 2.5 Deployment: Dockerfile + compose.yaml (production topology, verbatim highlights)

Two-stage build (`rust:1-bookworm` builder → `debian:bookworm-slim` runtime): builds the admin SPA and
the release binary with `bun ./📜️script.ts build` (never `nx run` inside the image — "the Nx
project-graph pass costs minutes in a cold container for nothing"), copies **only** the binary and the
admin SPA `dist` into the runtime image, runs as a non-root `semio` user under `tini`, one writable
volume (`/srv/semio-hub/data`), `HEALTHCHECK` against `/readyz` with a forced
`X-Forwarded-Proto: https` header, `ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/os-hub"]`.

`compose.yaml`: one `hub` service published on `127.0.0.1:8787:8787` only (reverse proxy expected on
the host), `stop_grace_period: 30s`, `OS_HUB_STORAGE_BACKEND`/`OS_HUB_DIRECTORY_BACKEND` env vars
select `fs`/`sqlite`/`postgres`/`neo4j` independently; optional `postgres`/`neo4j` services behind
compose `profiles`. Whole state lives on one named volume (`hub-data`), which is the entire backup unit
(`tar`, hub stopped — no online snapshot).

Production-mode gating (`README.md` "Read this first" table) requires **three** explicit statements to
bind a non-loopback interface: `OS_HUB_MODE=production`, `OS_HUB_ALLOWED_ORIGINS=…` (CORS allowlist —
wildcard is impossible together with credentialed CORS), `OS_HUB_TRUSTED_FORWARDING=proxy` (then the
hub enforces `X-Forwarded-Proto`, refusing cleartext with `403 x-semio-refusal: insecure-transport`
before any handler runs). **No TLS in-process ever** — a TLS-terminating reverse proxy is mandatory; the
README ships ready Caddy and nginx configs (with the WebSocket `Upgrade`/`Connection` headers, long
read timeouts for idle collaboration sockets, `client_max_body_size` for uploads). A first user is
seeded out-of-band with `os-hub credential set --email … --display-name …` (password on stdin) —
**there is no sign-up route**; `OS_HUB_CREDENTIAL_SIGN_IN=true` then enables `POST /auth/sessions`.

### 2.6 What is reusable for a much smaller proctor, and what is not

**Directly reusable pattern (not code):** the `ServerInstance`/`ServerModule` composition shape itself
— `HubInstance` is proof that composing this framework into a small, focused set of modules (proctor
would need something like `QuizModule` with a handful of deciders) plus a durable backend behind the
four storage traits is exactly how the framework wants to be used, and is *decoupled* from all of hub's
document/artifact/plugin/inference machinery.

**Reusable almost verbatim:**
- `axum`+`tokio` gateway, `ServerBuilder`, `base_router`, CORS middleware
  (`cors_middleware`/`apply_cors_headers` — hand-rolled, reflects origin, six headers, no dependency).
- `PolicyEngine`/`AdminGate`/`ResolverChain<R: PrincipalResolver>` (roles-as-data, closed-by-default).
- The Dockerfile/compose *shape*: multi-stage build → binary-only runtime image → one named volume →
  `tini` entrypoint → `HEALTHCHECK` → reverse-proxy-in-front posture; the Caddy/nginx snippets.
- `🌎️hub/README.md`'s environment-variable-driven, no-config-file convention
  (`std::env::var` directly, documented defaults, boot-time validation with a named refusal).

**Not worth reusing for proctor (too coupled/too large):**
- Hub's SQLite directory schema and its parallel, hand-rolled event-sourcing scheme
  (`hub_directory_event`/`hub_directory_command_receipt`) — this duplicates what the generic
  `AuthorityStore`/`ProjectionStore` traits already give for free; proctor should implement *those*
  traits directly rather than copy hub's directory pattern.
- Hub's document websocket, presence/colour-lease system, artifact-authority/plugin/inference
  subsystems, admin SPA — none of it is needed for a quiz app.
- `os-hub`'s three-database-backend flexibility (sqlite/postgres/neo4j) — the task calls for **one**
  SQLite database; carrying that flexibility in from day one is unnecessary generality AGENTS.md would
  flag ("You MUST NOT be pragmatic" cuts both ways — don't build unneeded options either).

---

## 3. Client side — command submission, query subscriptions, event replication, offline

### 3.1 The one generic TS client that speaks this contract

`🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🟦️.ts` (`@semio-tech/framework-server`) is
the *only* TypeScript client for the `CommandEnvelope`/`QueryEnvelope` wire contract in the repo (see
§1.8 verbatim `ServerClient` methods). It is transport-agnostic behind an owned `HttpTransport`
interface (`fetchTransport(baseUrl)` adapts the platform `fetch`), so proctor's browser client, a CLI,
or a test harness all share one typed surface:

```ts
export class ServerClient {
  constructor(transport: HttpTransport, credential: ServerCredential = {});
  async instance(): Promise<ServerInstanceDefinition>;
  async submitCommand(envelope: CommandEnvelope): Promise<CommandOutcome>;
  async query(envelope: QueryEnvelope): Promise<QueryResult>;
  async publishEphemeral(frame: EphemeralFrame): Promise<number>;
  async events(actor: ActorKey, since = 0): Promise<EventRecord[]>;
  eventStreamUrl(baseUrl: string, actor: ActorKey, since = 0): string;   // durable lane WS, replay-then-live
  documentSocketUrl(baseUrl: string, scope: Scope, join: DocumentJoin = {}): string;
  async blob(hash): Promise<Uint8Array>; async hasBlob(hash): Promise<boolean>; async putBlob(hash, bytes): Promise<BlobReceipt>;
}
```

### 3.2 Event replication / reconnection ("short connection shortage")

The **durable event lane already implements exactly the resume semantics proctor needs**: `GET
/actors/{tenant}/{kind}/{id}/events?since=N` (HTTP page) and `GET
/actors/{tenant}/{kind}/{id}/events/ws?since=N` (websocket) both replay from `AuthorityStore
::events_since` then hand off to the live `Fanout` broadcast, deduplicated by `seq` at the
replay/live seam — gateway doc comment: *"so a reconnecting client sees every fact exactly once."* A
client that stores the last `seq` it applied per actor and reconnects with `since=<that seq>` survives
a short outage without any special-case code. `Subscription::recv` on the server side silently skips
frames a lagged receiver missed on the *ephemeral* lane only (lossy by design); the *durable* lane
never drops — it is re-derived from storage on reconnect.

### 3.3 Command submission under a short outage — mostly unbuilt, by design

`OfflinePolicy` (`Optimistic`/`Deferred`/`AuthorityRequired`, declared per command kind in
`CommandDescriptor`) is defined in the contract as the seam for this, but **no client-side optimistic
replica or offline outbox consuming it exists in the repo today** (confirmed: zero references to
`OfflinePolicy` outside `🧰️framework/🛍️products/🖥️server` itself). `CommandEnvelope.idempotency_key`
plus the exactly-once law (§1.4) is the mechanism that makes client-side retry *safe*; the retry loop,
local pending-command queue, and `expectedRevision`-based rebase-on-`Transformed` handling are
application-level work proctor must build itself (small — see §6). This is consistent with AGENTS.md's
local-first requirement ("support short connection-shortages and not freeze the app... should NOT
accept long offline periods") — the primitives (idempotency key, revision fencing, `Transformed`
outcome, replay-since-seq) are all present; only the browser-side retry/queue glue is missing.

### 3.4 Identity/session (auth module) — hub-specific, not part of the generic framework

Hub's `🔐️auth/` module (session bearer resolution, password credentials, rate limiting) is a
`ServerModule` implementation specific to hub's identity model (email+password, admin subjects,
capability tokens for shares/invites) — it is *not* part of `🧰️framework/🛍️products/🖥️server` and is
not reusable wholesale. What proctor should reuse is the **shape**: a `PrincipalResolver` rung per
identity kind (anonymous passthrough, a pseudonym-cookie rung, a name+optional-password rung),
composed into one `ResolverChain`, exactly as `HubResolvers` closes hub's rungs. No OAuth/SSO exists in
this repo's server products.

### 3.5 Other client packages found

No other product implements a generic client against this contract. `🧰️framework/🛍️products/💻️os` has
its *own*, much larger client/replica stack (`🧰️framework/🔨️modules/📡️replication`,
`⚔️conflict`, `🎮️mutation`) for real-time collaborative documents — this is a different, older
system (predates or runs beside the CQRS framework; hub's documents use it directly, bypassing
`ServerInstance::Documents`) and is **not relevant to proctor's needs** (a quiz app submits discrete
commands and reads projections; it does not need concurrent multi-cursor document editing).

---

## 4. Schema-first: where schemas live, and how types are "generated"

**Two different schema-first strategies coexist in this repo — important not to conflate them:**

1. **`🧰️framework/🛍️products/🖥️server`'s own contract** is *not* JSON-Schema-driven. It is hand-written
   directly as Rust `serde` structs/enums (`🧬️contract/🦀️.rs`) with a **hand-maintained TypeScript
   twin** (`🟦️.ts`), and the two are held honest only by the language-agnostic fixture
   `🧫️fixtures/🔌️wire/🔣️.json` (one JSON vector per wire type + the route table) round-tripped by both
   sides' test suites (§1.9). There is no code generator here — a rename on either side is caught by a
   failing test, not prevented by a generator.

2. **Hub's domain modules** (`🔐️auth`, `📇️directory`, `📊️observability`, `🚧️refusal`,
   `🗿️artifact-authority/…`, `💡️inference/🧬️schema`, etc.) each carry a `🧬️schema/🔣️.json` — a real
   **JSON Schema (draft-07)** document, e.g. `🌎️hub/🔐️auth/🧬️schema/🔣️.json`:
   ```json
   {
     "$schema": "http://json-schema.org/draft-07/schema#",
     "$id": "https://json.schemas.assets.semio-tech.com/hub/auth/schema.json",
     "$defs": {
       "CredentialSignInRequestV1": {
         "x-semio-formats": ["🔣️jsonschema", "🦀️rust", "🟦️typescript"],
         "type": "object", "additionalProperties": false,
         "required": ["schema", "email", "password", "deviceInstanceId", "clientClass"], ...
   ```
   The `x-semio-formats` array names every representation that must exist for a type (JSON Schema
   itself, Rust, TypeScript — sometimes also GraphQL/protobuf, e.g. under
   `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🗂️map/🧬️schema/` which ships `.graphql` and
   `.proto` siblings too). **This is not codegen either.** A repo-wide "law" checker
   (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/…/⚖️laws/📏️field-parity/🟦️.ts` and
   siblings `🏷️type-name-parity`, `💾️state-parity`, `🔺️diff-coverage`, `💧️state-purity`,
   `🪞️config-fidelity`) treats the JSON Schema leaf as **normative** and flags any hand-written
   Rust/TypeScript/GraphQL/protobuf representation whose field names, optionality or cardinality
   disagree with it. Doc comment (verbatim): *"Compares every representation with normative JSON
   Schema fields and expressible shapes."* So: **author the JSON Schema first, hand-write each
   language's type to match it, and a linter (not a compiler step) enforces parity** — this is what
   AGENTS.md's "schema-first" rule maps onto operationally in this codebase.

For proctor, pattern 2 (a real `🧬️schema/🔣️.json` per command/event payload, with `x-semio-formats:
["🔣️jsonschema", "🦀️rust", "🟦️typescript"]`, hand-written Rust structs behind the opaque
`CommandEnvelope.payload`/`EventRecord.payload` bytes, and a TS twin for the browser client) is the
established, repo-consistent choice — not pattern 1, which is specific to the framework product's own
transport contract.

---

## 5. Tests: language-agnostic tests, harness, fixtures

**Framework `🖥️server` product:**
- `🧪️tests/🔬️wire/🦀️.rs` + `🧪️tests/🔬️wire/🟦️.ts` — the cross-language gate over
  `🧫️fixtures/🔌️wire/🔣️.json` (§1.9); registered from `🟦️.ts` via
  `registerServerWireTests(import.meta.vitest, {...exports...}, {directory, url})`, i.e. Rust-side
  fixtures are consumed through Bun's inline `import.meta.vitest` pattern, not a separate test file.
- `🧪️tests/🔬️conformance/🦀️.rs` — the four storage roles' contracts as `async fn(&mut impl XStore)`,
  generic over backend (§1.5); gated behind `#[cfg(any(test, feature = "conformance"))]` so a
  downstream durable backend can enable it as a dev-dependency feature.
- `🧪️tests/🧩️instance/🦀️.rs` — the reference `TestInstance` (§1.9), doubling as the worked
  composition example and as what the crate's own `🔨️modules/*/🧪️tests/🔬️unit/🦀️.rs` unit suites run
  against.
- `🧪️tests/🔒️closed-ports/🦀️.rs` — a `[[test]]` in `Cargo.toml` that links the crate the way an
  instance does, to gate the `dyn_enum_close!` recipe for every port.
- Run via `bun ./📜️script.ts test` (Rust) and `bun ./📜️script.ts test` (TS, i.e. `vitest` against
  `🧪️tests/🎚️config/🟦️.ts`, `includeSource: ["../../🟦️.ts"]` — inline-source tests, not a separate
  `__tests__` tree).

**Hub:** `🌎️hub/🧪️tests/` holds ~25 permanent suites (`.ts` orchestration, `.rs` unit/standalone) —
notably `🤝️two-client-document` (two-client relay over one document), `💾️backup-restore` (drives the
literal tar/stop/restore/diff cycle documented in the README), `🐳️docker-image` (the
`docker-image-build`/`docker-image-check` verbs referenced in the README's "Container image" section),
`🌅️boot-watch`, `📊️observability`, `🛡️access-policy`. Every check is a permanent Nx target
(`os-hub:<name>-check`) invoked through `📜️script.ts`, never an ad-hoc script — matches AGENTS.md's
"MUST implement all permanent scripts in `📜️script.ts`" exactly.

**Language-agnostic-test discipline** (AGENTS.md: "create the same output of a test with at least one
third-party library in order to validate our own implementation") is what the wire fixture + dual test
suites (§1.9) and the JSON-Schema-parity laws (§4) both implement structurally: one canonical JSON
document, independently consumed and asserted by each language's own test runner.

---

## 6. Recommendation for `teaching/proctor`

### 6.1 Language and structure

**Build proctor in Rust**, as a new `ServerInstance` composed exactly like `HubInstance` (§2.2), living
at (repo convention) `🧰️framework/🛍️products/🖥️server` stays untouched; proctor is its own product,
e.g. `🧰️framework/🛍️products/🎓️proctor` or a top-level `🎓️proctor/` sibling to `🌎️hub`, with
`📦️packages/🦀️rust` (bin) + `📦️packages/🟦️typescript` (thin browser client, if a bundled quiz UI is
served) + `📜️script.ts`/`📋️project.json` per AGENTS.md.

Rationale: the generic server product is Rust-only and axum/tokio-only (§1.8); there is no TypeScript
server runtime to build on, and introducing one would mean reimplementing the CQRS bus, actor turn
protocol and policy engine in a second language — a large, unjustified duplication AGENTS.md's
"multi-implementation… if code is repeated, it MUST be close to each other" argues against for a
product this small. `@semio-tech/framework-server` (TS) is reused as-is for the browser quiz client
(§3.1) — no rewrite needed there.

### 6.2 Modules and deciders

One or two `ServerModule`s, following `HubModules`'s shape (§2.2):
- A `QuizModule` contributing deciders per actor kind — plausibly `quiz-run` (one actor per
  participant's attempt: start, answer, submit, resume — `Decision::Emit` on each), `quiz` (the
  authored quiz definition/config, admin-only commands), and a `leaderboard` saga reacting to
  `run.submitted`/`run.scored` events to update badge/score projections. Keep each `Decider::decide`
  pure per §1.4 — score computation belongs in `decide`, not as an ambient side effect.
- Identity: three `PrincipalResolver` rungs (anonymous passthrough → cookie-bound pseudonym → optional
  name+PIN), composed in a `ResolverChain`, mirroring `HubResolvers` (§3.4). No passwords required for
  the base "no passwords" identity model; a lightweight per-device bearer cookie minted on first visit
  covers "continue on another device" if the participant is given a short resume code/link — simplest
  answer under "no passwords."
- Queries/projections: a `QueryHandler` per read (leaderboard page, own-run status, badge list) reading
  through `ProjectionStore`, folded by the saga/projector off committed events — never computed
  ad-hoc from the authority store at query time.

### 6.3 Storage: implement the four traits directly over one SQLite file

Do **not** copy hub's split design (file-journal for server-product roles + separate SQLite directory
with its own event scheme, §2.2/2.3). For proctor's "single SQLite database" requirement, implement
`AuthorityStore`, `ProjectionStore`, `BlobStore`, `SessionStore` **all four directly against one
`rusqlite::Connection` (or `Arc<Mutex<Connection>>`, following hub's own concurrency note in §2.3)** —
one file, tables named by role (`proctor_command_receipt`, `proctor_event`, `proctor_snapshot`,
`proctor_outbox`, `proctor_projection`, `proctor_blob`, `proctor_session`), `busy_timeout` set, `rusqlite`
`bundled` feature (zero-touch on devcontainer/Windows/macOS/Linux alike, matching AGENTS.md's
cross-platform rule and the repo's established binding, §2.3). This is a small amount of code (hub's
own file-journal equivalent is under 1000 lines for all four roles) and lets the crate's `conformance`
dev-dependency feature (§1.5) hold it to the exact same contract tests hub and the in-memory reference
are held to — satisfying AGENTS.md's "language-agnostic test… validate our own implementation" via the
mechanism the framework already built for this purpose. Enable `WAL` mode explicitly (`PRAGMA
journal_mode=WAL`) for better concurrent read/write behaviour than hub's directory store currently sets
(§2.3 notes hub does not set it) — worth doing since proctor's read (leaderboard) and write (command)
paths will be concurrent under real classroom load.

### 6.4 Schema-first payloads

For every command/event/query payload (opaque `Vec<u8>`/`Uint8Array` at the contract layer), add a
`🧬️schema/🔣️.json` (JSON Schema draft-07) per §4 pattern 2, tagged `x-semio-formats: ["🔣️jsonschema",
"🦀️rust", "🟦️typescript"]`, with hand-written Rust structs (serde) and a TS twin for the browser
client — this is what makes the repo's field/type-name/state-parity laws (§4) apply to proctor
automatically once it is discovered under the taxonomy.

### 6.5 Deployment for `quizze.architektur-und-technologie.de`

Copy hub's proven shape (§2.5), stripped to what proctor needs:
- Two-stage `Dockerfile` (`rust:1-bookworm` → `debian:bookworm-slim`), non-root user, one named volume
  for the single SQLite file + its WAL/SHM siblings, `tini` entrypoint, `HEALTHCHECK` against a
  `/readyz`-equivalent.
- `compose.yaml`: one service, `127.0.0.1:<port>:<port>` published (never publish the raw port to the
  internet), `stop_grace_period` long enough to let an in-flight quiz submission finish, no
  postgres/neo4j profiles (out of scope — one SQLite file was the explicit ask).
- Reverse proxy: adapt the repo's own Caddy example (§2.5) — trivially, since Caddy's automatic TLS is
  the least-touch option for a single custom domain:
  ```caddyfile
  quizze.architektur-und-technologie.de {
      reverse_proxy 127.0.0.1:<port> {
          header_up X-Forwarded-Proto {scheme}
      }
  }
  ```
  If proctor follows hub's `OS_HUB_TRUSTED_FORWARDING=proxy`-style pattern (enforce
  `X-Forwarded-Proto`, refuse cleartext), this is a direct copy of an already-battle-tested posture
  (hub's README records a real run of exactly this refusal behaviour, §2.5).
- Backup: same "stop, tar the data directory, restart" discipline as hub (§2.5) — trivial for proctor
  since it is one file plus WAL/SHM, not a multi-store tree.

### 6.6 Tradeoffs

- **Rust-only is the biggest constraint** but is not really a choice — it is what "reuse the existing
  framework" means here, since the server product has no other runtime. The payoff is reusing the
  entire CQRS bus, actor turn protocol, policy engine, gateway/CORS/websocket plumbing, and the
  conformance test suite for free; the cost is that whoever implements proctor must be comfortable
  with the `impl Future<...> + Send` / `dyn_enum`/`dyn_enum_close!` idiom used throughout layer 2
  (§1.4–1.6) — a real but bounded learning cost, and the `TestInstance` in §1.9 is a working, compiling
  template to copy from directly.
- **Implementing the four storage traits over SQLite from scratch** (rather than reusing hub's file
  journal) is more upfront work than pointing at an existing backend, but hub has no reusable *generic*
  SQLite backend for these four roles to import (its SQLite code is directory-specific, §2.2); writing
  one is the only way to land on "a single SQLite database" as asked, and the conformance suite
  (§1.5) makes this low-risk.
- **No client-side offline command queue exists to reuse** (§3.3) — proctor must build a small
  pending-commands-in-`localStorage`/IndexedDB queue keyed by idempotency key, retried on reconnect.
  This is genuinely new work, but it is small (the hard part — exactly-once on the server side — is
  already handled by the framework) and is squarely inside AGENTS.md's "progress and cancellation for
  expensive operations" / "short connection-shortages" requirements, so it has to be built regardless
  of what is reused.
- **Skipping hub's directory/auth subsystem** means proctor reimplements a minimal identity ladder
  rather than getting user accounts "for free" — but hub's identity model (password accounts, admin
  subjects, invites) is the wrong shape for "anonymous / pseudonym / name, no passwords" anyway; reuse
  would have cost more (stripping features) than the from-scratch resolver chain costs to write.
