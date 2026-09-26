# WP-H9 — Hub Backend: Check In Command, Fallible Stores, Declared Authorization, db Gate, Live Trace Leg

Slice: H9 (session 11). Ports: hubs 8010–8019, serves 6510–6519. Private cargo target: `.tmp-ticket/wp-h9/target`. Captures: `wp-h9/generated/`.

## Status

| # | Item | Status |
|---|------|--------|
| 1 | Check In as a hub command (H8 items 1–7) | DONE (native + wasm32 compile): ABI leg, schema (JSON + Rust + TS + fixture, Ajv **2/2**), upload route deleted with every caller, hub job + routes, os React shell + native wgpu shell wiring (en + de). Hub laws **3/3 PASS** (advance + cold open, stale/unknown/foreign, idempotent/cancel/revoke), job laws **3/3**, `check-in-check source` **32 checks** (`generated/laws-run-7.txt`, `check-in-source-1.txt`). Process phase (approval after edit + check-in, `proveGisMapProposalProcess`) needs a current-tree catalog → waits on W2's `w2-catalog-all` |
| 1a | `gis_map_abandoned_pre_witness…` flake root cause | DONE: root cause = the Preflight commit turn was the only turn that did not yield, so the approval could race through assembly, journal and publication inside one manual poll; the law then saw the test publisher's injected attempt-0 `Storage`. Fix: `Preflight` yields like every other turn. Before: 1/1000 alone, 6/3000 under load. After: **0/3000** under load (see Item 1a) |
| 2 | Store writes that can fail (O2-5b) | DONE: verified already fixed (W3d); one silent caller fixed (agent-delegation revoke → `503 directory-unavailable`); law `credential_sign_in_reports_a_failing_instance_session_store` **PASS** |
| 3 | Declared authorization (O2-10) | DONE: one schema-first policy (`HubAccessPolicyV1`), one authority `hub_access_permits`, every hub gate routed through it; Rust laws **2/2** (113 vectors) + TS/Ajv **2/2**; removed member → 403 on the live job and `authority-changed` on the running Check In |
| 4 | db gates (in-process `--lib` + nextest) | PARTIAL: the retirement-hook lost wake was found and fixed (deterministic law). nextest **705/705**. Plain in-process runs still flake under load from a separate, pre-existing cause (backend-control retirement starvation on the shared test pool; see the Item g note), so the earlier "6/6 in-process" was not proof |
| 5 | Observability live leg (document socket open/close + presence in two-client e2e) | DONE: two-client e2e on my hub **:8010** (W2's binary, catalog A copy in `wp-h9/catalog-a`, own temp data root) **2/2 PASS** in 347 s; 27 trace lines Ajv-valid + declared; document socket open (`upgrade`, ok) / close (`closed`, cancelled) share `requestId r000000000011`; presence join 2 / leave 2 / expiry 1; `server.shutdown:ok` (`generated/two-client-sqlite-1.txt`, `two-client-sqlite-receipt.json`) |
| 6 | Gates: hub quick + long, os-hub-ts vitest, pg/neo4j | DONE: hub nextest `long` **346/346** (1 skipped), `quick` **337/337** (10 skipped), `os-hub-ts` vitest **19 passed / 2 skipped** (5 files), live WAL-writer fence sqlite/postgres/neo4j **3/3**, directory live lanes postgres **12/12**, neo4j **7/7**, corpus 6/6 (inside the all-features run) |
| a | P0-1 creation law on a real genesis-capable guest; DB4 document lane pg/neo4j | **P0-1 DONE**: `space_artifact_creation_routes_are_author_owned_idempotent_and_genesis_backed` now runs genesis on the GIS plugin's real release component built from this tree (`verified_gis_map_release_profile`; nx `test-all-features` depends on `@semio-tech/gis-plugin:component-release`) and **PASSES in-process on sqlite** (208 s, moved to `long`). P1-1 verified stale. DB4 document lane pg/neo4j: running in the growth e2e (Item g) |
| b | P2-2 `Database::shutdown` on every exit path; restart < 1 s on all backends | PARTIAL: code DONE (every exit after `connect_db` closes the database, see Item b). Live, db level: WAL-writer fence `release-admits-contender` + `process-exit-releases` **3/3 backends**. Live, hub level sqlite: SIGTERM→exit **298 ms**, close code 1012, `server.shutdown:ok database=closed`, reopen on the first attempt. Hub-level pg/neo4j restart probe needs a current-tree catalog (same block as a) |
| c | P2-1 `HubSagas` decision (real saga or delete) | DECIDED — keep the drain, empty saga set (see Item c) |
| g | Documents stay writable as they grow: ≥ 500 authored edits (map + note), hub restart, next edit accepted; per-operation-bounded DB I/O admission | IN PROGRESS: RW gates (C10 fanout root cause) law-green (os-hub bin 156/157, 09:0x); g14 (sqlite, 24 docs) refused an edit on a worker-queue lock race → one db retry policy (`db_policy::worker_submit_retry_attempt`, lock races spend no attempt) on every hot-path owner, db laws 59/59 (nextest); g15 sqlite running on the rebuilt binary; pg/neo4j next |
| d | P2-4 hub suite with postgres/neo4j features (live-database-lanes) full count | DONE: `cargo test -p semio-hub --all-features` full count **372/375** (lib 225/227, bin 147/148; Docker live lanes included). Fixed in this pass: approval retry 409 (Item d), trusted-publication crash law ENOTDIR (Item d). Remaining 3: the P0-1 creation law (blocked, row a) and 2 trusted-catalog laws that pass alone but race on the process-wide codec registry in-process (W2's area, already flagged in `wp-w2.md` §2.1) |
| e | Local-bootstrap pipe: 64 exchanges per run, then the hub exits (W2) | DONE (law + live): the replay set is bounded per 15 s window, and every refusal is a signed answer. Live on this tree's binary + catalog B: a 70-exchange burst in 145 ms → **64 issued, 6 `resource-limit`**; the hub stays up (`/readyz` 200) and issues again after the window. See Item e |
| f | A hub still loading its catalog ignores pipe EOF (W2) | DONE (law + live): a loading hub with pipe EOF → **exit 0 in 70 ms**, `server.shutdown cancelled launcher-closed-during-catalog-load database=unopened` (catalog B, this tree's binary). See Item e |

## Session 12

Session 12 (2026-09-25 22:5x →). Ports: hubs 8010–8019, serves 6510–6519. Data: `.🧬semio/🌐hub/s12-h9-*`. Captures: `wp-h9/generated/` (expendable).

| # | Item | Status |
|---|------|--------|
| g | Documents stay writable as they grow: ≥ 500 authored edits (map + note), hub restart, next edit accepted; per-operation-bounded DB I/O admission | IN PROGRESS: RW gates (C10 fanout root cause) law-green (os-hub bin 156/157, 09:0x); g14 (sqlite, 24 docs) refused an edit on a worker-queue lock race → one db retry policy (`db_policy::worker_submit_retry_attempt`, lock races spend no attempt) on every hot-path owner, db laws 59/59 (nextest); g15 sqlite running on the rebuilt binary; pg/neo4j next |
| R | Agent revocation (coordinator add-on G12-P2-1) | LAW PASS + **LIVE PASS** on the real binary (two-client e2e `tc18` sqlite: agent socket 4401, B's roster drops the agent, agent 401, revoked delegation 403 `delegation-revoked`, 3 ms); pg/neo4j running (`tc19`) |
| L | Per-kind localized labels (en + de) descriptor → hub creation catalog → both shells' pickers | PATCH SET READY (guest wire change; dry run clean; post-publish landing, W2 told) |
| M | Hub memory bound: bounded compiled-guest cache + idle release, RSS before/after (`footprint`) | LANDED, laws 2/2 PASS → **handed to H10** (RSS measurement) |
| b | P2-2 live: restart < 1 s on sqlite, postgres, neo4j (hub level), database closed on every exit | TODO |
| 4 | db in-process gate flake (backend-control retirement starvation) + `semio-hub --all-features` 375/375 | TODO |
| D | P2-3 cold `docker build` + `docker run` → `/healthz`, `/readyz` | `.dockerignore` fixed (context was 344 GB) → **handed to H10** |
| V | Schema-driven hostile-input law over every hub HTTP/WS route, Ajv oracle | LANDED: Rust laws 2/2 PASS (420 requests), Ajv 3/3 PASS → **handed to H10** |
| C | Creation-phase progress + bounded creation (S15 "stuck 2d.puzzle") | PATCH SET READY (os-kernel schema is guest-linked → rule 20): `wp-h9/codemods/creation-progress/`; cause of the stall = residency release → H10 |

### Handoff to H10 (04:5x, coordinator split: H10 owns hub performance + operations)

What H9 landed in H10's areas before the split (all compile-checked; laws as stated), for H10 to own from here:
- **Residency bound** (`TrustedCatalogGuestResidencyV1`, `GuestResidencyV1<T>` replacing the forever `OnceCell` in
  `GuestArtifactComponent`, `VerifiedTrustedCatalog::release_idle_guests`, hub `GuestResidencySupervisorV1`): laws
  `guest_residency_bounds_are_the_declared_schema_values`, `an_idle_compiled_guest_is_released_and_compiled_again_on_its_next_use`
  **PASS**. RSS before/after NOT measured cleanly (g12 sampler `s12-h9-logs/rss-g12.txt` ran under 9.7/10 GB swap: RSS 80–400 MB
  vs footprint 330–645 MB).
- **Content-addressed guest verification cache** (`GuestCodecVerificationV1` in the trusted-catalog schema, export 27;
  `GuestCodecVerificationCacheV1` at `<hub data>/guest-codec-verifications/<sha256(key)>.json`, key = component SHA-256 of
  the bytes re-read at load + artifact schema + engine = hub executable len/mtime/inode; written by catalog load AND by
  publication, so the first boot after a publish is warm): law `a_guest_codec_verification_is_recalled_only_for_its_exact_key`
  **PASS**. A warm restart skips the `pack-schema-hash` interpretation; the first-use COMPILE (`OwnedSemioArtifact::parse`,
  W2: 229–422 s first create) is not cached yet — that is H10's compile pipeline. Live warm-boot not measured.
- **Typed refusals + hostile input** (`HubRefusalV1`, `refusal_middleware`, fixture `🚧️hostile-input-v1`, laws
  `the_hostile_input_fixture_covers_every_registered_route` + `every_route_answers_hostile_input_with_a_typed_signed_refusal`
  **2/2 PASS** (420 requests), Ajv oracle **3/3 PASS**). Fixed on the way: `GET /🧩️extension-modules` answered 500 when the
  extension root is absent; `POST /directory/socket-grants` accepted a 3 MiB body. Open design question for H10:
  `/directory/spaces` and `/directory/events` answer a forged bearer as anonymous (200, public view) instead of 401.
- **Session mint** (landed at the split): PBKDF2 keys HMAC once (2 compressions
  per iteration, was 4); sign-in and credential-change derivations run on `spawn_blocking` (they blocked a tokio worker
  ~2 s each on a debug hub); an unknown account is refused after a full-cost derivation against
  `PasswordCredentialV1::absent()` (the refusal's timing no longer tells unknown accounts from wrong passwords); root
  `Cargo.toml` `[profile.dev.package.semio-framework-hash] opt-level = 3` (and `[profile.wasm-dev.package.semio-framework-hash]
  opt-level = 0`, so guest bytes do not change). Laws: auth lib 29/29 (incl. PBKDF2 + HMAC third-party vectors), credential/sign-in/delegation bin laws 18/18 PASS (`s12-h9-logs/laws-4.txt`). Not measured live.
- **Docker**: `.dockerignore` now excludes `.🧬semio` (344 GB of cargo cache, hub data, tickets — the context was larger
  than the disk), `.tmp*`, and nested `node_modules`/`target`/`📤️dist`. No `docker build` run.
- Tools H10 can reuse: `wp-h9/e2e-s12.sh` (scenario runner), `wp-h9/rss-sampler.sh`, `wp-h9/laws-s12.sh`,
  `wp-h9/lib-laws-s12.sh`, catalog copy `.🧬semio/🌐hub/s12-h9-catalog-b2`.

### Session 12 log
- 23:0x removed my session-11 `[DEBUG]` reserve instrumentation from `🛢️db/🗄️storage/🦀️.rs` (it captured a backtrace on
  EVERY DB I/O operation reserve — committed in HEAD 650). db nextest (incl. `long`) on the current tree: **706/708**; the 2
  failures were regressions of my session-11 unmount change:
  - `db compact` CLI answered `closed`: `Database::compact_document_retained` returned the compaction's actor ask from a
    temporary handle, so the handle dropped, the document unmounted and the mailbox refused the queued turn. Fixed at the
    type: `ArtifactHandle::compact_retained` returns `ArtifactCompactionFuture`, which holds a handle lease until the turn
    answers (the only handle method whose future did not already hold the authority).
  - `database_shutdown_cancellation_and_vcs_error_preserve_exact_retry_owners` asserted the old owner (`closing_authority`);
    an unmounted document now retires through its `Closing` slot. The law now pins that owner: the last handle leaves a
    `Closing` slot, a cancelled shutdown keeps it, a resumed shutdown drains it, then the VCS retry leg as before.
  - After both: db nextest **708/708** incl. both growth laws (`generated/s12-db-nextest-2.txt`, 535 s).
- 23:25–23:36 all-driver `os-hub` built from this tree (`.🧬semio/🌐hub/s12-h9-bin/os-hub`, 11 min at load 70).
- 23:3x new e2e scenario **`📈️document-growth`** (schema-first fixture `🌎️hub/🧫️fixtures/📈️document-growth-v1/🔣️.json`,
  runner `🌎️hub/🧪️tests/📈️document-growth/🟦️.ts`, nx `os-hub-ts:document-growth-e2e <backend>`, launch.json + seed
  `⚖️gate📈️document-growth🪶️sqlite|🐘️postgres|🕸️neo4j` 411.19–411.21): 24 documents (12 note + 12 map, genesis on the real
  guests through the creation catalog, created concurrently), all grown at once with chained edits (22 × 120 + 2 × 520),
  no refusal, last-window ack latency ≤ 3 × first-window, SIGTERM → exit 0 + `database=closed`, restart spawn < 1 s, every
  document reopens and accepts 3 more edits, no refusal marker in the hub output. The hub TS router's backend runner is now
  one `BackendE2eScript` (scenario + minimum level) for `two-client-e2e` and `document-growth-e2e`.
  - Run g1 (sqlite) was killed by the 900 s `long` budget during creation (declared minimum level is now `exhaustive`).
- 23:5x in-process `cargo test -p …-db --lib` (load 110): **707/708**; the growth law failed on REOPEN with
  `wal cursor deadline reached`: `ArtifactWal::open` verified the whole retained chain under ONE 30 s / 1 M-fuel budget, so
  a document whose history outgrows it (sooner on a loaded host) can never be opened again — the same defect the replay
  cursor had already been fixed for. **Fix:** `WalCursorControl::stall_bounded` + `renew_step`; `ArtifactWal::open`
  renews the bound per verified segment. **Law** `wal_open_bounds_each_verified_segment_not_the_whole_history` (deterministic,
  fuel-based: ≥ 8 segments; one fixed step budget refuses the chain, the same budget per segment opens it). WAL laws **41/41**.
- 00:0x–01:0x growth e2e runs g2–g9 on sqlite (all failed before growth, none from the db):
  - g2: the runner's module picker matched a new `🧰️framework/node_modules` (also broke the two-client e2e); both
    pickers now skip `node*`.
  - g4: creation POSTs beyond the hub's 8 concurrent creations are answered with a bare `503` (no typed body, no
    `Retry-After`) — noted for item V; the runner now keeps ≤ `bounds.concurrentCreations` (8) in flight.
  - g3/g5/g6/g8/g9: **boot readiness stall** — the launcher's 300 s no-progress bound killed a hub that was still loading
    its catalog. `sample` (`generated/g9-hub-sample.txt`): main thread in `TrustedCatalogLoader::verify_selected` →
    `codec_pack_schema_hash_observed` → the owned interpreter, computing (not blocked). **Root cause:** the startup
    control's in-flight progress records (the guest codec's fuel reports) are `started` → level `debug`, so at the
    launcher's `info` level the hub printed nothing while it worked. **Fix:** in-flight catalog progress is emitted at
    `info` (`StartupCatalogControl::report`); **law** `startup_catalog_progress_is_visible_at_the_launcher_level`
    (written; compiles; run pending with the next hub test build).
  - Contributing (fleet-wide, reported to the coordinator → preamble rule 17): zsh `BG_NICE` runs every
    `nohup … &` job at nice +5; my hubs/builds are now launched with `setopt no_bg_nice`.
- 01:37 hub laws **3/3 PASS** (`s12-h9-logs/laws-1.txt`): `startup_catalog_progress_is_visible_at_the_launcher_level`,
  `an_agent_delegation_mints_a_session_that_works_until_it_is_revoked`, and the new
  **`revoking_a_delegation_closes_the_agents_open_document_socket_and_roster_row`** (coordinator add-on G12-P2-1): an
  agent holding a document socket, a human on the same document; the human revokes the delegation → the agent's socket
  closes `4401`, the human's next presence frame no longer carries the agent's row, both within 5 s, the agent's next
  request is `401`, and a reconnect is refused. The audit's "session keeps working for up to 1 h" was stale: revocation
  already revoked every minted session with an authorization-generation bump and invalidated their socket bindings; the
  law now pins it. Live leg (MCP `action_invoke` refused) still to run.
- 01:4x **item M (hub memory bound)**, code landed (compile-checked, laws running):
  - schema-first bound `TrustedCatalogGuestResidencyV1 {idleReleaseMs: 120000, sweepIntervalMs: 15000}` in the
    trusted-catalog schema (export 26) + Rust projection `TRUSTED_CATALOG_GUEST_RESIDENCY`;
  - `GuestResidencyV1<T>` replaces the forever `OnceCell` of `GuestArtifactComponent`: one compile at a time, every
    codec call holds its own `Arc`, `release_if_idle` drops the resident copy only when unheld and idle ≥ bound; the next
    call compiles again;
  - `VerifiedTrustedCatalog::release_idle_guests` + hub `GuestResidencySupervisorV1` (sweep per interval, one
    `server.artifact.maintenance guests-released=N guests-resident=M` record when something was released, drained on
    shutdown);
  - laws `guest_residency_bounds_are_the_declared_schema_values`,
    `an_idle_compiled_guest_is_released_and_compiled_again_on_its_next_use` (running). RSS before/after: pending.
- 01:49 g10 (sqlite, nice 15 per rule 18, load 60–90): hub booted in ~10 min (catalog progress now visible), 23 of 24
  documents created, the 24th genesis failed `reached no checkpoint within its no-progress span` (180 s): a single
  uninterruptible step (the concurrent first compile of the ≈50 MB GIS guest, which reports no progress, on a debug hub
  at nice 15) outlasted the creation's stall bound. Runner now keeps 4 creations in flight; g11 running with the
  budget lifted (`SEMIO_TEST_BUDGET_MS`).
- 02:08 g11 (sqlite, catalog B, nice 15, load ~60): hub ready, **24/24 documents created** (4 in flight, 11–364 s),
  growth started (edits 0–19 acknowledged on several documents), then document `1-3` got no Ack for its 4th edit within
  60 s (no Error frame, no hub error/warn line) → run failed. Unknown whether a hub hang or load; re-running on a quiet
  machine with the residency binary and catalog **B2** (current tree, copied to `.🧬semio/🌐hub/s12-h9-catalog-b2`),
  sampling the hub if an Ack stalls.
- 02:1x–03:40 cut by the usage limit.
- 03:4x residency laws **2/2 PASS** + trusted-catalog schema export law (26 exports) PASS (`s12-h9-logs/lib-laws-1.txt`).
- 03:5x **item L (per-kind labels)**: guest descriptor wire change → prepared as a post-publish patch set (ABI freeze
  rule 1): `wp-h9/codemods/kind-label-patch.py`, **dry run clean** (184 `ArtifactKindSpec` literal sites in 93 files,
  6 exact hunks): `ArtifactKindSpec.name: String` → `label: LocalizedLabel` (en + de for every kind; format kinds and
  norm codes as data), the hub creation catalog, the local picker (`artifact_kind_choices`) and the host
  `OsArtifactDescriptor` present the kind's own label. W2 told via `wp-w1/requests/h9.txt`. Law for the landing window:
  distinct kinds of one catalog get distinct labels, each equal to its `ArtifactKindSpec.label` (Ajv over the creation
  catalog schema in the TS twin).
- 03:55–04:2x g12 (sqlite, **catalog B2**, residency binary 03:54, RSS sampler `s12-h9-logs/rss-g12.txt`): boot ~12 min
  at load ~40, creation running (15/24 at 04:14). Runner pid 38244, sampler pid 38246 (mine).
- 04:0x **growth fixture schema-first**: `🌎️hub/🧬️schema/📈️document-growth-v1/🔣️.json`, the runner validates the fixture
  with Ajv (and refuses `concurrentCreations` > 8, the hub's admission): ungated fixture test PASS.
- 04:1x **item V (hostile input)**, landed + compile-checked, laws running:
  - schema-first `HubRefusalV1` / `HubRefusalCodeV1` / `HubRefusalStatusCodesV1` (`🌎️hub/🚧️refusal/🧬️schema/🔣️.json`),
    Rust projection `semio_hub::refusal` + law `refusal_codes_are_the_declared_schema_table`;
  - router's outermost `refusal_middleware`: every answer ≥ 400 carries `x-semio-refusal: <code>`, a bodiless refusal
    answers the typed `HubRefusalV1` body, and the handler runs as its own task, so a panicking handler answers a typed
    `500 internal` (+ trace `server.request failed handler-panicked`, vocabulary 24 → 25) instead of dropping the
    connection; a full creation queue answers `503` with `Retry-After: 1`;
  - language-agnostic route inventory `🌎️hub/🧫️fixtures/🚧️hostile-input-v1/🔣️.json` (65 method/route rows, 8 vectors);
    Rust laws `the_hostile_input_fixture_covers_every_registered_route` (source scan of every `.route(`) and
    `every_route_answers_hostile_input_with_a_typed_signed_refusal` (≥ 400 requests: unauthenticated, forged bearer,
    malformed JSON, wrong schema, wrong content type, 3 MiB oversized, hostile path segments, wrong method);
  - third-party oracle `🌎️hub/🧪️tests/🚧️hostile-input/🟦️.ts` (Ajv): all 19 declared request schemas refuse the
    wrong-schema vector, every status-table refusal validates as `HubRefusalV1`, an undeclared code does not — **3/3 PASS**.
- 04:5x coordinator split (H10 takes performance + ops, see handoff above); H9 order now: (1) P0 growth, (2) C10 fanout
  race, (3) stuck 2d.puzzle creation, (4) agent revocation, (5) labels, (6) db flake + all-features.
- 04:4x–05:1x **root cause of the growth e2e's missing Ack (g11/g12) and of C10's directory fanout race — one defect:**
  every hub authority gate (`SocketBindingGatesV1`) was an EXCLUSIVE mutex, and every document socket's record bindings
  include its space (`DirectorySpaceAuthority`) — so every frame of every socket of a space, every directory delivery of
  that space and every directory command on it serialized on one lock, held across the db submit. A frame waiting
  > 2 s for its bindings (or a frame running > 2 s) closed the socket `1013 authorization-unavailable` with no Ack.
  Reproduced by the new law `every_command_is_answered_when_the_database_refuses_writes` (12 sockets, one session, one
  space → `CloseFrame 1013 authorization-unavailable`, `s12-h9-logs/laws-5.txt`).
  **Fix (landed, compile-checked; laws running `laws-6.txt`):**
  - gates are reader-writer: `SocketBindingModeV1::{Shared, Exclusive}`, `SocketBindingGuardV1`; every USE of an
    authority (socket frames, directory deliveries, open plans/grants/execution targets, socket admissions, creations,
    GIS approval ingress) holds it shared; only a CHANGE holds it exclusive: space-wide for visibility/archive/delete,
    the target's membership for member upsert/removal (email resolved to the user first), the redeemer's membership for
    invite redemption, the session for a session revocation, and an administrator intent's changed keys;
  - the per-document writer gate moved out of the authority keys (`document_write(scope)`, still a mutex);
  - a document-socket frame now has a declared 30 s `DOCUMENT_SOCKET_FRAME_DEADLINE` and closes `frame-deadline`
    instead of a 2 s budget reported as `authorization-unavailable`;
  - law `a_command_in_flight_never_stalls_the_spaces_directory_commands_or_deliveries` (deterministic: a member's
    command held in flight on the document writer; the owner's upsert of a newcomer answers 202 and the member's
    directory socket delivers the event, both < 2 s; then the held command is acknowledged).
- 05:4x gate laws **35/35 PASS** (`s12-h9-logs/laws-7.txt`): the two new laws, the hostile-input laws, every
  directory-command-authority / admin-intent / membership / revoke / socket-grant law. Refusals stay body-free (the
  existing "a denial carries no body" laws are the no-leak contract): `refusal_middleware` now only stamps the
  `x-semio-refusal` code; `HubRefusalV1` is the (status, code) pair a client observes. H10 owns V from here.
- 05:4x S15's "stuck" 2d.puzzle creation on 7800 = my residency bound: 7800's capture shows `guests-released=N` every
  sweep; a guest idle 120 s is dropped and the next creation recompiles for minutes with no progress shown. Told the
  coordinator (H10 owns residency; not edited by me to avoid a double edit). Creation-phase progress is my next item.
- 04:5x found on the way (db, pathmap documents only): the generic `db.pathmap.v1` document keeps every path value in
  its own DB I/O operation in a fixed 64-entry map (`RetainedStateMap`, `RETAINED_STATE_ENTRIES`), so a pathmap document
  refuses its 65th distinct path (`LimitExceeded("retained state entries")`, law
  `a_document_keeps_accepting_edits_as_its_paths_multiply` FAILS) and ~24 concurrent pathmap documents exhaust the
  process ledger (`concurrently_grown_documents_each_answer_every_edit` FAILS: `DB I/O process aggregate credit
  exhausted`, 119 of 128 ledger slots held by 1-page resident values). Real plugin edits carry their own schema, which
  the db never interprets (`diff_entries` is empty for them), so they hold no resident values; the growth e2e now sends
  exactly such edits (fixture `diffSchema: "artifact"`). The pathmap resident-state redesign (values off the I/O
  ledger, no fixed path cap) is recorded as open, owned by H9.
  - Found, not yet fixed (next bound): the db never snapshots or compacts on its own (`SnapshotPolicy` has no non-test
    caller), so a document's retained WAL, its open verification and its replay (`engine.applied` holds every historical
    envelope) all grow with its whole history. A compaction horizon has to respect the hub's Check In ledger (the WAL
    tail since the active checkpoint), so the horizon belongs to the hub checkpoint.

- 22:5x resumed. Reconstructed the unrecorded end of session 11 (17:37–18:22) from `generated/`: (1) the 17:37 growth e2e
  (binary 17:37) failed on sqlite (`a-restart missing growth Ack 317`), postgres (`a missing growth Ack 79`) and neo4j
  (growth + restart passed, then `a-crash-0 missing Welcome` after SIGKILL; reopened welcome after SIGTERM took 187 s);
  (2) S15 finding a (`DB I/O process aggregate credit exhausted` after ~20 documents) reproduced by the db law
  `a_database_keeps_opening_documents_long_after_the_first_twenty` and fixed by two changes now in HEAD (commit 650):
  the WAL keeps only the unflushed tail of the active segment in I/O pages (`WalTailWindow`, an idle document holds none;
  before, every mounted document held a full operation's 64 pages → no 16th mount), and a document unmounts when its last
  `ArtifactHandle` drops (`ArtifactHandleLease::unmount_idle`, `DatabaseDocumentMountSlot::Closing`, shutdown drains
  closing mounts). The law passed at 18:22 (405 s). Not yet re-run: the db suite, the hub suites, the growth e2e.
- 05:5x **creation-phase progress** (item 3) implemented end to end, then parked by rule 20 (guest freeze): schema
  `SpaceArtifactCreationProgress` {stage: queued | compiling-guest | genesis | publishing, completedUnits, totalUnits}
  on accepted/preparing statuses only (allOf forbids it on terminal phases), 9 fixture rows, Rust + TS twins, Ajv oracle
  PASS; hub: the creation control records the authority's progress (`AuthorityProgressStage::GuestCompiling` new) and
  the status route attaches it. The schema lives in `semio-framework-os-kernel` (guest-linked) → reverted from the tree
  at 08:4x and kept as a patch set: `wp-h9/codemods/creation-progress/os-kernel-and-creation.diff` (`patch -p1`, dry
  run clean) + `hub-creation-progress.py [--reverse] [--dry-run]` (9 anchored hub edits + the oracle
  `creation-progress-oracle.ts`, dry run clean). Still to add on landing: `GuestCompiling` reports from the catalog's
  `compiled()` (H10 rewrote it) and a law that every creation reaches ready or a typed refusal within its bound.
- 06:01 growth e2e **g14** (sqlite, catalog B2, RW-gate binary, artifact-schema edits): creation + concurrent growth ran
  (edits accepted across all 24 documents) until `0-2 edit 76` was **Rejected `unavailable: artifact submit WorkerPool
  submission failed: Contended`**. Root cause: `ArtifactSubmitState::submit_exact` counts a queue-lock race
  (`Contended`, not a capacity signal) against an 8 × 1 ms retry cap, so under 24 busy documents an edit is refused
  instead of admitted. The storage layer already exempts `Contended` from its budget (`DB_IO_RETRY_LIMIT`); the artifact
  submit / history / runner / capability-open / catalog-read owners do not. Fix next.
- 08:4x resumed after the usage reset. `laws-8`: full os-hub bin suite **153/156** — two request-ownership laws
  (`canonical_pair_route_disconnect_…`, `directory_event_page_v1_…revalidates…`) failed: my `refusal_middleware` ran each
  handler as a SPAWNED task, so a disconnecting client no longer dropped its handler. **Fix:** the handler is polled in
  place under `catch_unwind` (no spawn, no per-request task); **law** `the_refusal_layer_types_a_panic_and_keeps_every_handler_request_owned`
  (a panicking route answers `500` + `x-semio-refusal`, a disconnect drops the pending handler < 5 s, the server survives).
- 09:0x `laws-9`: full os-hub bin suite **156/157** — gates law-green (told the coordinator for W2's 7800 binary). The one
  red law, `a_document_socket_keeps_acknowledging_commands_as_its_document_grows` (passes alone), fails under suite
  concurrency: edit 30 **Rejected `unavailable: DB I/O process aggregate credit exhausted`** — other laws' documents hold
  the 64-slot process ledger (pathmap resident values, see 04:5x) and the edit is refused rather than admitted when a
  slot frees. Same class as g14: a transient capacity refusal surfaces as a rejected edit.
- 09:0x **one retry policy for refused worker submissions** (`🛢️db/🎚️policy`): `worker_submit_retry_attempt(kind,
  attempt, limit)` — `Contended` (a queue-lock race: idle workers scan and steal from every queue under its lock) spends
  no attempt, `Saturated` spends one of the owner's limit, `Shutdown`/`Poisoned` are terminal. Language-agnostic
  decision table `🎚️policy/🧫️fixtures/🔁️worker-submit-retry/🔣️.json` + law
  `worker_submit_retry_follows_the_declared_decision_table` (10 000 consecutive races never terminal). Routed through it:
  artifact submit, artifact history, artifact runner, capability-open, catalog-read, create-catalog (×2; its test
  counter is now `spent_submission_attempts`, renamed in the root `📜️script.ts` source law too) and the DB I/O task
  owner (which already exempted `Contended`, now via the shared function). Compaction / sync-hello retry / catalog
  bootstrap keep their wall-clock deadlines (not on the edit path). db laws **59/59** (nextest).
- 09:1x hub law isolation: `every_command_is_answered_when_the_database_refuses_writes` deliberately exhausts the
  process DB I/O ledger, so it refused the concurrently running growth law's edits (laws-9 red). It now re-runs itself
  as the only law of a child process (`process_isolated_law`, the db crate's protocol; in place under nextest) —
  long laws **8/8** (`laws-10.txt`).
- 09:46 growth e2e **g15** (sqlite, B2): 24 documents created (21 s first … 451 s last), **3680/3680 edits accepted**
  (22 × 120 + 2 × 520), SIGTERM → exit 2.5 s, exit → respawn 33 ms, SIGTERM → ready 30.6 s (catalog load; H10). Then
  `0-2 refused: unavailable: database sync hello admission saturated`: 24 sockets reopening at once exceed the
  process's 8 sync-hello slots, and `Database::hello` refused instead of waiting.
  **Fix:** reusable fixed-capacity waiter table `db_policy::AdmissionWaiters<N>`; the sync-hello admission wakes every
  waiter on release; `DatabaseSyncHelloAdmissionReady` (pool-clock deadline, cancellation by drop, bounded 256
  waiters); `Database::hello` waits for a slot (≤ `DATABASE_HELLO_ADMISSION_WAIT_MS` = the hub frame deadline) and
  retries, while the retained `try_submit` keeps its exact capacity refusal (its laws unchanged).
  **Law** `hellos_beyond_the_admission_slots_wait_for_a_slot_instead_of_refusing` (8 held sessions; the retained 9th
  is refused `admission_saturated`, the awaited 9th registers as a waiter and is welcomed when one session drops).
  db hello/sync/admission/policy laws **84/84** (nextest) + in-process isolated run PASS. Binary rebuilt 09:5x → g16.
- 09:5x **agent revocation, live leg** (item R): the real-binary two-client e2e gains a delegated agent on the same
  document — fixture `🤝️two-client-document-v1` `agent` {audience edit, revocationWithinMs 5000, closeCode 4401,
  refusedStatus 401}, two new steps, expectation `revocationEndsAgentSession`; A delegates, the agent exchanges the
  delegation for a session, opens its own socket and beats presence, B sees it; A revokes → the agent's socket closes
  4401, B's roster drops it, `GET /auth/sessions/me` answers 401 and a second exchange of the revoked delegation 401,
  all within the bound (receipt `agentRevocationMs`). The fixture's schema had drifted (no `growth`, no
  `shutdown.restartWithinMs` under `additionalProperties: false`), so the fixture never validated: schema fixed, the
  fixture test now validates it with Ajv (+ a negative vector); Rust twin asserts the agent row. Fixture test PASS,
  hub typecheck clean; live run `tc16` (sqlite) launched 09:57. The MCP `action_invoke` refusal rides on the same
  refused session (G10's os-mcp surfaces it); not exercised through os-mcp here.
- 10:0x `tc16`/`tc17` (sqlite): the agent leg passed (2–3 ms), but the e2e asserted 401 for re-exchanging a revoked
  delegation where the hub's declared answer is `403 delegation-revoked` (law
  `an_agent_delegation_mints_a_session_that_works_until_it_is_revoked`) → fixture `reexchangeStatus`/`reexchangeError`.
  `tc17` then failed `a-restart missing Welcome` after 15 s with no diagnostics: the runner now names the bound and
  appends the live hub's output (`hubOutput` follows the restarted process), and the first reopen of the grown
  document is bounded by the fixture's new `shutdown.reopenWithinMs` = 30 000 (the growth fixture's bound).
- 10:16 **`tc18` sqlite PASS 2/2**: agent revocation 3 ms, 300 + 30 growth edits, SIGTERM → exit 1.5 s, exit → respawn
  16 ms, ready → reopened Welcome **16.2 s** (a 300 × 16 KiB pathmap document replays on its first open; debug binary,
  load ~30 — performance, noted for H10), SIGKILL → ready 12.9 s → reopened 31.3 s, 0 refused opens. `tc19`
  (postgres, neo4j) launched 10:16.
- 10:2x `g16` (sqlite, rebuilt binary): 24 created, **3680/3680 accepted**, then the latency law failed for both
  520-edit documents (note 882 → 3294 ms, gismap 864 → 3432 ms, bound 3×) — their last window coincided with my own
  `tc18`/`tc19` e2e runs started in parallel (g15, run alone, had 1048 → 1588 ms): my contamination, not document
  growth; the restart leg (hello admission fix) was not reached. Rerun alone: `g17`.
- 10:2x full os-hub bin suite **160/160** (`laws-11.txt`) on the current tree (RW gates, refusal layer, isolation, db
  retry policy + hello admission). Hub lib **225/226**: the one red law is H10's
  `a_default_cost_credential_derivation_fits_the_session_mint_budget` (absolute `< 1 s` bound; 1.44 s at load 40 with
  my e2e runs; its relative bound passed) — H10's.
- 10:42 **`tc19` neo4j PASS 2/2**: agent revocation 6 ms, 300 + 30 growth edits (523 s), SIGTERM → exit 0.5 s, exit →
  respawn 31 ms, 0 live writers after exit, ready → reopened Welcome 26.7 s, SIGKILL → ready 37.9 s → reopened 62.2 s,
  0 refused opens. **`tc19` postgres FAILED**: agent leg PASS, then growth edit 105's frame exceeded the 30 s frame
  deadline (`socket=closed 1013 frame-deadline`) — the same postgres-only stall session 11 saw at edit 79. `tc20`
  (postgres) running with a `sample` capture every 8 s (`wp-h9/hub-stall-sampler.sh`, `s12-h9-logs/stall-tc20/`).

## Landing (guest ABI, 00:56 → 01:13)

H8's frozen `codec.replay-envelopes` set, re-derived on the current tree (not the frozen text verbatim):
- WIT `codec.replay-envelopes(artifact-kind, pair, envelopes) -> result<document-pair, plugin-error>`.
- Kernel `store::replay_envelopes_onto_pair(pack, spr, envelopes, owners: impl FnOnce() -> DocumentStoreOwners)` — folds
  every envelope through `ArtifactStore::ingest_remote`; refuses a quarantined conflict (`!accepted || conflict`), an
  envelope left pending on an unknown dependency (`MutationDag::pending_is_empty`, new), a corrupt stream, a missing
  baseline. Owners are a factory (an unused `ArtifactStoreCursorDisposer` asserts in `Drop`). The close cursor shared with
  `apply_ops_binary` moved into `close_codec_reduction_store`. `ArtifactCodec::replay_envelopes` thunk.
- Plugin crate: `PluginApp::artifact_replay_envelopes`, `artifact_app_replay_envelopes::<A>` (app owner catalogue),
  `plugin_artifact_replay_envelopes`, component `codec::Guest::replay_envelopes`, owned `semio_owned_replay_envelopes_v1`
  (`OwnedSemioExport::ALL` 13 → 14). Plugin host: `OwnedOperation::ReplayEnvelopes`, owned
  `codec_replay_envelopes_observed` (fuel progress), wasmtime `codec_replay_envelopes`, `GuestRuntimes::codec_replay_envelopes`.
- MCP guest-backed codec `guest_replay_envelopes`; hub `TrustedArtifactReplayCodec` (+ impls for the native/guest
  catalog codec and the plugin-host codec); hub fixture codecs.
- Consequence recorded in `📓️landing.md`: a host built from this tree refuses components built before it.

## Item 1 — Check In design (as landed)

- **Contract** (`📇️directory/🧬️schema`): `DocumentCheckInV1 {schema, requestId, head: EditedArtifactFrontierV1}` and
  `DocumentCheckInStatusV1 {phase accepted|materializing|publishing|ready|failed|cancelled, progress {completedUnits,
  totalUnits = 8}, ready?, refusal?}`; refusals `unknown-head | stale-head | active-checkpoint-changed |
  ledger-not-replayable | codec-refused | authority-changed | unavailable`. `CheckpointPublicationFrontierV1` renamed
  `EditedArtifactFrontierV1` everywhere (GIS inference, MCP dispatch, wgpu test, TS). H8's `label` dropped: the version
  message is the ledger's own `CommitCheckpoint` transition; an echo the hub never stores would be decoration.
  Fixture `📌️document-check-in-v1/🧫️fixtures/🔣️.json` (9 valid, 18 invalid, `schemaValid` flags for the canonical-form
  and progress laws JSON Schema cannot express).
- **Job** (`🌎️hub/🗿️artifact-authority/📌️check-in`): `ReplayingArtifactAuthority::materialize_check_in` (resolve codec →
  validate input → `TrustedArtifactReplayCodec::replay_envelopes` → validate output → candidate at baseline = head,
  parent = active checkpoint), `DocumentCheckInJob` (monotonic progress, one terminal outcome, `cancel`, `revoke` →
  `authority-changed`, it is the `AuthorityOperationControl`), `DocumentCheckInJobs` (join by author/document/request,
  256 live, 1024 retained, conflict on a different request under the same id).
- **Hub** (`🏗️bootstrap` `📌️CheckIn`): author-only (401/403), durable idempotency through the existing
  checkpoint-publication claim ledger (request id = correlation id, SHA-256 of the canonical body), stall-bounded
  (30 s, guest fuel counts), 50 ms author revalidation monitor, active pair from `VerifiedRebootstrapSource`, ledger from
  `db::document::artifact_ledger_tail`, fenced publication (author still writes, descriptor unchanged, active
  checkpoint still the parent) completing the claim in the checkpoint event's transaction; `head == baseline` answers
  the active checkpoint (no-op), a head behind it `stale-head`, a head not on the ledger `unknown-head`; trace
  `server.document.check-in` (vocabulary 23 → 24).
- **Deleted** (no legacy): `POST …/checkpoint-publications`, `CheckpointPublicationCommandV1/CurrentV1/BlobV1/ReceiptV1`,
  `CHECKPOINT_PUBLICATION_*` consts, the command JSON defs, `📣️checkpoint-publication-command-v1/`,
  `🌱️artifact-genesis-v1/📤️current.json`, the two upload-route laws, `publishCheckpointPublicationProcessPairV1` and
  `proveCheckpointPublicationCommandV1`. Process probes (MCP cold mount, GIS proposal, two-author shell) now commit a
  real GIS Map ledger edit and Check In its head (`checkInProcessHeadV1`); gate renamed `check-in-check`
  (`os-hub:check-in-check|check-in-native-check|check-in-process-check`, launch.json `⚖️gate📌️check-in…` 411.11–13).

## Item 2 — Store writes that can fail (O2-5b)

Verified on the current tree: `server::storage::{ProjectionStore::put/set_checkpoint/clear, SessionStore::create/delete}`
all return `Result<(), StorageError>` (W3d, ticket 26/09/18) and the hub's `HubProjectionStore`/`HubSessionStore` append +
flush + `sync_data` before folding, with fault laws (`a_projection_write_reports_a_failing_sink`,
`a_session_write_reports_a_failing_sink`). Callers: sign-in (`record_instance_session` → refuse + withdraw the directory
issuance), `DELETE /auth/sessions/me` and credential change (→ 500/503) already surfaced it; **the agent-delegation revoke
route discarded it (`let _ =`)** and now answers `503 directory-unavailable` (`🏗️bootstrap` `delete_agent_delegation`).
New route-level law: a sign-in whose instance session directory refuses writes answers 503 `directory-unavailable` and
records nothing.

## Item 3 — Declared authorization (O2-10)

- **Policy** `🌎️hub/🔐️auth/🛡️access-policy/🔣️.json` (schema `schema://hub.auth/HubAccessPolicyV1`, defs `HubAccessRoleV1`,
  `HubAccessActionV1`, `HubAccessGrantV1`, `HubAccessDecisionVectorV1`): roles `admin | owner | author | spectator | share |
  authenticated`; actions `space.create|rename|visibility|archive|delete, member.upsert|remove, invite.create|revoke,
  document.announce|read|write|check-in, agent.delegate, artifact.create, blob.read|write`; archive denies every write.
- **Authority** `semio_hub::auth::access_policy::hub_access_permits(roles, action, space_kind)`; roles are derived
  (membership, ownership, share token, operator subject), decisions never hand-written.
- **Routed through it:** directory commands (`authorize_directory_command` = one `permits` call), document read
  (`authorized`, canonical pair), space blobs (`authorized_for_blob(…, BlobRead|BlobWrite)` — writes now need the write
  grant, a spectator can no longer PUT a blob), document-socket `SecurityGate` (compiled from `document.read`/`write` in the
  space's kind; a share now maps to its own `share` role), open-plan surface writability, Check In, artifact creation
  (route + final commit authority), agent delegation, GIS approval delivery. `db::security::space_grants` (the second,
  divergent copy) deleted with its two laws.
- **Laws:** Rust `declared_access_policy_matches_the_language_neutral_truth_table` (113 vectors in `🧫️fixtures`),
  `access_policy_is_closed_by_default_and_deny_overrides_allow`; TS twin + Ajv `🧪️tests/🛡️access-policy` **2/2**
  (`generated/hub-ts-access-1.txt`).

## Item 1 — os clients (as landed)

- **Worker protocol** (`💻️os/🟦️.ts`, `🏪️store/👷️worker/🟦️.ts` `📌️DocumentCheckIn`): `document-check-in {requestId,
  clientInstanceId, scope}` / `document-check-in-cancel`, answered by `document-check-in-status {status}`. The worker
  waits (≤ 15 s) until the replica is quiescent (no pending batches, empty outbox, no pending mutations) and names its
  acknowledged frontier as the head, submits, polls every 100 ms (deadline 180 s, 8 concurrent), cancels on request.
- **React shell** (`🏛️ShellHost`): a checkpoint this shell dispatched that lands in history triggers `requestHubCheckIn`;
  History panel item `framework.history.checkin-status` (`role=status`) + abort button; labels `checkinStatusText` /
  `checkinAbortText` en + de (`🛠️ShellHelpers`).
- **Native wgpu shell** (`🐚️Shell/🎯️targets/🧊️wgpu`, not wasm32): `ArtifactSyncStatus.acknowledged_head` (new; only when
  outbox and pending set are empty) is the head; `request_hub_check_in` on the landed checkpoint, `pump_hub_check_in`
  per frame from `pump_directory_events` (submit → poll → cancel, 401/403 → `authority-changed`), hub footer badge shows
  the phase via `shell_chrome_string` `checkIn.*` en + de.

## Item c — HubSagas decision

Keep `HubSagas` as the CQRS outbox drain with an empty saga set: it is the only retirement path for `/commands` outbox
events; every cross-aggregate consequence (membership removal → socket revocation, delegation revoke) is synchronous
under the membership fence, so there is no multi-step saga to run. Recorded on the type (`🌎️hub/🗄️stores/🦀️.rs`).

## Item 4 — db gate: lost wake in artifact-runner retirement

- **Symptom:** `cargo test -p semio-framework-os-kernel-db --lib` in-process failed under load: run 1 had 19 failures
  (engine laws failing at `Database::open(...).unwrap()`), and a later run had 1 failure:
  `artifact_authority_drop_reuses_registered_retirement_slot_beyond_capacity` reported "retirement did not release its slot:
  driver=5 (ClosingReady) terminal=false turns=0 maintenance=true" after its 60 s watchdog (`generated/db-lib-loop-3.txt`).
  nextest (process per law) was green.
- **Cause (`🛢️db/🗿️artifact/🦀️.rs`):**
  - `ArtifactRunnerRetirementReservation::commit` published the maintenance ticket to the handoff *before* it wrote the
    cursor into its global slot.
  - A runner transition in that window (the close-poll drop reaching `ClosingReady`) requested maintenance. A pool worker
    ran `artifact_runner_retirement_step`, found the row empty, and returned `Retire`, so the hook was removed.
  - The commit's own request then hit a stale ticket (ignored). The cursor, its slot (1 of 64, process-global) and its pool
    use stayed stranded for the life of the process (`turns=0`).
  - Stranded slots exhaust the 64-slot table, and every later mount fails. That is the 19-failure cascade.
- **Fix:**
  - `commit` writes the cursor first and publishes the ticket last.
  - The step answers `Idle` (keeps the hook) for a reserved-but-uncommitted slot. Only a generation that no longer owns
    the slot retires it.
  - New law `retirement_turn_before_its_commit_keeps_the_hook_for_the_committed_cursor` (deterministic, calls the step
    between reserve and commit).
- **Measured after the fix:** in-process 705/705 on 6 consecutive runs under load (`generated/db-lib-fixed-{1..6}.txt`);
  nextest 705/705 (`db-nextest-2.txt`).

## Item b — `Database::shutdown` on every exit path

- **Before:** every `?` in `main` after `connect_db` returned without shutting the database down. That covers the
  directory, CAS and coordinator setup, the extension dir, the GIS binding, the inference ledger, session-key minting,
  `compose_hub_server`, and a port-in-use `TcpListener::bind`. The storage close was skipped, so the Neo4j lease, the
  Postgres advisory session and the sqlite sidecar were only released by process death or TTL.
- **Now:**
  - Everything after the open runs inside one `served` block. `close_hub_database` runs unconditionally after it, on
    serve, on a refused startup and on a bootstrap-pipe close.
  - `hub_shutdown_record` emits one `server.shutdown` record for every exit (`retained-sockets=N` or
    `startup-refused=<error>`, plus `database=closed|<error>`).
  - Check In jobs, which hold `Arc<Database>`, are cancelled and drained before the close, so the database's `try_unwrap`
    can succeed.
- **Live restart-within-1 s probe:** still to run. It needs an os-hub with the postgres + neo4j drivers from this tree
  and a current-tree catalog (catalog A is refused by 14-export hosts).

## Item 1a — `gis_map_abandoned_pre_witness…` flake

- **Instrumentation:** every `GisMapApprovalCommitErrorV1::Storage` construction in the runtime (42 sites) now goes
  through one `#[track_caller] gis_map_storage_refusal(&error)`. It records `file:line: error` in test builds, which
  replaces H8's `[DEBUG]` print and the six hand-named sites (codemod `wp-h9/codemods/gis-storage-refusal-sites.py`).
- **Measurement:** failures kept reporting `last storage refusal None`, so the `Storage` came from outside the runtime.
  The only other constructor on that path is the law's own `OrderedApprovalCheckpointPublisherV1`, whose attempt 0
  returns `Storage` by design.
- **Mechanism:**
  - `poll_approval_to_phase` polls the approval by hand and checks the retained state between polls.
  - `Preflight` was the only commit turn with no yield. When the actor's snapshot reply was already there (the test
    thread preempted right after the ask, which is common under load), one poll ran Preflight → Assembly.
  - The harness never saw `Ready{pending}`. It kept polling through journal and publication until the injected failure.
- **Fix (`🏃️runtime/🦀️.rs`):** the `Preflight` arm of both commit loops (approval and undo) yields once before the
  snapshot, like `Continue` and `Committed`. The rule is documented on `GisMapCommitTurnV1`.
- **Measured** on a copied test binary, one law per process, under load (two loops plus a cargo build):
  - before: 1/1000 alone (`flake-exact` run 644) and 6/3000;
  - after: **0/3000** (`generated/flake-fixed-{a,b}.txt`).

## Item d — all-features fixes

- **Approval retries answered 409.**
  - Each retry of a prepared approval re-stamped a fresh command with the wall clock. This happens whenever
    `document_clock` is `None`: fail-closed committer, or document not mounted.
  - The ledger then refused the new command hash against the prepared outbox (`prepare_approval` → `Conflict`).
  - `approve_gis_map_job` now reuses the durable prepared command (`approval_recovery_by_mutation`, uncommitted, same
    job + proposal).
  - `gis_map_approval_is_idempotent_across_duplicate_requests_and_restart`: **PASS**.
- **`trusted_publication_owner_process_crash_releases_exact_lock` failed with ENOTDIR.**
  - Server-owned roots are opened per component with `O_NOFOLLOW|O_DIRECTORY`, so a `SEMIO_TEST_ARTIFACT_DIR` reached
    through a symlink (`.tmp-ticket`, or macOS `/tmp`) is refused.
  - `test_artifact_root()` (`🌎️hub/🧪️tests/🗂️artifact-root`) now returns the canonical path. **PASS**.

## Item e — local-bootstrap pipe (W2's two defects)

- **64 per run → per window.**
  - `consumed` was a non-evicting 64-slot set. The 65th exchange failed `insert` → `Unauthorized` → `serve_local_bootstrap`
    returned `Err` → the hub exited (W2 measured this at 11:41).
  - It is now `ExchangeWindow<LOCAL_BOOTSTRAP_REPLAY_WINDOW_MAX = 64>`. Entries are evicted once their own `expiresAt` is
    past; no frame carrying them can validate again, and the strict sequence already forbids replays. The fixture
    declares `replayWindowExchangesMax: 64`.
- **Refusal = answer.**
  - Integrity is still channel-fatal: an unparseable frame, a wrong run id, a wrong sequence or a bad proof.
  - Everything after authentication is answered with a signed `reject` and the loop continues:
    - an expired window → `expired`;
    - a device-text, profile or client-class violation → `denied`;
    - a replayed live id → `denied`;
    - a full window or a full pending set → `resource-limit`.
  - Stale cancel or shutdown frames are ignored. A cancel only marks a still-pending exchange, so the cancelled set can
    never overflow.
  - Request-task failures (issue, delivery, reject write) stay per-request; only a panicking task ends the service.
  - The TS launcher now raises `LocalBootstrapRefusedError(code)` on a verified reject, instead of reporting a binding
    mismatch.
- **EOF during loading.**
  - A pump task owns the read half from hello onward. It feeds `accept` through a bounded channel (8) and raises
    `closed` on EOF or a broken frame. `LocalBootstrapTransport::closed()` is new.
  - In `main`, a watcher turns `closed` into `StartupCancellationV1`. `StartupCatalogControl::is_cancelled` observes it,
    and so does the CAS handshake control.
  - A cancelled catalog load returns `Cancelled` without being retried as a stall. `main` emits
    `server.shutdown: cancelled launcher-closed-during-catalog-load database=unopened` and exits `0`.
  - A launcher that closes or shuts down the pipe while the hub is serving is now a clean `Ok` exit. It used to be
    `Err("local bootstrap endpoint closed")`.
- **Laws:**
  - `local_bootstrap_refusals_are_answers_and_the_replay_window_frees_with_time`: 64 exchanges fill one window, then
    resource-limit / replay / expired / unknown-profile each get a signed reject while the pipe stays ready. After the
    window passes, the next exchange is admitted. A forged proof is the one frame that ends the pipe.
  - `local_bootstrap_closed_resolves_when_the_launcher_leaves_before_anyone_accepts`.
  - `launcher_close_cancels_the_startup_catalog_load_instead_of_retrying_it`.
  - All **PASS**, with the existing local-bootstrap laws still green: 5/5 lib + 1 bin (`generated/laws-run-12.txt`).
- **Consequence of W2's one open-target rule (12:23):** the synthetic stdio JSON viewer can no longer be an open target.
  A manifest-level kind must equal its app's dialect, and dialects must be canonical `s.…`. The three Check In laws
  therefore moved onto the verified GIS Map profile (`check_in_map_edits`), under `integration-fixtures`. All 3 pass
  again, and the `check-in-check` native phase now builds with `integration-fixtures`.

## Item g — "hub refuses every change once a document grows" (C10)

Reproduced in `db` with `a_document_keeps_accepting_edits_as_it_grows_across_restart` (fs storage, `Profile::Prod`,
400 edits × 24 KB, full shutdown + reopen, 60 more). It hit three independent bounds in turn:

1. **Index runs outgrew one operation (edit 11).**
   - `db_index` decoded a run into one page writer *per key and per value*, all charged to the run's read operation.
     The per-operation credit is 16 controls and 64 pages, so a run of ≥ 8 entries could not be loaded.
   - The auto-merge always folded the two oldest runs, so the oldest run grew by one entry per edit.
   - Every submit records into the command, inverse, actor-seq and frontier indexes, so once any run reached 8 entries
     the document refused every write: `DB I/O aggregate admission exhausted`. The run is durable, so a restart hit the
     same wall at once.
   - **Fix:** runs are read *in place* (`RunView`: byte ranges over the run's own pages; one read costs its pages, never
     a page per entry). Merges copy key/value bytes straight from the source pages (`encode_run_from_views`).
   - A merge never grows a run past `MAX_RUN_ENTRIES`. It takes the oldest adjacent pair among the newest
     `max_runs_before_merge + 1` runs whose entries fit one run, and tombstones are dropped only with the oldest run.
     This is crash-safe (write older, then delete newer), so full runs simply accumulate.
   - `get` is a binary search in place. `FrontierIndex::latest`, `ActorSeqIndex::latest_for_actor` and
     `ProjectionIndex::latest_at_or_before` use the new newest-first `last_live_in_range` instead of materializing
     every entry. `scan_prefix` streams and materializes only live matches.
   - Run pages are closed to the arena instead of being parked as lost owners.
2. **The version graph saturated at 64 edits.**
   - `VcsVersionGraph` (the in-memory per-document hash history the commit pipeline feeds) is an `ArtifactStore` with
     the fixed 64-edit ledger, and nothing compacted it. Edit 64 failed with `edit history ledger is saturated`, and it
     failed *after* the WAL append.
   - **Fix:** a bounded window. When the ledger is full, the graph folds into one checkpoint, retires the store to its
     terminal witness (driven by progress) and continues on a fresh store from the folded hash. `head` falls back to
     the folded checkpoint.
3. **Replaced state values were never retired (edit ~123).**
   - `DocumentState::apply_entries` parks every replaced or removed value in the global retirement slots, but
     `artifact_state_retirement_maintenance_step` was only ever called by tests.
   - Each parked value keeps its own I/O operation (128 per process), so after about 125 overwrites across all
     documents, every write in the process was refused: `DB I/O process aggregate credit exhausted`.
   - **Fix:** every state apply drives the retirements until nothing is parked.

4. **The window rollover itself must not run inside one commit turn.**
   - The first live growth e2e (sqlite, postgres and neo4j alike) stopped acknowledging at the 65th edit: no Ack, no
     error.
   - A hub law, `a_document_socket_keeps_acknowledging_commands_as_its_document_grows` (150 chained commands over one
     live socket), reproduced it in 3 of 4 runs.
   - The backtrace showed the artifact runner dropping the commit turn while `roll_window_when_full` held a
     half-closed store (about 5 000 close steps). The store's Drop assertion then panicked on a pool worker.
   - **Fix:** the folded store is handed to its `VcsStoreCell` (`retiring`). Each later change advances it by at most
     256 close steps, and shutdown drains the rest. Nothing is held by a droppable future.
   - New vcs law `vcs_graph_rolls_full_windows_and_retires_them_in_bounded_steps` (3 × 64 changes, head answered,
     bounded shutdown).
   - The two vcs laws that claim the process-global admission now take the suite's `TEST_LOCK`.

**Measured:**
- db growth law, fs: **passes** (400 + 60 edits across restart, 430 s in a debug build, about 0.7–1 s per edit from
  the first edit, so it is not growing).
- db growth law, sqlite (150 edits): **passes**.
- Both db growth laws are in `long`.
- vcs laws **13/13**.
- Index laws 30/30.
- db nextest (non-long) **706/706**.
- Hub socket growth law: **8 runs, 0 panics**; 5 completed (**5/5 pass**) and 3 were cut by my own 110 s bound under
  load 26 (84–113 s each).
- Hub nextest `long` **355/355**, including the growth law, and `quick` **345/345** (17:16–17:19).
- Live: the two-client e2e now grows its note document with 300 × 16 KB chained edits before the SIGTERM restart and
  30 after it, on sqlite, postgres and neo4j (rerun on the fixed binary, running).

**Open, pre-existing:** plain in-process `cargo test -p …-db --lib` still flakes under load — 4 of 6 runs this
afternoon, and also with my state-retirement drain disabled (bisect). Every failure is `db I/O backend control capacity
exhausted` (64 backends per process, retired by maintenance on the shared 2-worker test pool). This morning's
19-failure cascade had the same failing-law set, so my earlier "6/6 after the retirement fix" was a lucky streak, not
proof of that fix. nextest (one process per law) is unaffected.

## Log
- 00:56 landing row announced; 01:13 all native + wasm32 checks green (`generated/abi-*.txt`).
- 05:2x resumed after usage reset; wgpu Check In already landed + compiled (native + 4× wasm32 green, `generated/wasm-check-2.txt`); Docker answers.

- 05:25–06:37:
  - Check In lifecycle law fixed (publication failure under a revoked author → `authority-changed`; a removed member
    gets 403, not 401). All three hub laws pass.
  - Two-client e2e on :8010 **2/2**.
  - db retirement lost-wake fixed.
  - Flake root-caused and fixed.
  - Every exit now closes the database.
  - All-features count **372/375**; gates green.
  - Cleaned up: binary copies, the catalog A copy, test-artifact dirs, the private target and the `*-before-*` backups.
- 12:16–12:35: an external low-disk cleanup deleted `wp-h9/generated` (captures cited above up to 06:37 are gone; the results stand as recorded, re-measured where noted). Data roots and catalog copies now live under `.🧬semio/🌐hub/s11-h9-*` (revised rule 15).
