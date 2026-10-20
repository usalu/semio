# Cache Correctness Audit 1020 (remote-cache readiness)

Auditor: `cache-audit` (read-only; writes only this report and scratch under the ticket). Snapshot taken 2026-10-20 during an active fleet, so counts can drift by one or two.

## Scope and method

- Inventory: `git ls-files '*project.json'` minus ticket copies under `.🧬semio/`, giving 1471 files (`cache-audit-tools/pj-list.txt`).
- Scripts (Bun/TS, in `cache-audit-tools/`): `scan.ts` (per-target classification, mirrors `🎯️target-policy/🟨️.mjs` and `⚡️caching/🔣️policy.json`), `shapes.ts` (token shapes), `named.ts` (named-input resolution and env-bearing named inputs), `rustbuild.ts` (Rust build outputs), `report.ts` and `top.ts` (ranking). Raw scan: `🗑️generated/cache-audit-scan.json`. All 165 heuristic rows: `cache-audit-tools/rows-all.md`.
- Static reads of the inference plugin (`🕸️graph/📥️inference/`), `nx.json`, and the Bazel adapter (`🔧️tools/🏗️bazel/📜️script.ts`). No `bun repo show projects`, no dependency sync, no Nx or Bazel run (freeze in force).
- Limit: 835 projects declare no explicit targets; their targets come from the inference plugin and are not covered by the JSON scan. Only the 3810 explicit targets were classified.

## Counts

| Measure | Value |
|---|---|
| Tracked project.json (non-ticket) | 1471; 636 with explicit targets, 835 inference-only |
| Explicit targets | 3810 |
| `cache: false` (explicit) | 423 |
| Policy-uncached or mutating (no explicit cache) | 3 |
| Policy cachedExact | 3 |
| Cached by family regex | 2818 |
| Cached by fallthrough (name matches no family; `cache: target.cache !== false`) | 563 |
| Raw `cache: true` or unset | 3387 |
| Cacheable explicit targets with no or empty `outputs` | 3083 (mostly test/check/lint, which is expected) |
| Cacheable explicit targets without explicit `inputs` | 1023 (augmented by inference for script-backed targets) |
| `cache: false` targets that still declare outputs | 13 |
| Fallthrough targets with emoji-prefixed names (no family match possible) | 106 of 233 fallthrough targets lacking explicit inputs |
| Heuristic finding rows (raw) | 165: high 146, medium 12, low 7 |

Non-finding counts from the raw scan (not reported): 2363 "explicit inputs omit toolchain env" and 300 "explicit inputs exclude project sources". Inference augmentation (`🧱️project-assembly` `genericTargetCommandInputs`, `🧭️script-inputs` closure and env inputs) adds these for script-backed targets, so the raw counts are noise.

## Classification

- **A. Confirmed in project.json (fix now):** root `prepare` (setup side effect cached); dev `build` with `inputs: []`; wgpu `browserBootSources` undeclared (5 targets); `options.env` values not hashed (7 targets); fixed `mutating` name rule miss.
- **B. Probable, confirm on the generated graph:** cacheable producers with `outputs: []` (89 non-check raw hits); outputs outside the owning project (35, most inside the same module, one level up); emoji-prefixed generator targets that fall through to cache-on (106).
- **C. Systemic policy:** fail-open classification in `targetPolicy` (any unmatched target is cached); name-regex classification defeated by emoji prefixes; `sharedGlobals` and `default` carry only `SEMIO_BUILD_MODE` and no workspace lock or toolchain files (augmented for script targets only).
- **D. Uncached but could be cached:** 6 candidates. `test-bestest-regenerate` must stay uncached (regenerates goldens).
- **E. Bazel, unverified:** `repo_task` declares only `<label>.snapshot` as its output. Real repo outputs on a remote-cache hit depend on the birth runner (`📜️script.ts` lines 478 and 610) materialising them. Not verified in this audit.

Calibration (reviewed, not findings):

1. Rust explicit `build` with `outputs: []` and `rustBuildIntent` (24 raw hits): the selected-graph path replaces the parent with an uncached noop aggregate (`🕸️graph/🦀️rust/🟦️.ts`, `selectedRustAggregateTargetV1`), and per-unit compile targets carry the store outputs. Not a finding, but this path is only exercised in the selected graph and was not run here.
2. Print `build` and `build-viz`: explicit targets are replaced by `printDocumentTargets` with `cache: false` (`🧩️component-targets`). Not a finding.
3. `describe-rs` `build`: outputs come from `hostPlatformOutputsV1` (`withHostPlatformOutputsV1`). Not a finding.
4. `leutwiler/realparts-of-powers-z-n` `build`: `repo:noop` orchestrator. Not a finding.
5. Cached command text (static): no network fetch, wall-clock, or random source found in cached targets. This covers only the command strings; the scripts they run were not read.
6. Bazel fingerprint (`📜️script.ts` around line 436): includes platform, arch, bun version, `SEMIO_BUILD_MODE`, declared env values, runtime-command stdout, and a random UUID for uncached actions. `bazelCacheDecisionV1` refuses conflicting cache declarations, and an `unbounded` report already lists cache-without-outputs targets.

## Verification still needed (after the freeze lifts, read-only)

1. Dump the generated config for the listed targets to confirm `inputs` and `outputs` after inference augmentation.
2. Confirm `browserBootSources` resolves (or not) in the wgpu graph.
3. Read `setup` in the `prepare` script to confirm its writes.
4. Run a Bazel remote-cache hit in a fresh clone with deleted outputs to confirm restore.

## Recommendations

- Fail closed in `targetPolicy`: unclassified targets default to `cache: false` unless a metadata flag says otherwise.
- Classify by metadata flag, not by target name regex, so emoji prefixes cannot bypass classification.
- Set `cache: false` on `prepare` and any setup or generate target that writes into the workspace without declared outputs.
- Give each generated tree one owner and keep outputs under `{projectRoot}`; ban `../../` outputs in lint.
- Forbid `inputs: []` on cacheable targets and fail on undeclared named inputs.
- Add `{ "env": ... }` inputs for every `options.env` key on cached targets, or drop the override.

## Top findings (48 rows, category-balanced; all 165 heuristic rows in `cache-audit-tools/rows-all.md`)

Columns: file, target, issue, suggested fix, severity (heuristic).

| # | File | Target | Issue | Suggested fix | Severity |
|---|---|---|---|---|---|
| 1 | `📋️project.json` | `prepare` | Setup side effect (`setup prepare`) is cached with `outputs: []`; a remote hit skips the setup and the generated files it writes. Target name evades the `setup` mutating rule. | Set `cache: false` (or name it under the uncached policy) and declare the written paths if it must replay. | high |
| 2 | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` | `build` | `inputs: []` on a cacheable build: none of the source closure, env or lockfile is hashed; only dependsOn results count. Command runs `completion/script.ts complete` with `outputs: []`. | Declare inputs (`default`, `^default`, the script closure, `{externalDependencies}`) and the produced directory as outputs. | high |
| 3 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/📋️project.json` | `🏭️generator-build` | Emoji-prefixed target names never match the policy family regexes (`^generator`, `^build`), so generator targets fall through to cache-on with implicit inputs and `outputs: []`. Pattern repeats for ~106 fallthrough targets (see below). | Use a name-independent classification (metadata flag) instead of the emoji-prefixed name, and declare generator outputs explicitly. | medium |
| 4 | `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📋️project.json` | `build` | Cached build writes into sibling modules (`{projectRoot}/../../icons/…`, `README.md`, `metabolism/…`) that other projects own. | Make each generated tree owned by one project and depend on it; keep outputs inside `{projectRoot}`. | medium |
| 5 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` | `test-browser, check-browser-worker, generate-browser-boot, generate-renderer-boot, browser-embedded-acceptance` | Reference the named input `browserBootSources`, which this project does not declare in its `namedInputs` (declared: default, nativeAssets, nativeSources, nativeTestSources, frameWorkerSources, reads, testEnv, cargoToolchain). Resolves only if the inference supplies it. | Declare `browserBootSources` in this project's `namedInputs`, or inline the globs. Verify the resolved inputs on the live graph. | high |
| 6 | `🧰️framework/📦️packages/🟦️typescript/📋️project.json` | `test (and all cached test/check targets)` | Explicit targets use `default` + `^default` + `testEnv` + `reads`; the default input has no `bun.lock`, root `package.json`, root `tsconfig.json` or `bunfig.toml`. Script-backed targets get these through inference augmentation, so this is a static gap that must be checked against the generated config. | Confirm the augmented inputs include `{externalDependencies}` and the toolchain files; if not, add `{workspaceRoot}/bun.lock`, `package.json`, `tsconfig.json`, `bunfig.toml` to `sharedGlobals`. | medium |
| 7 | `📋️project.json (workspace nx.json namedInputs)` | `fallthrough targets (563 explicit, plus inferred)` | Fail-open cache policy: `targetPolicy` returns `cache: target.cache !== false` for any target not matching the family regexes, so unclassified targets are cached by default with implicit inputs. | Fail closed: unclassified targets default to `cache: false` (or require an explicit `cache` flag). Ratchet the 563 explicit targets. | high |
| 8 | `🌎️hub/📦️packages/🦀️rust/📋️project.json` | `native-catalog-selection-check, native-openable-catalog-provider-check` | `CARGO_TARGET_DIR` and `SEMIO_TEST_ARTIFACT_DIR` are set in options.env but not hashed; a cache hit can replay results from another target dir or artifact dir. | Add `{ "env": "CARGO_TARGET_DIR" }` and `{ "env": "SEMIO_TEST_ARTIFACT_DIR" }` to inputs, or stop passing them per target. | medium |
| 9 | `📋️project.json` | `test-storybook (and ui-storybook-build)` | Storybook port/output env (`STORYBOOK_PORT`, `STORYBOOK_OUTPUT_DIR`) affects output location or behaviour but is not in inputs. | Add env inputs or remove the override. Ports are benign; output dir is not. | low |
| 10 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/…` | `Bazel repo_task (generated, tasks.bzl)` | Bazel `repo_task` declares only a `<label>.snapshot` directory as output, not each real repo output. Restore of real files on a remote-cache hit depends on the birth runner materialising them; not verified in this audit. | Confirm the birth/restore path on a fresh remote-cache hit before enabling remote cache for Bazel; add a test that deletes outputs and checks restore. | medium |
| 11 | `🌎️hub/📦️packages/🦀️rust/📋️project.json` | `native-catalog-selection-check` | options.env.SEMIO_TEST_ARTIFACT_DIR not in inputs {env}; the env value changes output location or behaviour but is not hashed | Add `{ "env": "NAME" }` to `inputs` or drop the env override from the cached target. | medium |
| 12 | `🌎️hub/📦️packages/🦀️rust/📋️project.json` | `native-catalog-selection-check` | options.env.CARGO_TARGET_DIR not in inputs {env}; the env value changes output location or behaviour but is not hashed | Add `{ "env": "NAME" }` to `inputs` or drop the env override from the cached target. | medium |
| 13 | `🌎️hub/📦️packages/🦀️rust/📋️project.json` | `native-openable-catalog-provider-check` | options.env.SEMIO_TEST_ARTIFACT_DIR not in inputs {env}; the env value changes output location or behaviour but is not hashed | Add `{ "env": "NAME" }` to `inputs` or drop the env override from the cached target. | medium |
| 14 | `🌎️hub/📦️packages/🦀️rust/📋️project.json` | `native-openable-catalog-provider-check` | options.env.CARGO_TARGET_DIR not in inputs {env}; the env value changes output location or behaviour but is not hashed | Add `{ "env": "NAME" }` to `inputs` or drop the env override from the cached target. | medium |
| 15 | `📋️project.json` | `test-storybook` | options.env.STORYBOOK_PORT not in inputs {env}; the env value changes output location or behaviour but is not hashed | Add `{ "env": "NAME" }` to `inputs` or drop the env override from the cached target. | medium |
| 16 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` | `test-worker-cell` | options.env.SEMIO_TEST_ARTIFACT_DIR not in inputs {env}; the env value changes output location or behaviour but is not hashed | Add `{ "env": "NAME" }` to `inputs` or drop the env override from the cached target. | medium |
| 17 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` | `ui-storybook-build` | options.env.STORYBOOK_OUTPUT_DIR not in inputs {env}; the env value changes output location or behaviour but is not hashed | Add `{ "env": "NAME" }` to `inputs` or drop the env override from the cached target. | medium |
| 18 | `♻️mit-bestand/🧺️demonstrator/📋️project.json` | `build` | `cache: false` with declared outputs and a non-mutating command; candidate for caching if deterministic | Verify determinism (no timestamps, no network); if clean, enable cache with the declared outputs. Keep uncached if it regenerates goldens. | low |
| 19 | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/📦️packages/🦀️rust/📋️project.json` | `test-bestest-regenerate` | `cache: false` with declared outputs and a non-mutating command; candidate for caching if deterministic | Verify determinism (no timestamps, no network); if clean, enable cache with the declared outputs. Keep uncached if it regenerates goldens. | low |
| 20 | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/📋️project.json` | `wasm` | `cache: false` with declared outputs and a non-mutating command; candidate for caching if deterministic | Verify determinism (no timestamps, no network); if clean, enable cache with the declared outputs. Keep uncached if it regenerates goldens. | low |
| 21 | `✏️s/🧑‍💻dev/📐️cad/📦️packages/🦀️rust/📋️project.json` | `wasm` | `cache: false` with declared outputs and a non-mutating command; candidate for caching if deterministic | Verify determinism (no timestamps, no network); if clean, enable cache with the declared outputs. Keep uncached if it regenerates goldens. | low |
| 22 | `🏢️semio-tech/🎡️play/📋️project.json` | `build` | `cache: false` with declared outputs and a non-mutating command; candidate for caching if deterministic | Verify determinism (no timestamps, no network); if clean, enable cache with the declared outputs. Keep uncached if it regenerates goldens. | low |
| 23 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust/📋️project.json` | `wasm` | `cache: false` with declared outputs and a non-mutating command; candidate for caching if deterministic | Verify determinism (no timestamps, no network); if clean, enable cache with the declared outputs. Keep uncached if it regenerates goldens. | low |
| 24 | `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json` | `generate-actor-network` | Output resolves outside the owning project: `{workspaceRoot}/♻️mit-bestand/📋️bericht/📎️anhang/🕸️akteursnetz-figuren.tex` | Move the output under `{projectRoot}`, or make the owning project produce it and depend on it with `dependsOn`. Sibling-directory writes break per-project cache ownership. | medium |
| 25 | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust/📋️project.json` | `graph-generate` | Output resolves outside the owning project: `{projectRoot}/../../🤖️generated` | Move the output under `{projectRoot}`, or make the owning project produce it and depend on it with `dependsOn`. Sibling-directory writes break per-project cache ownership. | medium |
| 26 | `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/📦️packages/🦀️rust/📋️project.json` | `graph-generate` | Output resolves outside the owning project: `{projectRoot}/../../🤖️generated` | Move the output under `{projectRoot}`, or make the owning project produce it and depend on it with `dependsOn`. Sibling-directory writes break per-project cache ownership. | medium |
| 27 | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust/📋️project.json` | `graph-generate` | Output resolves outside the owning project: `{projectRoot}/../../🤖️generated` | Move the output under `{projectRoot}`, or make the owning project produce it and depend on it with `dependsOn`. Sibling-directory writes break per-project cache ownership. | medium |
| 28 | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust/📋️project.json` | `graph-generate` | Output resolves outside the owning project: `{projectRoot}/../../🤖️generated` | Move the output under `{projectRoot}`, or make the owning project produce it and depend on it with `dependsOn`. Sibling-directory writes break per-project cache ownership. | medium |
| 29 | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/📦️packages/🦀️rust/📋️project.json` | `graph-generate` | Output resolves outside the owning project: `{projectRoot}/../../🤖️generated` | Move the output under `{projectRoot}`, or make the owning project produce it and depend on it with `dependsOn`. Sibling-directory writes break per-project cache ownership. | medium |
| 30 | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json` | `graph-generate` | Output resolves outside the owning project: `{projectRoot}/../../🤖️generated` | Move the output under `{projectRoot}`, or make the owning project produce it and depend on it with `dependsOn`. Sibling-directory writes break per-project cache ownership. | medium |
| 31 | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` | `graph-generate` | Output resolves outside the owning project: `{projectRoot}/../../🤖️generated` | Move the output under `{projectRoot}`, or make the owning project produce it and depend on it with `dependsOn`. Sibling-directory writes break per-project cache ownership. | medium |
| 32 | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📋️project.json` | `graph-generate` | Output resolves outside the owning project: `{projectRoot}/../../🤖️generated` | Move the output under `{projectRoot}`, or make the owning project produce it and depend on it with `dependsOn`. Sibling-directory writes break per-project cache ownership. | medium |
| 33 | `🌎️hub/📦️packages/🦀️rust/📋️project.json` | `trusted-stdio-gis-bundle-check` | cmd produces files but outputs empty: bun ./📜️script.ts trusted-stdio-gis-bundle-check | Declare outputs (see above). | high |
| 34 | `🌎️hub/📦️packages/🦀️rust/📋️project.json` | `trusted-stdio-gis-bundle-browser-check` | cmd produces files but outputs empty: bun ./📜️script.ts trusted-stdio-gis-bundle-check --browser | Declare outputs (see above). | high |
| 35 | `🌎️hub/📦️packages/🦀️rust/📋️project.json` | `trusted-stdio-gis-bundle-process-check` | cmd produces files but outputs empty: bun ./📜️script.ts trusted-stdio-gis-bundle-check --process | Declare outputs (see above). | high |
| 36 | `📋️project.json` | `test-runtime-build-producer` | cmd produces files but outputs empty: bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts test runtime-build-producer | Declare outputs (see above). | high |
| 37 | `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json` | `build` | Cached on `on-family` with `outputs: []`; name/command indicate a produced artifact (`bun ../../🔨️modules/📄️documents/📜️script.ts complete`) | Declare real outputs under `{projectRoot}`, or `cache: false` if the writes are not a replayable result. Confirm on the generated config before changing. | high |
| 38 | `♻️mit-bestand/🧺️demonstrator/📋️project.json` | `prepare-prototype` | Cached on `on-default` with `outputs: []`; name/command indicate a produced artifact (`bun ./🔨️modules/🧩️runtime/📜️script.ts prepare`) | Declare real outputs under `{projectRoot}`, or `cache: false` if the writes are not a replayable result. Confirm on the generated config before changing. | high |
| 39 | `♻️mit-bestand/🧺️demonstrator/📋️project.json` | `prepare-release` | Cached on `on-default` with `outputs: []`; name/command indicate a produced artifact (`bun ./🔨️modules/🧩️runtime/📜️script.ts prepare`) | Declare real outputs under `{projectRoot}`, or `cache: false` if the writes are not a replayable result. Confirm on the generated config before changing. | high |
| 40 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/📋️project.json` | `🏭️generator-build` | Cached on `on-default` with `outputs: []`; name/command indicate a produced artifact (`bun ./📜️script.ts build`) | Declare real outputs under `{projectRoot}`, or `cache: false` if the writes are not a replayable result. Confirm on the generated config before changing. | high |
| 41 | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/📋️project.json` | `🏭️generator-build-manifests` | Cached on `on-default` with `outputs: []`; name/command indicate a produced artifact (`bun ./📜️script.ts build-manifests`) | Declare real outputs under `{projectRoot}`, or `cache: false` if the writes are not a replayable result. Confirm on the generated config before changing. | high |
| 42 | `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/📋️project.json` | `shared-schema` | Cached on `on-default` with `outputs: []`; name/command indicate a produced artifact (`bun ./📜️script.ts shared-schema`) | Declare real outputs under `{projectRoot}`, or `cache: false` if the writes are not a replayable result. Confirm on the generated config before changing. | high |
| 43 | `✏️s/🧑‍💻dev/🗄️stdio/📦️packages/🟦️typescript/📋️project.json` | `package-contract` | Cached on `on-family` with `outputs: []`; name/command indicate a produced artifact (`bun ./📜️script.ts package-contract`) | Declare real outputs under `{projectRoot}`, or `cache: false` if the writes are not a replayable result. Confirm on the generated config before changing. | high |
| 44 | `✏️s/🧑‍💻dev/🗄️stdio/📦️packages/🟦️typescript/📋️project.json` | `package-graph` | Cached on `on-default` with `outputs: []`; name/command indicate a produced artifact (`bun ./📜️script.ts package-graph`) | Declare real outputs under `{projectRoot}`, or `cache: false` if the writes are not a replayable result. Confirm on the generated config before changing. | high |
| 45 | `🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/📋️project.json` | `surface-render-source` | Cached on `on-default` with `outputs: []`; name/command indicate a produced artifact (`bun ./📜️script.ts surface-render-source`) | Declare real outputs under `{projectRoot}`, or `cache: false` if the writes are not a replayable result. Confirm on the generated config before changing. | high |
| 46 | `🏢️semio-tech/🎡️play/📋️project.json` | `prepare-prototype` | Cached on `on-default` with `outputs: []`; name/command indicate a produced artifact (`bun ./🔨️modules/🧩️runtime/📜️script.ts prepare`) | Declare real outputs under `{projectRoot}`, or `cache: false` if the writes are not a replayable result. Confirm on the generated config before changing. | high |
| 47 | `👴️leutwiler/💤️realparts-of-powers-z-n/📋️project.json` | `build-lean` | Cached on `on-family` with `outputs: []`; name/command indicate a produced artifact (`bun ./📜️script.ts build lean`) | Declare real outputs under `{projectRoot}`, or `cache: false` if the writes are not a replayable result. Confirm on the generated config before changing. | high |
| 48 | `👴️leutwiler/💤️realparts-of-powers-z-n/📋️project.json` | `build` | Cached on `on-family` with `outputs: []`; name/command indicate a produced artifact (``) | Declare real outputs under `{projectRoot}`, or `cache: false` if the writes are not a replayable result. Confirm on the generated config before changing. | high |

Notes on the table: the `graph-generate`, `typegen`, and `generate-actor-network` outputs land one level above the package root but stay within their owning artifact or module; they are medium because cache ownership is blurred, not because they leave the workspace. The `prepare` row is the one to fix first. Rows 1 to 3 and the `browserBootSources` row are the strongest because they are visible in project.json without the inference.

## Final classification

- Fix now (A): `prepare`, dev `build` (`inputs: []`), wgpu `browserBootSources`, the seven `options.env` gaps.
- Confirm on the generated graph (B): producer targets with empty outputs, outside-project outputs, emoji-prefixed generators.
- Policy (C): fail-open fallthrough, name-regex classification, workspace lock and toolchain files outside `default`.
- Could cache (D): six candidates, with `test-bestest-regenerate` excluded.
- Bazel (E): snapshot-only outputs, restore unverified.

Scratch and raw output: `cache-audit-tools/` (scripts, `pj-list.txt`, `rows-all.md`) and `🗑️generated/cache-audit-scan.json`.
