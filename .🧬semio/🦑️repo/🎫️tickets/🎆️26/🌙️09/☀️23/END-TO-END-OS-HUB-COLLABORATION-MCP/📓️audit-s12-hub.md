# S12 Audit — Outcome 2: working hub server backend (db, presence, auth, observability, deploy)

Read-only, Sonnet 5, 2026-09-25 ~23:2x (session 12, before H9/W2's session-12 work has produced a `## Session
12` section of their own reports — H9's `wp-h9.md` still ends at its session-11 `## Log` entry 12:16–12:35;
this audit's "current state" is session 11's closing state carried forward, cross-checked against the
current source tree). **No builds/servers/edits performed** (per the auditor mandate). Method: read
`session-12-preamble.md`, the closing session-11 hub audit (`📓️audit-s11-hub.md`, full P0/P1/P2 list),
`wp-h9.md` (hub backend owner, full), `wp-w2.md` (catalog/7800 owner, full), `wp-c10.md` (collaboration,
live hub findings) and `wp-s15.md` (frontend, routed hub findings, §4/§10–§13) in full, then spot-verified
the highest-leverage claims directly against the current tree with `/usr/bin/grep`/`Read` (hub bootstrap
route file, DB I/O credit ledger, `HubSagas`, Dockerfile, README backup/restore and topology sections). All
checks matched their cited reports; no drift found beyond what is noted below.

## Summary

| # | Capability | State | Owner (session 12) |
|---|---|---|---|
| 1 | Storage: sqlite | DONE, live | — |
| 1 | Storage: postgres / neo4j | DONE (mechanics + WAL fence), **PARTIAL** for document creation live-proof | H9 (Item a, running) |
| 2 | Document authority + socket | DONE, live | — |
| 3 | Presence (join/leave/expiry/lease) | DONE, live (2 independent harnesses) | — |
| 4 | Auth (credentials/sessions/delegation/revoke/admin) | DONE, live | — |
| 5 | Authorization policy (declared, single surface) | **DONE this session** (was PARTIAL/TODO at S11 close) | H9 (Item 3) |
| 6 | Trusted catalog + plugin module bundles | **DONE, materially rebuilt this session** (was PARTIAL, 2/~30 kinds) | W2 + S15 |
| 7 | Inference services | PARTIAL, unchanged (GIS-only) | unowned |
| 8 | Observability (trace/admin/health) | DONE, live, re-proven post-ABI-change | H9 (Item 5) |
| 9 | Graceful shutdown | DONE (code, all backends); live hub-level restart proof PARTIAL for pg/neo4j | H9 (Item b) |
| 10 | Resource bounds: memory | PARTIAL (lazy retention DONE; compiled-guest cache growth UNBOUNDED, named follow-up) | W2 |
| 10 | Resource bounds: DB I/O credit | **PARTIAL** — root causes fixed + laws green, but the specific growth/uptime failure C10 and S15 hit has **no confirmed live re-proof after the fix** | H9 (Item g) |
| 10 | Resource bounds: request sizes | DONE | — |
| 11 | Input validation | DONE (bounds-checked broadly); fuzz/malformed-body coverage UNVERIFIED | — |
| 12 | Deployment: Dockerfile / config | PARTIAL, unchanged (`docker build --check` only, no cold build ever run) | unowned |
| 12 | Deployment: zero-touch local hub | **DONE this session** (was real-but-clunky; C10 fixed 4 real defects + added a session broker) | C10 |
| 13 | Backup / restore | DONE (documented, offline tar procedure); no live drill evidence found | unowned |
| 14 | Multi-instance / single-instance design | DONE (explicit single-binary design, documented) | — |

Full detail and citations below. **P0/P1/P2 ranked list is at the end.**

---

## 1. Definition-of-done checklist

### 1.1 Storage backends — sqlite / postgres / neo4j

**sqlite: DONE, live.** Unchanged from S11 close (kill+restart persistence proven repeatedly). H9 session-12
re-confirms at the code level: `Database::shutdown` now runs on **every** exit path (`main`'s `served` block
in `🌎️hub/🏗️bootstrap/🦀️.rs`), sqlite hub-level SIGTERM→exit measured **298 ms**, close code 1012,
reopen on first attempt (wp-h9.md Item b).

**postgres / neo4j: DONE for connection/WAL-fence mechanics** (carried from S11, unchanged this session —
`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs`,
`…/🗄️storage/🌐️neo4j/🦀️.rs`). WAL-writer fence `release-admits-contender` + `process-exit-releases`
**3/3 backends** (wp-h9.md Item b), directory live lanes postgres **12/12**, neo4j **7/7** (wp-h9.md #6).

**Document-creation on postgres/neo4j: PARTIAL, the single biggest open item carried from S11's P0-1.**
H9's Item a: the creation law (`space_artifact_creation_routes_are_author_owned_idempotent_and_genesis_backed`)
now runs genesis on a **real** GIS release component built from the current tree and **PASSES in-process on
sqlite** (208 s). But: *"DB4 document lane pg/neo4j: running in the growth e2e (Item g)"* — i.e. the
non-sqlite document-creation leg of S11's P0-1 has **no confirmed pass reported this session**, only "running."
**UNVERIFIED** whether P0-1 is actually closed for postgres/neo4j; treat as open until a report states a
measured pass.

### 1.2 Document authority + socket

**DONE, live.** H9's two-client e2e on its own hub :8010 (W2's binary, catalog-A copy) **2/2 PASS** in 347 s;
document socket open (`upgrade`) / close (`closed`, cancelled) share one `requestId`; `server.shutdown:ok`
(wp-h9.md #5). C10's independent live proof on hub 7800 reaches the same conclusion for a growing document:
0 faults through `addFeature`→undo→redo, panels reflect remote edits after the G-P1-4 fix (wp-c10.md
"G-P1-4 — actor-rendered panels"). Impersonation-vector closure (actor bound server-side, `?actor=` route
deleted) is unchanged from S11, re-verified current.

### 1.3 Presence (join/leave/expiry/lease)

**DONE, live, from two independent harnesses.** H9: presence join 2 / leave 2 / expiry 1 inside the two-client
e2e (wp-h9.md #5). C10 (`c10pres3`, hub 7800): roster symmetry, lease expiry with a live socket (22 s blocked
main thread, roster stays 2), leave → roster drops to 1 in **507 ms**, re-join + join-replay both PASS
(wp-c10.md "Presence lifecycle live"). Presence stays ephemeral by design (broadcast fan-out, never written to
`db::Database`), unchanged from S11.

### 1.4 Auth (credentials, sessions, agent delegations, revocation, admin)

**DONE, live, unchanged in scope this session but hardened underneath.** Credential sign-in, rate limiting,
agent delegations, revocation → cascading 401 were all WORKS-LIVE at S11 close and no report this session
contradicts that. H9 fixed one real silent-failure caller this session: **agent-delegation revoke discarded a
failing store write** (`let _ =`) — now answers `503 directory-unavailable`
(`🌎️hub/🏗️bootstrap/🦀️.rs` `delete_agent_delegation`; wp-h9.md Item 2). Store-write fault laws pass:
`a_projection_write_reports_a_failing_sink`, `a_session_write_reports_a_failing_sink`,
`credential_sign_in_reports_a_failing_instance_session_store`. Admin surface (`admin-relay` pipe-issued
session) unaffected this session; W2's hold script re-mints it on demand (wp-w2.md Hub Handoff).

### 1.5 Authorization policy — **the largest single change this session**

**S11 close: PARTIAL** (O2-10 — five hand-written admission functions, no declared policy, H9 item 3 was
`TODO`). **Session-12 evidence: DONE.** One schema-first policy `HubAccessPolicyV1`
(`🌎️hub/🔐️auth/🛡️access-policy/🔣️.json`), one authority `hub_access_permits`
(`semio_hub::auth::access_policy`), roles `admin|owner|author|spectator|share|authenticated`, actions covering
space/member/invite/document/agent-delegation/artifact/blob. Routed through it: directory commands, document
read, space blobs (a spectator can no longer PUT a blob — a real access hole closed), the document-socket
`SecurityGate`, open-plan writability, Check In, artifact creation, agent delegation, GIS approval delivery.
The old, divergent second copy (`db::security::space_grants`) was **deleted** with its two laws. Rust law
`declared_access_policy_matches_the_language_neutral_truth_table` — **113 vectors** — plus
`access_policy_is_closed_by_default_and_deny_overrides_allow`; TS/Ajv twin **2/2** (wp-h9.md Item 3). This
retires S11's P1-2 outright.

### 1.6 Trusted catalog + plugin module bundles — the second-largest change this session

**S11 close: PARTIAL, severe** (P1-1/TC3 — exactly 2 of ~30 plugin kinds could ever be created on a hub,
gated by a hardcoded 3-entry `NATIVE_OPENABLE_PROVIDER_SET_V1_ID` const requiring an `os-hub` recompile per
kind). **Session-12 evidence: DONE**, rebuilt from first principles by W2 + S15 (huge, multi-thousand-line
change across `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` and schema):

- **One Rust open-target rule** (`app_opens_kind` + `descriptor_open_targets` +
  `validate_descriptor_open_target`, all one predicate): an editor/viewer opens a kind it declares itself, or a
  plugin-level kind its own dialect names. The old TS re-implementation (stricter than Rust, the actual root
  cause of the 2-kind ceiling) is **deleted**. New verb `os-hub trusted-catalog open-targets`.
- **Lazy retention**: the old hardcoded `TRUSTED_{COMPONENT,DESCRIPTOR,BROWSER_ACTOR}_CLOSURE_MAX_BYTES`
  in-memory totals are **deleted**; per-file bounds are now schema-declared execution-target bounds, assets
  are `Weak`-held `TrustedCatalogAsset`s re-verified on use, not resident in memory. This is what makes >6
  packages structurally possible at all (the old 512 MiB / 128 MiB closure totals could never have held 34
  packages, measured ≈791 MB / ≈1 GB).
- **Catalog B published** (generation `e8167ce8…`, 9 packages: stdio, gis, note, animate, block, writer, draw,
  puzzle, wfc), **12/12 open targets pass** creation → ready → open-plan (wp-w2.md Hub Handoff). Every one of
  the 34 packages except stdio now has ≥1 open target (wp-w2.md §3 log 05:22).
- **Plugin module bundles (option a)**, schema v3, content-addressed, hub-served, verified end to end: S15
  live-proved install-from-hub with progress+cancel, a durable Cache-Storage device store with GC/eviction/
  re-verify, and a generation-resolution rule (store → byte-identical local → hub) — all with laws (wp-s15.md
  §10–§13, `TrustedBundlePluginModuleV1`, exports 19→25).
- **Two real defects found and fixed on the way**: gis's closed actor exceeded the 64 MiB per-actor bound
  because it embedded cores as base64 (fixed: deflate-raw compression, actor size 67 MB→16 MB, wp-w2.md
  10:2x); a `kit.catalog`-declaring-but-not-owning kind crashed publish (fixed: `unowned` codec-row answer,
  wp-w2.md 10:39).

**Caveat, UNVERIFIED for the canonical hub**: 7800 itself was still on catalog B (9 packages) at last
report, not the full `--packages all` (34); the `--packages all` publish + restart was in progress at
session-11 close and **restarted from scratch at session-12 start** (all processes died ~19:30). Whether
`--packages all` has landed is unknown as of this audit — check `wp-w2.md` for a session-12 update before
relying on more than the 9-package catalog. This closes S11's P1-1 in design and code; the *full-catalog live
instance* is a session-12 open item, not a design gap.

### 1.7 Inference services

**PARTIAL, unchanged.** No report read this session (`wp-h9.md`, `wp-w2.md`, `wp-c10.md`, `wp-s15.md`) touches
`💡️inference` scope beyond the existing GIS-map approval/check-in path. Still scoped to GIS only;
`execution_not_wired_error` for every other declared inference is presumed unchanged (not re-verified this
pass — **UNVERIFIED**, carried forward from S11 rather than freshly confirmed).

### 1.8 Observability (structured events, traces, metrics, health/readiness)

**DONE, live, re-proven after this session's ABI change.** H9's two-client e2e: **27 trace lines**, Ajv-valid
and declared, document-socket open/close sharing one `requestId`, presence join/leave/expiry all recorded
(wp-h9.md #5). `/readyz` and `/healthz` gates unaffected; W2's `readyz-7800-b.json` shows every feature gate
(`artifactAuthority.ready`, `openPlan`, `openPlanExchange`, `rebootstrap`, `mcpWorkspace`,
`inferenceServices`, `publicSessionIssuance`) open on catalog B. No metrics/Prometheus endpoint exists — by
design, documented in `🌎️hub/README.md` ("point a dashboard at the admin route... no Prometheus exposition
format").

### 1.9 Graceful shutdown

**DONE at the code level, PARTIAL for the full live cross-backend proof.** H9 Item b: every `?` after
`connect_db` used to skip the close (directory/CAS/coordinator setup, extension dir, GIS binding, inference
ledger, session-key minting, `compose_hub_server`, a port-in-use bind — all previously left the Neo4j lease,
Postgres advisory session, sqlite sidecar released only by process death or TTL). Now: one `served` block,
`close_hub_database` runs unconditionally, Check-In jobs (which hold `Arc<Database>`) are drained first so
`try_unwrap` can succeed, `hub_shutdown_record` emits one `server.shutdown` line on every exit path. **Live,
sqlite**: SIGTERM→exit 298 ms, `database=closed`, reopen on first attempt. **Live, postgres/neo4j at the hub
level**: *"still to run. It needs an os-hub with the postgres + neo4j drivers from this tree and a
current-tree catalog"* — explicitly not yet measured (wp-h9.md Item b). This closes S11's P2-2 gap **in
code** (the Neo4j-lease-survives-shutdown defect cannot recur once every exit path calls
`close_hub_database`), but the live restart-within-1s proof on postgres/neo4j specifically remains
**UNVERIFIED**.

### 1.10 Resource bounds

**Memory**: lazy retention (§1.6) took catalog-B idle hub RSS from 578 MB (6-package, eager, old binary) to
**346 MB** (9-package, lazy, right after ready). But after 12 document creations exercising all 7 guest
packages, RSS rose to **1002 MB**: compiled wasm guests are cached per-package in an `OnceCell` with **no
eviction** (`Component and actor bytes are no longer resident. An idle-release of compiled guests is the
next bound (follow-up, not done)` — wp-w2.md §2.1). This is an explicitly named, unfixed unbounded-growth
risk for a long-running multi-plugin hub: RSS is bounded by "how many distinct plugins have ever been used
since boot," not by working set.

**DB I/O credit** (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs`, process-wide bounded
ledger: `DB_IO_TOTAL_PAGES=1024`, `DB_IO_BACKEND_CONTROLS=64`, `DB_IO_OPERATION_ITEMS=64`,
`db_io_operation_add` at line 254, `"DB I/O aggregate admission exhausted"` at line 260,
`"DB I/O process aggregate credit exhausted"` in the process-credit path). H9's Item g found and fixed **four
stacked bounds**: (1) index runs decoded per-entry instead of in-place, so a run ≥8 entries blew the
16-control/64-page per-operation credit; (2) the version-graph's fixed 64-edit ledger saturated with no
compaction; (3) `DocumentState::apply_entries` parked replaced values in the 128-per-process retirement slots
but nothing ever drove the maintenance step outside tests — after ~125 overwrites *across all documents* every
write in the process refused; (4) the window-rollover itself held a half-closed store across a droppable
future, panicking the Drop assertion. All four are fixed with new laws (db growth laws in `long`, vcs 13/13,
index 30/30, hub socket-growth law 5/5 completed runs). **Nextest long 355/355, quick 345/345.** *But*: this
is exactly the failure signature **C10 and S15 both hit live**, independently, *after* H9's own fix landed
per their timestamps being cited from earlier in the day (C10: `c10out6` at 03:30–05:35 and the session-12
opening note citing `c10gp14h` at 11:3x; S15 §13: hub 8040 after ~20 documents). H9's own report states the
full live multi-backend growth e2e was *"rerun on the fixed binary, running"* — i.e. **not confirmed complete**
in any report read this pass. **This is the single most important thing for session 12 to re-verify before
trusting the hub under real multi-document, multi-hour use**: run C10's or S15's exact repro (many documents
over hours, or ~20+ creations exercising several plugins) against the current-tree binary and catalog and
confirm 0 `DB I/O … exhausted` refusals.

**Request sizes**: DONE, broadly enforced. `🌎️hub/🏗️bootstrap/🦀️.rs` carries per-route
`*_MAX_BYTES` checks throughout (`DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES = 8 * 1024`,
`DOCUMENT_EXECUTION_TARGET_REQUEST_MAX_BYTES`, `AUTH_TEXT_MAX_BYTES`, `HUB_BLOB_MAX_BYTES`,
`DIRECTORY_COMMAND_REQUEST_MAX_BYTES`, `ADMIN_INTENT_REQUEST_MAX_BYTES`, etc.), plus
`axum::body::to_bytes(body, <MAX>)` at every body-reading callsite checked (lines 3343, 3543, 3631).
`axum::extract::DefaultBodyLimit` is imported and in scope. Local-bootstrap pipe additionally bounds itself to
64 exchanges per 15 s window with time-eviction (H9 Item e, below).

### 1.11 Input validation

**DONE for the specific vectors checked**, unchanged assessment from S11: impersonation (actor bound
server-side) and CORS (real allowlist, proven against a real preflight) both closed. Session-12 adds one more
closed vector: the local-bootstrap pipe used to treat every post-authentication refusal class (expired window,
device/profile/client-class mismatch, replay, full window) as **channel-fatal** — the hub simply exited
(measured by W2: dies on the 65th admin-relay re-issue). H9 Item e: only integrity failures (unparseable
frame, wrong run id/sequence, bad proof) are now channel-fatal; every other refusal is now a **signed `reject`
answer** and the pipe stays open, with the replay set converted from a non-evicting 64-slot set to a genuinely
time-windowed `ExchangeWindow<64>`. This closes a real "many legitimate admin-relay re-issues eventually kill
the hub" liveness bug. **Still UNVERIFIED** (carried from S11, not re-assessed this pass): general fuzz/
malformed-body coverage beyond the specific vectors above.

### 1.12 Deployment (Dockerfile, config, zero-touch local hub)

**Dockerfile: PARTIAL, unchanged from S11.** Read directly this pass (`🌎️hub/Dockerfile`): the file's own
header comment is explicit that *"The image itself has **not** been built: the builder stage compiles the
hub's full release dependency graph, which on the machine this was written on takes longer than the session
that wrote it."* Only `docker build --check` and `docker compose config` have ever passed. This is still true
as of this audit (no session-12 report claims a cold `docker build` was run) — **S11's P2-3 is unchanged and
still open.** The file is otherwise well-formed: binary-only runtime image, `tini` PID 1 for signal delivery,
non-loopback bind gated behind three explicit env vars, `/readyz` healthcheck that correctly speaks
`X-Forwarded-Proto: https` (a previously-fixed real defect, per the comment).

**Zero-touch local hub: DONE this session**, a real fix, not carried-forward. C10 (session 11, `wp-c10.md`
"Task 4") found and fixed 4 concrete zero-touch defects: **Z1** stale hub binary could boot without staging
(fixed: stage through `os-hub:build-dev` on every launch); **Z2** two launch paths publish different catalog
package sets into the same data root, whichever runs first wins silently; **Z3** `dev s` reusing a hub it
doesn't own has no credential, shell shows sign-in with no way to get one; **Z4** `ensureDevLocalHub` gives up
immediately on a bound-but-not-ready port, so the shell can run with no hub after a compound launch. Fix:
**one detached local-hub owner per data root** holding the local-bootstrap pipe, a **session broker**
(`startLocalSessionBroker`, loopback-only, bearer secret in a `0600` record) every serve signs in through, one
canonical development-catalog package list. **Live-proven**: two fresh browser contexts on two serves end
signed-in with **zero manual steps** (8.2 s / 10.6 s to ready+signed-in), broker record removed on stop
(wp-c10.md "Task 4 — zero-touch fixes Z2–Z4"). Cross-platform (native Windows/Linux, devcontainer): still
**UNVERIFIED** — no report in this ticket documents a non-Darwin build or boot attempt.

### 1.13 Backup / restore or export

**DONE (documented design), UNVERIFIED (no live drill).** `🌎️hub/README.md` "## Backup and restore"
(read directly this pass, lines 348–383): the backup unit is the whole `OS_HUB_DATA` tree with the hub
**stopped** (no online snapshot, no `os-hub backup`/`restore` verb — by design, "There is no cross-version
upgrade path... Treat a hub upgrade as: back up, stop, and be prepared to start from an empty data root").
Documented `systemctl stop` → `tar` → `systemctl start` → `/readyz` procedure; explicit warning that
postgres/neo4j backends are **outside** the tarball and must be backed up with native tooling at the same
stopped moment. This is a sound, honestly-scoped design (matches AGENTS.md's "no legacy support / no
migrations" stance) but **no report read this session or in the S11 audit chain shows it was ever exercised
live** — treat the procedure as designed-correct, not measured-correct.

### 1.14 Multi-instance or explicit single-instance design

**DONE, explicit.** `🌎️hub/README.md` line 4: *"It is a single native binary, `os-hub`, with no runtime
service dependencies beyond a data directory on disk."* No horizontal-scaling, load-balancing or multi-writer
story exists or is claimed anywhere in the hub docs or source read this pass — this is a stated design
choice, not a gap. (The DB I/O credit ledger, presence roster, open-plan ledger, etc. are all explicitly
process-local `OnceLock`/`static` state, consistent with single-instance-by-design.)

---

## 2. Code-level risks (read, not run)

1. **`db_io_operation_add` / process-wide compiled-guest cache — unbounded growth, not a panic but a
   resource-exhaustion risk.** `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:71-260`
   (the credit ledger itself is correctly *bounded*, refusing writes rather than crashing) but the trusted-
   catalog guest compile cache (`OnceCell` per package, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/`)
   has **no eviction**, confirmed by W2's own measurement (578 MB → 346 MB after lazy retention → 1002 MB
   after 12 creations touching 7 packages, wp-w2.md §2.1, "next bound (follow-up, not done)"). Named risk,
   not yet mitigated: RSS on a long-lived hub serving many distinct plugin kinds grows monotonically.

2. **DB I/O credit exhaustion under sustained/multi-document use — fixed at unit level, live re-proof
   incomplete.** See §1.10 above. `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:254-266`
   (`db_io_operation_add`, `"DB I/O aggregate admission exhausted"` / `"DB I/O process aggregate credit
   exhausted"`) — H9's four fixes target exactly the signature C10 (`wp-c10.md`, `c10out6`, `c10gp14h`/`h`)
   and S15 (`wp-s15.md` §13 finding (a)) hit live. No report confirms a fresh multi-hour/multi-document live
   run against the fixed binary. **Recommend session 12 re-run this before any live collaboration proof is
   trusted.**

3. **Compiled-guest cache and DB I/O ledger are both process-global `static`/`OnceLock` state**
   (`🧧🧧🛢️db/🗄️storage/🦀️.rs:129-260`, trusted-catalog `OnceCell`) — consistent with the documented
   single-instance design (§1.14), but means a single wedged or leaked guest/operation cannot be isolated by
   restarting *part* of the hub; the only recovery is a full process restart. Not a bug, a structural
   consequence worth naming for operators.

4. **Two `.expect()` call sites checked directly and found safe, no action needed**:
   `🌎️hub/🏗️bootstrap/🦀️.rs:1852,1882,1900` (`inner.records.remove(&digest).expect("record was
   present")` / `inner.records.get_mut(&digest).expect(...)`) — each `.expect()` is on a value looked up
   under the *same* held `std::sync::Mutex` guard, with no `.await` between the existence check and the
   `.expect()`, so no concurrent mutation window exists. Line 4133 (`current.expect("matching active
   checkpoint exists")`) is similarly guarded by an immediately-preceding `is_some_and` on the same local
   binding. Verified by direct read; not flagging these despite the `.expect()` pattern looking risky at a
   glance.

5. **In-process (non-nextest) `cargo test -p …-db --lib` still flakes under load — open, pre-existing, not
   newly introduced.** H9's own report (wp-h9.md, end of Item g): *"plain in-process `cargo test -p …-db
   --lib` still flakes under load — 4 of 6 runs this afternoon... every failure is `db I/O backend control
   capacity exhausted` (64 backends per process, retired by maintenance on the shared 2-worker test pool)."*
   H9 attributes this to the shared *test* pool specifically (nextest, one process per law, is unaffected),
   not to the live hub binary — but the underlying mechanism (64 `DB_IO_BACKEND_CONTROLS` slots, maintenance-
   driven retirement) is the same one a live hub under heavy concurrent backend churn would exercise. Not
   independently reproduced against a live hub this pass; flagged as an open question, not a confirmed
   production defect.

6. **Local-bootstrap pipe liveness bug — fixed this session, worth naming as what it was.** Before H9's Item
   e fix, *any* non-integrity refusal after the 64th exchange in a run (e.g. routine admin-relay re-issuance)
   terminated the whole hub process — a legitimate-traffic liveness bug, not an attacker-triggered one (W2
   measured it killing catalog-A hub 7800 at 11:41 from ordinary 5-minute admin-token re-issuance). Now fixed
   (`ExchangeWindow<64>` with time-based eviction, refusals answered not fatal). Cite for completeness since
   it was a real production-facing bug, now closed.

7. **Docker image has never been cold-built** (`🌎️hub/Dockerfile`, header comment, confirmed by direct
   read) — `docker build --check` catches syntax/lint issues but not a real compile failure, a missing file in
   the `COPY --from=builder` step, or a runtime-image dependency gap. This is a real, acknowledged, unclosed
   gap (S11's P2-3), not a code bug but a deploy-path risk: the first real `docker build` could still fail.

---

## 3. Ranked P0/P1/P2 (session 12)

Ownership note: **H9's `wp-h9.md` has no `## Session 12` section as of this read** (its report ends inside
the session-11 `## Log`), and **W2's `wp-w2.md` likewise has no `## Session 12` section**. Per the session-12
preamble, H9 = hub backend owner, W2 = sole all-plugin-wasm/catalog/7800 owner. Items below are assigned by
scope-continuity from session 11, not by an explicit session-12 claim — **treat every "owner" below as
"who should pick this up," not "who has already claimed it."**

### P0-1 — Re-verify the DB I/O credit fix live, multi-document, multi-hour, on the current binary

**Scope.** §1.10/§2.2 above. H9's four root-cause fixes are laws-green but the specific live failure C10 and
S15 independently hit (long uptime / many documents / many distinct plugins) has no reported re-run against
the fixed binary + current catalog. This blocks trusting *any* other live collaboration proof this session,
since every one of C10's and S15's blocked items cites this exact failure.
**Acceptance.** Re-run C10's `c10out6`-style long-lived-document scenario or S15's ~20-document/multi-plugin
`hub-module` journey against the current-tree binary + W2's current catalog on all three backends; 0
`"DB I/O … exhausted"` refusals.
**Owner.** H9 (owns the fix) with C10 or S15 (own the repro harnesses) — **UNOWNED as an explicit session-12
item today.**

### P0-2 — Confirm document creation (P0-1 from S11) on postgres and neo4j specifically

**Scope.** §1.1 above. H9's Item a closes the sqlite leg with a measured pass; the postgres/neo4j leg is
stated as "running" with no completion reported. Since presence/socket/WAL-fence mechanics are already proven
live on all three backends, this is the single remaining piece to call S11's P0-1 fully closed.
**Acceptance.** The same creation law (or the DB4 five-step document lane) passes live on postgres and neo4j
using a trusted catalog from the current tree.
**Owner.** H9 (Item a is explicitly "in progress" in its own report) — **already in H9's stated scope**, not
newly assigned.

### P0-3 — Land / confirm `--packages all` on canonical hub 7800

**Scope.** §1.6 caveat. Catalog B (9 packages) is proven; the full 34-package catalog publish was in flight
at session-11 close and every process died at session-12's 19:30 restart, so its status is unknown at audit
time. Every downstream collaboration/frontend proof that needs writer/draw/puzzle/wfc/block etc. on the
*canonical* hub depends on this.
**Acceptance.** `wp-w2.md` reports a completed `--packages all` publish and a 7800 restart onto it, with the
open-plan probe passing for all creatable kinds.
**Owner.** W2 (sole wasm/catalog owner per the session-12 preamble, rule 2) — **already W2's stated next
step** ("Next: the full `--packages all` catalog... then restart onto it").

### P1-1 — Re-run the docs/label defect: hub creation catalog labels every kind "Editor"

**Scope.** Both C10 (F4) and S15 (§13 finding (c)) independently hit the same hub-side defect: the creation
catalog labels a kind with its surface app's label (`app.label`, always "Editor"/"Editor"), so a human kind
picker cannot distinguish `2d.drawing` from `text.document`. Reported twice, fixed by neither — a real UX/API
defect in `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` (`artifact_creation_catalog`).
**Acceptance.** The creation catalog answers a localized per-kind label (en+de, AGENTS.md's no-default-
language rule), not the app's own label.
**Owner.** **UNOWNED.** Needs a localized-kind-label source — hub/guest descriptor owners per S15's framing.

### P1-2 — Postgres/neo4j hub-level graceful-shutdown-then-restart-under-1s, live

**Scope.** §1.9. The code fix (unconditional `close_hub_database`) should close S11's P2-2 (Neo4j lease
surviving shutdown) by construction, but H9 states the live hub-level proof on postgres/neo4j explicitly
"still to run," blocked on the same tree-consistent-catalog dependency as P0-2.
**Acceptance.** SIGTERM → exit, `database=closed`, reopen on first attempt, measured on postgres and neo4j at
the hub level (not just the `db` WAL-writer-fence level, which is already proven).
**Owner.** H9 (Item b, already scoped as "still to run") — not newly assigned.

### P1-3 — Idle-release of compiled wasm guests (unbounded RSS growth)

**Scope.** §2.1. Explicitly named by W2 as a follow-up, not started: compiled guests cache per-package with no
eviction, 578→346→1002 MB measured across one short session touching 7 of 9 catalog-B packages. On a
long-lived hub serving many distinct plugins this is unbounded.
**Acceptance.** An idle guest's compiled representation is released after some bound (time or LRU/count);
RSS after N creations across all catalog packages, then idle, returns toward the post-boot baseline.
**Owner.** **UNOWNED**, W2 named it but did not claim it ("not done").

### P1-4 — Cold `docker build` has still never been run

**Scope.** §1.12, carried unchanged from S11's P2-3. `docker build --check` and `docker compose config` both
pass; the actual multi-stage build (full release dependency graph, no shared build-dir) has never completed
once in this ticket.
**Acceptance.** `docker build -f 🌎️hub/Dockerfile -t semio/os-hub .` succeeds; `docker run` answers
`/healthz`.
**Owner.** **UNOWNED** (S11 suggested R8/build-health or NEW; no session-11 or -12 slice claims it).

### P2-1 — Localized per-kind creation label is also blocking a clean kind-picker UX proof

Same underlying defect as P1-1; listed separately in the S11-style ranking only because C10's and S15's
probes both had to work around it by index rather than by label — every future live-journey probe inherits
this brittleness until fixed. Folded into P1-1's acceptance; not a separate owner.

### P2-2 — In-process `cargo test -p …-db --lib` flake under load (test-infra only)

**Scope.** §2.5. H9 attributes this to the shared 2-worker nextest-alternative test pool, not the live hub;
nextest itself (one process per law) is unaffected. Low production risk, real CI/dev-loop friction.
**Acceptance.** Either raise the shared test pool's backend-control capacity for in-process runs, or document
that in-process `--lib` runs of this crate are known-flaky under load and nextest is the authoritative gate.
**Owner.** **UNOWNED**, small.

### P2-3 — Cross-platform (native Windows/Linux, devcontainer) hub boot — still zero evidence either way

**Scope.** §1.12. AGENTS.md requires zero-touch cross-platform setup; every capture in this ticket (S11 and
S12 alike) is Darwin/arm64. Not contradicted, not confirmed.
**Acceptance.** At least one boot log from a non-Darwin host or CI runner.
**Owner.** **UNOWNED.**

### P2-4 — Backup/restore procedure has never been live-drilled

**Scope.** §1.13. The documented procedure is sound and honestly scoped, but "stop → tar → restore into an
empty path → `/readyz` 200" has no measured run anywhere in this ticket's evidence chain.
**Acceptance.** One live drill on sqlite (cheap) and, if practical, one on postgres+neo4j (need their own
native backup alongside the tarball, per the README's own warning).
**Owner.** **UNOWNED**, small and cheap.

---

## Honest gaps in this audit

1. Time-boxed read of exactly the five files named in the task plus direct spot-checks of the hub bootstrap
   route file, the DB I/O storage file, `HubSagas`, the Dockerfile and the README's backup/topology sections
   — did not open every file under `🌎️hub/` or `🛢️db/` (both are large multi-thousand-line trees); code-level
   risks above are what a targeted grep for panics/unbounded-growth/blocking-calls/trust-boundary patterns in
   the highest-traffic files (bootstrap route handler, DB I/O ledger) surfaced, not an exhaustive audit.
2. `wp-h9.md` and `wp-w2.md` have no `## Session 12` section at read time — every "DONE this session" claim
   above is actually session-11 work (dated within 2026-09-25's earlier hours) that this audit is folding
   into "session 12's opening state" per the task's own framing ("current state carried forward"); nothing
   in this report should be read as session-12 work having already happened, since session 12 only started at
   22:50 and every process had just died.
3. Postgres/neo4j document-creation (P0-2) and hub-level graceful-shutdown-restart (P1-2) are both reported
   by H9 as blocked on the same tree-consistent catalog W2 was mid-rebuilding at every point this audit's
   source reports were written — their true status may have changed already; re-check `wp-h9.md` and
   `wp-w2.md` directly before assuming this report's PARTIAL/UNVERIFIED marks still hold.
4. Inference services (§1.7) were not independently re-read this pass beyond confirming no session-11/12 hub
   report mentions touching them — carried forward from the S11 audit's PARTIAL finding without fresh
   verification.
5. Did not re-read `🌎️hub/README.md` end to end (only the sections cited: topology table, backup/restore,
   TLS-proxy); RB1's account (per the S11 audit) of nine documentation fixes was not independently re-diffed
   again this pass either.
