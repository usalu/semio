# TypeScript and JavaScript Test Layout Final Report

The TypeScript/JavaScript lane is complete. The repository-wide layout scanner subsequently reported zero JavaScript/TypeScript legacy suffix, inline-body, or delivery-scope findings. The durable runner inventory is `📓️test-layout-runner-map-2026-09-08.md`: it contains the raw 518-leaf classification plus the independently verified concurrently added caching/Wasm leaf, for 519 current leaves. Its current extension breakdown is 446 `.ts`, 63 `.tsx`, 10 `.js`, and 0 `.mjs`. The inventory scans current direct implementation leaves matching `🧪️tests/<case>/{🟦️.ts,🟦️.tsx,🟨️.js,🟨️.mjs}` across the repository and excludes dependencies, VCS metadata, generated `📤️dist`, ticket scratch/generated data, schemas, fixtures, and non-implementation files. Categories overlap: native suite, guarded production registration, exported self-test invoked by a dispatcher/another case, and feature adapter. No current leaf is unreferenced.

## Execution evidence

- Workspace contract focused run: 1 test, 6 assertions passed. The explicit `NX_ISOLATE_PLUGINS=true` preservation case passed through the public `bun nx exec` surface with 1 assertion.
- Browser actor runtime completed through public Bun/Nx before the later fixture-directory taxonomy rename: exit 0 with AJV, JCO, Wasm/JSPI, emitted actor, pack/stream, and close-law evidence. The later `🧪️fixtures` to `🧫️fixtures` path-only change was repaired in 12 actual path arguments; the 69-pair literal audit found no expected/template/ordinary-string rebase.
- Group visibility, UI browser host, WebGPU surface, and retained sequence actions executed through public Bun/Nx/private minimal workspace routing. Retained actions passed after the stale route count was corrected from 17 to 16.
- Parent final scanner proof covered 29,224 real-tree sources and reported zero JavaScript/TypeScript layout findings. Parent integration proof subsequently passed 45 tests and 1,443 assertions.

## Old/new mappings

| Old/source path | New canonical path(s) |
|---|---|
| `🗿️artifact.ts` | `🧪️tests/🗿️artifact-runner/🟦️.ts` |
| `🧰️framework/🔨️modules/🖱️ui/🧪️story.ts` | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-types/🟦️.ts` |
| `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/🧪️vitest.config.ts` | `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/vitest.config.ts` |
| `♻️mit-bestand/🧺️demonstrator/⚡️vitest.config.ts` | `♻️mit-bestand/🧺️demonstrator/vitest.config.ts` |
| `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/vitest.config.ts` |
| `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/vitest.config.ts` |
| `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/vitest.config.ts` |
| `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/vitest.config.ts` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/vitest.config.ts` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/vitest.config.ts` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/vitest.config.ts` |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/vitest.config.ts` |
| `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/vitest.config.ts` |
| `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/vitest.config.ts` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/vitest.config.ts` |
| `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/vitest.config.ts` |
| `🌎️hub/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `🌎️hub/📦️packages/🟦️typescript/vitest.config.ts` |
| `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/vitest.config.ts` |
| `🧪️tests/🟦️.ts` | `vitest.config.ts` |
| `🧰️framework/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `🧰️framework/📦️packages/🟦️typescript/vitest.config.ts` |
| `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/vitest.config.ts` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️tests/🟦️.ts` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/vitest.config.ts` |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts` | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/↔️Resizable/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/↔️Resizable/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚙️VirtualFileSystem/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚙️VirtualFileSystem/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⭕️Ring/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⭕️Ring/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎀️Ribbon/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎀️Ribbon/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎬️Scene/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎬️Scene/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️IconSelector/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️IconSelector/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📜️Scrollable/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📜️Scrollable/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📝️Field/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📝️Field/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔘️Button/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔘️Button/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔣️Icons/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔣️Icons/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔳️ButtonGroup/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔳️ButtonGroup/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕰️HistoryTable/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕰️HistoryTable/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🦴️Skeletons/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🦴️Skeletons/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪜️Stepper/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪜️Stepper/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️.story.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️tests/📚️storybook/🟦️.tsx` |
| `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/vitest.config.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/vitest.config.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/vitest.config.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/vitest.config.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/vitest.config.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/vitest.config.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/vitest.config.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/vitest.config.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/vitest.config.ts` |
| `🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/vitest.config.ts` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🧪️tests/🟦️.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/vitest.config.ts` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧪️tests/🖼️surface/🟨️.js` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🖼️webgpu-surface/🟨️.js` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/📨️browser-frame-transport/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🧩️package-integration/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌐️wasi-activation/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧹️fixture-sweep/🟦️.ts` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` |
| `✏️s/🔌️plugins/🗄️stdio/📜️script.ts` | `✏️s/🔌️plugins/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔎️scalar-witness/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️tests/🔎️scalar-witness/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/👁️group-visibility/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/📦️native-codec-send/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔗️backbone-detach/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌱️initial/🪪️identity/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🪪️initial-child-identity/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🗄️durable-owned-group/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🗣️member-dialect/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧵️canonical-edit/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🪪️runtime/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🪪️runtime-identity/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🏃️runner-self-tests/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🔣️codec/🧵️send/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔣️codec-caller-source/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📜️script.ts` | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧹️fixture-sweep/🟦️.ts` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` |
| `temp/merge/mit-bestand/präsentation/33.projektetage/js/index.ts` | `temp/merge/mit-bestand/präsentation/33.projektetage/🧪️tests/📽️deck/🟦️.ts` |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` |
| `.storybook/playwright.config.ts` | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-uncovered-components/🟦️.ts`<br>`🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-new-stories/🟦️.ts` |
| `.storybook/stories/ui/🎯Engagement.stories.tsx` | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎯️engagement-story-interaction/🟦️.ts` |

## Exact authored path manifest

This deduplicated array covers the lane’s removed paths, new canonical paths, retained extraction sources/dispatchers, runner/config/launch callers, fixture/count repairs, and durable reports. Paths owned and repaired separately by the parent are recorded in the parent manifest.

```json
[
  ".storybook/playwright.config.ts",
  ".storybook/stories/ui/🎯Engagement.stories.tsx",
  ".storybook/vitest.setup.ts",
  ".vscode/launch.json",
  "temp/merge/mit-bestand/präsentation/33.projektetage/js/index.ts",
  "temp/merge/mit-bestand/präsentation/33.projektetage/js/🧪️tests/📽️deck/🟦️.ts",
  "temp/merge/mit-bestand/präsentation/33.projektetage/🧪️tests/📽️deck/🟦️.ts",
  "vitest.config.ts",
  "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/vitest.config.ts",
  "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/🧪️vitest.config.ts",
  "♻️mit-bestand/🧺️demonstrator/vitest.config.ts",
  "♻️mit-bestand/🧺️demonstrator/⚡️vitest.config.ts",
  "✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/vitest.config.ts",
  "✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/vitest.config.ts",
  "✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🎬️sequence/📦️packages/🟦️typescript/📜️script.ts",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/📌️retained-actions/🟨️.js",
  "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧬️schema/🔣️.json",
  "✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/vitest.config.ts",
  "✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/vitest.config.ts",
  "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/vitest.config.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/vitest.config.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/vitest.config.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/vitest.config.ts",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/vitest.config.ts",
  "✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/vitest.config.ts",
  "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🗄️stdio/📜️script.ts",
  "✏️s/🔌️plugins/🗄️stdio/🧪️tests/📦️artifact-package-graph/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/vitest.config.ts",
  "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/vitest.config.ts",
  "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🌎️hub/📦️packages/🟦️typescript/vitest.config.ts",
  "🌎️hub/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/vitest.config.ts",
  "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "📓️test-layout-runner-map-2026-09-08.md",
  "📓️test-layout-typescript-audit-2026-09-08.md",
  "📓️test-layout-typescript-final-2026-09-08.md",
  "📓️test-layout-vitest-discovery-audit-2026-09-08.md",
  "🗿️artifact.ts",
  "🧪️tests/🗿️artifact-runner/🟦️.ts",
  "🧪️tests/🟦️.ts",
  "🧰️framework/📦️packages/🟦️typescript/vitest.config.ts",
  "🧰️framework/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/vitest.config.ts",
  "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/vitest.config.ts",
  "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
  "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧪️tests/🖼️surface/🟨️.js",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🖼️webgpu-surface/🟨️.js",
  "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧪️story.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎯️engagement-story-interaction/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-new-stories/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-types/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📚️storybook-uncovered-components/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/↔️Resizable/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/↔️Resizable/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚙️VirtualFileSystem/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚙️VirtualFileSystem/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⭕️Ring/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/⭕️Ring/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎀️Ribbon/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎀️Ribbon/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎬️Scene/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎬️Scene/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️IconSelector/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️IconSelector/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📜️Scrollable/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📜️Scrollable/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📝️Field/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📝️Field/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔘️Button/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔘️Button/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔣️Icons/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔣️Icons/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔳️ButtonGroup/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔳️ButtonGroup/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕰️HistoryTable/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕰️HistoryTable/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🦴️Skeletons/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🦴️Skeletons/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪜️Stepper/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪜️Stepper/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️.story.tsx",
  "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🧪️tests/📚️storybook/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/vitest.config.ts",
  "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json",
  "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/vitest.config.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/vitest.config.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/vitest.config.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔎️scalar-witness/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️tests/🔎️scalar-witness/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌱️initial/🪪️identity/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/👁️group-visibility/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/📦️native-codec-send/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔗️backbone-detach/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🗄️durable-owned-group/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🗣️member-dialect/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧵️canonical-edit/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🪪️initial-child-identity/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🪪️runtime/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🪪️runtime-identity/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/📨️browser-frame-transport/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🧩️package-integration/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/vitest.config.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌐️wasi-activation/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌊️actor-import/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️tests/🌐️wasi-activation/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/vitest.config.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/vitest.config.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🔣️codec/🧵️send/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌐️browser-bundle/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🏃️runner-self-tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔣️codec-caller-source/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/vitest.config.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/vitest.config.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🧹️fixture-sweep/🟦️.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📜️script.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/vitest.config.ts",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/vitest.config.ts",
  "🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️index/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/vitest.config.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🧪️tests/🟦️.ts"
]
```

## Integrity limits

The 69-pair AST literal audit compared moved sources with pre-goal base `6152f9ca6a0fbb55aa61992077837a230996b51d`: seven pairs contained relative-literal changes, all in module/path contexts, and zero were expectation/template/ordinary-string changes. The subsequent 12 plugin fixture-taxonomy repairs changed only filesystem anchor arguments. No optional full actor rerun was started after that path-only repair because its prior run took 9m35s; repository-level scanning and the focused route proofs above provide the closeout evidence.

## Coordinator Cardinality Review

The retained-actions fixture currently contains exactly 16 distinct routes, all classified migrated. Its separate disposition guard requires 16 migrated and zero pending. The corrected uniqueness guard therefore checks the same 16-route contract; all individual route/source, publication, hostile-schema, and third-party oracle checks remain present. No fixture route was removed for this correction.

Final destination correction: the temporary merge presentation case is at `temp/merge/mit-bestand/präsentation/33.projektetage/🧪️tests/📽️deck/🟦️.ts`. Its current `js/index.ts` guarded caller imports `../🧪️tests/📽️deck/🟦️.ts`. The earlier `js/🧪️tests` destination was an intermediate path; both historical and final authored paths are retained in the array. No source edit was required for this metadata correction.
