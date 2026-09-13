# Sol Vitest Configuration Ownership

## Fixed owner map

The exact 43-source map places each anonymous configuration below its enclosing semantic root test concern. The behavior-root column remains the package/module root selected by the pre-move configuration and is preserved explicitly in the moved owner.

| Previous source | Semantic owner | Preserved test root |
| --- | --- | --- |
| `vitest.config.ts` | `🧪️tests/🎚️config/🟦️.ts` | `.` |
| `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/vitest.config.ts` | `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🎚️config/🟦️.ts` | `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript` |
| `♻️mit-bestand/🧺️demonstrator/vitest.config.ts` | `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎚️config/🟦️.ts` | `♻️mit-bestand/🧺️demonstrator` |
| `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/vitest.config.ts` | `✏️s/🔌️plugins/🌊️flow/🧪️tests/🎚️config/🟦️.ts` | `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript` |
| `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/vitest.config.ts` | `✏️s/🔌️plugins/🎞️animate/🧪️tests/🎚️config/🟦️.ts` | `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript` |
| `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/vitest.config.ts` | `✏️s/🔌️plugins/🏗️fem/🧪️tests/🎚️config/🟦️.ts` | `✏️s/🔌️plugins/🏗️fem` |
| `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/vitest.config.ts` | `✏️s/🔌️plugins/📐️cad/🧪️tests/🎚️config/🟦️.ts` | `✏️s/🔌️plugins/📐️cad` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/vitest.config.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🧪️tests/🎚️config/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/vitest.config.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🧪️tests/🎚️config/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/vitest.config.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🧪️tests/🎚️config/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/vitest.config.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🧪️tests/🎚️config/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy` |
| `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/vitest.config.ts` | `✏️s/🔌️plugins/📸️remodel/🧪️tests/🎚️config/🟦️.ts` | `✏️s/🔌️plugins/📸️remodel` |
| `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/vitest.config.ts` | `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/🧪️tests/🎚️config/🟦️.ts` | `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript` |
| `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/vitest.config.ts` | `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/🧪️tests/🎚️config/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/vitest.config.ts` | `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🎚️config/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle` |
| `🌎️hub/📦️packages/🟦️typescript/vitest.config.ts` | `🌎️hub/🧪️tests/🎚️config/🟦️.ts` | `🌎️hub/📦️packages/🟦️typescript` |
| `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/vitest.config.ts` | `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🎚️config/🟦️.ts` | `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript` |
| `🧰️framework/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/📦️packages/🟦️typescript` |
| `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🔨️modules/⏳️async/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript` |
| `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🔨️modules/◻️2d/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript` |
| `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🔨️modules/🎠️kernel` |
| `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🔨️modules/🎭️actor/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript` |
| `🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🔨️modules/📡️replication/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🔨️modules/📡️replication` |
| `🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🔨️modules/🔄️machine/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/vitest.config.ts` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling` |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript` |
| `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🔨️modules/🧊️3d/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript` |
| `🧰️framework/🔨️modules/🧬️schema/vitest.config.ts` | `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🔨️modules/🧬️schema` |
| `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/vitest.config.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/vitest.config.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/vitest.config.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript` |
| `🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🛍️products/🖥️server/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🟦️typescript` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/📦️packages/🟦️typescript` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/vitest.config.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️tests/🎚️config/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript` |

## Evidence

- Before move: installed Vite 7.3.6's default bundled loader under Bun loaded 43/43 current exports in 21.721 seconds. This executed configuration exports and imports; it did not run test cases.
- First red: `bun test .../🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts -t 'requires anonymous semantic owners'` found the first absent semantic owner: 0 pass, 1 fail, 3 assertions. The red was captured before any source move.
- Bundled-loader control: the owner test loaded 43/43 semantic configuration exports through installed Vite 7.3.6's default bundled loader, preserved every exact behavior root, name and portable behavior projection hash, and completed as 1/1 with 215 assertions in 22.167 seconds. An API-argument audit found that the intended `"native"` value occupied Vite's `customLogger` parameter, so this run is deliberately labelled bundled rather than native.
- Corrected native-loader control: after passing `undefined` for `customLogger` and `"native"` as the sixth argument, the focused installed Vite 7.3.6 run passed 1/1 with 215 assertions in 95.32 seconds. This is configuration evaluation, not execution of the 43 test suites.
- Direct package route before the loader-mode correction: `bun ./📜️script.ts test vitest-configuration-ownership` in the repo-library TypeScript package passed 7/7 with 450 assertions in 62.59 seconds. Its configuration phase used the bundled loader; path, selector, schema and registration assertions remain valid.
- Final corrected registered route: `NX_DAEMON=false NX_ISOLATE_PLUGINS=false`, ticket-private `NX_WORKSPACE_DATA_DIRECTORY`, `NX_CACHE_DIRECTORY`, `TMPDIR`, and `bun nx run @semio-tech/repo-lib:test-vitest-configuration-ownership --skip-nx-cache` passed 7/7 with 450 assertions. The corrected test body took 83.19 seconds, Nx took 1 minute 24 seconds, and Nx reported cache skipped. Its installed-loader case used the true native loader.
- Current post-proof owner boundary: the exact-revert confirmation passed 1/1 with 172 assertions in 3.42 seconds. The independent Bun/TypeScript config-argument tokenizer control then passed 9/9 with 17 assertions in 4.54 seconds.
- Focused package-boundary classification passed 2/2 with 8 assertions in 2.08 seconds. Framework/OS source topology passed 10/10 with 253 assertions in 789 milliseconds. All 381 current `📋️project.json` documents parsed after the exact input additions.

## Resulting contract

- The 43 fixed-name files are absent. The exact table above is the complete old-to-owner map; each executable configuration is an anonymous `🟦️.ts` leaf below the enclosing semantic test concern and no owner is below `📦️packages`.
- `configurationRoot` remains explicit behavior data. Installed Vite resolves both `root` and `test.root` to that preserved package/module root, so relocating ownership did not silently change discovery scope.
- `runVitest` now requires an explicit third configuration argument. The portable gate parses all current product callers and the root router with TypeScript, resolves `this.root`, `join(this.root, ...)`, and named roots, and requires the resulting repository-relative path to equal one of the 43 manifest owners.
- Every owning package/project `namedInputs.default` includes its exact semantic owner. The root project includes the root owner and the schema owner. The registered ownership target additionally names the router, settings, launch projections, all relevant routers/project manifests, every owner, taxonomy/discovery/normalization, schema, fixture and test.
- VS Code now uses `vitest.configSearchPatternInclude = "**/🧪️tests/🎚️config/🟦️.ts"`; the prior fixed-name key is absent.
- The fixed `vitest-config` filename contract and its TypeScript/package-source binding are absent. The configurable `vitest-config-entry` disposition remains the semantic admission used by current owner leaves.
- The root no-tests configuration remains deliberate: `include: []`, `passWithNoTests: false`, and no `projects` property. The last condition is verified from the TypeScript syntax tree, so accurate documentation containing the word `projects` cannot trigger a false failure.

## Registration

- Owner test: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts`.
- Portable schema and data: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🎚️vitest-configuration-ownership/🔣️.json` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json`.
- Package router: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts test vitest-configuration-ownership`.
- Nx target and package selector: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`, target `test-vitest-configuration-ownership`.
- Launch seed and derived projection: `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`, exact name `🧹clean🧩️taxonomy🎚️vitest-configuration-ownership`.

## Exact supporting-file attribution

- The 43 removed files and 43 created owners are enumerated exactly in the fixed owner map above.
- The 43 input-bearing project manifests are listed below. They consist of the 42 projects that execute or own one of the moved configurations plus root `📋️project.json`.
- Runtime selection changed in root `📜️script.ts`, each package/module `📜️script.ts` that calls `runVitest`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts`, and `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts`.
- Shared selection and exact-token coverage changed in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏃️run-vitest-config-argument-tokens/🟦️.ts`, and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts`.
- Taxonomy and admission changed in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`, and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`.
- Source-as-data paths changed in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧰️framework-root-source-topology/🔣️.json` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖥️os-source-topology/🔣️.json`.
- Editor and launch consumers changed in `.vscode/settings.json`, `.vscode/🧩️launch.seed.jsonc`, and `.vscode/launch.json`.

## Exact input-bearing project manifests

- `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📋️project.json`
- `♻️mit-bestand/🧺️demonstrator/📋️project.json`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/📋️project.json`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📋️project.json`
- `🌎️hub/📦️packages/🟦️typescript/📋️project.json`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📋️project.json`
- `📋️project.json`
- `🧰️framework/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📋️project.json`

## Exact runVitest selectors and token consumers

- `♻️mit-bestand/🧺️demonstrator/📜️script.ts`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts`
- `🌎️hub/📦️packages/🟦️typescript/📜️script.ts`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📜️script.ts`
- `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🧪️command-routing/🟦️.ts`
- `📜️script.ts`
- `🧰️framework/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏃️run-vitest-config-argument-tokens/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts`

## Nested-Cargo boundary

A short speculative edit attempted to remove the virtual WGPU nested-Cargo fixed-name projection from discovery. It was reverted with a scoped patch before this report because that projection is not one of the 43 live configuration files. Current `🔍️discovery/🟦️.ts` again contains all four original elements: the `tool-config-vitest` union member, WGPU's required `vitest-config-entry`, the virtual `const configPath = "vitest.config.ts"` exact-authority check, and classification of both `vitest-configuration` and `tool-config-vitest`. Current `🧹️normalization/🟦️.ts` also retains the `tool-config-vitest` union member. The validator-owner maps intentionally bind only `vitest-configuration` to `vitest-config-entry`; no new fixed-filename exemption was restored.

The exploratory nested-Cargo collision command ran only while that speculative edit existed and returned 24 pass/2 fail because its full-current-package diagnostic stopped at another prerequisite and its launch catalog was incomplete. It is non-acceptance evidence and is not attributed to the Vitest ownership extraction.

## Limits and non-acceptance attempts

- A first composite rerun observed a concurrent HTML taxonomy contract failure before the owned assertions, and a later attempt hit the package's 120-second budget after the installed-loader phase took 66.941 seconds under workspace contention. Root subsequently reported HTML taxonomy green. Neither attempt is acceptance evidence.
- The first complete direct suite exposed two owned test defects: the product AST scan exceeded its default 30-second test budget and a textual `projects:` ban matched correct documentation. The scan now has an explicit 60-second case budget and the no-projects rule uses parsed object properties. The final direct and Nx results above include both repairs.
- The installed-loader controls execute configuration imports and compare normalized configuration data. They do not run all 43 test suites, drive the VS Code extension UI, or establish Windows/Linux native-loader execution.
- Existing selected native/application outcomes outside configuration ownership, including the eight previously repaired selector/consumer paths and Remodel's recorded semantic-parity failure, were preserved and not rerun.
- The WGPU nested-Cargo fixture describes a virtual fixed-name input inside a generated package. It is distinct from the live 43-file population and remains a follow-up for the nested-package projection owner.
