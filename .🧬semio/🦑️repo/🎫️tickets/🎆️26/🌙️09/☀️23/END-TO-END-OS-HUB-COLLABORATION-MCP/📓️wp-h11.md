# WP-H11 — Hub Backend Correctness (successor of H9)

Slice: H11 (session 13, 2026-09-26 19:0x). Coordinator = main chat. Ports: hubs 8010–8019, serves 6510–6519.
Private cargo target: `.tmp-ticket/wp-h11/target`. Captures: `wp-h11/generated/` (expendable). Durable data/logs:
`.🧬semio/🌐hub/s13-h11-*`. Handovers: [📓️wp-h9.md](📓️wp-h9.md) (Session 12 table + log), [📓️wp-h10.md](📓️wp-h10.md),
[📓️audit-s12-hub.md](📓️audit-s12-hub.md). Neighbours (not duplicated here): LC (H9 labels + creation-progress sets), LD (H9
opaque-concurrency), DB1 (db throughput).

## Session 13

| # | Item | Status |
|---|------|--------|
| 1 | Compile + run the laws of H9's uncompiled session-12 hub fixes (directory lists/event pages, RW gates, pg one-query lists, `inference_approve` answer, revocation fence both orders); `semio-hub --all-features`; `os-hub:test-quick`; sqlite + postgres + neo4j | **DONE**: H9's laws PASS (named bin 23/23); full **`semio-hub --all-features` 430/431** (1 = chain-generated descriptors); postgres: directory lanes 12/12 + db round trips 3/3 (pg one-statement lists) + fence; neo4j: directory lanes 7/7 + fence — all on the shared claimed servers; test-quick ⊂ the full run |
| 2 | Current-tree `os-hub` on catalog B2 (own port, fresh root): directory latency 80+ spaces, multi-document growth, restart same root, backup/restore live drill | PARTIAL — directory **85 spaces: list best 5.8 ms / median 11.6 ms** (7800: 84 s), event pages ≤ 275 ms; restart same root: SIGTERM → exit 1.07 s `database=closed`, ready 22.2 s. Growth + backup/restore wait for a post-LD binary (tree TS wire ≠ 19:07 binary) |
| 3 | db in-process gate flake (backend-control exhaustion at `MemoryStorage::new`) → root fix + law | **DONE**: bounded pool-clock admission waits, every storage facade opens through `open_db_io_backend_admitted`; law PASS; in-process `--lib` gate 5 runs: 0 capacity failures (776/776 ×3, 775/776 ×1 = an intermittent store Drop witness, LD) |
| 4 | S15 B2 kind-sweep hub reds (block2d/wfc2d/grid2d creation/open stuck, 2d.puzzle "accepted" forever) → reproduce, root-fix, law | DONE live on the current tree: 2d.block 31 s, 2d.wfc2d 222 s, 2d.wfcgrid2d 294 s, **2d.puzzle 749 s, 3d.puzzle 579 s** — all created, planned, all 4 execution-target assets 200 (the component 503s were 7800's old route). Puzzle = CPU-bound guest genesis with no visible stage → speed H12 (LA Q1/codec-app), stage LC |
| 5 | G10 quartet hub reds (`inference_approve` PLUGIN_UNAVAILABLE while ledger advanced; hub tools failing after a directory event) → verify or fix | Hub side law-green on the current tree: `a_committed_gis_map_approval_is_answered_with_its_receipt` PASS (no post-commit directory re-check), RW-gate laws PASS (agent execution-target after its own commit, S12 tc23/tc24 measured 3–10 ms). Live quartet = G11's battery once 7800 runs B3 + a current gateway (agreed, no duplicate) |
| F | Coordinator (audit-s13-os-frontend §4.17): forged/malformed/revoked bearer on optional-bearer routes → 401 + localized problem body | **DONE**: `HubCredentialRefusalV1` (en/de) + RFC 6750 challenge on the 3 credential-optional routes; bin law PASS (fixture-driven), Ajv oracle 5/5 PASS |
| P | Coordinator: pg/neo4j live laws onto the shared claimed server (rule 22) | **DONE**: one claim helper (`db_storage::claimed_backend`, db feature `claimed-backend-laws` for the hub's dev-deps) + `SEMIO_BACKEND_CLIENT`; db: pg round trips 3/3 (incl. new concurrent-bootstrap law), pg + neo4j fence lanes PASS; hub: directory lanes pg 12/12, neo4j 7/7 via `directory-live-lanes` under claims (were 25 private containers per suite run); real concurrent schema-bootstrap race root-fixed; launch rows added; nx target for `postgres-round-trips-live` after PUBLISH DONE |
| C | Coordinator: 17 red hub `artifact_authority::` laws (LA) | **ROOT-FIXED**: G11's CHANNEL_VERSION 17→18 had stopped at Rust; host copies (directory schema JSON + TS, hub inference schema + TS, 11 fixtures) at 18, version-dependent digests + the frozen GIS inference identity chain re-derived by independent Python oracles; laws green except the chain-regenerated descriptors; `stdio-gis-bootstrap` generation → G11's single-source follow-up |
| R | C11 relay: directory presence republished on every pointer move | **DONE**: projection-dedupe under the presence gate; law PASS |
| Qa | H9 Qa db half (opaque concurrency) — only if LD asks | ON REQUEST (LD; H9's Qa superseded by LD's design) |
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
- 21:2x–21:3x rest of `probe-kinds-2.txt` (same hub, same load): **2d.wfcgrid2d 293.5 s**, **3d.puzzle 579.3 s** — both created, planned
  and all four execution-target assets 200 (puzzle component 28.3 MB 1.4 s). Probe verdict `DONE kinds=5 failed=0`. Then cut by
  the account usage limit at ~21:30; every process died overnight (preamble rule 28), incl. hub 8010 and check-allf-3.

### Resume 2026-09-27 04:5x
- Reconciled: every edit of mine is in the tree (auto-commit 653, 22:00): refusal schema/Rust/unit law/fixture, bootstrap
  `CredentialOptionalRefusalV1`, bin law, Ajv case, db admission + open helper + law. Nothing half-written.
- 04:58 check-allf-4 (build-fleet-b, detached via `w2-detach.py`) died 05:02 on a peer's mid-edit of
  `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:625` (not mine; file rewritten 05:04). Retried 05:12 as check-allf-5 (pid 98745).
- 05:07 roll-call sent to main (nothing to land; all native).
- 05:0x **pg/neo4j live laws moved onto the ONE shared server (coordinator, rule 22)**: `db_storage::docker_server` (each law
  started its own `postgres:17-alpine`/`neo4j:5-community`) replaced by `db_storage::claimed_backend`
  (`🗄️storage/🧪️tests/🔬️claimed-backend/🦀️.rs`: the claim's `OS_HUB_*` environment + the server's own client);
  `os-hub-ts backend run` now also hands the command the claim's client argv in `SEMIO_BACKEND_CLIENT`
  (`HUB_BACKEND_CLIENT_ENV`; `psql` on the run database / `cypher-shell`). Converted: the two postgres round-trip laws (oracle
  now PostgreSQL's own `pg_stat_user_tables` scan counts in the run database, read once still for 12 s > the 10 s idle stats
  flush — the server log is shared, so `log_statement` counting no longer isolates a run) and the WAL-writer fence lanes
  (postgres: `pg_locks`/`pg_stat_activity` and `pg_terminate_backend` now filtered to `current_database()`, so another run's
  writer never counts; neo4j: claim credentials). `wal-writer-fence-live [sqlite|postgres|neo4j]` runs a server lane only
  under a claim (no lane → sqlite + every claimed lane present); launch rows `⚖️gate🔐️wal-writer-fence🪶️sqlite|🐘️postgres|🕸️neo4j`
  (411.15/411.151/411.152, the server rows wrapped in `os-hub-ts:backend-run`). Not yet compiled/run (db test build approved).
- **Rule breach, recorded:** while replacing the docker-server file I ran `git rm -q --cached` on
  `🗄️storage/🧪️tests/🔬️docker-server/🦀️.rs` (a modifying git command, index only; no work lost). The file is deleted in the
  tree as part of this conversion anyway, so the index now matches the intended state; no further git command was run.
- 05:12–05:32 check-allf-5 (build-fleet-b) stalled at 38 units, no rustc child 19 min → stopped (rule 25). 05:38 a db check started
  above the rustc bound was stopped at once (mine, rule 22).
- 05:5x **coordinator: os-hub red "Send is not general enough" (bootstrap 4298/4719/6498/11501)** — not reproduced on the current
  tree: `cargo check -p semio-hub --bins` in build-landing **EXIT 0** at 06:00 (8 m 51 s, `s13-h11-logs/check-send-1.txt`). The red
  capture is H12's `s13-h12-logs/check-8.txt` (05:35, `--tests`), a WIP snapshot that also carries H12's in-flight
  `GuestArtifactComponent::verify_rows` closure (`FnOnce not general enough` at 11501) and two E0609 in a bin-unit law. My storage
  edits date from 20:4x yesterday. `--bins --tests` in build-landing: 06:01 run stalled on locks (stopped 06:20); rerun 06:21
  (pid 53044) is recompiling from `semio-framework-hash` (a peer's hash landing).
- 06:2x **C11 relay (hub presence fanout), written**: `refresh_presence` republished the member-directory `Presence {actors}` to every
  member socket on every peer-byte change (pointer/camera/caret), although that projection (actor, user, surface, colour) did not
  change. The presence publication gate now holds `PresenceDirectoryProjections` (each document's last published projection);
  `publish_presence_delta` always sends the document roster and publishes the directory projection only when it differs (an
  emptied roster is published once, then forgotten → bounded). Law
  `a_peer_beat_reaches_the_document_roster_and_the_directory_projection_only_when_it_changes` (join 1+1, three ephemeral moves
  3+0, second join 1+1, leave 1+1, last leave 1+1 empty, rejoin 1+1). Compiles with check-send-3.
- 06:0x H12 asked for the puzzle genesis profile → sent (sample path, leaf frames: `execute_machine` 876, `step` 556, LEB `Decoder`
  211, malloc/realloc ~290, `ControlFrame` Vec grow/drop 72, `BTreeMap<ControlBounds>::get` 54 of ~2 200 busy samples).
- 06:3x–09:5x cut again (usage limit). check-send-3 never finished (EXIT 143 09:52, stopped by the coordinator's orphan sweep).

### Resume 2026-09-27 09:5x (REBUILD START, rule 30)
- Reconciled: the presence dedupe is complete in the tree (not half-applied). Refined: a document that never had a directory
  row publishes no directory projection at all (before: an empty `Presence {actors: []}` on every beat of rows without a
  document surface). Native cargo now only through `fleet-mutex.sh native h11 -- nice -n 15 …` in build-fleet-b
  (`h11-cargo.sh` takes `H11_NICE=0` then). `check -p semio-hub --bins --tests` queued (`check-hub-6.txt`).
- WG9 reverted its bin-unit pins 06:32 (no longer needs my `--tests` result).
- 09:57 W3's chain hub-prewarm (`os-hub:build-dev`, current tree incl. the presence dedupe) rc 0 (coordinator) — the os-hub bin compiles.
- 10:1x **coordinator: 17 red hub `artifact_authority::` laws (LA capture `s13-la-captures/hub-laws-5.txt`) → root causes:**
  1. **G11's CHANNEL_VERSION 17 → 18 bump (landed 05:0x) stopped at Rust**: every hand-written fixture and the TS/JSON side still
     pinned 17, so a fixture descriptor built with `os_spr::CHANNEL_VERSION` (18) no longer matched its trust record (15 laws:
     "decoded package descriptor identity does not exactly match its trust record" + their downstream asserts), and the creation law's
     `document-open-plan-v1` plan was Denied by `lease_fields_from_plan_v1`. Live impact after the rebuild: the browser (TS
     `DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 = 17`, directory JSON schema `const: 17` ×3) would refuse every rebuilt
     plan. **Fixed (host-only JSON/TS, no Rust/guest edit):** 17 → 18 in `📇️directory/🧬️schema/🔣️.json` (3 consts) + `🟦️.ts` const,
     `🌎️hub/💡️inference/🧬️schema` const + TS twin, and the fixtures two-package, stdio-gis-bootstrap, generation-stage,
     execution-target-lease, fresh-staging, local-hub, browser-document-open, document-open-plan, gis-map-frozen-binding. Digests
     that depend on the version were recomputed by an INDEPENDENT Python oracle (`wp-h11/channel18_oracle.py`: the document-open
     catalog encoding and the frozen-binding digest, each first proven equal to the fixture's current value at 17): plan catalog
     `expectedHex` + generation `7b3248e5…` → `97f393c7…` (2 sites), frozen binding `6343843a…` → `25a58ad6…` (+ the two fixtures that
     quote it: gis-map-proposal-approval, gis-inference-job). **Open:** `stdio-gis-bootstrap.profile.generationId` (0ca2d589…, 3 sites)
     is derived by the TS oracle `trustedBootstrapProfileEncoding` (incl. the channel) and could not be recomputed: its verb
     `trusted-stdio-gis-bundle-check --source` is red before it on a pre-existing, unrelated drift ("artifact kind formats fixture
     violates its owning scope contract", `🛂️manifest/🧫️fixtures/🗄️artifact-kind-formats.json`); no Rust law reads that fixture.
     Not touched: MCP `🏠️workspace/🧬️schema` `appChannelVersion const 15` (older drift, G11 told); the generated plugin descriptor
     JSONs (describe-all in the chain).
  2. `native_openable_provider::…linked_consumer_descriptors_bind_their_actual_compiled_stdio_dependency_and_catalog`: S17's exact
     pins made range forms (`*`, `^`, `~`, `>=`) undecodable, and the law `unwrap()`ed the fixture's negative rows. The contract is
     unchanged (a range is refused) — only the layer moved: the law now counts a decode-time refusal (`exact version`) as "not
     accepted", preview otherwise (fixture untouched).
  3. `every_committed_editor_that_edits_a_document_opens_a_kind_through_the_one_rule`: reads generated `🛂️.descriptor.semio`
     files (playbook procedural `^0.1.0`) → regenerated by the chain's describe-all, not a fixture.
  Laws re-run in my queued native hold (`artifact_authority::`, `inference::`, `refusal::` lib filters + the named bin laws).
- 10:14–10:20 **native hold 1** (build-fleet-b, `fleet-mutex native`): hub `--all-features` test build (first attempt of the bin test
  hit a peer's mid-edit `SplitStream` import at bootstrap 5679, fixed by its owner a minute later; the named-law step recompiled it
  green). **Named hub bin laws 23/23 PASS** (`s13-h11-logs/hold1-bin.txt`): H9's `a_member_of_many_spaces_reads_its_space_list_and_event_pages_within_the_page_budget`,
  `a_withdrawn_delegation_admits_no_agent_edit_after_it_in_either_order`, `a_committed_gis_map_approval_is_answered_with_its_receipt`,
  `a_command_in_flight_never_stalls_the_spaces_directory_commands_or_deliveries`, `every_command_is_answered_when_the_database_refuses_writes`,
  `revoking_a_delegation_closes_the_agents_open_document_socket_and_roster_row`, `an_agent_delegation_mints_a_session_that_works_until_it_is_revoked`,
  `startup_catalog_progress_is_visible_at_the_launcher_level`, both hostile-input laws, the GIS ingress law, 9 `document_open*`/`execution_target*`
  laws; mine: `credential_optional_routes_refuse_a_revoked_or_forged_session_instead_of_answering_anonymously` (item F) and
  `a_peer_beat_reaches_the_document_roster_and_the_directory_projection_only_when_it_changes` (presence). Hub lib
  (`refusal:: artifact_authority:: inference::`) **113/119** (`hold1-lib.txt`): the 15 channel reds are gone; left: 3 inference
  sqlite + 1 runtime law (frozen inference identity chain, below), the trusted-catalog `missing` provider case (→ H12), and the
  generated-descriptor law (chain). **db: test build compiles (my admission helper + claimed-backend conversions), 20/20**
  (`hold1-db-laws.txt`): the storage unit laws incl. **`a_backend_opened_while_every_backend_control_is_taken_waits_for_a_release_instead_of_refusing` PASS**
  (item 3) and the sqlite fence lane.
- 10:2x the frozen binding digest is quoted by the GIS inference identity chain (proposal/job/WAL-proof fixtures and the
  inference schema's frozen proposal const), so the channel bump rotates that frozen identity too. Re-derived with the TS
  oracle's formulas (`wp-h11/channel18_inference_cascade.py`, old chain proven equal first): identity `4ebcd528…`→`b36231ed…`, job
  `4da0cbcd…`→`4f454b84…`, mutation `f9c05f70…`→`02c8a31d…`, proposal hash, command bytes (equal-length ASCII substitution) and
  command hash; rewritten in 5 files (job, reconcile, WAL-proof fixtures; inference schema JSON + TS). The Rust ledger had
  independently computed the same identity `b36231ed…` and job `4f454b84…` in the red run — two implementations agree.
  All 9 oracle equations hold on the rewritten files. Re-run queued (hold 3: hub lib laws, db in-process `--lib` gate, pg/neo4j
  claimed laws).
- 10:31–10:33 **native hold 3**: hub lib `refusal:: artifact_authority:: inference::` **118/119** (`hold3-hub-lib.txt`) — all inference
  laws green on the re-derived identity chain, H12's rewritten `missing` provider law green (H12: a provider binding nothing
  falls back to the component's own, lazily verified answer — the old expectation was stale); the one red is the
  generated-descriptor law (chain's describe-all). **db in-process `--lib` gate: 770 passed, 5 failed** (`hold3-db-inprocess.txt`):
  no `backend control capacity exhausted` failure (the item-3 flake class) — the 5 are one deterministic cause, the db history
  envelope decoder (`🗿️artifact/🦀️.rs:3900`) not knowing LD's new trailing MutationEnvelope fields ("history envelope has
  trailing bytes"), also red as isolated processes → **LD told** (owner of the wire change). **Claimed shared servers (rule 22,
  first runs of the converted laws):** postgres fence lane **PASS**, neo4j fence lane **PASS** (24.4 s); both pg round-trip laws
  RED at connect: `duplicate key value violates unique constraint "pg_type_typname_nsp_index"` — two storages bootstrapping one
  fresh database at once (before, each law had a server of its own).
- 10:3x **root fix: concurrent PostgreSQL schema bootstrap** — `CREATE TABLE IF NOT EXISTS` is not safe against a concurrent
  creator (two hub processes, hub + db CLI, two laws on one fresh database: one is refused). `bootstrap_schema` now runs all DDL
  in one transaction under `pg_advisory_xact_lock(SCHEMA_BOOTSTRAP_LOCK)` (two-key form, a keyspace apart from the WAL writer
  locks). Law `concurrent_openers_of_a_fresh_database_all_find_its_schema` (16 concurrent `PostgresStorage::connect` on a fresh
  database). `claimed_backend::FreshPostgresDatabase` (created/dropped through the server's own client) gives the writer and
  bootstrap laws a database each, so the list law alone writes the run database whose statistics it reads. New verb
  `postgres-round-trips-live` in the framework-os `📜️script.ts` (claim required); its nx target + launch rows wait for PUBLISH
  DONE (rule 4: no `📋️project.json` edits during the rebuild) — recorded as pending. Hold 4 queued (pg round trips + pg fence,
  separate claims).
- 10:36–10:40 **native hold 4 (each under its own claim of the shared postgres):** `db_storage_postgres::round_trips::` **3/3 PASS**
  (`hold4-pg-round-trips.txt`, 91.8 s — the list law waits for PostgreSQL's statistics to settle): a list is one scan however many
  rows, 24 concurrent writers all admitted, **16 concurrent openers of a fresh database all find its schema** (the race that refused
  2 of 3 openers in hold 3 is gone); postgres fence lane **PASS** again (`hold4-pg-fence.txt`). Rule 22 conversion complete: no
  db law starts a server of its own any more.
- 10:40 hold 5 queued: the full `cargo test -p semio-hub --all-features` suite.
- 10:46–10:48 **native hold 5: full `cargo test -p semio-hub --all-features`** (`hold5-hub-all-features.txt`): lib **260/261**, bin
  **168/170** → **428/431** (last known 375 laws; the suite grew). Reds: (1) lib `every_committed_editor_that_edits_a_document_opens_a_kind_through_the_one_rule`
  reads the generated `🛂️.descriptor.semio` files (chain's describe-all regenerates them); (2)+(3) bin
  `launcher_close_cancels_the_startup_catalog_load_instead_of_retrying_it` and `quick::native_openable_stdio_provider_is_the_only_atomic_readiness_transition`
  share one fixture line: LB's open-kinds rename (`stdio.json` → `s.stdio.json`) left the asserted viewer id
  `"stdio.json@rfc8259/*#viewer"` in `bin-unit/🦀️.rs:439` — fixed (one obvious line, rule 14; the builder already said
  `s.stdio.json`). Re-run queued (hold 6).
- 10:55 hold 6: both stdio open-kind bin laws **PASS** (`hold6-stdio-open-kind.txt`). **`semio-hub --all-features`: 430/431**; the one
  red reads chain-generated descriptors. `os-hub:test-quick` was not run separately: its `quick` level is a subset of the full
  run above (default features instead of all).
- 10:5x LD fixed the db history envelope walk for its new fields (their report). 11:06–11:07 **native hold 7: db in-process `--lib`
  gate ×4** (the gate H9 saw flake in 4 of 6 runs): **776/776, 776/776, 775/776, 776/776** (`hold7-db-inprocess-{1..4}.txt`);
  **0 `capacity exhausted` failures in all 5 runs** since the item-3 fix (hold 3 + hold 7). The one red (run 3) is
  `artifact_history_empty_and_two_batch_replay_are_deterministic` in its isolated child: a store Drop-witness assert
  (`🏪️store/🦀️.rs:19261`), intermittent, same as LD observed — sent to LD.
- 11:1x **independent TS oracles over every fixture I rewrote (no cargo):** `gis-map-frozen-binding-check` **52 checks clean**;
  `open-plan-check --source` **passed** (catalog encoding/generation, descriptor digest, receipt, issuer/consume/exchange corpus);
  `gis-inference-ledger-oracle` **rc 0** (`s13-h11-logs/ledger-oracle-3.txt`: identity chain `hashes=6`, reconcile, WAL proof,
  command, approval request, catalog projection, trusted-catalog identity, lease corpus). Getting it green also needed two
  pre-existing hub-script drifts fixed: (1) `🌍️gis-v1` selection cases carry `bindings` (since 09-21) that
  `NativeCodecProviderSelectionCaseV1` did not declare → declared (schema-first: integer 0–4096, required) and the TS oracle
  now derives the new "unlinked package is accepted with 0 bindings" rule independently (exact=1, unlinked=1, binding counts
  checked); (2) `proveNativeDeficitFixture` imported `toolJobCooperativeMaintenanceSelfTests` from the root `📜️script.ts`,
  which no longer exports it → imported from its owning test module.
- G11 has since landed the single-source channel-version authority (`🧑‍💻dev/🔖️channel-version`: pin → generate/check/census over
  every consumer, incl. ones outside my list: browser-bundle action-handoff/describe contracts, MCP hub-live-catalog, a
  registry test; it records the three contracts still at 15). Channel follow-ups are G11's from here.
- 11:1x **hub directory live lanes off private containers (rule 22).** Found while reading hold 5: `cargo test --all-features` ran the
  18 `directory::postgres` / 7 `directory::neo4j` laws, each starting its OWN `postgres:17-alpine` / `neo4j:5-community`
  container (they passed; they should not have started). Now: the db crate's `db_storage::claimed_backend` is the ONE claim
  helper (public behind the db feature `claimed-backend-laws`, enabled only in the hub's `[dev-dependencies]` — no twin); the
  postgres fixture gives each law a fresh database on the claim (`PostgresFixture`, dropped with the guard), the neo4j fixture
  empties the claimed graph per law (`Neo4jFixture`, claim credentials; the format-stamp laws no longer hard-code
  `neo4j/semio-test`); all 25 lane laws are `#[ignore]`d outside a claim; `directory-live-lanes [postgres|neo4j]` requires the
  claim (neo4j one law at a time); launch rows `⚖️gate🗂️directory-live-lanes🐘️postgres|🕸️neo4j` (411.153/.154) wrap it in
  `os-hub-ts:backend-run` (the generated `▶️directory-live-lanes` row stays and now tells the operator to claim). First run
  queued (hold 8). Consequence for the full-suite count above: those 25 lanes are now ignored in a plain `--all-features` run and
  proven by the verb instead.
- 11:15–11:18 **native hold 8: `directory-live-lanes` through the verb, each lane under its own claim** (`hold8-directory-lanes.txt`):
  **postgres 12/12 PASS** (9.3 s; each law in its own database on the shared server), **neo4j 7/7 PASS** (19.9 s, one at a time over
  the emptied claimed graph) — the same counts as H9's private-container runs, now with zero extra servers.

### Open, waiting for 7800 on B3 (commands ready)
- Item 2 rest, on the chain's current-tree `os-hub` + a fresh copy of `.🧬semio/🌐hub/s13-w3-catalog-b3`:
  `OS_HUB_BINARY=<build-dev os-hub> bun nx run os-hub-ts:backup-restore-drill -- --catalog-root .🧬semio/🌐hub/s13-w3-catalog-b3 --kind note`
  (V1's permanent drill), and `document-growth-e2e sqlite` (then postgres/neo4j under `backend run`).
- Item 4 re-measure (puzzle 2d/3d creation after LA's Q1 + codec-app resolution) → H12 owns the latency; `wp-h11/h11-open-plan-probe.ts`.
- Item 5: G11's battery (quartet rows 9/14/16/17) on the rebuilt hub + gateway; hub-side reds come back to me.
- After PUBLISH DONE (rule 4): nx target `postgres-round-trips-live` in `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json` + its launch rows.

### Session 13 Files (H11)
- Hub: `🌎️hub/🚧️refusal/{🦀️.rs, 🧬️schema/🔣️.json, 🧪️tests/🔬️unit/🦀️.rs, 🧫️fixtures/🪪️credential-refusal-v1/🔣️.json}`;
  `🌎️hub/🏗️bootstrap/🦀️.rs` (`CredentialOptionalRefusalV1`, `PresenceDirectoryProjections`); `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`
  (credential law, presence law, `s.stdio.json` id); `🌎️hub/🧪️tests/🚧️hostile-input/🟦️.ts`; `🌎️hub/📇️directory/{🐘️postgres,🌐️neo4j}/{🧪️tests/🔬️unit,🌱️creation-v1/🧪️tests/🔬️standalone}/🦀️.rs`
  (claimed fixtures); `🌎️hub/📦️packages/🦀️rust/{Cargo.toml, 📜️script.ts}` (dev-dep feature; `directory-live-lanes`; selection oracle;
  self-test import); `🌎️hub/📦️packages/🟦️typescript/📜️script.ts` (`HUB_BACKEND_CLIENT_ENV`);
  `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/{🧪️tests/🔬️unit/🦀️.rs, 🧬️schema/🔣️.json}`; `🌎️hub/💡️inference/🧬️schema/{🔣️.json, 🟦️.ts}`;
  fixtures `🌎️hub/🧫️fixtures/{🧊️gis-map-frozen-binding-v1, 🗳️gis-map-proposal-approval-v1, 🗺️gis-inference-job-v1, 🧭️inference-job-reconcile-v1, 🧾️inference-wal-proof-v1}`,
  `🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1`, `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/{👥️two-package, 🧬️stdio-gis-bootstrap, 🧱️generation-stage}`.
- db: `🛢️db/🗄️storage/{🦀️.rs, 🪶️sqlite/🦀️.rs, 🐘️postgres/🦀️.rs, 🌐️neo4j/🦀️.rs}` (admission waits, open helper, schema-bootstrap lock),
  `🛢️db/🗄️storage/🧪️tests/{🔬️unit/🦀️.rs, 🔬️claimed-backend/🦀️.rs (new; replaces 🔬️docker-server, deleted)}`,
  `🛢️db/🗄️storage/🐘️postgres/🧪️tests/🔬️round-trips/🦀️.rs`, `🛢️db/🗄️storage/🔐️writer/🧪️tests/🔬️fence-conformance/🦀️.rs`, `🛢️db/📦️packages/🦀️rust/Cargo.toml`.
- Framework host-only: `📇️directory/🧬️schema/{🔣️.json, 🟦️.ts}` (channel 18), `💻️os/🧫️fixtures/📇️directory/{🧭️document-open-plan-v1, 🌐️browser-document-open-v1}.json`,
  `🔌️plugin/🖨️describe/🧫️fixtures/🧊️fresh-staging/🔣️.json`, `🧑‍💻dev/🧫️fixtures/🚀️local-hub.json`, `💻️os/📦️packages/🦀️rust/📜️script.ts`
  (`wal-writer-fence-live` lanes, `postgres-round-trips-live`).
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`: fence rows ×3, directory-live-lanes rows ×2.
- Ticket: `wp-h11/{h11-cargo.sh, h11-hub.sh, h11-hub-laws.sh, h11-native-hold*.sh, h11-open-plan-probe.ts, h11-directory-probe.ts,
  h11-restart-backup-drill.sh, channel18_oracle.py, channel18_inference_cascade.py, readyz-watch.sh, rss-sampler.sh}`.

