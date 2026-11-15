# 🧭️ Direct commands behind `repo run` for the mit-bestand demonstrator

Read-only audit. No build, no process killed, no git write. Paths are relative to `/Users/ueli/Documents/semio`. Line numbers were read from the files on disk at the time of the audit.

## 1. `repo run workspace:components -- describe energy --profile prototype`

**Target** `components` in the root `📋️project.json` (lines 4566-4587):
- executor `repo:script`, `cache: false`, `metadata.semio.cache` class `side-effecting`, reason `mutates-workspace`.
- `dependsOn`: `@semio-tech/os-plugin-describe-rs:build`, `@semio-tech/framework-plugin-web:support`.
- `options.command`: `repo ./📜️script.ts components`, `forwardAllArgs: true`. No `cwd` (workspace root), no `env` block.
- So the `-- describe energy --profile prototype` tail is appended as-is.

**Driver chain**
1. Root `📜️script.ts`: `components:` entry of the router map (line 21) lazily imports `📇️inventory/🎛️commands/🚀️release/🟦️.ts`, which re-exports `ComponentsScript` from `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🚀️release/🟦️.ts` (class at line 41).
2. `ComponentsScript.run` (line 43) for `describe` calls `describeComponentsV1` from `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts` (lines 111-116).
3. `describeComponentsV1` requires a control invocation, reads the build mode from the caller authority (`readNativeCallerAuthorityV1`), and throws unless `--profile` equals `componentProfileV1(buildModePolicy(mode).cargoProfile)`. It then calls `runRepositoryComponentDriverV1` (line 67) with `describeMode: "write"`, `facet: "model"`, `generation: "warm"`, and the environment from `buildModeEnvironment(mode)`.
4. The driver runs one Cargo pass per component workspace through `runNativeBuildDriverV1`, with cache root `<store>/components`. It prints `components describe: N component(s) described from <profile> bytes; report <path>`.

**Inputs**: the Cargo workspaces of the energy plugin (`✏️s/🔌️plugins/🔋️energy/**`), the hub composition `🌎️hub/🧩️compositions/🔋️energy/🧩️component/🔣️.json` (listed as a demonstrator input, `♻️mit-bestand/🧺️demonstrator/📋️project.json` `namedInputs`), and the describe emitter.

**Outputs**: the committed semantic descriptor pair per component, `🛂️.descriptor.semio` and `🧩️component/🔣️.json` (the pattern is confirmed in `🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs:117`). The exact owner directory for `energy` was not resolved. The `profile` (`prototype`) must match the build mode of the caller, and I did not confirm where that mode comes from when the `components` target has no `SEMIO_BUILD_MODE`.

**Direct command** (skips `repo run` planning; the two `dependsOn` targets must already be built):
```
cd /Users/ueli/Documents/semio && ./📜️script.ts components describe energy --profile prototype
```
The script has the shebang `#!/usr/bin/env repo`, so this needs `repo` on PATH. What the `repo` launcher injects into the invocation (`receiveScriptProcessInvocation`) was not verified, so treat this form as unconfirmed until one run has been made. The target as written is `repo ./📜️script.ts components describe ...`, which is what `repo run` executes after planning.

## 2. `repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype`

Project: `♻️mit-bestand/🧺️demonstrator/📋️project.json`. The `activate-prototype` target is at lines 260-294. Its `dependsOn` is `prepare-prototype` and `@semio-tech/mit-bestand-demonstrator:activate-lanes-prototype`.

Dependency chain, in order:

| Step | Definition | Executor and command | Pure script or needs planner |
|---|---|---|---|
| `activate-prototype` | l.260 | `repo:script`, `repo ./🔨️modules/🧩️runtime/📜️script.ts activate`, `SEMIO_BUILD_MODE=prototype`, cache `deterministic`, output `dist/♻️activation/prototype` | Script, but its inputs are the transitive outputs of its dependencies, so it needs the planner for ordering and cache keys |
| `prepare-prototype` (dep 1) | l.22 | `repo:script`, `repo ./🔨️modules/🧩️runtime/📜️script.ts prepare`, `SEMIO_BUILD_MODE=prototype` | Script. Depends on 9 `@semio-tech/framework-os-dev:prepare-<x>-react-prototype` targets (aggregator, aussuchen, bearbeiten, energy, fem3d, generation3d, generator, koordinator, verfolgen) and 4 wasm targets (`framework-surface-rs:wasm`, `framework-editor-rs:wasm`, `framework-os-node-graph-rs:wasm`, `puzzle-plugin:wasm`) |
| `activate-lanes-prototype` (dep 2) | l.296 | `repo:noop` | Aggregate only. Depends on `framework-os-dev:activate-energy-react-prototype`, `activate-fem3d-react-prototype`, `activate-generator-react-prototype` |

The `framework-os-dev` `prepare-*` and `activate-*` targets are not declared in `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json` (54 targets, none match). They are inferred by the graph. The inference lives in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs`:
- `activate-<variant>-react-<profile>` (lines 226-232): command `repo ./📜️script.ts activate <variant> react <profile>`, output `.cache/b/p/m/browser/<profile>/runtime/react/<variant>`, `dependsOn` the matching `prepare`.
- `prepare-<variant>-react-<profile>` (lines 234-240): command `repo ./📜️script.ts prepare <variant> react <profile>`. Its `dependsOn` includes the plugin registry session, `framework-plugin-web:support`, `semio-os-infinite:fonts`, renderer engines, and `bootComponents`. The boot components come from a graph closure, so this step needs the graph.

Pure script invocations: the `activate`, `prepare` and `activate-prototype` command bodies. The `activate` body (`🔨️modules/🧩️runtime/📜️script.ts` lines 40-47) only reads the activation receipts from `readDemonstratorActivation` and prints a count. Needs the graph or planner: the ordering of all 13 dependencies, the transitive input hashes, the component closures, and cache hits.

**What `--tool=bazel` changes**
- The neutral graph and task ids are the same. The `project.json` commands are the same, so the script invocations are unchanged (`README.md` at `🧰️framework/🛍️products/🦑️repo/README.md:172`; build guide table in `📓️build-guide.md`).
- The Bazel path generates a Bazel workspace per selection, runs each task as a Bazel action, and runs a service as a `bazel run` target.
- The planner runs again unless the plan is reused (`[bazel] planning again`, 20-60 s). The cache is the Bazel action cache plus the disk cache `bazel/actions`.
- `SEMIO_BUILD_TOOL` is exported to nested tasks, so the choice carries down.
- Nx remains the default. Bazel is not yet the default, so `--tool=bazel` is the explicit switch.

## 3. `serve` (port 6029), `test-e2e` and `build`

**`serve`** (`📋️project.json` lines 313-349)
- `dependsOn: activate-prototype`, `continuous: true`, `cache: false`.
- Command `repo ./🔨️modules/🧩️runtime/📜️script.ts serve`, `SEMIO_BUILD_MODE=prototype`. No `MIT_BESTAND_DEMONSTRATOR_PORT` is set in this target, but the script defaults to 6029 (`ServeScript`, runtime `📜️script.ts` lines 49-61).
- Pure script: yes. It reads the activation receipt (`readDemonstratorActivation`) and starts Vite from `🏗️builder/🌐️vite/🟦️.ts` on `127.0.0.1` (`0.0.0.0` when `DEVCONTAINER=true`).
- Direct: `cd ♻️mit-bestand/🧺️demonstrator && SEMIO_BUILD_MODE=prototype MIT_BESTAND_DEMONSTRATOR_PORT=6029 repo ./🔨️modules/🧩️runtime/📜️script.ts serve`. This needs a completed `activate` first.
- The sibling `dev` target (lines 112-152) is the same command with extra env (`SEMIO_BUILD_DYNAMIC_OUTPUT=false`, `SEMIO_BUILD_TERMINAL=false`, `NODE_OPTIONS=""`). It also depends on `activate-prototype`.

**`test-e2e`** (lines 416-440) is a chain:
1. `prepare-e2e` (line 351): `dependsOn` `activate-prototype` and `workspace:deps-browsers`. Command `repo ./🔨️modules/🧩️runtime/📜️script.ts prepare-test`, `SEMIO_BUILD_MODE=prototype`, class `side-effecting`, `mutates-host-store`. It opens a service session under `demonstratorE2eSessionRoot`, keyed by `DEMONSTRATOR_E2E_OWNER` and the invocation pid.
2. `serve-e2e` (line 383): `dependsOn prepare-e2e`. Command `... serve-test`, continuous. It reads that session. Direct runs must share the same invocation pid environment, or the session lookup fails.
3. `test-e2e` (line 416): `dependsOn serve-e2e`. Command `repo ./🔨️modules/🧪️e2e/📜️script.ts test` (line 425), `cache: false`. The test script calls `runTestScript` (`🧪️e2e/📜️script.ts`).
- Pure script at each step, but the session handshake between `prepare-test`, `serve-test` and `test` only works through the task runner.

**`build`** (lines 154-202)
- `dependsOn`: `prepare-release` (line 67), `framework-surface-rs:wasm`, `framework-editor-rs:wasm`, `framework-os-node-graph-rs:wasm`, `puzzle-plugin:wasm`, `semio-framework-os-flow-browser-package:wasm`, `semio-framework-canvas:stage-runtime-assets`.
- Command `repo ./🔨️modules/📦️site/📜️script.ts build` (line 167), `SEMIO_BUILD_MODE=release`, `cache: false`.
- Output `{projectRoot}/.🧾️store/build.json` (managed output in the store), `buildMode.fixed: release`, `defaultConfiguration: release`.
- `prepare-release` (line 67) runs `repo ./🔨️modules/🧩️runtime/📜️script.ts prepare` with `SEMIO_BUILD_MODE=release`. It depends on the 9 `framework-os-dev` `*-react-release` targets (inferred, same as above) and the same wasm targets.

## 4. Flags and env that affect graph reading

Nothing skips the graph for `repo run` itself. The graph is the planner, and the planner is what the task runner calls. The practical bypass is to run the target's script command directly (section 1 and 2).

- **(a) Reuse cached graph:**
  - `--graph-policy=current` serves the newest sealed graph without waiting. Declared for `graph` and `show` only (`📚️library/🏗️build/⚙️operations/🔣️catalog.json` lines 204-212). The default `fresh` waits for a rebuild when a project changed.
  - `SEMIO_BUILD_GRAPH_POLICY=current` is read for any invocation (`📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs:120`, and `📚️library/🏗️build/🧭️invocation/🟦️.ts:173-188`). The env form is the way to apply it to `run`. The `run` path strips `--graph-policy` from argv (`📥️caller/🟦️.ts:86`), so the flag is not the reliable path for `run`.
  - `SEMIO_BUILD_GRAPH_MEMO=on|off` (default `on`, `🧭️invocation/🕸️graph/🧠️memo/🟦️.ts:31`) answers a repeated identical request without starting the graph child.
  - Nx: `NX_FORCE_REUSE_CACHED_GRAPH` is set by the bootstrap from the seal (`📚️library/⚡️caching/🚀️bootstrap/📜️script.ts:532-539`; seal semantics at `📚️library/⚡️caching/🕸️graph/🔏️seal/🟦️.ts:146`). Setting `SEMIO_BUILD_GRAPH_DIGEST` makes it `false` (line 253).
  - Bazel: `SEMIO_REPO_PLAN_REUSE=off|verify` (`🔧️tools/🏗️bazel/♻️plan-reuse/`). A plan is reused when its key and witness still hold.
- **(b) Skip graph reading:** no flag. Run the script command directly, as in sections 1-3.
- **(c) Limit to the selected closure:** the resident service loads "the graph slice that exactly this selection admits" (`🏡️resident/🧬️schema/🔣️.json:263`), and the build guide describes scoped verification of an input closure (`📓️build-guide.md`, line 30). This is automatic per selection. There is no user-facing flag that forces a closure-only read on a cold start. The string "Reading the project graph slice" does not exist in the library. The nearest message is `host-load`: "Loading the project graph of {workspace}." (`🏡️resident/🧬️schema/🔣️.json:1845`).

## 5. Where the graph service persists its data

- **Partition:** `.cache/b/g`, ledger id `graph`, directory `b/g`, managed by image (`📚️library/⚡️caching/💾️storage/🧾️ledger/🔣️policy.json`, around line 54).
- **Image:** `.cache/b/g/fixed.dmg`, 6,441,402,368 bytes allocated (6.44 GB), mtime Nov 12 17:16. Mounted at `.cache/b/g/m` (`image.json` in `.cache/b/g/`).
- **Contents of the mount right now:**
  - `snapshots`: 45,123 `.gz` files
  - `build-input-closures`: 3,268 JSON files
  - `generations`: 9 entries (including `current.json`)
  - `inference`: 4 entries, `derived-nodes`: 1, `stat-index`: 1
  - `graph.json` (114 bytes), `format.json` (format `graph-records-v2`), `directory-stamps.json` (25 MB), `optional.sqlite`, `storage-events.jsonl`
- So per-owner entries exist on disk (the `build-input-closures` and `snapshots` files are per-owner or per-fact). I did not count the 683 owners directly, and I did not measure actual disk use: `du` on `.cache/b` ran past the 120 s limit and was stopped.
- **Host service endpoint:** `$XDG_RUNTIME_DIR` or `$TMPDIR/semio-repo/<uid>/`, socket `h-<tag>.sock`, descriptor `host-<tag>.json`, log `host-<tag>.log`, lock `start-<tag>.lock` (`📍️endpoint/🏠️host/🟨️.mjs`). Whether a service is currently running was not checked. `repo service host status` would answer it.
- **Source closure memo:** `~/Library/Caches/semio-repo` (build guide). Not measured.

## Next checks (not done)

- Run `repo service host status` to see whether a graph service is already up, and check its `host-<tag>.log`.
- Confirm the non-`repo` direct command form for `components describe` in one run.
- Resolve the energy owner path for the `describe` output files.
- Confirm the `activate` and `prepare` bodies of `framework-os-dev` (`prepare <variant> react <profile>`) can run without the planner.
