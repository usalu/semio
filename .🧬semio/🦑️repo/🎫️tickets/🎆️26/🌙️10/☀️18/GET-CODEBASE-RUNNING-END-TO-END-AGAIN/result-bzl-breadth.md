# result-bzl-breadth: Bazel workspace generation for every project and target (static graph)

`B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`, `C` = this ticket. Bazel 8.4.2 (provisioned binary), macOS arm64, output base `/private/tmp/bzlbreadth-ob` (ASCII), install base `/private/tmp/bzlbreadth-install`. No `bun repo`, no cargo, no builds executed (analysis only).

## Counts
| Item | Value |
|---|---|
| Projects (static reconstruction, tracked `📋️project.json`, nx ignore rules of the census) | 1,446 (census said 1,447; one project file vanished concurrently) |
| Explicit targets | 3,793 |
| Synthesized playground targets (156 variants, grammar of `🎮️playground-targets`, 28 per variant plus site/crate aggregates and the CDN aggregate; attached to framework-os-dev / plugin-registry / crate projects with the plugin's cache classes) | 4,217 |
| Targets planned and selected (`run-many --targets=*` equivalent) | 8,010 (dangling `dependsOn` pruned: 4; cycles: 0) |
| Dependency edges (package.json + implicit; Nx import inference not reproduced) | 494 |
| Rules generated | 8,010 task-like (6,701 `repo_task`, 1,308 `repo_launch`, 1 `repo_noop`) + 859 `filegroup` = 8,869 |
| Cacheable actions | 3,051 |
| Uncached with reason | continuous 1,308; driver-owned 1,560; unbounded 808; side-effecting 313; live 624; declared 346 |
| BUILD.bazel | 316,691,483 bytes (1,737,029 `s/...` source labels in 859 deduplicated filegroups); workspace 14,717 files, 365 MB |

## Analysis results
| Command | Before fixes | After fixes |
|---|---|---|
| generate workspace | failed: `Build native artifact authority changed` (5 targets set their own Cargo directories) after 8 min | OK, 345 s generate + 5 s implementation capture |
| `bazel query //...` | `ERROR: infinite symlink expansion` (the generated workspace lives inside the checkout; `s` links back to it) | OK 31 s, `--output=label_kind`: 8,869 lines (859 filegroup, 1,308 repo_launch, 1 repo_noop, 6,701 repo_task), 0 errors |
| `bazel build --nobuild //...` | not reachable | OK: "Analyzed 8869 targets (144967 targets configured)", 27 s, 0 errors, 0 actions |
| `bazel cquery deps(<target>)` x 8 | not reachable | OK, table below |

Starlark errors: 0. Analysis errors: before 1 (query crawl) + 1 (generator refusal); after 0.

## Product closures (`cquery deps`, rules = task/launch/noop rules; sources = checkout files reached)
| Task | rules | filegroups | source files |
|---|---|---|---|
| `@teaching/architecture-course:build` | 3 | 2 | 546 |
| `@teaching/architecture-course:test` | 1 | 1 | 3,283 |
| `@teaching/proctor:build` | 1 | 1 | 48 |
| `@teaching/proctor:test` | 1 | 1 | 198 |
| `@semio-tech/semio-tech-play:activate-prototype` | 107 | 2 | 590 |
| `@semio-tech/semio-tech-play:build` | 1 | 0 | 4 |
| `@semio-tech/mit-bestand-demonstrator:activate-prototype` | 28 | 5 | 970 |
| `@semio-tech/mit-bestand-demonstrator:build` | 24 | 5 | 962 |

Of 8,010 task rules each product pulls 1 to 107: the minimal closure holds. Caveat (lower bounds): the static graph has no Nx import-inferred or Cargo path-dependency edges (proctor `^cargo` closure 48 files), and the synthesized playground targets depend only on the session target, not on the wasm/engine closure, so play/demonstrator closures are understated until the real graph is loaded.

## Changes (all before the coordinator's freeze message; no further edits after it)
Adapter `B/📜️script.ts` (small anchored edits via exact-count replace, build verified with `bun build --target=bun`):
1. `generateBazelWorkspaceV1` (exported): the workspace-writing half of `publishCapturedBazelGraphV1` split out (takes plan, workspace dir, invocation, phase control, implementation), so generation runs without storage/partitions/Bazel. `publishBazelImplementationV1` exported.
2. `linkBazelSources` also writes `.bazelignore` containing the link name (`s`): `//...` / `rdeps(//...)` queries (bzl-ops affected parity, any user query) otherwise crawl the checkout and fail with an infinite symlink expansion, because the production workspace sits under `.cache/` inside it. Probe-verified: explicit `s/<emoji path>` labels still resolve with the ignore.
3. `taskFingerprint` runs each `{runtime}` input once per distinct command text and build mode (Nx runtime-hasher semantics) instead of 2 `bun exec` spawns per cacheable task.
4. `bazelTaskCargoDirectoriesV1` / `issueBazelTaskEnvironmentV1` (exported): `CARGO_TARGET_DIR`, `CARGO_BUILD_TARGET_DIR`, `CARGO_BUILD_BUILD_DIR` declared in a target's `execution.env` (os-hub native-catalog-selection-check / native-openable-catalog-provider-check x2, workspace runtime-fixture-graph, one configuration) are task data: removed from `bazelTaskBaseEnvironmentV1`, laid over the issued environment resolved against the workspace. Previously the caller authority check refused every such task for fingerprint, snapshot capture and execution, so they could never run under Bazel. Applied at the fingerprint, snapshot-environment, run and orchestration call sites.

Tools (`C/bzl-breadth-tools/`): `graph.ts` (static BuildGraphV1 + playground grammar), `run.ts` (issued process envelope launcher, selection `run-many --targets=*`), `gen.ts` (prune dangling/cycles, plan, publish implementation, generate), `bz.sh` (Bazel with output base /private/tmp), `products.ts` (8 product labels).

## Verification
- `bun C/bzl-breadth-tools/graph.ts`; `BZL_BREADTH_WS=<ws> BZL_BREADTH_SELECTION='run-many --targets=*' bun C/bzl-breadth-tools/run.ts C/bzl-breadth-tools/gen.ts` (6 min); `bz.sh query //... --output=label_kind`; `bz.sh build --nobuild //...`; `bz.sh cquery 'deps(//:t_...)' --output=label_kind` x 8. All outcomes above.
- NOT run: adapter test suites after these edits, any real Bazel build/run, Linux/Windows. No language-agnostic fixture test yet for items 2 and 4 (queued, adapter frozen).
- Cleanup done: servers shut down, output base, install base, probe dir and `🗑️generated/bzl-breadth*` removed.

## Open items (queued for after the freeze)
1. Tests: fixture rows for `bazelTaskCargoDirectoriesV1` (Python `os.path.join` cross-check) and `.bazelignore` content; rerun `C/bzl-act-tools/wrap.ts` suites (actions, fingerprint).
2. Scale: 316 MB BUILD with 1.74 M repeated file labels (generate 345 s). Compose per-project filegroups (one per project root and named input) and reference them via `deps`/nested `srcs` instead of listing every file in every distinct set; also `s/...` globs avoided because they stop at nested packages. Not changed here.
3. 808 targets are `cache:true` or deterministic without an `outputs` array (`unbounded`): add `outputs: []` or real outputs in manifests (cache-* owners).
4. 4 dangling `dependsOn` (real graph would throw `Unknown build task` for any `run-many`): `semio-hub-gis-component:component` (x3 consumers: gis-gismap-rs:verify-inference-mcp, gis-plugin:inference-discovery-oracle/-check) and `@semio-tech/trinity-jack-lsp:wasm` (s-dev-composition:typecheck; the trinity LSP shim was deleted). May be project-name or static-reconstruction artifacts: recheck against the real graph.
5. `^input` expansion in `taskInputs` is direct-dependency only (Nx hashes transitively); correct only while outputs flow through restored snapshots. Needs a decision.
6. Rerun this harness against the real graph (`graph.json` swap via `BZL_BREADTH_GRAPH`) once the dependency sync finishes.
