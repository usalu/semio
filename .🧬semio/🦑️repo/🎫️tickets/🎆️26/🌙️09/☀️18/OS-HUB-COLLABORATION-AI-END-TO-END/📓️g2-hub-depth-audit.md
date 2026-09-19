# G2 — Hub backend depth audit (`🌎️hub` + `🧰️framework/🛍️products/🖥️server` + `📡️replication`)

Slice G2, read-only, 2026-09-19. All paths relative to `/Users/ueli/Documents/semio`. No commands
were run (grep/find/Read only, per slice rules); every claim below is a static evidence citation,
not a runtime measurement. `🗑️generated/` was empty at audit time (wiped since the last relaunch),
so hub's live test pass/fail counts are **not re-verified here** — see `📓️h1-hub-build-and-boot.md`
§1/§5 for the last measured compile state (hub compiles standalone; the default-feature build is
blocked by a peer's stdio-pdf rewrite, not by hub itself) and note H1/W3b/D1 own the compile/boot/
`ServerInstance`/dispatch-macro work — this report does not re-litigate those, only cites their
outcome where a capability depends on it.

Excluded from this report's work-item list (owned elsewhere per `📓️status.md`): hub compile/boot
fixes (H1), `HubInstance: ServerInstance` + durable storage profiles (W3b), the `🔀️dispatch` macro's
`Send`-future closing shape (D1).

---

## 1. Durable event log + snapshots

**REAL**, and it lives one layer below `🌎️hub` in `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db`
(hub's `db::Database` dependency), not inside `🌎️hub` itself:

- WAL: `🛢️db/📝️wal/🦀️.rs` (3012 lines). `WalRecordBatch`, `WalRecoveryReport`, `WalSegmentChain`,
  `WalAuthenticatedSource<S>` (integrity-checked replay), `WalReplayCursor`/`WalCommittedCursor`
  (`replay_document`/`replay_committed_document`/`replay_committed_segment`, lines 1960/2177/2181).
  `DurabilityClass::Fsync` vs. memory-buffered is an explicit enum
  (`🛢️db/📝️wal/🧪️tests/🔬️unit/🦀️.rs:934`), and `fsync_durability_forces_immediate_commit_regardless_of_policy`
  (line 1353) plus `wal_recovery_abort_fsync_survives_two_independent_filesystem_reopens` (line 243)
  are real crash-recovery laws, not just round-trip tests.
- Snapshots: `🛢️db/📸️snapshot/🦀️.rs` (1439 lines), tested at `🧪️tests/🔬️unit` and `🔬️retained`.
- Compaction: `🛢️db/🗜️compact/🦀️.rs` (2378 lines).
- Hub's own durable-storage switch (`connect_db`, `🌎️hub/🏗️bootstrap/🦀️.rs:8190-8223`) selects
  `fs` (default)/`sqlite`/`postgres`/`neo4j` via `OS_HUB_STORAGE_BACKEND`, all routed through this
  same `db::Database`.
- Artifact CAS on top of it: `🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🦀️.rs` (1672 lines) — chunked
  content-addressed blob storage with its own admission/sweep lane
  (`OS_HUB_ARTIFACT_CAS_SWEEP_EXECUTE` env var, `🏗️bootstrap/🦀️.rs` env list, §7).

**Verdict: REAL.** Not re-run in this pass (no cargo); prior audits (`📓️audit-hub-backend.md` §3,
H1 §1) confirm `db`/hub compile and the WAL/snapshot/compaction suites are part of the standard
`semio-hub` test surface.

## 2. Database / persistence abstraction

**REAL for the document/blob side, PARTIAL for the directory side.**

- Documents/blobs: `db::Database::open_at` (`connect_db`, `🏗️bootstrap/🦀️.rs:8190-8223`), fs default,
  sqlite/postgres/neo4j opt-in Cargo features (`🌎️hub/📦️packages/🦀️rust/Cargo.toml:15`,
  `db`'s own `Cargo.toml:28-30`). H2's memo (§1) found these now pull real driver crates
  (`sqlx-postgres`, `neo4rs`), a fix from the prior "empty features" state — **but H2 explicitly did
  not recompile with those features** ("not reverified by compiling," `📓️h2-server-crate-and-wave3-memo.md`
  is the server-crate memo, not this claim's source — the claim is in
  `📓️audit-hub-backend.md` §1, itself unreverified). No new evidence gathered here either; still
  open.
- Directory (identity/tenancy): `connect_directory` (`🏗️bootstrap/🦀️.rs:8251-8288`),
  `OS_HUB_DIRECTORY_BACKEND` default `sqlite` via `SqliteDirectory::connect`
  (`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs`). Postgres/Neo4j directory backends exist as source
  (`🌎️hub/📇️directory/🐘️postgres/🦀️.rs`, `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`, both with their own
  `🧪️tests/🔬️unit/🦀️.rs`) but per `📓️audit-collaboration.md` §6 were historically "written to
  parity but never compiled" — not reverified by a fresh compile in this pass either (no cargo
  allowed under this slice's rules).
- **A second, generic storage abstraction exists and is unreachable from hub**: `semio-framework-server`'s
  `AuthorityStore`/`ProjectionStore`/`BlobStore`/`SessionStore` traits
  (`🧰️framework/🛍️products/🖥️server/🔨️modules/🗄️storage/🦀️.rs`). Per W3a (`📓️w3a-server-instance-seams.md`
  §2-3), these are now generic over one `ServerInstance` (de-closed from the four in-memory-only
  variants), but **hub still does not depend on `semio-framework-server`** — confirmed again in this
  pass: `grep -rn "semio-framework-server" 🌎️hub/📦️packages/🦀️rust/Cargo.toml` → 0 hits. This is
  W3b's job (excluded from this report's work list).

## 3. Presence

**REAL, ephemeral, hub-owned, no durable persistence (by design).**

- `PresenceLeaseSlot`/`PresenceLeaseTransition`/`PresenceSnapshot`
  (`🏗️bootstrap/🦀️.rs:481-520`) — owner-bound lease slots with heartbeat, identity, label, role,
  surface, colour, opaque peer payload, scoped `(space, document, surface)`.
- Fan-out is `tokio::sync::broadcast`, confirmed by the crate doc comment
  (`🏗️bootstrap/🦀️.rs:1-13`, cited in `📓️audit-hub-backend.md` §1): presence/preview lanes are
  explicitly ephemeral, never written to `db::Database`.
- Directory-level presence: `DirectoryPresenceActor` (`🌎️hub/📇️directory/🦀️.rs`).
- Wire encoding shared with the client: `PresencePeer` binary codec,
  `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` (`presence_to_bytes`/decode, per
  `📓️audit-collaboration.md` §3), fixture `🧫️fixtures/👥️presence-peer-codec-v1`.
- Expiry is real (lease TTL, not just a map entry) per the prior audits' `presence_roster_is_scoped_per_surface`
  and three-client test citations (not rerun here).
- **Not re-verified this pass**: whether the `SHARED-PRESENCE-SESSION-COLORS...` wire extension
  (`ServerFrame::Session{actor,color}`, `views: Vec<PresenceWindowView>`) is now fully compiled
  end to end — `📓️audit-collaboration.md` §6 flagged two `PLACEHOLDER` test results there as of its
  last check; current `git status` shows `🧰️framework/🔨️modules/📡️replication` and `🖥️server`
  files still mid-edit by other slices, so this is a moving target, not re-measured here.

## 4. Auth

**PARTIAL, and the PARTIAL is structural, not incidental — there is no browser-facing login flow
of any kind.**

- **Session introspection only**: the sole `/auth/*` route is
  `/auth/sessions/me` (GET `get_session_me`, DELETE `delete_session_me`,
  `🏗️bootstrap/🦀️.rs:8119`, handlers at `:6514`/`:6535`). **There is no `POST /auth/sessions` (no
  login-mint route) anywhere in the 48-route table** (§8 below) — confirmed by grepping every
  `.route(` call.
- **Identity/session model exists in the directory schema but the only wired mint path is local,
  dev-only, HMAC-signed pipe frames**: `AuthSessionKind::{External, DevelopmentLocal}`
  (`🌎️hub/📇️directory/🦀️.rs:135`), `password_hash`/`sso_subject`/`sso_provider` fields on
  `UserRecord` (`📇️directory/🦀️.rs:73-75`), `create_user`/`get_user_by_sso_subject` trait methods
  (`:2880`, `:2885`). **No verification code exists anywhere under `🌎️hub` for any of these
  fields** — `grep -rniE "argon2|bcrypt|passkey|webauthn|oidc|oauth" 🌎️hub --include=*.rs` → 0 hits,
  and `grep -n "verify_password\|/auth/login\|OIDC" 🏗️bootstrap/🦀️.rs` → 0 hits. The fields are
  schema plumbing for a login flow that was never built server-side.
- **The one real mint path is `🚀️local-bootstrap`**: HMAC-authenticated frames over an inherited
  pipe fd (`hmacProof`/`authenticatedFrame`/`verifyAuthenticatedFrame`,
  `🌎️hub/🚀️local-bootstrap/🛂authentication/🟦️.ts:10-26`), consumed by the os client's
  `LocalHubCredential::read_inherited`. This is single-machine, dev-boot-only.
- **The "browser broker proof" (`#semio-broker=` URL fragment) is schema-and-fixture-only on the
  hub side**: `🌎️hub/🧬️schema/🔐️browser-broker-proof-lifecycle-v1/🔣️.json` and its fixture exist,
  but `grep -rn "BrokerProof\|broker_proof" 🌎️hub --include=*.rs` → **0 hits**. The only
  implementation of "browser broker" logic in the whole repo is client-side, inside the os worker
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts:502-640`) — it decodes and
  locally times out a proof string, but nothing under `🌎️hub` issues or validates one over HTTP.
  This reads as a **documented-but-unimplemented (STUB) hub-side capability**: the schema was
  authored, a client-side consumer was built against it, and the hub-side issuer never landed.
- **Authorization is real but ad hoc, not a reusable engine** — matches H2's finding
  (`📓️h2-server-crate-and-wave3-memo.md` §B.3): `authorized`/`authorized_for_blob`/
  `authorized_for_canonical_pair` (`🏗️bootstrap/🦀️.rs:1942/1948/1956`), `is_admin` (`:2197`),
  `admit_writes` against `db::security::SecurityGate`/`Principal`/`TenantId` (`:4007`). Per-route
  hand-written predicates, no `PolicyEngine`/templates (that exists only in
  `semio-framework-server`, unreachable from hub — §2).
- **Rate limiting: ABSENT.** `grep -rniE "rate.?limit|governor|throttle" 🌎️hub --include=*.rs` →
  **0 hits**, in the whole `🌎️hub` tree. No `tower_governor`, no token bucket, nothing. Any hub
  endpoint (including `/directory/commands`, `/spaces/*/documents/*/socket-grants`, invite
  redemption) can be hit at unlimited rate today.

## 5. Directory (users, spaces, membership, invites)

**REAL.** Event-sourced command log + read model (`load_read_model`, `🏗️bootstrap/🦀️.rs:5102`,
per H2 §B.3, 17 255 lines). Invites are a real, multi-state flow, not a stub:
`InviteRecord`/`SpaceAdministrationInviteRow`/`IssuedInvite`
(`🌎️hub/📇️directory/🦀️.rs:227/254/310`), `prepare_invite` (`:1045`),
`invite_redemption_preflight` with named preflight states
(`InviteRedemptionPreflight`, `:1108-1118`) and a scope-hint verifier (`:1081`). Routes:
`/directory/spaces`, `/directory/spaces/{id}`, `/directory/invites/{token}/redeem`,
`/directory/commands`, `/directory/events`, `/directory/event-page/v1`,
`/directory/socket-grants`, `/directory/socket/v1` (§8). 54 `#[test]`/`#[tokio::test]` attributes
across 6 files under `📇️directory` (this pass's own count).

## 6. Artifact authority

**REAL, CQRS/event-sourced, no CRDT.** Trusted catalog (`🔏️trusted-catalog`), chunk CAS
(`🧱️chunk-cas/🦀️.rs`, 1672 lines), creation actor with admission/cancellation
(`🌱️creation`, `🏗️bootstrap/🦀️.rs:4582`/`:4929-5026`), file-fence, native-openable-provider,
adapters. Conflict policy confirmed by `📓️audit-collaboration.md` §1:
`ApplyOutcome::{Transformed, Rejected}` (`📡️replication/📡️wire/🦀️.rs:404-427`) — server-authoritative
rebase/reject, matching `AGENTS.md`'s "no CRDT" mandate. 61 `#[test]`/`#[tokio::test]` attributes
across 12 files. H1 §6.1 also documents a real, now-fixed structural bug here: the trusted-catalog
root walk used `O_NOFOLLOW`-everywhere and broke on any symlinked `OS_HUB_DATA` ancestor (e.g.
macOS `/var` → `/private/var`) — fixed at `🛡️opened-root/🦀️.rs:44-52`, with a regression law added.

## 7. Inference

**PARTIAL — the mechanics (approval/cancel/progress) are real and substantial, but scoped to
exactly one artifact kind.** This is not a generic LLM-inference service.

- 3849 Rust lines across `💡️inference/{🦀️.rs, 🏃️runtime/🦀️.rs, ✉️command/🦀️.rs}` (121+3526+202),
  all named `GisMap*` (`GisMapApprovalRequestTokenV1`, `GisMapApprovalCommitRequestV1`,
  `GisMapApprovalUndoCommitRequestV1`, `🏃️runtime/🦀️.rs:161-270`). `grep -rliE
  "anthropic|openai|claude|llm|model_provider" 🌎️hub/💡️inference` → **0 hits** — there is no
  provider abstraction of any kind.
- Real cancellation (`cancel_run`, `🪶️sqlite/🦀️.rs:465`), real approval/undo with durable ledger
  rows (`InferenceApprovalOutboxV1`/`InferenceApprovalPageV1`, `:124/133`), real progress via SSE-
  style job events (`get_inference_gis_map_job_events` route).
- Routes: `/spaces/{s}/documents/{d}/inference/gis-map/jobs` (POST), `.../jobs/reconcile` (POST),
  `.../jobs/{id}/events` (GET), `.../jobs/{id}/cancel` (POST), `.../jobs/{id}/approval` (POST),
  `.../approval-undos` (POST) — 6 routes, all GIS-map-specific (`🏗️bootstrap/🦀️.rs:8085-8090`).
- **The MCP-side "AI inference" surface for other artifact kinds explicitly reports itself
  unwired**: `execution_not_wired_error`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:177`) is a named function
  generating the error every non-GIS declared inference hits. This matches `📓️audit-ai-mcp.md`'s
  finding verbatim ("inference not-wired except GIS") and `📓️m2-agent-surface-and-inference.md`'s
  `AppCommand::Infer`/`inference_run` MCP tool, whose only live backend is confirmed by this pass's
  grep: `🌉️mcp/💡️inference/🦀️.rs:888` posts to the same
  `/spaces/{s}/documents/{d}/inference/gis-map/jobs` route.
- **Verdict**: REAL for one artifact kind's approve/cancel/progress lifecycle; STUB (explicitly
  self-reporting as such via `execution_not_wired_error`) for every other declared inference; no
  queue abstraction, no provider abstraction exists at the hub layer at all — a generic AI/chat
  inference request would need its own backend entirely, not a variant of this one.

## 8. HTTP / WebSocket surface — every route

48 `.route(` calls in `🏗️bootstrap/🦀️.rs:8085-8159` (this pass's own count, matching H1's "45"
approximate plus the 3-line multi-line entry at `:8149-8152`):

| group | routes |
|---|---|
| health | `GET /readyz` (**no `/healthz`**) |
| auth | `GET+DELETE /auth/sessions/me` |
| directory | `POST /directory/commands`, `GET /directory/spaces`, `GET /directory/spaces/{id}`, `POST /directory/invites/{token}/redeem`, `GET /directory/events`, `GET /directory/event-page/v1`, `POST /directory/socket-grants`, `GET /directory/socket/v1`, `POST /directory/spaces/{s}/documents/{d}/socket-grants`, `GET /directory/spaces/{s}/documents/{d}/socket/v1` |
| admin | `GET /admin/api/{overview,spaces,spaces/{id},users,connections,documents,events,operations/{id},audit}`, `POST /admin/api/operations/{id}/cancel`, `POST /admin/api/intents`, `GET /admin`, `GET /admin/`, `GET /admin/{*path}` |
| extension modules | `GET /🧩extension-modules`, `GET /🧩extension-modules/{id}/{*rest}` |
| blobs | `GET/HEAD/PUT /spaces/{s}/blobs/{hash}` |
| documents | `GET /spaces/{s}/documents/{id}`, `GET .../active-checkpoint/pair`, `POST .../checkpoint-publications`, `POST .../open-plan`, `POST .../socket-grants`, `POST .../execution-target/{manifest,component,descriptor,browser-actor}`, `GET .../socket/v1` |
| artifact creation | `GET+POST /spaces/{s}/artifact-creations`, `GET .../{request_id}`, `POST .../{request_id}/cancel` |
| inference | 6 GIS-map routes (§7) |

**Not called by any client at all** (grepped across `🧰️framework/🛍️products/💻️os` and `🌎️hub`
itself): `POST /spaces/{s}/documents/{d}/checkpoint-publications`. `checkpoint_publication`
concepts exist client-side (`🛢️db/🗿️artifact/🦀️.rs`, `🛢️db/⚙️engine/🦀️.rs`) but nothing posts to
this specific HTTP route from the os product — either the write happens over the document
WebSocket instead (plausible, not confirmed) or the route is dead. Flagged as a mismatch (§10).

## 9. Health/readiness, config/env, observability

- **Readiness**: `/readyz` reports per-subsystem booleans — `directory`, `storage`,
  `adminAssets`, `artifactCasBarrier`, `artifactPublication`, `artifactCasSweeper`,
  `artifactAuthority` (H1 §6.2, observed on a real boot). No `/healthz`, no liveness-vs-readiness
  split.
- **Config/env**: 14 `OS_HUB_*` variables read directly in `🏗️bootstrap/🦀️.rs`:
  `ADMIN_DIR`, `ADMIN_SUBJECTS`, `ARTIFACT_CAS_SWEEP_EXECUTE`, `BIND`, `DATA`,
  `DATABASE_URL`, `DB_SQLITE`, `DIRECTORY_BACKEND`, `DIRECTORY_DATABASE_URL`,
  `EXTENSIONS_DIR`, `MERGE_POLICY`, `MODE`, `PORT`, `STORAGE_BACKEND`,
  `TEST_INFERENCE_CHECKPOINT_FD` — no schema/validation layer over these (plain
  `std::env::var` reads), no `.env` file support seen, no central config struct.
- **Observability: ABSENT.** `grep -rn "tracing::" 🌎️hub --include=*.rs` → **0 hits** in the whole
  hub tree. No `tracing`/`log` dependency in `🌎️hub/📦️packages/🦀️rust/Cargo.toml`. No
  `prometheus`/`metrics::`/`opentelemetry` anywhere under `🌎️hub`
  (`grep -rln "prometheus|metrics::|opentelemetry" 🌎️hub` → 0 hits). The only diagnostic output in
  the main bootstrap file is 9 `println!`/`eprintln!` calls. There is no structured logging, no
  request tracing, no metrics endpoint, nothing a production deployment could scrape or correlate.

## 10. Integration harness, and OS-frontend-vs-hub mismatches

- **`🤝️integration-harness`** (`🌎️hub/🤝️integration-harness/🟦️.ts`) really does start the compiled
  hub binary and poll real HTTP (`startHub`/`resolveHubBinaryPath`/`waitForHttpReady`, per
  `📓️audit-collaboration.md` §5) — not a mock. But its consumer,
  `🌎️hub/🧪️tests/🤝️integration/🟦️.ts` (~14 `describe` blocks), is **contract/schema tests**
  (checkpoint hashing, socket-grant HMAC recomputation, trusted-catalog closure, chunk-CAS,
  lag-rebootstrap encoding) — **none of them drive two live document sockets against each other**.
  The actual multi-client collaboration E2E lives in a different product entirely:
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts`
  (`collabRunScenario`), last measured 2/8 steps (owned by C1, not this slice).
- **Schema-first vs. generated drift**: every hub subsystem has a `🧬️schema/🔣️.json` with
  Rust/TS twins (`💡️inference`, `🔐️auth`, `🚀️local-bootstrap`, `🛰️lag-rebootstrap`,
  `🗿️artifact-authority/{🌱️creation,🔏️trusted-catalog}`). The nx targets that check twin agreement
  (`foundation-source-check`, `socket-grant-command-source-check`,
  `🌎️hub/📦️packages/🦀️rust/📜️script.ts`) are themselves TS files under `🌎️hub/🧪️tests/**` — and one
  of them (`🧱️socket-grant-command-source/🏃️execution/🟦️.ts`) had a live off-by-one import bug
  (H1 §7 finding #2) that broke `os-hub:dev` before reaching any build step, meaning the drift
  checker's own runner was broken until H1 fixed it. Current `git status` shows this file plus
  `🌎️hub/🧪️tests/🤝️integration/🟦️.ts` still mid-edit — re-verify after H1 lands.
- **OS-frontend call sites actually observed** (grepped `🔌️directory/🔌️client/🦀️.rs`,
  `🌉️mcp/🏠️workspace/🔗️remote/🧩️pair/🦀️.rs`, `🌉️mcp/💡️inference/🦀️.rs`): `/directory/spaces`,
  `/directory/spaces/{id}`, `/directory/events`, `/directory/event-page/v1`, `/auth/sessions/me`,
  `/spaces/{s}/documents/{d}/execution-target/{manifest,descriptor}`, `/directory/commands`,
  `/spaces/{s}/documents/{d}/inference/gis-map{...}`, `/directory/socket-grants`,
  `/directory/spaces/{s}/documents/{d}`, `/spaces/{s}/documents/{d}` (GET),
  `/spaces/{s}/documents/{d}/active-checkpoint/pair`,
  `/spaces/{s}/documents/{d}/inference/gis-map/{jobs,approval-undos}`. The `✏️s/🔌️plugins/🪐️space`
  plugin itself never calls hub HTTP directly — it goes through `Effect::Navigate` (client-side
  routing only) and the shared `🏪️store`/`🔄️sync` actor, confirming the architecture in
  `📓️audit-collaboration.md` §1 (plugins never touch the wire; only the os-kernel actor does).
- **Mismatches found**:
  1. Hub serves `POST /spaces/{s}/documents/{d}/checkpoint-publications`; **no HTTP caller exists**
     anywhere in the os product (§8). Either dead code or the write path moved entirely onto the
     document WebSocket — undetermined from static reading alone.
  2. Hub serves `execution-target/component` and `execution-target/browser-actor`; the observed
     client call sites only exercise `manifest` and `descriptor`
     (`📇️directory/🔌️client/🦀️.rs:989,1015`). The other two may be called from the wgpu/native
     path (not grepped as thoroughly here) — flagged, not confirmed dead.
  3. Hub exposes 6 GIS-map inference routes and only they are wired end to end; every other
     declared-inference artifact kind hits `execution_not_wired_error` client-side (§7) — this is
     already known (audit-ai-mcp), restated here with the exact function name for a future fix
     target.
  4. Hub has no `POST /auth/sessions` (login/mint) route at all (§4); the os client's
     `SessionMintResponse`/broker-proof consumption code (`📇️directory/🔌️client/🦀️.rs:216`,
     `🏪️store/👷️worker/🟦️.ts:502-640`) has no hub-side issuer to call for the non-local-bootstrap
     (browser broker) case — this is the same STUB finding as §4, restated as a client/server gap.

---

## 11. Test counts per hub area (static `#[test]`/`#[tokio::test]` attribute count, this pass)

| area | files with tests | test attrs |
|---|---|---|
| `💡️inference` | 10 | 27 |
| `📇️directory` | 6 | 54 |
| `🔐️auth` | 1 | 1 |
| `🗿️artifact-authority` | 12 | 61 |
| `🚀️local-bootstrap` | 1 | 3 |
| `🚀️local-relay` | 0 | 0 |
| `🛰️lag-rebootstrap` | 1 | 6 |
| `🔨️modules` (admin) | 1 | 12 |
| `🧪️tests` (bin-unit, via `#[path]` from `🏗️bootstrap`) | 2 | 97 |
| `🏗️bootstrap`, `🏗️build`, `🤝️integration-harness`, `🧫️fixtures`, `🧬️schema` | 0 | 0 (no inline `#[test]`; bootstrap's suite is the `🧪️tests` row via `#[path]`) |

None of these were executed in this pass (no cargo, per slice rules) — treat as static inventory
only; H1's report is the source for actual green/red counts once its blocker (peer stdio-pdf
rewrite) clears.

---

## 12. Ranked work items (excludes H1 compile/boot, W3b `ServerInstance`/durable stores, D1 dispatch macro)

**P0**

1. **No login/session-mint route exists on the hub for anything but local-bootstrap.**
   `POST /auth/sessions` (or equivalent) needs to exist and either verify `password_hash` or
   implement the browser-broker-proof issuer the schema already describes
   (`🌎️hub/🧬️schema/🔐️browser-broker-proof-lifecycle-v1/🔣️.json`) in Rust under `🌎️hub` — today
   the schema and the client-side consumer (`🏪️store/👷️worker/🟦️.ts:502-640`) both exist with
   **nothing on the hub side implementing it** (§4, §10.5 mismatch #4). Any non-dev, non-local
   deployment of the hub currently has no way for a browser to authenticate at all.
2. **Rate limiting is completely absent** (§4) — `/directory/commands`, invite redemption, and
   socket-grant issuance are all unlimited-rate today. Add at minimum a per-IP/per-principal
   token bucket in front of the axum router in `🏗️bootstrap/🦀️.rs`.

**P1**

3. **Observability is absent** (§9) — no `tracing` dependency, 0 `tracing::` call sites, no
   metrics endpoint. Add `tracing`/`tracing-subscriber` to
   `🌎️hub/📦️packages/🦀️rust/Cargo.toml` and instrument at minimum the WS handlers
   (`document_ws_v1`, `directory_ws_v1`) and the directory command path
   (`post_directory_commands`) — currently un-debuggable in production beyond `println!`.
4. **Authorization is 5 hand-written predicate functions, not a reusable system** (§4,
   `🏗️bootstrap/🦀️.rs:1942/1948/1956/2197/4007`) — this is the same finding H2's memo already
   scoped as Wave-3 item 7, but it is independently exploitable risk today (easy to add a new
   route and forget one of the five checks). Not blocking on Wave 3: a lightweight audit of every
   route against these five predicates (which route uses which, which uses none) would catch gaps
   now, cheaper than waiting for the full `ServerInstance` migration.
5. **`checkpoint-publications` route has no known caller** (§10 mismatch #1) — resolve whether
   this is dead code (delete it) or the intended path for a not-yet-built client feature (wire it)
   before the Wave-3 module-split work (H2 §B.5 item 9) moves this route into a `ServerModule`
   and inherits the ambiguity.
6. **Postgres/Neo4j directory + storage backends still unverified by compilation** (§2) — audit's
   claim that db's features now pull real drivers has not been recompiled with
   `--features postgres,neo4j` by anyone since. A single `cargo check -p semio-hub --no-default-features
   --features postgres` / `--features neo4j` run (foreground, `-p` scoped) would close this out
   one way or the other.
7. **Inference is single-artifact-kind only, and the gap is self-reported** (§7,
   `execution_not_wired_error`, `🌉️mcp/💡️inference/🦀️.rs:177`) — if a general "AI chat"/agent
   inference surface is a stated goal (per `📓️m2-agent-surface-and-inference.md`), it needs its
   own hub-side job/queue lane analogous to `💡️inference/🏃️runtime/🦀️.rs`, not a GIS-map variant.

**P2**

8. **No `/healthz` vs `/readyz` split** (§9) — cheap to add if any external orchestrator expects
   liveness separate from readiness; currently only `/readyz`.
9. **`execution-target/component` and `/execution-target/browser-actor` routes' callers not
   confirmed** (§10 mismatch #2) — a quick grep of the wgpu/native client path (not covered by
   this pass's TS/Rust grep of the os-directory client) would confirm or refute a second dead
   route.
10. **`🚀️local-relay` is a 68-line, single-file path-admission allowlist** (§ local-relay,
    `🌎️hub/🚀️local-relay/🧭️routing/🟦️.ts`) — real but thin; worth confirming it is actually
    consumed by something (a native relay/proxy) rather than orphaned, since this pass did not
    trace its caller.

---

Report file: `📓️g2-hub-depth-audit.md` (this file).
