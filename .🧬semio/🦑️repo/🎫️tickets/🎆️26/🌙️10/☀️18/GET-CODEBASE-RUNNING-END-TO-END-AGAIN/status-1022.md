# Status 10/22 (read-only audit, auditor status-audit)

Snapshot: 2026-10-22 about 01:45 local. Wave 1022a was launched at 01:40 (coordination.md:737); its result files (bzl-tail, bzl-snapshot, slice-play, slice-demo, integrate3, rust-health, course-rest, graph-perf) did not exist yet; only their tool folders did.

Read: `HANDOVER-1021.md` (all), `coordination.md` lines 440-738 (the 10/21 entries; the file has no 10/21 heading), `report-e2e-orch-e2e-1021l.md`, `brief-wave-1022a.md`, `round6-plan.md` (status blocks), and the status lines, headers and apply commands of all 64 `result-*.md` / `audit-*.md` matched by `find -newermt 2026-10-21`. Bodies were read only where the status was unclear. No apply tool was run (dry runs can write backups), so every state below is from the documents and is marked UNVERIFIED where no later evidence exists.

Legend for section 2: APPLIED = in the live tree per a dated entry; READY = verified on overlays, applicable now under the 10/22 live-edit rule; STAGED = not yet verified against the current tree or waiting on another set; BLOCKED = a known red in the current tree.

## 1. Open items

| ID | Open item | Evidence (file:line) | Files / area | Blocks | Owner type |
|---|---|---|---|---|---|
| O1 | Failed Bazel action stderr is invisible: `@teaching/proctor:build:release` exits 1 after 221 s, 1,583,203 bytes of stderr exceed the 1 MiB UI cap, cause hidden. Course cold proctor-build FAIL 6m08s | HANDOVER-1021.md:26-29; coordination.md:731; report-e2e-orch-e2e-1021l.md:39 | Repo Bazel adapter (`🏗️build/🔧️tools/🏗️bazel`), e2e-orch | course | Sonnet executor (bzl-tail, agent ab94dab0086ae3345) |
| O2 | Bazel actions read the live tree (`//:a/<alias>` labels); peer edits between planning and action give "missing input file" for `ui-contract-rs` and `framework-rs` generate. Design gap, not patched | HANDOVER-1021.md:23-24; coordination.md:730 | Bazel adapter, planner input snapshot (CAS) | all (play and demo also run on Bazel) | Sonnet executor (bzl-snapshot, agent aff96d711b1c12e1d), coordinator review |
| O3 | `@semio-tech/assets:build:prototype` idles 30-60 s at 0% CPU after "build/native complete" (once 24 min). Not reproduced, no fix | HANDOVER-1021.md:5, 20; coordination.md:730 | assets builder; architecture-course build (6/8 steps) | course | Sonnet executor, repro under e2e first |
| O4 | Course steps after proctor-build have not run since 10/21: typecheck, test, build, check, proctor-test, test-e2e (all SKIP-FAILED) | report-e2e-orch-e2e-1021l.md:39 and course table | course products | course | Sonnet executor (course-rest, agent a1aacf8b0a5bc098b) |
| O5 | Play dry-run runaway after `build:graph` complete: 67 min silent, RSS 9.8 to 11.7 GB, killed by the fleet | coordination.md:714 | post-graph build:modules and command-source-packages | play | Sonnet executor (slice-play, agent ad5e74901135bf110) |
| O6 | Play and demo have no green cold or e2e run since the 10/21 fixes. Last recorded: play OOM 17:20 and demo OOM 17:23 (graph OOM fixed 18:52, not re-run for these products), demo FAIL on frozen lock 17:31 | coordination.md:698-699, 703, 713 | graph and run/exec | play, demo | Sonnet executor (slice-play, slice-demo agent ab6337a74eb446914) |
| O7 | Round 6 blocked: six owner-level reds from the 20:00 pass: (a) cache-gc apply4 `build-script4[0]`, (b) lock-sync `closure-14`, (c) pm-seam-2 test file missing, (d) backend-contract `scriptslices.py` anchor, (e) cli-latency two dropped hooks, (f) ide-free `[agent-client]` not gofmt-clean (live Go edit) | round6-plan.md:55-63; result-integrate2.md:169-170 | `🏗️build/📜️script.ts` was split into `⌨️command/*` at 18:56; owner sets | all (round 6 carries tool-iface, runtime-seam, legacy-purge, pm-seam, storage-seam, repo-launcher, cache-gc) | integrate3 (agent a0080bee08256357d) and Sonnet owners |
| O8 | Round 6 cascade: rexec-inhouse needs rcache-server; runtime-seam before tool-iface; polyglot-units needs the tool-iface git adapter; tool-iface registry messages not in its schema | round6-plan.md:30, 63; coordination.md:652; result-integrate2.md:172 | tool-iface, runtime-seam, rcache-server, polyglot-units | all | Sonnet executors |
| O9 | Round 5 lock batch never executed. Only a coordinator `dependencies lock --scope .` ran (17:29-17:34), which fixed the frozen-lock failure but did not apply the lock-sync batch (six scopes, new generations) | result-lock-sync.md:3, 39; coordination.md:705 | six lock scopes, `nx`/`bazel` root rows | all | coordinator |
| O10 | Pinned bun 1.4.2 is not installed (host 1.3.14). `bun test` segfaults on 1.3.14. Zero-touch provisioning (runtime registry) lives in repo-launcher, which is staged and not applied | package.json:145; report-e2e-orch-e2e-1021l.md:17; coordination.md:693, 717; result-repo-launcher.md:3 | runtime provisioning, launcher | all | Sonnet executor (apply repo-launcher) |
| O11 | Graph image `g` at or below its 1 GiB floor (822 MB, 815 MB, 990 MB, 1.0 GiB recorded). Every writer then queues for exclusive headroom maintenance (convoy). Recreate to 6 GiB (`--recreate-graph`) and the incremental proof (staged4) not applied; staged3 blocked by drift | coordination.md:651, 659, 669; result-cache-gc.md:48; result-integrate2.md:185 | `.cache/b/g`, cache-gc tools | all (every cold run) | Sonnet executor (cache-gc) |
| O12 | Peer live edits abort running builds ("Loaded build graph source changed", "Captured implementation Source differs", SyntaxError from a peer loader rewrite). Fixed for loader (22:25) and graph pin (03:17), not proven under a full e2e | coordination.md:576, 580, 715, 719, 725-726 | build library, closure loader, graph pin | all | coordinator (verify in e2e) |
| O13 | graph-parallel round 2 BLOCKERs: workers write leaves and sealed blocks into parent custody; duplicate-leaf race; 1 h lease safety releases capacity under a live build. Result doc not updated since 15:15 | audit-staged-1021b.md:11, 23-24, 31; result-graph-parallel.md:3 | graph inference farm | none (speed only) | Sonnet executor (graph-parallel) |
| O14 | cache-gc apply tools: drift check skipped when a file has no recorded base; `--restore` writes backups without hash check | audit-staged-1021b.md:49-50 | `cache-gc-tools/apply.py`, `apply2.py` | none (needed before O11 apply) | Sonnet executor |
| O15 | audit-live1021 #7: triage preview still runs bun directly and drops the refusal reason. #10: the 14:49 "Inference reference text capacity exceeded" fix is not proven (16 MB UTF-16 per call, not per project) | audit-live1021.md:47-50, 62-65; result-audit-fix.md:8 | triage preview; graph observation chunking | play, demo (both hit 14:49) | Sonnet executor (audit-fix) |
| O16 | Triage-r4: two fixes staged, not applied (`apply-staged.sh` pending post chain17) | result-triage-r4.md:3; coordination.md:533 | source-demand and build-surfaces suites | none | Sonnet executor |
| O17 | Live break at 20:14 (loader export missing for `.cjs`) fixed at 21:23, but the same class of break recurs with each peer loader rewrite | coordination.md:715, 719 | workspace owner `bun` loader | all | coordinator |
| O18 | CI still calls `cache remote status` at `build-system.yml:167` (rcache-server renamed it to `remote-cache status`) | coordination.md:530 | `.github/workflows` | none (CI only) | Sonnet executor after rcache-server |
| O19 | VS Code extension: a peer removed its package.json and workspace entries; location decision D1 open (keep as thin client or move out). Goal `REPO-VSCODE-EXTENSION` is past due (must not be opened or closed without the user's instruction) | HANDOVER-1021.md:15; audit-ide-1021a.md:36, 45 | 6 root manifests, `bun.lock`, extension folder | none (frozen-lock risk) | user decision |
| O20 | ide-free: 61 permanent deletes (59 are extension files) need explicit user approval; R4 made the extension group opt-in | audit-staged-1021a-r2.md:39, 61, 68; result-ide-free.md:3 | ide-free-tools | none | user decision |
| O21 | Ticket C holds 1,355 staged or backup copies of tracked source files (AGENTS.md forbids source copies in tickets); 21 of the 42 over-240-char paths come from them | audit-xplat-paths.md:72, 98; coordination.md:684 | `C/*-tools/staged*` | none | coordinator at close |
| O22 | Windows and Linux never executed: Docker down; Linux suites only read statically; 42 tracked paths over 240 chars; Bazel Windows execroot overflow (18,764 to 47,600) | result-xplat-linux-fix.md:5; audit-xplat-paths.md:98; result-bzl-paths.md:3, 31-32 | cross-platform | none on macOS; all for cross-platform acceptance | Sonnet executor (bzl-hermetic staged-paths) |
| O23 | Legacy leftovers still in code: tool-iface `after/0034` (`LEGACY_ENGINE_BIN`, `ensureLegacyBuilt`, `generateLegacy`); rcache-server schema is a `.txt` JSON fragment; hot-cache dev/bench uses `/tmp` and macOS-only paths | audit-staged-1021a-r2.md:34, 40, 71; audit-staged-1021a.md:109, 208 | tool-iface, rcache-server, hot-cache | none | Sonnet executor |
| O24 | Non-TS toolchains not at pins (Go, .NET, cmake, ninja, uv); langs-conc unapplied; go.work 1.25 vs pinned 1.26.9 | audit-langs-1020e.md (summary); coordination.md:555, 565; result-langs-conc.md:51 | languages | none (confirm for course and play) | Sonnet executor (langs-conc) |
| O25 | i18n: about 7,678 English prose sites; six resolvers default to `en` or first language. i18n-core (round 7) unapplied | audit-i18n.md (summary); result-i18n-core.md:3 | repo CLI | none | Sonnet executor |
| O26 | Test debt: 423 NOTEST units (39,659 lines) and 60 NOCORPUS units; lawcov sets staged | audit-lawcov.md (summary); result-lawcov-build.md:3 | build library tests | none | Sonnet executor |
| O27 | Docs refresh (`refresh.sh --write`) not run for docs-1021a | result-docs-1020e.md:53; coordination.md:642 | EN/DE build guide | none | Sonnet executor |

## 2. Staged change sets

| Set | State | Evidence | Apply command (dry run first) |
|---|---|---|---|
| mem-gov (round 1 and 2) | APPLIED (live 10:12) | coordination.md:625 | none |
| conc-tests (round 1 and 2) | APPLIED (live 20:42, round 2 09:09) | coordination.md:465, 602 | none |
| cargo-fix static set (77 files) | APPLIED; compile steps partly green (lock, value, retained, proctor, course libs), steps 088-114 queued | coordination.md:471, 624; result-cargo-fix.md:99 | `bun C/cargo-fix-tools/plan/gen.ts`, then the compile window (result-cargo-fix.md:62) |
| course-bzl | APPLIED (22:55; deploy suite 37/38) | coordination.md:501 | none |
| round 3 (integrate2, 22 groups, 436 changes) | APPLIED (22:51) | coordination.md:500 | none |
| round 4 (373 changes: agents-fix, fnsplit4a-finish, cli-zt2, docs, ci-bazel, bzl-tests-rest, xplat-linux-fix, conc-ts) | APPLIED (00:13) | coordination.md:526 | none |
| conc-leases base plus fix2 to fix8 | APPLIED (live, 23:01 to 15:22) | coordination.md:502, 508, 522, 589, 646, 663 | none |
| conc-deps groups 1 to 3 | APPLIED (groups 1-2 live, group 3 at 00:13); `all` not re-verified | result-lock-sync.md:10; coordination.md:526 | `python3 C/conc-deps-tools/apply.py check`, then `apply --group all --write` (result-conc-deps.md:43-45) |
| conc-graph (READY3 plus live fixes) | APPLIED partly (pin-fix 03:17, image-first order 04:57, slice-fix 14:42, OOM fix 18:52); READY3 47 ops UNVERIFIED | coordination.md:580, 588, 653, 713; result-conc-graph.md:3 | `sh C/conc-graph-tools/go.sh --write`, then `measure-after-go.sh` (result-conc-graph.md:71) |
| conc-nx | READY (UNVERIFIED) | result-conc-nx.md:3, 75 | `python3 C/conc-nx-tools/apply.py apply --write` (conc-cargo lease and lane groups first) |
| conc-bazel | READY (UNVERIFIED) | result-conc-bazel.md:50-55 | `python3 C/conc-bazel-tools/apply.py --root <checkout> --group all` |
| conc-cargo | APPLIED partly (scheduler deadlock fix live 04:49); set 1 and buildscript/broker groups UNVERIFIED | coordination.md:488, 586 | `python3 C/conc-cargo-tools/apply.py` set 1, then `--groups buildscript,broker` (coordination.md:488) |
| conc-harness | APPLIED early (22:51, sim accident) | coordination.md:500 | none |
| gran-keys | APPLIED early (22:51) and fixed live 23:40 | coordination.md:500 | none |
| gran-value (577 files, apply2) | READY (value group compiled 10:1x, so unblocked) | coordination.md 22:19 entry (value group compiled 10:1x) | `python3 C/gran-value-tools/staged/apply.py`, then `staged/apply2.py` |
| gran-hubs | Declined 23:30 (not applied); still listed in the round-7 queue | coordination.md (23:30 entry); result-integrate2.md:174 | do not apply |
| iface-wrap (250 files) | APPLIED partly (react group 10:15); rest STAGED | coordination.md:626; result-iface-wrap.md:3 | `python3 C/iface-wrap-tools/apply.py` (remaining groups) |
| fem-finish | APPLIED partly (engine and analyses green 17:45); os-store blockers | coordination.md:709; result: 21:28 entry | apply with the compile window (no command recorded) |
| wire-fix | APPLIED live (edits 15:07-17:45); document, composition, durable-group still red | coordination.md:658, 668; result-wire-fix.md:57-59 | `cargo update -p semio-framework-os-store-durable-group-wire` (result-wire-fix.md:59) |
| repo-mcp-build | APPLIED live 02:27 | coordination.md:570 | none |
| input-precision | APPLIED (round 2, 10-20 18:29), later fixes live (02:46, 04:21, 06:14) | result-input-precision.md:3; coordination.md:575, 584, 593 | none |
| graph-speed | APPLIED live (19:0x); speed2 APPLIED 10:07; followup spec STAGED | result-graph-speed.md:3, 76; coordination.md:623 | `python3 C/graph-speed-tools/apply.py apply --spec followup --write` |
| ci-bazel | APPLIED in round 4 (13 changes) | coordination.md:526 | none |
| bzl-tests-rest | APPLIED in round 4 (26) | coordination.md:526 | none |
| xplat-linux-fix | APPLIED in round 4 (66); Windows follow-up live; Linux UNVERIFIED (no Docker) | coordination.md:526; result-xplat-linux-fix.md:5 | none |
| cli-zt2, agents-fix, fnsplit4a-finish, conc-ts | APPLIED in round 4 | coordination.md:526 | none |
| pets-fix, rust-sync | APPLIED live (GREEN) | coordination.md:598, 599 | none |
| boot-fast | APPLIED partly (caller 17:42, validator 17:44); door, launcher, slo STAGED | coordination.md:707-708; result-boot-fast.md:124-125 | `python3 C/boot-fast-tools/staged/apply.py --write --group launcher,slo` (after repo-launcher and perf-slo) |
| audit-fix | APPLIED partly (live 15:32); #7 and #8 STAGED for round 6 | coordination.md:666; result-audit-fix.md:8 | per result-audit-fix.md |
| storage-seam | APPLIED partly (live 11:59 and 16:14); 3 groups, 128 ops STAGED | coordination.md:638, 670; result-storage-seam.md:5, 43 | `python3 C/storage-seam-tools/apply.py` (dry), then `--write` |
| cache-gc base (7 patches, 11 new) | UNVERIFIED (live edits 15:26, 16:15 made parts) | coordination.md:664, 671; result-cache-gc.md:31 | `python3 C/cache-gc-tools/apply.py`, then `bun C/cache-gc-tools/regen-help.ts` |
| cache-gc staged2 (apply2) | STAGED (UNVERIFIED) | result-cache-gc.md:45 | `python3 C/cache-gc-tools/apply2.py --write` |
| cache-gc staged3 (apply3, recreate) | BLOCKED (drift in gc-runner.ts) | coordination.md:710; round6-plan.md:50 | `python3 C/cache-gc-tools/apply3.py`, then `recreate.py` (g 4 to 6 GiB) |
| cache-gc staged4 (apply4) | BLOCKED (round 6 owner item 1) | round6-plan.md:57; result-integrate2.md:185 | `python3 C/cache-gc-tools/apply4.py --write` |
| loop-hunt | STAGED (10 anchored ops; round 3 status UNVERIFIED) | result-loop-hunt.md:25 | `python3 C/loop-hunt-tools/apply.py apply --write` |
| sync-incr (incl. sync-incr-3) | APPLIED partly (recipe v6 live); sync-incr-3 STAGED in round 6 | result-lock-sync.md:10; result-sync-incr.md:45 | `python3 C/sync-incr-tools/apply.py apply --write` |
| lock-sync (round 5) | STAGED, not run; ROUND5 GO pending | result-lock-sync.md:3 | `python3 C/lock-sync-tools/run.py --yes` |
| round 6 (integrate2, 30 steps) | BLOCKED (owner items O7, O8) | round6-plan.md:48-63; result-integrate2.md:169-170 | the ordered list in round6-plan.md:5-14 (integrate3 owns the rerun) |
| tool-iface | BLOCKED (registry messages vs schema; round 6) | result-tool-iface.md:49-50; coordination.md:652 | `python3 C/tool-iface-tools/apply.py --write` (dry run first) |
| runtime-seam | STAGED (READY2; round 6 step 1 of the chain) | result-runtime-seam.md:63 | `cd C/runtime-seam-tools && python3 manual-gen.py && python3 place.py && nice -n 10 bun apply.ts` (dry), then `--write` |
| legacy-purge (242 files) | STAGED (READY3; after tool-iface) | result-legacy-purge.md:7-9 | `python3 C/legacy-purge-tools/apply.py`, then `--write` |
| pm-seam round 1 and 2 | STAGED (inside round 6; pm-seam-2 owner item) | result-pm-seam.md:51; result-pm-seam2.md:40; round6-plan.md:59 | `python3 C/pm-seam-tools/apply.py check --round 2` |
| pm-seam round 3 (re-cut) | STAGED (READY3) | result-pm-seam3.md:12 | `python3 C/pm-seam-tools/apply.py apply --round 3` (dry), then `--write` |
| backend-contract (READY4) | BLOCKED (scriptslices owner item) | result-backend-contract.md:5; round6-plan.md:60 | `python3 C/backend-contract-tools/apply.py` (dry), `--write`, then `sh C/backend-contract-tools/recut.sh <root>` |
| polyglot-units (READY3) | BLOCKED (needs tool-iface git adapter) | result-polyglot-units.md:3 | `python3 C/polyglot-units-tools/apply.py` (dry run first) |
| rcache-server (READY2) | STAGED (round 6; after cache-gc-4 and lock-sync) | result-rcache-server.md:6; round6-plan.md:63 | `python3 C/rcache-server-tools/apply.py --write` |
| rexec-inhouse | STAGED (needs rcache-server) | result-rexec-inhouse.md:5, 57 | `python3 C/rexec-inhouse-tools/apply.py` (dry), then `--write` |
| repo-launcher (READY3) | STAGED (round 6) | result-repo-launcher.md:15 | `python3 C/repo-launcher-tools/apply.py [--group ...] --apply` (dry run default) |
| repo-doctor | STAGED (after repo-launcher) | result-repo-doctor.md:34-35 | `python3 C/repo-doctor-tools/apply.py --write --regen-help` |
| agent-session (READY2) | STAGED (round 6 step) | result-agent-session.md:56, 90 | `python3 C/agent-session-tools/apply.py --root . --group core,routing,nx-watchers` (dry), `--write`; `service` after hot-cache |
| hot-cache (READY4) | STAGED (round 6 step; audit blockers cleared) | result-hot-cache.md:11, 117-119 | `python3 <hot-cache staged dir>/apply.py` (dry), then `--write` |
| perf-slo | STAGED (round 6 step) | result-perf-slo.md:5 | `python3 C/perf-slo-tools/apply.py` (dry), then `--write` |
| cli-latency | BLOCKED (owner item 5: two dropped hooks) | result-cli-latency.md:50-60 | `python3 C/cli-latency-tools/staged/apply.py`, then `apply-profile-hooks.py --write` |
| graph-mem (READY2) | STAGED; round-6 copy is stale (old violations) | result-graph-mem.md:89; audit-staged-1021a-r2.md:48 | `sh C/graph-mem-tools/go.sh --write` |
| graph-parallel (round 2) | BLOCKED (O13) | audit-staged-1021b.md:11 | `sh C/graph-parallel-tools/go.sh --write` only after the blockers |
| langs-conc (READY3) | BLOCKED (go-plan-env anchor red) | result-langs-conc.md:51; coordination.md:652 | `python3 C/langs-conc-tools/apply.py` (dry), then `--apply --coordinator-go` |
| i18n-core (round 7) | STAGED | result-i18n-core.md:8-11 | `python3 C/i18n-core-tools/apply.py --root /Users/ueli/Documents/semio` (dry), then `--yes` |
| ide-free (READY4) | READY for default groups; extension group opt-in (user decision) | result-ide-free.md:10-13 | `python3 C/ide-free-tools/apply.py --write` (default groups only) |
| lawcov-build | STAGED (round 7) | result-lawcov-build.md:77 | `python3 C/lawcov-build-tools/staged/apply.py`, then `--apply` |
| lawcov-cli | READY (tests only; verifies manifest) | result-lawcov-cli.md:108 | `python3 C/lawcov-cli-tools/staged/apply.py` |
| lawcov-laws | STAGED (round 7) | result-integrate2.md:183 | registry patches under `C/lawcov-laws-tools/staged` |
| tool-direct-code | STAGED (after round 6) | result-integrate2.md:177; result-tool-direct-code.md:3 | `python3 C/tool-direct-code-tools/apply.py apply --write` |
| tool-direct-manifests | STAGED (after round 6; round-6 defect: `repo repo` codemod) | coordination.md:706; result-tool-direct-manifests.md:55 | `sh C/tool-direct-manifests-tools/regen.sh <final tree>`, then `python3 C/tool-direct-manifests-tools/apply.py apply --write` |
| bzl-hermetic staged-paths | READY (round 7) | result-bzl-paths.md:28 | `python3 C/bzl-hermetic-tools/staged-paths/apply.py --write` |
| triage-r4 (2 fixes) | STAGED | result-triage-r4.md:3 | `sh C/triage-r4-tools/apply-staged.sh` |
| docs-1021a | STAGED (refresh not run) | result-docs-1020e.md:53 | `sh C/docs-1020e-tools/refresh.sh --write` |

## 3. Environment gaps

- bun: host 1.3.14 (`/Users/ueli/.bun/bin/bun`) vs `packageManager` bun@1.4.2 (package.json:145). Pinned 1.4.2 not installed. `bun test` segfaults on 1.3.14 (coordination.md:717).
- Go: host go1.25.0 darwin/arm64; go.work `go 1.25`; langs-conc pins 1.26.9 (coordination.md:555, 562, 565). Not provisioned.
- Python: `/usr/bin/python3` 3.9.6; langs-conc pins 3.14 via uv (result-langs-conc.md:61-62).
- uv: host 0.11.15 vs pin 0.11.6 (coordination.md:565).
- .NET: host SDK 10.0.300; global.json pin SDK 8.0.421 (coordination.md:555, staged).
- cmake 4.3.4 vs pin 4.4.4; ninja 1.13.0 vs pin 1.13.2 (coordination.md:555).
- Rust: host default rustc/cargo 1.99.0-nightly dated 2026-07-19; repo-root `rust-toolchain.toml` pins nightly-2026-07-20. Provisioning through `repo` to be confirmed.
- Bazel: not on the host PATH; `.bazelversion` 8.4.2 is provisioned by repo (report-e2e-orch-e2e-1021l.md:18).
- Node 24.15.0; git 2.54.0; ripgrep 15.1.0; playwright CLI 1.64.1 with chromium revision 1248 installed (report-e2e-orch-e2e-1021l.md prerequisites table).
- Docker: not running (`docker version` returned no server). Linux and Windows runs therefore never executed (result-xplat-linux-fix.md:5).
- Disk: data volume 209 GiB free of 926 GiB (77% used) at 01:35. Root and data are on separate volumes (`df`).
- Memory: 32 GB RAM, 10 cores. Swap 2.7 GB used of 4.0 GB at 01:35. Capacity gate 17.4 GB available of 32.8 GB (report-e2e-orch-e2e-1021l.md prerequisites table).
- Load: 27.3 / 16.2 / 10.4 (1, 5, 15 min) at 01:35. Wave 1022a is above the one-heavy-process rule: `bun repo show projects` (pids 30260, 30735), `bun repo cargo ... check --workspace --all-targets` (30376), two `bun --eval` workers at 95-98% CPU (30864, 31763), and two repo-wide greps started by agents (pids 31457 `grep -rl ... .`, 31771 `grep -l graph-phase -r .`). Also Spotlight `mds` (pid 132, 54%), Claude Helper (pid 3726, 91%), GitKraken renderer (pid 30581, 57%).
- Codex peer pid 37466: `bun ... repo run semio-framework-os-flow-browser-package:test-producer-source --skip-cache --output=stream`, parent 1181 (Codex app-server), elapsed 21 h 38 min, 49.8% average CPU. It has held shared images idle since 10/21 04:53 (coordination.md:587). Left alone per brief-wave-1022a.md. Other 10/21 Codex graph and demonstrator processes: 40609 ended 05:05 (coordination.md:589), 64604 ended 08:56 (coordination.md:601), 49036 status not recorded after 09:10 (coordination.md:602).
- Graph image `.cache/b/g` was modified 01:35 and is in use. Its free space was not measured in this audit (`repo capacity status` not run to avoid load); last recorded free 822 MB (coordination.md:651).
- Quota: weekly usage 99% at 23:16 (coordination.md:729); new 5 h and weekly windows opened 01:35 (coordination.md:733).
- Auditor note: one repo-wide `find`/`grep` started by this audit ran over 120 s and was killed (no files changed).

## 4. Ten items most likely to block course, play and demo end to end (ranked)

1. Bazel failure output is lost (O1): the course proctor-build fails and nobody can see why (coordination.md:731; HANDOVER-1021.md:26-29). Blocks course.
2. Bazel reads the mutable tree (O2): peer edits produce "missing input file" in the course build (HANDOVER-1021.md:23-24; coordination.md:730). Blocks all three products (all Bazel).
3. Round 6 is not applied and is blocked by six owner items (O7, O8): the repo CLI pieces the e2e runs depend on (tool-iface, runtime-seam, legacy-purge, pm-seam, repo-launcher, storage-seam, cache-gc) are not in the live tree (round6-plan.md:55-63). Blocks all.
4. Host bun 1.3.14 vs pinned 1.4.2, and repo-launcher unapplied (O10; package.json:145; report-e2e-orch-e2e-1021l.md:17). Blocks all.
5. Graph image `g` at its 1 GiB floor, causing exclusive-maintenance convoys (O11; coordination.md:651, 669). Blocks all cold runs.
6. Lock batch not executed (O9): the frozen-lock failure was fixed by a one-off relock, the planned six-scope lock-sync is still pending (result-lock-sync.md:3; coordination.md:705). Blocks all.
7. Course assets builder idles after completion (O3; HANDOVER-1021.md:20). Blocks course (architecture-course build 6/8).
8. Play post-graph runaway: 67 min silent with RSS up to 11.7 GB (O5; coordination.md:714). Blocks play.
9. Play and demo have no green run since the 10/21 fixes; the 14:49 capacity failure fix is unproven (O6, O15; coordination.md:655, 698-703, 713). Blocks play and demo.
10. Peer live edits abort builds mid-run and break the loader (O12, O17; coordination.md:576, 715, 725). Blocks all, most often course.

Next checks for integrate3 and the owners: re-run the course proctor-build and read the captured tail (O1); confirm O2 with a second clean run; then play and demo cold runs with `SEMIO_BUILD_GRAPH_POLICY=current`.
