# Result lay16: re-measure of every boundary rule, root-cause fixes for all rules except the export-key-blocked `no-cross-package-relative` edges

Executor `lay16`. C = this ticket folder, tools and kept inputs in `C/lay16-tools/`. HARD FREEZE respected: no `package.json`/`bun.lock`, nothing under the repo-product `📚️library/`, no root `📜️script.ts`, no `🔣️taxonomy.json` edit; everything that needs one of those is staged (section 5). No sub-agent, no modifying git command, no cargo, no dependency sync/lock/show, no product activation. `C/exports-stage-tools/` (the export-key batch) was not touched or duplicated.

## 1. Measurement method

The 12-area dependency-cruiser walk of lay13 is slow under load (9 min for one area at load 50, `rc=142` for the second). So the measurement is a static evaluator of the live policy, `lay16-tools/scan.ts`:

- policy: `node lay16-tools/policy-dump.cjs <out>/policy.json` dumps the dependency-cruiser config built from the live taxonomy and manifests (all 303 `no-cross-package-relative-*` rules and `no-circular` are left to the cruiser);
- edges: every import specifier of 16,224 tracked/untracked source files (`ts.preProcessFile`, compact code included), relative and `@semio-tech/*` workspace aliases resolved through the real package `exports` (realpath, conditions `semio-source/import/require/node/default`); dependency types `local` (relative) and `aliased,aliased-workspace,local` (alias);
- rules: `from`/`to` `path`/`pathNot`/`dependencyTypes(Not)` of every forbidden rule evaluated with the cruiser's own regex semantics; `lay16-tools/cycles.ts` computes the strongly connected components of the production graph (paths with `🧪️tests/` excluded like `no-circular`).

Calibration against the real cruiser, same config minus the ncpr rules (`lay16-tools/area-config.cjs`, `cruise-some.sh`), run before and after my edits:

| area | cruiser before | cruiser after | scanner after |
| --- | --- | --- | --- |
| splug1 (9 plugins) | sdk-only 38, plugin-no-ext 8, nct 5 | sdk-only 6, plugin-no-ext 3, nct 5 | 6 / 3 / 5 (before: 38 / 8 / 5, identical) |
| splug4 (9 plugins) | not run | sdk-only 25, plugin-no-ext 13, nct 19 | 25 / 13 / 19 |
| osrenderer | not run | renderer-hosts 47 | 45 |

The two missing renderer edges are `bun:sqlite` imports in two host tests (a builtin the scanner does not model); they are counted below as `+2`.

## 2. Result: rule, before, after (error severity, whole repository)

| rule | before | after | after the staged items of section 5 (expected) |
| --- | ---: | ---: | ---: |
| `plugins-framework-sdk-only` | 214 | **37** | about 21 (rest: section 4) |
| `renderer-hosts-only-ui` | 54 + 2 | **45 + 2** | about 8 + 2 (more only if the exports-stage moves add OS fixture imports, decision 4) |
| `framework-no-products` | 28 | 28 | 3 (exports-stage moves 19+, 3 corpus files, 1 blocked) |
| `no-cross-technology` | 34 | 34 | about 8 (all 34 are missing export keys, 26 are in the exports-stage batch) |
| `plugin-no-extension-or-artifact` | 21 | **16** | 16 |
| `ui-no-framework-packages` | 10 | **8** | **0** |
| `ui-foundation-closure` | 2 | **1** | **0** |
| `no-circular` | 1 (source) | **0** | 0 |
| `ui-runtime-no-test-support`, `no-state-outside-os`, `no-core-path`, `framework-no-plugin-packages`, `no-cross-plugin-*` | 0 | 0 | 0 |

Why "before" is not the 155 of lay15: peers added about 60 plugin edges since (the lifted `*-package-commands` modules import the process port). Not measured here: `no-cross-package-relative` (exports-stage), `not-to-unlisted`, `no-escaping-relative-imports`, the warning-level `no-impl-segment`.

By owner (rule :: area, before to after): sdk-only :: s plugins 214 to 37; renderer-hosts :: OS renderer 54 to 45; plugin-no-ext :: s plugins 21 to 16 (fem 12, raster 3, puzzle 1); ui-no-framework :: framework ui 10 to 8; ui-foundation :: framework modules 2 to 1; framework-no-products :: SDK glue 25 + replication test 3, unchanged; nct :: s plugins 27, s other 4, hub 3, unchanged.

## 3. Changes (160 files; list in `lay16-tools/changed-files.txt`)

### 3.1 Plugin to framework: `plugins-framework-sdk-only` 214 to 37 (143 files)

- `lay16-tools/sdkfix.ts` re-points every violating import to an entry of the admitted package. A candidate is accepted only when (a) the replacement does not violate any rule of the live policy (the policy is evaluated per candidate, so a plugin can only land on `@semio-tech/framework`), (b) every imported name has the identical deep origin through the entry (AST, `export {a as b} from`, `export *`, type/value kind), (c) bare workspace re-exports of the SDK glue are resolved, (d) `import("…")`, `require`, `export … from` and `vi.mock`-style string uses of the same specifier are rewritten together. `📜️script.ts` and config files that had a relative path keep a relative path to the SDK glue (zero-touch), everything else uses the bare key.
- Result: 129 imports of `process/routing` (108 relative deep paths into the process package glue in the plugin bridge and package scripts), 16 `process/execution`, 12 `process/cargo-test`, 7 `process/compiler-capacity`, 7 `process/cargo-laws`, 7 `value/ieee754`, `process/test-budget`, `replication`, `replication/mutation/map/schema`, `compiler/syntax/module-tree`, `schema/oracles/validator`, `schema`; 6 SDK glue files extended by explicit names (`process/execution` +`resolveTypescriptCompilerV1`, `replication` +`encodeScopedIdentity,decodeScopedIdentity`, `actor/wire-turn` +`TypedOperationToken`, `process/typescript-check` +`typecheckAdmittedProjectV1`, `value/resident` +`OwnedResident*`).
- Port swaps (not identity): the plugin scripts that imported the repo product's implementation now use the framework port that the repo-lib provider is bound to (`runPackageTests: runRepositoryCargoTests`, `runCommand: runRepositoryCommand`): `runRepositoryCargoTests` to `runAdmittedCargoPackageTestsV1` (fem 2d/3d generator scripts), `runRepositoryCommand` from `@semio-tech/repo-lib/process` to `@semio-tech/framework/process/execution` (energy package script and `rust-package-commands`).
- Renderer tests that imported `@semio-tech/value-ts/resident` use the SDK entry (`value/resident`, 59 specifiers in 7 `UiDocumentStore` tests), and two edges through `actor/wire-turn` and `process/typescript-check` (the renderer-hosts rule allows the SDK package).

### 3.2 `plugin-no-extension-or-artifact` 21 to 16

`⚡️electrical/🧪️tests/🔆️pv-plane` (energy plugin core) imported five artifact mutation schemas. `verifySolarPvOracle(schemas)` now receives them (`SolarPvMutationSchemas`), and the artifact's own `rust-package-commands` module, the only caller, imports and injects them (the artifact may depend on the plugin, not the reverse). tsc with the caller's flags: clean.

### 3.3 Cycle

`🏢️semio-tech/🎡️play`: the in-source test block of `🔨️modules/🧩️runtime` imported `🪧️brand.ts`, which imports the runtime. The pane-defaults registration moved into `🪧️brand.ts`'s own in-source block (it already imports the runtime); source SCCs 1 to 0 (`cycles.ts`, 9,174 production nodes).

### 3.4 ui foundation closure and ui boundary (3 edges)

- `🕹️interaction` re-exported the scoped-identity codec from `📡️replication` (interaction is a ui-foundation module, replication is not). The re-export is removed from the module and from the SDK `interaction` glue; the one consumer (fem story scenes) imports it from `@semio-tech/framework/replication`, where it already was. The Rust twin keeps its `pub use`.
- `ui/🎯️targets/⚛️react/🗣️terminology` imported `ephemeralSet` and `StoragePort` through the whole `@semio-tech/framework` barrel: now `@semio-tech/framework/transient` and the ui-owned `🗄️storage-port` module. The ui `introduction` test imports `IntroductionDefinition` from its foundation module `🎓️onboarding/🏷️declaration` instead of the barrel.

### 3.5 Tools kept (`C/lay16-tools/`)

`scan.ts`, `cycles.ts`, `sdkfix.ts` (+`list.ts`, `group.ts`, `coverage.ts`, `importers.ts`, `bundle-check.ts`, `errs-in-changed.ts`), `area-config.cjs`, `cruise-area.sh`, `cruise-some.sh`, `areas.txt`, `policy-dump.cjs`, `sdk-keys-needed.json`, `changed-files.txt`, `glue-edited.txt`, `staged/`.

## 4. What is left and why (nothing here is fixable without a frozen file or a ruling)

| rule | left | blocker | after the sequence in section 6 |
| --- | ---: | --- | --- |
| sdk-only | 37 | **A** 11 edges (5 targets: `infinite/world/r3f`, `math/algebra/dense`, `…/dense/inverse`, `job/context/testing/caller/testing`, `ui/contract/testing/color-input`): the key exists in the exports-stage batch for `@semio-tech/framework`; rerun `sdkfix.ts`. **B** 5 edges (+1 DSL oracle): SDK mirrors of owner entries that the batch only adds to the owner package (`actor/retained-turn`, `value/sort/testing`, `tool-run/payload/stream`, `replication/…/shared-map-delta-source`, DSL variant-field oracle): `staged/sdk-mirror.ts`. **C** 14 edges: plugin `renderer-contract`, `canvas-catalogue`, `session-factory`, `brush-mesh-upload`, `pixel-selection`, `renderer-boot-selection`, gismap inference probe and svg unit tests import renderer elements, oracles and the renderer react vitest config. These tests exercise one plugin against the real renderer; the SDK should not export renderer internals, they belong to `✏️s/🧪️tests/🏘️plugin-composition` (not a plugin owner). **D** 6 edges: `fem/🧪️tests/🧪️shared-authority` and `camera` consume OS plugin-declaration contracts (`operationEntityId`, `renderRevisionMatches`, view schemas/fixtures, `testCameraPresenceContract`) that live in `@semio-tech/framework-plugin`; the SDK cannot re-export OS code (`framework-no-products`): ruling needed (admit `framework-plugin` as an SDK package, or move the declaration contracts below `🧰️framework/🔨️modules`) | about 21 |
| renderer-hosts | 45 (+2) | actor `lifetime`, `lifetime/patch`, `page`, root `Lane`/`TurnScheduler`, `wire-turn/diagnostic`, `diagnostic/controlled`, `infinite/world/r3f` (17 edges) have no `@semio-tech/framework` key: SDK mirrors (`staged/sdk-mirror.ts`) or the batch's r3f key. Policy-conflict edges: media (`MediaTransportHost` to the plugin window-kit contract, 3 edges), renderer oracles used by `react/📜️script.ts` (3), OS fixtures and root barrel in host tests (to be re-measured after the exports-stage moves). `bun:sqlite` oracles in `Scenes/…/wgpu-admitted-surface-map` and `NodeGraph/📋️runtime/🧪️tests` (+2): the policy corpus forbids `bun:sqlite` in hosts on purpose; the tests are third-party cross-checks and belong to `engine/🧪️tests/` (their Rust twin `#[path]` in `🗃️scene-state/🦀️.rs` moves with the Scenes one) | about 8 (+2) |
| framework-no-products | 28 | 25 misplaced glue files under `@semio-tech/framework` are handled by the exports-stage "move" tier (19 moved, importers rewritten); 3 are the replication retained-mutation test reading three OS `dsl/✨️derive` corpora (`include_str!` twins and the schema index of the derive crate point at them: needs cargo to move) | 3 |
| no-cross-technology | 34 | every edge targets a module without an export key; the batch covers 26 | about 8 |
| plugin-no-ext | 16 | fem `🧪️tests/🧱️field-intent` (12): plugin-level tests of the 2d/3d artifact mutation schemas; raster (3) and puzzle-5d (1) plugin packages expose artifact modules through glue below the plugin package directory (the package has to be owned by the artifact, a new workspace package) | 16 |

## 5. Staged behind the freeze (`C/lay16-tools/staged/`)

- `apply.sh [--apply]` (anchored, idempotent, dry run by default, validates the JSON and that every owner path exists and foundation/test-support do not overlap): patch `taxonomy-ui-roles` adds to `dependencyDirections.roles`: `ui-foundation` gets the leaf module `🧰️framework/🔨️modules/🔏️hash` (no imports) with its SDK glue, and the generated-axes glue `📦️packages/🟦️typescript/🛂️manifest/🎚️ui-axes` (the manifest key `manifest/generated/ui-axes` and the law fixture point there; the older `🤖️generated` glue copy is an orphan); `ui-test-support` gets process `🎛️execution` and `🏗️build/🎟️capacity` with their SDK glue (the lifted ui package-command tests run toolchains). Dry run: both anchors match. Effect by construction: ui-no-framework 8 to 0, ui-foundation 1 to 0.
- `sdk-mirror.ts [check|apply]` + `sdk-mirror.json`: 11 SDK entries (`actor`, `actor/page`, `actor/lifetime`, `actor/lifetime/patch`, `actor/retained-turn`, `actor/wire-turn/diagnostic[/delivery]`, `diagnostic/controlled`, `value/sort/testing`, `tool-run/payload/stream`, `replication/…/shared-map-delta-source`) following the SDK convention (explicit re-exports from the owner's bare key). Computed from the owner entry at run time; `check` today: 7 keys ready, 4 pending the exports-stage keys.
- `sdk-keys-needed.json`: the 41 remaining sdk-only/renderer targets with imported names, importers, and the keys the batch already stages for them (classes A to D above).

## 6. Sequence after the freeze lifts

1. Exports-stage procedure (`result-exports-stage.md` section 6), then `bun repo dependencies lock`.
2. `C/lay16-tools/staged/apply.sh --apply` (taxonomy), `bun C/lay16-tools/staged/sdk-mirror.ts apply`.
3. `node C/lay16-tools/policy-dump.cjs C/🗑️generated/lay16-run/policy.json`, `bun C/lay16-tools/scan.ts <policy.json> <out.tsv> --refresh`, then `bun C/lay16-tools/sdkfix.ts <edges of plugins-framework-sdk-only> <plan.json> sdk apply` and the same with `any` for renderer-hosts, ui and nct edges (before `lay15-tools/ncpr2.ts`, which would choose owner packages and trade one rule for another). Review dry runs first: the barrel gap-fill of repo-lib entries (`sdkfix` offered `repo-lib/inventory|testing|cargo` for 3 nct edges) must not be applied, those need the narrow keys of the batch.
4. `bun C/lay16-tools/cycles.ts <graph>` and one cruiser pass per changed area (`cruise-some.sh`).

## 7. Verification (exact commands)

- Calibration: `PAR=1 C/lay16-tools/cruise-some.sh a1 splug1` (4 min 32 s), `PAR=2 … a1 splug4 osrenderer` (4 min 10 s); `bun C/lay16-tools/scan.ts … --roots <splug1 plugins>` equal per rule; `b0-splug1` (cruiser, before) vs scanner before: equal.
- Type check: tsc 5.9, strict, root config, `include` = the 160 changed files (`lay16-tools/changed-files.txt`), 12 GB heap: rc 0, 0 diagnostics (3,109 files in the program). `bun build --no-bundle` of every changed `.ts/.tsx`: 0 parse failures. `bun build --target=bun` of the 136 first-pass files: ok; of the second pass: 10 ok, 8 fail only on `?raw` Vite imports and the uninstalled `chromium-bidi`, both pre-existing and unrelated to the edits.
- Runtime: `bun test ./✏️s/🔌️plugins/🏗️fem/🧪️tests/🧱️field-intent/🧩️geometry/🟦️.ts` 6 pass (rewritten `value/ieee754` import); the writer bridge script and the fem generator script load through the new SDK path and stop at the expected "Original parent Script process invocation required"; `bun build --target=bun 📜️script.ts` ok (24.1 MB); `bun repo --help` boots.
- `staged/apply.sh` dry run: `taxonomy-ui-roles: anchored, anchored`. `sdk-mirror.ts check`: 7 keys, 4 pending.
- Not run: vitest suites of the 143 rewritten plugin/renderer test files (bun/vitest runners under load 50 to 70; the renderer `UiDocumentStore` tests also need the Vite `?raw` pipeline), the full 12-area cruise (the scanner stands in, calibrated above), cargo, any product activation.

## 8. Open decisions

1. SDK scope for the OS plugin-declaration contracts (D above): admit `@semio-tech/framework-plugin` as a plugin-facing package or move the contracts into `🧰️framework/🔨️modules`.
2. Renderer-contract tests of plugins (C above): relocate to `✏️s/🧪️tests/🏘️plugin-composition`; they need the composition `typescript-package-commands` list and the renderer react vitest config alias.
3. `bun:sqlite` in host tests: relocate the two oracles to `engine/🧪️tests/` (preferred) or allow `bun:sqlite` from host test sources.
4. After the exports-stage moves, host tests import OS fixtures from `@semio-tech/framework-os/…`, which the renderer-hosts allow-list does not admit (only `./modules/renderer/engine/elements/`): re-measure first, then either declare the test-fixture subpaths or add a test-source allowance.
5. fem field-intent suite (12 edges): move into the 2d and 3d artifact test directories and extend `🧪️tests/🎚️config` includes.
