# Root Artifact and Dependency Source Extraction

## Result

The extracted root WGPU projection/publication branches, JCO package-adapter publication branch, dependency inventory, dependency truth model, and dependency command branch now live in six anonymous TypeScript leaves under their semantic owners. Their former root and package `📜️script.ts` branches retain task dispatch and sequencing. Tests and current command consumers import the semantic owners directly; no compatibility export or runtime adapter remains for these extracted APIs.

The WGPU generator still owns exactly six outputs. After the concurrent framework facade settled, its exact browser graph admitted the scene owner and rejected the no-longer-reached mesh facade. The authoritative generator refreshed the frame-worker and browser-boot outputs; both the direct and registered freshness routes then reported six exact artifacts and zero changes. The JCO adapter preview retains its single schema-owned Rust leaf, canonical mode and exclusive creation behavior, and that leaf compiles with native Cargo.

The dependency self-test, current inventory summary, JS lock parity scan, and ratchet all execute from the extracted owners. The repository's zero-dependency target remains unmet: this slice preserves the baseline and reports the live failures without widening classifications or writing a new baseline.

## Source Ownership Map

| Former implementation host | Anonymous semantic owner | Responsibility |
| --- | --- | --- |
| root `📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📽️projection/🟦️.ts` | exact no-follow WGPU browser inputs, Bun browser compilation, package/catalog identity, pinned Bun authority, deterministic six-output rendering and progress/cancellation callbacks |
| root `📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️publication/🟦️.ts` | bounded projected stdin, preview manifest, no-follow freshness checks and publication of the six owned outputs |
| WGPU TypeScript package `📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🏗️builder/🟦️.ts` | frame-worker selection from the canonical projection, generation, byte freshness and credential-carrier census |
| root `📜️script.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📽️projection/🧩️package-adapter/📦️publication/🟦️.ts` | exact JCO package-adapter preview, freshness and exclusive publication |
| root `📜️script.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/📇️inventory/🟦️.ts` | Cargo, JS/Bun, Go, Python and .NET declaration discovery/parsing; phase and oracle classification; lock parity; first/third-party inventory; baseline read/check/write |
| root `📜️script.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts` | literal-external truth model, mandated toolchain audit, policy-router boundary and all `verify dependencies` command behavior |

The WGPU projection and package-adapter publication are separate concerns. The JCO path explicitly carries `🧩️package-adapter/📦️publication`; it is not placed in a generic WGPU publication bucket.

## Consumer, Producer and Input Closure

- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts` imports `runWgpuPackageGenerator` and remains the `generate-wgpu`, `preview-generated`, and `check-wgpu` router.
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts` imports `runNestedCargoPackageAdapter` for its generate, preview and check routes.
- root `📜️script.ts` imports `runNestedCargoPackageAdapter` for the root taxonomy route and `runDependencyVerification` for `verify dependencies`; the moved implementation definitions and their former discovery imports were removed.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts` imports the frame-worker builder for generate, preview, check and browser-worker composition.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts` imports `renderFrameWorker` directly from its semantic builder owner.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-js-lock-parity/🟦️.ts` imports lock parity from inventory.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-truth/🟦️.ts` imports inventory and truth APIs from their owners and obtains the repository coordinate from `🗂️workspaces/🟦️.ts`.

The `wgpu-frame-worker.inputPatterns` authority is byte-sorted and now includes the OS command route, WGPU projection/publication, repository discovery, normalization, and every source reached by the current browser facade. The final compiler closure added the scene owner and removed the no-longer-reached mesh facade from both exact browser lists. The obsolete root `📜️script.ts` implementation input was removed. Browser-only `sourceModulePaths` remain separate from native build/publication code.

The stale Trunk cache fixture no longer assumes a Node project manifest beside the Rust `Trunk.toml`. `⚡️caching/🧫️fixtures/🔒️trunk-lockfile/🔣️.json` explicitly pairs the Rust configuration with `wgpu/📦️packages/🟦️typescript/📋️project.json`, and the test uses that path for the exact Nx project/cache authority while retaining the Cargo-lock hash assertions.

The existing launch routes remain the executable authority for WGPU and JCO generation/checks. This slice adds one portable route, `🧹clean🧩️taxonomy🧪️root-artifact-dependency-source`, to both `.vscode/🧩️launch.seed.jsonc` and generated `.vscode/launch.json`; it invokes `bun nx run @semio-tech/repo-lib:test-root-artifact-dependency-source`.

## Semantic Contexts

The portable fixture binds these exact contextual directory kinds:

| Directory | Parent context | Kind |
| --- | --- | --- |
| `📽️projection` | `wgpu-target` | `wgpu-artifact-projection` |
| `📦️publication` | `wgpu-target` | `wgpu-artifact-publication` |
| `🏗️builder` | `frame-worker` | existing `builder` |
| `📽️projection` | `members-of-modules` | `repo-library-projection` |
| `🧩️package-adapter` | `repo-library-projection` | `repo-package-adapter-projection` |
| `📦️publication` | `repo-package-adapter-projection` | `repo-package-adapter-publication` |
| `📇️inventory` | `members-of-members-of-modules` | `repo-dependency-inventory` |
| `⚖️truth` | `members-of-members-of-modules` | `repo-dependency-truth` |

Strict taxonomy loading passed with 456 semantic kinds and 23 generator contracts. Scoped inventories found no violation on any new owner or ancestor. The wider WGPU scope contained four other unresolved directories (`⏱️turn-budget`, `⚙️browser-build`, `🗄️wasm-module-cache`, and `🫀️boot-liveness`) plus one package-implementation warning on the Rust `🌐️.html`. The wider dependency scope contained five other unresolved directories in existing test/TypeScript trees plus two opaque-reference warnings in the new inventory source. The warnings are textual dependency coordinates, not ownership failures. This report makes no baseline claim for the ambient paths.

## Portable and Native Oracles

The new schema, fixture and test prove:

- all six owners use anonymous `🟦️.ts` leaves;
- every declared API exists and every live consumer imports the owner directly;
- the root script no longer defines the moved WGPU/dependency implementation;
- all eight contextual owner relationships resolve through the shared classifier;
- exact six-output membership and exact generator implementation inputs;
- dependency Go parsing matches native `go mod edit -json`, including a local replacement;
- the real OS package dispatcher accepts an empty bounded `package-projected-inputs-v1` document and emits the same six paths;
- the same dispatcher returns nonzero promptly with no stdout when a uniquely owned cancellation marker exists;
- Bun/Nx/editor registration is unique.

The first portable run was red with four failures because the six semantic owners and their contexts did not exist. After extraction, the first dispatch case exceeded Bun test's implicit five-second case budget; the test now declares a 15-second bound matching its two native child processes. A concurrent direct and registered run then exposed a shared cancellation-marker collision: one test removed the other's marker, making cancellation falsely succeed. Each run now creates and removes a unique directory under the repository test cache. The final direct and registered runs both pass.

The first WGPU freshness execution also failed with `resolve is not defined` from the newly extracted resolver callback. Adding the omitted `node:path` import fixed the real compiler path before any output was accepted.

## Output Identity

Pre-extraction executable-source hashes, retained only as report provenance:

| Source | SHA-256 before |
| --- | --- |
| root `📜️script.ts` | `72f7456959c974d0c51ecfc8d7ce92ba430445510b8f395d4120fd3c62549a9b` |
| OS TypeScript `📜️script.ts` | `527345d31848f50f4961a27f13bf1769bb7a29be34b06160db33be1b292c5c52` |
| OS Rust `📜️script.ts` | `248abbd59b6724af623aca53ba9701bc6ec8a356b8289693767d15d77076eca2` |
| WGPU TypeScript `📜️script.ts` | `e638cd987882500950565cf75249e23238ef06d166691488041252b0c821412d` |

Anonymous-owner source hashes observed after final validation:

| Owner | SHA-256 |
| --- | --- |
| WGPU projection | `13239d9d43f37552f3948d53fdc39571032db37de97c7fbd748682535aaa106b` |
| WGPU publication | `5d5c4efdddd9fad356d1e792fc622fe88a86819e726876e91b14d211a26af16b` |
| frame-worker builder | `2bdf91b5d57bd0423185189359d9c1c804ae292595eefd184f7ae68d3e476731` |
| JCO package-adapter publication | `212ea085cbc15fc80d9f202dd20b8752dc794c2fc78b47eed5c62556a6845dfc` |
| dependency inventory | `079694b8a00fbed09a006c4f3f30bf4293f5cb27c32061cc5bc872e8a9ef2b08` |
| dependency truth | `2a246bff6951731d5af37a33c162fcb0fb2b04304b93731790709bdf7d1a01cb` |

No permanent source-hash assertion was added.

The six WGPU outputs observed after final validation are:

| Output | Bytes | SHA-256 |
| --- | ---: | --- |
| `🎞️frame-worker/🤖️generated/🟨️.js` | 1,136,255 | `f5eef572671ed03778617e83b2f8879eda663fd33fee37d2c9c17b4394f3cb8d` |
| `📦️packages/🟦️typescript/📚️library/🟦️.ts` | 99 | `e8c2fe73f1ee90e6aa8aac43db6e652d9176abb0569d6daeaba528c01eff5f15` |
| `📦️packages/🦀️rust/🏗️builder/🦀️.rs` | 93 | `165411752c27f19ed2eb6a9cab94c539e8aea9e02532e60bee8b4394d6cfb64c` |
| `📦️packages/🦀️rust/💾️binary/🦀️.rs` | 109 | `ffe0e71f9f3b655dbe2772fd3f198c0d429f0bb296abe7755afd82d2682d2050` |
| `🚀️browser-boot/🤖️generated/🟨️.js` | 58,304 | `1f83f41ca45b32e497afe7d776f25db8875b876ed4fb222ded6246e0e518aed2` |
| `🧊️renderer/📇️registry/🦀️.rs` | 234 | `69acfd40343d94a43c13e0e02d393cebf487e72cc71b44b839f97e63d02b3391` |

An intermediate pre-facade generation refreshed browser boot from `b297d3c5de590c7430aeab9c057460c54be967cab7beb2a7ed191b53deaf457a`. The final scene/mesh authority reconciliation then reported `6 exact artifacts; 2 changed` for frame worker and browser boot, followed by direct and registered `6 exact artifacts; 0 changed`. The rendered projection admitted 112 exact inputs. These hashes record the observed final outputs; no source-byte stability assertion was added.

The JCO preview emitted one 132-byte mode-0644 leaf at `🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/📚️library/🦀️.rs`, SHA-256 `e3173e78ce5ec51c9d0226f5a87fe99c29206f9247f50bee119e1a6786bb2a55`, with no stale removals.

## Validation

| Command | Result |
| --- | --- |
| direct portable Bun test | 5 passed, 0 failed, 84 assertions in 3.72 seconds; includes Ajv, TypeScript compiler, native Go, real projected dispatcher and cancellation |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/repo-lib:test-root-artifact-dependency-source` | passed through the registered route; same 5 tests and 84 assertions |
| `bun ./📜️script.ts test-preview-generated` in the WGPU TypeScript package | 1 file, 20 tests passed in 5.15 seconds on the settled source authority; includes two compiler implementations, Ajv/emoji identities, deterministic SHA-256 and cross-CWD rendering |
| `bun ./📜️script.ts check-frame-worker` in the WGPU package | passed; frame-worker bytes fresh |
| direct OS `generate-wgpu`, then `check-wgpu` | passed; one stale output refreshed, then six exact outputs and zero changes |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/framework-os:check-wgpu` | passed; six exact outputs, zero changes |
| direct OS Rust `check-jco-package-adapter` | passed |
| `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/framework-os-kernel:check-jco-package-adapter` | passed through the registered route |
| native `cargo check --locked --offline` for `semio-jcoprobe-guest` with ticket-local target | passed in 23.83 seconds |
| direct `testTrunkLockfile(process.cwd())` | passed; Bun/smol-toml parse agreement, current project/cache authority and lock-hash semantics |
| root script import plus imports of all six owners | passed |
| `bun ./📜️script.ts verify dependencies self-test` | passed; 18 hostile mutations |
| `bun ./📜️script.ts verify dependencies summary --format json` | passed; raw 196, third party 194, first party 2, literal external 193, production reachable 96 |
| strict taxonomy load | passed; 456 kinds, 23 generator contracts |
| three scoped taxonomy inventories | no findings on the new owner/context paths; ambient findings recorded above |

The originally reported `@semio-tech/repo-lib:test-cache-closure-contracts` target is not present in the live Nx project (`Cannot find configuration for task`). The affected cache assertion was therefore rerun directly through its exported native test function after rebasing the fixture. The existing broad `repo:test` cache suite was not repeated because its body had already been independently reported green and the changed assertion has no separate live target.

## Current Dependency Limits

`verify dependencies parity js` completed its 27.32-second scan and reported 131 manifests, 269 external rows, 134 evidenced rows, 135 unowned rows, 1,162 undeclared imports, 89 lock workspaces, zero lock mismatches, and five lock fixtures. It then failed on the undeclared-import count as designed. The first findings were undeclared `ajv`, `esbuild`, `@napi-rs/canvas`, and `pdfjs-dist` imports in `♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📜️script.ts`.

The default ratchet reported baseline 175 at commit `7ad363fd1ec91cb0c83cf716bc66522be99a4785`, current 194, and 19 new identities: JS `@iarna/toml`, `ajv-formats`, `d3-force`, `emoji-regex`, `fast-json-patch`, `graphology`, and `jsonschema`; Python `anastruct`, `honeybee-energy`, `honeybee-openstudio`, `jsonpatch`, `jsonschema`, `ladybug-core`, `networkx`, `openstudio`, `PyNiteFEA`, `scikit-fem`, and `shapely`; Rust `roxmltree`. The baseline was not changed.

The truth summary reports 15 oracle conflicts and two toolchain-owner conflicts. The root Bun/Nx audit itself has no failure and its three authorized rows are lock-owned. No dependency exception or baseline entry changed in this slice.

## Remaining WGPU Package Script Responsibilities

The WGPU TypeScript package command is not yet a routing-only file. The following implementations remain explicitly queued for a later source lane:

- native environment/cache ownership: `assertRendererCacheHome`, `nativeRunnerEnvironmentKeyIsProtected`, `nativeRunnerEnvironment`, `runNativeBinary`, `proveNativeRunnerEnvironment`;
- Trunk/tool acquisition and process control: `trunkEnv`, `runInteractiveCommand`, `ensureWasmTarget`, `ensureTrunk`;
- artifact staging/serving and selection: `syncStableRendererArtifacts`, `assetServerBaseUrl`, `variantAssetSpecs`, `ensureAssetServer`, `resolveNativeAppArgs`;
- native/browser task composition: `TrunkBuildScript`, `TrunkServeScript`, `scaleModeArgValue`, `scaleModePassthroughArgs`, `nativeBinaryPath`, `NativeBuildScript`, `NativeRunScript`;
- embedded source/native oracles and lint scan: `directoryRetainedHomeBootstrapOracle`, `normalizedPresenceRowsOracle`, their four check command classes, and `collectWgpuColorLiteralViolations`.

The package's `TestScript`, `NativeTestScript`, browser test/check classes and final `ScriptRouter` are task glue. This extraction does not claim the whole WGPU command file is thin.

## Exact Files

New source and portable-contract files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📽️projection/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️publication/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🏗️builder/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📽️projection/🧩️package-adapter/📦️publication/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/📇️inventory/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/⚖️truth/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️root-artifact-dependency-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️root-artifact-dependency-source/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️root-artifact-dependency-source/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-root-artifact-dependency-extraction-2026-09-12.md`

Updated source, consumer, authority and route files:

- `📜️script.ts`
- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-js-lock-parity/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-truth/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/🔒️trunk-lockfile/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔒️trunk-lockfile/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🤖️generated/🟨️.js`

Only this lane's ticket-local Nx data, dependency JSON output, preview JSON and Cargo target are disposable. They are removed after this report retains the evidence above.
