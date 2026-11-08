STATUS: UNVERIFIED. Static, read-only audit of `repo run @semio-tech/mit-bestand-demonstrator:build` (lane aud-release, wave 1107). No repo run, no build, no edit outside this file.

# 1107 Release Build Audit

Scope: the definition-of-done target `@semio-tech/mit-bestand-demonstrator:build` (static site, `prepare-release`, four engine wasm tasks, flow browser wasm, canvas runtime assets, map tile prefetch, vite artifact, store receipt `.🧾️store/build.json`). Brief: `1107-brief.md` addendum 04:08. Earlier notes checked: memory `project-component-release-does-not-materialize` (2026-09-09) and `project-release-serve-wedges-after-host-edit-bursts` (2026-09-16); both old, and only the first is relevant to release descriptors.

## 1. Verdict

- Target names: all valid statically. No dangling name found in the `build` chain.
- Release-only failure modes found, ranked in section 5. The likeliest to fail the first real run are the descriptor materialization (memory 09-09 says `component-release` alone does not write the served release modules) and the package opt-level asymmetry (release drops the prototype-only overrides, which can break timing tests only in release). The tile bundle does not fail the build; it makes the output too large and not reproducible (about 600 MB on this machine).
- No earlier release run exists to take timings from (section 7).
- Nothing here is proven. The first real run is the proof.

## 2. Exact chain (what `repo run ...:build` executes)

Paths relative to `♻️mit-bestand/🧺️demonstrator` unless noted. Project file: `📋️project.json`.

1. Mode resolution (framework `🧭️invocation/🟦️.ts:159-166`): `build` has `defaultConfiguration: release` and `metadata.semio.buildMode.fixed: release`. Any inherited `SEMIO_BUILD_MODE=prototype` in the shell throws "Conflicting build modes". Run it with `SEMIO_BUILD_MODE` unset.
2. `build` depends on, in graph order (not guaranteed):
   a. `prepare-release` (`repo ./🔨️modules/🧩️runtime/📜️script.ts prepare`, env `SEMIO_BUILD_MODE=release`). It depends on nine inferred targets `@semio-tech/framework-os-dev:prepare-<lane>-react-release` (aggregator, aussuchen, bearbeiten, energy, fem3d, generation3d, generator, koordinator, verfolgen) and on the four `:wasm` targets. `PreparationScript` then checks every release component marker (`.semio-artifact.json` with `🌉️bridge.js` and `🔣️.json`) and every staged session module. Failure text: `Incomplete Demonstrator runtime component` or `Mismatched Demonstrator session`.
   b. `@semio-tech/framework-surface-rs:wasm`, `@semio-tech/framework-editor-rs:wasm`, `@semio-tech/framework-os-node-graph-rs:wasm`, `@semio-tech/puzzle-plugin:wasm`. Each runs `repo ./📜️script.ts wasm`, mode taken from the invocation (no per-target env), output `.🧾️store/wasm.json`.
   c. `semio-framework-os-flow-browser-package:wasm` (`repo "../../🏗️build/📜️script.ts" wasm`).
   d. `semio-framework-canvas:stage-runtime-assets` (`repo ./📜️script.ts stage-runtime-assets`, output `.🧾️store/stage-runtime-assets.json` in the canvas root).
3. `build` itself (`repo ./🔨️modules/📦️site/📜️script.ts build`, `cache: false`, output `{projectRoot}/.🧾️store/build.json`), `runBuildScript` in `🔨️modules/📦️site/🟦️.ts`:
   a. `prefetchDemonstratorMapTiles(repoRoot, signal)` (section 4).
   b. `buildViteArtifact` with `mode: release`, `renderer: react`, `command: build`, `engines: play`, env `GIS_MAP_TILE_SERVE_MODE=bundle`, config `🏗️builder/🌐️vite/🟦️.ts`, owner `demonstrator:site`.
   c. Vite build: no activation receipt plugin (those are added only when `command === serve`, `🏗️builder/🌐️vite/🟦️.ts`). Plugin modules come from `builtPluginModulesRootIn(repoRoot, release)`. `browserArtifactVitePlugin(demonstratorRuntimeAssetSources(repoRoot, release))` replaces the static-dir plugins. Tile proxy plugin runs in `bundle` mode, so its `closeBundle` copies the whole tile cache into `dist` (section 4).
   d. Receipt `.🧾️store/build.json`: declared output. The writer was not traced (repo store-receipt owner). UNVERIFIED whether it is written on failure.

Command for the first real run (coordinator only, through the slot wrapper; no `SEMIO_BUILD_MODE` in the environment):

```
cd /Users/ueli/Documents/semio && env -u SEMIO_BUILD_MODE SEMIO_LANGUAGE=en "$C/1105b-tools/slot.sh" W-build-160 repo run @semio-tech/mit-bestand-demonstrator:build
```

`$C` is the ticket folder. Run it detached with nohup and an rc marker, as in `1107-runbook.md` step 10d.

Pre-checks before that run: `df -h /` (106 GiB free on 2026-11-07); `.cache/b/t/m/binaryen/130` present (yes); `.🧬semio/🗺️map/osm-tiles` and `openfreemap-vt` present (yes, 14,411 files, section 4).

Post-checks: `test -f .🧾️store/build.json`; `du -sh dist` and `find dist -name '*.wasm' -exec ls -l {} +`; compare `dist/…/🗺️` tile bytes with the plan (section 4); grep the log for `Prepared Demonstrator release` and `prefetch done`.

## 3. Target names (validity after today's merge)

| Name used by `build` chain | Where defined | Status |
|---|---|---|
| `@semio-tech/framework-os-dev:prepare-<9 lanes>-react-release` | inferred: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs:234-240` from the catalog `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎠️playgrounds.json` (156 rows, all 9 lanes present) | valid, per `1107-audit-merge.md` section 5; not re-run |
| `@semio-tech/framework-os-dev:activate-<lane>-react-prototype` (energy, fem3d, generator) | inferred, same file lines 226-232 | valid (prototype chain only) |
| `@semio-tech/framework-surface-rs:wasm` | `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📋️project.json:55` | valid, no `defaultConfiguration`, mode from invocation |
| `@semio-tech/framework-editor-rs:wasm` | `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/📋️project.json:44` | valid |
| `@semio-tech/framework-os-node-graph-rs:wasm` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust/📋️project.json:36` | valid |
| `@semio-tech/puzzle-plugin:wasm` | `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/📋️project.json:23` | valid (only one project with this name outside `.🧬semio` overlays) |
| `semio-framework-os-flow-browser-package:wasm` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust/📋️project.json:242` | valid; the project has no `Cargo.toml`, its script is `🏗️build/📜️script.ts` (exists) |
| `semio-framework-canvas:stage-runtime-assets` | `🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust/📋️project.json:39` | valid |
| `@semio-tech/mit-bestand-demonstrator:prepare-release`, `activate-lanes-prototype` | demonstrator `📋️project.json` | valid |
| `workspace:deps-browsers`, `@semio-tech/s-dev-composition:generate` | root `📋️project.json` (name `workspace`, line 2768/4779), `✏️s/🧑‍💻dev/📋️project.json:225` | valid (not in the `build` chain; used by `prepare-e2e` and `test-runtime-catalog`) |

Uncertainty to close: the framework-level registry `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/*.json` is empty on disk (3-byte `[]` files and `📣️publication.json` with `contributions: []`, all dated 11-04 19:38). The inference reads the catalog contributions it is given (`collectPlaygroundCatalog` returns `[]` when there are none). The audits name the `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/` catalog as the source, and that one is populated. UNVERIFIED which publication the graph reads. Check (read-only): `/usr/bin/python3 -c` count of `contributions` in both `📣️publication.json` files.

## 4. Map tile prefetch and tile bundle

Source of tiles: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/🗺️tiles/🟦️.ts` (re-exports `🗺️tiles/🟦️.ts` in the gismap artifact).

- Raster: `https://tile.openstreetmap.org/{z}/{x}/{y}.png`. Vector: TileJSON `https://tiles.openfreemap.org/planet`, then the template from it. No API key, token or account. Network only on cache misses.
- User-Agent `ComposeGisMapPlay/0.1 (+https://github.com/usalu/semio; dev playground)` is sent on every request.
- Plan (`🔨️modules/📦️site/🗺️map-tiles/🟦️.ts`, zoom cap `🗺️tile-serve-mode/🟦️.ts`): bounds Switzerland west 5.95 / south 45.82 / east 10.52 / north 47.81, raster and vector z0-10. That is 198 raster + 198 vector = 396 tiles. Checked against disk with the same tile math: 0 missing. On this machine the prefetch is a no-op.
- Cache roots: `.🧬semio/🗺️map/osm-tiles` and `.🧬semio/🗺️map/openfreemap-vt` (also `terrarium-dem`, not used by the build). Current contents (apparent size, my count): osm 11,084 files, 276.8 MB, z0-14; vt 3,327 files, 330.1 MB, z0-14. The z0-10 subset is 802 osm files (26.7 MB) and 1,009 vt files (178.7 MB), larger than the 396-tile plan because older dev prefetches left other tiles.
- Concurrency 4, delay 120 ms, per-request timeout from `TilePlanLimits`. Any single failed tile throws (`map tile prefetch failed for N tile(s)`); there is no retry or backoff. A 429 or 403 from the OSM server during a cold run fails the whole build. OSM's tile-usage policy discourages bulk download; the rate here is about 8 requests/s at full concurrency (UNVERIFIED against the current policy).
- Writes are not atomic (`writeFile` directly into the cache). The skip test is `existsSync`, so a truncated tile from an interrupted run is never refetched. The canary (`3/4/2.pbf`, `0/0/0.png`) checks presence only.
- Bundle (`🛰️tile-proxy/🟦️.ts:170-186`): in `bundle` mode `closeBundle` runs `cpSync(cacheRoot, dist/<route>, recursive)`. It copies the whole cache root, not the 396-tile plan. On this machine that is about 607 MB of tiles, of which about 402 MB is z11-14 left over from older runs. A fresh checkout would ship about 205 MB (the z0-10 subset). The same build gives different output on two machines. Ship-size and reproducibility risk; the comment in `🗺️tile-serve-mode/🟦️.ts` ("Switzerland through z10 ... not full offline play z13/z14") describes an intent the code does not implement.

## 5. Profile differences (prototype vs release) and release-only risks

Profile policy: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts` and its schema `🧬️schema/🔣️.json` (`PolicyProtocol.settings`).

| Setting | prototype-wasm | release-wasm | prototype-native | release-native | release-descriptor |
|---|---|---|---|---|---|
| opt-level | 1 | s | 0 | 3 | 3 |
| codegen-units | 256 | 16 | 256 | 1 | 16 |
| lto | off | thin | off | thin | false |
| debug | 0 | 0 | 1 | 0 | 0 |
| debug-assertions | on | off | on | off | off |
| overflow-checks | on | off | on | off | off |
| strip | none | symbols | none | debuginfo | debuginfo |
| incremental | on | off | on | off | off |

Host (build-script) settings: prototype opt 0, release opt 0, codegen 256. Same in both.

Release-only risks, ranked:

1. Package opt-level overrides exist only in prototype. In `planCargoProfilePolicyV1` the `overrides` map (52 wasm package rows, opt-level 2, for example `semio-os-flow*`, plus the 28 native rows at opt 3) is filled only when `request.mode === "prototype"`, and the browser-canvas hot crates are added inside that same block. Release gets none of them; those crates compile at opt-level s. Any budget, deadline or timing test that passes in prototype can fail only in release. The repo rule says "maximum performance"; this asymmetry is the first thing to check. Intent is not documented; UNVERIFIED whether deliberate.
2. wasm-opt runs only in release. `🕸️wasm-build/🟦️.ts:130` passes `--no-opt` in prototype only, so wasm-pack runs binaryen on every release package. The pinned binaryen 130 is present (`.cache/b/t/m/binaryen/130/darwin-arm64-…`). On Windows and devcontainer the pinned tool is not verified. `wasm-pack` on this machine resolves to `/Users/ueli/.cargo/bin/wasm-pack` (outside the repo) when no workspace bin exists: a host dependency, not zero-touch.
3. Debug-only code. `debug_assert*` appears on 220 lines and `cfg(debug_assertions)` in 26 `.rs` files under framework, plugins and hub compositions. Release compiles these out, and overflow checks are off in release (silent wrap instead of panic). Any test that expects a debug panic passes only in prototype.
4. Feature leakage, closure check. The four engine crates (`surface`, `editor`, `node-graph`, `puzzle`) have no test features in their normal `[dependencies]`; only `serde`/`web-sys`. The demonstrator playground artifact's normal deps enable no test features; its dev-deps enable `artifact-app-testing`, `protocol-laws`, `mutation-testing` on `semio-plugin` and `semio-os-kernel`. The wasm build uses `cargo build --lib` (`🕸️wasm-build/🟦️.ts:83`), so dev-dep features are not activated under resolver 2 (workspace `resolver = "2"`, root `Cargo.toml:4`). A heuristic scan of 4,436 manifests found about 700 lines that enable test features in normal dependency sections, almost all in test or companion crates. UNVERIFIED for the real closure: `cargo tree -e features` on the wasm targets, run through `repo cargo`. Not run.
5. Test feature gates in source: `feature = "test-support"` (about 7,340 uses), `testkit` (1,882), `sut` (1,369), `testing` (647). They compile out in release unless a normal dep enables them (see 4).
6. wasm function ceiling. The 1M-function ceiling in memory `project-stdio-wasm-component-unlinkable` applies to stdio components. Not measured for browser wasm. Release LTO and opt-level s can change function counts. UNVERIFIED; measure with `wasm-tools print | grep -c '(func '` on the release output after the first run.
7. Stack and memory ceilings are the same in both modes (`.cargo/config.toml:27-46`: wasm32-wasip2 `--max-memory=536870912`, `-zstack-size=8388608`; wasm32-unknown-unknown `-zstack-size=16777216`). Not a profile difference; listed because the policy calls them "preserved".
8. Descriptors. Release descriptor role uses `release-descriptor` (opt 3, lto false). `PreparationScript` requires `.semio-artifact.json` markers in `builtPluginModulesRootIn(repoRoot, release)`. Memory `project-component-release-does-not-materialize` (2026-09-09, old, verify) says `component-release` alone writes only `<crate>/dist/component-release/*.wasm`; the served release module directory is written only by `support release` then `materialize release --manifest …`. If the inferred `prepare-<lane>-react-release` does not run those steps, `prepare-release` fails with `Incomplete Demonstrator runtime component`. UNVERIFIED on this tree.
9. The release vite build has no activation gate (no `semioActivationVitePlugin` or union receipt plugin when `command === build`). So `build` does not check the 28-component closure that `serve` checks. A release site can be produced from an incomplete closure. Not a failure, a gap.
10. `wasm` tasks have no `defaultConfiguration` and no fixed mode; their cache keys include `SEMIO_BUILD_MODE` (`🎚️mode/🟨️.mjs:19`), but their output file `.🧾️store/wasm.json` has one path per project. Prototype and release outputs overwrite the same receipt. Restore from cache can hand a release run a prototype receipt. UNVERIFIED.

Descriptors per profile (summary): prototype descriptors and plugin modules under `dist/prototype/…`; release under `dist/release/…` (`builtPluginModulesRootIn`); sessions per profile via `playgroundSessionsRootV1(repoRoot, profile)`; the release browser artifact uses `demonstratorRuntimeAssetSources(repoRoot, release)` (includes `infinite:fonts`). None of these were listed on disk: no `dist` under the demonstrator, no `.cache/b/p/m/browser/release`.

## 6. Serve gap (outside the build DoD, noted for the release browser phase)

`serve` and `serve-e2e` are prototype only. `ServeScript` reads the prototype activation receipt; there is no `serve-release` target. `release-serving-resumed-2026-10-21.md` lists this with a proposed `serve-release` (dependsOn `build`, mode fixed release). Not needed for the build DoD; needed for any release browser proof.

## 7. Evidence of earlier release runs

- No release build log exists. Search over `🗑️generated` (names with build, release, prep, store, tile, site, demo, mit): zero hits for `mit-bestand-demonstrator:build`, `prefetch done`, `Prepared Demonstrator release`, `build.json`. `1107-runbook.md:305-311` marks step 10d as never run.
- No `.cache/b/p/m/browser/release` directory. Only `prototype` (298 MB).
- The demonstrator test logs (`demo-d104`, `demo-d106`, 04:01 and 06:57 today) show `repo run …:test` works; they do not exercise `prepare-*` or `build`.
- 10-21 docs (`release-proof-current-handoff-2026-10-21.md`, `release-serving-resumed-2026-10-21.md`, `release-serve-plan-2026-10-21.md`) are planning and read-only audits. They contain no timings.

Estimates (none measured):
- Disk: prototype browser cache is 298 MB; release adds a similar tree; the bundle copies up to about 607 MB of tiles into `dist` on this machine (about 205 MB on a fresh checkout); plus wasm release target dirs (unknown). Free space on `/` is 106 GiB, enough for one run.
- Time: activation cold run is about 41 min and cached about 11 min (memory `project-demonstrator-build-and-stale-artifacts`, 10-18). Release adds the release prepare for nine lanes, four wasm packages at LTO thin plus wasm-opt, flow browser wasm, canvas assets, and tile prefetch. Expect longer than the prototype cold run. UNVERIFIED; the runbook's `[inferred]` "second full component phase" is the same unknown.

## 8. Unknowns to close (ordered by cost of being wrong)

1. Which catalog publication the graph reads (section 3). Read-only check first.
2. Whether `prepare-<lane>-react-release` materializes release modules (risk 8). A `prepare-release` log will show `Materializing <plugin> release: transpile`.
3. Whether the package opt-level asymmetry (risk 1) is intended. Ask the owner of `🕸️wasm-build` / cargo profile policy.
4. Tile bundle contents on the first run: `du -sh dist/<tile route>`; compare with 205 MB (plan z0-10) and about 607 MB (whole cache).
5. Whether the OSM server rate-limits the cold prefetch. Cold runs happen only on a fresh checkout or after deleting `.🧬semio/🗺️map`.
6. Wasm function counts and sizes for release output.
7. Whether `build.json` is written on failure.

## 9. Recommended actions (not applied; this audit edits nothing)

1. Before the first release run: decide the tile bundle policy. Either copy only the plan's tiles (use `listMapTilesForBounds` over `GIS_MAP_DEFAULT_PREFETCH_BOUNDS` with the z0-10 caps) or make the bundle a manifest-driven list. Also make tile writes atomic (write to a temp name, then rename) and add a bounded retry with backoff in `prefetchMapTiles`. These are owner changes in `✏️s/🔌️plugins/🌍️gis` and the tile-proxy builder.
2. Decide whether release should get the package opt-level overrides. If yes, apply them in both modes (the policy function); if no, document it in the profile constraints.
3. Run the build once in a fresh checkout state (empty `.🧬semio/🗺️map`) only after an explicit go-ahead, because it needs OSM and OpenFreeMap access.
4. Keep `SEMIO_BUILD_MODE` unset for the run; set it only through the target.
