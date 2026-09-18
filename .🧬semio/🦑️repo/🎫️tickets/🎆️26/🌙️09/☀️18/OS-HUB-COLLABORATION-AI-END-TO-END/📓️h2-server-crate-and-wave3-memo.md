# H2 — `semio-framework-server` repair + Wave 3 decision memo

Slice H2, 2026-09-18. All commands run from `/Users/ueli/Documents/semio`, foreground, no sub-agents.
Captures: `🗑️generated/h2-server-test.txt`, `🗑️generated/h2-replication-test.txt`.
Nothing under `🌎️hub` was touched (H1 owns it). No git-modifying command was run.

---

## Part A — `cargo check`/`cargo test -p semio-framework-server` green

### The change (one line of real code)

`🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs:545` — the derive list on `FrontierSummary`:

```rust
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrontierSummary {
```

plus six docstring lines above it (`🔗️causal/🦀️.rs:545-551`) recording why the derive belongs to the
replication crate and why no `rename_all` is applied.

### Why the fix goes in the replication crate and not in `server`'s contract

The audit (`📓️audit-hub-backend.md` §3, §7 P0-3) offered two options: derive on `FrontierSummary`, or
strip `Serialize`/`Deserialize` from the `server` contract types that embed it. Four facts decide it
for the first option:

1. **Every sibling wire type in the same crate already carries the derives.** In
   `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` the serde derives sit at lines 1920, 1984, 2044,
   2099, 2137, 2166, 2237, 2278, 2322, 2355 — and the doc comment at `📡️wire/🦀️.rs:2095-2097` states
   the crate's own policy verbatim: *the serde derives are carried ALONGSIDE the hand-written
   `ToValue`/`FromValue` twin, the transitional state the serde-fanout playbook prescribes ("add
   alongside, do not blind-swap")*. `FrontierSummary` is the only wire-carried type in the crate that
   was left without them; `ArtifactId`, the struct's own first field, already has them
   (`🧰️framework/🔨️modules/📡️replication/🆔️ids/🦀️.rs:24-26`). The derive restores the crate's rule
   rather than adding an exception to it.
2. **`FrontierSummary` is genuinely a wire type**, carried by eight separate frame fields:
   `📡️wire/🦀️.rs:49` (`SocketHelloV1.frontier`), `:51` (`FrontierAdvertise`), `:90`, `:97`, `:108`,
   `:523` (`ServerFrame::Welcome.server_frontier`), `:536`, `:541`. Removing the bound from the
   `server` contract instead would have made `server` the only consumer that cannot put a frontier in
   a JSON envelope — an adapter, which this repo forbids.
3. **No feature or target risk.** `serde`/`serde_json` are unconditional `[dependencies]` of
   `semio-framework-replication` (`📦️packages/🦀️rust/Cargo.toml`, not optional, not behind a
   feature), so the derive adds nothing to any target. Verified by compiling for both wasm targets
   the crate is built for, not by argument — see the command block below.
4. **The wire shape is unchanged.** No `#[serde(rename_all = …)]` was added, matching the hand-written
   `ToValue`/`FromValue` twin at `🔗️causal/🦀️.rs:562` / `:573`, whose docstring already recorded that
   the original derive carried no rename and the field names stay snake_case. `[u8; 32]` is inside
   serde's own array impl range, so `chain_hash` needs no helper.

### Commands and output tails

```
$ cargo check -p semio-framework-replication
    Finished `dev` profile [unoptimized] target(s) in 0.40s
$ cargo check -p semio-framework-replication --target wasm32-unknown-unknown
    Finished `dev` profile [unoptimized] target(s) in 0.67s
$ cargo check -p semio-framework-replication --target wasm32-wasip2
    Finished `dev` profile [unoptimized] target(s) in 1.28s
$ cargo test -p semio-framework-replication --lib
test result: ok. 274 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.83s
```

274 passing matches the audit's measured baseline exactly — no regression.
`rustup target list --installed` confirms both wasm targets are present on this machine
(`wasm32-unknown-unknown`, `wasm32-wasip1`, `wasm32-wasip2`).

```
$ cargo check -p semio-framework-server
    Checking semio-framework-server v0.1.0 (…/🖥️server/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 3.82s
$ cargo test -p semio-framework-server
test result: ok. 73 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s
   Doc-tests server
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

All 28 `E0277`s are gone. **No test failures had to be fixed** — the crate's 73 tests (contract 5,
storage 10, policy 27, authority 11, gateway 20) were never broken, only unbuildable. The only
warning in the `server` build tree is a pre-existing `deprecated` on
`Atomic::<u64>::fetch_update` in `semio-framework-trace` (`🧰️framework/🔨️modules/⏱️trace/🦀️.rs:889`),
which is another crate and outside this slice.

Goal A is complete. The audit's P0-3 is closed.

---

## Part B — decision memo: hub's axum app vs. the generic `server` product

Report only; no code was written for this part.

### B.1 What each side actually is, measured

| | `🌎️hub` (`semio-hub`) | `🧰️framework/🛍️products/🖥️server` (`semio-framework-server`) |
|---|---|---|
| Rust lines | 54 598 (+20 360 TS) | 4 214, of which 1 095 are tests → **3 119 production lines** |
| Routes | 48 (`🏗️bootstrap/🦀️.rs:8117-8164`) | 12 (`base_router`, `📡️gateway/🦀️.rs:1300-1311`) + 1 demo route (`:207`) |
| Depended on by | the `os-hub` binary; nx `os-hub:dev`; the whole collaboration E2E | **nothing** — `grep -rln semio-framework-server --include=Cargo.toml .` returns only the root workspace list and its own manifest |
| Durable storage | `db::Database` (fs default, sqlite/postgres/neo4j features) + `SqliteDirectory` | **none** |
| TS twin | 20 360 lines across the subsystems | `🖥️server/🟦️.ts` is 3 lines: a doc comment and `export {};` |

### B.2 What the generic product provides that hub lacks

Real and worth keeping, all of it absent from hub:

- **A CQRS command/query vocabulary** — `CommandEnvelope`, `CommandReceipt`,
  `CommandOutcome{Accepted,Transformed,Rejected,Pending}`, `Rejection`, `OfflinePolicy`,
  `QueryEnvelope`, `QueryConsistency{Local,AtFrontier,Authority}`, `QueryResult{Snapshot,Page,Subscription}`
  (`🔨️modules/🧬️contract/🦀️.rs:89-211`). Hub has no such contract: every hub route is a bespoke
  request/response type.
- **A declarative policy engine** — `PolicyEngine` with `PolicyTemplate`s, per-principal assignments,
  wildcard and prefix resource matching, explicit deny beating allow, scoped vs. instance-wide
  assignments, and a `ResolverChain` authentication ladder with an anonymous fallback
  (`🛡️policy/🦀️.rs:70-310`, 27 tests). Hub authorizes with hand-written per-route predicates
  (`authorized`, `authorized_for_blob`, `authorized_for_canonical_pair`, `is_admin`,
  `admit_writes` at `🏗️bootstrap/🦀️.rs:1943`, `:1949`, `:1957`, `:2198`, `:4008`) plus
  `db::security::SecurityGate` — 58 `policy` mentions total, none of them a reusable engine.
- **A hybrid logical clock** stamping every command (`ServerState::now`, `📡️gateway/🦀️.rs:616`).
  `grep -rin "HybridLogicalClock\|hybrid_logical" 🌎️hub --include="*.rs"` → **0 hits**.
- **A saga runner** — `Saga`, `SagaRunner` (`🎭️authority/🦀️.rs:447`, `:490`).
  `grep -rin "\bsaga\b" 🌎️hub --include="*.rs"` → **0 hits**.
- **Lease-epoch fencing and an exactly-once outbox** as first-class store operations
  (`🗄️storage/🦀️.rs:94`, `:106`, tests `lease_epoch_bump_fences_out_the_previous_holder`,
  `outbox_delivers_each_entry_exactly_once`). Hub has ad-hoc leases (177 `lease` mentions) and 85
  `outbox` mentions, but no fenced store contract.
- **A replay/live seam with no gap and no duplicate** for the event stream
  (`EventSeam`, `📡️gateway/🦀️.rs:903`) and a resumable subscriber.
- **Static app hosting and an install registry** — `StaticAppHost`, `AppRegistry`, `scan_installs`
  (`📡️gateway/🦀️.rs:720`, `:800`, `:817`) and a `GET /instance` self-description.

### B.3 What hub provides that the generic product lacks

Everything that makes it a working server:

- **Socket grants** — `SocketSubjectV1`/`SocketAudienceV1`/`SocketBindingValidityV1`/
  `SocketGrantReceiptV1`, one-shot issue-then-consume admission for every WS route
  (`🏗️bootstrap/🦀️.rs:2250`, `:2263`, `:3746`, `:3795`, `:3814`, plus
  `🌎️hub/📇️directory/🔐️authorization/🔌️socket-grant/**`). The generic gateway has no admission
  mechanism for its sockets at all.
- **Directory / identity / tenancy** — 17 255 lines: event-sourced command log, read model
  (`load_read_model`, `🏗️bootstrap/🦀️.rs:5102`), spaces, invites, sharing, a live `/directory/socket/v1`.
- **Artifact authority** — 9 056 lines: trusted catalog, chunk CAS, genesis materialization,
  creation-request actor with admission and cancellation (`🏗️bootstrap/🦀️.rs:4582`, `:4929-5026`).
- **Inference** — 9 572 lines: the GIS-map job lane, event stream, approval and approval-undo
  (`🏗️bootstrap/🦀️.rs:8086-8091`).
- **A real presence implementation** — expiring owner-bound lease slots with heartbeat, user identity,
  label, role, surface, colour and opaque peer payload (`PresenceLeaseSlot`, `PresenceLeaseTransition`,
  `PresenceSnapshot`, `🏗️bootstrap/🦀️.rs:481-520`), scoped `(space, document, surface)` and proven by
  a three-client test. The generic gateway's `Presence` (`📡️gateway/🦀️.rs:392`) has colour leasing and
  a session map but no expiry, no heartbeat and no identity.
- **Durable everything**, an admin plane (11 `/admin/api/*` routes), execution-target leases, checkpoint
  publication, rebootstrap control, blob CAS, and the whole `protocol_wire` binary frame path.

### B.4 The finding that changes the Wave 3 plan

`26/08/18/SERVER-FRAMEWORK-PRODUCT/📋️summary.md` designs Wave 3 as: *"the gateway expresses the
document engine as the `DocumentAuthority` port: hub already depends on both server and db, so it
implements that port over `db::ArtifactHandle` and registers its directory/auth/sharing/extensions
modules. bin.rs then shrinks to a `Server::builder` call."*

**Two of that sentence's three premises are false today.**

1. *"hub already depends on both server and db"* — hub depends on `db` and on `directory`
   (`semio-framework-os-kernel`), and **not** on `semio-framework-server`
   (`🌎️hub/📦️packages/🦀️rust/Cargo.toml:59-62`). It never did.
2. *"the gateway expresses the document engine as the `DocumentAuthority` port"* — it no longer does,
   in any usable sense. A later de-dyn pass (O1, `dedyn-fw-server`) replaced every `dyn Trait` seam
   with a `dyn_enum_close!` enum **closed inside the framework crate**:

   | seam | closed over | file:line |
   |---|---|---|
   | `AuthorityStores` | `Memory(MemoryAuthorityStore)` only | `🗄️storage/🦀️.rs:302` |
   | `ProjectionStores` | `Memory(MemoryProjectionStore)` only | `🗄️storage/🦀️.rs:377` |
   | `BlobStores` | `Memory(MemoryBlobStore)` only | `🗄️storage/🦀️.rs:442` |
   | `SessionStores` | `Memory(MemorySessionStore)` only | `🗄️storage/🦀️.rs:516` |
   | `PrincipalResolvers` | `BearerToken(BearerTokenResolver)` only | `🛡️policy/🦀️.rs:234` |
   | `Deciders` | `Counter(CounterDecider)` only — a demo | `🎭️authority/🦀️.rs:156` |
   | `Sagas` | `Echo(EchoSaga)` only — a demo | `🎭️authority/🦀️.rs:481` |
   | `ServerModules` | `Counting(CountingModule)` only — a demo | `📡️gateway/🦀️.rs:211` |
   | `DocumentAuthorities` | **empty / uninhabited** | `📡️gateway/🦀️.rs:243` |
   | `QueryHandlers` | **empty / uninhabited** | `📡️gateway/🦀️.rs:265` |

   `ServerBuilder`, `ServerState` and all 13 handlers are typed against those concrete enums
   (`📡️gateway/🦀️.rs:580-611`, `:1222-1290`). A downstream crate therefore **cannot supply any
   implementation of any port** without editing the framework crate. The gateway's own comment at
   `📡️gateway/🦀️.rs:237-243` admits it: *"this crate has zero implementors by design … the day a real
   engine lands, it is added here as the first variant."* Adding hub's engine "here" means
   `semio-framework-server` naming hub types — the exact dependency inversion the crate description
   forbids.

   The consequence is observable, not theoretical: `get_document_ws` starts with
   `if state.documents.is_none() { return Err(ServerError::NotFound(…)) }`
   (`📡️gateway/🦀️.rs:1041`), and `documents` is `Option<Arc<DocumentAuthorities>>` over an uninhabited
   enum, so **the document websocket answers 404 unconditionally and the gateway's entire presence
   implementation (`📡️gateway/🦀️.rs:1073-1134`) is unreachable code**.
3. `ServerBuilder::build` destructures `StorageProfile::Embedded { data_dir }` and then constructs
   `AuthorityStores::Memory`, `ProjectionStores::Memory`, `BlobStores::Memory`,
   `SessionStores::Memory` (`📡️gateway/🦀️.rs:1222`, `:1250-1269`); `data_dir` is only copied into
   `state.data_dir` and never opens anything. **The product persists nothing.** Its 73 green tests
   all exercise in-memory demo variants.

So the generic product is not a server missing an instance. It is a well-shaped 3 119-line design
sketch whose every extension point is welded shut, whose storage is a stub, whose TS twin is
`export {};`, and which until this slice did not compile.

### B.5 What "hub as instance #1" concretely requires

Ten work items. Line estimates are from the files named.

| # | Item | Where | Est. diff |
|---|---|---|---|
| 1 | **Re-open the closed seams.** Delete the ten `dyn_enum_close!` invocations from the framework crate; keep only `#[dyn_enum]` on the traits (the macro doc at `🔀️dispatch/🦀️.rs:12-16` explicitly supports closing at a downstream site). Introduce one `ServerInstance` trait with ten associated types and make `ServerBuilder`/`ServerState`/`Server` and all 13 route handlers generic over it, so hub closes each set in its own crate. | `📡️gateway/🦀️.rs` (1 357), `🗄️storage/🦀️.rs` (525), `🎭️authority/🦀️.rs` (530), `🛡️policy/🦀️.rs` (314) | ~2 700 rewritten |
| 2 | **Re-profile the 73 tests** against a test instance profile carrying today's `Memory*`/`Counter`/`Echo` variants. Nothing is lost; the demo types move from production into the test module. | the four `🧪️tests/🔬️unit/🦀️.rs` (1 095) | ~1 100 rewritten |
| 3 | **Durable stores.** Hub implements `AuthorityStore`, `ProjectionStore`, `BlobStore`, `SessionStore` over `db::Database` and `SqliteDirectory`; `StorageProfile` gains the variants that actually exist. | new `🌎️hub/🗄️stores/🦀️.rs` | ~1 200 new |
| 4 | **`DocumentAuthority` over `db::ArtifactHandle`.** `welcome` ← `best_effort_frontier` + bootstrap chunking; `submit_frame` ← `handle_client_frame` + `submit_commands` + `admit_writes`. | `🏗️bootstrap/🦀️.rs:3915`, `:3977`, `:4008`, `:4020`, `:4082` | ~600 moved |
| 5 | **One presence implementation.** Fold hub's lease/heartbeat/identity model into the gateway's `Presence` (expiry, `user_id`, label, role, surface, `DirectoryPresenceActor` projection); delete hub's copy. Closes audit P1-6. | `📡️gateway/🦀️.rs:359-476` ← `🏗️bootstrap/🦀️.rs:481-520` + roster logic | ~600 moved, ~250 new |
| 6 | **Promote socket grants into the gateway** as the admission mechanism for every WS route — they are generic (subject/audience/binding validity/one-shot receipt), not hub domain. | `🏗️bootstrap/🦀️.rs:1549`, `:2250-2800`, `:3746-3830` + `📇️directory/🔐️authorization/🔌️socket-grant/**` | ~1 000 moved |
| 7 | **One authorization system.** Express hub's directory roles, space membership and share tokens as `PolicyTemplate`s; retire the per-route predicates and fold `db::security::SecurityGate` admission into the `PolicyHook`. | `🏗️bootstrap/🦀️.rs:1943`, `:1949`, `:1957`, `:2198`, `:4008` | ~500 rewritten |
| 8 | **Six `ServerModule` impls** — directory, auth, artifact-authority, inference, local-bootstrap, lag-rebootstrap — each supplying `manifest()`, `routes()`, `deciders()`, `resolvers()`, `templates()`. The subsystem bodies (17 255 + 95 + 9 056 + 9 572 + 1 136 + 893 lines) are unchanged; only their route functions move out of `🏗️bootstrap`. | six new `🦀️.rs` module heads | ~800 new, ~6 500 relocated |
| 9 | **`🏗️bootstrap/🦀️.rs` shrinks** from 8 522 lines to `connect_db`/`connect_directory`/`connect_artifact_cas`/`hub_worker_pool` plus a `Server::builder(profile).module(…)×6.document_authority(…).run(addr)` main. | `🏗️bootstrap/🦀️.rs` | ~300 remain |
| 10 | **The TS twin becomes real** — a typed command-submit / query / subscription client mirroring the contract, replacing `export {};`. Hub's 20 360 TS lines repoint at it. | `🖥️server/🟦️.ts`, `📦️packages/🟦️typescript/🟦️.ts` | ~700 new |

**Total: roughly 13 000–16 000 lines touched, of which ~4 500 genuinely new and the rest relocated or
re-typed.** It is a multi-week migration, and it touches the one crate the whole collaboration E2E
depends on.

### B.6 Recommendation — **(a) execute Wave 3**, with a correction and a precondition

Not because the generic product is valuable as it stands — §B.4 shows it is nearly all unreachable —
but because the alternative contradicts the stated product direction. The owner's own prompt record
is explicit (`.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md:872-880`):

> *"Introduce a new framework product: server … It is designed to be compatible with the os as client
> and uses CQRS Dual-Bus Actor Model instead of CRUD … A server must be deployable on edge, on-prem,
> etc. The hub is an instance of a server. mit-bestand/zentrale will be another instance of the server."*

A second instance is planned, and `♻️mit-bestand/` already exists in the tree. Option **(b)**, folding
the product into hub, would delete the seam that second instance needs and make `zentrale` a fork of
`semio-hub` — duplication on a far larger scale than today's. Option **(c)**, keeping both, is what
produced the current state: a crate nothing depends on, which silently stopped compiling and was
invisible until this ticket. The repo's rule of one clean design with no duplication and no
compatibility layers rules (c) out by name.

**The correction:** Wave 3 cannot be executed as written. Its premise — hub plugging into the
`DocumentAuthority` port — was invalidated by the O1 de-dyn pass, which closed that port over an
uninhabited enum. Item 1 in §B.5 (re-opening the seams, generic over one `ServerInstance` profile)
is a *precondition* of Wave 3, not part of it. Closing the sets at the instance site is what the
`dyn_enum_close!` design intends (`🔀️dispatch/🦀️.rs:12`: *"at the site that closes the set"*); the
current invocations are simply in the wrong crate.

**The precondition:** do not start before hub compiles and boots. The audit's P0-1
(`SpaceArtifactCreationReadyV1` field drift, `🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs:64` and
`🧬️schema/🦀️.rs:249`) and P0-2 (the off-by-one `../` at
`🌎️hub/🧪️tests/🧱️socket-grant-command-source/🏃️execution/🟦️.ts:2`) must land first — H1's slice.
Migrating a crate that does not build has no green baseline to diff against.

### B.7 Concrete step list

1. **(H1, blocking)** Land audit P0-1 and P0-2; `cargo test -p semio-hub --lib` and
   `cargo test -p semio-hub --bin os-hub` green; `bun nx run os-hub:dev` answers `/readyz`. Record the
   test counts as the Wave 3 baseline.
2. **Freeze a behavioural baseline.** Capture the hub collaboration E2E (`26/08/17`'s two-browser
   Playwright run) and the hub unit counts. Every later step is judged against these numbers, not
   against `cargo check`.
3. **De-close the seams.** Define `pub trait ServerInstance` with associated types
   `Modules`, `Queries`, `Documents`, `Deciders`, `Sagas`, `Resolvers`, `AuthorityStore`,
   `ProjectionStore`, `BlobStore`, `SessionStore`. Delete the ten `dyn_enum_close!` blocks from
   `🗄️storage`, `🛡️policy`, `🎭️authority`, `📡️gateway`; make `ServerBuilder<I>`, `ServerState<I>`,
   `Server<I>` and the 13 handlers generic. Move `CountingModule`, `CounterDecider`, `EchoSaga` and the
   four `Memory*` stores into a `TestInstance` profile under the crate's `🧪️tests/`. Gate: 73 tests
   still green, zero production-side demo types remaining.
4. **Give hub the dependency.** Add `semio-framework-server` to `🌎️hub/📦️packages/🦀️rust/Cargo.toml`
   and define `HubInstance: ServerInstance` closing all ten sets with `dyn_enum_close!` in hub. Gate:
   `cargo check -p semio-hub`, no route moved yet.
5. **Durable stores** (§B.5 item 3) over `db::Database`/`SqliteDirectory`. Port the framework's ten
   storage tests to run against the hub-backed stores as well, including
   `lease_epoch_bump_fences_out_the_previous_holder` and `outbox_delivers_each_entry_exactly_once`.
   Gate: both store suites green, restart-survival test still passing.
6. **`DocumentAuthority` over `db::ArtifactHandle`** (item 4), then flip the document WS from hub's
   `document_ws_v1` to the gateway's `get_document_ws`. Gate: the two-browser E2E from step 2
   reproduces byte-identically; `ServerFrame` round-trip fixtures unchanged.
7. **Socket grants into the gateway** (item 6), so the gateway route that just started working is
   admission-gated exactly as hub's was. Gate: `socket-grant-command-source-check`,
   `presence-lease-check`, `presence-normalization-check` still green.
8. **One presence implementation** (item 5). Gate: `presence_roster_is_scoped_per_surface` and the
   three-client test pass against the gateway's roster.
9. **One policy system** (item 7): roles and share tokens as templates, per-route predicates deleted.
   Gate: every hub authorization test green with zero `authorized*` free functions left in
   `🏗️bootstrap`.
10. **Six `ServerModule` impls** (item 8), one at a time, smallest first (`🔐️auth` 95 lines →
    `🛰️lag-rebootstrap` → `🚀️local-bootstrap` → `🗿️artifact-authority` → `💡️inference` →
    `📇️directory`). Gate after each: full hub suite + `/readyz` + the E2E.
11. **Shrink `🏗️bootstrap/🦀️.rs`** to the builder call (item 9). Gate: the file is under ~350 lines and
    the binary still boots zero-touch with `OS_HUB_STORAGE_BACKEND` unset.
12. **Real TS twin** (item 10); repoint hub's admin SPA and the os client at it. Gate: `bun tsc
    --noEmit` on the server product's tsconfig clean (hub's own tsconfig breadth is audit P2-8,
    separate).
13. **Retire the leftovers.** Delete `StaticAppHost`/`AppRegistry` if hub's extension-module routes
    already cover them, or delete hub's if the framework's do — one of the two, never both.
14. **Close the loop on this memo's own finding:** add a workspace gate so
    `semio-framework-server` can never again go red unnoticed — it is currently reachable from no
    other crate's build, which is precisely how 28 errors survived a month.
