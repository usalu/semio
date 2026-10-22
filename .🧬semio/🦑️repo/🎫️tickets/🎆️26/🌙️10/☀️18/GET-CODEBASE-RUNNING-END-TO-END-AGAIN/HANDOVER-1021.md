# Handover 10-21 ~21:20 (weekly quota 99%)

## State
- proctor release build on Bazel: GREEN (`bun repo --tool=bazel run @teaching/proctor:build --configuration=release`).
- architecture-course:build: reaches 6/8; `@semio-tech/assets:build:prototype` action hangs after "build complete / native complete" (0% CPU, never exits) → next fix (lingering handle/child in the assets builder).
- play/demo: graph-artifact OOM fixed (streamed CAS records); play dry-run later ran away post-graph (RSS 11.7 GB, silent) → needs investigation before rerun.
- Live fixes today (loader-fix, see result-loader-fix.md): .cjs default-import contract, captured revision importers, runtimeDataDiscoveryV1 + ComputedImportSites law, planner schema request.authority, createRequire walk, readBazelPathsV1, children load parent's admitted images (live edits no longer fail runs), proctor inputs, bazelOtherModeOutputV1, typegen exporter via launchRepositoryCargoV1 (layering debt: framework module imports product library → move launcher API behind a framework tool port).
- Round 6 (integrate2, round6-plan.md): 34 steps; script.ts split handled via view.py; 4 re-stages owned by integrate2 (cache-gc apply4, lock-sync closure-14, pm-seam-2, backend-contract scriptslices). Round-7 queue in result-integrate2.md.
- E2E driver: `C/e2e-orch-tools/run.sh <course|play|demo|all> --go --run-id <id>` via `python3 C/chain-tools/detach.py <log> sh …` with SEMIO_BUILD_GRAPH_POLICY=current. Preflight parses JSON block + retries ×3.

## Paused agents (resume with SendMessage, ids in coordination.md)
integrate2, wire-fix, fem-finish, graph-parallel, i18n-core, lawcov-{laws,cli,build}, tool-direct-code, audit-fix, gran-hubs, conc-graph (pinning redesign mostly superseded by loader-fix's admitted-image children), cache-gc, bzl-hermetic, boot-fast.

## Open user decisions
- VS Code extension: a peer removed its package.json + workspace entries 10-21 17:18 (lock regenerated); rest of its folder remains.
- legacy-purge dropped edits treating ticket `agents` as legacy log (live schema defines it).

## loader-fix final (2026-10-21 23:3x)
- Loader/closure/planner chain is green: `show projects|targets`, proctor `--dry-run` and real release build exit 0 (details: result-loader-fix.md parts 1-6). Typegen exporter runs through `launchRepositoryCargoV1`.
- assets:build:prototype hang: NOT reproduced. Direct `bun repo --tool=bazel run @semio-tech/assets:build --configuration=prototype` exits 0; after "build/native complete" the action idles ~30-60 s at 0% CPU (bazel snapshot capture/restore, one ESTABLISHED localhost socket to the parent's control listener, no children) and then exits. The 24-min idle seen inside the course build had the same signature but was not caught with lsof/sample; no lingering handle found, so no code change was made (no process.exit hacks).
- Last course build run (23:3x) exited 1 for a different reason: Bazel "missing input file //:a/<hash>/🟦️.ts / 🦀️.rs" for `@semio-tech/ui-contract-rs:generate:prototype` and `@semio-tech/framework-rs:generate:prototype`: input aliases of files that peers edited/removed between graph planning and action start (live-edit churn in the workspace alias store). Next step: rerun when the tree is quiet, or make the planner re-plan on a vanished input alias.

## Design gap found 10-21 ~21:45 (next session)
Bazel reads action inputs from the LIVE tree (`//:a/<alias>` labels), so peer edits between planning and action start fail builds ("missing input file"). Same root as the loader issue: actions must consume a captured, content-addressed source snapshot (the planner's admitted inputs), not the mutable worktree. Fix in the in-house Bazel rules / repo Bazel adapter: materialize declared inputs from the CAS snapshot taken at planning time (bzl-hermetic's aliased-input staging is the natural place).

## e2e-1021l (23:31-23:38): course proctor-build FAIL in the pipeline (passed by hand at ~21:00)
Bazel action `@teaching/proctor:build:release` exited 1 after 221 s; its stderr (1,583,203 bytes) exceeded `--experimental_ui_max_stdouterr_bytes=1048576` and was skipped, so the cause is invisible. Next session, first:
1. Repo Bazel adapter: persist every failed action's stderr/stdout tail (bounded, e.g. last 64 KiB) into the run's log dir and print the tail on failure; never rely on Bazel's UI echo (schema-declared limit, EN/DE message).
2. Rerun `bun repo --tool=bazel run @teaching/proctor:build --configuration=release` and read that tail. Suspects: peer edits between planning and action (see design gap above), or env difference between the e2e driver (SEMIO_BUILD_GRAPH_POLICY=current, detached) and a manual run.
