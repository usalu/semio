# Plugin Registry Input Authority Audit

## Scope and result

This was a bounded, read-only audit requested after the clean CAD/Draw source-scope plans each demanded the existing `plugin-registry` regeneration. No input grammar was changed.

The current input grammar is simultaneously very broad for plugin trees and incomplete as a literal record of every file read by the shared discovery implementation. It must not be narrowed by deleting ignored/cache paths without a separate schema-owned dependency contract.

| Observation | Count |
| --- | ---: |
| CAD plan declared input nodes | 107,695 |
| Draw plan declared input nodes | 107,715 |
| Ignored Git candidate files under plugin prefix | 31,203 |
| Ignored Git candidate files under framework prefix | 28,931 |
| Instrumented preview observed filesystem paths | 35537 |
| Instrumented preview content reads | 805 |
| Content reads matched by current input patterns | 410 |
| Content reads not matched by current input patterns | 395 |
| Plugin-tree content reads | 353 |
| Ignored content reads | 10 |
| Actual Compose access attempts | 0 |
| Write attempts | 0 |

The retained read-path census is `🧪️cad-draw-scoped-consumers/📇️registry-read-audit/🔣️.json`. Its content path list is gzip/base64 JSON, with no file contents.

## Method and limitations

The actual `PreviewGeneratedScript.run` implementation was loaded from its existing `📜️script.ts` with a process-local Bun loader hook appending only an export of that private class. Process-local `bun:test` module interception recorded Node synchronous filesystem reads/metadata operations. All write methods were replaced by throwing guards. Exact actual Compose prefixes were denied before filesystem dispatch. No local helper script or modified production file was created.

The preview’s ordinary in-memory stdout was captured, not written. It rendered 419,902 bytes with SHA-256 `e3225404de2d5597a474d7fac654afc9dfa65ab2db91c19dd7f235a4059d4add`. Two instrumented runs produced the same preview hash while concurrent discovery additions changed the raw read count from 799 to 805. This supports stability of that observed output, not a universal proof that all extra reads are irrelevant.

This trace covers explicit synchronous Node filesystem calls in the loaded implementation and its imports, not native module-loader reads or every possible future branch. Read counts are therefore observations, not a proposed permanent allowlist. Metadata operations include 34161 distinct directory reads, 1069 existence checks, 278 stats, and 22 lstats.

## Actual data flow

The registry’s `renderCatalogFiles` calls `generatePluginRegistry`, `generatePlaygroundRegistry`, `generateFrameworkPackageRegistry`, and `resolveDefaultHostVariant`; launch bytes additionally read `.vscode/🧩️launch.seed.jsonc`.

- Plugin rows consume discovered component-role Rust package manifests and each package owner’s optional exact `🔣️descriptor.json`.
- Playground rows consume Cargo metadata, artifact/surface directory membership, and existence of the schema-named example leaf. They do not read every asset/model byte.
- Framework rows consume shared discovered package identity, owner, language, target, area, and maturity. Discovery spans all declared ecosystems/areas, not just framework Cargo manifests.
- Shared `scanRepo` also performs `collectPackageRoles`, reading source-format package leaves to classify diagnostics. Those reads are incidental to this shared scan; the preview does not consume `discoverPackageProblems`. Their direct output influence must be separated from diagnostic execution before a narrow contract can be claimed.
- Owner maturity derives from residual implementation-directory and owner entry-file presence. A correct narrow contract therefore needs membership/existence authority, not only a list of content hashes.

Relevant existing code:

- Registry `📜️script.ts`: `readDescriptorJson`, `parsePluginCargo`, `discoverExamplesForPlayground`, `generateFrameworkPackageRegistry`, `renderCatalogFiles`, `PreviewGeneratedScript`.
- Discovery `🔍️discovery/🟦️component.ts`: `scanRepo`, `collectPackageRoles`, `scanPackagesDir`, `DISCOVERY_SKIP_DIRS`.
- Registry `🖥️launch.ts`: `readSeed`.

## Ignored content actually read

No ignored plugin-tree file was read for content in the retained preview trace. The ten ignored content reads were all elsewhere:

- `🧰️framework/📦️packages/🦀️rust/target-root-framework-schema/CACHEDIR.TAG`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/🎨️styling/🤖️generated.py`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/target-root-ui-contract-check/CACHEDIR.TAG`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/target-root-ui-contract-native/CACHEDIR.TAG`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/target-root-ui-contract-wasm/CACHEDIR.TAG`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️14/SCOPED-COMMANDS-AND-WINDOW-LOCAL-ACTIONS/🎯️target/CACHEDIR.TAG`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🟦️next-env.d.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🔷️dotnet/obj/Debug/net8.0/.NETCoreApp,Version=v8.0.AssemblyAttributes.cs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🔷️dotnet/obj/Debug/net8.0/Semio.Repo.Test.AssemblyInfo.cs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🔷️dotnet/obj/Debug/net8.0/Semio.Repo.Test.GlobalUsings.g.cs`

These comprise cache marker reads that stop package traversal, a generated styling Python source, a generated Next type declaration, and three .NET `obj` sources read by package diagnostics. This demonstrates why blanket “all ignored inputs are irrelevant” would be false.

The shared discovery walker explicitly skips exact `node_modules`, `target`, `dist`, `📤️dist`, `.git`, `.🧬semio`, `🤖️generated`, `🔌️plugin-modules`, `pkg`, `storybook-static`, `temp`, `.venv`, `coverage`, `__pycache__`, `client`, and `client_bin` directories. Cargo cache directories with nonstandard names are pruned only after reading their `CACHEDIR.TAG`. The current broad plugin `/**` input grammar binds many nodes from these skipped subtrees anyway.

## Authority needed before narrowing

A clean long-term closure should declare the registry’s exact discovery membership/presence facts separately from content inputs, and separate catalog discovery from diagnostics-only package source classification. It must retain optional descriptor existence, all ecosystems that contribute framework rows, example-leaf presence, the launch seed, and generator implementation/schema dependencies.

The 395 unmatched observed content reads are evidence that the current contract is not a complete read-set contract. Some are diagnostics-only and may not influence generated bytes; proving that distinction is required before either adding them wholesale or omitting them. This audit does not authorize a narrower grammar, skip any existing ledger input, or claim production apply readiness beyond the clean planned reference graph.
