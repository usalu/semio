# Audit: wasm guard, wasm pipeline, and what is still computed twice (2026-11-14, read-only)

Read-only audit of `/Users/Shared/semio`. No build, no `repo` command, no git write, no sub-agent. Paths: `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, `UC` = `LIB/⚡️caching/🦀️cargo/🗃️unit-cache/`, `PLUG` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`, `T` = this ticket folder. "Verified" means read in code or in a result file's verdict line; "not verified" is stated as such.

## 0. Short answer

- The wasm guard is a name match on the Nx target name `wasm` (and two other names). It is not a rule about wasm units. Rust `wasm32` units are NOT guarded: they go through the same fleet tier as every other unit (B's live plugin proof, 1017 units, 879 uploaded).
- The guard does not cover the `component` target that builds a plugin's wasm payload. That target is `cache: true`, writes a store receipt, and is therefore shareable through Nx and the fleet content store today, with no two-checkout proof. This is the most important finding of this audit (section 1.3, 4).
- No two-checkout proof for a wasm target exists. The only related tests use synthetic Nx tasks or Rust crates (`pets`, `procedural-generation2d` through one live run).
- Of the wasm pipeline's seven steps, only the Rust compile and the Nx receipt can be reused across machines. The jco transpile, wasm-opt, and probe steps use a local stage cache and are recomputed on every machine.
- Of the 13 items in `audit-computed-twice.md`, the Nx single-flight, TeX/tectonic downloads, pinned toolchain downloads and the Rust unit cache are fixed in code. The Nx project graph, discovery walks, dependency install store, generators, Go build cache, CMake and Playwright are still computed twice (table in section 5).

## 1. The guard (question 1)

### 1.1 Where it is

| What | File:line | Condition |
|---|---|---|
| Authored rows | `LIB/🏗️build/🧮️task-model/🛡️remote-guard/🧬️schema/🔣️.json:19-21` | targets `["wasm"]` |
| | same, `:28-30` | targets `["preview-generated"]` |
| | same, `:37-43` | targets `["test-api-contract","check-types","test-types-source","test-crate-boundaries","test-browser-ownership"]` (read a gitignored generated file by path) |
| Lookup | `LIB/🏗️build/🧮️task-model/🛡️remote-guard/🟦️.ts:9-12` | `remoteGuardV1(name)`: strips leading non-alphanumerics from the target name, then exact match against the rows. No file-type, executor, crate-type or output test. |
| Nx input | same, `:15-16` | a guarded target gets the runtime input `hostname` (`REMOTE_GUARD_POLICY_V1.machineRuntime`, schema line 16), so its Nx hash is unique to the machine. |
| Projection | `LIB/🟨️.mjs:97` | `remoteGuardInputsV1(name)` is added to the inputs of every projected target. The name comes from `LIB/🟨️.mjs:105` (`projectBuildTargetForNxV1(target,name)`), so the guard is applied for every project. |
| Upload drop | `LIB/🌐️remote-cache/🧶️nx/🏁️task-finish/🟦️.ts:14` | `guarded = remoteGuardV1(targetName) !== undefined`, the single-flight front drops the PUT. |
| Law | `LIB/🏗️build/🧮️task-model/⚖️cache-law/🟦️.ts:57` | counts `guarded`. |

Note on the file path: the guard module lives under `LIB/🏗️build/🧮️task-model/🛡️remote-guard/`, not under `LIB/🧮️…`. The `wasm` row also matches any project target that is literally named `wasm`, whatever it builds.

### 1.2 Other exclusions in the Rust unit cache (the wasm-relevant ones are marked)

Bypass = the command line is compiled and never looked up or published. Fleet = the unit is local-only or not uploaded.

| # | Rule | File:line | Reason given in code or reports |
|---|---|---|---|
| a | source not `.rs`, no crate name, no out dir | `UC/🔏️key/🟦️.ts:54-56` | not a crate compile |
| b | `--print`, `-vV`, `--version`, `-V`, `--explain` | `UC/🔏️key/🟦️.ts:57` | queries, bypass `query` |
| c | no `--json=artifact` messages | `UC/🔏️key/🟦️.ts:58` | the wrapper needs artifact messages to find outputs |
| d | no `link` or `metadata` emit | `UC/🔏️key/🟦️.ts:59` | nothing to cache |
| e | crate type outside `cacheableCrateTypes` (`lib, rlib, proc-macro, bin, cdylib, dylib, staticlib`) | `UC/🔏️key/🟦️.ts:60-61`, schema `cacheableCrateTypes` | list in `UC/🧬️schema/🔣️.json:60` |
| f | linked type (`bin, cdylib, dylib, staticlib`) with `-l` or native search path | `UC/🔏️key/🟦️.ts:62` | native link inputs are outside the key. **Wasm cdylibs without `-l` pass this rule.** |
| g | unreadable source or extern | `UC/📜️script.ts:112-113,116` | bypass `unreadable-input` |
| h | `-C incremental` | not excluded. `UC/🔏️key/🟦️.ts:46,90` drops the path from the key. | T: incremental rlibs are not byte-reproducible even against themselves; shared under first-upload-wins (`T` section "What remains non-deterministic"). |
| i | focused `-p` crate | removed by B | a unit served and published like all others (`B` follow-up) |
| j | `-C` flags | none excluded. All `-C`, `-Z`, `--cfg` values stay in the normalised argv (`UC/🔏️key/🟦️.ts:84-98`), except the lane and `incremental=` parts. Wasm link args `-C link-arg=--max-memory=…` and `-zstack-size=…` (`.cargo/config.toml:45-47`, and `:27-30` for `wasm32-unknown-unknown`) therefore are in the key. | the key covers them |
| k | env names in the key | `UC/🧬️schema/🔣️.json:49-58`: `CARGO_CRATE_NAME, CARGO_BIN_NAME, CARGO_MANIFEST_DIR, CARGO_PRIMARY_PACKAGE, CARGO_ENCODED_RUSTFLAGS, RUSTC_BOOTSTRAP, SDKROOT, MACOSX_DEPLOYMENT_TARGET, IPHONEOS_DEPLOYMENT_TARGET`, plus prefix `CARGO_PKG_` | included in the key, not an exclusion |
| l | path-sensitive unit (its bytes hold a real path of this machine not announced by the source scan) | `UC/📜️script.ts:170-172` (`leakedPathsV1`, `:78-87`) | `remote-skip path-dependent`, not shared. **Roots checked are only cwd, manifest dir, workspace root, lane root** (`:170`). CARGO_HOME, rustup home and the registry path are not in that list. Residual risk for wasm payloads, not verified. |
| m | fleet tier skipped: breaker `down` or `denied` | `UC/🛰️fleet/🟦️.ts:34-36`, `UC/📜️script.ts:143-144` | B: breaker is per cache root, not per endpoint (known weakness) |
| n | fleet prepare fails (no settings, not usable, or any exception) | `UC/🛰️fleet/🟦️.ts:39-47` (`return undefined`) | local only |
| o | output size: local store refuses above 1 GiB | `UC/💾️store/🟦️.ts:112` (`maximumOutputBytes` 1073741824) | local only |
| p | fleet manifest: zero or more than 64 outputs, blob or entry over 256 MiB | `UC/🛰️fleet/📦️wire/🟦️.ts:102` (`maximumBlobs` 64), `UC/🛰️fleet/📤️uploader/🟦️.ts:75` (256 MiB), schema `UC/🛰️fleet/🧬️schema/🔣️.json:29` | not uploaded. Wasm cdylibs with many outputs would hit the 64 limit (not measured). |
| q | fleet key platform: OS, arch, host triple, rustc `-vV` digest, launcher toolchain, Darwin major for linked kinds, CPU for `target-cpu=native` | `UC/🛰️fleet/🪪️identity/🟦️.ts:49`, key at `:109` | the platform part is in the key; no glibc (Linux) yet (K) |
| r | build-script RUN | `UC/🏗️buildscript/🟦️.ts:62,78`; schema `buildScript.platforms` darwin, linux (`UC/🧬️schema/🔣️.json:81-85`) | "RUN output depends on host state the key cannot see" (B). Not shared: 23 `script-miss` in the live plugin build. |
| s | offload | `UC/📜️script.ts:129` (`OFFLOAD_POLICY_V1.crateTypes`) | offload only, separate from sharing |

### 1.3 The wasm guard is narrower than the audit assumed

- Only Nx targets named `wasm` are guarded. Rust `wasm32-wasip2` and `wasm32-unknown-unknown` units are shared under the normal rules (rows a to q); B's live proof used `wasm32-wasip2` for `semio-s-artifact-procedural-generation2d`.
- The plugin payload is not built by a target named `wasm`. It is built by the inferred `component` target: `LIB/🏗️build/🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs:79-97` (`componentBuildTargets`). That target is `cache: true` (line 89), `withCacheDecision(… "deterministic", "pure-function-of-inputs")` (line 96), and its only output is `storeReceiptOutputV1("component")`, which is `{projectRoot}/.🧾️store/component.json` (`LIB/🏗️build/🧾️store-receipt/🟨️.mjs:11-13`). That output is admitted by `LIB/🟨️.mjs:91-92` (receipt rule, added by G landing 6). The name `component` is not in the guard rows, so **the component target is not guarded**.
- Consequence (by code, not run): once a component's receipt is in the Nx cache, the payload is restored through `fleet-content` (G: `restore` before the re-execute repair). G's two-checkout proof was never run for a component. This is a gap between G's stated plan ("keep the wasm guard until the two-checkout proof") and the code.
- I could not locate where a target named `wasm` is generated. No authored `📋️project.json` under `✏️s/🔌️plugins` has a `"wasm": {` target (checked with find and grep). G's census counts 11 `wasm` targets in the authored set. Not verified where they come from.

## 2. The two-checkout proof (question 2)

What it must show, for one wasm plugin target (the test shape named in G is `🧶️nx/🚦️single-flight/🧪️tests/🧰️nx-e2e`):

1. Same unit keys and Nx hash at two checkout paths (dump the keys; compare).
2. Byte-equal outputs: `.wasm` (after the transpile stage's debug strip), jco module files, `🧾️component.json`, the store receipt, with sha256 per file.
3. No absolute path of either checkout, the registry or the rustup home in any shared file (grep over the outputs, and over the fleet store).
4. Second checkout run: zero rustc spawns for the crate and its dependencies (counted by a shim), zero stage recomputes for transpile, optimize and probe, and an Nx hit or a fleet-content restore.
5. `remote-skip path-dependent` count equal to zero.

What exists today:

- `T/🧰️scripts/B/make-checkout.ts`: copies the pets crate and its path dependencies to a scratch root. Rust only, not a plugin target.
- `T/🧰️scripts/T/mirror.sh`, `T/🧰️scripts/T/runtest.sh`: scratch mirror and clean test runner for the touched members (Rust).
- `T/🧰️scripts/G/census.ts`: the task-set census. It does not build.
- `UC/🛰️fleet/🧪️tests`: "two machines at different paths" (Rust, `pets`-like crate, 12 pass, per B). `UC/🛰️fleet/🪪️identity/🧪️tests` has `wasm32` fixtures for keys only (`🧪️tests/🧫️fixtures/🔣️.json`), not an output proof.
- `LIB/🧶️nx/🚦️single-flight/🧪️tests/🧰️nx-e2e`: real Nx 23.2.0, two checkouts, synthetic tasks (G).
- No script in `T/🧰️scripts/` names wasm, component, jco or two-checkout as a proof step (grep over the scripts directory and the result files: only G and T mention the proof, as not done).

What is missing to run it:

1. A completed `repo graph`. The component target is only projected from the graph. G, M, S and U all report that no graph run finished for the whole tree (U verdict (a): not finished after 1 210 s; (c): 4 775 projects cold).
2. `node_modules` with `jco`, the preview2 shim packages and the pinned binaryen in both checkouts. The install path is not proven end to end (H section 1: "NOT achieved (blocked)"; N: `mirror`/`install pack` seeded and restored for real, but the full install was not run here).
3. The `wasm32-wasip2` target in the pinned toolchain (not checked on this machine).
4. A scratch fleet cache server with the signed sidecar (L). B's live proof used a scratch server on 127.0.0.1:6391 and the 6094 sidecar with the real credential.
5. Disk headroom. B's second follow-up: the host CAS guard refuses writes above 97 % used, and the disk was at 98 % at the time. A publish under that condition gives `publish-skip` and the unit is not uploaded, so the second machine recomputes it.
6. A runner that: copies the checkout to a second path with the same `.cargo/config.toml`, runs `repo run <component target>` twice with the fleet variables set, and writes the comparison report into the ticket's `🗑️generated` folder.
7. The profile that the audit and the memory note call `wasm-dev` (`CARGO_PROFILE_WA…`). I did not find a `[profile.wasm-dev]` table in the root `Cargo.toml` (it has none), in `.cargo/config.toml`, or in the plugin manifests I searched (`✏️s/🔌️plugins/🌀️procedural/…/Cargo.toml`), and no `CARGO_PROFILE_` setting in `LIB`. The profile's debug settings are therefore not verified.

## 3. Wasm inputs that could make outputs path- or machine-dependent (question 3)

| Input | Where | Covered by a key or a check? | Risk |
|---|---|---|---|
| Linker args `-C link-arg=--max-memory`, `-zstack-size` | `.cargo/config.toml:45-47` (wasm32-wasip2), `:27-30` (wasm32-unknown-unknown) | yes: argv in the unit key (`UC/🔏️key/🟦️.ts:84-98`) | low |
| `-Z threads=0` | `.cargo/config.toml:45-47` | yes (argv) | low |
| `CARGO_ENCODED_RUSTFLAGS` | key env names `UC/🧬️schema/🔣️.json:49-58` | yes | low |
| Remap of source names | `UC/📜️script.ts:76,92`: `--remap-path-scope=all` replaces Cargo's scope; `UC/🧭️remap` (T) maps machine roots by role | yes for rustc metadata and object code of workspace and registry units (T, `strings` check) | medium: the wasm payload and DWARF of the final link are not checked for absolute paths. Only the outputs of fleet-eligible units are scanned (`:170`), with four roots. |
| Leak scan roots | `UC/📜️script.ts:170` | partial | CARGO_HOME, rustup home and `/private` variants not in the scan list. Not verified for wasm. |
| `wasm-ld` / `wasm-component-ld` (the component linker) | the toolchain identity is the `rustc -vV` text and the launcher's toolchain digest (`UC/🛰️fleet/🪪️identity/🟦️.ts:24-33`) | no: the linker binary bytes are not hashed. Rust ships `rust-lld` inside the toolchain (the `gcc-ld` directory, `.cargo/config.toml:54-57` comment). A separately installed `wasm-component-ld` on PATH would not be in the key. | medium, not verified which linker the wasm32-wasip2 target uses here |
| `wasm-opt -Oz --strip-debug …` | `PLUG/🌐️browser-bundle/🏗️materialization/🪶️wasm-opt/🟦️.ts:12-21`, command at `:44-46` | the stage key uses `--version` text, not the binary digest (`PLUG/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:121`). The binary itself is a pinned download (H: binaryen, sha256). | low |
| `BINARYEN_CORES` | `…/🪶️wasm-opt/🟦️.ts:57` | depends on the job slots granted; not in the key | low for bytes (wasm-opt output is deterministic per input in Binaryen, not verified here) |
| jco version and transpile output | key = digest of the debug-stripped wasm, jco version from `package.json`, materialization sources (`PLUG/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:98-110`) | yes; the transpile output is written location-independent by design (comment at `:104`) | low |
| debug info | `stripWasmDebugSectionsV1` is used for the transpile key only; the optimize stage key uses the core digest (not stripped) | the optimize key sees debug info; the Rust unit key sees `-C debuginfo` | low |
| `wasm-dev` profile | not located (section 2.7) | - | unknown |
| Platform part for wasm cdylibs | `UC/🛰️fleet/🪪️identity/🟦️.ts:49`: linker = `darwin-<major>` for linked kinds (cdylib included) | yes | low (strictly more keyed than needed) |
| Absolute paths in the component receipt | `PLUG/🧩️driver-stages/🟦️.ts:42` (`deliverable` with `directory`, `wasm`, `receipt`) | not checked | medium: receipts store absolute `directory` and `wasm` paths (`dirname(component.staged)`); the fleet-content manifest rewrites roots only for the store (G: root-neutral manifest for Nx). Not verified for the component receipt. |

## 4. The plugin build pipeline, end to end (question 4)

Entry: `component` Nx target (`LIB/🏗️build/🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs:87-96`) runs `repo <driver script> component --manifest <Cargo.toml>`. Driver: `PLUG/🏗️build/🧩️driver-stages/📜️script.ts:12-21` (`ComponentScript`) calls `buildComponentClosureV1` (`PLUG/🏗️build/🧩️driver-stages/🟦️.ts:58-76`, `runRepositoryComponentDriverV1`). Stages per component (`PLUG/🏗️build/🧩️driver-stages/🟦️.ts:40-56`): describe, model, transpile, optimize, browser, complete, discard. Probe runs inside the materialization module.

| # | Step | Code | Cache that covers it | Second machine recomputes today? |
|---|---|---|---|---|
| 1 | Cargo wasm build (one pass per component, through the rustc wrapper) | driver `PLUG/🏗️build/🧩️driver-stages/🟦️.ts:67-76` → `runNativeBuildDriverV1`; rustc invocations go through `UC/📜️script.ts` | unit cache local + fleet tier (B). Build-script RUN: local only. Cargo fingerprints: per lane. | rustc units: no (fleet hits). Build-script RUN and fingerprints: yes. Live proof: 47 min cold for one plugin, 1017 units, 879 uploaded (B). |
| 2 | describe (descriptor pair) | `PLUG/🏗️build/🧩️driver-stages/🟦️.ts:46` → `describeComponentModelV1` (`PLUG/🏗️build/📦️model/🟦️.ts`, not read in full) | not traced. The committed descriptor pair is the output of record. | not verified |
| 3 | model publish (provenance-bound receipt) | `PLUG/🏗️build/🧩️driver-stages/🟦️.ts:47` → `publishComponentModelV1` | products store (local custody), receipt. Shared only through the Nx receipt, if step 8 hits. | yes unless step 8 hits |
| 4 | jco transpile | `PLUG/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:106-110` (`transpileComponentModulesV1`) → `work.cache.run` | StageCache, a local directory tree (`🗃️stage-cache/🟦️.ts:105-124`, root = `components` under the store, `…/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts`). No fleet tier. | **yes**, when step 8 does not hit |
| 5 | wasm-opt -Oz per core | `…/commands/🟦️.ts:118-125` (`optimizeComponentCoresV1`) → StageCache per core digest | StageCache, local | **yes** |
| 6 | browser publish | `…/commands/🟦️.ts` `publishComponentBrowserV1` | products store, receipt | yes unless step 8 hits |
| 7 | probe (node + JSPI, `describe()` of the module) | `…/commands/🟦️.ts:139-151` | StageCache, local | **yes** |
| 8 | Nx `component` target | `LIB/🏗️build/🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs:87-96`; upload via `LIB/🌐️remote-cache/🧶️nx/…` single-flight (G); restore via `fleet-content` (G) | Nx remote cache through the fleet front (single-flight claims) + receipt payload in the fleet content store | **shared, if the Nx hash matches** (platform-bound by `sharedGlobals`, G). Not guarded by name (section 1.3). |
| 9 | the `wasm`-named targets (11 per G's census) | guarded by name (`🛡️remote-guard`) | local cache only | yes (by design) |

Reading of the table: a second macOS arm64 machine, with an equal Nx hash, takes step 8 and skips 1 to 7. With a different Nx hash it does step 1 from the fleet (Rust units) and then steps 2 to 7 locally. Steps 4, 5 and 7 are the expensive non-Rust steps with no fleet tier. Their time was not measured in any result file.

## 5. Still computed twice: updated table (question 5)

Source list: `T/audit-computed-twice.md` section 1 (ranks 1 to 13), checked against verdict lines of `result-A` to `result-U` (verdict and "NOT verified" sections). Severity: very high, high, medium, low. "Minutes saved" uses the number in the report or memory note; where none exists it says so.

| Rank | Item | Status after A to U | Evidence (file, verdict) | Second machine recomputes today | Severity | Minutes saved if shared |
|---|---|---|---|---|---|---|
| 1 | Nx project graph (158 MB, 1271 projects) | partly fixed: fleet tier and root-neutral digest built (`LIB/🕸️graph/…` M/🧶️nx, measured stage hit 2.5 s); whole-graph fleet hit not achieved | F section 2 (stage measured), U verdict (c) "not obtainable" (cold 4 775 projects after reset), M "fleet hit 150 to 250 s, target 60 s missed", S "no run completed" | yes whenever no fleet generation exists (U (c)) | very high | 3.5 to 6 min per graph read (audit, memory note; F measured 3.5 to 6 min computed vs 2.5 s stage hit) |
| 2 | Discovery walks at dev boot and generated catalogs (`🤖️generated/🔌️plugins.json`, `🎠️playgrounds.json`) | not covered: no result mentions the catalogs; U1 removed the 230 s change-journal epoch, but three verification passes of 40 to 90 s remain per command (U section 1, item 6) | U section 1, item 6; `.gitignore:97` (audit) | yes | high | 80 to 210 s per walk, 3 to 5 walks per dev boot (memory note 2026-09-05, not re-measured here) |
| 3 | Wasm plugin payload and activation | partly fixed: receipts now shared through `fleet-content` (216 `build` receipt targets, 11 wasm targets cacheable but guarded); the `component` target is shared with no proof | G landings 6, census (shared 3226, guarded 48, uncacheable 779), this report 1.3 and 4 | step 8 yes if Nx hash equal; steps 4, 5, 7 yes otherwise | high | not measured; cold plugin Rust build 47 min (B), of which the rustc part is shared at 86 % |
| 4 | Nx task single-flight | done: R `🚦️single-flight`, registry `adapter`, 14 tests, real Nx two checkouts | G "Built", "Proof", "Second pass 5" | no (claims route on the server not verified live, G NOT verified 4) | low | 2 to 6 min per duplicate Nx task (the claim window) |
| 5 | Target hygiene (632 no flag, 657 `cache: false`, 2393 `cache: true` without outputs) | partly: 2981 authored tasks are check-without-outputs (stdout replay); 779 uncacheable with reasons (238 host resources, 177 side-effecting); law on inferred targets never run | G census (authored only) and "NOT verified 1" | yes for the 779 by design, the rest unknown | medium | not measured |
| 6 | TeX bundle and tectonic download | done for downloads: tectonic 20.6 MB and 500 TeX files 18.9 MB by sha256 through the fleet fetch tier; second machine 0 bytes from upstream (proof 5). **Correction to the audit: the 2.88 GB figure is the upstream tar; the repo fetches only these 500 ranges (H section 3).** Print compile itself (`deps-tectonic`, `deps-tex` `cache: false`) is unchanged. | H proof 5 and measured sizes | download: no. Print compile: yes (not measured) | low (download), medium (compile) | 2 to 4 min per machine for the 492 MB pinned set (H estimate) |
| 7 | Toolchains and package caches (rustup, uv, bun, go modules, cargo registry, playwright) | mostly covered: rustup manifest and toolchain archives pinned, crates, bun runtime, uv, npm and go pass-through mirror (H, N). Not covered: Playwright browsers (no sha256 pin, H left it), per-machine copies on disk | H "Covered" list and "NOT verified"; N mirror live | Playwright yes; others no | low | bandwidth and disk, not minutes (H: 492 MB per machine) |
| 8 | Dependency install store (`.cache/b/d/m/j/<lock-hash>`, absolute symlinks) | code landed (install hook in H, install pack in N, Windows junction in K), **end-to-end not achieved** (H section 1 "blocked") | H section 1; N; K section 3 item 1; F "36 + 34 min installs on this machine" | yes until the end-to-end run is proven | medium to high | 34 to 36 min cold install on one machine (F, measured here) |
| 9 | Go builds (`GOCACHE` per repo cache dir) | not covered: go module downloads through the mirror (H), `GOCACHE` still per machine (`PROC/🌿️environment/🟦️.ts:40`, audit) | H proof 4 (modules only) | yes | low to medium | not measured |
| 10 | Codegen: `generate` (13), `🏭️generator` (36), `🏭️generator-manifests` (32), `🏭️generator-encryption` (4) `cache: false`; `preview-generated` (29, `cache: true`, no outputs) | preview-generated kept guarded, and the audit's reason was wrong (it writes nothing; G). Generators unchanged. | G "Findings (2)", census | yes | medium (correctness first) | not measured |
| 11 | CMake C++ (`cpp-*` targets, `cache: false`) | not covered (only the cmake binary is pinned, H) | H "Covered" list | yes | low | not measured |
| 12 | Playwright browsers (568 MB) | not covered (H left, no pin) | H section 6 / "left" | yes | low | not measured |
| 13 | Bazel | not default | audit | n/a | negligible | - |
| new | Cargo build-script RUN, native-link bins, and units whose outputs carry a machine path | not shared by design | B "NOT shared", live `script-miss` 23; T "remote-skip path-dependent" | yes | low to medium | not measured |
| new | Units not published while the host CAS is above 97 % | the disk guard refuses the write, so the unit is not uploaded; "uploading from the compiler outputs" is the next step (B second follow-up, item 1) | B second follow-up, item 1 | yes, in that window | medium | not measured |

## 6. Not verified, and what to run next

- Whether the 11 `wasm` targets are produced by an inference step (section 1.3).
- Whether the component target's Nx hash is equal across two macOS arm64 machines (platform binding is by `sharedGlobals` per G; the component's own inputs were not diffed).
- Whether `wasm32-wasip2` uses rustc's bundled `rust-lld` or an external linker on this machine.
- Whether the component receipt contains absolute `directory`/`wasm` paths (section 3, last row).
- The `wasm-dev` profile.
- Steps 2 and 7 (describe, probe) cache behaviour beyond the stage cache.
- Cost numbers for steps 4, 5 and 7 (transpile, wasm-opt, probe): none exists in the result files.

Suggested order, after the guard decision: (1) decide whether the `component` target must join the guard until the two-checkout proof exists (its payload is the 123 MB case the guard was written for); (2) add `component` to the proof runner with a scratch cache server; (3) make the transpile, optimize and probe stages fleet-keyed through the same receipt path, or keep them inside the component receipt.

## 7. Files read

- `T/design.md`, `T/result-B.md`, `T/result-G.md`, `T/result-K.md`, `T/result-T.md` (full); `T/audit-computed-twice.md` (full); verdict and "not verified" sections of `T/result-F.md`, `T/result-H.md`, `T/result-I.md`, `T/result-M.md`, `T/result-S.md`, `T/result-U.md`, plus grep hits in `T/result-N.md`, `T/result-O.md`, `T/result-R.md`.
- Code: `LIB/🏗️build/🧮️task-model/🛡️remote-guard/` (module and schema), `LIB/🟨️.mjs:86-106`, `LIB/🌐️remote-cache/🧶️nx/🏁️task-finish/🟦️.ts`, `UC/🔏️key/🟦️.ts:1-64,84-117`, `UC/📜️script.ts:60-200`, `UC/🛰️fleet/🟦️.ts:1-60`, `UC/🛰️fleet/🪪️identity/🟦️.ts:1-49,109`, `UC/🧬️schema/🔣️.json`, `UC/🛰️fleet/🧬️schema/🔣️.json`, `UC/💾️store/🟦️.ts:112`, `LIB/🏗️build/🕸️graph/📥️inference/🧩️component-targets/🟨️.mjs`, `PLUG/🏗️build/🧩️driver-stages/📜️script.ts`, `PLUG/🏗️build/🧩️driver-stages/🟦️.ts:1-76`, `PLUG/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts:98-151`, `PLUG/🌐️browser-bundle/🏗️materialization/🪶️wasm-opt/🟦️.ts:1-60`, `…/🗃️stage-cache/🟦️.ts:1-124`, `/Users/Shared/semio/.cargo/config.toml`, `/Users/Shared/semio/Cargo.toml` (no `[profile]` tables).
- Not read in full: `PLUG/🏗️build/📦️model/`, the `🏁️task-finish` body beyond line 14, `LIB/🏗️build/🕸️graph/📥️inference/` other stages, the `R` remote-cache server.
- No command was run except read-only `ls`, `find`, `grep`, `cat`, `head`, `sed`. No `repo` command, no build, no git write.
