# W3d — `Result`-returning projection/session writes, sagas in `ServerState`, the workspace gate, the TS twin

Slice W3d, on top of W3a (`📓️w3a-server-instance-seams.md`), W3b
(`📓️w3b-hub-instance-and-durable-stores.md`) and D1 (`📓️d1-dispatch-send-future-ports.md`).
All commands foreground, from `/Users/ueli/Documents/semio`, no sub-agents, no git-modifying
command, no worktree. Captures: `🗑️generated/w3d-*.txt`.

Scope, from the slice brief:

1. `ProjectionStore`/`SessionStore` writes return `Result`, with fault-injection laws in the
   conformance suite proving the fault path in all three profiles.
2. `ServerSagas<I>` wired into `ServerState` + an exactly-once-across-restart outbox law.
3. Memo step 14 — a workspace gate so `semio-framework-server` cannot go red unnoticed.
4. Memo step 12 groundwork — the real TS twin of the server product with shared round-trip fixtures.
5. Aside: `cargo test -p semio-framework-math` unit-test compile errors.

Out of scope by instruction: hub route moves, `🌎️hub/🏗️bootstrap/🦀️.rs` (slices H1/AU1/K1),
repointing hub's admin SPA or the os client at the new TS twin (W3c).

---

## 0. Inherited state

`git status --short` on `🧰️framework/🛍️products/🖥️server`, `🌎️hub/🗄️stores` and
`🗑️generated/w3d-*` at slice start: **no partial W3d work existed** — `🗑️generated/` held no `w3d-*`
capture, and the two directories carried exactly W3a + W3b + D1's finished state (`🌎️hub/🗄️stores/`
and the two new `🧪️tests/` directories still untracked, the rest `M`). So this slice started from
zero, not from a dead worker's half-edit.

Measured baseline before any edit:

| gate | result |
|---|---|
| `cargo test -p semio-framework-server` | **83 passed** (78 lib + 5 `closed_ports`), matching D1's report |
| `cargo test -p semio-framework-math` | **red**, 7 compile errors — D1 §7's pre-existing item, this slice's item 5 |

## 1. `Result`-returning projection and session writes

### 1.1 The contract change

`🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs` — six method signatures, reads
untouched:

| port | method | before | after | line |
|---|---|---|---|---|
| `ProjectionStore` | `put` | `()` | `Result<(), StorageError>` | `:248` |
| | `set_checkpoint` | `()` | `Result<(), StorageError>` | `:261` |
| | `clear` | `()` | `Result<(), StorageError>` | `:265` |
| `SessionStore` | `create` | `()` | `Result<(), StorageError>` | `:344` |
| | `delete` | `()` | `Result<(), StorageError>` | `:351` |
| | `revoke_principal` | `usize` | `Result<usize, StorageError>` | `:356` |

Reads (`get`, `list`, `checkpoint`) keep their plain return types on purpose and both trait
docstrings now say why: every backend in the tree folds its journal into memory at open, so a read
has nothing to fail at, and widening them would have been speculative surface.

Two docstring paragraphs carry the reasoning rather than a changelog:
`🗄️storage/🦀️.rs:227-235` (**"Every write answers"** — rebuildability is the *repair*, not a licence
to swallow the fault) and `:323-331` (the session case, where a `revoke_principal` answering a bare
`usize` through a failed removal tells a caller "signed out everywhere" while the record that opens
the door is still on disk).

### 1.2 The bug the `Result` uncovered — `Journal::append` reported success through a dead sink

`🌎️hub/🗄️stores/🦀️.rs:212-228` (`Journal::append`, `:220`). Making the write fallible was necessary and **not sufficient**:
the first run of the new fault law failed on the durable profile, with a store whose sink was a
genuinely unwritable file handle still answering `Ok`. `tokio::fs::File` buffers a write and
performs it on a blocking worker, so `write_all` returns before the filesystem has seen anything,
and `sync_data` completes the in-flight write **discarding its error** (tokio's `complete_inflight`
ends in `.ok()`) before fsyncing. One explicit `flush()` between the two is the only call that
surfaces the deferred error:

```rust
sink.write_all(&line).await.map_err(…)?;
sink.flush().await.map_err(…)?;
sink.sync_data().await.map_err(…)
```

Found by the test, not by reading — the law is the thing that makes the `Result` mean anything.

### 1.3 Ripple

- `🌎️hub/🗄️stores/🦀️.rs:544` — `HubProjectionStore::commit` is now
  `journal.append(&record).await?; self.fold(record); Ok(())`. The previous shape counted the fault,
  dropped the sink and folded anyway, which left the read models answering from state the next open
  would not reproduce. `faults`/`journal_faults()` are **deleted** from both stores (W3b's
  counter-instead-of-report workaround has no reason to exist any more).
- `🌎️hub/🗄️stores/🦀️.rs:693` — `HubSessionStore::forget` removes from **disk first** and only
  drops the in-memory record once the removal succeeded, so a refused delete leaves the store
  honestly holding a session it could not revoke. `revoke_principal` refuses on the first record it
  cannot remove rather than returning a partial count.
- `🧪️tests/🧩️instance/🦀️.rs` — the two reference in-memory stores gained a `ReferenceSink`
  (`Working`/`Failing`) and a `fail()` method, so a backend with no real sink can still be held to
  the fault contract.
- `🧪️tests/🔒️closed-ports/🦀️.rs` — `Rung<N>`'s six write impls, and the `revoke_principal`
  assertions, now speak `Result`. The two ports are still closed over two-variant enums by
  `dyn_enum_close!`, so D1's macro path covers the new signatures unchanged.
- `📡️gateway/🦀️.rs` handlers: **none needed changing.** Measured, not assumed —
  `grep -rn "projections\|sessions" 📡️gateway/🦀️.rs` shows `post_query` is the only handler that
  reaches either store and it only reads; `sessions` is carried in `ServerState` and not yet read by
  any route. `ServerError: From<StorageError>` already exists (`📡️gateway/🦀️.rs:140`), so the day a
  handler does write one, `?` is all it needs. This is an honest gap, not a completed ripple — see
  §7.

### 1.4 The fault laws

Two new laws in `🧪️tests/🔬️conformance/🦀️.rs`, each with a seed helper so the store arrives in a
known state and the sink fails *after* it holds something:

- `seed_projection_fault_fixture` / `a_projection_write_reports_a_failing_sink` (`🔬️conformance/🦀️.rs:151-181`) — all
  three writes refuse, and the visible state is still the seeded one: `space/a` is `[1]` not the
  refused `[2]`, the checkpoint has not moved off 5, and a refused `clear` dropped nothing.
- `seed_session_fault_fixture` / `a_session_write_reports_a_failing_sink` (`🔬️conformance/🦀️.rs:214-239`) — `create`,
  `delete` and `revoke_principal` all refuse, **and `s1` is still readable afterwards**: the door
  really is still open, and the caller was told rather than reassured.

No fault-injection hook was added to either port. The law is written against the port and states its
precondition; each profile brings its store into the failing state the way its own backend can
genuinely fail:

| profile | how the sink is made to fail | where |
|---|---|---|
| `TestInstance` (memory) | `ReferenceSink::Failing` via `store.fail()` | `🗄️storage/🧪️tests/🔬️unit/🦀️.rs:78`, `:100` |
| `HubInstance` ephemeral | journal sink ← a **read-only `File` handle**; session `dir` ← a path that is a regular file | `🌎️hub/🗄️stores/🧪️tests/🔬️unit/🦀️.rs:111-152` |
| `HubInstance` embedded | same two mechanisms, on the store's **own** `log.jsonl` and directory | same |

Both hub cases are real OS-level failures (`EBADF` on a read-only handle, `ENOTDIR` under a regular
file), not flags, and both are cross-platform — a read-only handle and a non-directory parent both
refuse writes on Windows too. The durable half additionally reopens the store afterwards and asserts
the refused write is absent from the journal as well.

## 2. `ServerSagas<I>` in `ServerState`

W3a shipped the `ServerSagas<I>` alias and left the runner to be constructed by whoever called
`build` — its own §8 listed this as the gap. Four changes close it, all in `📡️gateway/🦀️.rs`:

1. `ServerState<I>` gains `pub sagas: Arc<Mutex<ServerSagas<I>>>` (`:675-681`) and clones it like
   every other handle (`:723`). The docstring states the reason it belongs *here*: the outbox and
   the workflows that drain it are two halves of one exactly-once guarantee, and a runner living
   somewhere else is a runner that can be forgotten — a forgotten runner is a queue that grows
   forever while every event looks delivered.
2. `ServerModule::sagas()` (`:250-256`), defaulting to none, so a subsystem owns its workflows for
   the same reason it owns its deciders.
3. `ServerBuilder::saga()` (`:1351-1356`) for a workflow that belongs to the deployment rather than
   to a module; `build` folds module workflows and builder workflows into one runner in registration
   order (`:1401-1404`).
4. `ServerState::drain_sagas(limit) -> Vec<CommandOutcome>` (`:770-795`) — drains the outbox through
   the runner **and runs every follow-up command as its own turn**, because a drain that only
   returned commands would leave the rows acknowledged while their consequences sat in a `Vec` the
   caller might drop, which is the exact failure the transactional outbox exists to rule out. Both
   locks are released before the first follow-up is submitted: a turn takes the bus lock itself, so
   holding it across the drain would deadlock the server on its own workflow.

### The law

`conformance::a_saga_reacts_to_a_committed_event_exactly_once_across_a_restart`
(`🧪️tests/🔬️conformance/🦀️.rs:255-297`), plus the `OneCommandPerEvent` fixture workflow whose
one-command-per-event shape makes the drain's length *be* the reaction count.

The restart is passed as a **factory**, not as a second already-open store: the reopen has to happen
after the commit and the drain, and handing the law a second store opened up front would let it
observe a journal that had not been written yet. That detail is what makes the durable run real
rather than decorative.

Four facts, in order: an append commits the event and its outbox row in one write; a drain hands the
row to every workflow once, carrying the idempotency key that makes the follow-up safe to retry; the
row leaves the pending queue behind it; the restarted store hands the workflow nothing. Run in all
three profiles — `TestInstance` (`🗄️storage/🧪️tests/🔬️unit/🦀️.rs:61`) and both `HubInstance`
shapes (`🌎️hub/🗄️stores/🧪️tests/🔬️unit/🦀️.rs:97`).

Wired end to end by `gateway::tests::a_committed_event_reaches_the_instance_saga_runner_exactly_once`
(`📡️gateway/🧪️tests/🔬️unit/🦀️.rs:379`): a module-contributed workflow and a builder-contributed
one land on the **one** runner (`sagas.len() == 2`), a real `post_command` commits an event, the
first `drain_sagas` runs exactly one follow-up turn that is `Accepted`, the second drain's re-issued
command is deduplicated by its idempotency key into an `Accepted` with **no** events rather than
committing twice, and the pending queue ends empty.

## 3. The workspace gate (memo step 14)

### 3.1 What the hole actually was — the memo's diagnosis is half right

The memo says the crate "is currently reachable from no other crate's build, which is precisely how
28 errors survived a month". The first half is true and unchanged (`grep -rln semio-framework-server
--include=Cargo.toml .` still finds only hub's manifest and the workspace list). The second half —
"therefore nothing runs it" — is **not** what the tree says, and the difference decides the fix.
Measured, not assumed (`🗑️generated/w3d-nx-projects.txt`, 968 projects):

- `@semio-tech/framework-server-rs` **is** an nx project with a `test` target, and
  `workspace:test`'s `dependsOn` is `{ target: "test", projects: ["*", "!workspace", …] }`, so the
  crate *is* on the aggregate test lane.
- `@semio-tech/framework-server` (the TypeScript package) is too.
- **Neither had a `launch.json` row.** `grep -n framework-server .vscode/launch.json` returned
  nothing before this slice. AGENTS.md is explicit that *"all devs are using `launch.json` and never
  use the cli"*, so a target with no row is a target no human ever runs — the aggregate lane is a
  full-repo run nobody does per change.
- And the `conformance` **feature** had no gate at all: the module is
  `cfg(any(test, feature = "conformance"))`, `cargo test` compiles it through the `test` half, so a
  break in the half every *instance* crate consumes as a dev-dependency stayed invisible until hub's
  own build hit it.

### 3.2 The three changes

1. `🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/📜️script.ts:17-18` — `test` now runs
   `cargo test -p semio-framework-server` **and** `cargo check -p semio-framework-server --features
   conformance`. The docstring records why, pointing at this memo's step 14. `📋️project.json` was
   not touched: it already calls `📜️script.ts test`, which is the rule (`project.json` MUST only
   call the script).
2. `.vscode/🧩️launch.seed.jsonc` — two rows in the `4_build` group, placed with the other product
   test rows (`📦️test🗄️os-hub` at 206.161–206.162) and named in the same shape with the server
   product's own emoji:
   - `📦️test🖥️server` → `bun nx run @semio-tech/framework-server-rs:test` (order 206.1625)
   - `📦️test🖥️server🟦️typescript` → `bun nx run @semio-tech/framework-server:test` (order 206.1626)
3. `.vscode/launch.json` regenerated by the owning script — `bun nx run
   @semio-tech/plugin-registry:generate` (`🗑️generated/w3d-registry-generate.txt`), never hand-edited,
   as `🚀️launch/🟦️.ts`'s own docstring requires. Result: 325 configurations, zero duplicate names,
   both rows present at `launch.json:4174-4194`. The regeneration is large (+815/−280) because the
   generated file was stale against peer seed edits, not because of these two rows.

### 3.3 Hub's conformance lane

Already satisfied, and now proven rather than assumed: hub declares
`semio-framework-server = { workspace = true, features = ["conformance"] }` on its **dev**-dependency
(`🌎️hub/📦️packages/🦀️rust/Cargo.toml:79`), so every `cargo test -p semio-hub` — which is exactly
what `os-hub:test` runs (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:2568-2576`) — compiles and runs the
conformance suite. The 21 `stores::` laws in §6 are that lane executing.

## 4. The TS twin (memo step 12 groundwork)

### 4.1 Where the schema is

There is no `🧬️schema` directory under `🖥️server` and there was never a generator: the wire shape is
owned by the Rust contract's serde derives (`🔨️modules/🧬️contract/🦀️.rs`) plus `base_router`'s
route table (`📡️gateway/🦀️.rs`, `fn base_router`). Neither is expressible in TypeScript, so
**generation was not possible and a hand-written copy would have been a second contract**. The
schema-first requirement is met the way AGENTS.md's "language-agnostic test for every feature"
intends instead: a shared fixture document both twins are held against.

New `🧰️framework/🛍️products/🖥️server/🧫️fixtures/🔌️wire/🔣️.json` (`semio.framework.server.wire/v1`):
the 14 mounted method/path pairs, and **30 canonical JSON vectors across 15 wire types**. Three
serde spellings live in this contract and each has a vector that pins it:

| spelling | example vector |
|---|---|
| `rename_all = "camelCase"` (most structs) | `command-envelope-full` |
| **no** rename — snake_case on the wire | `trace-context-snake-case`, `frontier-summary-snake-case` |
| tagged enum: variant renamed, **fields not** | `rejection-unknown-command-kind-keeps-snake-case-field` (`command_kind`), `command-outcome-transformed-keeps-snake-case-fields` (`canonical_events`) |

That third row is the one a hand-written twin gets wrong every time, and it is the same trap W3b hit
from the other side (`rename_all_fields`).

### 4.2 The twin

`🧰️framework/🛍️products/🖥️server/🟦️.ts`, 3 lines (`export {};`) → **1176 lines**, tests included:

- **Types** for every contract shape the twelve routes and two sockets carry: identity newtypes as
  `string`, `ActorKey`, `Principal`, `HybridLogicalClock`, `TraceContext`, `FrontierSummary`,
  `CommandEnvelope`/`CommandReceipt`/`Rejection`/`Notice`/`CommandOutcome`, `QueryConsistency`/
  `QueryEnvelope`/`QueryResult`, `EventRecord`, `EphemeralFrame`, the policy and module manifest
  types, `ErrorBody`, `BlobReceipt`, `AppInstall`.
- **A codec**, not a cast. Rust's `Vec<u8>` is a JSON number array; the owned types say
  `Uint8Array`, and `encode*`/`decode*` are the boundary. Every decoder validates and throws
  `WireError` naming the field path (`receipt.revision`), so a wire drift surfaces where it happened
  instead of as `undefined` three layers later.
- **`ServerClient`** over an owned `HttpTransport` interface — never `fetch` in the exported API,
  because nothing this module exports may name a type it does not own; `fetchTransport(baseUrl)`
  adapts the platform. Methods: `instance`, `submitCommand`, `query`, `publishEphemeral`, `events`,
  `blob`, `hasBlob`, `putBlob`, `apps`, `appInstalls`, `eventStreamUrl`, `documentSocketUrl`.
  Failures arrive as `ServerCallError` carrying the gateway's own `kind` tag — the thing to branch
  on, since the status code is a transport detail.
- **Socket frames**: `decodeEventStreamFrame` (the durable lane sends each `EventRecord` as JSON
  text) and `decodeDocumentFrame` (binary = the engine's opaque bytes, text = the gateway reporting
  an error on that socket without closing it), plus `streamLane`/`documentLane`/`ephemeralLane`
  keyed exactly as the gateway keys them.
- **`SERVER_ROUTES`**, checked against the Rust source rather than trusted.

### 4.3 The cross-language gate

Two suites, one document:

- Rust: new `[[test]] wire` → `🧪️tests/🔬️wire/🦀️.rs` (120 lines, **3 tests**). Every vector is
  `serde_json::from_value` into the contract type its `type` names and serialized straight back,
  asserted byte-identical; vector names are unique; and the fixture's route table is extracted from
  `base_router` in the gateway source (`include_str!`) and compared.
- TypeScript: in-source `import.meta.vitest` blocks in `🟦️.ts` (**14 tests**) — the same 30 vectors
  decoded and re-encoded, a check that the set of vector types equals the set of implemented codecs
  (so a new vector cannot be added without a twin), the same `base_router` source scan, the byte
  and field-path error cases, and six client tests driving a recording transport.

A serde rename now fails **both** suites. Neither twin can drift alone, which is the only
arrangement under which two implementations of one contract can be said to agree.

Per instruction, **nothing was repointed**: hub's admin SPA and the os client still speak their own
clients; W3c flips routes once the collaboration E2E baseline exists.

## 5. Aside — `semio-framework-math` unit tests

`🧰️framework/🔨️modules/🧮️math/🎯️sampling/🧪️tests/🔬️unit/🦀️.rs`, five lines, all asyncify damage
from another slice's sweep (D1 §7 reported it and correctly did not touch it):

| line | was | is |
|---|---|---|
| `:501` | `geometry::random::Rng::from_seed(4242).await` | `…from_seed(4242)` — the constructor is sync (`📐️geometry/🎲️random/🦀️.rs:42`) |
| `:503` | `reference.next_u64().await` | `reference.next_u64()` — sync too (`:15`) |
| `:1300`, `:1302`, `:1303` | `penalty.counts.count(TokenId::new(n))` | `…count(TokenId::new(n)).await` — `count` is `async` (`🎯️sampling/🦀️.rs:3360`) |

`cargo test -p semio-framework-math` → **191 passed, 0 failed**
(`🗑️generated/w3d-math-test.txt`).

## 6. Gates

Every number below was produced by running the command, in the foreground, from the repo root.

| gate | result | capture |
|---|---|---|
| `cargo test -p semio-framework-server` | **90 passed, 0 failed, 0 warnings** — 82 lib (was 78) + 5 `closed_ports` + 3 `wire` | `w3d-server-test-final.txt` |
| `bun nx run @semio-tech/framework-server-rs:test` | green, including the new `--features conformance` check | `w3d-server-nx-test.txt` |
| `cargo check -p semio-framework-server --features conformance` | clean | (in the above) |
| `bun nx run @semio-tech/framework-server:test` | **14 passed (1 file)** | `w3d-ts-test-1.txt` |
| `tsc --noEmit --strict` on `🖥️server/🟦️.ts` | clean, zero diagnostics | `w3d-ts-typecheck.txt` |
| `cargo check -p semio-hub --lib --no-default-features --features sqlite` | clean | `w3d-hub-check-1.txt` |
| `cargo test -p semio-hub --lib … -- stores::` | **21 passed, 0 failed** (was 18) | `w3d-hub-stores-test-2.txt` |
| `cargo test -p semio-hub --lib …` (whole suite) | **155 passed, 1 failed** — the failure is a peer's, see §7 | `w3d-hub-lib-test-final.txt` |
| `cargo test -p semio-framework-math` | **191 passed, 0 failed** | `w3d-math-test.txt` |
| `bun nx run @semio-tech/plugin-registry:generate` | `.vscode/launch.json` regenerated, 325 configurations, 0 duplicate names | `w3d-registry-generate.txt` |

New tests this slice, by suite:

- server lib +4: `a_projection_write_reports_a_failing_sink`, `a_session_write_reports_a_failing_sink`,
  `a_saga_reacts_to_a_committed_event_exactly_once_across_a_restart`,
  `a_committed_event_reaches_the_instance_saga_runner_exactly_once`.
- server integration +3: the whole `wire` target.
- hub `stores::` +3: the same three storage/saga laws, each run twice (ephemeral and embedded).
- TypeScript +14.

Counting the conformance bodies rather than the harnesses: each of the two fault laws executes in
**three** profiles (TestInstance, HubInstance ephemeral, HubInstance embedded) and so does the saga
outbox law, from one body each — which is the property the suite exists for.

## 7. Honest gaps

- **Verified by tests only.** No server was bound to a socket in this slice, and `Server::run` stays
  unexercised as it was before. `drain_sagas` is proven through a real `post_command` and a real
  `CommandBus` turn, but nothing schedules it yet: there is no background drain loop, on purpose —
  *when* to drain is the caller's decision and inventing a timer here would be new behaviour outside
  the slice. An instance that never calls it has an outbox that grows, and the day hub registers its
  first saga it must also decide the cadence.
- **The handler ripple for the new `Result`s is a compile-time ripple, not a behavioural one.** No
  gateway handler writes a projection or a session today (`post_query` only reads; `sessions` sits in
  `ServerState` unread by any route), so nothing had to learn to surface a write fault. The
  conversion exists (`ServerError: From<StorageError>`), and the first handler that writes gets it
  with `?`. Claiming "rippled through the handlers" would overstate what is there.
- **The TS twin is not wired to anything.** By instruction: hub's admin SPA and the os client are
  W3c's. It is also unexercised against a live server — the client tests drive a recording
  transport, and the contract half is gated by the shared fixture, not by an HTTP round trip.
- **There is still no tsconfig for the server product**, so the memo's step-12 gate (`bun tsc
  --noEmit` on the product's own tsconfig) is met here by an ad-hoc strict `tsc` invocation over the
  one file rather than by a permanent target. The framework-wide typecheck target is T3/T3b's path
  and adding a competing one here would duplicate it.
- **`u64` is `number` in the twin.** `seq`, `revision` and the clock are 64-bit in Rust and exact to
  2^53 in JavaScript. Every one of them is far below that in practice; a counter that ever approaches
  it needs a wire change on both sides, not a silent `bigint` on one. Stated in the module docstring.
- **Every hub gate ran on the `--no-default-features --features sqlite` lane**, as W3b's did. The
  default-feature hub lane was not measured here; it pulls `native-artifact-execution` and is another
  slice's ground.
- **One hub test is red and it is not this slice's**:
  `artifact_authority::trusted_catalog::opened_root::publication_tests::trusted_publication_owner_process_crash_releases_exact_lock`
  fails with `Os { code: 20, kind: NotADirectory }` at
  `🛡️opened-root/🧪️tests/🔬️publication/🦀️.rs:63`. That file and its module are a peer's live edit
  (`🛡️opened-root/🦀️.rs` last written 2026-09-19 02:14, `MM` in `git status`), the failure is in a
  cross-process publication fence this slice touches nothing of, and every `stores::` law is green.
  Reproduced alone as well as in the suite, so it is not fleet flakiness.
- **`.vscode/launch.json` was regenerated wholesale.** The two rows are mine; the other ~1000 changed
  lines are the generator catching up with peer seed edits that had not been regenerated. That is the
  generated file's own contract (never hand-edit, always regenerate), but it does mean the diff is
  not a clean two-row diff.
- **The route-table check is a source scan, not router introspection.** axum's `Router` cannot be
  enumerated at runtime, so both twins parse `base_router`'s body out of the gateway source. It
  catches a route added, removed or renamed; it would not catch a route mounted by a `ServerModule`
  at build time, which is by design — those are the instance's, not the product's.

## 8. Files changed

```
M  🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs                    (6 write signatures + 2 doc paragraphs)
M  🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🧪️tests/🔬️unit/🦀️.rs      (+3 laws)
M  🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs                    (ServerState.sagas, ServerModule::sagas, ServerBuilder::saga, drain_sagas)
M  🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🧪️tests/🔬️unit/🦀️.rs      (+1 law)
M  🧰️framework/🛍️products/🖥️server/🧪️tests/🧩️instance/🦀️.rs                     (ReferenceSink, fail(), Result writes, CountingModule::sagas)
M  🧰️framework/🛍️products/🖥️server/🧪️tests/🔬️conformance/🦀️.rs                   (+2 fault laws, +2 seeds, +1 saga law, +OneCommandPerEvent)
M  🧰️framework/🛍️products/🖥️server/🧪️tests/🔒️closed-ports/🦀️.rs                  (Rung<N> write signatures)
A  🧰️framework/🛍️products/🖥️server/🧪️tests/🔬️wire/🦀️.rs                          (120)
A  🧰️framework/🛍️products/🖥️server/🧫️fixtures/🔌️wire/🔣️.json                     (30 vectors / 15 types, 14 routes)
M  🧰️framework/🛍️products/🖥️server/🟦️.ts                                        (3 → 1176)
M  🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/Cargo.toml                  ([[test]] wire)
M  🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust/📜️script.ts                 (conformance-feature check)
M  🌎️hub/🗄️stores/🦀️.rs                                                        (Journal::append flush, commit Result, session forget/create/revoke, faults deleted)
M  🌎️hub/🗄️stores/🧪️tests/🔬️unit/🦀️.rs                                          (+3 laws, failing_sink/blocked_dir)
M  .vscode/🧩️launch.seed.jsonc                                                  (+2 rows)
M  .vscode/launch.json                                                          (regenerated by @semio-tech/plugin-registry:generate)
M  🧰️framework/🔨️modules/🧮️math/🎯️sampling/🧪️tests/🔬️unit/🦀️.rs                  (5 await fixes)
```

Nothing under `🌎️hub/🏗️bootstrap/🦀️.rs` or any hub route was touched. No git-modifying command was
run; no sub-agent; every command foreground.
