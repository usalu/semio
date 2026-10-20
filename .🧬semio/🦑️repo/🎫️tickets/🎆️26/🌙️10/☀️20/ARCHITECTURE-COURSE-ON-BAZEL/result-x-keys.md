# x-keys result: action-key stability, warm hits, snapshots

Paths: `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`, `A` = `B/📜️script.ts`. Tools: `T/x-keys-tools/` (`keys-probe.ts`, `snapshot-probe.ts`, `runner-closure.mjs`, `staged/`). Scratch: `T/🗑️generated/x-keys/`.

Status 16:25 (after the 16:03 kernel panic): static analysis and offline experiments DONE (they ran at 15:40-15:55 before the panic: `keys-probe.ts`, `snapshot-probe.ts`, staged test 3/3 green). Since the reboot the dependency images are unmounted (`node_modules` symlink dangling), so `bun test` and the probes fail to resolve `ajv`; they re-run once `bun repo boots`. Real `bun repo --tool=bazel run` experiments are pending that (coordination.md). Ready-to-run: `x-keys-tools/run-twice.sh @teaching/architecture-course:test` (two runs, `cache status` before/between/after, key-file diff, bazel process summary) and the same for `:typecheck` plus snapshot inspection. My earlier run (run1, 15:23) hit the persistent-child blocker; run-a (15:53) was killed by the panic.

## 0. Blockers found

1. Every `bun repo` run, including `--dry-run`, dies with `Exact original parent child allocation required` (my run: `🗑️generated/x-keys/run1-test.log`, 15:23 to 15:34, after 11 min of queue and walks). Cause: stale half-born persistent child, see `handoff-persistent-development-child-blocks-every-run.md`. The coordinator is repairing it. No real run was retried in a loop.
2. At 15:44:47 the x-inputs mirror (`🗑️generated/x-inputs/mirror/…`) hard-linked real sources (`build/📥️inputs/🔏️revision/🟨️.mjs` nlink=2). The revision capture refuses `nlink!==1` ("Bounded physical Source revision required"), so every `bun` script and `bun test` that imports it fails, including the existing `📥️fingerprint` test (baseline failure, not caused by my files). Reported to the coordinator.

## 0b. State at 16:50 (hand-off)

- 16:28 "bun repo boots", but the peer's quiet-tree window and graph inference refusal still block real runs: my dry-run of `@teaching/architecture-course:typecheck --dry-run` (`🗑️generated/x-keys/dry-typecheck.log`) stops at graph inference with `Generated task cannot consume its own output: @semio-tech/framework-os-shell-rs:typegen` (importer `🐚️shell/🟦️.ts`). Same refusal as the coordinator's `dry-boot-2.log`; the peer owns it (coordination 16:39). It is outside the course closure but blocks graph loading for every target.
- Pending, in this order, after the peer's GO: (1) `sh x-keys-tools/staged/check.sh --staged` (tmp-only writes, 3 tests, needs node_modules mounted); (2) `x-keys-tools/run-twice.sh @teaching/architecture-course:test`; (3) `x-keys-tools/run-twice.sh @teaching/architecture-course:typecheck`, then inspect the six dependency snapshots (`snapshot.json` roots non-empty) and `sha256` of `🖼️assets/README.md` and `🎨️styling/🎨️palette/🎨️.css` before/after the second run; (4) during a live action, `lsof -a -d cwd -p <runner pid>` for the runner cwd.
- Verdicts 1 to 4 below are decided offline; the live runs can only change them if the counters contradict the byte-level proof.

## 1. Does a per-invocation value enter a Bazel action key? REFUTED (offline proof; real hit pending)

Action key inputs (`STARLARK_RULES`, `A:512-566`; `renderBazelBuildV1`): `<label>.json` manifest, `<label>.input` fingerprint, declared source groups, dependency snapshots, `runner`, `source`, attrs `source_identity`, `bun`, `identity`, `mnemonic`; `env = {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0"}` only. Bazel is started with `--incompatible_strict_action_env` and the adapter passes no `--action_env`/`--test_env`, so the client environment (which carries `SEMIO_SCRIPT_PROCESS_INVOCATION`) cannot reach an action.

Where the envelope goes: `writeBazelRuntimeV1` (`A:435-439`) writes `runtime.json` (mode 0600, beside `implementation.json`, not a rule input) with `scriptInvocationEnvironment(invocation, process.env)`. `readBazelRuntimeV1` reads it at action time. The workspace root also comes from `runtime.json` (`runtime.workspace`), not from the manifest.

`taskFingerprint` (`A:482-491`) hashes platform, arch, bun version, `SEMIO_BUILD_MODE`, implementation sha, cargo toolchain digest (cargo tasks), sorted source identities (content hashes via `bazelSourceIdentityV1`), runtime-command stdout and declared `env` inputs. `randomUUID()` only for uncached tasks. Course `test`, `typecheck` and `check` declare `env` inputs only for `SEMIO_TEST_*`/`SEMIO_BUILD_BUDGET_MS`/`PROCTOR_*` (check) and none for `test`/`typecheck`.

Experiment (offline, real adapter code, no CLI, no lease): `T/x-keys-tools/keys-probe.ts` fabricates two caller envelopes and calls `generateBazelWorkspaceV1` on a course-shaped fixture (generator dependency + cached `test` + `typecheck` with `dependentTasksOutputFiles`).
- Variant A: deadline 30 s, cwd = repo. Variant B: deadline 90 min, cwd = tmpdir, random `SEMIO_NOISE`, other `HOME`/`TMPDIR`, different temp checkout path.
- Result (`🗑️generated/x-keys/keys-probe.json`): 14 files compared, `diffAB = ["runtime.json"]`. BUILD.bazel, MODULE.bazel, tasks.bzl, implementation.json, kinds.json, every `<label>.json` and every `<label>.input` are byte-identical although the temp checkout path differs. `runtime.json` does carry `SEMIO_SCRIPT_PROCESS_INVOCATION` in both and differs.
- Selection reuse: `gen:generate` manifest and fingerprint are identical between selection `[test, typecheck]` and `[typecheck --quiet]`, so dependency actions can hit across namespaces through the shared disk cache; the primary `typecheck` manifest differs by its forwarded args (P3-2, by design).
- Side finding: the implementation closure (`source_identity`, `implementation.json`, `runner`) is part of every key. `runner-closure.mjs` measured 470 files reachable from `A`; any edit to the adapter, build library or process layer invalidates all actions. Correct, but in this shared tree warm-hit measurements are noisy while peers edit those files and `ui/**`.

Pending (needs unblock): two consecutive `bun repo --tool=bazel run @teaching/architecture-course:test` runs with `cache status` counters and a diff of `.cache/b/p/m/bazel/workspaces/<ns>` between them. Script: `x-keys-tools/run-twice.sh` (written). Note for reading its output: a second run may report `0 processes` (Bazel's own up-to-date check) instead of `N disk cache hit`; both mean no action re-ran. `runtime.json` is rewritten on every run but is not referenced by any rule (BUILD has no `glob(`), so it cannot invalidate.

## 2. `findWorkspaceRoot` lstat/isFile inside an execroot? REFUTED for course actions (latent for execroot callers)

- `executeTask` (`A:672-686`) resolves `cwd = execution.cwd ? resolve(workspace, …) : workspace`, with `workspace = runtime.workspace` (the real checkout from `runtime.json`) and sets `SEMIO_WORKSPACE_ROOT = workspace` (`bazelTaskBaseEnvironmentV1`). The task command (`bun exec <source>`) therefore runs in the real checkout where markers are regular files. No course task uses a `hermetic` declaration (`grep hermetic` = 0 in course, proctor, framework-rs manifests), so none runs inside the execroot. Test-kind actions (`repo_test`) also run the same `runBazelActionV1` and spawn the task in the real workspace.
- The Bazel-owned runner process (cwd = execroot or runfiles) only uses explicit roots: `withRepositoryCacheStoreV1(runtime.workspace,…)`, `restoreBazelSnapshotV1(runtime.workspace,…)`, `captureBazelSnapshotV1({workspace,…})`. Intersection check (`runner-closure.mjs`): of 38 library files that call `findWorkspaceRoot()`/`getWorkspaceRoot()` without an argument, only 2 are inside the runner's 470-file import closure (`🏃️process/🦀️cargo/🎟️admission/🟦️.ts:46` `runRepositoryCargoTests`, `🏃️process/🦀️cargo/🎯️exact/🟦️.ts:47`); both are cargo test entry points that run in the task child, not in the runner.
- Real confirmation pending: runner cwd (`lsof -a -d cwd`) during a live action. The audit's probe (symlinked markers throw) stays true for any future caller in an execroot, so the hardening (`statSync`) is optional. No fix staged ("fix only if it really breaks").

## 3. P2-3: do generator restores overwrite tracked files? REFUTED as a defect (harmless by construction); behavior proven

- `restoreBazelSnapshotV1` (`A:646-665`) compares every entry (kind, mode, link target, sha256) and the entry count with the snapshot and `return`s without writing when they match.
- Experiment `snapshot-probe.ts` (real `captureBazelSnapshotV1`/`restoreBazelSnapshotV1`, fixture tracked file plus output dir, `🗑️generated/x-keys/snapshot-probe.json`): identical files keep inode and mtime-ns (no rewrite); edited files are restored to snapshot bytes; so a restore is a no-op when the checkout already matches.
- Why a differing restore cannot silently destroy work: both tracked outputs are also declared inputs. `assets:build` writes `🖼️assets/README.md` (taxonomy `generatorContracts.assets-build.outputRoots[0]`, tracked) and `ui-styling-tokens:generate` writes `🎨️styling/🎨️palette/🎨️.css` (tracked); their default inputs are `🖼️assets/**/*` and `🎨️styling/**/*`, so a hand edit changes the action key, which is a miss and the generator regenerates (same as Nx). A hit with different bytes needs the same key and different bytes (checkout moved back to an older state), and then the restore writes what the generator wrote for that exact input state.
- Residual hazards (documented, not defects): (a) a restore whose entry count differs (stray extra file in a declared output directory) removes the whole root (`rm -r roots`), so a declared directory root must not host undeclared files; (b) restores run under the exclusive lease `bazel-snapshot-restore`.
- Checksum before/after on the real README/palette: pending unblock (`snapshot-probe.ts` already proves the mechanism on the fixture).

## 4. P2-5: missing declared outputs silently accepted? REFUTED (stale audit; strict is the live default)

- Live `bazelMissingOutputRefusedV1` (`A:620`) refuses a missing plain-path output for `cache === true` unless `metadata.semio.optionalOutputs === true`. Probe: cached task with a missing output throws `Declared output of a replayed Bazel task was not produced`; optional accepts; uncached accepts. Glob outputs are not checked (none in the generators).
- The four generators do get outputs: `project.json` declares none for `framework-rs:generate`, `ui-contract-rs:generate`, `ui-rs:generate` (only `framework-actor-rs:typegen` declares one), but `generatorOutputOwners` (`INF/🏭️generator-outputs/🟨️.mjs:24-37`) injects every `outputRoots` path of the owned taxonomy contract as `{workspaceRoot}/…`. Resolved outputs: framework-rs `🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts`; ui-contract-rs `…/📜️ui-contract/🟦️.ts`; ui-rs `🎚️axes/🤖️generated/🦀️.rs` and `…/🎚️ui-axes/🟦️.ts`; actor `🎭️actor/🤖️generated`; styling-tokens palette css, `🔷️.cs`, `🐍️.py`, `🦀️.rs`, `🤖️generated`; assets README plus 8 generated roots. All of these paths exist on disk right now (checked: none MISSING), so strict mode will not refuse a correctly generated tree, and a generator that stops writing one fails the run instead of caching an empty snapshot.
- Remaining check (pending unblock): `bun repo --tool=bazel run @teaching/architecture-course:typecheck` and inspect the six dependency snapshots under `bazel-bin/*.snapshot/snapshot.json`.

## Fixes

None required: no risk was confirmed as a defect. Regression contract staged (test-first invariants, fixture + schema + Python oracle):
- `T/x-keys-tools/staged/files/📥️keys/{🟦️.ts,🔣️.json,🧬️schema.json}` (installed to `B/🧪️tests/📥️keys/`)
- `T/x-keys-tools/staged/apply.py` (idempotent; prints created/updated/unchanged), `T/x-keys-tools/staged/check.sh [--staged]`.
- Tests: (1) fixture closed by schema; (2) only `runtime.json` differs between two envelope/cwd/ambient variants (Python sha256 tree diff oracle), envelope absent from BUILD/tasks.bzl, rule env is exactly `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`; (3) restore no-op/repair/missing-output refusal (Python sha256 oracle).
- Run status: not yet green under `bun test` because of blocker 0.2 (baseline `📥️fingerprint` fails the same way). The same logic passed as plain `bun` scripts at 15:40 (`keys-probe.ts`, `snapshot-probe.ts`).

Apply order: none against frozen layers. After the peer freeze lifts: `python3 T/x-keys-tools/staged/apply.py`, then `sh T/x-keys-tools/staged/check.sh`.

## Backend edits

None.
