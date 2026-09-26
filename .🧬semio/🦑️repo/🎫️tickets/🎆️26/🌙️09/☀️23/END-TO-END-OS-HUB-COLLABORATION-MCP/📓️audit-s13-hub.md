# S13 Audit — Outcome 2: working hub server backend (db, presence, auth, observability, operations)

Read-only, Sonnet 5 (A13-hub), 2026-09-26 ~19:1x. **No builds/servers/edits performed** (auditor mandate: report file only).
Method: read `AGENTS.md`, `📓️session-13-preamble.md`, `📓️audit-s12-hub.md` (full), the session-12 sections of `📓️wp-h9.md`
(lines 24–488), `📓️wp-h10.md` (whole file — it is entirely session-12), `📓️wp-db1.md` (whole file — session-12 baseline +
session-13 opening), `📓️wp-w2.md` Hub Handoff + Session 12 (lines 1–259), `📓️wp-z2.md` (whole file — entirely session-11,
no session-12 update exists). Cross-checked against: the session-13 slice reports that exist so far (`wp-h11.md`, `wp-c11.md`,
`wp-s16.md`, `wp-t13.md`, `wp-la.md`, `wp-lb.md`, `wp-lc.md`, `📓️fleet-13-agents.md`, `📓️landing.md`), a live `curl` of
`127.0.0.1:7800/readyz`+`/healthz`, `git status`/`git log` on hub- and db-related paths, `docker images`, and direct `grep`
of the hub bootstrap route file and the trusted-catalog file for specific claimed symbols.

## 0. Where things stand right now (19:1x), in one paragraph

Hub **7800 is live on catalog B2** (9 packages, 16 creatable kinds, runId `8d0ec7a5…`, `uptimeMs` ≈ 179 min, `/readyz`
`status:ready` with every feature gate open — confirmed live this pass). The **`--packages all` publish has now failed
four times** (latest: 18:45, imperative plugin's browser-actor wasm codegen, `unsupported import interface`,
`🔌️plugin/🌐️browser-bundle/📜️script.ts:317`) and session 13 opened a **landing window** that has not landed a single
row yet (`📓️landing.md` "# Session 13 Landing Window" table is a header with 0 rows). The git tree currently carries a
**huge amount of staged-but-uncommitted hub and db source** (H9's directory/RW-gate/GIS-approval/revocation fixes, DB1's
whole write-path rewrite) that **no build has compiled since it was written** — H11's own session-13 log says so
explicitly (`wp-h11.md:24-25`: "H9's session-12 hub edits sit staged in the index … no build has compiled them yet").
Last auto-commit was `d1dd02f785d` at **15:13:45**; everything in H9's log after roughly that time (directory latency
fixes, GIS-approval post-commit-check removal, G10 S4 revocation fence, opaque-concurrency prep, the DB I/O admission-wait
fix) and essentially all of DB1's session-12 write-path work postdate it and are **uncommitted**. Every session-13 hub-area
slice (H11, DB1, C11, Z3) is at its 19:0x-19:1x opening log entry; none has produced a fresh measurement yet. **This audit's
job is therefore mostly: confirm what session 12 actually closed with live evidence, and give session 13 an accurate,
file:line-cited punch list of what is claimed-fixed-in-source vs. actually-verified, plus what is still a bare gap.**

---

## 1. Session-12 P0/P1/P2 items — closed vs. open, with evidence

| Item (from `audit-s12-hub.md`) | Verdict now | Evidence |
|---|---|---|
| P0-1 Re-verify DB I/O credit fix live, multi-doc, multi-hour | **STILL OPEN**, materially progressed | H9 fixed 3 more stacked bottlenecks after the S12 audit was written (worker-submit retry policy, sync-hello admission waiter, DB I/O admission-wait instead of refusal — `wp-h9.md:246-268,438-451`), but the growth e2e never got a clean run across all three backends on one binary; DB1 then found the *real* bottleneck (index maintenance: 630 DB I/O tasks/commit → 2) and its own throughput laws are still red at session-12 close (`wp-db1.md:26`: "777/779, the two failures = the fs/sqlite throughput laws on the storm ratio") and TODO at session-13 open (`wp-db1.md:16-20`) |
| P0-2 Confirm document creation on postgres/neo4j | **CLOSED**, live evidence | `wp-h9.md:298-301` `tc19` neo4j PASS 2/2 (agent revocation 6 ms, 300+30 growth edits, restart+reopen clean); `wp-h9.md:347-354` `tc23` postgres: all 300×16 KiB edits now accepted (689 s; was a frame-deadline stall at edits 30-256), agent revocation 22 ms, execution-target 5/4 ms |
| P0-3 Land/confirm `--packages all` on 7800 | **OPEN, WORSE** | Now failed 4 times (demonstrator version pins 06:34; usage-limit cut 11:43; disk-guard self-inflicted deletion 15:54; imperative wasm codegen 18:45 per `📓️session-13-preamble.md:19-22`). Owner W3 has not yet produced a report (`wp-w3.md` does not exist; only `wp-w3/generated`,`wp-w3/requests` dirs, mtime 19:06-19:10) |
| P1-1 Kind-label "Editor" defect | **OPEN**, patch ready not landed | `wp-h9.md:146-152` (`kind-label-patch.py`, 184 sites/93 files, dry-run clean); now LC item 4, state `queued` (`wp-lc.md`) |
| P1-2 pg/neo4j hub-level graceful-shutdown-restart, live | **CLOSED**, live evidence | `wp-h9.md:34` "LIVE PASS … SIGTERM → exit 0.5–2.3 s … 0 live WAL writers left (pg, neo4j), restart spawned 22/207/31 ms … first-attempt reopen on sqlite (tc24), postgres (tc23), neo4j (tc19)" |
| P1-3 Idle-release of compiled wasm guests | **CLOSED but by a different design — re-open as a new risk** | H9's idle-sweep (`GuestResidencySupervisorV1`/`release_idle_guests`) was **deleted** by H10 and replaced with capacity-LRU-by-size (`wp-h10.md:100-103`); confirmed by `grep` — `release_idle_guests` no longer exists in `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`. This bounds RSS by a declared 256 MiB budget instead of releasing idle guests, which is a legitimate fix for the *stated* problem, but B2's 9 packages total 240.6 MB (`wp-h10.md:144-145`) — **under** the budget, so the LRU-eviction code path has *never actually fired*. The 34-package catalog will be the first real test and its publish keeps failing before reaching that point — see new P1 below |
| P1-4 Cold `docker build` never run | **STILL OPEN** | H10 got to build attempt 6 (context 23 GB→1.2 GB, jobs tuning, Linux-winit finding) but the coordinator stopped it for build-quiet at 15:58 (`wp-h10.md:230-231`); confirmed this pass: `docker images` on this host shows **zero** hub images |
| P2-1 Localized label UX (dup of P1-1) | still open, folded into P1-1 | — |
| P2-2 In-process `cargo test -p …-db --lib` flake | **STILL OPEN**, root-caused not fixed | `wp-h9.md:462-464` names the exact cause (backend-control exhaustion at `MemoryStorage::new`) and a plan, not a fix; now explicitly H11 item 3, state TODO (`wp-h11.md:15`) |
| P2-3 Cross-platform hub boot, zero evidence | **PARTIALLY CLOSED**, real progress, not landed | Z2 (session 11) proved a **native Linux cargo build/link of `os-hub` clean** in a plain Ubuntu 24.04 container (`cargo check -p semio-hub` 3 m 40 s, `cargo build --bin os-hub` 8 m 27 s, `wp-z2.md:70`) — but only with patches B1-B4 applied to a *copy*; none of B1-B4 is landed in the tree. Docker image still never built (see P1-4). Windows: zero live evidence (`wp-z2.md:140-160`, static-only). Session-13 owner Z3 has not started (no `wp-z3.md` yet) |
| P2-4 Backup/restore never drilled | **CLOSED**, live evidence | `wp-h10.md:127-134`: SIGTERM 277 ms → tar 164 MB/25 s → restore → ready 13.5 s → frontier/checkpoint byte-identical, 5/5; relocation 5/5; README updated |

**Net for session 12: 4 of 10 audited items closed with live evidence (P0-2, P1-2, P2-4, and P1-3-as-redesigned), 1 made
structurally worse (P0-3), 5 still open** (P0-1, P1-1, P1-4, P2-2, P2-3-partial).

---

## 2. Fresh gap list by capability area

### Auth (sign-in, sessions, delegation, revocation)
DONE and hardened further, **committed** (git shows these files clean against HEAD; H10's own commit `f7791a96178`
11:22:56 per `git log`): PBKDF2 single-HMAC-compression fix, `spawn_blocking` for derivation, timing-safe unknown-account
refusal (`wp-h9.md` handoff §"Session mint"). Session-mint p50 261-264 ms measured (`wp-h10.md:11`). **Open, uncompiled**:
the G10 S4 revocation-fence fix (`a_withdrawn_delegation_admits_no_agent_edit_after_it_in_either_order`,
`🌎️hub/🏗️bootstrap/🦀️.rs:8492` comment confirmed present by direct grep this pass) has never been compiled since it was
written (`wp-h9.md:389-399`) — H11 item 1 now owns compiling and re-running it.

### Directory / spaces / membership
Major latency fixes **in the tree, uncompiled**: `HubDirectory::list_visible_space_summaries` (confirmed present,
`🌎️hub/🏗️bootstrap/🦀️.rs:6832,6838`), one membership read per event page, new sqlite/postgres indexes, neo4j indexes
(`wp-h9.md:362-377`). **The RW-gate fix (shared/exclusive authority locks) that root-caused C10's directory-fanout race is
also uncompiled** (`wp-h9.md:172-190`) — this is the fix that makes a document socket frame not stall on an unrelated
space-wide lock; until it is compiled and re-verified, C10/S15's "document closed"/"target changed" flakes cannot be
considered fixed, only diagnosed and patched-on-paper. `git status` confirms all four directory backend files
(`sqlite`,`postgres`,`neo4j`,the dispatcher) plus `🏗️bootstrap/🦀️.rs` are `M` (staged, uncommitted).

### Documents (authority, submit, replay, checkpoints, durable redo)
`ArtifactWal::open`'s per-segment renewal (fixes "a long document can never reopen", `wp-h9.md:95-100`) and the WAL
outbox-duplicate removal are folded into the same uncompiled/in-flux region DB1 is now rewriting
(`🧰️…/🛢️db/📝️wal/🦀️.rs`, `🔢️index/🦀️.rs`, `🗿️artifact/🦀️.rs`, `🔄️sync/🦀️.rs` are all `M` per `git status`, several by
both H9 and DB1 in the same session). No single report this pass shows a green full db nextest run against the CURRENT
state of these five files together — the last confirmed run was DB1's own 777/779 at session-12 close (`wp-db1.md:26`).
Check In / durable redo: unchanged from session 12 (DONE, live) — not touched by any report read this pass.

### db engine (fs/sqlite/postgres/neo4j: durability, crash, fences, throughput)
Durability/crash/fence mechanics: unchanged, still DONE per session-12 evidence (WAL-writer fence 3/3 backends, live).
**Throughput is the single most active area in the whole hub scope right now**: DB1's session-12 root fixes cut one
16-envelope fs commit from **~630 DB I/O tasks / ~14 400 executor steps to 2 tasks** (`wp-db1.md:46-54,88`), sqlite hello
from 688 ms to 7.9 ms, and 24-doc growth ack p50 from 3.2 s to 27 ms — all measured, all real. But the **reopen/greeting-storm
bound is explicitly unfixed**: H9's g17 hit "0-0 missing Welcome" when 24 grown documents reopened at once
(`wp-h9.md:420-428`), root-caused to db write/replay throughput and hand-carried to DB1 as item 3 (`wp-db1.md:18`,
still **TODO** at session-13 open) — this is the scenario that blocks trusting a hub restart under any realistic
multi-document load. DB1's own two throughput laws (fs/sqlite storm ratio) were still the only two db nextest failures
at session-12 close.

### Presence (leases, expiry, symmetry)
Unchanged, DONE, live, from two independent harnesses (H9 two-client e2e, C10 hub-7800 harness). No report read this pass
touches presence — lowest-risk area in scope, but has not been re-run against any of the uncompiled edits above, so a
fresh smoke test after H11 compiles is still owed before trusting it under the new RW-gate/directory code.

### Trusted catalog (publish, boot, readiness, residency, memory)
**Publish**: blocked, 4th failure, now an imperative-plugin wasm codegen bug (`browser actor artifact: unsupported import
interface`, `🔌️plugin/🌐️browser-bundle/📜️script.ts:317` per `📓️session-13-preamble.md:20-22`) — this is a **new class
of bug** (not a version-pin/disk-guard/usage-limit failure like the first three) and is on W3's critical path; nothing
downstream that needs writer/draw/puzzle/wfc/block/imperative etc. beyond the 16 kinds already live on B2 can proceed
until it is fixed.
**Boot**: DONE and **committed** — H10's engine-keyed verification cache + row-parallel boot + off-worker codec calls cut
cold boot 483.8 s → 153.9 s, warm restart 14.5 s (`wp-h10.md:10,138`); confirmed live this pass (`/healthz` uptime ≈ 3 h
on the 16:11-restarted B2 hub).
**Residency/memory**: bounded-by-design (256 MiB LRU budget) but **the eviction path itself has never fired** — B2's 9
packages (240.6 MB) sit under budget (`wp-h10.md:144-145`); the 34-package catalog is the first real test and keeps
failing before publish completes. Flagged as a fresh, previously-invisible P1 below.
**Creation latency**: still the dominant user-facing cost — first-use `3d.puzzle` 422 s, `2d.puzzle` 354 s, `gis` 229 s on
B2 (`wp-w2.md:148-149`) — root-caused to the guest constructing every app of its bundle per codec call
(`wp-h10.md:70-84,146-152`); both fixes (Q1 interpreter ~2.2×, guest codec-app single-app resolution, jointly estimated
6-8× on `pack-schema-hash`) are fully prepared, dry-run clean, but **not landed** (guest-linked, ABI freeze) — now LA
items 4a-4c, state `pending`.

### Observability (logs, metrics, /readyz progress)
DONE, unchanged, and improved: boot-readiness now streams real progress from 68 ms instead of going dark for minutes
(`wp-h10.md:168-181`, `🌎️hub/🏗️bootstrap/🦀️.rs` `BootReadinessServerV1`). No metrics/Prometheus endpoint — documented
design choice, not a gap. Confirmed live this pass: `/readyz` body matches the declared schema exactly.

### Operations (Docker, backup/restore, graceful shutdown, upgrades)
Docker cold build: **still never completed** (§1 P1-4; `docker images` empty on this host, checked this pass). Graceful
shutdown: DONE, live, all three backends (§1 P1-2). Backup/restore: DONE, drilled twice (§1 P2-4). Upgrades: no
cross-version path by explicit design (matches AGENTS.md's no-legacy-support stance) — documented, not a gap.

### Security (hostile input, rate limits, CORS, TLS assumptions)
Hardened further and **committed**: credential-optional routes (`/directory/spaces`, `/directory/events`) used to answer
a forged/revoked bearer as anonymous 200 — closed (`wp-h10.md:120-125`, `resolve_optional_bearer_user`). Local-bootstrap
pipe liveness bug (any legitimate 65th admin-relay re-issue killed the whole hub) — fixed and live-proven
(`wp-h9.md:21`, a 70-exchange burst → 64 issued + 6 typed refusals, hub stays up). CORS/rate-limit/TLS-proxy assumptions:
unchanged from session 11, still sound. **Still UNVERIFIED, unchanged for three sessions running**: general fuzz/
malformed-body coverage beyond the specific enumerated vectors — nobody has picked this up; still a real, if low-severity,
gap.

### Cross-platform boot
See §1 P2-3. Net new information this pass: a real native-Linux `cargo build -p semio-hub` now compiles and links clean
in a container (`wp-z2.md:70`), which is genuine progress since the S12 audit had "zero evidence either way" — but every
one of B1-B4 is a patch sitting in `wp-z2/pending/`, not in the tree, and the session-13 owner (Z3) has not started.
Windows remains entirely unproven live. Devcontainer is on hold behind the same B4 patch plus an unrelated macOS
Docker-Desktop permission gap for `~/Documents` (`wp-z2.md:166-170`) that no one can fix without the user's own OS
settings change (outside any agent's authority — correctly left alone).

---

## 3. In-tree-but-never-compiled hub/db code (written during the session-12 build freeze, still true now)

Confirmed by direct `git status` + `git log` this pass (`d1dd02f785d` 15:13:45 is HEAD; everything below is staged, not
committed, and per H11's own log "no build has compiled them yet" — `wp-h11.md:24-25`):

| File | What's uncompiled in it | Source |
|---|---|---|
| `🌎️hub/🏗️bootstrap/🦀️.rs` (11 379 lines) | directory latency rewrite (`list_visible_space_summaries`), RW-gate shared/exclusive authority locks, GIS-approval post-commit-check removal (`revalidate_gis_map_approval_authority` at line 10411), G10 S4 revocation fence (comment at line 8492), boot-readiness server, refusal middleware, hostile-input typed refusals | `wp-h9.md:172-190,315-399`; `wp-h10.md:120-181` |
| `🌎️hub/📇️directory/{🦀️.rs,🪶️sqlite/,🐘️postgres/,🌐️neo4j/}🦀️.rs` | one-query space-summary + event-page reads, new indexes, per-backend RW-gate plumbing | `wp-h9.md:362-377` |
| `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` + its unit tests | (already compiled/committed for H10's residency-LRU redesign — confirmed by grep that `release_idle_guests` is gone; **this file is still `M` in git status**, so a further, more recent edit sits on top uncompiled — likely H9's Item L/C label/creation-progress prep touching the same schema) | `git status` this pass |
| `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` | shows as `MM` (staged AND further modified) — two different agents (H9 then H11) have queued edits to the same bin-law file | `git status` this pass |
| `🧰️…/🛢️db/{⚙️engine,📝️wal,🔄️sync,🔢️index,🗄️storage,🗄️storage/🌐️neo4j,🗄️storage/🐘️postgres,🗄️storage/🪶️sqlite,🗿️artifact}/🦀️.rs` | DB1's whole write-path rewrite (batched index maintenance, group-commit fsync, admission-wait, cursor-slot keying) — 9 files `M`, 2 new throughput-test files `A` | `git status`; `wp-db1.md:42-100` |
| `wp-h9/codemods/opaque-concurrency/*.py`, `land.sh` | **NOT in the source tree at all** — confirmed by `git status`: only the *ticket-folder scripts* are staged (`A`), the actual store/db source files they would patch show no opaque-concurrency-related diff. This item is fully unlanded, not just uncompiled | `git status`; `wp-h9.md:400-419` |

**Practical consequence for session 13**: every "DONE"/"FIXED"/"LIVE PASS" claim in `wp-h9.md`'s session-12 log dated
after ≈15:13 (roughly everything from "16:4x directory latency" onward) is **source-level work whose laws have never
executed against this exact code**, not a verified fact. H11 is already treating it this way (item 1); this audit
concurs that is the correct posture.

---

## 4. Ranked P0/P1/P2 (session 13)

### P0-1 — Land the `--packages all` publish (4th failure, now a wasm-codegen bug)
**Scope.** `🔌️plugin/🌐️browser-bundle/📜️script.ts:317`, `buildClosedBrowserActorArtifactOwned`: jco `generate` returns an
import outside the admitted list for the imperative plugin's browser actor, package 16/34.
**Acceptance.** `--packages all` publish reaches rc=0; 7800 (or a fresh hold) restarts onto it; open-plan probe passes for
every creatable kind.
**Owner.** W3 (already the sole owner per session-13 preamble rule 2/4; no report filed yet — first thing to check on it).

### P0-2 — Compile and re-verify every uncompiled hub/db edit before trusting any of it
**Scope.** §3 above — the directory-latency/RW-gate/revocation-fence/GIS-approval fixes and DB1's entire write-path
rewrite have not been through a single green build since being written. Any live collaboration, MCP, or frontend proof
that touches directory listing, document sockets under load, agent revocation, or db throughput is currently resting on
unverified source.
**Acceptance.** `cargo check -p semio-hub --all-features --tests` + full hub/db nextest green on the current tree;
H9's specific new laws (`a_command_is_answered_when_the_database_refuses_writes`,
`a_withdrawn_delegation_admits_no_agent_edit_after_it_in_either_order`, the directory-page laws) all pass.
**Owner.** H11 (item 1, already IN PROGRESS — `wp-h11.md:13`).

### P0-3 — Fix the db reopen/greeting-storm bound (many-document restart)
**Scope.** H9's g17: 24 grown documents reopening at once after a restart missed the 30 s Welcome bound; root-caused to
db write/replay throughput (index-merge-dominated commits, one worker turn per replayed envelope) — `wp-h9.md:420-437`.
DB1's own two throughput laws (storm ratio) are still red as of session-12 close.
**Acceptance.** DB1's item 3/4 acceptance: the 24×128-edit storm + reopen-storm laws pass on fs/sqlite AND postgres/neo4j,
matching or beating H9's g17 numbers.
**Owner.** DB1 (items 1/3/4, currently TODO/in progress — `wp-db1.md:16-19`).

### P1-1 — Land the creation-latency fixes (Q1 interpreter + guest codec-app resolution)
**Scope.** First-use creation of large packages still costs minutes (`3d.puzzle` 422 s measured on B2, `wp-w2.md:148-149`);
both fixes are dry-run clean and ready (`wp-h10.md:36-43`).
**Acceptance.** Land via `wp-h10/q1-land.sh` + `codec-app-resolution.py --apply`; re-measure `3d.puzzle` first-creation
time on the current catalog; expect roughly 6-15× improvement per H10's own estimate.
**Owner.** LA (items 4a-4c, `pending` — `wp-la.md`).

### P1-2 — Trusted-catalog residency LRU eviction is untested at the scale that matters
**Scope.** The 256 MiB budget has never actually been exceeded (B2 = 240.6 MB); the 34-package catalog is the first real
test and keeps failing before it gets there (see P0-1).
**Acceptance.** Once `--packages all` publishes, measure RSS through the full open-plan probe over every kind and confirm
at least one real LRU eviction+recompile cycle happens cleanly (no refusal, bounded latency).
**Owner.** **NEW** (falls out naturally once W3 lands P0-1; H11/LA are adjacent but neither has explicitly claimed the
post-publish residency measurement — flag to the coordinator).

### P1-3 — Kind-label "Editor" defect + creation-phase progress, still both unlanded
**Scope.** Every kind's creation-catalog label is the app's own label (always "Editor"); multi-minute creations show no
progress. Both patches ready (`wp-h9.md:146-152,221-229`).
**Acceptance.** Distinct kinds get distinct localized labels (en+de); a creation in progress reports stage/units.
**Owner.** LC (items 4/5, `queued` — `wp-lc.md`).

### P1-4 — Cold `docker build` for the hub image has still never completed
**Scope.** Confirmed this pass: zero hub images on the host. H10 reached build attempt 6 before being stopped for
build-quiet.
**Acceptance.** `docker build -f 🌎️hub/Dockerfile -t semio/os-hub .` succeeds; `docker run` answers `/healthz`.
**Owner.** Z3 (scope includes "Docker hub" per `📓️fleet-13-agents.md`; no report filed yet).

### P1-5 — Linux winit-backend patch (B4) still not landed; blocks every native Linux hub build
**Scope.** `winit`'s `compile_error!` on Linux stops `cargo check -p semio-hub` natively and in the devcontainer; proven
fixed in a container copy (`wp-z2.md:91-95`) but the patch was never applied to the actual tree, despite W2's Hub Handoff
(the stated landing gate) having happened many hours ago (03:59).
**Acceptance.** `wp-z2/pending/winit-linux-backends.py --apply` on the real tree; `cargo check -p semio-hub` clean on
macOS (regression check) and re-proven on native Linux.
**Owner.** Z3.

### P2-1 — Opaque-concurrency (vigilant same-field conflict detection) is fully unlanded
**Scope.** Confirmed by `git status`: only the ticket-folder script exists; no store/db source carries the change.
**Acceptance.** `land.sh` applied; store causal-head stamping + db `mutation.clamped` grading laws pass.
**Owner.** LD (no report filed yet — `wp-ld.md` does not exist despite being listed in the fleet roster).

### P2-2 — In-process `cargo test -p …-db --lib` flake, root-caused, not fixed
**Scope.** Backend-control exhaustion at `MemoryStorage::new` under the shared test pool.
**Owner.** H11 (item 3, TODO).

### P2-3 — General hostile-input/fuzz coverage beyond enumerated vectors, unchanged for 3 sessions
**Owner.** **NEW/unowned.**

### P2-4 — Windows hub boot, zero live evidence; devcontainer on hold
**Scope.** Static-only Windows coverage; devcontainer blocked by P1-5 plus a macOS Docker-Desktop `~/Documents` permission
gap that no agent can or should change.
**Owner.** Z3.

---

## 5. Honest gaps in this audit

1. Did not open `wp-c10.md`, `wp-s15.md`, `wp-g10.md` in full this pass (out of the assigned reading list); §2's presence
   and directory-fanout framing relies on `wp-h9.md`'s own citations of them, not a fresh independent read.
2. Did not run any command that touches a build, a hub process I did not already find running, or any test — per the
   auditor mandate. Every "uncompiled"/"unverified" claim in §3 is inferred from `git status` (staged vs. committed) plus
   the session-13 slices' own opening logs, not from attempting a compile myself.
3. `wp-z2.md` has no `## Session 12` section at all (confirmed by header scan) — Z2's cross-platform work is entirely
   session-11-dated and was never picked up again in session 12; this audit treats it as still-current or still-open in
   §1/§2 exactly as the S12 audit did, since nothing has touched it since.
4. Session-13 slices H11/DB1/C11/S16/T13/LA/LB/LC are all still at their opening (19:0x-19:1x) log line; this audit could
   not observe any session-13 measurement and instead cross-checked session-12 claims against the current tree directly.
   Re-read this file's §1/§3 against fresh slice reports before trusting it more than a few hours from now.
5. Did not independently verify the wp-w2.md publish-failure line beyond citing the session-13 preamble and W2's own log;
   `wp-w3.md` (the actual owner's report) does not exist yet, so the imperative-codegen bug's root cause is second-hand.
