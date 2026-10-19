# result-plugin-cache: cacheability of inferred targets (inference plugin)

Executor `plugin-cache`. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `P` = `L/🏗️build/🕸️graph/📥️inference`. No graph load, no `bun repo`, no cargo, no builds. Tools: `C/plugin-cache-tools/census.ts`, envelope runner `C/bzl-act-tools/wrap.ts`.

## 0. Verdict

PARTIAL by design. The plugin now (a) classifies every target it forces, synthesizes or refuses to replay with a closed, schema-validated decision `{version, class, reason}` in `metadata.semio.cache`, (b) honors manifest and workspace-default `inputs` on native (Cargo) targets instead of dropping them, (c) uses the integration-test input group for every target that runs Cargo tests, (d) lets role:artifact TypeScript `test` (75 packages) replay. It does NOT flip the targets whose outputs live in the managed products store (TS artifact `build`, store-slot `wasm`) or whose freshness authority is the build driver (all playground `prepare-*` / `build-*-react*`). Reasons are evidence-based (section 2); each carries a decision so the Bazel adapter can read it instead of guessing.

## 1. Changes

New (schema + tests):
- `L/🏗️build/🏃️execution/🧬️schema/💾️cache.json`: closed schema `Repository Target Cache Decision`. Classes and reasons: `deterministic` (pure-function-of-inputs, check-without-outputs), `driver-owned` (freshness-owned-by-driver), `managed-output` (outputs-are-store-links, outputs-in-managed-store), `continuous` (serves-until-stopped), `side-effecting` (publishes-live-state, publishes-remote, mutates-workspace), `live` (drives-running-process), `aggregate` (orders-prerequisites), `diagnostic` (admission-refusal, measures-environment).
- `P/🧪️tests/💾️cache/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json}`: language-agnostic corpus (closed by its own schema) + test.

Changed (`P/`):
- `📍️coordinates/🟨️.mjs`: `TARGET_CACHE_SCHEMA`, `targetCacheSchema`. `♻️revision/🟨️.mjs`: the schema is part of the implementation revision (3 anchored token edits).
- `🎯️target-policy/🟨️.mjs`: `cacheDecisionV1` (admits one decision against the schema once, frozen and shared between targets: no per-target validation), `withCacheDecision`, `uncachedDecisionV1`; `targetPolicy` attaches the decision to every refusal it makes (continuous names, `uncached`/`uncachedExact`, mutating and live names). Cache flags unchanged.
- `🎮️playground-targets/🟨️.mjs`: every synthesized target is wrapped with its decision at its construction site (session, prepare react/wgpu/native, activate react/wgpu, serve/dev, run/smoke, mcp, build react, site and crate `build` aggregates, CDN publish, refusal path). No cache flag changed.
- `🧱️project-assembly/🟨️.mjs`:
  - native targets: new exported pure helpers `nativeTargetRunsTestsV1`, `nativeTargetInputsV1`, `nativeAuthoredInputsV1`. Inputs = Cargo module closure group + command/lock groups + every manifest/default input that is not a broad group (`default`, `^default`, `production`, `^production`, `nativeSources`, `nativeTestSources` and their `^` forms) . Generated Cargo targets repeat the shared `commandInputs` verbatim in `inputs`; those are filtered (they are already one interned group), so graph size does not grow. For test-running targets the workspace `targetDefaults.test.inputs` env inputs (`SEMIO_TEST_LEVEL`, `SEMIO_TEST_BUDGET_MS`, `SEMIO_BUILD_BUDGET_MS`) are now hashed (they were dropped before: a pass at one test level could replay at another).
  - test detection: a native target without a contract runs tests when its name starts with `test` OR its command closure reaches the Cargo test driver (`/🏃️process/🧪️testing/🦀️cargo/`), so `verify-*` handlers that run integration tests use `nativeTestSources`. An explicit `execution` contract still wins.
  - role:artifact TypeScript: `test` keeps its policy/manifest cache flag (explicit `cache:false` is respected) with `outputs: []` and decision `deterministic/check-without-outputs`; `check` gets the same decision; `build` stays `cache:false` with the declared link outputs and decision `managed-output/outputs-are-store-links`.
  - wasm: store-slot producers (`wasmPackageSourceV1`) stay off with `managed-output/outputs-in-managed-store`. The dead `else if` that forced every unmarked non-wgpu `wasm` off was removed (0 such targets exist: 9 marked + wgpu; it would only have switched off caching that the wgpu test proves correct for plain `dist` trees).
  - `component` (per-crate driver pass) gets `driver-owned`.
  - robustness fix: `nativeTargetCommandInputs` returns strings AND objects (`{env}`, `{runtime}`, `{externalDependencies}`); the cargo-owner detection called `path.includes` on every element and would throw for an authored Cargo-project target whose script closure has no driver match. Now guarded with `typeof path === "string"`.
- `🧰️internals/🟨️.mjs`: exposes the new members (`cacheDecisionV1`, `withCacheDecision`, `uncachedDecisionV1`, `projectWithDefaults`, the three native helpers).

## 2. Why the big families stay uncached (evidence)

1. **TS artifact `build` (79 packages, manifests say `cache:true`)**: `dist` and `📤️dist/<mode>` are SYMLINKS into the leased products store, created by `bindTypeScriptPackageArtifactV1` (`L/⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🟦️.ts`, `plan.links`; `symlinkSync(..., "dir"/"junction")`). The binder refuses any existing non-symlink at those paths ("Foreign package output binding preserved and refused") and readers require `realpathSync(link) === store output`. A cache restore would write a real directory there: the next miss fails to bind and consumers fail. The artifact itself is staged into the store layout. So the 79 manifest `cache:true` are dead by design, now documented in data.
2. **wasm store slots (9 producers)**: outputs are the products-store slot `.cache/b/p/m/browser/<mode>/<ns>/<key>` governed by `repositoryWasmPackageRequestV1` (manifest, hash units, lease). Not a plain tree.
3. **Playground `prepare-*`, `build-*-react*`**: their command is the build driver (`📜️script.ts prepare …`, `distribution build …`) whose freshness is mtime-based (`newestComponentSourceMtime`/`stagedModuleMtime` in `♻️activation/🧰️preparation/🟦️.ts`) plus the stage cache; the sibling `component` target documents "Cargo freshness and the stage cache are the authorities", the release pipeline contract throws unless its orchestrator is uncached, and module output directories are shared between variants. `activate-*-react` (already cached) keys off the prepared bytes through `dependentTasksOutputFiles`, so content-level caching already sits one stage downstream. The pinned test `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🏗️preparation/🟦️.ts` asserts the same `cache:false` and still agrees.
4. `activate-*-wgpu` publishes live extensions/fonts: `side-effecting/publishes-live-state`. `serve|dev|mcp`: continuous. `run|smoke`: live. CDN complete: `publishes-remote`.

Precondition to flip 1-3 later (not doable inside the plugin): the products store must provide a restore protocol (ingest an output tree and re-bind links / re-issue the manifest on a cache hit), or the pipeline must declare a plain-tree deliverable. Then change `cache` + outputs at the construction sites and the one pinned test together; the decision data tells the Bazel adapter which targets those are (`class:"managed-output"|"driver-owned"`).

## 3. Census (`bun C/plugin-cache-tools/census.ts` from the repo root)

Playground grammar, 156 variants x 28 targets = 4368 (audit upper bound 4417 adds mcp/refusal paths):

| class / reason | targets | cache |
|---|---|---|
| deterministic / pure-function-of-inputs (activate-react, session) | 468 | true |
| driver-owned / freshness-owned-by-driver | 1560 | false |
| continuous / serves-until-stopped | 1248 | false |
| live / drives-running-process | 624 | false |
| side-effecting / publishes-live-state | 312 | false |
| aggregate / orders-prerequisites | 156 | false |

role:artifact TypeScript (tracked manifests): 79 `build` -> managed-output (unchanged off); 75 `test` -> replayable (was forced off; 4 explicit `cache:false` kept); 79 `check` -> deterministic. Before, none of these targets carried a reason; now every grammar target and every target the plugin forces or refuses to replay carries one.

## 4. Verification

| Command (repo root, `export SEMIO_WORKSPACE_ROOT=$PWD SEMIO_TEST_ARTIFACT_DIR=…`) | Result |
|---|---|
| `bun C/bzl-act-tools/wrap.ts ./P/🧪️tests/💾️cache/🟦️.ts` | 8 pass / 0 fail, 699 expects (schema vs Ajv strict on 19 decisions, 16 policy names, 6 test-detection rows, 3 input-plan rows, 3 authored-filter rows, real `playgroundPreparationTargets`/session/site output on a fixture workspace incl. CDN aggregator, real `inferReadOnlyBuildProjectSourcesV1` on an artifact fixture) |
| `bun C/bzl-act-tools/wrap.ts ./P/🧪️tests/🟦️.ts` (roster/imports/reads law) | pass |
| `… ./L/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` | 1 pass |
| `… ./L/🏗️build/🏃️execution/🧪️tests/🟦️.ts -t "closed source and native\|generated Cargo commands"` | 2 pass |
| `bun -e` import of `P/🟨️.mjs` + `libraryBootstrap` + sample `targetPolicy`/`cacheDecisionV1` calls | loads; refusal of a wrong class/reason pair |
| `bun build --target=bun` of project-assembly and playground-targets | ok |
| `bun C/plugin-cache-tools/census.ts` | table above |
| NOT run | `bun repo show projects`/graph load, any nx/Bazel task, the 949-line `🏗️build/🧪️tests/🕸️graph/🟦️.ts` suite (its `playground preparation shares…` test was already failing at baseline: stale fixture `publication.outputRoot` escapes its owner and the engine owner projects are missing), `🏃️execution` test 3 (needs `SEMIO_TARGET_EXECUTION_INVENTORY_FILE`), `🔌️providers` test 3 (fails on a peer-created actor test file missing from the closure; unrelated) |

## 5. Open items

1. First graph load after this change re-infers every project (implementation revision changed; schema is in the revision). Added data is about 70 bytes per decorated target (about 5k targets), perf-neutral: no per-project work was added (decision objects are frozen and shared, one `Set` per shared `commandInputs` array via `WeakMap`).
2. A native Cargo target in a fixture workspace cannot be exercised through `projectWithDefaults` hermetically (the lock-script and cargo-driver inputs live in the real library, outside any tmp workspace); the native input logic is therefore covered through its three exported pure helpers. A graph load should confirm `inputs` of a `verify-*` target now start with `nativeTestSources` and contain the three test env inputs.
3. `component-targets` (`componentBuildTargets`, `cargoTargets`, print documents, `rootCommandTargets`) are not owned here; only `component` was decorated (inside project-assembly). `build-V-site`/`build` were decorated in playground-targets.
4. The Bazel adapter can now read `metadata.semio.cache.class` (`deterministic` is the only replayable class) instead of inferring from names; the existing `cache` flag semantics are unchanged.
5. The 79 manifest `cache:true` on TS artifact `build` are inert; cache-fw/cache-rest may drop them or leave them until the store restore protocol exists.
