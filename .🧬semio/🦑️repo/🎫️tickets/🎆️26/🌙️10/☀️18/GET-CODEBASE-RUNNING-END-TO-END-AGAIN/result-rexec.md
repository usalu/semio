# result-rexec: hermetic, remote-execution-ready Bazel actions

`C` = this ticket. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. `BZ` = `L/🏗️build/🔧️tools/🏗️bazel`. Tools and the staged set: `C/rexec-tools/`. Bazel 8.4.2 (provisioned binary), macOS 27 arm64, host bun 1.3.14 (the repo pin is 1.4.2). Nothing outside `C/` was written (PAUSE phase): the whole change is STAGED; `C/rexec-tools/apply.sh` applies it after "PAUSE LIFTED".

## Verdict

- Every action the adapter emits today except the ordering `noop` is host-state by construction. Measured, not assumed: table below.
- The adapter now has a second action class, `hermetic` (`repo_hermetic_task`), selected per target by a neutral declaration `metadata.semio.hermetic = {version:1, tools:["bun"]}`. It runs a self-contained runner under a declared bun toolchain, in a sandbox or on a remote executor, with a strict environment, no host paths and normalised outputs. No existing target declares it yet, so nothing changes for current tasks (verified: generated BUILD for a real task is byte-identical except the implementation identity).
- The law (`BZ/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts`) builds a sample through the real generator and the real Bazel, reads the execution log and fails on any undeclared input, wrong tool, host variable, absolute path, wrong platform, non-isolated runner, undeclared write, or a non-identical rebuild. It passes (40+ expectations, 296 across both suites).
- Real repo tasks are NOT hermetic yet and cannot be made so by Bazel alone: they need the task runtime (issued authority, storage images, cargo gate, captured implementation rooted at the checkout) to run without a live owner. That is the gap list at the end.

## 1. Audit (execution log on a scratch copy; harness `C/rexec-tools/run/`)

Runs: scratch Bazel host (ASCII output base), static graph reconstruction (8,027 targets), task `@semio-tech/framework-graph:generate` (T3, 69 declared inputs, cache true). `sandbox-exec` wrappers around bun for the network and write probes. Raw logs deleted.

| Kind | Rule | Evidence | Hermetic |
|---|---|---|---|
| `noop` | `repo_noop` | `ctx.actions.write`, no spawn | yes |
| `task` | `repo_task`, mnemonic `RepoTask` | exec log: `runner=local`, `remotable=false`, `platform.properties=[]`, `commandArgs[0]=/Users/ueli/.bun/bin/bun` (not an input, not a tool), env = `PATH=/bin:/usr/bin:/usr/local/bin` only | no |
| `launcher` | `repo_launch` build action, `RepoLauncher` | `workspace:dev-mcp` built: output digest `e0c90ec1...` = digest of the host bun binary (63 MB executable copy); `cacheable=false`, empty env, empty platform | no |
| `launch` | `bazel run` of the launcher, `birth-launch` | static (code): runs the task in the real checkout, continuous, never cached | no |

Violation classes found (each authored in schema `Audit` with its reason; the law's sample class has none):

1. `absolute-tool-path`: the executable is the orchestrator's own bun path. PROVEN KEY IMPACT: the same bun binary copied to two paths gives a disk-cache MISS (`k1` executes, `k2` same path = `1 disk cache hit`, `k3` other path = `1 local`). Action keys are host and user specific, so no cache sharing across machines for any current task. Highest-value finding for the cache owner.
2. `host-environment`: the task env is read from `runtime.json` (the orchestrator's whole environment), outside the action key.
3. `host-implementation-root`: `implementation.json` embeds the absolute checkout root (`value.root`) and the loader re-reads captured sources from it (`currentImage`); the 5.9 MB input digest differs per checkout path and the action cannot run where that path is absent.
4. `undeclared-inputs`: the execroot symlink `s` points at the whole checkout. Static check of T3: 131 workspace modules in its import closure, 118 not declared (static graph carries no closure markers; the real graph does via the applied inputs-closure change, so re-measure there).
5. `undeclared-writes`: tasks write into the checkout and shared stores. What-if `--modify_execution_info=RepoTask=-no-sandbox --spawn_strategy=sandboxed` fails: `Original Bazel birth parent disconnected`.
6. `live-authority` and `loopback-control`: birth rendezvous over a 127.0.0.1 socket of the orchestrator.
7. `umask-modes`: `captureBazelSnapshotV1` records `mode` from the producing file (umask dependent) and lists directories in `readdir` order (filesystem dependent): non-reproducible snapshot bytes across machines.
8. `shared-transpiler-cache` (added on the coordinator's request after bzl-coverage): Bun's transpiler cache is content keyed, so identical sources under a second root resolve imports against the first root. Fix: `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` in the action environment contract. Hermetic actions carry it in the contract (law-checked) and in the runner's child env. Host rules get `env = {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0"}` on `RepoTask` and `RepoLauncher` (adapter edit; `rexec` verified the exec log shows it). The runtime reproduction is bzl-coverage's; my own direct two-root probe did not reproduce it outside the captured loader.

Not violated (measured): no external network use by T3 (run under `sandbox-exec` deny `network-outbound` except loopback: success); T3 writes nothing under the checkout outside `.cache` (deny-write profile: success; it declares no outputs). Timestamps: no mtimes in snapshots; tree artifacts carry none.
Not measurable here: file reads. `sandbox-exec (trace)` is dead on macOS 27 and fs_usage/dtrace need root; read-closure evidence is the static import closure plus the differential Q7/Q8 of `result-bzl-real.md`, and, for the hermetic class, the sandbox itself (the law's `neg:leak` task reads an undeclared file and fails with ENOENT).

## 2. Toolchains, platforms, exec properties (contract, schema first)

- Neutral: `L/🏗️build/🏃️execution/🧬️schema/🔒️hermetic.json` (target declaration, Draft-07, closed). File name avoids `🛰️`, which the taxonomy uses for protobuf.
- Bazel adapter schema `BZ/🏃️execution/🧬️schema/🔣️.json`: `Contract` (six platforms darwin/linux/windows x arm64/x64 with Bazel constraints and exec properties `OSFamily`, `Arch`; toolchain type; env contract; remote policy; strict action env), `Environment`, `Flags`, `Manifest` (closed hermetic task manifest), `Audit` (violations per kind), `Reasons`.
- bun is a declared, digest-pinned tool input per platform: pins are the archive SHA-256 and size of the existing bun release manifest (`⚡️caching/🚀️bootstrap/🛠️tools/🟦️bun/🧬️schema`, read, not duplicated). `prepareBazelBunV1` downloads under lease with progress and cancellation, verifies size and SHA-256 BEFORE extracting (`verifyBazelBunArchiveV1`), publishes atomically under `tools/bazel/bun/<pin16>/`. The generated package links it at `tools/bun/<id>/bun[.exe]`; `repo_bun_toolchain` (carries `pin`) exposes it via `toolchain_type` `bun_toolchain_type`; the rule `repo_hermetic_task` takes it through `ctx.toolchains`, so it is an action input flagged as tool (law-checked: the only tool input).
- No `@platforms` and no MODULE.bazel change: `@platforms` is not visible without a `bazel_dep` and a MODULE change would invalidate `MODULE.bazel.lock`, so the adapter defines its own constraint universe (`exec_os`, `exec_cpu`, `exec_pool`) in the generated BUILD and registers toolchains and platforms with `--extra_toolchains` and `--extra_execution_platforms` (no repository rule, no network at build time, `--repository_disable_download` stays).
- Routing: the host platform is `exec_<id>` (pool `local`); with a remote executor, `exec_remote_<id>` carries pool `remote` and the contract properties plus `SEMIO_REMOTE_EXECUTION_PROPERTIES`; hermetic targets get `exec_compatible_with = [":exec_pool_remote"]`, host-state rules keep `no-remote-exec` and stay local.
- Flags (`bazelExecutionFlagsV1`, only when the run has hermetic actions): `--extra_execution_platforms`, `--extra_toolchains`, and with an executor `--remote_executor=<endpoint>`, `--noremote_local_fallback`, plus `--jobs=<n>` (default 128, cap 512). It never emits cache, instance, upload, compression, download or credential flags.
- Environment (naming aligned with rcache's `SEMIO_REMOTE_CACHE_*`): `SEMIO_REMOTE_EXECUTION_ENDPOINT` (grpc, grpcs, unix), `SEMIO_REMOTE_EXECUTION_PLATFORM` (default `linux-x64`), `SEMIO_REMOTE_EXECUTION_PROPERTIES` (`name=value,...`), `SEMIO_REMOTE_EXECUTION_JOBS`. Unset keeps everything local; malformed fails closed.
- Hermetic action properties (rule): no `no-sandbox`, no `no-remote-exec`, `block-network`, `use_default_shell_env = False`, env exactly `LC_ALL=C TZ=UTC SOURCE_DATE_EPOCH=0 BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`, mnemonic `RepoHermetic`.
- The runner (`BZ/🏃️execution/🏭️runner/🟦️.ts`, shipped as `hermetic-runner.mjs`) imports nothing from the repo: restores dependency snapshots (refuses overlap with declared inputs, symlinks, escapes), runs `bun exec` per command with a 13-variable environment (measured: `PATH HOME TMPDIR TEMP TMP LC_ALL TZ SOURCE_DATE_EPOCH BUN_RUNTIME_TRANSPILER_CACHE_PATH SEMIO_*`), snapshots declared outputs in the existing snapshot format with sorted entries and modes normalised to 0644/0755/dir 0755, and FAILS on writes outside declared outputs and when `s` is a symlink (an unisolated root). Snapshots plug into the existing `restoreBazelSnapshotV1`.
- Adapter edits (anchored, idempotent, `C/rexec-tools/adapter_edits.py`, applied to the CURRENT live text so concurrent edits survive): `BazelActionV1.kind` gains `hermetic`; `bazelActionsV1(..., {hermetic})`; `renderBazelBuildV1` (execution section, load symbols, rule); `bazelExecutionPlanV1`; `generateBazelWorkspaceV1(..., execution?)` (manifest, runner, toolchain links, refuses hashed inputs for hermetic tasks); `runBazelTasksInStorageV1` (provisions bun, adds flags, skips births, restores snapshots); `RepoTask`/`RepoLauncher` env.

## 3. The law (what it proves)

`BZ/🏃️execution/🧪️tests/🔒️hermetic/🟦️.ts`, gated by `SEMIO_TEST_BAZEL_BINARY`, uses `generateBazelWorkspaceV1` on a scratch checkout, a producer->consumer pair plus negative controls, and `bazelHermeticViolationsV1` (unit-tested against eight mutations) on the execution log:
- isolated runner (`darwin-sandbox`), inputs equal declared sources + manifest + runner + dependency snapshot files, exactly one tool input (the bun toolchain), environment equals the contract, no absolute argument, platform properties equal the contract's;
- two clean builds in different output bases, the second under `umask 077`, give identical tree digests (snapshot bytes and modes);
- the same inputs generated at a different checkout path and workspace hit the disk cache (`2 disk cache hits`): keys are host-path independent, i.e. shareable through a remote cache;
- negative controls fail as designed: undeclared read (ENOENT on `secret.txt`), undeclared write (named in the error), `--spawn_strategy=local` (refused), remote executor unreachable with `--noremote_local_fallback` (fails, never runs locally);
- network: asserted on Linux only. MEASURED on darwin-sandbox 8.4.2: neither the `block-network` tag nor `--nosandbox_default_allow_network` stops a connection to the machine's LAN address (both probes `reached`). macOS cannot enforce no-network; Linux (network namespace) and remote workers do.

## Verification (commands and outcomes)

All via a re-pointed overlay of COPIES (`C/rexec-tools/overlay.py` -> `run/overlay`, deleted afterwards):
- `bun test ./<overlay>/🏃️execution/🧪️tests/🟦️.ts`: 15 pass (contract vs bun manifest with Python, env parser vs Python, flags, decisions vs Ajv, manifest, Starlark parse of BUILD and rules with Python `ast`, archive pin vs Python `zipfile`, links, runner determinism across umask with a Python verifier, runner refusals, dependency restore, judge mutations, audit shape, transpiler-cache env).
- `SEMIO_TEST_BAZEL_BINARY=... bun test ./<overlay>/.../🔒️hermetic/🟦️.ts`: 1 pass, about 60-110 s.
- `node_modules/.bin/tsc -p run/tsconfig.rexec.json` (adapter, module, both test files): 0 errors.
- Regression on the overlay adapter vs live, per suite: 📥️actions 10 pass, 📥️workspace 4, 🔏️implementation 1, 📥️delegation 1, 📥️sources 9, 📥️closure 8, 📦️snapshot 1, 📥️fingerprint 2, 📥️inventory 1: identical to live. (📥️environment, 📥️framing, 📤️arguments, 🕸️dependencies fail identically live and overlay; pre-existing, need issued envelopes.)
- Generation on the overlay adapter for a real host task (`framework-graph:generate`): `bazel build` ok, `2 processes: 1 internal, 1 local`; `BUILD.bazel` differs from live only in `source_identity`, `tasks.bzl` and `MODULE.bazel` identical. Applying re-keys every existing action once (implementation identity), as any adapter edit does.
- `apply.sh --dry-run`: plan printed, refused because the dependency sync is running (as designed). `apply.py` tested on a temporary root: apply, idempotent re-apply, drift refusal, applied adapter equals the staged adapter.
- NOT run: `runBazelTasksInStorageV1` end to end (needs the storage images; the dependency sync holds them), `prepareBazelBunV1` against the network (no download was approved; archive verification is covered offline), Linux, Windows, any real remote executor.

## Files (all staged under `C/rexec-tools/staged/`, mirroring the live paths)

New: `L/🏗️build/🏃️execution/🧬️schema/🔒️hermetic.json`; `BZ/🏃️execution/{🟦️.ts, 🏭️runner/🟦️.ts, 🧬️schema/🔣️.json, 🧪️tests/🟦️.ts, 🧪️tests/🧫️fixtures/🔣️.json, 🧪️tests/🔒️hermetic/🟦️.ts}`. Patched at apply time: `BZ/📜️script.ts` (staged copy shown for review), `BZ/🧪️tests/🟦️.ts` (two imports). Tools: `apply.sh apply.py adapter_edits.py build-adapter.py overlay.py scratch.part.ts run/{drive.ts,run.ts,go.sh,tsconfig.rexec.json}`.
After the lift: `C/rexec-tools/apply.sh --apply`, then the live suites (`🏃️execution/...`, `📥️actions`), and confirm the taxonomy gate admits the new directory names `🏃️execution` (adapter), `🏭️runner`, `🔒️hermetic` and the file `🔒️hermetic.json`. Re-run `build-adapter.py` first if the adapter changed (anchors fail loudly).

## What a real remote executor still needs (Buildbarn, BuildBuddy; nothing signed up)

1. A target must opt in: add `metadata.semio.hermetic` to real targets. Today none qualifies: the task runtime is not isolated-capable. Needed: a storage-free, authority-free task entry (the `runBazelTasksOnHostV1` of `result-bzl-real.md` item 2, plus the issued-env designation `SEMIO_SCRIPT_PROVIDERS`), declared tools beyond bun (cargo toolchain, rustup, node, python as further `tools` entries with their own pins and platforms), and the import closure of each script as declared inputs (real graph, inputs-closure).
2. Worker image: Linux workers with `OSFamily=linux`, `Arch=amd64` (and arm64) matching `exec_properties`; pool or image keys through `SEMIO_REMOTE_EXECUTION_PROPERTIES` (Buildbarn matches properties exactly; BuildBuddy uses `container-image`, `pool`). Workers need only the uploaded bun; no repo checkout.
3. Cache/executor wiring with rcache: one gRPC endpoint serving CAS, action cache and execution. rcache's HTTP cache flags (`--remote_cache=http...`) must be replaced by the executor's gRPC URL when execution is on (set `SEMIO_REMOTE_CACHE_ENDPOINT` to it; confirm rcache's endpoint contract admits grpc/grpcs); rcache's credential helper is scoped to the cache host, so the executor host must equal it or the helper needs a second host. Download mode must stay `toplevel` or `all` (rcache already refuses `minimal`). `--remote_instance_name` is shared with the cache contract. Hermetic results are sandboxed, so rcache may allow `--remote_upload_local_results` for them.
4. Portable keys for host-state kinds (needed before any remote cache sharing of current tasks): run bun from a declared toolchain instead of `process.execPath`; make `implementation.json` root-independent; stop copying the host bun binary into launcher outputs.
5. Windows hermetic actions: no sandbox, so the runner refuses (`s` is a junction). Needs a runner-materialised isolated workspace from the declared input list, or remote-only execution for Windows hermetic tasks.
6. Network isolation on macOS is not enforceable by Bazel 8.4.2; only Linux sandbox and remote workers isolate. Document hermetic as "enforced on Linux and remote".
7. Observability: surface `bazelHermeticDecisionV1` reasons in `--dry-run` and the uncached report (not wired).
8. The bun pin is 1.4.2 while developers run 1.3.14: hermetic actions run the pinned one, so first adoption can differ from `bun repo` behaviour.

## Cleanup

Bazel servers of this run stopped, `/private/tmp/rexec` removed (chmod first), `C/🗑️generated/rexec`, the regenerated `C/🗑️generated/bzl-breadth` and `run/overlay` removed; `rexec-tools/run/go.sh` needs `REXEC_ADAPTER` (an overlay adapter) and `bun C/bzl-breadth-tools/graph.ts` before reuse.
