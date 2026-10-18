# Bazel Target Audit (Read-Only)

Scope: what `bun repo --tool=bazel run <project>:<target>` does for seven requested targets. Source paths are relative to `/Users/ueli/Documents/semio`. Nothing was built, installed, run or modified. Closures were computed by reading tracked `📋️project.json` files and the build-owner registry. The graph loader's inferred targets could not be enumerated without running the graph, so those are marked as such.

Short paths: `ADP` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/📜️script.ts`, `SEL` = `…/🏗️build/🧭️selection/🟦️.ts`, `GRF` = `…/🏗️build/🕸️graph/🟦️.ts`, `INF` = `…/🏗️build/🕸️graph/📥️inference/🟨️.mjs`, `REP` = `…/🏗️build/📜️script.ts`.

## 1. Requested Target Names Versus Manifests

| Requested | Manifest | Result |
|---|---|---|
| `@teaching/architecture-quiz:dev` | `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript/📋️project.json` | Exists. Continuous. |
| `@teaching/architecture-quiz:test-e2e` | same | Exists. Not continuous, `cache:false`. |
| `@teaching/proctor:build` | `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json` | Exists. No dependsOn. |
| `@semio-tech/semio-tech-play:activate-prototype` | `🏢️semio-tech/🎡️play/📋️project.json` | Exists. |
| `@semio-tech/semio-tech-play:serve` | same | Exists. Continuous. |
| `@semio-tech/mit-bestand-demonstrator:activate` | `♻️mit-bestand/🧺️demonstrator/📋️project.json` | **Does not exist.** Targets are `prepare-prototype, prepare-release, dev, build, test, activate-prototype, activate-lanes-prototype, serve, prepare-e2e, serve-e2e, test-e2e, canonical-architecture, prepare, test-runtime-catalog`. |
| `@semio-tech/mit-bestand-demonstrator:serve` | same | Exists. Continuous. |

Project names are unique across all tracked manifests (no duplicate `name`).

## 2. Per-Target Executor, Options And Closure

All targets in the closure use `executor: "repo:script"`, which the graph accepts (`GRF` l.81; `repo:noop` also). Commands are `bun ./📜️script.ts <sub>` or `bun ./🔨️modules/🧩️runtime/📜️script.ts <sub>` with `cwd` set to the project directory. Every `📜️script.ts` referenced exists.

### 2.1 `@teaching/architecture-quiz:dev`
- Command `bun ./📜️script.ts dev`, `cwd` = project root, `env` = `NODE_OPTIONS:""`, `SEMIO_BUILD_DYNAMIC_OUTPUT:"false"`, `SEMIO_BUILD_TERMINAL:"false"`, `VSCODE_INSPECTOR_OPTIONS:""`, `forwardAllArgs:true`, `continuous:true`, `cache:false`.
- dependsOn (transitive, complete):
  - `@semio-tech/assets:build` (`🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript`, cache true, no deps)
  - `@semio-tech/ui-styling-tokens:generate` (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust`, cache true, no deps)
- Closure size: 3 tasks, 2 non-continuous actions plus the continuous root.

### 2.2 `@teaching/architecture-quiz:test-e2e`
- Command `bun ./📜️script.ts test-e2e`, `forwardAllArgs:true`, `cache:false`, `outputs:[]`.
- Same two dependsOn as `dev`. Closure size: 3 tasks.

### 2.3 `@teaching/proctor:build`
- Command `bun ./📜️script.ts build`, `cwd` = Rust package, `cache:true`, `outputs:["{projectRoot}/dist/build"]`, no dependsOn. Closure size: 1 task.

### 2.4 `@semio-tech/semio-tech-play:activate-prototype`
- Command `bun ./🔨️modules/🧩️runtime/📜️script.ts activate`, `env.SEMIO_BUILD_MODE:"prototype"`, `cache:true`, `outputs:["{projectRoot}/dist/♻️activation/prototype"]`, `inputs:[{dependentTasksOutputFiles:"**/*",transitive:true}]`.
- dependsOn: `prepare-prototype` (local, 35 deps) plus 35 `@semio-tech/framework-os-dev:activate-<variant>-react-prototype` targets.
- **The 35 `prepare-<variant>` and 35 `activate-<variant>` framework-os-dev targets are absent from the tracked `framework-os-dev` manifest.** That manifest has 53 targets and no key containing `prototype`. They are synthesized by the graph inference (see section 4). Closure: 1 + 35 + 35 = 71 names, of which 70 are not in any tracked manifest.

### 2.5 `@semio-tech/semio-tech-play:serve`
- Command `bun ./🔨️modules/🧩️runtime/📜️script.ts serve`, `continuous:true`, `cache:false`, `metadata.semio.dashboard.ready.port=6033` (not consumed by the adapter, 0 references), dependsOn `activate-prototype` (closure = section 2.4).

### 2.6 `@semio-tech/mit-bestand-demonstrator:serve`
- Command `bun ./🔨️modules/🧩️runtime/📜️script.ts serve`, `continuous:true`, dependsOn `activate-prototype`.
- `activate-prototype` (`cache:true`, outputs `{projectRoot}/dist/♻️activation/prototype`) depends on `prepare-prototype` and `activate-lanes-prototype`.
- `prepare-prototype` (`cache:true`) depends on 9 unresolved `framework-os-dev:prepare-{aggregator,aussuchen,bearbeiten,energy,fem3d,generation3d,generator,koordinator,verfolgen}-react-prototype` plus 4 resolved wasm producers: `@semio-tech/framework-surface-rs:wasm`, `@semio-tech/framework-editor-rs:wasm`, `@semio-tech/framework-os-node-graph-rs:wasm`, `@semio-tech/puzzle-plugin:wasm`.
- `activate-lanes-prototype` (`cache:false`, command `… runtime/📜️script.ts complete`) depends on 3 unresolved `framework-os-dev:activate-{energy,fem3d,generator}-react-prototype`.
- Closure: 1 continuous root, 3 local targets, 4 resolved wasm producers, 12 unresolved framework-os-dev names.

### 2.7 `@semio-tech/mit-bestand-demonstrator:activate`
- Not a target. Selection fails before the adapter runs (`SEL` l.58-62, `Unknown build task: @semio-tech/mit-bestand-demonstrator:activate`). The adapter's own equivalent is `ADP` l.55. The name the project provides is `activate-prototype`.

## 3. Adapter Support Matrix

| Feature used | Supported | Where |
|---|---|---|
| `executor: repo:script` | Yes | `GRF` l.81-83 (normalised to `execution.kind:"command"`) |
| `options.command` | Yes | `ADP` `executeTask` l.278-293; `buildExecutionArgumentsV1` |
| `options.forwardAllArgs` | Yes | `ADP` l.283-285 |
| `options.cwd` | Yes | `ADP` `runTask` l.283 (`resolve(workspace, …)`; `workspace` = repo root, see `ADP` l.230) |
| `options.env` | Yes | `ADP` l.288 (`base`, spread into the child environment) |
| `dependsOn` string / `project:target` | Yes | `ADP` `planBazelTasksV1` l.62-87, specifically l.71-81 |
| `dependsOn` with `^`, `projects`, `dependencies` | Yes in code | `ADP` l.72-79; none occur in the requested closures |
| `continuous:true` | Yes, via `repo_launch` + `bazel run` | `ADP` l.233, l.326-347 (`run … //:<label> -- …` at l.342) |
| `cache:true` / `false` | Yes | `ADP` l.234 (`no-cache` requirement at STARLARK l.201-202); fingerprint l.165-173 |
| `outputs` with `{projectRoot}` | Yes | `ADP` `interpolate` l.121-123 |
| `outputs` with `{options.env.X}` | Only if `X` is in the task's `options.env` | `ADP` l.122 throws otherwise (see 5.3) |
| `inputs` string / `default` / `{env}` / `{runtime}` | Yes | `ADP` l.142-152, l.171 |
| `inputs` object `{dependentTasksOutputFiles, transitive}` | Parsed but **ignored** | `ADP` l.171 (only `runtime`, `env`, `externalDependencies` are read) |
| `readyWhen` | Yes, but unused by these targets | `ADP` l.341 |
| `metadata.semio.dashboard` | Not read by the adapter | 0 references |
| `configurations` / `defaultConfiguration` | Parsed; none defined on these targets, so no effect | `ADP` l.53-58 |
| Custom executors | Rejected | `GRF` l.81 |

Readiness for continuous tasks is detected only by the stdout marker `[bazel] task-started <id>`. That marker is printed before the command starts (`ADP` l.280, l.342). It does not prove the server is listening.

## 4. How The Bazel Workspace Is Generated

1. `REP` l.196-204 selects the Bazel branch. `SEL` l.58-65 parses `run project:target[:cfg]`, and `args` after `--` are forwarded.
2. `prepareBazel` installs or verifies the pinned Bazel (`🛠️tools/🏗️bazel/📜️script.ts`). `prepareBazelModulesV1` (`ADP` l.43-50) builds the fixed module catalog in `…/bazel/modules` under the **dependencies** partition, with `--lockfile_mode=update`, and writes a `ready.json` receipt.
3. `planBazelTasksV1` (`ADP` l.62-87) computes the closure, sets `label = t_<sha256(id)[0:16]>`, and sorts dependencies.
4. `publishBazelGraphV1` (`ADP` l.218-220) writes `planner-input.json` and runs the captured planner child, which returns the plan.
5. `publishCapturedBazelGraphV1` (`ADP` l.223-237) writes:
   - `MODULE.bazel` (`module(name="semio_tasks")`, `ADP` l.39 / l.225)
   - `tasks.bzl` (`STARLARK_RULES`, `ADP` l.196-215)
   - `<label>.json` manifest, `<label>.input` fingerprint
   - `BUILD.bazel` (`ADP` l.232-236): continuous tasks become `repo_launch(...)`, others `repo_task(...)`, with `deps` = non-continuous dependency labels only.
   - `runtime.json` (process environment, mode 0600)
6. `runBazelTasksInStorageV1` (`ADP` l.326-347):
   - `bazel info bazel-bin` (`ADP` l.335)
   - Continuous tasks: a separate `bazel run //:<label>` with its own `--output_base` under `outputs/launch/<ns>/<label>`
   - All selected labels: `bazel build … //:<label>` (`--keep_going` unless `bail`), then `restoreBazelSnapshotV1` for each non-continuous product (`ADP` l.260-277, restores into the repo root)
7. Non-continuous tasks run inside Bazel as `repo_task` actions (`no-sandbox`, `local`, `no-remote`). Output snapshots are captured with `captureBazelSnapshotV1` (`ADP` l.240-247).

Authored BUILD files in the source tree: none. Generated BUILD files live in the planner workspace under the tools store.

## 5. Findings: What Would Make The Run Fail

### 5.1 Hard failure, requested name wrong
- `@semio-tech/mit-bestand-demonstrator:activate` does not exist. Selection throws at `SEL` l.62 (`Unknown build task`). Use `:activate-prototype`. The adapter would throw at `ADP` l.55 if it were reached.

### 5.2 Hard failure risk, prototype targets are inferred, not authored
- The play and demonstrator closures reference 70 + 12 = 82 `@semio-tech/framework-os-dev:(prepare|activate)-<variant>-react-prototype` names (section 2.4, 2.6). None are in the tracked `framework-os-dev` manifest, which has 53 targets and no prototype keys.
- They are injected by the graph inference:
  - `INF` l.1645 and l.1771 add `playgroundPreparationTargets(...)` to `@semio-tech/framework-os-dev`.
  - `INF` l.1405-1416 generate `prepare|activate|serve|dev-<variant>-<engine>-<profile>` for every playground variant declared in plugin `Cargo.toml` metadata. The demonstrator's `fem3d`, `generator`, `energy` and the play variants must match these.
- Failure modes:
  - Variant not declared: `ADP` l.55 throws `Unknown build task`.
  - Admission refused: `INF` l.1406-1411 replaces the whole group with a `runtime-input-check` command target with `dependsOn: []`. That target runs, but does not build the prototype. The failure appears downstream, not at the Bazel step.
- Not verified: whether each variant is declared. This needs `bun repo --tool=bazel run <target> --dry-run` (`REP` l.176-184, plan JSON only) on a machine where the graph loads.

### 5.3 Adjacent latent failure, not in the requested closures
- `@teaching/architecture-quiz:build` has `outputs: ["{projectRoot}/dist/{options.env.SEMIO_BUILD_MODE}"]` but no `options.env`. `interpolate` (`ADP` l.122) throws `Build task requires option env.SEMIO_BUILD_MODE`, raised during snapshot capture (`ADP` l.241-242). Not triggered by any requested target.

### 5.4 Silent gaps, no failure
- `inputs: [{dependentTasksOutputFiles, transitive}]` on play and demonstrator `prepare-`/`activate-prototype` is ignored (`ADP` l.171). Bazel still re-runs these actions because dependency snapshots are action inputs (`STARLARK` l.199-203), so the practical risk is low.
- Continuous readiness is the stdout marker printed before start (`ADP` l.280), not a real readiness probe. `serve` and `dev` report "ready" before the server listens.

### 5.5 Preconditions blocking every Bazel run (from the prerequisite audit)
- `withBazelStorageV1` opens `tools`, `dependencies` and `products` (`🔧️tools/🏗️bazel/💾️storage/🟦️.ts` l.19-21). The module catalog lives in `dependencies/bazel/modules` (l.14). Any pending `.publication` or unmounted dependencies image refuses admission first (`ADP` l.44 `issueBuildTaskEnvironmentV1`, `storage/🌐️platform/📋️contract/🟦️.ts:63`).
- The 257-unit dependency path refusal recorded in the prerequisite audit (Dependencies run 46185) therefore also blocks the Bazel route. It is not Bazel-specific.

## 6. Unsupported Features Found (Summary)

No requested target uses an unsupported executor, custom executor, or unsupported option. The only unsupported names are:
1. `@semio-tech/mit-bestand-demonstrator:activate` (wrong name, `SEL` l.62).
2. Unresolved `framework-os-dev` prototype names, unless the graph inference synthesises them (`INF` l.1405-1416).
3. `{options.env.*}` outputs without a matching env (`ADP` l.122), only for `architecture-quiz:build`.

## 7. Validation Not Performed

- No `--dry-run`, no build, no Bazel run, no graph load. The prototype-name resolution in 5.2 is unverified.
- Recommended order, when the environment permits: `bun repo --tool=bazel tools setup bazel`, then `bun repo --tool=bazel run @teaching/proctor:build --dry-run`, then the same for `@teaching/architecture-quiz:dev`, `:test-e2e`, `@semio-tech/semio-tech-play:activate-prototype` and `@semio-tech/mit-bestand-demonstrator:activate-prototype`. The dry run prints the plan JSON and exposes unknown names before any action executes.
