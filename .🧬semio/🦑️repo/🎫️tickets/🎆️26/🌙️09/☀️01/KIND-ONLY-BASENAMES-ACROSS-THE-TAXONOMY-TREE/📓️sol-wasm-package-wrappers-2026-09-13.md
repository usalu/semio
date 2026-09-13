# Sol Stable Wasm Package Wrapper Report

## Result

Actor and Puzzle now have stable authored Node package manifests at their existing Rust Nx project roots. Each manifest keeps the package identity emitted by wasm-bindgen and explicitly maps its root JavaScript module, TypeScript declaration, core WebAssembly module and WebAssembly declaration from the generated `pkg` payload. A fresh checkout can discover the stable outer package before `pkg` exists; after the producer materializes `pkg`, the generated child manifest is recognized as the payload of the outer package rather than a second workspace member.

Actor preserves both package and Nx project identity as `@semio-tech/framework-actor-rs`. Puzzle preserves package identity `@semio-tech/puzzle-wasm` while its existing Nx project identity remains `@semio-tech/puzzle-plugin`. Neither wasm producer, target name nor generated companion basename changed.

## Authored package owners

Created:

- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/package.json`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/package.json`

Each stable manifest is private ESM metadata with an empty Nx included-script list. Its `main`, `module`, `types`, `files` and `exports` fields point into the existing `pkg` producer output. The root and explicit JavaScript subpath pair the module with its declaration. The explicit WebAssembly subpath pairs the `.wasm` file with its `.wasm.d.ts` declaration.

Existing compiler payloads were read but not changed:

- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/pkg/package.json`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/pkg/framework_actor.js`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/pkg/framework_actor.d.ts`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/pkg/framework_actor_bg.wasm`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/pkg/framework_actor_bg.wasm.d.ts`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/package.json`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.js`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle.d.ts`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle_bg.wasm`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/pkg/semio_puzzle_bg.wasm.d.ts`

Existing producer metadata was read and retained:

- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📋️project.json`: project `@semio-tech/framework-actor-rs`, target `wasm`, command `bun ./📜️script.ts wasm`, output `{projectRoot}/pkg`.
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📋️project.json`: project `@semio-tech/puzzle-plugin`, target `wasm`, command `bun ./📜️script.ts wasm`, output `{projectRoot}/pkg`.

## Schema-first portable contract

Created:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️wasm-package-wrappers/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️wasm-package-wrappers/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️wasm-package-wrappers/🟦️.ts`

The language-agnostic fixture binds two stable manifests, two generated payload manifests, eight compiler companions, both existing Nx producers and one exact 25-input acceptance target. Ajv 2020 validates the fixture and hostile schema versions/extra fields/incomplete owner sets. The portable membership oracle exercises `computeWorkspaces` in private trees and against the current repository. It requires each outer owner exactly once and suppresses each generated child identity only through the explicit same-name export edge.

The runtime oracle copies only the two manifests and eight companions into a ticket-private installed-package tree. Bun 1.3.14, Node 24.15.0 `import.meta.resolve`, and TypeScript `NodeNext` resolution agree on the root, explicit JavaScript and explicit WebAssembly targets. It also checks both native WebAssembly magic/version headers and each generated JavaScript module's exact `new URL(<wasm>, import.meta.url)` coordinate. It does not execute the wasm module.

The test requires `SEMIO_TEST_ARTIFACT_DIR`, allocates a unique `run-*` child, removes only that child and preserves the caller-provided root. The repo-library router supplies the existing `repoTestArtifactEnvironment` when the caller omits the variable.

## Registration and cache closure

Updated:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`

The exact route is `test-wasm-package-wrappers`. Its cached Nx target reads `wasmPackageWrapperSources`, whose 25 entries equal the fixture: shared globals; two outer manifests; two payload manifests; eight compiler companions; two existing producer projects; workspace discovery; the test-output environment owner; schema, fixture and test; repo-library router/project/package; and seed/derived launch catalogs. Package and both launch catalogs each select the target once. The launch renderer regenerated the derived catalog after the seed edit.

Root workspace membership publication is intentionally not part of this slice. The live read-only oracle proves the owners are discoverable; the coordinator owns the subsequent root `package.json` membership write.

## First reds and repairs

The first schema-first run was `3 pass / 3 fail / 18 assertions`. The schema, compiler bytes and private membership model passed. It failed exactly because:

- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/package.json` did not exist;
- installed resolution could not read that absent manifest;
- `wasmPackageWrapperSources` and `test-wasm-package-wrappers` were not registered.

After materialization, a current-repository membership scan exceeded Bun's implicit 5-second per-case limit after 7.10 seconds while its assertions were still running. The deliberate repository-wide read-only case now has a 30-second case bound; the next runs completed in 9.17 and 7.37 seconds. The package-level budget remains 45 seconds.

An auxiliary unisolated `nx show project` formatting probe produced a JSON parse diagnostic for Actor before a separate Puzzle query returned its project. It is not acceptance evidence. The cache-skipped registered target below is the graph evidence for the final source.

## Final evidence

Direct ordinary package route with `SEMIO_TEST_ARTIFACT_DIR` unset:

`bun ./📜️script.ts test wasm-package-wrappers`

- exit 0;
- 8 tests, 59 assertions, 0 failures;
- Bun test 9.85 seconds, wall 9.97 seconds;
- current repository membership scan 9.17 seconds;
- caller artifact root preserved and owned child removed.

Final isolated registered route with `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false`, ticket-private Nx workspace/cache/temp/artifact roots and `--skip-nx-cache`:

`bun nx run @semio-tech/repo-lib:test-wasm-package-wrappers --skip-nx-cache`

- exit 0;
- 8 tests, 59 assertions, 0 failures;
- Bun test 8.04 seconds;
- Nx target 8.3 seconds, critical path 8.2 seconds, cache skipped;
- wall approximately 30.2 seconds including workspace graph setup.

The actual launch renderer command was `bun ./📜️script.ts generate` from the plugin registry package. It exited 0 in 9.29 seconds and reported 59 plugin crates, 61 playgrounds and 50 framework packages.

## Limits

- No wasm, Rust, application or package build ran.
- No package install or root workspace membership write ran.
- Resolver checks use the existing prepared generated companions in a private installed-package layout; they do not claim runtime initialization of either wasm module.
- Generated child manifests and companions remain compiler output. Their current presence supplied native identity evidence, while stable outer manifests supply fresh-checkout workspace identity.
- The root package membership count remains a coordinator-owned follow-up.

## Adjacent Flow wording correction

The Flow ownership test debug line and `📓️sol-flow-browser-ownership-2026-09-13.md` were corrected to describe staged publication success and pre-promotion missing-companion preservation. They no longer claim a multi-path atomic transaction. The report explicitly retains the post-promotion manifest-removal/rollback limit.
