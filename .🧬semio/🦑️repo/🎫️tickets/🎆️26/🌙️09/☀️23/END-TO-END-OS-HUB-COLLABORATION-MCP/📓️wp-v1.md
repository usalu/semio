# WP-V1 — Acceptance Harnesses As Permanent, Zero-Touch Commands

Slice V1 (session 13). Source: [📓️acceptance-s13.md](📓️acceptance-s13.md) §6 (NO HARNESS), §7 (not zero-touch), §8 (final
verification plan). Ports: hubs 8180–8189, serves 6680–6689. Scripts/codemods `wp-v1/`; expendable captures `wp-v1/generated/`;
durable data `.🧬semio/🌐hub/s13-v1-*`. Builds/tests prefixed `nice -n 10`.

## Session 13

| # | Item | Status | Evidence |
|---|---|---|---|
| 1 | Zero-touch postgres + neo4j backends (`📜️script.ts` verb, used by the pg/neo4j gates, launch rows) | **DONE (harness measured)**; full two-client pg run needs a current-tree all-driver `os-hub` (the only one, H9's 15:33, refuses today's catalog) | `backend-up-1.txt`, `backend-oracle-1.txt`, `claim-probe-1.txt`, `s13-v1-logs/two-client-postgres-2.txt` |
| 2a | editor/viewer/locale matrix → `@semio-tech/framework-os-dev:program-matrix` (`verify matrix`) | code landed, tsc 0 errors; **live run pending** (memory rule 22: swap 18.8/20 GB) | `🧑‍💻dev/🧪️tests/🧮️program-matrix/` |
| 2b | two-human orchestrator → `@semio-tech/framework-os-dev:two-human` (`verify two-human`) | code landed, tsc 0 errors; live run pending | `🧑‍💻dev/🧪️tests/👥️two-human/` |
| 2c | `@semio-tech/framework-os-mcp-rs:plugin-coverage-check` + `user-path-check` | code landed, tsc 0 errors; live run pending | `🌉️mcp/🧪️tests/🧩️plugin-coverage/`, `🚶️user-path/` |
| 2d | `os-hub-ts:backup-restore-drill`, `os-hub-ts:residency-watch`, `@semio-tech/framework-os-kernel:reopen-storm-check` | code landed (hub tsc: 0 errors in my files); runs need a hub boot / cargo test (rule 25: coordinator approval) | `🌎️hub/🧪️tests/💾️backup-restore/`, `🧠️residency/` |
| 2e | command-reachability census + `unimplemented!()`/`todo!()` production census as gates | open | |
| 2f | `os-hub-ts:hub-freshness` (Cargo freshness over the `os-hub.sources.json` record the staging verbs now write) | **measured** on hub 8021: `unverifiable` (C11's copied binary has no record) — correct verdict | `generated/freshness-8021.json` |
| 3 | goal gate `@semio-tech/repo-test-domain:acceptance-goal` (+ `acceptance-plan`) | schema + plan + runner landed; laws **15/15 pass** (Ajv 2020 oracle); live run pending | `🧪️test/🎯️acceptance/` |

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
