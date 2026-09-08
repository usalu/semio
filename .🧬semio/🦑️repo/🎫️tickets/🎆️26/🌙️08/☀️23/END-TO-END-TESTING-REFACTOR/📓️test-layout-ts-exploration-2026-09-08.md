# JavaScript and TypeScript Test Layout Exploration

## Scope and method

This is a read-only initial inventory for the test-layout refactor. It used `rg` over tracked workspace paths, excluding `.git` and `node_modules`.

The required layout is:

```text
<semantic parent>/🧪️tests/<test-name>/<implementation>
```

The existing implementation filenames are language markers such as `🟦️.ts`, `🟦️.tsx`, and `🟨️.js`; preserve those names below the direct semantic case directory. A direct `🧪️tests/🟦️.ts` file has no `<test-name>` directory and is therefore not a test-layout success. A `*.test.*` or `*.spec.*` filename is legacy regardless of whether it is already below `🧪️tests`.

Initial counts:

- 361 JavaScript/TypeScript paths matched a test/spec basename or a `🧪️tests` path.
- 284 JavaScript/TypeScript files are under `🧪️tests`.
- 50 JavaScript/TypeScript files use a `*.test.*` or `*.spec.*` basename.
- 70 executable source files guard suites with `if (import.meta.vitest)`; every one is a test location outside the new test-tree convention unless the tests are extracted.

## Main migration partitions

| Partition | Scope | Primary hazard |
| --- | --- | --- |
| Renderer React | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react` and sibling renderer elements | Package config chooses a test-level-dependent `include`, and `includeSource` points outside the package root. |
| Framework and OS inline suites | `🧰️framework/**` and `🧰️framework/🛍️products/💻️os/**` | `import.meta.vitest` suites currently reside in production sources; moving them requires replacing `includeSource` and preserving the selected quick/fundamental/long/exhaustive levels. |
| `s` plugin artifact examples | `✏️s/🔌️plugins/**/🗿️artifacts/**/📚️examples/**` | Package `📜️script.ts` files pass explicit example test paths to `bun test`; every path changes when its test case directory is added. |
| MCP, repo client, registry, and test infrastructure | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp`, `🧰️framework/🛍️products/🦑️repo/**`, plugin registry | Vitest config files have explicit `include` arrays of legacy filenames; repo test infrastructure itself hardcodes legacy paths. |
| Historical demonstrator | `♻️mit-bestand/🧺️demonstrator` | Playwright matches `*.acceptance.spec.ts`; the spec must move and `testMatch` must stop encoding the legacy suffix. |

## Explicit legacy-file partition

The following groups are immediately safe to assign to file-moving agents, subject to their runner/config updates.

- Renderer React: `⚡️quick.test.ts`, `🏛️space-administration.test.tsx`, `👥️scoped-presence.test.tsx`, `📇️directory-home-bootstrap.test.tsx`, `🔬️artifact-creation-ready-opening.test.ts`, `🔬️document-opening.test.ts`, `🔬️index.test.ts`, `🚪️opening.test.ts`, `🧯️router-plugin-faults.test.ts`, `🩺️window-fault.test.ts` beneath `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/`.
- Renderer element cases: `🧱️elements/🔗️AgentBridge/🧪️component.test.ts`, `🚦️AgentPresence/🧪️component.test.tsx`, `🛠️ShellHelpers/🧪️component.test.ts`, `🤖️AgentApprovals/🧪️component.test.tsx`, `🧭️TiledMapHost/🧪️component.test.ts`, and `🧵️TaskManager/🧪️component.test.tsx` beneath `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`.
- OS MCP: `🌅️modern-era.test.ts`, `🏛️legacy-conformance.test.ts`, `💡️inference-bridge.test.ts`, `🔄️end-to-end.test.ts`, `🔐️authenticated-hub-workspace.test.ts`, and `🧹️hygiene.test.ts` beneath `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/`.
- Plugin registry: `✅️catalog-complete.test.ts`, `📖️generated-projection.test.ts`, `🚀️launch.test.ts`, and `🪪️plugin-identity.test.ts` beneath `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/`.
- Other framework/repo cases: `🧰️framework/🔨️modules/🧬️schema/✅️draft07-oracle.test.ts`; `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧹️config.test.ts`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🧪️extension.test.ts`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/📦️packages/🟦️typescript/🔬️schema.test.ts`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🔬️schema.test.ts`; and `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🔬️server-persistence.test.ts`.
- JavaScript/WASM cases: `✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/🧪️tests/🌐️sequence-browser-consumer.test.js`, `🔮️sequence-protocol-oracle.test.js`, and the sequence/flow WASM `🧪️tests/🧪️*.test.*` files; these need a JS implementation leaf `🟨️.js` under a named test case rather than a filename suffix.
- Historical demonstrator: `♻️mit-bestand/🧺️demonstrator/🧪️demonstrator.acceptance.spec.ts`.

## Executable inline Vitest sources

The following exact paths contain a real `if (import.meta.vitest)` guard. They are source-embedded executable tests, not mere references to Vitest. This is the principal extraction inventory.

```text
♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📦️index.ts
♻️mit-bestand/🧺️demonstrator/📜️script.ts
♻️mit-bestand/🧺️demonstrator/🪧️brand.ts
✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️presentation/⚡️implementations/🟦️typescript/🟦️.ts
✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️.tsx
✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️.ts
✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/🔌️pdf-canvas-port/🟦️.ts
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️actions/🟦️.ts
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🟦️.ts
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️.ts
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🟦️.ts
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🟦️.ts
✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/🟦️.ts
✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/🟦️.ts
✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/🟦️.ts
✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/🟦️.ts
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🟦️.ts
✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🟦️.tsx
✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️.ts
✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🗺️spatial/🟦️.ts
✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts
✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧱️brepjs/🟦️.ts
🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🟦️.tsx
🧰️framework/📦️packages/🟦️typescript/🟦️.ts
🧰️framework/🔨️modules/◻️2d/🟦️.ts
🧰️framework/🔨️modules/🎠️kernel/🟦️.ts
🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts
🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts
🧰️framework/🔨️modules/🎭️actor/📃️page/🟦️.ts
🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts
🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🟦️.ts
🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts
🧰️framework/🔨️modules/🎭️actor/📬️mailbox/🟦️.ts
🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts
🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts
🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts
🧰️framework/🔨️modules/🎭️actor/🪪️activation/🚪️instance/📥️output/🟦️.ts
🧰️framework/🔨️modules/📡️replication/🟦️.ts
🧰️framework/🔨️modules/🔄️machine/🟦️.ts
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts
🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts
🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts
🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts
🧰️framework/🔨️modules/🛂️manifest/🟦️.ts
🧰️framework/🔨️modules/🧊️3d/🟦️.ts
🧰️framework/🛍️products/💻️os/🟦️.ts
🧰️framework/🛍️products/💻️os/⚡️effect-backbone.ts
🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/🟦️.tsx
🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️store.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📥️poll/🏘️composition/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🌳️tree/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📃️document/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/📊️table/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🖼️image/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧊️mesh/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts
```

## Discovery and runner wiring

1. [`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`](../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts) defines `runVitest` at line 2522. Its default config argument is the layout violation `🧪️tests/🟦️.ts`, and it always invokes Vitest with `run --config <config>` at lines 2519–2525. Moving a config or test file must update its owning package `📜️script.ts` call; do not rely on Vitest's default `*.test.*` discovery.
2. Seven dedicated Vitest config files were found: `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/🧪️vitest.config.ts`, `♻️mit-bestand/🧺️demonstrator/⚡️vitest.config.ts`, `🧰️framework/🔨️modules/◻️2d/📦️packages/🟦️typescript/vitest.config.ts`, `🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/vitest.config.ts`, `🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/vitest.config.ts`, `🧰️framework/🔨️modules/🔄️machine/📦️packages/🟦️typescript/vitest.config.ts`, and `🧰️framework/🔨️modules/🧊️3d/📦️packages/🟦️typescript/vitest.config.ts`. Five of them rely on `includeSource`; the `2d`, `machine`, and `3d` configs point at their parent `🟦️.ts`, while kernel and replication use source globs.
3. The renderer React config is itself `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts`. It conditionally includes `⚡️quick.test.ts`, `🧱️elements/🔗️AgentBridge/🧪️component.test.ts`, or outside-root inline source suites. It is the first config execution agent must update after moves.
4. The MCP config is `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts`; its explicit `include` array is the six root `*.test.ts` files listed above, and `includeSource` points to `../../🟦️.ts`.
5. Playwright configuration is `♻️mit-bestand/🧺️demonstrator/🎭️playwright.config.ts`, whose `testDir` is the demonstrator root and whose `testMatch` is `*.acceptance.spec.ts`. The new semantic path must become its test directory/match contract.
6. Artifact example packages invoke `bun test` with literal test paths in their `📜️script.ts` files. Confirmed callers include flow, writer, sequence, lowpoly, process, layout, architect, shooting, note, sourcing, draw, mathematical, and trinity. Those calls will not follow moved files automatically.
7. `nx.json` defines the production named input at line 20. Its exclusion glob embeds `spec|test|e2e|integration`; this is legacy discovery metadata and must be replaced with an exclusion for the semantic `🧪️tests` tree so test sources never enter production inputs.
8. The root `package.json` routes `test` to `bun nx run workspace:test`. The repository has no `project.json` files (`rg --files -g project.json` returned zero), so test target discovery is inferred from scripts/Nx plugin configuration. No `launch.json` file was found with `rg --files -g launch.json`; test launch registration needs its actual workspace configuration location found before the final completion audit.

## Migration rules for execution agents

1. Move a test to `<owner>/🧪️tests/<semantic-case>/🟦️.ts` or `🟦️.tsx`/`🟨️.js`; rewrite imports relative to the new case leaf.
2. Extract `import.meta.vitest` code from its production source into an owner-local semantic case. Production files must lose the test guard and Vitest-only imports.
3. Replace every explicit legacy filename in `include`, `testMatch`, `runVitest`, `bun test`, documentation used as test input, and validation assertions in the same logical change.
4. Preserve test-level selection in package configs. A generic recursive include can unintentionally add long/exhaustive process suites to quick and fundamental runs.
5. Do not move runner configurations as if they were executable tests. They must still resolve from their owning `📜️script.ts`; their direct `🧪️tests/🟦️.ts` paths are a separate configuration-layout issue to resolve while retaining `vitest --config` semantics.

## Validation status

No tests were run: this report is read-only exploration. Counts and wiring statements above were validated from repository source with `rg` and `sed` on 2026-09-08.
