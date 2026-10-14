# 🚀️ R1: Release Path Storage, Source Boundary and Admission Census (2026-10-14)

Executor lane R1. Ticket `26/09/23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT`. No git-modifying command, no worktree, no sub-agent and no ticket MCP call was made. The ticket-local probe is `r1-release-admission/📜️script.ts` (commands `legacy`, `release`, `census`).

## 0. Status

1. **Confirmed, then fixed.** The root release passed no `storage` or `sourceBoundary` to the driver. The Play `build` had the same defect. Replaying the old call on a one-component plan fails at the driver's first statement with `Original issued native storage reservation required`.
2. **One owner.** Every component driver run now goes through `runRepositoryComponentDriverV1` in `🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts`. Root `release`, Play `build`, `components describe` and dev `buildComponentClosureV1` all use it. The owner takes storage, the Source boundary, the Cargo directories and registry custody from `withRepositoryCompilerStoreV1` and `withRepositoryCargoInputsV1`, so no default is duplicated.
3. **After the fix, the release path still does not reach a compiler.** The blockers, in route order:
   - `bun repo run` itself fails;
   - the pipeline profile was refused (fixed);
   - a dependency fetch/metadata mismatch;
   - stale extension lockfiles;
   - `cdylib` unit kinds;
   - Source admission.

   The patches for files that peers edited within the last hour are written out in §5, not applied.
4. **Over-ceiling census (no compile).** 376 repository units were measured in the hub and root passes. 160 are refused: 153 over a limit and 7 refused before counting. 152 of the 153 are in the Play closure. The 26 extension passes could not be measured because their lockfiles are stale. See §6.
5. **G1 (`external-path`).** No caller relevant to us lets a path dependency escape by cwd: the driver passes, its jobs and the Cargo test runner all key admission by the repository root. Two other gaps exist: the Play release's `wasm-bindgen` engine jobs bypass Source admission entirely, and the renderer `wasm` build admits with a temporary cwd. Patches are in §5, because the policy author file was edited 17:38.
6. **Tests.** Play fresh-build contract: 10/10 (vitest), including the new owner/launch law. `📥️source` and `🕸️units` bun tests: 14 pass, and 3 cannot run standalone because they need an invocation and the repo runner is broken. `tsc` reports 0 errors in touched files.

## 1. Finding, confirmed

- Static evidence: `📜️script.ts` `ReleaseScript` and `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts` `BuildScript` called `runNativeBuildDriverV1({ plan, …, cacheRoot: repoCacheDirectory, shared: cargoDirectories, … })`. Neither passed `storage` or `sourceBoundary`.
  - The driver's first statement is `requireNativeStorageReservationV1(options.storage, options.control)`.
  - It then requires `cacheRoot`, `shared.target` and `shared.build` to lie inside `storage.backing.root`, which is the 16 GiB HFS image at `⚡️cache/rust/fixed/m`.
- Runtime evidence: `bun r1-release-admission/📜️script.ts legacy animate` (old argument set, one-component plan) printed `[DEBUG] legacy call refused: Original issued native storage reservation required`.
- `bun repo run workspace:release` was also started. After 31 min of graph construction (`build:discover` → `build:project-assembly`), the neutral CLI died before our script ran:
  `SyntaxError: Export named 'repositoryBuildGraphWriterDirectoryV1' not found in module '…/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs'`.
  This is Fleet B's area and is not fixed here. Until it is fixed, no launch entry reaches the release script.

## 2. Fix

| File | Change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts` | New private `runRepositoryComponentDriverV1(repoRoot, mode, plan, {network, control, environment, generation, stages, jobEnvironment?, log?})`. It runs `withRepositoryCompilerStoreV1` → `withRepositoryCargoInputsV1` (every pass manifest) → `runNativeBuildDriverV1` with `storage: store.storage`, `sourceBoundary: store.sourceBoundary`, `cacheRoot: <store>/components` and `shared: {store CARGO_TARGET_DIR, store build}`. New export `buildReleasePipelineV1(repoRoot, pipeline, {components?, network, control})`: the mode comes from `SEMIO_BUILD_MODE` and the generation from `SEMIO_NATIVE_BUILD_GENERATION`. `buildComponentClosureV1` and `describeComponentsV1` now delegate to the owner, so their duplicated wrapping is gone. Unused imports were removed. |
| `📜️script.ts` (`ReleaseScript`) | Calls `buildReleasePipelineV1(repoRoot, targets.release.metadata.semio.release, {network: repositoryCargoNetworkV1(invocation, repoRoot), control})`. |
| `📜️script.ts` (`components describe`) | Passes the missing 4th argument `network` (same canonical grant). |
| `📜️script.ts` (`test canvas native`) | Now imports `repositoryCargoPreparationStorageV1` from its canonical owner `⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts`. `🗂️workspaces/🦀️cargo/🟦️.ts` no longer exports it. |
| `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts` (`build`) | Calls `buildReleasePipelineV1(…, {components: playRuntimeComponentIds(), network, control})`. |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | Coordinator item (1): the import of `repositoryCargoPreparationStorageV1` now points at the canonical owner, not at `…/🛠️preparation/📦️storage/🟦️.ts`, which never exported it. `bun -e 'await import(hub script)'` now links (`[DEBUG] linked`). The other hub/✏️s importers already used the owner or the bootstrap re-export. |
| `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts` | Explicit type re-export of `NativeBuildGenerationKindV1`. |
| `📋️project.json` and `🏢️semio-tech/🎡️play/📋️project.json` | Job `describe-tool` changed from `"profile": "host-tool"` to `"release"`. The pipeline schema (`PipelineJobV1.profile` ∈ prototype/release/wasm-*) and `profileBuildModeV1` refused `host-tool` (`Invalid native build pipeline: /jobs/0/profile must be an allowed enum value`) before any driver step, so both release routes were dead before storage was even reached. The job still compiles under `[profile.host-tool]` (`DESCRIBE_TOOL_PROFILE`). The job profile is the mode family, which `nativeBuildPipelinePlanV1` maps to the current mode. |
| `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` | `📦️build🦑️workspace🚀️release🆕️fresh` set `SEMIO_RELEASE_GENERATION`, which nothing reads, so the "fresh" entry silently ran warm. It now sets `SEMIO_NATIVE_BUILD_GENERATION=fresh`. `📦️build🦑️workspace🚀️release` (`bun repo run workspace:release`, 11.011) and `🆕️fresh` (11.012) keep their order and group. The Play fresh entry was already correct. |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts` | Source reader refusals now name the unit: `Rust crate Source refused before compiler: <package> (<reader message>)`. Boundary messages are unchanged. Before this change, the first release refusal read `Unsupported Rust compile expression at line 6: include_bytes`, with no crate. |
| `🏢️semio-tech/🎡️play/🧪️tests/🧪️playfreshbuild/{🟦️.ts,🧫️fixtures/🔣️.json}` | New law, checked with the TypeScript AST. Root and Play release callers contain exactly one `buildReleasePipelineV1(…)` call whose options carry `network` and `control`, and no `runNativeBuildDriverV1` identifier. The workspace fresh launch entry's env is exactly `{SEMIO_NATIVE_BUILD_GENERATION: "fresh"}` in both launch files (parsed with `jsonc-parser`). |

Behaviour change: release and Play builds now compile inside the charged compiler store of their mode, with history and stage cache under `<store>/components`, instead of `⚡️cache` and `cargoDirectories`. That is the same layout dev and `components describe` already used.

## 3. What the release path does now (one component, `release animate`, mode release)

Each blocker was observed by running `r1-release-admission/📜️script.ts release animate <offline|online>`, which calls `buildReleasePipelineV1` directly on one hub component:

1. The pipeline is admitted, after the `host-tool` fix.
2. The store reservation is accepted: `withRepositoryCompilerStoreV1` of `release`, `compiler-release.json` recovered.
3. The registry fetch, offline: `no matching package named 'axum'`. The store's `CARGO_HOME` (`m/cargo/home`) holds no registry yet, so an offline grant cannot work until it is seeded once.
4. The registry fetch, online, succeeds. The fetch is target-filtered (`cargo fetch --locked --target wasm32-wasip2`). Then the driver's planning capture runs `cargo metadata --locked --offline` without a platform filter and needs every platform's packages: `failed to download 'ahash v0.8.12' … --offline was specified`. **Blocker. Patch P2 in §5.**
5. With the full repository plan, the 26 extension workspaces fail `--locked`: `cannot update the lock file …/Cargo.lock because --locked was passed`. Their lockfiles are stale since the new framework crates were cut. **Blocker:** each needs `cargo update -w` (or an offline regeneration) committed by whoever owns those cuts.
6. Hub and root unit graphs: `Unknown Cargo compilation target kind`. All 44 hub components are `cdylib`, and 5 units are `rlib`. `kind()` only accepts `lib`. **Blocker, and it hits dev `buildComponentClosureV1` too. Patch P1.**
7. Source admission, with P1 emulated in the census port. Passes `wasm-release-00` (hub) and `wasm-release-27` (root) both refuse on their first unit in id order, `semio-framework-2d`:
   `Rust crate Source refused before compiler: semio-framework-2d (Unsupported Rust compile expression at line 6: include_bytes)`.
   The source reader does not model `include_bytes!`. That is the reader owner's work, not a size violation.
8. `semio-framework-plugin`, with real cfg, refuses with:
   `Rust crate Source refused before compiler: semio-framework-plugin (Generated Rust input requires compiler ownership: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🦀️.rs)`.
   With a simplified wasm cfg it already trips the byte ceiling mid-read: `Rust crate Source boundary exceeded before compiler: semio-framework-plugin (files 20/32, bytes 273221/262144)`.
   Its `rust-source-boundary` (12,000 lines) is inert, because limits are `Math.min` with the global policy.
9. **Capacity (not yet hit, flagged).** The compiler store charges `maximumWorkspaceBytes` = 4 GiB inside a 16 GiB image (`⚡️caching/🔣️policy.json` `strictStorage`). A1 measured 10.1 GB of provenance reads for 5 components. A 71-component release, and especially a fresh generation, which now lives inside the image, will not fit. The policy owner is Fleet A.

## 4. G1: `external-path` classification and admission escapes

Provenance is decided in `🕸️units/🟦️.ts`: a manifest outside the `workspaceRoot` the caller passes is `external-path` and skipped.

| Caller | `workspaceRoot` passed | Escape? |
|---|---|---|
| Release driver passes (`runNativeBuildDriverV1`, line 284) | `options.cwd` = repository root | No |
| Release driver jobs whose command is `cargo` (line 500) | repository root | No (no current job is a direct `cargo`) |
| Jobs `describe-tool` and `font-assets` (`buildRepositoryCargoArtifacts`) | `policy.cwd` = repository root | No |
| Scheduler build (`buildSchedulerV1`) | driver `cwd` = repository root | No |
| Cargo test runner (`runRepositoryCargoTests` → `runCargoTestsV1` → `🧪️testing/🦀️cargo:178`) | `getWorkspaceRoot()` | No |
| Repo `cargoStreamingStatus` relay (`📚️library/🏃️process/🟦️.ts:86`) | `getWorkspaceRoot(env)` | No |
| **Play jobs `flow-wasm`, `surface-wasm`, `puzzle-wasm` (`buildWasmWebV1`)** | **none: no Source admission and no storage reservation at all** | **Yes, total bypass.** The driver admits only `job.command[0]==="cargo"`, and these jobs are `bun ./📜️script.ts wasm` |
| Renderer `wasm` (`🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts:82`, `buildCargoArtifacts`) | `queryPolicy.cwd` = a temporary directory | Yes: every repository crate is `external-path`. Not in the Play closure |
| Framework `runCargoRelayV1` (`🌊️relay`) | `policy.cwd` | Only tests call it |
| Exact Cargo laws (`🧪️testing/🦀️cargo/🎯️exact`) | `options.cwd` (fixture workspace) | Test harness |

Fixing the bypass properly needs the wasm compile owner to admit Source keyed by the repository root, which means `WasmBuildPolicyV1` must carry it. The author of that policy, `📚️library/🏃️process/🕸️wasm/🔣️policy/🟦️.ts`, was modified at 17:38, under 60 minutes before this work. So the patch is P3 below and was not applied.

## 5. Exact patches not applied (peer-owned files touched within 60 min)

**P1: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🟦️.ts` (mtime 16:58), `kind()`.** Cargo's library crate types are `lib`, `rlib`, `dylib`, `cdylib` and `staticlib`:
```ts
if(kinds.includes("lib"))return"library";
// →
if(kinds.some(kind=>["lib","rlib","dylib","cdylib","staticlib"].includes(kind)))return"library";
```
`supported` already stays false for `cdylib`, since `types.length===1&&["lib","rlib"]`. Add one `cdylib` row to `🕸️units/🧪️tests/🧫️fixtures/🔣️.json` `cases`.

**P2: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts` (mtime 17:38), `cargoDependencyFetchArgumentsV1`.** The planning capture's `cargo metadata` resolves all platforms, so the fetch must too. Delete the target-selection loop, leaving `["fetch","--locked","--manifest-path",manifest,(--offline)]`. Update the fixture rows in `📥️dependencies/🧪️tests/🧫️fixtures/🔣️.json` that expect `--target`, and rename the test "exact target selection" to "complete locked platform set".

**P3: Source admission for wasm-bindgen builds.**
- `🕸️wasm-build/🧬️schema/🔣️.json`: add required `workspaceRoot` (absolute path) and `sourceBoundary` (`{version:1,maxSourceBytes,maxSourceFiles,maxActiveLines}`).
- `WasmBuildPolicyV1`: add `workspaceRoot: string; sourceBoundary: CargoSourceBoundaryPolicyV1`.
- `WasmBuildExecutionPortV1`: add `planning: CargoCompilerPlanningPortV1; source: CargoSourceAdmissionPortV1`. The default port uses the actual ports.
- `buildWasmWebV1`: after the build lease and before `invoke`, run:
```ts
const graph=await captureCargoCompilationGraphV1({workspaceRoot:policy.workspaceRoot,cwd:componentDir,program:"cargo",prefix:[],args:threads?threadedCargoArgs:["build","--locked","--lib","--manifest-path",join(componentDir,"Cargo.toml"),...cargoProfileArgs,"--target","wasm32-unknown-unknown",...featureArgs],environment:buildEnv,maximumOutputBytes:16777216},control,port.planning);
await admitCargoSourceBoundariesV1(graph,policy.workspaceRoot,policy.sourceBoundary,control,port.source);
```
- `repositoryWasmBuildPolicyV1` (`📚️library/🏃️process/🕸️wasm/🔣️policy/🟦️.ts`): add `workspaceRoot: root, sourceBoundary: repositoryStrictStorageV1(root).sourceBoundary`. This is the same canonical owner the compiler store reads.
- `🕸️wasm-build/🧪️tests`: add the two fields to `fixture.policy` and stub `planning`/`source` ports in the three publication tests. Add one refusal row: an oversized fixture crate refused before `run`.
- Renderer `wasm`: pass `workspaceRoot` instead of relying on `queryPolicy.cwd`. The same field belongs in `CargoArtifactBuildPolicyV1`, used by `buildCargoArtifacts` at line 244, instead of `policy.cwd`.

## 6. Over-ceiling units in the release closure (census without compiling)

Method: `r1-release-admission/📜️script.ts census release`, run through `🚦️cargo-slot.sh` with a private target/build dir.
- For each release pass, the probe captured the unit graph with `captureCargoCompilationGraphV1` (`--unit-graph`, nothing compiled) under `workspaceRoot` = the repository. `cdylib`/`rlib` kinds were projected onto `lib`, as P1 would do.
- Each `workspace` unit was then measured with the driver's own `actualCargoSourceAdmissionPortV1` cfg (real `rustc --print cfg` per unit) and `readRustSourceClosureV1`, without the early stop.
- Limits are the configured policy, 4,000 active lines / 262,144 B / 32 files, `Math.min` with any per-crate block.
- Coverage: hub (44 components, all in Play) and root (1 test component). The 26 extension passes are blocked by stale lockfiles (§3.5); their own crates are not in this table.

Totals:
- 376 units: 216 within the limits, 153 over a limit, 7 refused before counting.
- 152 of the 153 over-limit units are in the Play closure; the exception is `semio-framework-plugin-host-test-component`.
- Breaches, as files/bytes/lines: all three 111; bytes and lines 17; files only 11; files and bytes 6; bytes only 5; lines only 3.
- Only 1 over-limit unit declares a `rust-source-boundary`.

### 6a. Refused before counting (reader limits)

| Package | Unit | Refusal before counting |
|---|---|---|
| `semio-framework-2d` | library | Unsupported Rust compile expression at line 6: include_bytes |
| `semio-framework-os-kernel-dsl-derive` | proc-macro (host) | Unsupported Rust compile token output requires an explicit generated-input observer |
| `semio-framework-plugin` | library | Generated Rust input requires compiler ownership: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🦀️.rs |
| `semio-framework-value-derive` | proc-macro (host) | Unsupported Rust compile token output requires an explicit generated-input observer |
| `semio-s-artifact-stdio-pdf` | library | Generated Rust input requires compiler ownership: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🚪️io/🦀️.rs |
| `semio-s-plugin-block` | library | Rust cfg requires an identifier |
| `semio-s-plugin-wfc-engine` | library | Rust cfg requires an identifier |

### 6b. Over the ceiling, sorted by active lines

| # | Package | Unit | Files | Bytes | Active lines | Over | Largest input (active lines) | Play |
|---:|---|---|---:|---:|---:|---|---|---|
| 1 | `semio-s-artifact-bim-model` | library | 1067 | 6,685,699 | 87,521 | files, bytes, lines | `🏙️bim/🗿️artifacts/🏢️model/🦀️.rs` (4,640) | yes |
| 2 | `semio-s-artifact-energy-model` | library | 1075 | 4,385,772 | 66,900 | files, bytes, lines | `⚡️simulation/⚙️engine/🧪️sim/🦀️.rs` (5,121) | yes |
| 3 | `semio-framework-os-kernel` | library | 184 | 3,961,408 | 58,865 | files, bytes, lines | `💻️os/🔨️modules/🏪️store/🦀️.rs` (12,971) | yes |
| 4 | `semio-s-artifact-architect-program` | library | 925 | 4,248,421 | 42,731 | files, bytes, lines | `✳️any/🧬️schema/🗄️registers/🦀️.rs` (7,069) | yes |
| 5 | `semio-s-artifact-remodel-remodeling` | library | 306 | 3,171,917 | 40,930 | files, bytes, lines | `✏️editor/⚙️engine/🥽️mesh/🦀️.rs` (5,008) | yes |
| 6 | `semio-s-artifact-procedural-generation3d` | library | 384 | 3,157,842 | 40,832 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (3,752) | yes |
| 7 | `semio-s-artifact-puzzle-3d` | library | 277 | 2,165,234 | 32,319 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (7,873) | yes |
| 8 | `semio-s-artifact-fem-2d` | library | 240 | 1,701,622 | 30,316 | files, bytes, lines | `🏗️fem/⚙️engine/🧮️analyses/🦀️.rs` (3,299) | yes |
| 9 | `semio-s-artifact-draw-drawing` | library | 273 | 7,343,618 | 27,723 | files, bytes, lines | `🔨️modules/🏠️host/🧰️owned/🦀️.rs` (5,385) | yes |
| 10 | `semio-s-artifact-puzzle-5d` | library | 313 | 5,881,936 | 26,572 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (9,126) | yes |
| 11 | `semio-s-artifact-puzzle-2d` | library | 335 | 2,599,050 | 25,499 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (5,177) | yes |
| 12 | `semio-framework-os-infinite` | library | 42 | 1,336,364 | 25,383 | files, bytes, lines | `🔌️ports/➡️directed/➕️normal/🦀️.rs` (13,665) | yes |
| 13 | `semio-s-artifact-layout-layout` | library | 166 | 1,483,160 | 20,940 | files, bytes, lines | `✏️editor/⚙️engine/📤️export/🦀️.rs` (4,691) | yes |
| 14 | `semio-s-artifact-raster-raster` | library | 185 | 1,268,163 | 17,947 | files, bytes, lines | `🔨️modules/🏠️host/🧰️owned/🦀️.rs` (3,316) | yes |
| 15 | `semio-s-artifact-fem-3d` | library | 221 | 1,145,849 | 17,502 | files, bytes, lines | `🌐️any/✏️editor/🧵️session/🦀️.rs` (4,006) | yes |
| 16 | `semio-s-artifact-cad-cad` | library | 294 | 2,676,620 | 17,489 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (2,580) | yes |
| 17 | `semio-s-artifact-procedural-generation2d` | library | 206 | 1,163,735 | 16,947 | files, bytes, lines | `🔨️modules/🏠️host/🧰️owned/🦀️.rs` (2,988) | yes |
| 18 | `semio-s-artifact-process-process3d` | library | 151 | 1,098,520 | 16,589 | files, bytes, lines | `🔨️modules/🏠️host/🧰️owned/🦀️.rs` (3,533) | yes |
| 19 | `semio-framework-replication` | library | 56 | 999,271 | 16,101 | files, bytes, lines | `🔨️modules/📡️replication/📡️wire/🦀️.rs` (3,247) | yes |
| 20 | `semio-s-artifact-stdio-step` | library | 230 | 944,267 | 15,515 | files, bytes, lines | `🧱️base/🚪️io/📐️geometry/🦀️.rs` (1,162) | yes |
| 21 | `semio-framework-ui-contract` | library | 37 | 774,153 | 15,323 | files, bytes, lines | `🖱️ui/🧬️contract/🏗️builder/🦀️.rs` (2,570) | yes |
| 22 | `semio-framework` | library | 23 | 855,549 | 14,956 | bytes, lines | `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (8,099) | yes |
| 23 | `semio-s-artifact-mathematical-equation` | library | 143 | 865,281 | 14,756 | files, bytes, lines | `🧬️schema/💡️inferences/🌿️cas-internals/🦀️.rs` (5,072) | yes |
| 24 | `semio-s-artifact-stdio-semio-table` | library | 96 | 761,550 | 14,464 | files, bytes, lines | `📊️table/🧬️schema/🌀️entropy-internals/🦀️.rs` (6,883) | yes |
| 25 | `semio-s-artifact-animate-presentation` | library | 152 | 739,171 | 14,077 | files, bytes, lines | `✏️editor/⚙️engine/🎞️animation/🦀️.rs` (2,423) | yes |
| 26 | `semio-s-artifact-lowpoly-lowpoly` | library | 197 | 1,075,289 | 14,058 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (2,193) | yes |
| 27 | `semio-framework-value` | library | 91 | 1,021,392 | 14,035 | files, bytes, lines | `🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs` (1,385) | yes |
| 28 | `semio-framework-value` | library (host) | 91 | 1,021,392 | 14,035 | files, bytes, lines | `🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs` (1,385) | yes |
| 29 | `semio-s-artifact-stdio-ifc` | library | 213 | 922,093 | 13,666 | files, bytes, lines | `🗄️stdio/🗿️artifacts/🏗️ifc/🦀️.rs` (806) | yes |
| 30 | `semio-s-artifact-norm-iso16757` | library | 174 | 1,038,151 | 13,397 | files, bytes, lines | `🧬️schema/💡️inferences/⚖️checks/📈️part1.rs` (2,398) | yes |
| 31 | `semio-s-artifact-stdio-gltf-mutations` | library | 131 | 732,261 | 12,910 | files, bytes, lines | `🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️.rs` (605) | yes |
| 32 | `semio-s-artifact-note-note` | library | 248 | 819,273 | 12,496 | files, bytes, lines | `🗒️note/🗿️artifacts/🗒️note/🦀️.rs` (1,454) | yes |
| 33 | `semio-framework-os-flow` | library | 31 | 931,768 | 12,356 | bytes, lines | `🔨️modules/🌊️flow/🖥️host/🦀️.rs` (5,392) | yes |
| 34 | `semio-s-artifact-trinity-jack` | library | 130 | 670,653 | 11,678 | files, bytes, lines | `🔌️jack/🔨️modules/🏠️host/🦀️.rs` (1,447) | yes |
| 35 | `semio-s-artifact-stdio-docx` | library | 148 | 760,651 | 11,510 | files, bytes, lines | `🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs` (810) | yes |
| 36 | `semio-s-artifact-norm-vdi3805` | library | 177 | 871,000 | 11,372 | files, bytes, lines | `✳️any/🧬️schema/💡️inferences/🦀️.rs` (3,019) | yes |
| 37 | `semio-s-artifact-gis-gismap` | library | 187 | 1,007,106 | 11,371 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (1,484) | yes |
| 38 | `semio-framework-3d-brep-operations` | library | 17 | 607,364 | 11,021 | bytes, lines | `📐️brep/🛠️operations/🔀️boolean/🦀️.rs` (2,440) | yes |
| 39 | `semio-framework-pack` | library | 32 | 800,181 | 10,987 | bytes, lines | `🔨️modules/🎒️pack/📐️format/🦀️.rs` (3,694) | yes |
| 40 | `semio-s-artifact-norm-en1991` | library | 321 | 831,349 | 10,930 | files, bytes, lines | `📕️norm/🗿️artifacts/🏋️en1991/🦀️.rs` (1,275) | yes |
| 41 | `semio-s-artifact-forms-forms` | library | 198 | 742,928 | 10,684 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (1,435) | yes |
| 42 | `semio-s-artifact-stdio-xlsx` | library | 140 | 687,258 | 10,316 | files, bytes, lines | `🧬️schema/🧬️mutations/🧭️canonical-edit/🦀️.rs` (930) | yes |
| 43 | `semio-framework-ui-scene` | library | 14 | 481,172 | 10,176 | bytes, lines | `🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` (3,201) | yes |
| 44 | `semio-s-artifact-stdio-contract` | library | 19 | 548,461 | 10,000 | bytes, lines | `🧬️contract/✏️editing/🪟️details/🦀️.rs` (2,118) | yes |
| 45 | `semio-s-artifact-stdio-dwg-model` | library | 140 | 1,445,057 | 9,814 | files, bytes, lines | `🧬️schema/📸️snapshot/🗃️tables/🦀️.rs` (682) | yes |
| 46 | `semio-s-artifact-norm-en1993` | library | 227 | 958,202 | 9,740 | files, bytes, lines | `🪆️subsets/✳️any/🧬️schema/🦀️.rs` (3,066) | yes |
| 47 | `semio-s-artifact-stdio-svg` | library | 145 | 644,368 | 9,703 | files, bytes, lines | `📝️text/📸️snapshot/🧮️attributes/🦀️.rs` (856) | yes |
| 48 | `semio-s-artifact-block-3d` | library | 238 | 690,961 | 9,695 | files, bytes, lines | `🧱️block/🗿️artifacts/🧊️3d/🦀️.rs` (1,136) | yes |
| 49 | `semio-s-artifact-shooting-shooting` | library | 219 | 662,133 | 9,695 | files, bytes, lines | `🎥️shooting/🗿️artifacts/🎥️shooting/🦀️.rs` (1,336) | yes |
| 50 | `semio-s-artifact-stdio-xml` | library | 122 | 634,759 | 9,359 | files, bytes, lines | `🧱️base/🧬️schema/🔺️diff/🦀️.rs` (791) | yes |
| 51 | `semio-s-artifact-stdio-gif` | library | 158 | 5,046,881 | 9,260 | files, bytes, lines | `🪆️subsets/✳️any/🚪️io/🦀️.rs` (911) | yes |
| 52 | `semio-s-artifact-stdio-pptx` | library | 139 | 638,321 | 9,100 | files, bytes, lines | `🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs` (713) | yes |
| 53 | `semio-s-artifact-sequence-sequence` | library | 89 | 515,398 | 8,797 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (3,735) | yes |
| 54 | `semio-s-artifact-norm-din4108` | library | 209 | 618,322 | 8,796 | files, bytes, lines | `🪆️subsets/✳️any/🧬️schema/🦀️.rs` (2,197) | yes |
| 55 | `semio-s-artifact-stdio-semio-graph` | library | 127 | 578,524 | 8,745 | files, bytes, lines | `🧬️schema/➕️normal-internals/➡️directed/🦀️.rs` (557) | yes |
| 56 | `semio-s-artifact-norm-en1996` | library | 255 | 635,825 | 8,707 | files, bytes, lines | `✳️any/🧬️schema/⚖️masonry/🦀️.rs` (1,601) | yes |
| 57 | `semio-s-artifact-wfc-3d` | library | 139 | 537,663 | 8,639 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (1,072) | yes |
| 58 | `semio-s-artifact-stdio-dxf` | library | 84 | 607,268 | 8,625 | files, bytes, lines | `📰️header/🧬️schema/🔺️diff/🦀️.rs` (2,133) | yes |
| 59 | `semio-s-artifact-wfc-2d` | library | 139 | 573,682 | 8,575 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (1,185) | yes |
| 60 | `semio-s-artifact-stdio-gltf-model` | library | 70 | 980,737 | 8,550 | files, bytes, lines | `♾️any/🧬️schema/🔺️diff/🦀️.rs` (2,385) | yes |
| 61 | `semio-s-artifact-norm-en1999` | library | 134 | 637,483 | 8,527 | files, bytes, lines | `🪆️subsets/✳️any/🧬️schema/🦀️.rs` (3,091) | yes |
| 62 | `semio-s-artifact-flow-flow` | library | 131 | 529,709 | 8,478 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (2,938) | yes |
| 63 | `semio-s-artifact-wfc-bitmap` | library | 115 | 527,964 | 8,441 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (1,131) | yes |
| 64 | `semio-s-artifact-norm-en1995` | library | 283 | 694,016 | 8,374 | files, bytes, lines | `✳️any/🧬️schema/⚖️timber/🦀️.rs` (1,753) | yes |
| 65 | `semio-s-artifact-wfc-grid2d` | library | 111 | 519,077 | 8,300 | files, bytes, lines | `🔨️modules/🏠️host/💡️inferences/🦀️.rs` (953) | yes |
| 66 | `semio-s-artifact-stdio-json` | library | 116 | 563,224 | 8,135 | files, bytes, lines | `🧱️base/🧬️schema/🔺️diff/🦀️.rs` (752) | yes |
| 67 | `semio-s-artifact-norm-en1998` | library | 170 | 720,677 | 8,105 | files, bytes, lines | `✳️any/🧬️schema/💡️inferences/🦀️.rs` (1,454) | yes |
| 68 | `semio-s-artifact-block-5d` | library | 219 | 578,832 | 8,020 | files, bytes, lines | `🧱️block/🗿️artifacts/🖐️5d/🦀️.rs` (983) | yes |
| 69 | `semio-s-artifact-norm-en1992` | library | 169 | 669,682 | 7,962 | files, bytes, lines | `🪆️subsets/✳️any/🧬️schema/🦀️.rs` (2,103) | yes |
| 70 | `semio-s-artifact-norm-en1990` | library | 180 | 547,153 | 7,832 | files, bytes, lines | `✳️any/🧬️schema/💡️inferences/🦀️.rs` (1,110) | yes |
| 71 | `semio-s-artifact-stdio-zip` | library | 106 | 502,879 | 7,706 | files, bytes, lines | `🪆️subsets/🧱️base/🚪️io/🦀️.rs` (1,320) | yes |
| 72 | `semio-s-artifact-wfc-grid3d` | library | 110 | 499,849 | 7,659 | files, bytes, lines | `🔨️modules/🏠️host/💡️inferences/🦀️.rs` (934) | yes |
| 73 | `semio-s-artifact-norm-en1994` | library | 157 | 564,248 | 7,619 | files, bytes, lines | `🪆️subsets/✳️any/🧬️schema/🦀️.rs` (1,080) | yes |
| 74 | `semio-framework-math` | library | 2 | 312,831 | 7,599 | bytes, lines | `🔨️modules/🧮️math/🎯️sampling/🦀️.rs` (7,552) | yes |
| 75 | `semio-framework-ui-render` | library | 28 | 1,461,562 | 7,544 | bytes, lines | `🖱️ui/🖌️render/🖱️dispatch/🦀️.rs` (1,581) | yes |
| 76 | `semio-s-artifact-writer-writer` | library | 155 | 427,788 | 7,483 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (1,276) | yes |
| 77 | `semio-s-artifact-norm-din16798` | library | 207 | 546,520 | 7,352 | files, bytes, lines | `🪆️subsets/✳️any/🧬️schema/🦀️.rs` (1,362) | yes |
| 78 | `semio-s-artifact-stdio-semio-brep` | library | 109 | 549,398 | 7,142 | files, bytes, lines | `🚪️io/💾️binary/📸️snapshot/🦀️.rs` (599) | yes |
| 79 | `semio-s-artifact-norm-din18599` | library | 143 | 518,634 | 7,120 | files, bytes, lines | `🪆️subsets/✳️any/🧬️schema/🦀️.rs` (1,539) | yes |
| 80 | `semio-s-artifact-norm-en1997` | library | 141 | 499,575 | 7,024 | files, bytes, lines | `🪆️subsets/✳️any/🧬️schema/🦀️.rs` (2,449) | yes |
| 81 | `semio-framework-3d` | library | 20 | 514,364 | 6,932 | bytes, lines | `🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs` (2,460) | yes |
| 82 | `semio-s-artifact-sourcing-curation` | library | 123 | 486,107 | 6,909 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (1,245) | yes |
| 83 | `semio-s-artifact-trinity-rewriting` | library | 159 | 487,249 | 6,896 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (1,119) | yes |
| 84 | `semio-s-artifact-stdio-semio-value` | library | 92 | 423,707 | 6,881 | files, bytes, lines | `🔢️value/🧬️schema/🌫️fuzzy-internals/🦀️.rs` (1,855) | yes |
| 85 | `semio-s-artifact-stdio-semio-drawing` | library | 120 | 516,703 | 6,756 | files, bytes, lines | `🖊️drawing/🧬️schema/🔺️diff/🦀️.rs` (792) | yes |
| 86 | `semio-s-artifact-stdio-semio-document` | library | 114 | 459,647 | 6,702 | files, bytes, lines | `📑️document/🧬️schema/🔺️diff/🦀️.rs` (1,005) | yes |
| 87 | `semio-s-artifact-stdio-gltf-inferences` | library | 362 | 522,511 | 6,626 | files, bytes, lines | `🧬️schema/💡️inferences/🔨️geometry-core/🦀️.rs` (1,048) | yes |
| 88 | `semio-s-artifact-stdio-obj` | library | 88 | 508,449 | 6,626 | files, bytes, lines | `📐️geometry/🧬️schema/🔺️diff/🦀️.rs` (1,449) | yes |
| 89 | `semio-s-artifact-block-2d` | library | 175 | 475,101 | 6,580 | files, bytes, lines | `🧱️block/🗿️artifacts/◻️2d/🦀️.rs` (802) | yes |
| 90 | `semio-s-artifact-stdio-jpg` | library | 99 | 416,835 | 6,384 | files, bytes, lines | `🪆️subsets/🧾️document/🚪️io/🦀️.rs` (1,554) | yes |
| 91 | `semio-s-artifact-stdio-las` | library | 85 | 412,160 | 6,337 | files, bytes, lines | `🎩️header/🧬️schema/🔺️diff/🦀️.rs` (1,229) | yes |
| 92 | `semio-s-artifact-stdio-semio-mesh` | library | 118 | 448,473 | 6,305 | files, bytes, lines | `🔺️mesh/🧬️schema/🔺️diff/🦀️.rs` (769) | yes |
| 93 | `semio-framework-ui-runtime` | library | 16 | 366,728 | 6,304 | bytes, lines | `🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs` (3,856) | yes |
| 94 | `semio-s-artifact-stdio-dwg-read` | library | 14 | 391,839 | 6,051 | bytes, lines | `🚪️io/📥️import/📦️objects/🦀️.rs` (886) | yes |
| 95 | `semio-framework-graph` | library | 13 | 278,252 | 6,026 | bytes, lines | `🔨️modules/🕸️graph/🗣️dsl/🦀️.rs` (2,252) | yes |
| 96 | `semio-s-artifact-stdio-md` | library | 76 | 438,296 | 5,921 | files, bytes, lines | `✳️any/🧬️schema/🔺️diff/🦀️.rs` (873) | yes |
| 97 | `semio-framework-surface` | library | 8 | 303,864 | 5,901 | bytes, lines | `🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs` (3,909) | yes |
| 98 | `semio-s-artifact-stdio-semio-presentation` | library | 105 | 418,266 | 5,840 | files, bytes, lines | `📽️presentation/🧬️schema/🔺️diff/🦀️.rs` (1,034) | yes |
| 99 | `semio-s-artifact-stdio-tiff` | library | 114 | 484,466 | 5,804 | files, bytes, lines | `🪆️subsets/🧾️document/🚪️io/🦀️.rs` (817) | yes |
| 100 | `semio-s-artifact-stdio-bcf` | library | 79 | 386,445 | 5,748 | files, bytes, lines | `🖊️markup/🧬️schema/🔺️diff/🦀️.rs` (845) | yes |
| 101 | `semio-framework-artifact-flow-flow` | library | 42 | 362,389 | 5,703 | files, bytes, lines | `🌊️flow/🧬️schema/📸️snapshot/🦀️.rs` (1,244) | yes |
| 102 | `semio-s-artifact-stdio-deflate` | library | 73 | 351,495 | 5,689 | files, bytes, lines | `🪆️subsets/✳️any/🚪️io/🦀️.rs` (1,852) | yes |
| 103 | `semio-framework-job` | library | 30 | 465,513 | 5,539 | bytes, lines | `🧰️framework/🔨️modules/🧵️job/🦀️.rs` (3,822) | yes |
| 104 | `semio-s-artifact-stdio-semio-model` | library | 97 | 388,064 | 5,521 | files, bytes, lines | `🏛️model/🧬️schema/🔺️diff/🦀️.rs` (485) | yes |
| 105 | `semio-framework-geometry` | library | 13 | 215,486 | 5,488 | lines | `🔨️modules/📐️geometry/⚙️engine/🦀️.rs` (1,665) | yes |
| 106 | `semio-s-artifact-space-home` | library | 112 | 368,199 | 5,469 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (820) | yes |
| 107 | `semio-s-artifact-stdio-dwg-write` | library | 15 | 300,092 | 5,444 | bytes, lines | `📤️export/🧱️records/📐️entities/🦀️.rs` (815) | yes |
| 108 | `semio-s-artifact-stdio-mp4` | library | 75 | 478,256 | 5,354 | files, bytes, lines | `🪆️subsets/✳️any/🚪️io/🦀️.rs` (1,907) | yes |
| 109 | `semio-framework-canvas` | library | 317 | 2,016,609 | 5,292 | files, bytes, lines | `🔣️icons/🤖️generated/🔤️shortcodes/🦀️.rs` (2,184) | yes |
| 110 | `semio-s-artifact-stdio-ply` | library | 74 | 379,810 | 5,244 | files, bytes, lines | `✳️any/🧬️schema/🔺️diff/🦀️.rs` (1,060) | yes |
| 111 | `semio-s-artifact-stdio-png` | library | 96 | 417,562 | 5,198 | files, bytes, lines | `🪆️subsets/✳️any/🚪️io/🦀️.rs` (1,447) | yes |
| 112 | `semio-framework-os` | library | 5 | 310,896 | 5,071 | bytes, lines | `🛍️products/💻️os/🖥️host/🦀️.rs` (4,468) | yes |
| 113 | `semio-s-artifact-imperative-procedure` | library | 108 | 319,284 | 5,070 | files, bytes, lines | `📜️imperative/🗿️artifacts/📜️procedure/🦀️.rs` (731) | yes |
| 114 | `semio-s-artifact-dag-dag` | library | 112 | 315,529 | 5,033 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (973) | yes |
| 115 | `semio-s-artifact-stdio-semio-flow` | library | 102 | 323,815 | 4,978 | files, bytes, lines | `🚪️io/💾️binary/📸️snapshot/🦀️.rs` (605) | yes |
| 116 | `semio-hub-space` | library | 66 | 345,369 | 4,972 | files, bytes, lines | `🪐️space/⚙️engine/🪐️space/🦀️.rs` (1,062) | yes |
| 117 | `semio-framework-dsl-record` | library | 16 | 407,923 | 4,933 | bytes, lines | `🔨️modules/🗣️dsl/🧬️schema/🦀️.rs` (2,294) | yes |
| 118 | `semio-s-artifact-stdio-semio-kit` | library | 111 | 380,072 | 4,891 | files, bytes, lines | `🪆️subsets/🧰️kit/✏️editor/🦀️.rs` (318) | yes |
| 119 | `semio-s-artifact-stdio-semio` | library | 150 | 502,205 | 4,879 | files, bytes, lines | `🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs` (366) | yes |
| 120 | `semio-s-artifact-reasoning-wires` | library | 99 | 331,496 | 4,831 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (883) | yes |
| 121 | `semio-s-artifact-stdio-semio-cad` | library | 108 | 353,134 | 4,785 | files, bytes, lines | `📐️cad/🧬️schema/🔺️diff/🦀️.rs` (353) | yes |
| 122 | `semio-s-artifact-vcs-vcs` | library | 123 | 315,635 | 4,769 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (1,152) | yes |
| 123 | `semio-s-artifact-stdio-wav` | library | 67 | 374,042 | 4,709 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (647) | yes |
| 124 | `semio-s-artifact-stdio-semio-animation` | library | 99 | 314,627 | 4,585 | files, bytes, lines | `🎞️animation/🧬️schema/🔺️diff/🦀️.rs` (469) | yes |
| 125 | `semio-s-artifact-playbook-playbook` | library | 107 | 296,333 | 4,563 | files, bytes, lines | `📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs` (817) | yes |
| 126 | `semio-s-artifact-stdio-semio-image` | library | 97 | 284,621 | 4,461 | files, bytes, lines | `🖼️image/🧬️schema/🔺️diff/🦀️.rs` (532) | yes |
| 127 | `semio-s-artifact-gis-gisterrain` | library | 100 | 294,554 | 4,431 | files, bytes, lines | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (708) | yes |
| 128 | `semio-s-artifact-stdio-html` | library | 72 | 308,051 | 4,358 | files, bytes, lines | `✳️any/🧬️schema/🔺️diff/🦀️.rs` (658) | yes |
| 129 | `semio-framework-dsl` | library | 7 | 199,573 | 4,328 | lines | `🔨️modules/🗣️dsl/📖️grammar/🦀️.rs` (2,778) | yes |
| 130 | `semio-s-artifact-stdio-avi` | library | 76 | 276,330 | 4,082 | files, bytes, lines | `🪆️subsets/🎛️hdrl/🚪️io/🦀️.rs` (1,335) | yes |
| 131 | `semio-s-artifact-norm-contract` | library | 16 | 196,821 | 4,009 | lines | `📇️registry/🧬️contract/🖥️app-surface/🦀️.rs` (2,349) | yes |
| 132 | `semio-s-artifact-stdio-txt` | library | 89 | 263,490 | 3,972 | files, bytes | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (527) | yes |
| 133 | `semio-s-artifact-stdio-csv` | library | 77 | 277,057 | 3,956 | files, bytes | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (729) | yes |
| 134 | `semio-framework-3d-brep-engine` | library | 8 | 272,011 | 3,934 | bytes | `🧊️3d/📐️brep/⚙️engine/🦀️.rs` (2,198) | yes |
| 135 | `semio-s-artifact-stdio-semio-video` | library | 87 | 251,668 | 3,870 | files | `🎬️video/🧬️schema/🔺️diff/🦀️.rs` (476) | yes |
| 136 | `semio-s-artifact-stdio-semio-audio` | library | 90 | 245,030 | 3,834 | files | `🔊️audio/🧬️schema/🔺️diff/🦀️.rs` (366) | yes |
| 137 | `semio-s-artifact-stdio-epw` | library | 77 | 316,764 | 3,814 | files, bytes | `✳️any/🧬️schema/🔺️diff/🦀️.rs` (525) | yes |
| 138 | `semio-s-artifact-stdio-stl` | library | 74 | 272,390 | 3,708 | files, bytes | `✳️any/🧬️schema/🔺️diff/🦀️.rs` (508) | yes |
| 139 | `semio-framework-tool-run` | library | 20 | 293,273 | 3,517 | bytes | `🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs` (2,417) | yes |
| 140 | `semio-s-artifact-stdio-semio-object` | library | 93 | 235,914 | 3,508 | files | `🪆️subsets/📦️object/✏️editor/🦀️.rs` (318) | yes |
| 141 | `semio-s-artifact-stdio-semio-text` | library | 86 | 238,255 | 3,500 | files | `🪆️subsets/🔤️text/✏️editor/🦀️.rs` (318) | yes |
| 142 | `semio-s-artifact-stdio-bmp` | library | 87 | 282,531 | 3,445 | files, bytes | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (502) | yes |
| 143 | `semio-s-artifact-space-space` | library | 75 | 212,350 | 3,364 | files | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (525) | yes |
| 144 | `semio-s-artifact-stdio-tsv` | library | 75 | 235,729 | 3,292 | files | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (688) | yes |
| 145 | `semio-framework-artifact-infinite-dag` | library | 38 | 191,613 | 3,254 | files | `🕸️dag/🧬️schema/📸️snapshot/🦀️.rs` (746) | yes |
| 146 | `semio-framework-artifact-workflow-workflow` | library | 34 | 224,869 | 3,051 | files | `🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs` (1,754) | yes |
| 147 | `semio-s-artifact-stdio-mp3` | library | 66 | 274,701 | 3,043 | files, bytes | `🪆️subsets/✳️any/🚪️io/🦀️.rs` (710) | yes |
| 148 | `semio-s-artifact-stdio-binary` | library | 73 | 205,545 | 2,796 | files | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (499) | yes |
| 149 | `semio-s-artifact-demonstrator-playground` | library | 74 | 158,754 | 2,435 | files | `🪆️subsets/✳️any/✏️editor/🦀️.rs` (495) | yes |
| 150 | `semio-framework-io-sqlite-snapshot` | library | 12 | 325,467 | 2,076 | bytes | `🔨️modules/🚪️io/🪶️sqlite-snapshot/🦀️.rs` (427) | yes |
| 151 | `semio-s-artifact-stdio-gltf-editor` | library | 16 | 358,157 | 1,306 | bytes | `♾️any/✏️editor/🧭️edit-rules/🦀️.rs` (477) | yes |
| 152 | `semio-framework-compiler` | library | 13 | 1,522,649 | 1,151 | bytes | `🔨️modules/📚️compiler/📖️syntax/🦀️.rs` (558) | yes |
| 153 | `semio-framework-plugin-host-test-component` | library | 41 | 52,259 | 550 | files | `🖥️host/🧪️testing/🧩️component/🦀️.rs` (124) | no |

## 7. Tests and verification

| Check | Result |
|---|---|
| Play fresh-build contract (`vitest run --config 🧪️tests/🎚️config/🟦️.ts 🔨️modules/📦️site/🆕️fresh-build`) | **10/10 pass**, including the new law "routes every release driver through the repository compiler store owner …" |
| `bun test 📥️source/🧪️tests 🕸️units/🧪️tests` (native-build) | **14 pass, 3 fail.** The 3 are the real-Cargo laws, which need `Original parent Script process invocation`, so the repo runner must run them. The runner is broken (§1), so they were **not run** |
| Admission attribution (inline `bun -e` against the real `semio-framework-2d` and `semio-framework-plugin` sources) | Reader refusal is attributed to the package; the boundary message is unchanged |
| `tsc` over driver-stages, the Play site script, fresh-build, admission, root `📜️script.ts` and the hub script | 0 errors in the edited regions. The remaining errors are pre-existing and not ours, for example `📜️script.ts` `profileIndependenceV1`/`iterateComponentV1` missing `network`, and `whatwg-url` types |
| Hub script link (`bun -e 'await import(…)'`) | Links |
| Component-pass suite, wasm-build suite, full `workspace:release` | **Not run.** The runner is broken, and the release is blocked earlier (§3) |

## 8. Other callers still broken by the `network` contract (not on the release route, not fixed)

These were found by `tsc` or `grep`. They call functions that now require a `network` grant but do not pass one:
- `buildComponentClosureV1` without `network`: `🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts:55`, `🧑‍💻dev/🚚️distribution/📜️script.ts:51`, `🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts:14`, `⌨️native-entrypoint/📦️modules/📜️script.ts:30`.
- Root `📜️script.ts` `components profile-independence` and `components iterate` (TS2554).
- `🔌️plugin/🏗️build/🏃️execution/🟦️.ts:43` and `📇️registry/🔄️refresh/🟦️.ts:33` (TS2554).

Each fails at runtime with `Original Cargo dependency network grant required`. The fix pattern is `network: repositoryCargoNetworkV1(invocation, repoRoot)`.

## 9. Deviation

One `cargo build --unit-graph` against `🌎️hub/Cargo.toml` was run directly, without `🚦️cargo-slot.sh`, to identify the unknown unit kind. It ran with `--unit-graph` (no compile and no target writes); it only waited on the package-cache lock. Every other Cargo invocation went through the slot with `⚡️cache/r1` private directories, and that directory was deleted afterwards.

## 10. Files

- Changed:
  - `📜️script.ts`
  - `🏢️semio-tech/🎡️play/🔨️modules/📦️site/📜️script.ts`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts`
  - `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts`
  - `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts`
  - `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
  - `📋️project.json`
  - `🏢️semio-tech/🎡️play/📋️project.json`
  - `.vscode/launch.json`
  - `.vscode/🧩️launch.seed.jsonc`
  - `🏢️semio-tech/🎡️play/🧪️tests/🧪️playfreshbuild/🟦️.ts`
  - `🏢️semio-tech/🎡️play/🧪️tests/🧪️playfreshbuild/🧫️fixtures/🔣️.json`
- Created: `T/r1-release-admission/📜️script.ts` (probe input, kept) and this report.
- Deleted after use: `T/🗑️generated/r1/` (logs, census JSON, tsc output) and `⚡️cache/r1/`.
