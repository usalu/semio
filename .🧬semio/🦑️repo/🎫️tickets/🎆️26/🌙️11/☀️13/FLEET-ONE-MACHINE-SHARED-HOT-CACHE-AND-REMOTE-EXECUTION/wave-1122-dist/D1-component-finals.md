# D1: Component finals, stage cache and fleet tier (read-only audit)

Scope: `🏢️semio-tech/🎡️play` `build` (`🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts build`). No code edited, no git writes. Paths are relative to `/Users/ueli/Documents/semio/`; `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `NB` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build`, `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`.

## 0. Correction to the earlier audit

`T/audit-wasm-and-still-twice-1114.md` says transpile, wasm-opt and probe use a local-only stage cache. The current tree no longer matches that. All three stage calls pass a `slot`, and the driver wires the fleet decorator, so the fleet tier is engaged whenever fleet settings exist:

- transpile: `PL/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:124`, slot `base`
- optimize: same file `:139`, slot `base/<file>`
- probe: same file `:165`, slot `componentBase`
- describe: `PL/🖨️describe/🧩️component-stage/🟦️.ts:~93`, slot `packageName`
- decorator: `PL/🏗️build/🧩️driver-stages/🟦️.ts:111` (`decorateStageCache: stageFleetDecoratorV1(...)`), wired by `componentDriverStagesV1` / `runRepositoryComponentDriverV1`

Gaps that remain: the progress line (`NB/🧩️component-pass/🟦️.ts:694`) prints `cache.stats()`, and `stats()` in `LIB/⚡️caching/☁️stage-fleet/🟦️.ts` (inside `sharedStageCacheV1`, ~line 186) folds fleet `served` into `hits`. So `stage cache H/M` cannot tell local hits from fleet hits. Fleet `shared` / `waited` / `skipped` are only printed as events via `stageFleetEventTextV1` (`driver-stages:111`). Whether fleet settings exist on a given machine is a runtime question, not answered by the code.

## 1. Native-build driver map

**Cargo invocation and unit cache**
- Driver entry: `NB/🧩️component-pass/🟦️.ts:438` `runNativeBuildDriverV1`. The stage cache is built at `:459`: `cache = (options.decorateStageCache ?? identity)(stageCacheV1(generation.stages))`.
- Cargo passes run through the repo rustc wrapper. `NB/🎟️admission/🟦️.ts:412-416` `compilerWrapperEnvironmentV1` sets `RUSTC_WRAPPER = SEMIO_UNIT_CACHE_WRAPPER`, `SEMIO_UNIT_CACHE_ROOT`, the scheduler inner chain and `SEMIO_UNIT_CACHE_LANE=g`. It returns `{}` when no `SEMIO_UNIT_CACHE_ROOT` / `SEMIO_UNIT_CACHE_WRAPPER` is named or when `SEMIO_UNIT_CACHE=off`. The unit cache script reads the same root at `LIB/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts:36`.
- Off switches:
  - `SEMIO_UNIT_CACHE=off`: disables the whole wrapper chain (`admission:415`, `unit-cache/📜️script.ts:36`).
  - `SEMIO_UNIT_CACHE_REMOTE=off`: disables the unit fleet tier (`LIB/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🟦️.ts:17`, and `🚪️launch/🟦️.ts:94` `fleetRemoteEnvironmentV1`).
  - `SEMIO_STAGE_CACHE_REMOTE=off`: disables only the stage fleet tier (`LIB/⚡️caching/☁️stage-fleet/🟦️.ts:207-214` `stageFleetClientV1`).
- Live compile units: `NB/🎟️admission/🦀️.rs` (`semio-build-scheduler`, the RUSTC_WRAPPER) and the REAPI sidecar under `LIB/🏗️build/🌐️remote-execution/🛰️sidecar/`.

**Components and stages**
- Stage model: `NB/🧩️component-pass/🟦️.ts:88-94` (`LinkedComponentV1`, `NativeBuildStageContextV1` with `cache: StageCacheV1`, `NativeBuildStageV1` with `after`, `capacity`, `run`).
- Per-component stage plan built in `PL/🏗️build/🧩️driver-stages/🟦️.ts:56-66`: `describe`, `model`, `transpile`, `optimize`, `browser`, `complete`, `discard`.
- The component is "staged" (`NB/🧩️component-pass/🟦️.ts:765`, `:770` `options.stages?.(linked, driverScope)`) the moment Cargo links it. Progress line `:694`, end summary `:1223`, report section "Stage cache" `:1213`.
- Build report paths: `NB/🧩️component-pass/🟦️.ts:106-122` `nativeBuildGenerationV1` / `nativeGenerationLayoutV1`. Warm generation: the stage cache lives at `<cacheRoot>/stages` (`cacheRoot = store.root/components`, `driver-stages:111`). Fresh generation (`SEMIO_NATIVE_BUILD_GENERATION=fresh`, set by `🏢️semio-tech/🎡️play/🔨️modules/📦️site/🆕️fresh-build/🟦️.ts`): `<generation>/s`, cold.

**Stage cache: key, store, locality**
- Local store: `NB/🗃️stage-cache/🟦️.ts:117` `stageCacheV1(root)`. Key = `stageKeyDigestV1` = sha256 of canonical JSON of `{stage, schemaVersion, inputs, tools}` (`:67-75`, `:88`). Entry directory `<root>/<stage>/<digest>` with a manifest. Publish is an atomic rename (`:~150`). Restores copy the files, they are never linked.
- Fleet tier: `LIB/⚡️caching/☁️stage-fleet/🟦️.ts`.
  - `:13` `stageRemoteKeyV1`: sha256 of `[policy.namespace, "stage", platform, digest]`.
  - `:157` `sharedStageCacheV1`. A request without a `slot` goes straight to the local cache (`if(slot===undefined) return inner.run`). With a slot it does: fleet fetch, or wait on a claim (flight), or compute and publish (manifest + files by digest).
  - `:216` `stageFleetDecoratorV1` (the decorator applied by the driver).
- Remote endpoint: `LIB/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🟦️.ts:17` `readFleetSettingsV1` (needs `SEMIO_REMOTE_CACHE_ENDPOINT`, read from env or from the machine's fleet facts via `fleetRemoteEnvironmentV1`). Upload needs `SEMIO_REMOTE_CACHE_ACCESS=read-write`.

## 2. Per-component final steps: cache, key, locality, offloadability

| Step | Where | Cached? | Key | Locality | Offloadable today? |
|---|---|---|---|---|---|
| cargo link of wasm32 cdylib | Cargo via `RUSTC_WRAPPER` (`admission:412`) | yes, unit cache | unit key; `unit-cache/🛰️fleet/🧬️schema/🔣️.json:32` lists `hostIndependent.targetPrefixes: ["wasm32-"]`; cdylib in `🧭️remap/🟦️.ts:61` | unit local + fleet | yes: REAPI sidecar (`LIB/🏗️build/🌐️remote-execution/`) |
| describe | `PL/🖨️describe/🧩️component-stage/🟦️.ts:~93` | yes, stage | `{component}` digest, `tools.emitter` | stage: local + fleet (slot = packageName) | no, runs in-process |
| model | `PL/📦️model/🟦️.ts` (`publishComponentModelV1`) | no | n/a | n/a | no |
| transpile (jco) | `materialization/🚀️commands/🟦️.ts:115-125` | yes, stage | inputs `{component: sha256(stripped wasm)}`; tools `materialization` (sha of 4 source files `:98-103`), `base`, `native-transpile` (binary digest) | stage local + fleet (slot = base) | no, runs node/jco in-process |
| optimize (wasm-opt -Oz, per core) | same file `:132-145` | yes, stage | inputs `{core: digest}`; tools `wasm-opt` (`--version` string), `args` | stage local + fleet (slot = base/file) | no |
| browser assemble | `publishComponentBrowserV1` `:186` | no | n/a | n/a | no |
| probe (module descriptor) | same file `:155-170` | yes, stage | inputs = module and vendor file digests; tools `node` version, `probe` = sha of probe source | stage local + fleet (slot = componentBase) | no |
| packaging / receipt | `📦️model/`, `driver-stages` `complete` / `discard` | no | n/a | n/a | no |
| Vite bundle / pages | `🏢️semio-tech/🎡️play/🔨️modules/📦️site/🟦️.ts:24-35` (`buildReleasePipelineV1` then `publishPlayPageCohortV1`) | no | n/a | n/a | no |

Note on the transpile key: `materializationIdentityV1` covers the four `.ts` sources but `preview2VendorDir` (`join(builtPluginModulesRootIn(...), PREVIEW2_VENDOR_RELATIVE)`, `:124`) is an input that is not in the key. Node/jco version is not in the key either. A shared cache would be unsafe until these are keyed (see step 3).

## 3. Minimal clean change: one fleet cache, component finals remote-executable

The store is already shared: one fleet endpoint (6094) and one content-addressed manifest store. What differs is the key namespace (`stageRemoteKeyV1` vs the unit cache key) and the fact that the stage closures run only on the coordinator. Proposed change set, in order:

1. Make keys complete (correctness before sharing):
   - `PL/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:105-106` `transpileStageRequestV1`: add the sha256 of the preview2 vendor tree, the node/jco version and the profile (`mode`) to `inputs`/`tools`, bump `schemaVersion` to 3.
   - Same file `:110-111` `optimizeStageRequestV1`: confirm wasm-opt output is independent of `cores`; if so keep, else add it.
   - `:155-156` `probeStageRequestV1`: already complete (module and vendor digests, node version, probe source hash).
2. One namespace: replace the per-tier key in `LIB/⚡️caching/☁️stage-fleet/🟦️.ts:13` `stageRemoteKeyV1` with the unit cache's key function, so a stage digest and a unit digest live in one keyspace under one `policy.namespace` (`stage-fleet/🧬️schema/🔣️.json` `Policy`). Keep the local stage store as a local tier only.
3. Remote execution of stage produce closures: add a stage job to the scheduler (`LIB/🏗️build/🌐️remote-execution/🗓️scheduler/🟦️.ts`), and in `sharedStageCacheV1`'s `wrapped` closure (`stage-fleet/🟦️.ts`, inside `run`, the `produce(directory)` call) dispatch the serialized request (`stage`, `key`, input digests) to a ready worker when one is available, falling back to local `produce`. Inputs travel as digests through the fleet CAS (`publishStage` already uploads files by digest). Each stage closure (`transpilePluginComponentModulesAsync`, `optimizePluginCoreModuleAsync`, `probeModuleDescriptor`, the describe emitter) needs a serializable spec first; today they capture `work.repoRoot`, `work.environment`, `control`.
4. Progress: split `cache.stats()` hits into local and fleet in `NB/🧩️component-pass/🟦️.ts:694` so the line reports both.

Not proposed: a second cache store, a migration, or a separate remote-execution path for finals. Remote execution reuses the scheduler and REAPI sidecar already used for units.

## 4. Nx / repo-run caching of the play site build

- `🏢️semio-tech/🎡️play/📋️project.json`, target `build`: `"cache": false`, command `repo ./🔨️modules/📦️site/📜️script.ts build`, output `.🧾️store/build.json`. The `serve`, `dev`, `test-*` targets are also `cache: false`. Only `prepare-prototype` and `activate-prototype` (prototype activation, not the release build) and `test` are cacheable.
- The fresh release graph (`🆕️fresh-build/🟦️.ts:17`) runs `repo run @semio-tech/semio-tech-play:build --skip-cache --skip-remote-cache`.
- `nx.json` has no `tasksRunnerOptions` and no `cacheableOperations` for `build`. The `6094` references in the repo are the repo's own fleet cache (`LIB/…/🛰️fleet`, `🤝️shared-worktree/🗺️topology`), not an Nx remote cache.
- Conclusion: no Nx or repo-run caching applies to the play build. The only caching is the driver-internal unit and stage tiers described above.

## Open items

- Runtime check not done (read-only): whether fleet settings (`SEMIO_REMOTE_CACHE_ENDPOINT`, access) are present on the build machine. Verify by the fleet event lines (`stageFleetEventTextV1`) of one play build.
- The stage closures are not yet serializable (section 3 step 3), so offloading is design, not existing code.
- Unverified: whether the play component cargo pass actually sets `SEMIO_UNIT_CACHE_ROOT` in the launcher environment (not traced to the launcher).
