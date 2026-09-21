# G16 — depth re-audit of outcome 2 ("a working server hub backend") against the CURRENT tree

Read-only, 2026-09-21, Sonnet 5. No builds, no servers, no edits outside this file. Method: read G2
(2026-09-19) and `audit-hub-backend.md` (earlier still) as the *prior* baseline, then re-grepped/read
the current source tree plus every H1/H1b/HT1–HT12/HS1/OB1r/P4/TC1/TC2/TC2b/GM1/C3/M6/M6b report to see
what changed. **Headline: G2's audit is two days stale on auth, rate-limiting and observability** — all
three of G2's "P0: absent" findings (`§4` login route, `§4` rate limiting, `§9` observability) are now
real, shipped code, most of it live-probed. What is still open is narrower and different: Postgres/
Neo4j are still never-connected-to, the hub test suite's last reds are fixed-in-source-but-unverified,
and artifact CREATION on a hub is still limited to three plugin kinds (stdio/gis/vcs) by a static const
table — the TC3 gap G2 didn't know to ask about is real and still unfixed.

Classification used throughout: **OBSERVED-AT-RUNTIME** (a probe/script hit a real booted `os-hub`
process over real HTTP/TCP and printed the result), **TEST-ONLY** (an in-process `#[tokio::test]`
exercises it, no external process), **COMPILED-ONLY** (`cargo check`/`cargo build` passed but the code
path has not executed since being written — usually because of "rule 26": workers in this ticket may
not run `cargo test`/`nextest`/`build -p semio-hub`, only the coordinator may), **OPEN** (missing, or
found broken and not yet fixed).

---

## (a) Durable storage

**sqlite — OBSERVED-AT-RUNTIME, kill+restart persistence proven twice, independently.**
- H1b §17/§28 (`🐍️h1b-hub-runtime-probe.ts`): boots `os-hub`, creates a space + two members, **kills
  the process**, boots the **same `OS_HUB_DATA`** again, and re-reads: `44 passed, 0 failed`
  (`h1-hub-build-and-boot.md:1082-1113`). The membership roster survives byte-exact
  (`[[ada,"author"],[bo,"spectator"]]`), the credential re-mints, presence is `[]` after restart (by
  contract, ephemeral).
- P4 §8d (`📜️p4-production-runtime.sh`): a **separate**, production-mode boot, SIGTERM, restart, same
  `user_id` persists (extracted verbatim above from the P4 sub-report).
- Code: `connect_db` (`🌎️hub/🏗️bootstrap/🦀️.rs:8190-8223`, line numbers per G2, not re-walked digit-
  by-digit this pass but the function is unchanged in `git status`), `fs` default rooted at
  `{data}/db`; `OS_HUB_DB_SQLITE` path for the `sqlite` backend.

**postgres/neo4j lanes — COMPILED-ONLY, never connected to, on this host or apparently anywhere in
this ticket.**
- `cargo check -p semio-hub --lib --no-default-features --features postgres` → 0 errors, 111 warnings,
  16m38s; `--features neo4j` → 0 errors, 110 warnings, 2m59s (H1b §25, `h1-hub-build-and-boot.md:987-990`).
  That is the **only** verification either backend has ever received in this ticket — a type-check,
  not a socket opened to a real database.
- Grepped this pass across every `📓️*.md` in the ticket for `postgres://`/`bolt://`/a real
  `OS_HUB_DATABASE_URL` value: **zero hits** outside the README's documentation and G2's own
  restatement of the Cargo feature wiring. No report shows a `psql`/`cypher-shell` session, a docker-
  compose file for either database actually started, or a hub boot with `OS_HUB_STORAGE_BACKEND=postgres`
  answering `/readyz`.
- **What it would take on this host, stated exactly.** This machine has Docker Desktop **installed**
  at `/usr/local/bin/docker` (client v29.5.3) but the daemon is **not running**
  (`docker ps` → `failed to connect to the docker API … no such file or directory`, checked live this
  pass) — contradicting `🌎️hub/README.md:427-445`'s and P4's "Docker is not installed on this machine"
  framing, which is now stale for the CLI (the daemon still needs starting). Two paths, neither
  exercised by anyone in this ticket:
  1. **Start Docker Desktop** (`open -a Docker`, wait ~30-60s for the daemon socket), then
     `docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=x postgres:16` and
     `OS_HUB_DATABASE_URL=postgres://postgres:x@127.0.0.1:5432/postgres OS_HUB_STORAGE_BACKEND=postgres
     cargo run -p semio-hub --features postgres --bin os-hub -- …` — cheapest path once the daemon is up.
  2. **Homebrew, fully Docker-less**: `brew install postgresql@16 neo4j`, `brew services start
     postgresql@16` (or `pg_ctl -D /usr/local/var/postgresql@16 start`), `createdb semio_hub_dev`, point
     `OS_HUB_DATABASE_URL` at it. Neither `postgresql` nor `neo4j` is currently installed via brew on
     this host (checked live this pass: `brew list --formula | grep -iE "postgres|neo4j"` → empty).
  Either path is a same-day task; nobody in this 3-day ticket did it. **Rank this a concrete slice
  (see §12, slice 6).**

**Store versioning / migrations — REAL, and it is the repo's stated policy, not an oversight.**
- `🌎️hub/🗄️stores/🦀️.rs:248-255`: `STORE_FORMAT_FILE = "format.json"`,
  `STORE_FORMAT_SCHEMA = "semio/hub/store-format/v1"`, `STORE_FORMAT_VERSION = 1`.
- `:294` warns and adopts an unstamped pre-existing store at the current version (backward
  compatibility for data written before this stamp existed); `:301` refuses a wrong schema string;
  `:306-308` refuses a **newer**-than-this-binary store by name; `:313-315` refuses an **older**-than-
  this-binary store by name, verbatim: *"there is no migration framework, so this data root must be
  opened by the build that wrote it"* — directly downstream of `AGENTS.md:70-71`'s repo-wide "no
  migration scripts … handcraft all assets… manually fix all assets… all at once" rule, and restated
  independently for the directory backend at `📇️directory/🐘️postgres/🦀️.rs` ("greenfield: there are
  no users yet, so schema changes are edited in place, not migrated" — quoted in `README.md:322-323`).
- OBSERVED-AT-RUNTIME: P4 §8d hand-edited a `format.json` to v2 and the hub refused to boot with the
  named message, exit status 1 (extracted above). This is a real, tested refusal, not aspirational.
- **Consequence, stated plainly for outcome 2's "working" bar**: this hub has **no upgrade story**. A
  production deployment across a binary change must back up, stop, and accept "start from empty if the
  new binary can't read the old root" (`README.md:320-321`). That is a documented policy, not a hidden
  gap — but it is a real limit on "working" for anyone who wants continuity across hub upgrades.

---

## (b) Event sourcing / CQRS

**Command path + event log + read models — REAL, OBSERVED-AT-RUNTIME.** Directory is event-sourced
(`load_read_model`, G2 §5, 17k+ lines, 54 test attrs across 6 files) and the runtime probe drives real
commands through `/directory/commands` and reads them back after a restart (H1b §17 row 3). Artifact
authority is CQRS/event-sourced with no CRDT — server-authoritative rebase/reject
(`ApplyOutcome::{Transformed, Rejected}`, G2 §6, unchanged this pass).

**Sagas — the wiring runs, but there is nothing registered to run: literally zero sagas exist today.**
- `HubSagas` is declared `pub enum HubSagas {}` at `🌎️hub/🗄️stores/🦀️.rs:120` — an **uninhabited**
  enum, i.e. a Rust type with no values that can ever be constructed. `impl server::authority::Saga
  for HubSagas` at `:122` is a trait impl over a type nothing can instantiate.
- `SagaDrainSupervisor` (`🌎️hub/🏗️bootstrap/🦀️.rs:469-521`) is real machinery: it runs
  `drain_instance_sagas` every `SAGA_DRAIN_INTERVAL = 500ms` (`:450`) in batches of
  `SAGA_DRAIN_BATCH = 64` (`:453`), started after the listener binds (`:10197`) and drained one final
  time on shutdown (`:10264`, `SagaDrainSupervisor::shutdown`, `:497-514`).
  `ServerState::drain_sagas` lives at `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs:784`.
- OB1r's own words (extracted this pass, `📓️ob1r-trace-and-hub-observability.md` §5): *"hub registers
  no deciders and no sagas yet (`HubSagas` is an uninhabited enum), so today every pass moves zero rows
  and reports nothing."* OB1r added the law `the_saga_drain_supervisor_stops_and_drains_once_more_on_
  shutdown`, which is TEST-ONLY but PASSED per the coordinator's 09:17 nextest capture.
- **Verdict: the saga *runner* is production wiring (real supervisor, real cadence, real drain-on-
  shutdown, tested) but there is no saga decider registered anywhere in this hub. "Nothing is a saga
  today" is the correct, literal reading** — the CQRS command/event/projection spine is real; the
  cross-aggregate orchestration layer CQRS usually calls "sagas" is an empty type with a heartbeat.
- **Snapshots vs events**: real and separate from the saga question — WAL + snapshot + compaction
  live one layer below hub in `db::Database` (G2 §1, unchanged), confirmed still wired via
  `connect_db`.

---

## (c) Presence

**REAL, ephemeral by design, OBSERVED-AT-RUNTIME for the directory socket; the document-socket roster
is asymmetric/decaying in the one live two-human run this ticket has ever produced, and that defect
is unroot-caused.**
- Lease slots: `PresenceLeaseSlot` (`🏗️bootstrap/🦀️.rs:616`), TTL `PRESENCE_LEASE_TTL_MS = 15_000`
  (`:919`), installed/refreshed via `install_presence_slot` (`:2009`) and minted at `:4951-4953`.
  Presence is `tokio::sync::broadcast` fan-out, never written to `db::Database` (G2 §3, unchanged).
- H1b §17 row 7 (OBSERVED-AT-RUNTIME): a real `/directory/socket/v1` upgrade + grant + `SocketHelloV1`
  handshake, 3 live event frames delivered to a joined peer for commands the *other* principal issued
  over HTTP, then silence after `close()` — this is the directory-level socket, proven live.
- C3 §3.4 (OBSERVED-AT-RUNTIME, the only two-human run in the ticket): *"user1's roster listed both
  peers with distinct colours; user2's listed only itself, in every run — and in `c3final` the
  asymmetry reversed… Reading the roster at the END of a run showed it empty in both contexts…
  Presence beats ride the document socket, and the socket stays open the whole time, so an empty
  roster on a live socket is a real defect — not investigated here"* (`c3-two-user-collaboration-
  scenario.md:192-200`). This is **OPEN**, filed by C3 as an honest, un-root-caused gap (§5: "The
  presence roster asymmetry (§3.4) was not root-caused"), possibly downstream of the browser-actor
  activation defect (§3.2) that also blocked live editing in the same run.
- Document-scoped `ServerFrame::Presence{peers}` roster growth/shrink specifically: H1b §17.2 says its
  own probe does **not** exercise this (needs a committed document genesis, blocked by the
  artifactAuthority gate) — the in-process laws (`presence_lease_*`, 6 laws) cover it TEST-ONLY, real
  TCP, not a real two-client browser session. M6b's agent-presence row (step 6 of its live proof) is
  the same story: **not observed**, blocked on the same gate (`m6-agent-principal-in-hub-space.md:733-
  743`).
- **Eviction**: TTL expiry is real (lease TTL, not a bare map entry) per prior audits' three-client
  test; not independently re-verified this pass beyond the citations above.

---

## (d) Auth — the single biggest delta from G2's audit

G2 (2026-09-19) found: no login route, no password verification code anywhere, no rate limiting. **All
three are now false.** This is real, substantial work landed by AU1/AU3/M6/M6b/P4 in the ~48 hours
since G2's pass.

**Credential sign-in — OBSERVED-AT-RUNTIME.**
- `POST /auth/sessions` is registered at `🏗️bootstrap/🦀️.rs:9720` via
  `semio_hub::auth::SESSION_MINT_ROUTE` (defined `🔐️auth/🦀️.rs:32`, `"/auth/sessions"`); handler
  `post_auth_session`, `DefaultBodyLimit::max(SIGN_IN_REQUEST_MAX_BYTES)`.
- Password scheme: PBKDF2-HMAC-SHA256 over the repo's own `Sha256` (no external crypto dependency,
  `🔐️auth/🔑️password/🦀️.rs:1-3`), `DEFAULT_ITERATIONS = 210_000` (`:17`, OWASP's 2023 floor),
  constant-time verification (`PasswordCredentialV1::verify`, `:108-113`, via
  `constant_time_digest_eq`). This is a real, competently-implemented credential scheme — not argon2/
  bcrypt (both are external crates the repo's no-external-runtime-deps rule forbids), but a correct,
  parameterized PBKDF2 with a documented cost floor.
- H1b §17 rows 4-6 (OBSERVED-AT-RUNTIME): mint → `200` + `session.v1.<hex>.<hex>` token; introspect;
  revoke → `204` then `401`; **expiry is server-clocked**: booted at `OS_HUB_SESSION_TTL_SECONDS=60`,
  the probe waits 63s, `/auth/sessions/me` flips to `401` with no client action.
  `OS_HUB_SESSION_TTL_SECONDS` default is 43200 (12h), bounds `60..=31536000` (`README.md:169`).
- `README.md:12-38` ("Read this first"): `os-hub credential set` seeds the first user with no server
  running; `OS_HUB_CREDENTIAL_SIGN_IN=true` (default `false`) is what makes **production mode**
  reachable without an external IdP — this used to be impossible (the old gate hard-required an
  `IdentityAssertionVerifier`), and the doc itself narrates the exact "used to be unreachable" history
  that matches `🏗️bootstrap/🦀️.rs:2599-2616`'s doc comment verbatim.

**Production `identity_verifier` — confirmed still `None`, i.e. no external IdP, exactly as designed
and documented, not a bug.**
`let identity_verifier: Option<Arc<dyn IdentityAssertionVerifier>> = None;` at
`🏗️bootstrap/🦀️.rs:10007`. `validate_auth_startup` (`:2620-2645`) admits production either via this
adapter (none exists) or via `credential_sign_in_enabled` (real, and now the only path). This is the
correct OPEN item to carry forward, but it is **known and load-bearing**, not an oversight — SSO/OIDC/
WebAuthn integration is a real, unstarted feature, not a defect.

**Rate limiting — REAL, OBSERVED-AT-RUNTIME, contradicting G2's "0 hits" finding entirely.**
- `🔐️auth/🚦️rate-limit/🦀️.rs`: millisecond-budget token buckets, four classes
  (`RateLimitClassV1::{Auth, DirectoryCommand, InviteRedemption, SocketGrant}`, `:35-40`), policies at
  `:55-58` — Auth `burst:10, cost_ms:6000`; DirectoryCommand `burst:60, cost_ms:100`; InviteRedemption
  `burst:10, cost_ms:6000`; SocketGrant `burst:30, cost_ms:200`. Subjects are hashed, never raw
  (`RateLimitSubjectV1::{remote_address, principal, claimed_identity}`, `:80-101`, domain-separated
  SHA-256 digests) — a sign-in attempt is charged against **both** the remote address and the claimed
  email (`claimed_identity`, `:91-94`), so one address cannot spray many accounts and one account
  cannot be sprayed from many addresses without also charging the shared identity bucket.
- H1b §17 row 8 (OBSERVED-AT-RUNTIME): the auth bucket refuses with `429 {error:"rate-limited"}` and
  an integer `retry-after ≥ 1`; a **correct** password is then also refused `429` — i.e. the bucket
  charges per remote address regardless of outcome, exactly matching the policy's design.

**Agent delegations — REAL, and the fullest live-proof in this whole audit: 6 of 7 steps observed
end-to-end through a real MCP process talking to a real hub.**
- Three routes, all event-sourced through the same log sessions use: `POST /auth/agent-delegations`
  (create, `AGENT_DELEGATION_ROUTE = "/auth/agent-delegations"`, `🔐️auth/🤖️agent/🦀️.rs:22`),
  `GET .../agent-delegations` (list), `DELETE .../agent-delegations/{id}` (revoke,
  `🏗️bootstrap/🦀️.rs:9724`), `POST /auth/agent-sessions` (exchange, `AGENT_SESSION_ROUTE`, `:23`).
  TTL bounds `MIN_DELEGATION_TTL_SECS=60 .. MAX_DELEGATION_TTL_SECS=90d` (`:39-40`), default 7 days
  (`:38`); the minted agent *session* is short (`AGENT_SESSION_TTL_SECS = 3600`, `:44`) — the agent
  re-exchanges hourly and each exchange re-reads revocation state.
- M6b §M6b.4 (OBSERVED-AT-RUNTIME, production posture, real MCP binary, real hub on :7611, no launcher,
  no fd 3): steps 1 (sign-in), 2 (`201` delegation, token shown once), 3 (credential file mode `0600`),
  4 (`semio-os-mcp stdio --hub … --credential-file` prints `acting as agent principal agent:<id>
  ("Drafting agent") in space <id>` and JSON-RPC `initialize` answers), 5 (`context_resolve` reports
  `"principal": "agent:<delegation id>"`), 7 (revoke → `204`, already-minted session cascades to `401`,
  a fresh spawn with the same credential file exits non-zero with `PermissionDenied: … delegation-
  revoked`) — **all five observed live**. Two real defects were found and fixed in the same run: (A) a
  sealed one-core worker pool that made every `--hub --credential-file` invocation on a multi-core
  machine panic at exit 101 (`🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs`), (B) `context_resolve` reporting the
  launcher's default principal instead of the exchanged agent principal (`🌉️mcp/🦀️.rs`). Both fixes
  were rebuilt and **re-observed**, not merely compiled.
- Step 6 (a directory socket roster showing `principalKind: "agent"`) is the **one not observed** —
  it needs a document socket, which needs the artifactAuthority gate open, which needs a published
  trusted catalog (§e below). The hub-side label work for this (`agent_actor_display`, presence label
  falling back correctly) is **COMPILED-ONLY** (M6b §M6b.2.5, §M6b.6.1).
- **Password policy**: `PASSWORD_MIN_BYTES=8, PASSWORD_MAX_BYTES=256` (`🔑️password/🦀️.rs:23-24`) — a
  length floor only, no complexity/entropy rule, no breach-list check. Reasonable for a self-hosted
  single-tenant-per-space hub, thin for anything else.
- **Credential delivery**: the one-time delegation token is rendered into a downloadable file **inside
  the client hook**, never the raw string except as a fallback when the browser refuses to save
  (M6b §M6b.1.3) — a real, deliberate anti-leak design, and a law asserts the token string is absent
  from the DOM on the healthy path. File mode `0600` is written and verified (M6b §M6b.4 step 3).
- **The three hub-side integration laws for the whole delegate→session→use→revoke→refuse lifecycle are
  written and `cargo check -p semio-hub --all-targets` is 0 errors, but are COMPILED-ONLY — "needs hub
  rerun"** (M6b §M6b.3, three named laws at `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` region `🔖️AgentDelegation`).

**CORS / network-bind posture — REAL in source, `cargo check` green, loopback path OBSERVED-AT-
RUNTIME, network-bind path never run by anyone.**
- `CrossOriginPolicyV1::{LoopbackDevelopment, Allowlist(Arc<[String]>), Closed}`; `Vary: Origin`
  always appended; `OS_HUB_ALLOWED_ORIGINS` up to 32 entries, exact-origin comparison, no wildcards
  (`README.md:236-271`). 3 laws, one over a real TCP socket
  (`a_refused_origin_receives_no_credentialed_grant_over_a_real_socket`) — confirmed green in the
  coordinator's shared nextest per P4's headline table (relayed, not independently re-run this pass).
- `validate_auth_startup` (`🏗️bootstrap/🦀️.rs:2620-2645`) now **does** admit a non-loopback production
  bind under three simultaneous statements (`OS_HUB_MODE=production`, `CrossOriginPolicyV1::Allowlist`,
  `ForwardedTlsTrustV1::TerminatingProxy`) — confirmed by reading the function this pass. **Note:**
  `README.md:264-267`'s "Cross-origin access" section still says *"validate_auth_startup refuses a
  non-loopback bind in both modes today"*, which is now **stale documentation** contradicting the same
  file's own "Read this first" section (`:12-24`) and the code. Minor, but worth a one-line fix so an
  operator doesn't read two contradictory claims in one README.
- `README.md:531-542` ("Known gaps") still states *"Production mode has never been run"* — also now
  **stale**: P4 §8d and M6b §M6b.4 both ran real `OS_HUB_MODE=production` processes and signed in
  against them (loopback only). The **narrower, still-true** claim is: **no network-bound production
  hub, behind a real reverse proxy, has ever been started** — P4 §9.3 says this explicitly and it
  still holds; only the allowlist/proxy code paths are TEST-ONLY (6 unit laws), never runtime-observed.

**Rank for auth overall**: this capability moved from G2's "P0, structurally absent" to "real and
mostly observed" in two days. The one true remaining gap is **no external identity provider** (by
design, documented) and **no network-bound deployment has ever been booted** (real gap, not by
design — see §12 slice list).

---

## (e) Artifact authority

**Trusted catalog publish — REAL, OBSERVED-AT-RUNTIME end to end including a real document creation,
but scoped to exactly one artifact kind, and the underlying "which kinds CAN be created" gap (TC3) is
real, designed, and unimplemented.**

- **Bootstrap cost, stated exactly.** Publishing drives **two cold `wasm32-wasip2 --profile wasm-
  release` component builds** (stdio + gis) plus two `describe` runs and a jco codegen, inside the
  fleet's wasm build mutex; `produceFreshComponentV1` explicitly refuses to reuse an on-disk component
  because *"that coldness IS the provenance guarantee the trusted catalog publishes"*
  (`tc1-trusted-catalog-to-ready.md`, cited via extraction, ~line 300). Measured full-publish time:
  **44 minutes end to end** (materialise + native cold-map proof + candidate hub +
  `publishTrustedBootstrapCurrent`) in GM1's run — *"That gate had never been reached"* before GM1
  (`gm1-gis-cold-load-law-and-publish.md` §4e). A 3-package catalog (adding `vcs`) would need three
  such cold builds, "~45-60 min each" (TC2).
- **Reusability, stated exactly.** Once written, the catalog **is** reusable across a hub *restart* on
  the same data root without repeating the 44-minute bootstrap — GM1's own handoff table says
  verbatim: *"restart command (catalog already published — do NOT re-run the 44-min bootstrap)"*. It
  is **not** reusable across a `rm -rf`/emptied data root (TC1 hit exactly this: a cleaned staging dir
  meant "no generation was ever published there"), and it is **binary-pinned** — *"a hub built before
  this change cannot serve this catalog"* (TC1, GM1) — so a rebuilt `os-hub` after any schema-affecting
  change invalidates every previously published catalog on disk. Sharing a catalog across two hub data
  roots requires an explicit file copy plus `chmod 700`, never a shared root (TC1/GM1, identical
  wording).
- **A real document, not just the catalog, has been created and opened on a live hub over HTTP** —
  GM1 §4e, `🐍️gm1-live-open-plan.ts` against port 7611: sign-in `200` → create-space `202` →
  creation-catalog `200` → artifact-creation ready → **open-plan `200`** with a full surface/grant/
  checkpoint payload, for a real `s.gis.gismap` document. This is currently possible for exactly the
  one kind the published catalog carries (§ below) — GM1 §7 states plainly *"the catalog is still
  stdio+gis with one open target."*
- **The TC3 gap, exact evidence.** The const table is
  `NATIVE_OPENABLE_PROVIDER_SET_V1_ID = "stdio+gis+vcs/native-codecs/v1"`
  (`🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:9`), whose `linked()` lists exactly
  three entries: stdio (`semio:stdio`), gis (`semio:gis`), vcs (`semio:vcs`) (`:28`).
  `artifact_creation_selection` (`🔏️trusted-catalog/🦀️.rs:351-368`) requires `codec.genesis.is_some()`
  — genesis is a Rust function pointer, so a plugin needs **a statically linked native genesis factory
  compiled into the `os-hub` binary itself**, not a manifest declaration, to be creatable on a hub. TC2
  states the governing sentence directly: *"a plugin needs a statically linked native genesis factory
  in the hub binary, and only `stdio`, `gis` and `vcs` have one"* (`tc2-n-plugin-trusted-catalog-and-
  kd1.md` §4.1). Of the ~30-32 plugins with declared `ArtifactKindSpec` kinds in this repo (TC2 §2.1's
  census: note, draw, writer, raster, cad, flow, trinity, wfc, procedural, fem, block, puzzle, layout,
  forms, process, remodel, lowpoly, energy, architect, dag, reasoning, playbook, imperative, sequence,
  mathematical, shooting, demonstrator, vcs, stdio, gis), **exactly three have a native genesis
  factory, and one of those three (`vcs`) is currently non-selectable anyway** due to an unrelated
  one-string kind mismatch (`codec.identity.artifact_kind = "s.vcs.vcs"` vs. manifest
  `ArtifactKindSpec::id = "vcs.vcs"`, TC2 §4.2). **So today, functionally, only two kinds — stdio and
  gis — can have a brand-new document CREATED on a hub; every other kind can at best be opened once it
  exists via some other path.**
- **"TC3 never launched" — parked design, not an attempted-and-failed slice.** TC2 §9 contains a full
  4-step, migration-free "catalog-carried genesis" design (`tc2-n-plugin-trusted-catalog-and-kd1.md`
  §9, ~lines 412-483) that would let the *catalog itself* declare which kinds are creatable instead of
  a hardcoded Rust const table — but TC2's own handoff table states flatly: *"§9 — the design the
  coordinator asked for … Not implemented."* No TC3 report exists anywhere in this ticket folder
  (`ls` confirms — no `📓️tc3-*.md`). This is the correct framing for the task brief's quoted finding:
  it is real, it is understood, a fix is designed, and nobody has built it.
- **TC2b regression — fixed in source, not confirmed by a coordinator rerun.** TC2's document-index
  change went from `321 — 315/6` to `321 — 297/24` (20 new reds, all `Conflict("document index
  descriptor binding differs")`), root-caused to a **third, previously-missed** kind==dialect equality
  site (`validate_checkpoint_index_v1` in every checkpoint publication across all four backends) plus
  two gis-shaped fixture producers plus a client-side fold that silently dropped every non-gis document
  row from space listings. Fixed by moving the canonical grammar into
  `DocumentIndexEntryV1::validate` (single declaration site) across 9 files. `cargo check -p semio-hub
  --all-targets`: 0 errors, 316 warnings. **No coordinator rerun number for the post-fix state appears
  anywhere in this ticket** — TC2b's own words: *"needs hub rerun: YES… The Rust laws are compile-
  verified only."*
- **Execution targets / checkpoints**: real routes exist (`execution-target/{manifest,component,
  descriptor,browser-actor}`, `checkpoint-publications`) per G2 §8, unchanged this pass; G2's finding
  that `POST .../checkpoint-publications` has no known HTTP caller anywhere in the os product is
  **not contradicted or resolved by anything found this pass** — still OPEN.

---

## (f) Inference relay/runtime

**REAL and substantially fixed since G2, but still scoped to one artifact kind end to end (unchanged
architectural finding) and its last two reds are fixed-in-source, unverified.**
- `execution_not_wired_error` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:177`)
  is still the live behavior for every non-GIS declared inference (confirmed by re-reading the file
  this pass — 4 call sites at `:310`/`:364` and others construct it via `InferenceLookup::Execute`).
  No provider abstraction exists (`grep -rliE "anthropic|openai|claude|llm|model_provider"
  🌎️hub/💡️inference` → 0 hits, unchanged from G2).
- **What changed**: HT3a through HT12 (six slices, ~10 hours of work) drove the inference test family
  from 8 reds (of the suite's 58) down to a root-caused-and-fixed state, closing real product defects
  along the way — not test-fixture massaging. Named product fixes, each verified by
  `cargo check -p semio-hub --all-targets` and TEST-ONLY unit reproduction, **pending a coordinator
  hub rerun for final confirmation**: a lost-wakeup global map lock held across a document-actor
  mailbox round-trip (HT7 §1, `finish_document_recovery`), a `Published`-state join comparing advanced
  Stores against a genesis pack instead of the frontier the approval produced (HT10 §1, HT11 §1), a
  fold that re-minted the stamped mutation id instead of keeping it (HT5 §1, `ArtifactStore::
  fold_batch_item`), four previously-unguarded `Drop` witnesses in the retirement-cursor family
  (HT6 §1), and — the final one, HT12 §10 — a phase-ordering deadlock in `SemioStoreOwnedDisposer`
  where adopting a durable-group member aliased the same `Arc` into both the displaced-retirement
  queue and the tail-undo cache, so the close cursor could never `Arc::try_unwrap` it (drained
  `DisplacedOwners` nine phases before `TailSnapshot`). **This fix is landed
  (`🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:674-685`) and `cargo check -p semio-hub --all-targets`
  is 0 errors, but the coordinator's own status log's last entry (03:35 session, `status.md:537`) still
  says HT12's fix "needs the coordinator hub rerun (best verified 319/2, last 318/3)"** — i.e. as of
  the last recorded state in this ticket, the hub suite's true pass/fail count with every landed fix
  compiled in has **never actually been run to completion and read**.
- Full trajectory, for context on how real this work was (not cosmetic): 58 red (09:17) → 18 (15:20,
  HT3a/HT3b split) → 11+1 timeout (17:08) → 9 (17:35) → 7 (18:41) → 5/4 (19:22/19:32) → 4+1 timeout
  (20:26) → 3 (22:04, best-known-good before HS1's stack-overflow noise) → 2 (00:28/00:47, HT10/HT11)
  → 3 again at 02:45 (HT12, one of the three being JC1's unrelated browser-actor law) — a real,
  monotonically-attacked bug hunt, not a suite that was loosened to pass.

---

## (g) Admin module

**REAL, gated, and broader than G2's audit described — a full React SPA, not just an API.**
- Rust side: every `/admin/api/*` handler (overview, spaces, spaces/{id}, users, connections,
  documents, events, operations/{id}, audit, operations/{id}/cancel, intents) is gated behind
  `authenticate_admin_principal` (`🏗️bootstrap/🦀️.rs:2670`), confirmed called at 11+ handler sites
  including `:8494/:8510/:8525/:8938/:9054/:9120/:9161/:9217/:9236/:9253/:9293/:9317` (grepped this
  pass). `OS_HUB_ADMIN_SUBJECTS` (comma-separated `provider:subject`, max 64, required in production)
  is the admission list.
- Frontend: a real Vite-built React SPA at `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/` with
  seven pages (`🏠️OverviewPage`, `🏛️SpacesPage`, `📃️DocumentsPage`, `📰️EventsPage`,
  `🔗️ConnectionsPage`, `🙋️UsersPage`, plus `🔑️AdminSession` and i18n), served from `OS_HUB_ADMIN_DIR`
  (default: the SPA's own built `dist` next to the crate). `/admin`, `/admin/`, `/admin/{*path}` routes
  exist and a built `dist/` is present on disk (not re-verified as *current* this pass).
- `GET /admin/api/observability` (OB1r, new since G2) is gated the same way (`ob1r §2b`, confirmed by
  the same `authenticate_admin_principal` call at `:9120`) and was OBSERVED-AT-RUNTIME: no-cap → 401,
  forged bearer → 401, non-admin session → 401, admin → 200 with 13 declared events and
  `droppedEvents: 0`, no identity leakage in the body (extraction above).
- G2's 12-test-attribute count for this area is a floor, not the whole picture — no full recount was
  done this pass (out of scope for the time available); flagged as a small open item (§12 has no slice
  for this — low priority).

---

## (h) Observability / tracing

**REAL now, and OBSERVED-AT-RUNTIME — the single largest reversal of a G2 "ABSENT" finding besides
auth.**
- New crate half: `semio-framework-trace`'s server-side `📝️record/🦀️.rs`, `cargo test
  -p semio-framework-trace`: **49 passed, 0 failed** in 0.04s. Primitives: `TraceLevel`
  (Off<Error<Warn<Info<Debug), `TraceOutcome` (Started/Ok/Refused/Failed/Cancelled), `TraceRecord`
  with a hand-written `to_json_line()` (no serde — the repo's no-external-runtime-deps rule), `Tracer`
  (`Arc`-cloned per handler, no global mutable state, a bounded 64-slot counter table), injectable
  `TraceSink` (Null/Stream/Capturing).
- Wired into `🏗️bootstrap/🦀️.rs`: `observability_view`, `admin_observability` handler behind
  `authenticate_admin_principal`, `readiness_trace_detail`, `termination_signal`. 22 pre-existing span
  call sites (both WS handlers, the directory command path, auth mint/read/revoke, credential change,
  rate limits) were left as they were, not rewritten.
- OBSERVED-AT-RUNTIME: `🐍️ob1r-observability-probe.ts` against a real booted hub on :8868 — auth
  gating confirmed as above, and **live counters observed**: `server.artifact.maintenance(ok=1)`,
  `server.auth.session.mint(ok=2)`, `server.auth.session.read(ok=1)`, `server.boot(ok=1)`,
  `server.readiness(ok=0,refused=1)`.
- **The saga drain's own span** (`server.saga.drain`, §b above) is wired through this same tracer and
  covered by a shutdown-drain law that PASSED per the coordinator's 09:17 capture.
- **Gap, stated by OB1r itself**: the `checkpoint-publications` decision is "read evidence, not runtime
  evidence" — no process probe was ever run against that specific route; `server.auth.agent.*` spans
  (added by M6, a sibling slice) are emitted but not yet listed in the vocabulary fixture
  (`SERVER_SPAN_EVENTS`) — legal, but a real gap if agent-delegation activity needs to be alertable.
  Two laws hang under in-process `cargo test` but pass under `nextest` — an unexplained, unresolved
  runtime artifact OB1r flagged and handed off.
- **README staleness, again**: `README.md:543` ("Known gaps") still says *"No metrics, no request
  tracing. `/readyz` and stdout are the whole observability surface"* — this is now **false**; a
  gated `/admin/api/observability` endpoint with real counters exists. Same class of drift as the
  auth section above (§d) — the README has not been touched since before OB1r/P4/M6b landed.

---

## (i) Shutdown/restart semantics; worker pool stack

**SIGTERM drain — REAL, OBSERVED-AT-RUNTIME on a real production-mode process, not just a unit test.**
- Before P4: `grep -n "signal::ctrl_c|signal::unix|tokio::signal" 🌎️hub` → 0 hits (P4 §1) — the hub had
  **no** shutdown future at all.
- Landed: `TerminationSignalV1`/`first_termination_signal`/`termination_signal()`, SIGTERM biased ahead
  of SIGINT, `#[cfg(not(unix))]` fallback to `ctrl_c` alone. `📜️p4-production-runtime.sh` (OBSERVED-AT-
  RUNTIME): "os-hub ran as a plain process in production mode … for the first time in this repository's
  history" — nine numbered steps including **SIGTERM drains and exits clean**
  (`{"event":"server.readiness","outcome":"cancelled","detail":"SIGTERM-received-draining-in-flight-
  work"}`, exit status 0), and restart persistence (same `user_id`). The SIGTERM event line is emitted
  through OB1r's tracer, not a bare `eprintln!` — the two pieces of work compose.
- **Dev-mode pipe-holder death**: not independently re-verified this pass; `README.md:14-24`'s topology
  table and the `LocalBootstrapTransport`/fd-3 design (unchanged from G2/H1's description) is the only
  evidence — a bare `os-hub` with no fd 3 exits, by construction, in development mode. No probe in this
  ticket specifically kills the launcher process and observes the hub's reaction (an OPEN item, low
  priority — the dev topology is a convenience, not outcome 2's production surface).
- **Docker**: the image is authored and **unbuilt** — confirmed this pass that a Docker Desktop client
  exists on this host but its daemon is not running, which is new information beyond what P4's report
  says ("Docker is not installed"); either way, nobody has built or run this image, so its `tini`
  entrypoint's SIGTERM forwarding, the `HEALTHCHECK`, and the multi-stage build have never executed.
- **CORS/network-bind posture**: covered under (d); loopback path runtime-proven, network-bind path
  never booted by anyone.

**Worker pool / stack — a real crash, root-caused to the byte, and fixed by measurement, then
OBSERVED-AT-RUNTIME with a 300-second live socket hold.**
- Bug: not a leak or infinite recursion — "six frames that together reserve 8.5 MiB on a 2 MiB stack"
  (HS1), spawned with no explicit `stack_size` at `🧰️framework/🔨️modules/⏳️async/🦀️.rs:1868`, so pool
  workers got Rust's 2 MiB default while the main thread got macOS's 8 MiB. Masked in every `cargo
  test` run because the repo floors `RUST_MIN_STACK` at 32–256 MiB in test scripts and 64 MiB
  repo-wide via `.cargo/config.toml:65` — *"no cargo test can ever observe the production budget."*
- Fix: boxed the `ArtifactEngine` instead of carrying it by value through `catch_unwind` (removes
  ~529 KB copies per turn), split a 10-arm match with a combined 10.7 MB frame into one boxed coroutine
  per arm, and gave `WorkerPool::new` an explicit `stack_size(WORKER_STACK_BYTES = 16 MiB)`.
- Verification, explicitly runtime not just compiled (HS1 §5, its own words: *"every row was run;
  nothing here is reasoned about"*): a repro script crashed a real hub deterministically in ~1s before
  the fix; after the fix, zero `"overflowed its stack"` lines in the coordinator's nextest capture
  (was 3); a **document socket held live for 300 seconds with 20 client Presence frames** on the exact
  data root that had originally crashed, zero overflows in the hold log; a new law
  (`native_pool_workers_own_the_stated_worker_stack`) consumes 6 MiB in-process via `black_box` and
  passes.
- Honest gap HS1 states itself: the release profile is entirely unmeasured — "every figure in this
  report is debug" — no release `os-hub` binary exists anywhere in this ticket to check whether release
  optimization changes the frame sizes; the last ~7 MiB of the 16 MiB budget is empirical, not accounted
  frame-by-frame; `.cargo/config.toml`'s 64 MiB and `WORKER_STACK_BYTES`'s 16 MiB are two independent
  literals with no enforced relationship.

---

## (j) Hub test suite — what the last reds are, and which areas have no law at all

**The 58→3 trajectory is real, product-defect-driven work (see (f) for the blow-by-blow), not fixture
massaging — but the suite's true current state has never been measured after the last fix landed.**

- Last recorded coordinator rerun in this ticket: **02:45, 321 run / 318 passed / 3 failed**
  (`status.md:537`-adjacent entry) — two of those three (L1, L3) are the `SemioStoreOwnedDisposer`
  phase-ordering deadlock HT12 diagnosed and fixed in source at 02:52-ish (§f above); the third was
  JC1's unrelated browser-actor-identity law, six of seven of which JC1 had already repaired by then.
  **No rerun after HT12's landed fix appears anywhere in this ticket's `status.md` or any `📓️` file.**
  The planned successor, **HT13**, was named in the coordinator's session-7 relaunch plan
  (`status.md:537`, "HT13 after the coordinator's hub rerun") but **no `📓️ht13-*.md` file exists** —
  confirmed by `ls` this pass. **This is the single most important open item for calling outcome 2
  "working": the last known state is 3 reds, with a fix for 2 of them landed and compiling, and nobody
  has pressed the button to find out if it is actually 321/0, 320/1, or something newer entirely.**
- **Rule 26 explains the gap, not excuses it**: every HT-series worker in this ticket was explicitly
  forbidden from running `cargo test`/`nextest`/`build -p semio-hub` (to avoid stacking on the shared
  build lock that starved earlier sessions — see H1b §15-§25's long fight with `flock` contention).
  Only the coordinator may run the full suite. That means the suite's true state is bottlenecked on
  one actor's availability, and the ticket ran out of that actor's turns before the last rerun happened.
- **Product areas with reasonable law coverage** (per G2 §11's static count, not re-walked digit-by-
  digit this pass but not contradicted by anything found): artifact-authority (12 files/61 laws),
  directory (6 files/54 laws), inference (10 files/27 laws, now root-cause-fixed per (f)), the
  bin-unit integration layer (97 laws via `#[path]`).
- **Product areas with thin or no law coverage**, found this pass:
  - `🔐️auth` proper shows only **1** file/1 test-attr in G2's static count, but that count predates
    AU1/AU3/M6/M6b entirely — the real current auth surface (password, rate-limit, agent delegation)
    clearly has many more laws now (M6b alone names 3 new hub-side integration laws plus M6's ten
    `auth::agent` laws plus AU1's seven credential-sign-in laws plus P4's CORS/shutdown/posture laws)
    — G2's static count is simply stale and a fresh `grep -rc '#\[test\]\|#\[tokio::test\]'
    🌎️hub/🔐️auth` was not re-run this pass (recommend as a cheap follow-up, not worth its own slice).
  - **`🚀️local-relay`**: 0 files/0 test-attrs per G2, unchanged — a 68-line path-admission allowlist
    with no test of its own (some coverage exists under
    `🚀️local-relay/🧭️routing/🧪️tests/🔬️admission/🟦️.ts`, a TS file G2's Rust-only count missed, so the
    "0 law" framing is itself imprecise — there IS TS coverage, just not Rust `#[test]`).
  - **Postgres/neo4j lanes**: zero runtime law of any kind — every law that exercises `db::Database`
    or `HubDirectory` runs against `fs`/`sqlite` only; the two feature-gated backends have never been
    exercised by a single integration test that actually opens a connection (only `cargo check`, see
    (a)).
  - **`semio-framework-server` (the generic product hub is supposed to eventually instance)**: W3a/
    W3b/W3d did substantial real work here this ticket (78→90 passing tests per the extracted table),
    but `semio-hub` still does not depend on it (`grep -rln "semio-framework-server"
    🌎️hub/📦️packages/🦀️rust/Cargo.toml` → 0, unchanged from G2/audit-hub-backend) — so none of that
    coverage protects the code path a user actually boots.
  - **Two-human live collaboration on a hub document**: C3/C4's ten-step scenario is the only thing in
    this entire ticket that approaches a true multi-user law, and it is explicitly **not** wired as a
    permanent nx target/gate ("an nx target today would pin a red path as a gate," C3 §4) — so there is
    **no standing law anywhere in this repo that two real humans can collaboratively edit one hub
    document**, only a one-off manual script that observed it half-working.

---

## Ranked slices to make outcome 2 "working for a user" (not just for a test)

Ordered by (blast radius if unfixed) × (how close it already is). File lists are the concrete starting
points, not exhaustive migrations.

1. **Run the coordinator hub suite to a final number and act on it.** No new code — just
   `cargo nextest run -p semio-hub --no-fail-fast --profile long` (rule 25 private
   `CARGO_TARGET_DIR`) after confirming `🏪️store/🦀️.rs` (`MM` in `git status` today) has settled, then
   either close HT13 at green or file the real remainder. Everything else in this list assumes a green
   or near-green baseline; right now nobody actually knows if HT12's fix works.
   Files: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs`
   (HT12's fix), `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`.

2. **Land TC2's §9 catalog-carried-genesis design** (the TC3 gap). This is the single highest-leverage
   fix for "outcome 2 works for a user": today a user can create exactly two kinds of document
   (stdio, gis) on any hub, ever, no matter what plugins are installed, because genesis factories are
   a hardcoded Rust const table. Files:
   `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs`,
   `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` (`artifact_creation_selection`,
   `:351-368`), plus TC2 §9's own 4-step landing order.

3. **Actually connect to a live Postgres and Neo4j once, on this host.** Docker Desktop is installed
   (daemon not running) or `brew install postgresql@16 neo4j` — either is same-day. Boot
   `OS_HUB_STORAGE_BACKEND=postgres`/`neo4j` against it and run the directory/document suites against
   real connections at least once. Files: `🌎️hub/📇️directory/🐘️postgres/🦀️.rs`,
   `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`, `🧰️framework/…/🛢️db` postgres/neo4j drivers.

4. **Root-cause and fix the C3 §3.4 presence-roster asymmetry** before wiring the ten-step
   collaboration scenario as a permanent gate. This is the one concrete defect standing between "two
   humans can concurrently hold sockets on one document" (proven) and "two humans see each other and
   can edit together" (not proven — also blocked on item 5). Files:
   `🌎️hub/🏗️bootstrap/🦀️.rs` (`refresh_document_presence`, `install_presence_slot`), C3's own harness
   `🐍️c3-collab-scenario.mjs` for reproduction.

5. **Root-cause C3 §3.2's browser-actor activation rejection** (jco `reactor.poll` async task-return
   bug) end to end in a real browser — JC1 fixed the jco version issue but C4/C3 never confirmed live
   edit-both-ways in a browser afterward. Without this, "collaboration" on a hub document is proven at
   the socket layer only, never at the edit layer, for two real humans. Files: JC1's jco bump (jco
   1.27.0 → 1.34.0), `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/`.

6. **Boot one real network-bound production hub behind a real reverse proxy once.** All the code
   (`ForwardedTlsTrustV1::TerminatingProxy`, `CrossOriginPolicyV1::Allowlist`,
   `transport_security_middleware`) is written, `cargo check`-green, and unit-tested, but per P4 §9.3
   "no one has yet started a real OS_HUB_MODE=production process" on a network interface. A local
   nginx/caddy TLS-terminating proxy in front of a loopback hub would close this for good. Files: none
   to change — this is a runtime exercise, `README.md:328-427` is the recipe.

7. **Register at least one real saga decider**, or delete `HubSagas`'s uninhabited-enum wiring and
   say so plainly. Right now the supervisor, cadence, and shutdown-drain are all production-grade
   machinery guarding an empty set — that is either dead weight or a half-finished feature; both are
   worth resolving explicitly rather than leaving as a silently-empty CQRS layer. Files:
   `🌎️hub/🗄️stores/🦀️.rs:120` (`HubSagas`), `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs`
   (`drain_sagas`, `:784`).

8. **Fix the two stale README.md sections** (network-bind refusal claim at `:264-267` contradicting
   `:12-24`; "no metrics, no request tracing" at `:543` contradicting OB1r's shipped
   `/admin/api/observability`; "production mode has never been run" at `:538-541` contradicting P4/
   M6b's own runtime proofs). Cheap, but a production operator reading this file today will make
   wrong decisions from at least three sentences in it. File: `🌎️hub/README.md`.

9. **Resolve the `checkpoint-publications` route's dead-or-undiscovered-caller question** (G2 §10
   mismatch #1, restated and unchanged this pass) before Wave-3's module split moves it into a
   `ServerModule` and inherits the ambiguity. File: `🌎️hub/🏗️bootstrap/🦀️.rs` (the route handler) plus a
   grep of the wgpu/native client path G2 did not cover.

10. **Decide Wave 3** (hub as `semio-framework-server` instance #1) **or retire the generic `server`
    product.** W3a/W3b/W3d have now put 90 passing tests behind a product `semio-hub` still doesn't
    depend on (`grep -rln "semio-framework-server" 🌎️hub/📦️packages/🦀️rust/Cargo.toml` → 0, unchanged
    since G2/the original audit). Every hour spent on `server`'s generic contract/storage/policy/
    authority/gateway layer protects zero lines a user's hub actually runs until this is decided.
