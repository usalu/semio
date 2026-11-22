# 🗺 Coordination Plan

## Done
- 00:41 Ticket opened (MCP wrote it into the stale `C:\git\semio`; moved to `C:\semio`, the live shared worktree). `repo` fails in `C:\git\semio` (launcher binary built from `C:\semio` expects registry text `namespaceDangling`), works in `C:\semio`.
- Related open ticket: `26/11/20/GET-DEV-FEM-2D-WORKING-END-TO-END` (codex) drives the FEM 2D React dev launch and browser verification.

- 01:00 BLOCKER: C: full (2 GB free; `repo` cargo gate needs 16 GiB). The stale checkout `C:\git\semio` holds 233 GB (`.cache` 55 GB, `.🧬semio` 113 GB, `.git` 31 GB). Moved (not deleted) `C:\git\semio\.cache\b` (last write 2026-11-05) to `G:\semio-offload\git-semio-cache-b` with robocopy /MOVE (log beside it). Reversible.
- 01:10 Audit (`s-loading-audit.md`): fem listed in the s catalog with 2d+3d apps, diagnostics empty, channel 25 = pin. Catalog generated 11-19, descriptor rewritten 11-21 → regenerate catalog later. Fleet does not dispatch builds (round 31) → native host only.

- 01:25 All `repo cargo` runs sit 10–15 min in preparation (`native-orchestration … synchronize`, 200–600 CPU-s each) before rustc → dedicated agent (`cargo-synchronize-performance.md`).
- 01:30 Dev route (`dev-launch-route.md`): fem2d on 6086 (codex Vite, PID 32260), fem3d `dev-fem3d-react-prototype` on 6087 (not running). Served component is the stale Nov 20 publication (disposer refusal); canonical publication blocked by stale descriptor emitter (`--prepare-actor`). Browser phase waits for green Rust + fresh component publication.

- 02:10 C: 46 GB free (move ongoing). `repo cargo` build-entry stuck (engine 53 min) → 2D/3D/engine agents switched to native cargo with short target dirs `G:\t\{2d,3d,s}`; plugin root already native. TS (`typescript.md`): fem-js 62/0, fem-2d/3d type-check+tests, 6 dev verifies ✅; open: microcursor pins (44), live-visual-publication, surface-contract OBJ/STL, builds → TS agent continues.

- 02:40 C: at 0 again: `.cargo/config.toml` `build-dir` (`.cache/b/r/m/u/l/d/b`) received native builds' intermediates → os error 112 surfaced as silent "could not compile". All native agents now set `CARGO_BUILD_BUILD_DIR=G:\t\<x>b` (`native-env.sh`). Moving stale `C:\git\semio\.🧬semio\🦑️repo\⚡️cache` (34.5 GB, last write 11-05) to `G:\semio-offload\git-semio-repo-cache`.
- 02:45 ✅ plugin root: check --all-targets in root, 📍node, 🧱️material, 🏭️bridge; root tests pass. Fixed shared receiver-driver harness (nested-turn metadata grant), sqlite test API, material unboxing, natural-publication `terminal_is_empty`. Open: node planar/volumetric tests, material re-run.
- 02:50 ✅ synchronize body ~7x faster (quadratic ancestor lstat, repeated walks, MSVC rehash). Continuing: shared preparation lease, scoped publish walk.

- 03:00 ✅ engine: 17 crates check --all-targets natively; tests running. TS: pin laws replaced by behavioural cargo tests (invariant map in `typescript.md`), OBJ/STL corpus producer test written (unrun), fem-2d/3d bundle+declarations pass via `repo tools exec`. Open: framework `framework-reserved-exact` verify still calls the deleted text law.
- 03:00 INCIDENT: TS agent ran `taskkill /F /IM repo.exe` → killed other sessions' repo processes incl. codex's 6086 Vite server and fem3d native preview. Fleet replica self-restarted. Rule added (kill only own PIDs). 6086 to be relaunched in the browser phase.

- 03:20 ✅ preparation: shared lease when receipt current, scoped publish walk (126 s → 0.04 s), seeded listings (19.5k → 63 readdir). Open: concurrency test blocked by `Closed host capacity Policy required: /swap/activeBasisPointsPerSecond` → same agent; witness-launcher st_dev oracle.
- 03:25 2D (`artifact-2d-rust.md`): libs + editor_app compile; test fixtures lag. Split: 2D-A (snapshot-tests, sqlite, results-state, editor bins, whole-workspace test) → `artifact-2d-rust-a.md`; 2D-B (interaction, gumball, preparation laws) → `artifact-2d-rust-b.md`.

- 03:40 C: at 0 a third time. Culprits: another session's private build dir `C:\cargo-nr\b` (96 GB, trinity, active — left alone) and old Claude shell-output logs of a Nov 18–19 session (`%TEMP%\claude\…\ab112164…\tasks`, >11 GB) → moved to `G:\semio-offload\claude-temp-ab112164-tasks`. C: 59.8 GB free. Rule added: TMP/TEMP/TMPDIR on G:, CARGO_INCREMENTAL=0.

- 04:30 ✅ witness oracle (Windows st_dev/ctime semantics) fixed. Queued: shared-lease concurrency test `@semio-tech/cargo-workspaces:original-control-contract-check` (cold graph slice >1400 s at 533/3600 owners).

- 05:40 ✅ node tests 2/2; material 10/14 (in progress). ✅ engine p6h tests (ldlt/subspace fuel accounting fixed). 2D-B: interaction tests type-check; editor crate checking. ✅ 3D: all libs + framework compile; only `semio-s-artifact-fem-3d-editor` test targets stale (lib 8, editor_app_2 68, editor_app 170) → 3D agent (lib + editor_app_2), new agent (editor_app, `artifact-3d-editor-app.md`). TS items handed to a fresh agent (old one near context limit). Framework UI-layout refactor by another session caused transient app-builder breakages (resolved by itself).

- 07:00 C: 16 GB and falling: peer session "Trinity and Jack end-to-end" build dir `C:\cargo-nr` 120 GB (+25 GB/3 h). Messaged that session to move it to G:. 2D-A at whole-workspace test; 2D-B tests building; 3D ports compiling.

- 07:10 Trinity session emptied `C:\cargo-nr` (C: 124 GB free) and moved its builds to G:.
- 07:25 HANG: engine test `semio-s-fem-analyses` `fem_job_graph_checkpoint_resume_preserves_stage_order` spun 3.5 h (9.7k CPU-s) unnoticed; killed PID 27740; engine agent root-causing + bounded-turn assertion + per-test watchdog.

- 07:40 Engine hang root cause: tests used bare `StepContext::new` (no original parent source supplier) → Fault outcome looped; tests moved to `OriginalNumericalFixture`, legacy free test-support helpers removed. Libraries compile everywhere → publication agent started early (descriptor regen, s catalog, republish, dev servers 6086/6087) → `fem-publication.md`.

## Wave 1 (running)
| Agent | Model | Scope | Report |
|---|---|---|---|
| plugin root | Sonnet | fem root workspace, 📍node, 🧱️material, 🏭️bridge | `plugin-root-rust.md` |
| artifact 2d | Sonnet | `🗿️artifacts/◻️2d` workspace incl. editor_app fixtures | `artifact-2d-rust.md` |
| artifact 3d | Sonnet | `🗿️artifacts/🧊️3d` workspace | `artifact-3d-rust.md` |
| engine | Sonnet | `✏️s/🔨️modules/🏗️fem/⚙️engine` crates + oracles | `engine-modules.md` |
| typescript | Sonnet | all fem TS projects, dev fem, hub composition | `typescript.md` |
| audit | Haiku | s catalog/descriptor state, launch targets, fleet status | `s-loading-audit.md` |

## Queued
- Descriptor regeneration (`describe fem`) + s catalog generate + catalog-check, once Rust is green.
- FEM 3D dev launch end to end in the browser (edit, calculate, display), mirroring the 2D ticket.
- Final audit: `[DEBUG]` sweep, generated logs cleanup, ticket close.
