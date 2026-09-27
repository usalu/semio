# WP-V1 — Acceptance Harnesses As Permanent, Zero-Touch Commands

Slice V1 (session 13). Source: [📓️acceptance-s13.md](📓️acceptance-s13.md) §6 (NO HARNESS), §7 (not zero-touch), §8 (final
verification plan). Ports: hubs 8180–8189, serves 6680–6689. Scripts/codemods `wp-v1/`; expendable captures `wp-v1/generated/`;
durable data `.🧬semio/🌐hub/s13-v1-*`. Builds/tests prefixed `nice -n 10`.

## Session 13

| # | Item | Status | Evidence |
|---|---|---|---|
| 1 | Zero-touch postgres + neo4j backends (`📜️script.ts` verb, used by the pg/neo4j gates, launch rows) | **DONE (harness measured)**; shared pair survived the night (Up 15 h); `backend-run` (DB1) measured; full two-client pg run needs the rebuilt all-driver `os-hub` | `backend-up-1.txt`, `backend-oracle-1.txt`, `claim-probe-1.txt`, `s13-v1-logs/two-client-postgres-2.txt` |
| 2a | editor/viewer/locale matrix `@semio-tech/framework-os-dev:program-matrix` + S16's tool-run column `:tool-run-matrix` + S16's hub-document sweep `:hub-document-sweep` | matrix **measured PASS 4/4** on local-only 6680; tool-run + hub sweep ported 11:xx (dry-run `blocked` records, tsc 0) — first live runs S16's after the restage | `s13-v1-logs/matrix-proof-3.txt`, `generated/dry-{tool-run,hub-sweep}.json` |
| 2b | two-human orchestrator → `@semio-tech/framework-os-dev:two-human` (`verify two-human`) | code landed, tsc 0 errors; live run pending | `🧑‍💻dev/🧪️tests/👥️two-human/` |
| 2c | `@semio-tech/framework-os-mcp-rs:plugin-coverage-check` + `user-path-check` | coverage **measured PASS 2/2** (note, draw: created + mutated over the semio MCP); user-path needs hub + serve (pending) | `s13-v1-logs/plugin-coverage-2.txt` |
| 2d | `os-hub-ts:backup-restore-drill`, `:residency-watch` (+H12), `:boot-watch` (H12), `@semio-tech/framework-os-kernel:reopen-storm-check` (+DB1 pg/neo4j via `os-hub-ts:backend-run`) | drill **measured PASS 1/1** (B2 catalog, note, 20 edits; SIGTERM→exit 590 ms, tar 568 MB in 1.2 s, 134 files byte-identical, restored ready 62 s, frontier/descriptor/checkpoint/listing/next edit all equal); residency/storm unrun (hub / rule 25) | `s13-v1-logs/backup-drill-2.{txt,json}` |
| 2e | `workspace:verify -- production-placeholders` / `-- interactivity commands` (scanner + `git grep` oracle) | **measured**: placeholders **PASS** (22 875 sources, 0 production); commands 20:5x FAIL 34/1160 → 06:1x after LC's P8 landing **FAIL 4/1165** (space 3, animate 1), oracle agrees; laws 12/12 | `generated/placeholders-1.txt`, `commands-3.txt`, `s13-v1-logs/goal-gate-compliance-1/` |
| 2f | `os-hub-ts:hub-freshness` (Cargo freshness over the `os-hub.sources.json` record the staging verbs now write) | **measured** on hub 8021: `unverifiable` (C11's copied binary has no record) — correct verdict | `generated/freshness-8021.json` |
| 3 | goal gate `@semio-tech/repo-test-domain:acceptance-goal` (+ `acceptance-plan`), launch row `⚖️gate🎯️repo-goal` | landed; plan 9 steps / 40 checks (G11 security, F2 connection-budget, H12 boot-watch, DB1 pg/neo4j storm in); zero-touch backends pre-step; laws **16/16** (Ajv 2020 oracle); live run after REBUILD | `🧪️test/🎯️acceptance/` |

### Log (session 13)

- 19:47 slice start; read AGENTS.md, preambles 13 + 12, `acceptance-s13.md`, `fleet-13-agents.md`, `wp-c11.md` hub recipe,
  `wp-r9.md`. Machine: load 87, 23 rustc, **87 GiB free** (guard threshold 80) → no cargo builds from V1 unless unavoidable.
  Asked R9 (af7aac1bfa443ca5c) which launch-registration mechanism to use.
- 19:5x item 1 code: `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts` region `BackendServers` (`HUB_BACKENDS`, `hubBackendIdentity`,
  `hubBackendStatus`, `ensureHubBackend` with progress + AbortSignal cancellation, `stopHubBackend`, `claimHubBackend`); verb
  `os-hub-ts` `backend <up|down|status> <postgres|neo4j|all>` + nx targets `backend-up/-down/-status`; `os-hub` `dev postgres`
  zero-touch + new `dev neo4j` (`dev-neo4j` target); `BackendE2eScript` (two-client-e2e, document-growth-e2e) reuses the shared server.
  Cross-platform story (docstring of `HUB_BACKEND_ENGINE`): Docker Desktop (macOS, Windows), Docker Engine (Linux), docker-in-docker in
  the devcontainer — the engine always publishes on this process's loopback, so every platform reaches `127.0.0.1:<port>`.
- 20:0x `backend up all` (measured, `generated/backend-up-1.txt`): postgres ready 12 s (`127.0.0.1:49706`), neo4j ready 60 s
  (`127.0.0.1:49723`) at load 87; second `up` reuses in 9 s. Third-party oracles (`v1-backend-oracle.ts`, `generated/backend-oracle-1.txt`):
  Bun.SQL `SELECT current_database(), 1+1` → `semio`, `2`, PostgreSQL 17.11; neo4j bolt handshake agreed version 5.4.
- 20:0x coordinator rule 22 (shared Docker pair, never a second pg/neo4j): my first two-client run (ephemeral container design) was stopped
  before it created a container (pids 54464/54468/54469, mine, killed). Redesign: gates CLAIM the shared server — postgres = a fresh
  database `semio_run_<owner>` dropped on release; neo4j = exclusive lease `.🧬semio/🌐hub/backend-neo4j.lease` (pid-owned, stale lease taken
  over, waited with progress + cancellation) + graph reset. Probe (`v1-claim-probe.ts`, `generated/claim-probe-1.txt`): pg claim 6.0 s
  (probe query answers `semio_run_v1probe_57374`), neo4j claim 17.8 s (graph 0 nodes), after release only `postgres,semio,template0/1`
  remain and no lease file.
- 20:1x hub typecheck (`os-hub-ts:typecheck`): rc 0, 0 errors; `tsc --listFilesOnly` includes the three edited files.
- 20:11 `two-client-e2e postgres` on the shared pg (`OS_HUB_BINARY` = H9's all-driver 15:33 binary, hub port 8180): claim + env OK
  (`postgres=127.0.0.1:49706 (semio-hub-backend-postgres)`), hub loaded the fixture catalog 10328/10328 then exited
  `ArtifactAuthority(Catalog("missing field pluginModule"))` — the 15:33 binary predates the schemaVersion-3 catalog; 1 passed / 1 failed.
  Run database dropped afterwards (verified). A pass needs `os-hub:build-dev-postgres` on the current tree (a cargo build; deferred
  under rule 22 — H11 owns the pg/neo4j runs and has the recipe now).
- 20:1x launch rows (seed + launch.json, identical, via `wp-v1/v1-launch-rows.py`): `🛠️dev🗄️os-hub🐘️postgres` (387.002),
  `🛠️dev🗄️os-hub🕸️neo4j` (387.003), `🛠️dev🐳️hub-backends⬆️up` / `🩺️status` / `⬇️down` (387.004–387.006). R9 agreed to this mechanism
  (seed row + identical rendered row) until its generator lands.
- Taxonomy: new directories of this slice are NOT registered in `🔣️taxonomy.json` now (a taxonomy edit recompiles every MutationLeaf crate
  via `include_str!`, and hub/MCP test siblings are unregistered today: `verify taxonomy report --scope 🌎️hub/🧪️tests` = 13 pre-existing
  `directory-kind-unresolved`). The registration patch is prepared in `wp-v1/` and lands after PUBLISH DONE.
- 20:2x acceptance module (item 3 core, first because every harness writes its record): `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/`
  — `🧬️schema/🔣️.json` (JSON Schema 2020-12: `checkResult` = `semio.acceptance.check-result/v1`, `goalPlan`, `goalSummary`),
  `🎚️config/🔣️.json` (the §8 plan: 9 steps, 33 checks, criteria ids, requirements, browser counts), `📋️orchestration/🟦️.ts`
  (schema interpreter, `publishAcceptanceCheckResult`, `runGoalGate`: serial/parallel steps within `--max-browsers` (default 1,
  rule 22) and `--max-parallel`, gating, cancellation of the running checks' process groups, `summary.json` + `summary.en.md` +
  `summary.de.md`), verbs `acceptance goal|plan` on `@semio-tech/repo-test-domain` (targets `acceptance-goal`, `acceptance-plan`).
  Credentials only through `--users <json>` → env tokens `{user1Email}`…, refused as arguments. Laws
  `🎯️acceptance/🧪️tests/🎯️goal-gate/🟦️.ts` + fixture: interpreter and Ajv agree on 9 records, the repo plan validates under both,
  4 scripted runs (gating, missing serve, record-vs-exit precedence, browser budget) → **15/15 pass, 51 expects** (`bun test`).
- 20:3x item 2a: S6 witness + S15 matrix ported to TS (`🧮️program-matrix/🟦️.ts`), pins as data (`🧮️program-matrix/🔣️.json`,
  `semio.os-dev.program-matrix-pins/v1`), census read live from `window.__semioOsCatalogProbe` (no census file), resume, per-row
  flush, cancellation after the current row, `matrix.json` + `table.md` + screenshots under `🧑‍💻dev/🤖️generated/🧮️program-matrix/<tag>/`,
  acceptance record `program-matrix`. S16's `s16-matrix.mjs` is byte-identical to S15's apart from paths → S16 can switch.
- 20:4x item 2b: `👥️two-human/🟦️.ts` ports C11's `c11-collab-matrix.mjs` + `c11-lib.mjs` + `c11-journey.mjs` (as of 19:21): two isolated
  browser contexts, `--hub/--serve [--serve-b] --locale en|de --kinds --space --admin-capability`, credentials via
  `SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD}` or `--users <json>`; reuses the matrix's witness helpers and pins.
- 20:4x item 2c: `🌉️mcp/🧪️tests/🧩️plugin-coverage/🟦️.ts` (G10 S3 sweep; only NON-destructive candidate mutations) and
  `🧪️tests/🚶️user-path/🟦️.ts` (G11's 20:14 version, official MCP SDK client as oracle); env names follow the sibling gates
  (`OS_MCP_HUB_ORIGIN/EMAIL/PASSWORD`, `S_OS_MCP_LIVE_SHELL_URL/LOCALE`).
- 20:5x item 2d/2f: shared hub probe client added to `🌎️hub/🤝️integration-harness/🟦️.ts` (sign-in, create-space, creation, open
  plan, document socket edits — H10's client, envelopes now carry `observed: null, target: []` for LD's new wire fields);
  `💾️backup-restore` (fresh root + catalog copy + credential user + SIGTERM + `tar` + restore to another root + per-file sha256 +
  reboot + frontier/descriptor/checkpoint-pair/listing/next-edit), `🧠️residency` (open plan per creatable kind, `ps`/`tasklist`
  resident set before/after, settle window), `🏷️build-freshness` (pid → executable via `lsof`//proc/`Get-Process`, Cargo's
  freshness rule over `os-hub.sources.json`; `os-hub:build-dev`, `build-dev-postgres`, `build` now pass `sourcesRecord`).
  Reopen storm = the db engine's own laws (`two_dozen_grown_documents…`, fs/sqlite throughput) as
  `@semio-tech/framework-os-kernel:reopen-storm-check`; the pg/neo4j twins start their own containers (rule 22) → not selected.
- Typecheck: scratch `wp-v1/tsconfig-v1.json` over every new os/repo file → 0 errors; hub `tsc` → my files 0 errors; remaining
  hub errors are peers' (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:882` builds a wire envelope without LD's new `observed`/`target`;
  `🧑‍💻dev/🔌️vite-plugins/🟦️.ts` `Database.transaction`).
- 20:4x coordinator rule 25 (no cargo test/build from V1 before REBUILD START without approval) → reopen-storm-check and any hub
  build are unrun; backup drill needs a hub binary with a published catalog (C11's copy works; runs ~11 min boot ×2 at load 80).
- 20:5x item 2e: `runSourceCensus` (acceptance module) + root `verify production-placeholders` / `verify interactivity commands`: comment-
  and string-aware Rust scanner over every git-tracked `*.rs` (22 807), test-only by path segment, cross-checked per line / per file
  against `git grep` (third-party oracle). Placeholders **PASS** (0 production; 3 test-only in `🔄️machine`/`🖌️render` tests).
  Commands: 1160 production `.action_interactive_job` declarations (literal ids, const paths, `*tool_id` loops, qualified
  classification), **34 `BatchOnlyPendingRewrite` = unreachable**: space 28 (patchParameter … renameSpace), cad 5 (applyTransformation,
  importCadFile, saveSelected/InPlay/Current), animate 1 (exportVideoFromDeck); architect 0 (T12's 8 are fixed); non-literal 0;
  oracle agrees → gate FAIL is a real finding for T13/S17. Laws `🧪️tests/🧮️source-census` 12/12.
- 21:0x repo contract (coordinator: 4 HIGHs in my layout): fixtures moved to owners (`🎯️acceptance/🧫️fixtures/{🎯️goal-gate,🧮️source-census}/`,
  `🧑‍💻dev/🧫️fixtures/🧮️program-matrix.json`); contract re-run `generated/contract-2.txt`: **0 findings in V1 files**.
- 21:0x launch rows (seed + launch.json via `v1-launch-rows.py`, group `4_gate`, 411.22–411.35) + inputs `acceptanceHubUrl`,
  `acceptanceServeUrl`, `acceptanceUsers`: `⚖️gate🎯️repo-goal`, `…📋️plan`, `⚖️gate🧮️program-matrix⚛️react` (+`🌐️de`),
  `⚖️gate👥️two-human⚛️react` (+`🌐️de`), `⚖️gate🌉️os-mcp🧩️plugin-coverage`, `⚖️gate🌉️os-mcp🚶️user-path` (+`🌐️de`),
  `⚖️gate💾️hub-backup-restore`, `⚖️gate🧠️hub-residency`, `⚖️gate🏷️hub-freshness`, `⚖️gate🚧️production-placeholders`,
  `⚖️gate🎛️command-reachability`. two-human defaults to the development users when `--users` is empty.
- 21:09 matrix proof 2 on S16's hub-joined 6541 (S16 agreed): beacon `ready:s`, registry 60, loaded 1 (space), 59 `available`,
  5 programs → 0 selected → FAIL 0/0. Cause: a hub-joined lane installs plugins per document; S15's 75/75 ran on a local-only
  serve (all 60 loaded). Harness now treats `available` as resting and fails with that explanation.
- 21:30–04:5x usage cut (rule 28). 05:0x reconcile: all V1 files present (auto-committed 22:00), shared pg/neo4j containers
  **survived** (Up 9 h, healthy). tsc: 0 errors in V1 files (os/repo scratch config and hub config; remaining errors are peers':
  `🖱️ui/🧬️contract/…/🔬️graph`, `🧑‍💻dev/🔌️vite-plugins`, `💻️os/🧪️tests/🧪️backbone-envelope-io`).
- 05:0x integrated peers' extensions: H12 (`residency-watch --rounds` + admin observability, new `os-hub-ts:boot-watch`,
  `hubSeedTrustedCatalog`), DB1 (`os-hub-ts:backend-run <pg|neo4j> -- <cmd>`, storm laws read the claimed env), G11
  (`mcp-security` in the plan), F2 (`connection-budget` in the plan). Launch rows `⚖️gate🌅️hub-boot-watch`, `⚖️gate🌪️reopen-storm`
  (+`🐘️postgres`, `🕸️neo4j` through `backend-run`; nested `--` measured: `backend-run -- postgres -- printenv OS_HUB_DATABASE_URL` →
  `…/semio_run_run_98962`, claim released). Plan fix: `hub-reopen-storm` is `@semio-tech/framework-os-kernel` (was mis-named os-hub-ts).
  Gate: any check requiring `backends` → one `os-hub-ts:backend-up -- all` pre-step; failure blocks only those checks (new law).
- 05:1x launch.json parity with the generator (`v1-launch-parity.ts`: `generateLaunchJson(renderCatalogFiles)` vs file):
  **fresh=true, 11 220 lines identical**. Backup drill proof launched (w2-detach pid 2047; C11's 19:07 binary + B2 catalog clone,
  kind note, 20 edits, 1 round; the 09-23 hub-dev catalog predates `pluginModule`, so B2).
- 05:1x drill proof 1 (`backup-drill-1.txt`) hung: `startHub` (integration harness) spawns the hub without the fd-3 local-bootstrap
  pipe, and a development-mode hub panics `Bad file descriptor` at once (reproduced directly) while `startHub` keeps polling its HTTP
  wait. Stopped my pids (2047/2049/2050/3749). Drill now boots through `startLocalHub` (pipe on fd 3, credential sign-in) +
  `waitForReadiness(TRUSTED_CATALOG_READINESS_STALL_BOUND_MS)`; stop = SIGTERM + exit wait + `finishLocalHub`.
- 05:28 matrix proof 3 on MY local-only serve 6680 (coordinator-approved; w2-detach pid 9684, vite 9763): census registry 60,
  **loaded 60**, 148 programs, selected 4, installs settled in 0.3 s → **PASS 4/4**: note editor `addBlock` edits [0,1,0,1]
  `Add Block↶`, raster editor `addLayer` [0,1,0,1] `Add Layer↶`, both viewers rendered, 0 faults, chips hit-testable
  (`matrix-proof-3.txt`). Serve stopped 05:3x (pids 9684, 9763 mine; 6680 free).
- 05:35–05:57 drill proof 2 (w2-detach 20351, C11's 19:07 binary + B2 catalog clone, kind note, 20 edits): cold boot ready
  **562.5 s** (load ~100), seed, SIGTERM → exit **590 ms**, `tar` **567 668 736 B in 1 191 ms**, restore to a second root, **134/134
  files sha256-identical**, restored boot ready **62 s**, frontier = descriptor = checkpoint pair = listing = before, next chained edit
  accepted → **PASS 1/1**, scratch roots removed. Seen on the way: 38 `wire str: truncated` from `decodeServerFrame` on `Commands`
  frames — this tree's TS codec expects the new `origin` field the 19:07 binary does not send (tree ahead of binary, expected until
  the rebuilt hub); the probe client now counts undecodable frames (`undecodableFrames`) instead of throwing in the listener.
- 05:45 plugin-coverage proof 1 (note): created OK but 0 mutations/20 empty descriptions → my port had dropped G10's shape: the
  `capabilities_describe` structured content IS the capability (quick law `capabilities_describe_tool_call_returns_the_full_definition`).
  Fixed; proof 2 (note, draw; staged 15:12 `semio-os-mcp`, `--folder` gateway): **PASS 2/2** — draw 18 capabilities / 17 mutations /
  5 destructive / 3 empty descriptions, created + `invoke ok`; note 20 / 18 / 4 / 0, `addBlock` SUCCEEDED (`plugin-coverage-2.txt`).
- 06:0x coordinator: live two-human/user-path/residency only after 7800 is on B3 (the tree's host refuses B2 guests' edit batches
  after LD's wire change); until then robustness + plan wiring + dry runs. Done:
  - **Plan wiring**: new requirement/token `localServe` (`--local-serve`, schema enum + summary field): program-matrix en/de,
    F2's `idle-budget` + `memory-soak` run on a local-only serve (every plugin loaded), hub-joined checks keep `{serve}`; launch
    input `acceptanceLocalServeUrl` (default 6070, the `🔒local-only` row), matrix rows + `⚖️gate🎯️repo-goal` edited in seed +
    launch.json (`wp-v1/v1-launch-edit.py`); parity with the generator re-measured **fresh=true (11 270 lines)**. F2's
    `connection-budget` added (`{serve}`). Plan: 9 steps, 42 checks (`acceptance plan`).
  - **The gate owns check identity**: a harness record is filed under the plan check id (`program-matrix` → `program-matrix-en`/`-de`,
    `hub-boot` → `hub-boot-watch`, …), so several plan checks can share one harness.
  - **No harness ends without its record**: `withAcceptanceRecord(repoRoot, check, body, blockedWhen)` (acceptance module) publishes a
    `fail` — or `blocked` for a missing precondition — record with the error in en + de; used by hub freshness, residency,
    backup drill, program-matrix, two-human, plugin-coverage; user-path's setup is stepwise and stops at the first refusal.
  - **Negative dry runs** (hub 8189 / serve 6689 absent, measured): hub-freshness `blocked` "no process listens on port 8189",
    residency-watch `blocked`, two-human `blocked` (`ERR_CONNECTION_REFUSED`, 19 s, browser closed), user-path `blocked` (row 0
    "hub unreachable", 0.3 s, no browser launched), program-matrix `blocked`; every record schema-valid (`generated/dry-*.json`).
  - **Goal gate end to end** (`acceptance-goal --only compliance`, nx → gate → 3 nx checks → records): placeholders PASS 45 s,
    command-reachability FAIL 4/1165 23 s, dependencies-literal-external FAIL exit 1 in 893 s (literal-external 247, oracle
    conflicts 20 — T13's scope). The run's summary write refused itself: my `localServe` schema edit landed while the gate ran
    with the older code (`/: missing localServe`) — a self-inflicted race, re-run 2 launched 06:22 (pid 53466) on the final code.
- 06:3x **Freshness laws** (`🎯️acceptance/🧪️tests/🏷️hub-freshness/`, synthetic staged executable + sources record, oracle
  `find -newer <reference stamped at build start>`): missing executable / no record / malformed record → unverifiable,
  untouched → fresh (find: none newer), touched → stale (= find's list), vanished → stale — **6/6**. All acceptance laws
  (goal-gate 16, source-census 12, hub-freshness 6) **34/34** (`bun test`); tsc 0 errors in V1 files.
- 06:3x F2's hub-open timings folded into two-human's record: `createToMountedP50Ms/MaxMs`, `openRowToMountedP50Ms/MaxMs`, the
  bounds and `overBound`, en + de summary; new flags `--max-create-to-mounted-ms`, `--max-open-to-mounted-ms` (a kind over a bound
  fails the check) and `--mount-budget-ms` (default 15 min: how long a creation/open may take to mount before the row errors — the
  bounds judge, the budget only stops a hang). Plan: two-human en/de pass **120 000 / 30 000 ms** (create→mounted includes the
  server-owned creation; open→mounted = the hub's 30 s Welcome/reopen bound) — V1's choice, F2/coordinator may tighten.
- 06:4x goal gate compliance re-run 2 on the final code (`s13-v1-logs/goal-gate-compliance-2/`): placeholders PASS 23 s,
  command-reachability FAIL 4/1165 16 s, dependencies-literal-external FAIL 366 s; **summary.json (schema-valid) + summary.en.md +
  summary.de.md written**, exit 1 (verdict fail). German header labels localized afterwards ("Oberfläche", "Lokale Oberfläche").
- 10:5x R9 changed the launch generator (every declared project target registered; `generateLaunchJson(repoRoot, playgrounds,
  declaredProjectTargets(repoRoot))`); V1's tooling now edits the SEED only (`v1-launch-rows.py`, `v1-launch-edit.py`) and renders
  launch.json with the generator (`v1-launch-parity.ts --write`); parity **fresh=true (21 068 lines)** after my 4 new rows.
- 11:xx S16's two remaining acceptance harnesses ported (rule 21, S16's 06:3x versions, S16 confirmed unchanged):
  - `🧑‍💻dev/🧪️tests/⏯️tool-run-matrix/` (`verify tool-run` / `program-matrix --column tool-run`, nx `tool-run-matrix`): the 18
    tool-declaring programs + `mutating` flags in `🧑‍💻dev/🧫️fixtures/⏯️tool-run-matrix.json`; run A (start → progress → pause holds →
    resume → finalize → commit + undo, or read-only finalize) and run B (start → abort → nothing committed); PASS = A meets the
    declared contract AND B aborts clean. Witness helpers reused from program-matrix (exported).
  - `🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/` (`verify hub-sweep`, nx `hub-document-sweep`): one persistent (scratch) profile, kinds
    enumerated live from the staged `createArtifact` choices, sign in → Space Browser open → create by encoded choice → saga wait
    (300 s / 900 s puzzle) → pinned verb (B2 pins now `hubKindVerbs`/`hubKindPre` in the matrix pins fixture) → clean round trip,
    0 faults; `--reopen`, `--cancel install|open`; every notice/creation/execution-target/install band recorded.
  - Plan: `tool-run-matrix-en/-de` (`{localServe}`), `hub-document-sweep-en/-de` (`{serve}`, creds via env tokens). Launch rows
    `⚖️gate⏯️tool-run⚛️react(+🌐️de)`, `⚖️gate🗂️hub-document-sweep⚛️react(+🌐️de)`. Dry runs (absent serve): both `blocked`.
- 11:1x rule 30 (REBUILD START 09:53, no project.json/nx edits until 7800 on B3): my two ~11:00 additive framework-os-dev targets
  (`tool-run-matrix`, `hub-document-sweep`) were flagged to main → **kept** (command-only, not derive inputs); further project.json/nx
  edits held until 7800 is on B3.
- 11:2x repo contract re-run (`s13-v1-logs/contract-3.txt`): **0 findings in V1 files** (incl. the new ⏯️/🗂️ harnesses and the
  🏷️hub-freshness laws). All acceptance laws 34/34. No V1 process or port left (8180–8189/6680–6689 free); shared pg/neo4j up.
- **Waiting** (coordinator 11:2x) for the live-verification wave after 7800 is on B3: then two-human en/de, user-path en/de,
  residency, hub-freshness (fresh case, the rebuilt hub's `os-hub.sources.json`), storm (via `backend-run` for pg/neo4j),
  tool-run + hub sweep (S16 first), and the full `⚖️gate🎯️repo-goal`.
