# S11 Audit — Outcome 2: working server hub backend (db, presence, auth, …)

Read-only, Sonnet 5, 2026-09-25 ~00:40 (session 11, before any session-11 slice landed work — H9/H8's
own status tables are still `TODO`). No builds/servers/edits. Method: read the full chain of hub-backend
audits/slices in order — `audit-hub-backend.md` (09-23 baseline) → `g16-hub-backend-reaudit.md`,
`g15-production-readiness-reaudit.md`, `g12-deploy-and-onboarding-audit.md` (09-20/21, `.tmp-ticket-0918/`)
→ `rb1-release-builds-and-production-posture.md` (09-21 session 7) → `db2/db3/db4` (09-21/22, sessions
7-8) → `peer-audit-g10.md` (09-24 12:45, session 10 start) → `wp-h2.md`…`wp-h9.md`, `wp-r4.md`, `wp-r7.md`
(09-24, session 10, the newest evidence) — then spot-verified the highest-leverage claims directly against
the current tree with `/usr/bin/grep` (all six checks below matched; none had drifted). H9 (session 11's
own hub slice) has not started — its status table is `TODO` end to end, so this report's "current state"
is exactly session 10's closing state, carried forward unchanged into session 11's start.

**Headline**: outcome 2 moved further in the last 48 hours than in the whole ticket before it. Auth,
rate-limiting, agent delegations, observability, graceful shutdown, and — new since the last hub audit —
**Postgres and Neo4j are now live-connected and pass the full two-client e2e (open/relay/presence/
lease-expiry/rejoin/restart) on all three backends**, with a real WAL-writer fence for both non-sqlite
backends (H3) and cross-backend graceful shutdown (H5). A real release binary, a real production network
bind behind a simulated TLS proxy, real distribution tarballs, and a rewritten production-topology
Dockerfile all now exist and were runtime-observed (RB1). Against that: **artifact/document CREATION is
broken on every backend, including in-process sqlite** (DB4, 09-22, root-caused, partially instrumented,
not yet fixed) — this is the single most severe unresolved defect for outcome 2, more severe than DB4's
own framing suggests, because it blocks the "create a real document and collaborate on it" proof for the
whole outcome, not just the Postgres lane. The genesis-authority-is-hardcoded-to-3-kinds gap (TC3) is
unchanged and unowned. `HubSagas` is still a real supervisor cycling over zero deciders.

---

## Capability-by-capability status

### 1. Storage backends — sqlite / Postgres / Neo4j

**sqlite: WORKS-LIVE.** Kill+restart persistence proven repeatedly across sessions (H1b, P4, and again this
session H2/H3/H5's staged binaries). `connect_db` at `🌎️hub/🏗️bootstrap/🦀️.rs`, backend selected by
`OS_HUB_STORAGE_BACKEND`.

**Postgres / Neo4j: WORKS-LIVE, upgraded from COMPILED-ONLY since the last hub audit.** This is the
biggest correction to carry forward — `audit-hub-backend.md` (09-23) and `g16` (09-21) both said these
backends had "never been connected to... only a type-check." That is now **false**:
- DB2 (09-21): both directory lanes pass against real `postgres:16`/`neo4j:5-community` containers.
- H3 (session 10, 09-24): root-caused and fixed the missing **WAL-writer fence** for both backends
  (Postgres session advisory lock, Neo4j lease node + fencing token — `🛢️db/🗄️storage/🐘️postgres/🦀️.rs`,
  `🌐️neo4j/🦀️.rs`), then ran the **full two-client e2e (open, Ack relay, presence join/roster
  replay/leave/15s-lease-expiry, late joiner, rejoin, hub restart) 2/2 PASS on sqlite, postgres AND
  neo4j** with independent `psql`/`cypher-shell` cross-checks (`wp-h3.md` §Evidence).
- H5 (session 10): added graceful shutdown + the same 2/2×3-backend proof again, plus
  `directory-live-lanes`: postgres 12/12, neo4j 7/7, corpus 3/3 (`wp-h5.md`).
- DB4 (session 8, 09-22): first `/readyz` **200 with every gate open on a non-sqlite backend**
  (postgres+postgres and postgres+neo4j directory), hub suite with both drivers compiled in: 349 run,
  341 passed, 8 timed out (7 of 8 are one Neo4j-container-concurrency defect, fixed in
  `.config/nextest.toml` but the fix itself is unexecuted — see Gaps).
- **What sqlite has that Postgres/Neo4j do not: nothing, structurally** — DB4 §6 verified `HubDirectory`,
  the 33-variant `DbIoTask` set, sagas and the rate limiter are identical closed-set dispatch across all
  three backends; the asymmetry is "what has been run," not "what exists."
- **Regression found in the same pass (DB4, 09-22, still open at session-11 start — see Gaps P0-1):**
  artifact/document creation fails on **every** backend, not just Postgres. Root cause: genesis authority
  moved to the guest wasm component (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:362`,
  verified current), so the only catalog DB4 could build without a wasm compile (the
  `integration-fixtures` synthetic-bytes profile) fails at the component decoder
  (`"expected WebAssembly component version 13.1"`). This makes the Postgres/Neo4j "document" leg of
  outcome 2 **PARTIAL, not WORKS-LIVE**: the hub, directory and readiness gates are live; a real document
  has never been created on any non-sqlite backend, and — per DB4's in-process sqlite reproduction of the
  same failure — not reliably on sqlite either without a tree-matched wasm catalog.
- **Store versioning**: WORKS-LIVE, adversarially proven (P4 hand-edited `format.json` to v2, hub refused
  the exact named message, exit 1). No migration framework exists, by explicit repo design
  (`AGENTS.md`-aligned "no legacy support" + the store's own doc comment) — this is a real, permanent
  limit on upgrade continuity, not a gap to fix.

### 2. Event-sourced ledger + projections (CQRS, no CRUD)

**WORKS-LIVE** for the command/event/projection spine (directory + artifact authority); **MISSING** for
sagas as a concept, despite real production wiring around the empty set.
- Directory command path + read models: real, event-sourced, exercised live through restarts (H1b, DB4).
- Artifact authority: CQRS/event-sourced, server-authoritative rebase/reject
  (`ApplyOutcome::{Transformed,Rejected}`), no CRDT anywhere — verified unchanged.
- `HubSagas` — **verified still `pub enum HubSagas {}` at `🌎️hub/🗄️stores/🦀️.rs:229`** (grepped this
  pass, current tree) — an uninhabited enum. `SagaDrainSupervisor` runs a real 500ms-cadence drain,
  started after listener bind and drained once more on shutdown, and is unit-tested — but there is
  **nothing registered to run**. No session-10/11 slice touched this (grepped every `wp-*.md`: zero hits
  for `HubSagas`). Ranked P2 below: either register one real saga decider or delete the machinery and say
  so.

### 3. Directory (spaces, members, roles, invites)

**WORKS-LIVE.** Unaffected by this session's churn — AU1/AU3's original live-sign-in proof (create-space,
invites, role promotion) is still exercised by every later e2e (H2/H3/H5's two-client harness creates a
space and promotes a second member on every run; DB4 §3 does the same on Postgres). No new defects found.

### 4. Auth (sign-in, sessions, rate limits, agent/delegated credentials)

**WORKS-LIVE**, the largest reversal from the oldest baseline in this ticket (the 09-23
`audit-hub-backend.md` never even mentions a login route existing; two audit generations later this is a
mature, production-observed surface).
- Credential sign-in: `POST /auth/sessions`, PBKDF2-HMAC-SHA256 (210k iterations), constant-time verify —
  mint/introspect/revoke/expiry all observed live (H1b), and again over a **real network bind in
  production mode** by RB1 (`user_id 01a0c33b…`, wrong-password 400, session persists across restart).
- Rate limiting: 4 classes, subjects hashed and domain-separated (remote address + claimed identity both
  charged) — observed live producing 429 with `retry-after`.
- Agent delegations: the fullest live proof in the whole hub surface — M6b (cited by g16) drove
  sign-in→delegate→credential-file→MCP `--hub`→`context_resolve` reports the delegated principal→revoke→
  cascading 401, all live against a real MCP binary and a real hub. RB1 additionally proved the
  **release** MCP binary against a documented `.mcp.json`-shaped config end to end (27 tools, real
  `PERMISSION_DENIED` boundary), and **found and fixed a real defect**: 3 of the 7 documented `--scopes`
  names in both READMEs were not real scope names and silently granted nothing
  (`🌉️mcp/🛡️policy/🦀️.rs:26-53` `MCP_SCOPE_TABLE`) — fixed in source and docs.
- Production reachability: `identity_verifier` is **verified still hard-coded `None`**
  (`🌎️hub/🏗️bootstrap/🦀️.rs:10721`, current tree) — by design; `validate_auth_startup` accepts
  `OS_HUB_CREDENTIAL_SIGN_IN=true` as an equally-valid identity authority, which RB1 proved live on a
  **real, non-loopback network bind** (192.168.178.70:7661) with the allowlist/proxy posture enforced,
  not advisory (unproxied requests refused 403).
- Gap, real and by design: no external IdP (SSO/OIDC/WebAuthn) exists or is started.

### 5. Authorization (declared policy vs hand functions)

**PARTIAL, unchanged this session.** peer-audit-g10 (09-24) logs this as new finding **O2-10** ("authorization
as 5 hand functions") and marks it untouched; H9's own skeleton lists "declared authorization (O2-10)" as
item 3, status **TODO**. No session-10 or session-11 work has touched this yet. Distinct from the MCP-side
scope table (§4 above), which is a real declared table — this gap is about the **hub's own** admission
checks (membership/role → `SecurityGate`, admin-subject checks) being ad hoc functions rather than a single
declared policy surface.

### 6. Presence

**WORKS-LIVE for the sqlite/postgres/neo4j document-socket lane** (upgraded from g16's "asymmetric/decaying,
un-root-caused" finding). H2/H3/H5's two-client e2e — run on all three backends — explicitly proves: join
with hub label + UI, **roster replay to a late joiner**, leave drops the peer, 15s lease expiry strips a
peer to identity while its socket stays open, rejoin with resume all PASS 2/2 on sqlite, postgres, and
neo4j (`wp-h2.md`, `wp-h3.md`, `wp-h5.md`). This directly retires g16 (d)'s open item ("the document-socket
roster is asymmetric... not root-caused" — C3's finding from an older guest generation) for the harness
that now exists; it does not re-run C3's original two-browser scenario, so a claim that C3's *specific*
browser defect is fixed would be a stale extrapolation — flagged, not asserted. Presence stays ephemeral
by design (broadcast fan-out, not written to `db::Database`), confirmed unchanged.

### 7. Document sockets + resume/late join

**WORKS-LIVE**, and materially redesigned this session. H2 (session 10) **deleted** the framework-level
`HubDocumentAuthority` and made hub the sole owner of `GET .../document/ws`, closing the exact
impersonation gap `work-packages.md`'s "Security (impersonation)" line documents as fixed by the earlier
C4d slice (`?actor=` removed; actor bound server-side from the resolved credential). H3 then removed the
document-socket-grant's unused secret (schema-first `DocumentSocketGrantReceiptV1`, credential+binding
admission only). Resume/late-join, catch-up replay, and idempotent resend-after-reconnect are all
exercised in the same live two-client e2e across all three backends (§6 above). A real defect was found
and fixed along the way (H2's "late fix"): the guest minted a **constant** edit id per process, so a
resend after reconnect was silently treated as an idempotent replay and never advanced `head_seq` — fixed
at the Store/db layer by H4 (wasi:random entropy, replica-scoped ids, hub refuses colliding-content
resends) but **not yet live-proven**, because it needs a wasm guest rebuild from the current tree, which
W1/W2 owns (see Gaps P0-2).

### 8. Trusted catalog + open plans + lazy plugin delivery

**PARTIAL.** Catalog publish/load/reuse-across-restart mechanics are WORKS-LIVE (RB1 booted a release
binary against a catalog published by a *different* binary, 9/9 gates resolved with no republish). The
**creatable-kind surface is MISSING almost entirely and by hardcoded design**: verified this pass,
`NATIVE_OPENABLE_PROVIDER_SET_V1_ID = "stdio+gis+vcs/native-codecs/v1"` is still exactly 3 entries
(`🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:9`, current tree, unchanged since g16's
09-21 finding), and of those three `vcs` is separately non-selectable (kind-string mismatch, TC2). So
**exactly two kinds — stdio and gis — can ever have a new document created on a hub**, regardless of how
many of the ~30 plugins with declared kinds are installed. TC2's own 4-step "catalog-carried genesis"
redesign (§9 of that report) is fully specified and **still not started by anyone** (no `tc3-*.md` exists;
no session-10/11 `wp-*.md` mentions TC3/`artifact_creation_selection`/`NATIVE_OPENABLE_PROVIDER`).
"Lazy plugin delivery" (fetching a wasm component on demand rather than eagerly) is a separate, working
mechanism per the catalog/publication design — not itself broken — but is gated behind the same
2-creatable-kinds ceiling for anything a user would actually *create* fresh.

### 9. Check-in / checkpoints

**MISSING** for the hub-materialized command; **PARTIAL/decided-but-blocked** for the existing upload
path. H7 (session 10) made the **decision** to keep the `checkpoint-publications` route (verified this
pass, still mounted at `🌎️hub/🏗️bootstrap/🦀️.rs:10355`) because GIS-Map approval and every cold-open/
cold-mount path depend on it as the only thing that ever advances the approval baseline — but os-client
wiring to actually *call* it from a real Check-In gesture is **BLOCKED**: no guest exports its canonical
pack+spr pair to the host today (H7 §G5). H7's own recommended fix — the hub materializes the checkpoint
itself from its WAL tail instead of needing a client upload — became **H8's whole mandate** in the same
session, and **H8 never started**: its status table is 100% `TODO` (schema-first check-in command, hub
job with progress/cancel, deleting the old upload path, os client wiring, laws) — carried forward
unchanged into H9, which lists the identical item 1 also still `TODO` at session-11's start. This is a
straight continuation gap, not a newly-found defect.

### 10. Inference services

**PARTIAL, unchanged in scope this session, substantially hardened underneath.** Still scoped to GIS only
end to end — `execution_not_wired_error` remains the live behavior for every non-GIS declared inference,
no LLM-provider abstraction exists (g16, unchanged, not re-verified this pass but no session-10/11 report
touches `💡️inference`). Underneath that unchanged ceiling, HT3a-HT12 (pre-session-10) drove the inference
test family from 58 reds to what g16 called "3, fixed-in-source-unverified" — R4/R7 (session 10) then
independently ran the full kernel/db suites this fix sits inside to **765→769/769 nextest green ×3 and
plain in-process `cargo test` 769/769 ×3**, which is materially past g16's "nobody knows if it's actually
green" finding, though no report in this ticket names the inference suite number specifically post-fix.
`inference_submit/events/cancel/approve`'s GIS-only hardcode is untouched (peer-audit-g10, confirmed no
later slice claims otherwise).

### 11. Observability (structured trace, admin)

**WORKS-LIVE**, and this session **corrected a stale gap in the ticket's own bookkeeping**: peer-audit-g10
(09-24 12:45) logged "hub emits structured trace... **STILL OPEN** (N8)... NOBODY" as its gap G4. H7
(session 10, same day) found this framing itself stale — the facility (`semio-framework-trace`,
zero-dependency, `TraceRecord`/`TraceLevel`/`TraceOutcome`/`Tracer`/pluggable `TraceSink`) already existed
from the 0918 ticket and the hub was already reporting through it. H7 closed the real remaining gaps: no
schema for the record line (added, draft-07 + 13 fixture vectors), no file sink (added, survives restart),
socket admission/close using two unrelated request ids (fixed, `Span::fork()`), no presence records
(added: `server.presence.{join,expiry,leave}`), and a 13→23 event vocabulary drift. Live-probed on a
booted binary (9/9 trace lines valid + declared) and wired into the cross-backend two-client e2e (asserts
the whole trace including `database=closed` on shutdown). The document-socket-open/close + presence *live*
leg specifically is still blocked on a tree-consistent wasm catalog (same W1/W2 dependency as §7). Admin
SPA (`/admin/api/*`, gated by `authenticate_admin_principal`) unaffected, still real per g16.

### 12. Graceful shutdown

**WORKS-LIVE across all three backends**, upgraded since the last full audit. H5 (session 10) built
`HubSocketDrainV1` (every socket gets 1012 `hub-shutdown` + cleanup on SIGTERM/SIGINT, ≤5s deadline) and
`close_hub_database` (drives Postgres advisory-unlock+session-close, Neo4j lease release, sqlite/fs sidecar
release, ≤10s deadline) — live-proven with a **held socket** across the signal on sqlite (SIGTERM: exit 0,
socket closes cleanly, `database=closed`), postgres (same, exit 0, server released the lock), and neo4j
(same, exit 0, though the **lease itself is not released by graceful shutdown** — a real, named gap: hub
never calls `Database::shutdown` before process exit finishes on the Neo4j path in one code shape H5 flags,
so a restart within the 15s TTL gets `Conflict`; SIGKILL naturally leaves the lease live for its TTL on
all backends, expected). RB1 independently re-proved SIGTERM drain on a **release** binary with
`RUST_MIN_STACK` unset (i.e., outside cargo's 64MiB floor) — this closes HS1's stack-overflow production
correctness fix for the exact standalone-process topology it was written for.

### 13. Release build + deployment (container/devcontainer, zero-touch local hub, cross-platform)

**PARTIAL, dramatically improved this session (RB1), one leg still not closed.**
- `os-hub` release binary: **built for the first time in this ticket** (138.5 MiB, 39min, RB1 09-21) and
  since re-used by later slices as a proof platform.
- `semio-os-mcp --release`: **built for the first time**, proven end-to-end over a documented config.
- Distribution tarballs (`os-hub`, `semio-os-mcp`, checksummed): **produced for the first time** from real
  release binaries.
- Docker: image still **unbuilt** as a container (no cold `docker build` was run — would recompile the
  whole release graph with no shared build-dir), but `Dockerfile`/`compose.yaml` were **rewritten from the
  dev topology to the production topology** (binary-only image, `tini` entrypoint, `0.0.0.0` bind inside
  the container, `OS_HUB_ADMIN_DIR`, proxy-aware healthcheck) and validated with `docker build --check`
  (exit 0) + `docker compose config` (resolves) — both previously unrun, both real defects found and fixed
  in the process (the old Dockerfile bound `127.0.0.1` *inside* the container, which would have made the
  published port unreachable; the old healthcheck spoke plain HTTP, which the new transport-security
  middleware would 403 forever).
- `build-s-react-release` (the `s` frontend's own release bundle — outcome-1-adjacent but load-bearing for
  "self-hosted hub + your own compiled frontend"): **still never completed**. RB1 run 1 failed on peer
  churn after 2h29m (33/99 tasks red); run 2 was queued behind the fleet's wasm mutex and its outcome is
  not recorded in any report read this pass — **UNKNOWN**, most likely still open (G15's ranked item #7,
  carried since 09-20).
- Zero-touch local hub: real (`os-hub:dev` self-provisions sqlite + fs + trusted catalog on first boot),
  unaffected this session; this session's contribution is entirely on the *production*-deploy side.
- Cross-platform: no evidence of a non-macOS build attempt anywhere in this ticket (every capture this
  pass and prior is Darwin/arm64); untested, not claimed either way.

### 14. Security (impersonation, input validation)

**WORKS-LIVE for the impersonation vector this ticket tracked.** `work-packages.md`'s own top-line entry
records the finding and fix: the framework `document/ws` route used to take the socket actor from the
caller's `?actor=` query param with no server-side check — WP-C4d closed this (actor bound from the
resolved credential; anonymous → Unauthorized), and H2 went further, deleting the vulnerable framework
route entirely so hub's own admission-checked route is the *only* document socket in the product. Verified
current (H2's design section, still true per this session's e2e evidence). CORS: real allowlist,
non-advisory, proven against a real preflight from both an allowed and a foreign origin (RB1). No new
input-validation defect surfaced by any report read this pass; general fuzz/malformed-input coverage was
not independently assessed in this audit (out of the 45-minute budget) — treat as **UNKNOWN**, not
"clean," for anything beyond the specific impersonation/CORS vectors above.

### 15. Progress / cancel on hub jobs

**PARTIAL.** Artifact-creation is a real job with cancellation semantics per g16 (unchanged, not
re-verified this pass). H8/H9's planned check-in job explicitly specs "progress/cancel" as a requirement
(item 2 of H9's table) but that job does not exist yet (§9 above). Inference jobs have real
`notifications/progress` phases on the MCP side (peer-audit-g10, `artifact_create`/binding phases) — this
is client-visible progress for a hub-driven job, live-observed with cancellation latencies of 1-17ms
across multiple slices. No hub-side job was found this pass that has progress/cancel *specified* but
provably missing, beyond check-in.

---

## Stale-claim flags (explicit)

1. **`audit-hub-backend.md` (09-23) and `g16` (09-21)**: "Postgres/neo4j... COMPILED-ONLY, never connected
   to." **False as of DB2/H3/DB4** (09-21/22/24) — both backends are live-connected, WAL-fenced, and pass
   the full two-client e2e. Anyone reading only the 09-23 baseline audit today will materially
   underestimate outcome 2.
2. **`peer-audit-g10.md` (09-24 12:45)**: hub observability "STILL OPEN (N8)... NOBODY" owns it. **Stale
   within the same day** — H7 (session 10, same date) found and documented that this framing was already
   wrong even at G10's own writing (the trace facility predates the 0918 ticket) and then closed the real
   remaining gaps. Also flagged this gap as G4 in its own list; H9's skeleton correctly does not re-list it.
3. **`peer-audit-g10.md`'s G6** ("plain in-process db `--lib` gate... NOBODY... actually R7, downgrade").
   **Now fully closed** — R7 (session 10) got plain in-process `cargo test -p semio-framework-os-kernel-db
   --lib --all-features` to **769/769 ×3**, not just nextest. This should be removed from any forward-
   looking gap list entirely, not merely downgraded.
4. **`🌎️hub/README.md`** (per RB1, itself already corrected once): confirm on next read that RB1's nine
   documentation fixes (network-bind refusal claim, "no metrics" claim, "production never run" claim, the
   tarball/container claims) are still the current text — this report did not re-diff the README, only
   relied on RB1's own account of having fixed it.
5. **DB4's genesis-authority defect is more severe than DB3's framing ("no artifact was created on the
   Postgres hub") suggested** — DB4 proved the identical failure in-process on **sqlite**, so this is a
   whole-hub regression from a prior redesign (genesis moved from linked native codec to guest authority),
   not a Postgres-specific gap. Any status line that still frames this as "Postgres-only" is stale; carry
   the corrected framing forward.

---

## Ranked gaps (P0/P1/P2)

### P0-1 — Artifact/document creation fails on every backend (sqlite, postgres, neo4j)

**Scope.** `VerifiedNativeArtifactCodec::initial_pair` calls `self.guest.genesis(document_id)`
unconditionally (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:362`, verified current) — genesis
authority is always the guest wasm component's, never the linked native codec. The hub's own law
`space_artifact_creation_routes_are_author_owned_idempotent_and_genesis_backed` fails at exactly this step
in-process on sqlite with no database container involved (DB4 §4.1); the same failure reproduces
identically on live Postgres and Postgres+fs. This blocks "create a real document and collaborate on it"
for the entire outcome, not one backend.
**Acceptance criteria.** A `POST /spaces/{s}/artifact-creations` for `s.gis.gismap` (or `s.stdio.*`)
reaches phase `ready`, not `failed`, on all three backends, using a trusted catalog built from the current
tree (not the `integration-fixtures` synthetic-bytes profile). The hub's own creation law passes
in-process. DB4's five remaining document-lane steps (two sessions edit, restart, re-attach) then run to
completion on at least the Postgres lane.
**Owner.** **W2** (it needs a real wasm component in the catalog — TC3e's un-finished republish is the
direct precedent DB4 names) working with **H9** (H9 already owns "store writes that can fail" and the db
gate, adjacent code) or **NEW** if W2's queue cannot absorb it. Not cargo-light: needs the wasm mutex.

### P0-2 — Agent/human edit-id collision fix is unproven live; blocks 3 outcomes on one dependency

**Scope.** H4 (session 10) root-caused and fixed in source (Store replica identity via wasi:random
entropy, db collision refusal) the defect where every fresh guest process minted the *same* edit id for
the same first gesture, silently losing concurrent human/agent edits as idempotent replays. The fix is
laws-green at the Store/db layer but has **never run live**, because it needs a wasm guest rebuild from
the current tree — every rebuild attempt in H4's own log failed for unrelated reasons (stale catalog bind,
cold-compile timeout, a peer's structural schema-hash change, peer compile breaks, two silent job deaths).
This is the **same shared dependency** peer-audit-g10 flagged as blocking Outcome 3's ten-step
collaboration scenario and Outcome 4's "agent edit seen by a human, ledger actually advances" proof.
**Acceptance criteria.** `hub-edit-durability-check` (already written, `⚖️gate🌉️os-mcp🤝️hub-edit-durability`
in launch.json) goes green: two fresh MCP agent lanes performing the identical gesture both commit with
distinct edit ids, head_seq advances per commit, a late joiner's catch-up shows all of them.
**Owner.** **W2** (guest rebuild + catalog publish) is the blocking dependency; **H9** or **G10** re-runs
the gate once a tree-consistent catalog exists.

### P0-3 — Hub-materialized Check-In command does not exist

**Scope.** H8's full mandate (schema-first check-in command, hub job that materializes a checkpoint from
its own WAL tail with progress/cancel, deletion of the now-superseded client-upload
`checkpoint-publications` path, os client wiring, laws) never started — 100% `TODO`, carried unchanged into
H9. Without it, GIS-Map approval's baseline can only ever advance through the old upload path, which is
itself blocked on a guest ABI export nobody has built (H7 §G5).
**Acceptance criteria.** A user-facing "Check In" gesture in `s` produces a hub-materialized checkpoint;
head advances; a stale check-in is refused; approval-after-edit-then-check-in succeeds; a cold open starts
from the new checkpoint. All four laws H8 already specified pass.
**Owner.** **H9** (already the literal continuation of H8 in this session's plan).

### P1-1 — TC3: only 2 of ~30 plugin kinds can ever have a document created on a hub

**Scope.** `NATIVE_OPENABLE_PROVIDER_SET_V1_ID` is a hardcoded 3-entry Rust const table (one entry,
`vcs`, is separately dead from a kind-string mismatch), so genesis on a hub requires a statically-linked
native factory compiled into the `os-hub` binary itself — a manifest declaration is not enough. TC2 §9 has
a full, unimplemented, migration-free redesign (catalog-carried genesis) ready to build.
**Acceptance criteria.** A plugin with a declared `ArtifactKindSpec` and no native genesis factory can
still have a document created on a hub whose published catalog carries that plugin, without an `os-hub`
recompile.
**Owner.** **NEW** — no current session-11 slice claims this scope; H9's item list does not include it.
Suggest carving a dedicated slice next session (large: touches trusted-catalog + artifact-authority + the
catalog publish pipeline).

### P1-2 — Declared authorization policy (O2-10)

**Scope.** Hub-side admission (membership/role gates, admin-subject checks) is ~5 hand-written functions,
not one declared policy surface. H9 lists this as item 3, still `TODO`.
**Acceptance criteria.** One schema-first policy definition drives every admission check the hub makes
(document socket, directory command, admin route); a law asserts hand-written gates cannot silently drift
from it.
**Owner.** **H9** (already scoped, not yet started).

### P1-3 — Store writes (`ProjectionStore`/`SessionStore`) that can fail return `()`

**Scope.** O2-5b, unchanged since G10's original finding; H9 item 2, still `TODO`.
**Acceptance criteria.** Every store write that can fail (disk full, corrupt page, backend fenced) returns
a typed result the caller can act on; a fault-injection law proves at least one caller reacts correctly
instead of silently continuing.
**Owner.** **H9** (already scoped, not yet started).

### P1-4 — `build-s-react-release` still never completed end to end

**Scope.** G15's ranked item #7, still open after two attempts (RB1 run 1 failed on peer churn, run 2's
outcome is unrecorded in any report read this pass). Blocks "a self-hoster builds their own frontend
release and points it at their own hub" — the last piece of RB1's own scope.
**Acceptance criteria.** The bundle builds, is served, and boots headless against a live hub (RB1's own
`rb1-serve-release-bundle.ts` / `rb1-release-bundle-probe.mjs` are written and unrun — reuse them).
**Owner.** **W2** (owns the wasm mutex queue this needs) or **NEW**.

### P2-1 — `HubSagas` is a production-grade empty saga runner

**Scope.** `SagaDrainSupervisor` runs real cadence, drains on shutdown, is unit-tested — over an
uninhabited `enum HubSagas {}`. Either dead weight or a half-finished feature; both deserve an explicit
decision rather than silently shipping a CQRS layer that never does anything.
**Acceptance criteria.** Either one real saga decider is registered and observed draining a real
cross-aggregate effect, or the supervisor/enum is deleted and the decision is recorded.
**Owner.** **NEW** (small; nobody currently owns it).

### P2-2 — Neo4j lease not released by graceful shutdown

**Scope.** H5's own gap: the hub never calls `Database::shutdown` in one shutdown code path before
process exit, so a Postgres/sqlite writer releases via OS-level connection close but a Neo4j lease survives
until its 15s TTL — a restart inside that window gets `Conflict` for documents that were open.
**Acceptance criteria.** `Database::shutdown` (or its Neo4j-specific release) runs unconditionally before
`main` returns; a restart within 1s of a graceful stop opens on the first attempt on all three backends
(today only sqlite/postgres do).
**Owner.** **H9** (adjacent to its db-gate and store-write work) or **NEW**, small.

### P2-3 — Docker image itself is still unbuilt as a container

**Scope.** `docker build --check` and `docker compose config` both pass; a real cold `docker build` has
never been run (would recompile the whole release dependency graph with no shared build-dir).
**Acceptance criteria.** `docker build -f 🌎️hub/Dockerfile -t semio/os-hub .` succeeds on this host and
`docker run` answers `/healthz`.
**Owner.** **NEW** or **R8** (build-health slice), cargo-heavy but mechanical.

### P2-4 — 7-of-8 Neo4j-lane suite timeout fix (DB4) is landed but unexecuted

**Scope.** DB4's `.config/nextest.toml` `live-database-lanes` test-group fix (max-threads=1 for
`directory::(neo4j|postgres)::` tests) is in the tree but the confirming rerun (`349/349`) was never
performed — the machine hit load 267/swap-full right after landing it.
**Acceptance criteria.** `sh 📜️db4-hub-run.sh` (or equivalent) reports 349/349 (or the current total) with
the postgres/neo4j features compiled in.
**Owner.** **H9** or **R8**, cheap once machine load allows.

---

## Honest gaps in this audit

1. 45-minute time-box: read the full chain of prior audits and every session-10 hub-adjacent `wp-*.md`,
   but did not open every file in `.tmp-ticket-0918/` this ticket's own index names (e.g. `g2`, `g4`, `g7`,
   `h1`, `h1b`, `tc1`, `tc2`, `hs1`, `ob1r`, `jc1`, `m6`/`m6b` are cited only via the later audits that
   quote them, not independently re-read).
2. Six targeted `grep` checks against the current tree (HubSagas, native-openable const, checkpoint-
   publications route, `AuthorityOperationControl::fault`, `identity_verifier`) all matched their cited
   reports exactly — no drift found — but this is not an exhaustive re-verification of every citation in
   this report.
3. `🌎️hub/README.md`'s current text was not independently re-read; RB1's account of having fixed nine
   stale claims in it is taken on the report's word (flagged in stale-claims §4 above as worth a follow-up
   spot check).
4. No number exists anywhere in this ticket for "the inference test family's exact pass count after HT12's
   fix landed" — this report can only say the surrounding kernel/db suites are now provably green (R4/R7),
   not that the inference suite specifically was re-counted.
5. Input validation beyond the impersonation/CORS vectors (fuzzing, malformed-body handling across the
   ~60+ hub routes) was not assessed — marked UNKNOWN rather than assumed clean.
