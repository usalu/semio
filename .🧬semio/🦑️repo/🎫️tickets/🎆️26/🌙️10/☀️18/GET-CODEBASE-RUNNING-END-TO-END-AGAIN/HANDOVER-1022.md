# Handover 10-22 ~22:45 (weekly quota 91 %, resets 10-28 23:00 CEST)

Session rules: `brief-wave-1022a.md` (+ 12:40 addendum: no pkill/killall; check `bun repo leases status` before graph commands). Timeline + agentId table: `coordination.md` from "01:35 (10/22". Audits: `status-1022.md`, `e2e-map-1022.md`, `audit-*-1022.md`, `close-prep-1022.md` (files list for ticket_close).

## State per product
- **Course** (`@teaching/architecture-course`): GREEN on Bazel: `@teaching/proctor:build --configuration=release`, `@teaching/proctor:test`, course `:build`, `:typecheck`, `:test`, `:check` (result-course-build2.md, result-course-rest2.md). E2E driver e2e-1022-course3: proctor-build/typecheck/test PASS, build FAIL (peer added a schema file mid-run). Rerun `e2e-1022-course3b` detached (driver pid 42003, log `🗑️generated/course-rest2/e2e-course3b.log`). Open: Nx-tool path ("Original process admission closed", parent `repo` spin after nx child exit; existing `[DEBUG]` daemon line in the nx patch now prints a stack → remove).
- **Play** (`@semio-tech/semio-tech-play`): dry-run passes; real activation reached 200/201 earlier, latest loop ~58/201 then `os-plugin-describe-rs:build` ("Finite physical artifact Source required"). describe-rs fixed 5 root causes (unverified live). Next: `slice-play3-tools/loop.sh` (result-slice-play3.md top). Plugin Rust chain verified through plugin-extension + instance jobs/docs/render/dispatch; `plugin-instance-app` and note/dwg workspaces not compiled (result-plugin-chain2.md: `chain.sh c13 --update check -p semio-framework-plugin-instance-app`). `plugin-window-config` at 99 % of its source gate.
- **Demonstrator** (`@semio-tech/mit-bestand-demonstrator`): all print steps GREEN (kompaktbericht, bericht ×3, Projektetage deck). Activation best 60/75; act6 queued detached (wrapper pid 23444, `🗑️generated/slice-demo2/act6.raw`). Blockers: leases image `.cache/b/l` 63 MiB fills (sweep fix by describe-rs, unverified), stale `.cache/b/n/m/<hash>/native` images attached.

## Fleet-wide fixes made live today (all in result-*.md)
dep-receipt (inputs + walkStamp), graph-receipt (🔖️stamps, graph-generations-v3), graph-reuse (generation reader registers source parsers — readers no longer re-infer everything; live two-reader proof still TODO, commands in result-graph-reuse.md), stale-impl-guard (🧷️format-stamps, yield + re-exec), loader-own (derived capture + closure law), lease-proof, graph-parallel2 (farm), graph-assembly (host-derived memory budget, streaming), test-dep-fix (withheld test projects), cap-gc (g 6 GiB + collector), rust-image (rust 48 GiB + collector + ENOSPC retry), cargo-home (nested workspaces share root registry), lock-auto (self-repairing Cargo.lock), boot-rebind2 (old native receipts = stale format), course-build2 (Bun 1.4.2 transpiler cache off, null-prototype custody), launcher-zt (bare-machine Go, one help catalogue, pinned bun everywhere), rounds 6 + 7 applied (integrate3, integrate-r7), langs-pin (20 pins), store-split + rust-carve (5 over-gate crates split), rust-port/apps-b/apps-a2 (ColdMutation port), rust-tests/rust-rebase (SealedShared re-baseline + 3 real bugs), boundaries-fix, buildlib-reds.

## Detached runs handed over (no tokens; check/stop)
- course E2E driver pid 42003 (above).
- demo act6 wrapper pid 23444.
- apps-a2 runner (`🗑️generated/apps-a2-x/runner.pid`, last 9473; stop: touch `🗑️generated/apps-a2-x/stop`).
- apps-b driver pid 9464 (stop: touch `🗑️generated/apps-b-1022/stop`; summary `check-summary.tsv`).
- lock-auto live test pid 8710 (must remove the temporary `semio-framework-tool-run` line from the dag-host Cargo.toml; coordinator watcher runs `python3 lock-auto-tools/edit2.py remove` if left).

## Open items (next session order)
1. Run describe-rs build, then play `loop.sh` → serve :6033 → `run.sh play --go`.
2. `plugin-instance-app` → note/dwg workspaces → apps-a2/apps-b workspace tables → plugin TS `:test`.
3. Demo activation (act6) → serve :6029 → `run.sh demo --go`.
4. Course E2E rerun until test-e2e passes; Nx path.
5. Rust reds: os-store-engine derived-operation (fix written), os-store-space-schema index, os-store-tests-02, pack-record-encode-cursor `--2`, guest-contract paired_admission, ComputeJobCompletion close_step after WorkerLost (result-rust-reds-*.md, result-rust-port.md).
6. Stray mounted 16 GiB image `🧰️framework/.🧬semio/🦑️repo/⚡️cache/rust/fixed` (pre cargo-home fix): detach + remove when its holder exits.
7. Ticket hygiene before close: `🗑️generated` ≈ 5 GiB left, 167 `*-tools/staged*` source-copy dirs (AGENTS.md forbids) — see close-prep-1022.md.

## User decisions pending
- VS Code extension: half-applied removal (policy dropped vscode rows, taxonomy digest + extension dir remain) → 18/37 lawcov-laws files red. Keep or remove?
- Play E2E `prepare-e2e` may try to download Chromium (pinned 1248 is installed via `repo tools`; a re-download needs approval if it happens).
8. ts-verify: only `@semio-tech/framework:typecheck` proven on Bazel (rc 0); other 9 targets direct-tsc only; Nx typecheck dies "Original native caller capabilities required" in the daemon child + parent spin (same as course Nx path). Jack LSP worker imports a wasm pkg whose crate is gone → delete the dead worker package or build a wasm bridge over `dsl_lsp` (stale entries in 3 package.json, engines catalog, build-owners, 2 fixtures). `framework-os:typecheck` now dependsOn flow browser-package wasm (untested).
9. demo-verify: Bazel GREEN for demonstrator:test, :canonical-architecture, :test-runtime-catalog, bericht:akteursnetz-validate; detached chain wrapper pid 66743 continues (bericht:check-generated, praesentation-projektetage ×2, bericht:test; summary `🗑️generated/demo-verify/bz1-summary.txt`). Nx RED for every target (same daemon fault as course/ts-verify: "Original native caller capabilities required" / "Original process admission closed" + parent spin) → ONE Nx fix unblocks course, ts, demo Nx lanes.
10. rust-reds-b: os-store-space-schema GREEN (16); pack-record-encode-cursor negative-literal precedence fix written (unrun); os-store-tests-02 compiles, 15 runtime fails (13 store-close stall in ReturnedReads, 2 "native replay requires its retained original preparation issuer"); os-store-tests-01/03..09, sync-tests-1/2, companions, world_model_unit not evaluated.
