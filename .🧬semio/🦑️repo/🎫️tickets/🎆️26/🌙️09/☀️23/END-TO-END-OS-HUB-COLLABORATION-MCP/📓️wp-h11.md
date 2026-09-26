# WP-H11 — Hub Backend Correctness (successor of H9)

Slice: H11 (session 13, 2026-09-26 19:0x). Coordinator = main chat. Ports: hubs 8010–8019, serves 6510–6519.
Private cargo target: `.tmp-ticket/wp-h11/target`. Captures: `wp-h11/generated/` (expendable). Durable data/logs:
`.🧬semio/🌐hub/s13-h11-*`. Handovers: [📓️wp-h9.md](📓️wp-h9.md) (Session 12 table + log), [📓️wp-h10.md](📓️wp-h10.md),
[📓️audit-s12-hub.md](📓️audit-s12-hub.md). Neighbours (not duplicated here): LC (H9 labels + creation-progress sets), LD (H9
opaque-concurrency), DB1 (db throughput).

## Session 13

| # | Item | Status |
|---|------|--------|
| 1 | Compile + run the laws of H9's uncompiled session-12 hub fixes (directory lists/event pages, RW gates, pg one-query lists, `inference_approve` answer, revocation fence both orders); `semio-hub --all-features`; `os-hub:test-quick`; sqlite + postgres + neo4j | IN PROGRESS |
| 2 | Current-tree `os-hub` on catalog B2 (own port, fresh root): directory latency 80+ spaces, multi-document growth, restart same root, backup/restore live drill | TODO |
| 3 | db in-process gate flake (backend-control exhaustion at `MemoryStorage::new`) → root fix + law | TODO |
| 4 | S15 B2 kind-sweep hub reds (block2d/wfc2d/grid2d creation/open stuck, 2d.puzzle accepted forever) → reproduce, root-fix, law | TODO |
| 5 | G10 quartet hub reds (hub-bound `inference_approve` PLUGIN_UNAVAILABLE while ledger advanced; hub tools failing after a directory event) → verify on current tree or fix | TODO |
| F | Coordinator (audit-s13-os-frontend §4.17): a presented-but-invalid bearer on `/directory/spaces`, `/directory/events` (every optional-bearer route) → 401 with a localized problem body; only an absent credential is anonymous. Fix + law, after item 1 | TODO |
| Qa | H9 Qa db half (opaque concurrency) — only if LD asks | ON REQUEST (LD) |
| — | Former 6/7/8 (all-package boot/readiness, residency LRU at scale, generative hostile-input law) | MOVED to H12 (coordinator 19:3x) |

### Session 13 log

- 19:06 start. Read AGENTS.md, preambles 13 + 12, `📓️wp-h9.md`, `📓️wp-h10.md`, `📓️audit-s12-hub.md` (head), `📓️wp-s15.md`
  S12-3, `📓️wp-g10.md` S2/S11, `📓️wp-db1.md`. Load 23, 1 rustc, 111 GiB free, Docker 29.5.3 answers. Ports 8010–8013 /
  6510–6511 free. H9's session-12 hub edits sit staged in the index (bootstrap, directory ×4 backends, bin-unit laws,
  two-client fixture/schema/runner); no build has compiled them yet.
- 19:10 detached build `test -p semio-hub --all-features --no-run` (pid 16968, `s13-h11-logs/build-allf-1.txt`); load climbed
  23 → 87 (fleet landing builds). Found on the way: `MemoryStorage::new` already waits on `db_io_admission_released()` in
  the working tree (unrecorded author, uncommitted; H9's 18:2x plan) — no law pins it yet (item 3).
- 19:2x coordinator add-ons (a) residency LRU at scale → item 7, (b) generative hostile-input law → item 8 (after 1–5).
- 19:3x coordinator: forged-bearer 401 + localized problem body → item F; items 6/7/8 moved to the new slice H12.
- 19:35 build 1 (`--all-features --no-run`) RED on a peer's in-progress edit (LB open-kinds landing:
  `trusted-catalog/🧪️tests/🔬️unit/🦀️.rs:1494` E0502, lib test target); build 2 (`--bin os-hub` only) RED at 20:07 on another
  peer WIP (`semio_hub::observability` imported by bootstrap before the lib module existed; H12's). Neither is mine; the
  tree is moving under 4+ hub editors (LB, R9 auth routes, H12 observability, me).
- 19:47 **item 4 reproduction started without a build (preamble rule 20)**: C11's 19:07 current-tree `os-hub` copied to
  `.🧬semio/🌐hub/s13-h11-bin/os-hub-c11-1907` (re-signed); fresh root `s13-h11-hub-8010` = B2 clone (generation
  `f485bf7e…`) + users 1/2 + C11's guest-verification memory (same engine); hub **8010** pid 44499 (`h11-hub.sh`), `/readyz`
  200 after **93 s** at load ~90 (`s13-h11-logs/s13-h11-hub-8010-8010-1.log`). Note (rule 24): a binary built before 20:14
  may carry stale deps — this one is used only for the creation reproduction.
- 19:52–20:07 **2d.puzzle creation on 8010** (`h11-open-plan-probe.ts`, phase transitions logged): phase `accepted` for the
  whole 900 s probe bound, no `progress` field, then the probe gave up (`probe-puzzle2d-1.txt`). `sample`
  (`generated/sample-puzzle2d-1.txt`): the hub is NOT stuck — one blocking-pool thread runs
  `GuestArtifactCodecBinding::genesis` → `OwnedRuntime::codec_genesis_observed` (owned interpreter), every pool worker parked.
  But the whole hub got **21 s of CPU in 15 min** (5 % CPU, RSS 23 MB of a 138 MB footprint): swap was 24.3/25.6 GB at load
  88, so the single interpreting thread was starved and paged. H10 measured the same creation at 213 s wall at load ~35.
  So on the current tree the "accepted forever" red = a CPU-bound guest genesis (≈830 M instructions for `pack-schema-hash`
  alone, H10) that (a) reports no progress while it runs — the status stays `accepted` although the work is running — and
  (b) outlasts every client bound under fleet load. 7800's extra cause (H9: the 120 s idle release recompiling the guest)
  is gone from the tree (H10's LRU residency, no idle release). Fixes owned elsewhere: speed = LA 4a/4c (Q1 interpreter +
  codec-app resolution) → H12; visible stage/progress = LC (creation-progress set). Reported main 20:02 (swap).
- 20:1x rule 22 (memory budget) applied: one cargo at a time, start only below 10 rustc, `cargo check` first. Rule 24
  (stale fingerprints before 20:14): nothing of mine was recorded green before 20:14. `check -p semio-hub --all-features
  --tests` running (pid 62602, `s13-h11-logs/check-allf-1.txt`).
- 20:2x item F: the `401` itself was already in the tree (H10 session 12: `resolve_optional_bearer_user`, a presented
  bearer must authenticate), but body-free. Landed (not yet compiled, check running): schema-first
  `HubCredentialRefusalV1` + `HubCredentialRefusalMessageV1` (en/de const) in `🌎️hub/🚧️refusal/🧬️schema/🔣️.json`; Rust
  projection `semio_hub::refusal::{HUB_CREDENTIAL_REFUSAL, HUB_CREDENTIAL_REFUSAL_CHALLENGE, …}` + unit law
  `the_credential_refusal_is_the_declared_schema_body`; bootstrap `CredentialOptionalRefusalV1(StatusCode)`: the three
  credential-optional routes (`GET /directory/spaces`, `/directory/spaces/{id}`, `/directory/events`) answer every `401`
  with the typed body + `WWW-Authenticate: Bearer error="invalid_token"` (RFC 6750 §3.1); other statuses stay bare.
  Language-agnostic fixture `🌎️hub/🚧️refusal/🧫️fixtures/🪪️credential-refusal-v1/🔣️.json` (routes, vectors malformed /
  forged (well-formed, never issued) / revoked, exact answer, 4 near-miss bodies); the bin law
  `credential_optional_routes_refuse_a_revoked_or_forged_session_instead_of_answering_anonymously` now runs the fixture
  over all three routes. **Ajv oracle 5/5 PASS** (`🌎️hub/🧪️tests/🚧️hostile-input/🟦️.ts`, new case: the answer validates as
  `HubCredentialRefusalV1`, message = the schema const, en + de, every near miss refused, every `credential-optional` row of
  the hostile-input fixture is covered; `s13-h11-logs/ajv-credential-3.txt`). Client side (HubConnection `listSpaces`
  throws `hub.spaces.unavailable` on any non-200) not touched: frontend owner's.
- 20:2x vitest note: `vitest run --root 🌎️hub` scans the whole hub tree (hung > 5 min, stopped); the hub suites run with
  the absolute config `🌎️hub/🧪️tests/🎚️config/🟦️.ts` from `📦️packages/🟦️typescript` (as `runVitest` does).
- 20:4x rule 25: hub law runs need coordinator approval — asked (item 1 is critical path). `check -p semio-hub
  --all-features --tests` (pid 83785, `check-allf-2.txt`) started at 9 rustc.
- 20:4x **item 3 root fix (db, written, compiling with the hub check):** the in-tree `MemoryStorage::new` wait (unrecorded
  author) covered memory only, and — like H9's `submit_db_io_task_admitted` — its wait was unbounded: `DbIoAdmissionReleased`
  woke only on the next release, and the `DB_IO_ADMISSION_WAIT_MS` deadline was checked only after a wake, so a capacity that
  never frees hung the caller forever instead of answering its refusal (for the hub: a frame hanging until the 30 s socket
  deadline instead of the documented refusal). Now: `DbIoAdmissionReleased` carries the pool and a deadline on the pool
  clock (`callback_at`, as the sync-hello admission does); `submit_db_io_task_admitted` takes the clock of the task's
  backend (`db_io_backend_pool`); one generic `open_db_io_backend_admitted(pool, open)` opens EVERY storage facade
  (memory, fs, sqlite, postgres, neo4j — each keeps a single-attempt `open_once`/`open_owned_once`/`connect_once`/
  `connect_owned_once`). Law `db_storage::tests::a_backend_opened_while_every_backend_control_is_taken_waits_for_a_release_instead_of_refusing`
  (process-isolated): fills the controls with single attempts (refused at once when full = the pre-fix behaviour), an
  admitted open stays Pending, is served when one held backend closes, and with the capacity held answers the capacity
  refusal after the bounded wait (≥ ½ and < 3 × `DB_IO_ADMISSION_WAIT_MS`).
- 20:4x **item 2, directory latency (hub 8010, C11's 19:07 current-tree binary, sqlite):** user1 owns **85 spaces** (83 created
  via `/directory/commands`, p50 23 ms, max 209 ms). `GET /directory/spaces` (85 entries, 26.8 KB): **best 5.8 ms, median
  11.6 ms, worst 25.2 ms** of 5 (7800's 02:31 binary: 84 s for ~53 spaces, C11 19:13). `GET /directory/event-page/v1` from 0:
  2 pages / 173 events, 256 + 256 ms in the probe; per-request 24–275 ms (curl 23–75 ms; `/directory/events` 85 KB 8–13 ms)
  at load ~70 — inside H9's 400 ms law budget. List == event pages (85/85 spaces, 0 missing). Probe
  `wp-h11/h11-directory-probe.ts`, capture `s13-h11-logs/dir-probe-8010-1.txt`. Verdict: WG8/U5's 25–58 s is fixed on the
  current tree.
- 20:5x restart + backup/restore drill on 8010 running (`wp-h11/h11-restart-backup-drill.sh`, pid 89782,
  `s13-h11-logs/drill-8010-1.txt`, client = H10's `h10-backup-drill.ts`).
- 20:5x drill attempt 1 on 8010 failed client-side: the tree's TS replication codec already carries LD's in-flight
  MutationEnvelope wire change (`encodeEnvelope` → `writeVecStr(undefined)`), which the 19:07 binary does not speak → the
  restart/backup drills wait for a post-LD binary (V1's permanent `os-hub-ts:backup-restore-drill` will be used then).
  SIGTERM → exit **1069 ms** (`server.shutdown ok retained-sockets=0 database=closed`); **restart on the same root → `/readyz` 200
  in 22.2 s** (warm verification memory, load ~60). Hub 8010 stopped 20:5x (rule 22), restarted 21:01 for item 4.
- 21:00 check-allf-2 stalled 16 min without a rustc child (lock convoy, 20+ cargos) → stopped (rule 25). 21:22 check-allf-3
  started in the separate build dir `build-fleet-b` (rule 26; cold), pid 29397.
- 21:01–21:2x **item 4 live on the current tree (hub 8010, C11 19:07 binary, fresh B2 space; `probe-kinds-2.txt`)** — creation
  → open-plan → all four execution-target assets:
  | kind | creation (accepted → preparing → ready) | manifest | component | descriptor | browser-actor |
  |---|---|---|---|---|---|
  | 2d.block | 31.4 s | 200, 3 ms | 200, 17.7 MB, 759 ms | 200, 9 ms | 200, 5.8 MB, 410 ms |
  | 2d.puzzle | **749 s** (accepted 746 s; load 60 → 177, swap 22.4/23.5 GB) | 200, 8 ms | 200, 28.3 MB, 2.1 s | 200, 7 ms | 200, 8.9 MB, 586 ms |
  | 2d.wfc2d | 221.7 s | 200, 3 ms | 200, 23.9 MB, 2.8 s | 200, 4 ms | 200, 7.9 MB, 916 ms |
  S15's component 503s after a slow creation (block2d/wfc2d/grid2d on 7800) are gone (S12-3e streamed route, in the tree since
  11:22). 2d.puzzle is NOT stuck on the current tree: it completes; its genesis is CPU-bound (≈12 min under this fleet load,
  213 s at load 35 per H10) and reports no stage while `accepted`. Remaining fixes are owned: speed (LA 4a/4c → H12), stage
  (LC creation-progress). Client bound: S15's journey gives up at 180 s — too short for puzzle genesis under load.

