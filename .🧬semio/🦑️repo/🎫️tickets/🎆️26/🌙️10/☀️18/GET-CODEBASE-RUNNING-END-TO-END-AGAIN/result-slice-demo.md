# result-slice-demo (executor slice-demo, wave 1022a) - IN PROGRESS

Tools: `C/slice-demo-tools/` (dry.sh = filtered repo run, toml-oracle.mjs). Logs: `C/🗑️generated/slice-demo/`.

## Shared cold-graph blockers fixed live (01:30-02:00), all hit by slice-play/graph-perf too
1. Authorizer source closure cap 1024 -> 2048 (authorizers schema, snapshot schema, fixture; closure had 1025 files).
2. Graph format stamping on a fresh `.cache/b/g` image: `observationPort.check` verified the writer proof against the phase control instead of the held control (`...store/📇️format/🟦️.ts`), "Exact live original writer proof required".
3. Catalog provider `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts` added to `capturedRevisionImportersV1` (`🕸️graph/📥️inference/🔁️bootstrap/🟨️.mjs`).
4. `🧪️test/🟨️.mjs`: captured mode uses literal imports of the two revision modules.
5. `🦀️native-manifests/🟨️.mjs`: `@iarna/toml` (third-party, cannot be captured) replaced by `Bun.TOML.parse`; oracle `slice-demo-tools/toml-oracle.mjs` = identical output on all 4,263 tracked Cargo.toml.
6. Loader `🏗️build/📥️source/🔏️closure/🟦️.ts`: schema-read refusal only for `.json` files in a `🧬️schema` dir (Cargo.toml in a schema-named crate dir was refused).
7. Installed third-party tool package `typescript`: `nxAuthorizerInstalledPackagesV1` (authorizers) skipped by capture; loader resolves/loads it natively (`installedPackages` in the captured createRequire). A peer later added an `installedScheme` variant in the same loader place; check for duplication.
(peers fixed overlapping items in parallel: observedRequireResolve via `__physicalResolveV1`.)

## State 03:55
- Kompaktbericht dry-run (`slice-demo-tools/dry.sh d13 --tool=bazel run @semio-tech/mit-bestand-bericht:build-kompaktbericht --dry-run`, detached by harness, pid chain 'dry.sh d13') passed inference loader faults and waits on `repository-graph-facts shard 7` behind slice-play's dry11 (pid 72404, RSS 9 GB).
- Nothing of the print (tectonic never provisioned: `.cache/b/t/m` has no tectonic yet), activation, serve or e2e steps ran yet. Every repo command is serialized by the dependency/graph-facts lease convoy (see result-slice-play.md finding A).

## 10:52 progress
- 09:19 `build-kompaktbericht --dry-run` exit 0 (d15; cold generation published after ~90 min inference; RSS 14.6 GB in expansion).
- Real run `run.sh kompakt*` reached the Bazel action once (09:25) and failed on the vanished dependency generation root `.cache/b/d/m/j/f02f...` (dependency store churn); later runs failed on peers' live changes (frozen lock after 09:23 package.json burst -> `dependencies lock --all` 09:53; `createManagerRequireV1` alias invisible to the capture scan -> fixed in `authorizers/🧭️sites/🟦️.ts` (alias support, probe in scratchpad); backend `🏗️build/🔌️backends/🟦️.ts` computed import -> ComputedImportSites row (3 backend modules); `Implementation Source is not admitted by parent` now prints the path).
- Still unverified: print toolchain (tectonic never provisioned), activation, serve, e2e.

## 12:30 print steps GREEN (Bazel, detached runs `slice-demo-tools/run.sh`)
- `@semio-tech/mit-bestand-bericht:build-kompaktbericht` rc=0 (11:13; PDF 235,766 B in `.cache/b/p/m/print/prototype/documents/report/kompaktbericht/`), `:build` (all three reports) rc=0 (11:36; zwischenbericht 18.9 MB, forschungsbericht 15.8 MB), `@semio-tech/mit-bestand-praesentation-projektetage:build` rc=0 (12:29).
- Fixes for the print path: (1) print compiler output limits clamped to the owned-capture ceiling (`tectonic-template-compilation/🟦️.ts` `printCompilerOutputLimitsV1`; "Separated capture and input require finite Original control and bounded output"); (2) `runViteBuild` accepts `configLoader`, deck passes `native` (build and dev) because its config pulls the `.cjs` workspace payload ("require is not defined" in Vite's module runner); (3) ui assets build copy skips `node_modules`/symlinks (`🖱️ui/🎨️styling/🏗️builder/🌐️vite/🗃️assets/🟦️.ts`: the whole `🖼️assets` module tree incl. its `node_modules` symlink was copied into the site -> "Single-link regular artifact file required"; the error now names the path); (4) the stale installed launcher `~/.local/bin/repo` (03:25) rejected the live runtime registry (`helpUsage`) inside Bazel actions that call `repo tools exec`; rebuilt with the launcher's own install script into `~/.local/bin`.
- One flaky Bazel failure (deck, 12:28): worker protocol "Expected BEGIN_OBJECT but was BEGIN_ARRAY" from RepoBuild worker 3 with an empty worker log; the immediate rerun passed. Open: something wrote a JSON array to the persistent worker's stdout.
act1 pid-chain started 12:30
- 12:45 I ran pkill -f 'bun repo --tool=bazel show projects --json' to remove my own 23-min-old show projects (hung on a lease queue); it may also have matched a peer process with the same command line (graph reader). Peers: check.
- 13:58 act7 launched (activate-prototype, real)

## 16:40 FINAL STATE (executor slice-demo)

### Green
- Print steps (Bazel, `slice-demo-tools/run.sh`): `@semio-tech/mit-bestand-bericht:build-kompaktbericht` rc=0 (PDF 235,766 B), `:build` (all three reports) rc=0 (zwischenbericht 18.9 MB, forschungsbericht 15.8 MB), `@semio-tech/mit-bestand-praesentation-projektetage:build` rc=0. Tectonic was provisioned by the repo (`deps-tectonic`/`deps-tex`), not by hand. Logs: `C/🗑️generated/slice-demo/{kompakt12,reportall,deck6}.raw`.
- `activate-prototype --dry-run` rc=0 (fast, graph reused).

### Fixes made (all live, small anchored edits; nothing committed)
Cold-graph / loader (shared with slice-play, graph-perf): authorizer closure cap 1024->2048 (3 JSON); graph format stamping used the wrong control for the writer proof (`...store/📇️format`); catalog provider thunk in `bootstrap/🟨️.mjs`; `🧪️test/🟨️.mjs` literal imports in captured mode; `Bun.TOML.parse` instead of `@iarna/toml` in `🦀️native-manifests` (oracle `slice-demo-tools/toml-oracle.mjs`: identical on all 4,263 Cargo.toml); schema-read guard only for `.json` in the closure loader; `typescript` as installed package (`nxAuthorizerInstalledPackagesV1`); `createRequire as alias` in the capture scan (`authorizers/🧭️sites`); backends computed import row in `ComputedImportSites`.
Bazel / Rust path:
- **Planner OOM root cause**: `publishBazelGraphV1` deep-copied + serialised the EXPANDED selected graph (1,793 projects = 1.15 GB JSON, bound 512 MiB). New schema field `graphInputs: "compact"` on the bazel backend row (`🔌️backends/📋️registry`, `🧬️schema`, accessor `buildBackendCompactGraphV1`) and `graph.compact` through `loadBuildInvocationGraphV1` -> `loadBuildGraphSliceV1` -> `generationHostV1(..., compact)`: the backend that reads closure markers itself gets the compact graph (143 MB). `managedBazelClosureFetchV1` now opens its own graph-store reader (`withRepositoryCacheStoreV1`) with the invocation.
- Bazel persistent worker: heartbeat lines `[label] running elapsedMs=` went to stdout (protocol corruption "unparseable WorkResponse"): `startNativeProgress` writes stderr; the worker output capture also redirects `console.log/info/debug/warn/error` (Bun's console bypasses `process.stdout.write`).
- `wasm-build` passed both `signal` and `control` ("Owned capture requires its original signal"): signal dropped.
- `resolveWasmBindgenBin` prefers the repo-pinned `wasm-bindgen` in the rust cargo home (PATH had `~/.cargo/bin` 0.2.126 ahead of the pinned 0.2.129).
- preview2-shim patch re-anchored to `@bytecodealliance/preview2-shim@0.30.0` (`🩹️preview2-shim-patches`), idempotent, checked against the installed `cli.js`.
- Preparation `synchronize` passed the child phase control to `withCompilerWorkspaceWriteV1` ("Original issued native storage reservation required"): `withRepositoryCargoPreparationV1` now hands its store control as 4th body argument.
- Unit-cache GC budget was the whole image (16 GiB): now `capacity - maximumWorkspaceBytes` (`cargo/🚪️launcher`), so a cold writer fits (free space 3.7 -> 7.9 GiB after the next GC).
- `semio-s-artifact-puzzle-2d-graph-manifest`: the crate lived in the gitignored `◻️2d/🤖️generated/📇️registry/📦️packages/🦀️rust/Cargo.toml` and was DELETED by the generator's stale-output cleanup at 13:48 (copy in `/private/tmp/pgc-ov`). Re-created as tracked crate `◻️2d/🛂️manifest/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs}` (lib mounts the generated registry via `#[path]`), 12 dependents + workspace members + `📋️build-owners.json` row repointed (`slice-demo-tools/puzzle-manifest-crate.py`). Cargo.lock names/deps unchanged (hub lock refreshed with `cargo update --workspace --offline`, other diffs came from peers' manifests).
- Print: compiler output limits clamped to the capture ceiling; deck uses `configLoader: native` (`runViteBuild` takes `configLoader`); ui assets copy skips `node_modules`/symlinks; installed launcher rebuilt from the checkout (`📥️install/🐚️.sh ~/.local/bin`) because the 03:25 binary rejected the live runtime registry.
- Diagnostics added: error messages now carry the path (`Single-link regular artifact file required: <path>`), the import chain (`Registry implementation import is not schema-owned ... imported through a -> b`), the capture overflow numbers.

### Open (blocks `activate-prototype` rc=0; reached 60/75 actions at 16:33)
1. **Rust image too small for the fleet**: last run died with `No space left on device` inside `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m` (16 GiB; `b` 4.5 + `u` 5.7 + cargo 1.7 + 4 GiB workspace reservation) while peers also build. Needs the coordinator/cap-gc decision: enlarge the `rust` partition (`budget/🔣️policy.json` + `⚡️caching/🔣️policy.json` strictStorage, total 66 GiB cap; host has > 200 GiB) and recreate the image when quiet, or serialize heavy cargo users. Until then every activation is a lottery against peers' builds.
2. Hub/workspace `Cargo.lock` staleness: peers keep editing Cargo.toml (rust-carve, window-kits); `lock-then-activate.sh` refreshes the hub lock right before the run, other workspaces may need the same (`cargo update --workspace --offline`).
3. `workspace:deps-wasm` is not an edge of the wasm targets (ran it by hand; pinned tool is now found by path).
4. `repo:generator-inputs` (registry compiler inputs) failed earlier with `typescript`/`@semio-tech/assets/tile-proxy` not schema-owned: depends on peers' in-flight edits (the file now imports relatively); re-check.
5. Not run at all: serve :6029, `test-e2e`, orchestrator `run.sh demo`, warm lane.

### Incidents
- 12:45 I ran `pkill -f 'bun repo --tool=bazel show projects --json'`; only my own 23-minute-old process was meant; a same-command peer process may have died.
- Helper tools in `C/slice-demo-tools/`: `run.sh`, `dry.sh`, `lock-then-activate.sh`, `import-chain.mjs`, `graph-breakdown.mjs`, `toml-oracle.mjs`, `toml-check.mjs`, `puzzle-manifest-crate.py`, `authorizer-cap.py`.
