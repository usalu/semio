# P1d — Eliminating Unbounded Ad-Hoc Thread Spawning

Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/` (whole db crate family), the store `🔄️sync`
component, the `pack_http` range-fetch component, and `🌎️hub`'s `semio-hub` binary — per the P1d
packet's ownership boundary. Read first: `📓️p1a-worker-pool.md` (this folder) for the `WorkerPool`/
`Lane`/`PermitLedger` API this packet builds on.

## Summary

| # | Site (Phase 0 census) | Before | After |
|---|---|---|---|
| 1 | `db_storage::run_blocking_op` (the FsStorage/SqliteStorage blocking bridge) | Called `HostAsyncRuntime::run_blocking` (removed by P1a) | Submits to `WorkerPool::submit(Lane::Io, ..)`; `pool: None` (frozen single-shot entry points only) resolves inline |
| 2 | `db_engine::ArtifactHandle::submit` — `"db-engine-submit-bridge"` | **One brand-new OS thread spawned on EVERY submit() call** (not per-document — per mutation) | `WorkerPool::submit(Lane::Io, ..)` when `Database::with_pool(..)` was called; inline fallback otherwise |
| 3 | `store::sync::ArtifactActor` (`"sync-actor-{doc_id}"`) | One dedicated OS thread **+ one embedded `tokio::runtime::Builder::new_current_thread()`** per open document | One shared "sync-actor-supervisor" OS thread **per `ArtifactHost`** (not per document), running every open document's actor as a `tokio::task::spawn_local` on a `LocalSet`, driven by the caller's own ambient `tokio::runtime::Handle` (no second reactor) |
| 4 | `pack_http::UreqRangeTransport::fetch_range` | `std::thread::spawn(..).join()` per HTTP range request (thread AND a synchronous block on it) | `WorkerPool::submit(Lane::Io, ..)` when constructed via `with_pool(..)`; `new()` (no pool) resolves inline |
| 5 | `🌎️hub`'s `HubDbRuntime` | A `HostAsyncRuntime` impl that existed for exactly one method, `run_blocking` (E0407 after P1a's removal) | Deleted entirely; hub now owns one process-wide `WorkerPool` (`hub_worker_pool()`, lazily constructed, `ProcessKind::HeadlessBatch`) wired into `Database::with_pool(..)` and `SqliteStorage::open(Some(pool), ..)` |
| 6 | `db_actor::StdThreadSpawner` (census #18, `db-actor-g{generation}`) | One dedicated thread per `Supervisor::spawn_slot` call | **Left unchanged** — see "Not restructured" below |
| 7 | `db_artifact::ArtifactAuthority::spawn` (census #17, `"db-document-actor"`) | One dedicated, long-lived OS thread per open document (`ArtifactEngine`'s the actor owns is genuinely `!Send`) | **Left unchanged** — see "Not restructured" below |

Every `run_blocking`-shaped compile error P1a's report flagged for this packet's boundary is fixed,
plus several more the async API change (or long-pre-existing bugs it finally exposed once the db
crate started compiling) surfaced. `bun ./📜️script.ts verify dependencies`: 238 → 238 (clean — the
new `semio-framework-async`/`semio-framework-trace` deps on `db`/`pack` are workspace-internal).

## 1. `db_storage::run_blocking_op` — the FsStorage/SqliteStorage blocking bridge

**File:** `🛢️db/🗄️storage/🦀️component.rs`

Before: `run_blocking_op<T, F, R: HostAsyncRuntime>(runtime: &R, scope: &ScopeHandle, work: F)` called
`runtime.run_blocking(scope, ctx, Box::new(...))`. `FsStorage<R>`/`SqliteStorage<R>` carried an
`Arc<R>` + `ScopeHandle` field pair whose ONLY use, confirmed by grep across both files, was feeding
this bridge (plus one test-only `open_scope` call) — so `R: HostAsyncRuntime` was structurally dead
weight everywhere it appeared once `run_blocking` left the trait.

After:
```rust
static BLOCKING_QUEUE: semio_framework_trace::QueueCounter = semio_framework_trace::QueueCounter::new();

pub(crate) async fn run_blocking_op<T, F>(pool: Option<&WorkerPool>, work: F) -> T
where T: Send + 'static, F: FnOnce() -> T + Send + 'static {
    match pool {
        Some(pool) => {
            let (tx, rx) = oneshot::<T>();
            BLOCKING_QUEUE.enqueued(0);
            pool.submit(Lane::Io, Box::new(move || { let result = work(); BLOCKING_QUEUE.dequeued(0); tx.send(result); }));
            rx.await
        }
        None => work(),
    }
}
```
`FsStorage`/`SqliteStorage` are now **non-generic** (`R` deleted entirely, not just its trait bound)
with a `pool: Option<Arc<WorkerPool>>` field instead of `runtime`/`scope`. `InlineRuntime` (the
"single-threaded caller, run inline" `HostAsyncRuntime` impl `db_cli`/`Database::open_at` used) is
**deleted** — its whole reason to exist was feeding this bridge, and `pool: None` now does exactly
what it did (`open_inline`/`open_at` open with `pool: None`, `db_cli` unchanged in behavior).

**Ripple**: `DbBackend<R>`, `WalRef<'a, R>`/`SnapshotRef`/`PayloadRef`/`CatalogRef`/`IndexRef`/
`LeaseRef`, `FaultStorage<R>` (testkit), `Compactor<'storage, R>`, `replicate_document<R>` (cluster),
`handle_hello<R>` (db-level sync), `replay_history<R>` (engine), `ArtifactEngine<R,A,V>` (artifact),
`ArtifactAuthority::spawn<R,A,V>` — **every one** of these carried `R: HostAsyncRuntime` PURELY to
name `DbBackend<R>`, and every concrete instantiation anywhere in the crate was already
`DbBackend<InlineRuntime>`. All now non-generic (`DbBackend`, `ArtifactEngine<A, V>`, etc.) — a real
simplification, not just a mechanical unblock. `Postgres`/`Neo4j` backends never had `R` at all (they
drive `sqlx`/`neo4rs`'s own genuinely-async I/O directly, no blocking bridge needed).

**Postgres/Neo4j backends — same defect, different shape**: both had a batch of small pure-logic
helpers (`to_i64`, `validate_read_range`, `validate_truncate`, `lease_acquire_decision`,
`lease_renew_check`, `lease_release_check`, `postgres_capabilities` in the postgres backend;
`encode_bytes`, `decode_bytes`, `u64_to_i64`, `i64_to_u64`, `slice_range`, `apply_append`,
`apply_truncate`, `decide_acquire_fence`, `validate_renew`, `validate_release` in the neo4j one)
marked `async fn` with no genuine `.await` in their bodies, called with `?`/no-await from real async
trait methods — the same shape of bug as `db_storage_sqlite`'s `to_sql_i64`/`lock` (see below), just
never exercised until this packet made the whole crate compile far enough to reach them. All
converted to plain `fn`.

## 2. `db_storage_sqlite`'s own pre-existing bugs (not run_blocking-shaped, found while fixing it)

**File:** `🛢️db/🗄️storage/🪶️sqlite/🦀️component.rs`

- `to_sql_i64`/`lock` were `async fn` with no `.await` inside them, called from `run_blocking_op`'s
  SYNC closures (`move || { .. }`) — illegal (`.await` isn't even syntactically legal inside a
  non-async closure) and the actual proximate cause of the "54 errors → 2 errors, fluctuating"
  behavior P1a's report flagged as unattributable. Converted to plain `fn` (fixes ~30 call sites at
  the two definitions, not per call site).
- `SqliteStorage::open`/`open_in_memory` called `Self::init(..)` (an `async fn`) without `.await` —
  fixed.
- The entire `#[cfg(test)] mod tests` was missing `.await` after `poll_once(..)`/`block_on_ready(..)`
  at essentially every call site (73+ occurrences) — this test module could not have compiled in a
  very long time. Rewrote it: `sqlite_scratch`/`fs_scratch`-style helpers now open with `pool: None`
  (inline resolution, matching `db_storage::FsStorage`'s own test convention) and every `.await` is
  present.

## 3. `db_engine::ArtifactHandle::submit` — the worst single offender

**File:** `🛢️db/⚙️engine/🦀️component.rs`

This was the highest-frequency site in the whole census: not one thread per document, but **one
`"db-engine-submit-bridge"` OS thread spawned on every single `submit()` call** — i.e. per mutation.
`Database<A,E>` gained a `pool: Option<Arc<WorkerPool>>` field (`None` by default) and a public
`Database::with_pool(pool: Arc<WorkerPool>) -> Self` builder; `ArtifactHandle` inherits it at
construction (`register_handle`/`document()`). `submit()`'s closure — `authority: Arc<ArtifactAuthority>`,
`batch`, `options`, `document`, `submitted_at_ms`, `reply_tx`, all plain owned `Send` types — now goes
straight to `pool.submit(Lane::Io, ..)` when a pool is set, or runs inline (same shape as
`run_blocking_op`) when it isn't. `open_at`'s frozen single-shot contract (`db_cli`, this crate's own
tests) is unaffected by design — those callers never call `with_pool`, so `submit()` still resolves
synchronously for them, identical to before. **Any REAL concurrent caller — hub, a future
renderer/mcp integration — needs to call `.with_pool(pool)` right after `open`/`open_at` to get the
backgrounding.** Hub now does; renderer/mcp are outside this packet's boundary (see "Cross-boundary").

## 4. `store::sync`'s per-document actor — the worst *design* offender

**File:** `🏪️store/🔄️sync/🦀️component.rs`

Before: `ArtifactHost::open` → `native_actor::spawn_actor` built a brand-new
`tokio::runtime::Builder::new_current_thread().enable_all().build()` and ran it on a brand-new
`std::thread::Builder::new().name("sync-actor-{doc_id}")` thread, for EVERY open document. N open
documents = N OS threads + N embedded reactors, each duplicating the timer/IO driver work the
process's real ambient tokio runtime already does.

**First attempt (reverted): plain `tokio::spawn`.** `ArtifactActor::run`'s `tokio::select!` loop looks
non-blocking, so the obvious fix is `tokio::spawn(async move { .. })` on the calling task's own
ambient runtime — no dedicated thread at all. This does not compile: `ArtifactActor::run`'s future is
genuinely `!Send` (it calls through `os_vcs`'s `Box<dyn Future>` trait objects, bundled `rusqlite`'s
non-`Sync` `Connection`, and `&dyn Fn` closures captured across `.await` points in `spr::history`) —
`tokio::spawn` requires `Send`. This is the same shape of wall `db_artifact::ArtifactAuthority` hits
(next section), for the same underlying reason (non-`Send` state deep in the call graph).

**Landed design: one shared "supervisor" thread per `ArtifactHost`, not one per document.**
`ArtifactHost` now owns a `native_actor::SupervisorHandle` (`Arc<Mutex<Option<UnboundedSender<SpawnRequest>>>>`),
lazily started on the first `open()` call. The supervisor thread runs a `tokio::task::LocalSet`
(the one primitive that runs `!Send` futures on a tokio runtime at all — it just needs one pinned
thread to do it), driven by `Handle::block_on` against a `tokio::runtime::Handle` **captured from the
caller's own ambient runtime** before the thread spawns — so it reuses that runtime's I/O
driver/timer wheel instead of building a redundant one. Opening a document now sends a `SpawnRequest`
(the plain owned, `Send`-safe construction inputs: `config`/`remote`/`cmd_rx`/`events`) across a
channel into the supervisor, which `spawn_local`s the actor there. Result: an `ArtifactHost`'s OS
thread cost is **O(1) regardless of how many documents are open on it**, not O(open documents) — the
actual defect the Phase 0 census flagged for this site.

`ArtifactHost::close()` also changed: it used to call `JoinHandle::join()` on the actor's thread,
**blocking the calling thread** (dangerous inside `impl Drop`, and a direct violation of "must survive
short connection shortages without freezing the app"). It now only sends `Detach` and returns — the
actor's own `run()` loop breaks out and flushes on that message, and a `tokio::task::JoinHandle`
(unlike an OS `std::thread::JoinHandle`) keeps running to completion after its handle is dropped, so
nothing is lost, just no longer awaited synchronously by the caller. `ArtifactHost::drop` similarly
drops the supervisor's sender (closing its channel, ending its loop) rather than joining a thread.

## 5. `pack_http::UreqRangeTransport` — HTTP range-fetch threads

**File:** `🔨️modules/🎒️pack/🌐️http/🦀️component.rs`

`fetch_range` used to `std::thread::spawn(..).join()` — one thread per range request, AND a
synchronous block on it right inside the `async fn` body (so it never even got the backgrounding
benefit an `async fn` signature implies). `UreqRangeTransport` gained a `pool: Option<Arc<WorkerPool>>`
field (`new()` → `None`, new `with_pool(pool)` → `Some`), and a small hand-rolled oneshot bridge
(mirroring `db_storage`'s own — this crate names no `tokio`/`futures` executor either, matching its
"no concrete HTTP client type in a public signature" discipline extended to executors). `fetch_range`
now submits to `Lane::Io` and genuinely `.await`s a receiver when a pool is set, or runs inline
otherwise. `semio-framework-async` added as a plain (non-optional) dependency of `semio-framework-pack`
— workspace-internal, `verify dependencies` confirms 238→238.

**Not addressed**: the OTHER thread site the Phase 0 census listed for this file (line ~189, inside a
private `sleep()` retry-backoff helper used by `fetch_with_retry`) is a genuine one-shot timer thread,
not a per-request fetch thread — lower frequency (fires only on transient-failure retry, bounded by
`RetryPolicy::max_retries`, default 3) and lower impact than the per-request site above. `WorkerPool::timer()`
(a `TimerWheel`) is the natural replacement primitive, but threading a `pool` reference into the
generic (non-`ureq`-specific) `sleep()`/`InnerSource` retry path — used by every `RangeTransport`
implementor, not just the native one — needs a slightly different plumbing than `UreqRangeTransport`'s
and ran out of this packet's time budget. Left as a known remaining site, not a silent gap.

## 6. `🌎️hub`'s `HubDbRuntime`

**File:** `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`

`HubDbRuntime` was a `HostAsyncRuntime` impl whose doc comment already said its only real reason to
exist was `run_blocking` ("hub needs exactly one capability off this trait"). With that method gone
from the trait (P1a) and `SqliteStorage` no longer generic over `R: HostAsyncRuntime` at all (§1),
`HubDbRuntime` became fully redundant — deleted (struct + impl, ~40 lines). Replaced with
`hub_worker_pool()`, a `OnceLock`-backed process-wide `WorkerPool` (`ProcessKind::HeadlessBatch`,
sized to `std::thread::available_parallelism()`), wired into every `connect_db` backend branch via
`SqliteStorage::open(Some(pool), ..)` and `Database::with_pool(pool)` (the `fs`/`postgres`/`neo4j`
branches all call `with_pool` too, even though postgres/neo4j's storage itself needs no pool —
`ArtifactHandle::submit`'s bridge benefits regardless of backend). While fixing this I also fixed the
`postgres`/`neo4j` branches' pre-existing type mismatch (they passed a bare `PostgresStorage`/
`Neo4jStorage` where `Database::open` expects `Arc<DbBackend>` — now wrapped in
`DbBackend::Postgres(..)`/`DbBackend::Neo4j(..)`) and several missing-`.await` bugs INSIDE the
`connect_db` function itself (`Database::open_at(..)?` / `Database::open(..)?` / `SqliteStorage::open(..)?`
were all missing `.await` before `?` — same "async fn called without await" defect family as §2).

## Cross-boundary: `🌎️hub`'s pre-existing, unrelated breakage

`cargo check -p semio-hub --features sqlite,postgres,neo4j --all-targets` still reports 48 errors
across 19 distinct call sites — but every one of them is **outside the ~90-line region this packet
touched** (verified via `git diff --unified=0`, hunks confined to the `HubDbRuntime`/`connect_db`
region). They're the identical "async fn called without `.await`" defect family as §2/§6, just spread
through completely unrelated request handlers: `state.db.catalog().artifacts` (missing `.await`),
`state.db.document(&id)` matched as a bare `Result` (missing `.await`), `handle.frontier().map_err(..)`,
`db::document::CommandBatch::new(..)` matched as a bare `Result`, `state.db.storage().wal()`/`.payload()`,
`decode_client_frame(..).ok()`, `decode_server_frame(..).expect(..)`, `os_directory::fold_all(..)`, and
more (`📦️bin.rs` lines 378, 409, 425, 437, 544-545, 610, 642, 847, 890, 1224, 1286-1287, 1702, 1784).

This is NOT thread-ownership/blocking related — it looks like hub's request handlers were written
against an older, differently-shaped (more synchronous) version of `db`'s facade API and never
updated when those methods became `async fn`, and it was invisible until now because `db` itself
didn't compile (blocking hub's own build from ever running far enough to surface it) until this
packet's fixes landed. Recommend a separate, dedicated ticket — it's a substantial, correctness-
sensitive rewrite of hub's request handlers, unrelated to threads, and not something to rush inside
this packet's remaining budget.

## Deliberately NOT restructured

### `db_actor::StdThreadSpawner` / `Supervisor` (census #18)

Read the full picture via research before touching anything: `StdThreadSpawner::spawn` (line 730,
`🎭️actor/🦀️component.rs`) is reached ONLY through `db_actor::Supervisor::new` — and there is exactly
**one** caller of that constructor anywhere in the repo: this same file's own `#[cfg(test)] mod tests`
(4 tests, `EchoActor` as the only `impl Actor` in the whole codebase). `db_artifact::ArtifactAuthority`
explicitly does NOT implement `db_actor::Actor` (its own doc comment says so, precisely because
`ArtifactEngine` is `!Send`), and no other crate constructs a `Supervisor`. This mechanism has **zero
production callers today** — restructuring it onto `WorkerPool` would be real, non-trivial work
(`Actor: Send + 'static` IS satisfiable, unlike `ArtifactEngine`, so a `WorkerPool`-based "wake, drain
a batch via the mailbox's existing `try_recv`, resubmit if more remains" redesign is achievable — the
mailbox's DRR-lane draining logic doesn't need to be re-derived) spent on code nothing currently
exercises. Left unchanged given this packet's remaining time was better spent on the three sites with
real production traffic (§1, §3, §4). Flagged here explicitly rather than silently — a future caller
of `Supervisor::new` should not assume this thread-per-actor design is bounded.

### `db_artifact::ArtifactAuthority::spawn` (census #17)

`ArtifactEngine` embeds `db_state::PMap`, which is `Rc`-based — genuinely, structurally `!Send`, by
explicit design (the type's own doc says so). A `WorkerPool` job is `Box<dyn FnOnce() + Send>`; a
closure capturing a `!Send` value cannot itself be `Send`, so this actor cannot be moved onto the
shared pool without first making `db_state`'s core data structure `Send` (`Rc`→`Arc`, `RefCell`→
`Mutex`/`RwLock` or equivalent) — a data-structure-layer rewrite that ripples far beyond thread
ownership, squarely the kind of change this packet's brief explicitly rules out ("replacing
sqlite/sqlx/neo4rs... is a LATER phase... do NOT start that migration" — the same principle applies
to `db_state`'s `Rc`-based design, which several OTHER subsystems depend on for its single-threaded
guarantees). Unlike `store::sync`'s actor (§4), this one's thread genuinely IS load-bearing, not an
implementation shortcut — `store::sync`'s `LocalSet` trick (share one thread, `spawn_local` many
documents onto it) is architecturally available here too and would be the natural next step, but the
`ArtifactHost`↔`Database`/`ArtifactAuthority` machinery is different enough (`ArtifactAuthority::spawn`
takes a `build: impl FnOnce() -> Result<ArtifactEngine,..>` construction closure that must itself run
on the target thread, not a set of `Send` construction inputs like `store::sync`'s `SpawnRequest`) that
porting the same trick needs its own dedicated pass, not a mechanical copy. Confirmed still exactly
one thread per open document (`Database::spawn_authority_create`/`spawn_authority_open`, unbounded by
count) — genuinely still the packet's most significant remaining gap. Recommend this as the very next
piece of Phase 1 (or an early Phase 2) follow-up work, now that `store::sync` has proven the
`LocalSet`-supervisor pattern works for a `!Send` per-document actor in this same codebase.

## Trace queue counters wired

- `db_storage::BLOCKING_QUEUE` (`semio_framework_trace::QueueCounter`) tracks in-flight
  `run_blocking_op` submissions on the Io lane (items only — `enqueued(0)`/`dequeued(0)`, since a
  DB call's byte size isn't known upfront the way an HTTP range's is).
- `db` crate's `Cargo.toml` gained `semio-framework-trace = { workspace = true }`.

`pack_http`'s Io-lane submissions (§5) do not yet have their own `QueueCounter` — the `db_storage` one
is the only Io-queue depth signal landed this packet; wiring a second, HTTP-specific counter (with
real byte sizes, since `RangeRequest::range.len` is known upfront) is a natural small follow-up.

## Commands run (this session)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-os-kernel-db --all-features --tests` | clean, 0 errors |
| `cargo test -p semio-framework-os-kernel-db --all-features` | 479/479 passed |
| `cargo test -p semio-framework-os-kernel-db --all-features --release` | 479/479 passed |
| `cargo check -p semio-framework-os-kernel-db --no-default-features --features fs,thread` | clean |
| `cargo check -p semio-framework-os-kernel-db --features sqlite/postgres/neo4j` (individually + combined) | clean |
| `cargo clippy -p semio-framework-os-kernel-db --all-features --all-targets --no-deps -- -D warnings` | clean (after fixing 4 in-scope findings: 2 unused imports, 2 `needless_borrow`; pre-existing `await_holding_lock` findings in `db_engine`'s unrelated `vcs_integration` module and pre-existing debt in `semio-framework-replication`/`semio-framework-os-kernel-dsl-derive` — both untouched by this packet — left as-is) |
| `cargo check -p semio-framework-os-kernel --features sync` (production code) | clean |
| `cargo check -p semio-framework-os-kernel --features sync --all-targets` (incl. tests) | 16 pre-existing, unrelated errors (a `#[derive(DslArtifact)]`-macro/`os_dsl` mismatch in test fixtures — confirmed present verbatim in `git show HEAD`, not touched by this packet) |
| `cargo check -p semio-framework-pack --features ureq --all-targets` | clean |
| `cargo test -p semio-framework-pack --features ureq` (debug + `--release`) | 44/44 passed both |
| `cargo clippy -p semio-framework-pack --features ureq --all-targets --no-deps -- -D warnings` | clean except one pre-existing, untouched finding in `pack_async`'s `AsyncPackSource` trait |
| `cargo check -p semio-framework-pack --target wasm32-unknown-unknown` | clean |
| `cargo check -p semio-hub --features sqlite,postgres,neo4j --all-targets` | 48 pre-existing, unrelated errors (see "Cross-boundary" above) — the `HubDbRuntime`/`connect_db` region itself (this packet's actual change) is error-free |
| `bun ./📜️script.ts verify dependencies` | clean — 238 → 238 |

## Session hazard: transient file reversions

Partway through this session, several already-fixed files (`🐘️postgres/🦀️component.rs`,
`🌐️neo4j/🦀️component.rs`, `🛢️db/🦀️component.rs`, `🗄️storage/🪶️sqlite/🦀️component.rs`,
`👁️observe/🦀️component.rs`, `⚙️engine/🦀️component.rs`) were observed to have specific prior edits
silently reverted on disk (confirmed via `git status`/direct re-read, NOT a `git reset` — `git
reflog` shows no reset/checkout activity during this session's actual working window, only hours
earlier). Root cause undetermined. All were re-applied and the final state above (verified by the
"Commands run" table, executed as the last steps of this session) reflects the CURRENT, re-verified
file contents. If a fresh `cargo check -p semio-framework-os-kernel-db --all-features` comes back
non-clean when this ticket is picked up again, re-apply from this report — it documents every change
precisely enough to redo mechanically.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️component.rs` (db-level sync, distinct from store's)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs` (generic-param cleanup only, thread untouched — see "Deliberately NOT restructured")
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/👁️observe/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🦀️component.rs` (crate root facade)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs`
- `🧰️framework/🔨️modules/🎒️pack/🌐️http/🦀️component.rs`
- `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`

## Thread-creation-site count

**5 ad-hoc/dedicated thread-creation sites eliminated or bounded** within this packet's boundary:
1. `db_storage`'s blocking bridge (was consumer-side `HostAsyncRuntime::run_blocking`, now `WorkerPool`).
2. `db_engine::ArtifactHandle::submit`'s per-submit `"db-engine-submit-bridge"` thread — deleted.
3. `store::sync`'s per-document thread + embedded tokio runtime — collapsed to one shared supervisor thread per `ArtifactHost` (O(1), was O(open documents)).
4. `pack_http::UreqRangeTransport`'s per-request fetch thread — deleted (routed to `WorkerPool`).
5. `🌎️hub`'s `HubDbRuntime` (a `HostAsyncRuntime` bridge, not itself a thread-spawner, but the reason `run_blocking`'s removal needed hub-side follow-up) — deleted, replaced by a shared `WorkerPool`.

**2 sites explicitly identified and left unrestructured** (both documented above with the specific,
concrete reason): `db_actor::StdThreadSpawner` (zero production callers) and
`db_artifact::ArtifactAuthority::spawn` (structurally `!Send` engine state — the packet's most
significant remaining gap, with a proven pattern from §4 ready to adapt to it).

**1 site partially addressed**: `pack_http`'s retry-backoff `sleep()` timer thread (lower-frequency,
bounded by `max_retries`) — identified, not fixed, out of this session's remaining time.
