# result-bzl-rust: Rust/wasm under Bazel, decided by measurement

`C` = this ticket. `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`. Bazel 8.4.2 (provisioned binary), macOS arm64, load average 39 to 75 during the runs. Everything is staged under `C/bzl-rust-tools/`; nothing in the repo outside `C/` was written.

## Verdict
**(a) wins: one Bazel action per crate that wraps the repository Cargo ingress, with declared source inputs and artifact outputs. (b) rules_rust/crate_universe is rejected.** The existing graph already is (a): native `build|check|test` targets run `native cargo <t> --manifest <crate>` (a `repo:script` task, `cache: true`, inputs from the inference's exact Rust module closure, `outputs: dist/build/<mode>`), and the generic `repo_task` turns each into a Bazel action. What was missing, and is staged: (1) Bazel-side serialization of Cargo actions, (2) a spawn-free toolchain identity in the action key, (3) a typed `repo_cargo` rule for the wasm component chain, (4) the measurements below.

## Why not (b), by evidence
| Fact | Source |
|---|---|
| The ingress is configured for Cargo (`.cargo/config.toml` `[build] rustc`). rules_rust drives a downloaded rustc itself, so it **bypasses** the gate, storage custody (output would live in the Bazel output base, not the fixed image), the profile injection (`--config profile.prototype.*`), nightly `[unstable]` flags (`fine-grain-locking`, `embed-metadata`, `build-dir-new-layout`), `rust-source-boundary` admission and the rustc memory broker. Policy forbids a hand-set RUSTC. | `.cargo/config.toml`, launcher `🚪️launcher/🟦️.ts` (read, not run) |
| Bazel modules are a fixed embedded catalog (`module(name="semio_tasks")`, `--repository_disable_download`, `--incompatible_autoload_externally=`); the root `MODULE.bazel` has no `bazel_dep`. rules_rust needs BCR downloads and crates.io fetches at build time. | adapter `BAZEL_MODULE`, `runBazelTasksInStorageV1`, root `MODULE.bazel` |
| Scale: root `Cargo.lock` 1,561 packages (801 registry crates), `🧰️framework/Cargo.lock` 1,064; cargo-plan counts 290 workspaces / 3,421 packages in scope, 110 lock-bearing workspaces in the required steps. crate_universe regenerates per workspace lock. Proc-macros, build scripts, `cargo-features = ["trim-paths"]` and the wasm32-wasip2 component flow are not covered by it. | `C/result-cargo-plan.md`, `Cargo.lock` |
| Per-unit actions would need one ingress call per rustc unit; the ingress has a large fixed cost (next table). | measured |

Ingress fixed cost (one `bun repo cargo --store=shared -- check -p bzlrust-bulge`, `SEMIO_CARGO_TIMING=1`, load 75): store 10.8 s, inputs (`fetch --locked`, config views) 10.1 s, unit-graph capture 0.7 s, source admission 1.1 s, writer lease 0.8 s, scheduler 2.2 s, **Cargo itself 0.8 s of about 27 s total (3 %)**. A cache MISS pays this once per action; a Bazel HIT pays nothing, so the unit of caching must be coarse (crate), which is (a). A finer, still ingress-compatible granularity (a unit action fed by the ingress's own `--unit-graph` capture) is possible later but unproven; not chosen.

Ingress constraints found (they shape the design):
- The launcher fixes `CARGO_TARGET_DIR`/`CARGO_BUILD_BUILD_DIR` (`inputs.environment = {...store.environment, CARGO_TARGET_DIR: selected.target ...}`). **A private `CARGO_TARGET_DIR` is not possible through the ingress**; the only private variant is `--store=isolated` (swept after the call). I used `--store=shared` with uniquely named scratch packages (`bzlrust-*`) and removed their units afterwards.
- `-Z unstable-options --artifact-dir <dir>` is refused: `Cargo generated publication belongs to original repository admission`. Artifacts can only be taken from `--message-format=json-render-diagnostics` `compiler-artifact.filenames` (this is what the staged runner does), or by the in-process `launchRepositoryCargoV1(..., {output:"capture"})` consumer. The shared store is a mounted image; the runner copies right after the CLI exits and worked every time, but an in-process consumer under custody is the robust form (open item 2).
- `rustc` metadata/rlibs are never Bazel outputs: a Bazel HIT gives a verdict (check/test receipt) or the declared final artifacts, not intermediate rlibs. After a cache restore, a dependent MISS recompiles its dependencies inside the shared build-dir (cargo's own freshness handles that).

## Measurement setup (scratch, nothing in the checkout)
- Scratch Cargo workspace (own `[workspace]`, `cargo-features = ["trim-paths"]`, hand-pinned lock made by `… -- update --workspace --offline`): copies of three framework crates `semio-framework-geometry-{primitives,vector,bulge}` (renamed `bzlrust-*`, path deps, owner trees incl. `#[path]` modules, one unit-test directory) plus a 4-line `cdylib` `bzlrust-wasm`. Max path 198 chars.
- Scratch Bazel workspace in ASCII `/private/tmp/bzlrust` (own output base, install base, disk cache), source link `s -> checkout` like the production adapter, runner reading env/workspace from a `runtime.json` beside the manifest (outside every key), `--lockfile_mode=off --incompatible_strict_action_env`, local disk cache.
- Every cargo invocation went through `bun repo cargo --store=… --configuration=prototype --parallelism=1`. At most one cargo at a time (enforced by the rule, see R1). Not run: the production `runBazelTasksV1`, product activations, anything in the real graph.
- The staged module renders the manifests/BUILD/rule; the harness only adds paths (`harness/gen.ts`).

## Evidence tables (Bazel summary line; spawns = ingress calls counted by the runner journal)
Series R (declared inputs = owner trees, `glob` mode; 3 `check --lib` actions + 1 `build --target wasm32-wasip2` action):
| Phase | Bazel summary | Ingress spawns | Wall |
|---|---|---|---|
| R0 all four, no scheduling hint | 4 local | 4, all started within 30 ms (ingress leases serialized only the compile; ends 61/84/120/130 s) | 130 s |
| R1 cold, with `resource_set` | `6 processes: 1 internal, 1 darwin-sandbox, 4 local` | 4, strictly sequential (next start 0.1 s after previous end) | 194.6 s |
| R2 warm | `1 process: 1 internal` | **0** | 1.6 s |
| R3 edit bulge | `2 processes: 1 action cache hit, 1 internal, 1 local` | 1 (bulge only) | 29.5 s |
| R4 revert | `1 action cache hit, 1 disk cache hit` | 0 | 0.8 s |
| R5 edit primitives | `1 action cache hit, 3 local` | 3 (primitives, vector, bulge; wasm untouched) | 95.9 s |
| R6 revert | `1 action cache hit, 3 disk cache hit` | 0 | 0.4 s |
| R7 `bazel clean` + rebuild | `6 processes: 5 disk cache hit, 1 internal` | **0** | 5.4 s |
Series G/E (input precision; unit-test file edit in the primitives owner tree, never read by rustc for check/build):
| Phase | Inputs | Bazel summary | Spawns |
|---|---|---|---|
| G1 cold | owner globs | 4 local | 4 (135 s) |
| G2 edit unit-test file | owner globs | `1 action cache hit, 3 local` | **3** (over-invalidation) |
| G3 revert | owner globs | 3 disk cache hit | 0 |
| E1 cold | exact (rustc dep-info + Cargo.toml) | 2 + 2 local | 4 |
| E2 edit unit-test file | exact | `1 process: 1 internal` | **0** |
| E3 edit primitives source | exact | `1 action cache hit, 3 local` | 3 (wasm untouched) |
Exact inputs came from rustc's own dep-info (third-party oracle, `harness/oracle.ts`): primitives 15 `.rs`, vector 1, bulge 1, plus every `Cargo.toml`/`Cargo.lock`/`rust-toolchain.toml`/`.cargo/config.toml`. This equals what the inference's `nativeSources` (exact Rust module closure) declares; the Nx-style `default` owner glob over-invalidates.
Series S/F (wasm chain: `build` wasm32-wasip2 cdylib -> `jco transpile` -> `wasm-opt -Os`, each a Bazel action; tool identities as action attributes):
| Phase | Bazel summary | Spawns | Note |
|---|---|---|---|
| S1 warm | `1 process: 1 internal` | 0 | 2.3 s |
| S2 comment-only edit of the wasm crate | `1 action cache hit, 1 local` | 1 (cargo, 57 s) | jco and wasm-opt **not rerun**: wasm bytes identical, early cutoff across the cargo -> jco -> wasm-opt chain |
| S3 revert | 1 disk cache hit | 0 | |
| V1 runner file changed | `2 action cache hit, 1 local` | 1 (wasm crate only) | runner is a declared input; dependents cut off |
| F1 final code, cold for changed runner | `4 action cache hit, 3 local` | 3 | 276 s under load |
| F2 warm | `1 process: 1 internal` | **0** | 0.7 s |
| F3 `bazel clean` | `8 processes: 7 disk cache hit, 1 internal` | **0** | 5.1 s; wasm and optimized core module restored from the disk cache (7.9 MB total) |
Artifacts: `bzlrust_wasm.wasm` 2,683,692 bytes (debug), receipt `{"version":1,"id":"b_wasm","status":0,"artifacts":[{name,sha256,bytes}]}` (no timings, so outputs are reproducible); `wasm-opt -Os` 2,681,098.

Scheduling findings (Bazel 8.4.2, probe actions, 3 x 4 s sleeps, `--jobs=4`): `execution_requirements` `cpu:6` and `resources:cargo:1` (+ `--local_extra_resources=cargo=1`) did **not** serialize (all concurrent); `ctx.actions.run(resource_set = fn)` returning `{"cpu": 64}` (clamped to the host) serialized exactly. `resource_set` is not part of the action key (a rule change from `resources:` to `resource_set` was a 4-hit warm run).

## Staged changes (copies under `C/bzl-rust-tools/staged/`, mirror of `B/`)
| File | Content |
|---|---|
| `B/🦀️cargo/🟦️.ts` | `bazelCargoClassV1` (native task -> `{subcommand,manifest}`), `bazelCargoArgumentsV1` (exact ingress argv), `bazelCargoManifestV1` (closed, host-path-free, carries the toolchain digest), `bazelCargoArtifactsV1`, `bazelCargoToolchainDigestV1` (no spawn: pinned `rust-toolchain.toml` + `multirust-channel-manifest.toml` of the resolved toolchain + host), `BAZEL_CARGO_EXCLUSIVE_V1`, `BAZEL_CARGO_RULE_V1` (`repo_cargo` + `cargo_resources`), `renderBazelCargoBuildV1` |
| `B/🦀️cargo/🧬️schema/🔣️.json` | Policy const, Action, Manifest (schema-first) |
| `B/🦀️cargo/🏃️action/🟦️.ts` | dependency-free runner (ingress child, JSON messages, artifact copy, receipt); exports `selectCargoArtifactsV1` |
| `B/🦀️cargo/🧪️tests/🟦️.ts` + `🔣️.json` | language-agnostic corpus: classes, argv (Python oracle), manifest closure (Ajv), artifacts from real captured cargo JSON (Python `json` oracle, plus runner equality), toolchain digest (Python `hashlib` oracle), rendered BUILD (independent Python rendering), adapter rows, and a `SEMIO_BAZEL_BIN`-gated test that the real Bazel loads the rule |
| `patches.py` + `apply.py` | 8 anchored edits of `B/📜️script.ts` (each anchor must occur exactly once; refuses on drift or an existing module dir; `--dry-run`): import; `BazelActionV1.exclusive`; `bazelActionsV1` marks native Cargo task actions; `renderBazelBuildV1` emits `exclusive = True`; `repo_task` gets `resource_set = cargo_resources if ctx.attr.exclusive` and the attr; `tasks.bzl` = `STARLARK_RULES + BAZEL_CARGO_RULE_V1`; `taskFingerprint` adds the toolchain digest (memoized) for Cargo tasks |
Apply after "PAUSE LIFTED": `cd /Users/ueli/Documents/semio && python3 C/bzl-rust-tools/apply.py --dry-run && python3 C/bzl-rust-tools/apply.py`, then the existing Bazel suites (`C/bzl-act-tools/wrap.ts` for `🧪️tests/📥️actions`, `📥️fingerprint`, `🔏️implementation`) and `bun test ./B/🦀️cargo/🧪️tests/🟦️.ts`.
Nx is untouched: no target, input or command changes; the new module is Bazel-only.

## Verification
- `bun test ./<overlay>/🧪️tests/🟦️.ts` (overlay = module + patched adapter copy with absolute specifiers, built by `harness/overlay.py` from the live `B/📜️script.ts`): 7 pass, 1 skip; with `SEMIO_BAZEL_BIN` set: 8 pass (real Bazel query loads the rule). Mutation check (flip one adapter expectation): fails as it must.
- `tsc --noEmit --strict` on the module and runner: clean.
- `python3 apply.py --dry-run`: patch ok, 8 anchors.
- Bazel runs: tables above (real binary, real `bun repo cargo`). The anchored patch was **not** applied live and the existing adapter suites were **not** run on the patched file (frozen library).
- Path lengths of all staged files: max 198 chars.

## Cleanup
Bazel server shut down, `/private/tmp/bzlrust*` and `/tmp/bzlrust-*` removed, scratch workspace and logs under `C/🗑️generated/bzl-rust-*` removed, the 10 `bzlrust*` units/uplifts removed from the shared Rust image. The harness scripts stay (`harness/*.sh|ts|py`, `mk-scratch.py`).

## Coordinator note
After the measurements the coordinator asked not to start new `bun repo` invocations before the lift (my `--store=shared` calls took the image lease, the sync was queued). My last `bun repo cargo` finished before the message arrived; none started afterwards; I did not substitute raw cargo/rustc (the gate forbids it). Ingress measurements are therefore complete as listed; re-measurement waits for the lift.

## Open items
1. Apply the patch set, run the suites above, then `bun repo --tool=bazel run @semio-tech/framework-trace-rs:test` (a native-style task) twice to see `exclusive` + the toolchain key in the real graph; check `cache status`.
2. Replace the runner's "spawn the CLI and read JSON" by the in-process capture consumer (`launchRepositoryCargoV1` `output:"capture"` + `consume`) so artifact copy runs under store custody; until then production artifacts come from the native script's `dist/build/<mode>`.
3. Generate typed `repo_cargo` targets for the component pipeline (per component: cargo cdylib `-p`, describe, `jco transpile`, `wasm-opt`) instead of the uncached `component` orchestrator; the S-series shows the early cutoff this buys.
4. `execution.command` strings and `bun` are absolute paths: for a shared remote cache the `bun` attr and `s/` links must be normalized (rexec owner).
5. Library edits re-key every action (`implementation.json` input); the exact-input fix (`nativeSources`) already covers crate sources but `cargoToolchain` globs still list whole files by name.
6. `test` actions run tests in the action (verdict cached on exact inputs `nativeTestSources`); flaky/host-dependent tests need `cache: false` classes from the inference, unchanged here.
