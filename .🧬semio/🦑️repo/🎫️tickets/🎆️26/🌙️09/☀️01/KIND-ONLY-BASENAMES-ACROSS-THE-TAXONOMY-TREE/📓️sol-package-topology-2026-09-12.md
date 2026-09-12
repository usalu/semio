# Package Topology and Non-UI Source Repair

Date: 2026-09-12

## Result

The UI React, OS renderer React, and puzzle 5D React workspaces now use target-first package roots. Their authored implementations live at the semantic target owner and their package entry leaves only re-export those sources. The OS renderer WGPU plugin bridge is now an anonymous TypeScript leaf under its registered semantic owner. The OS services and flow WASM Rust sources now use anonymous leaves under domain directories.

A final filesystem scan found zero directories matching `*/📦️packages/*/🎯️targets*`. The removed package/source paths do not exist. `bun install --frozen-lockfile` refreshed the three workspace links without changing dependency resolution, and root `node_modules` resolves each package to its target-first package root.

## Exact Source and Package Moves

### UI React

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` -> `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/⚛️runtime.ts` -> `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎠️runtime/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🖌️render.ts` -> `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🖌️render/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🏗️build-tooling.ts` -> `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts`
- The package files `LICENSE.md`, `README.md`, `eslint.config.ts`, `package.json`, `postcss.config.ts`, `tailwind.config.ts`, `tsconfig.json`, `vitest.config.ts`, `📋️project.json`, and `📜️script.ts` moved from `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/` to `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/`.
- Added thin package leaves `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx` and `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.ts`.
- The package-export test remains outside the package at `🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️react-package-export/🟦️.ts`; empty package-internal `🧪️tests/🧪️package-export` directories were removed.

### OS Renderer React

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` -> `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx`
- The package files `package.json`, `tsconfig.json`, `vitest.config.ts`, `📋️project.json`, and `📜️script.ts` moved from `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/` to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/`.
- Added thin package leaf `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx`.

### Puzzle 5D React

- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🟦️.tsx` -> `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/🟦️.tsx`
- The package files `package.json`, `vitest.config.ts`, `📋️project.json`, and `📜️script.ts` moved from `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/` to `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/`.
- Added thin package leaf `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/🟦️.tsx`.

### OS Renderer WGPU

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts` -> `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`
- Removed obsolete tracked generated bundle `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js`; its producer now writes `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🤖️generated/🟨️.js`.
- Removed obsolete tracked generated bundle `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🚀️boot.js`; its producer now writes `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🤖️generated/🟨️.js`.
- `.gitignore` now admits the frame-worker generated directory and exact leaf because the taxonomy declares that output tracked; browser boot remains ignored as declared.
- The removed browser bundle and regenerated canonical browser bundle are byte-identical: SHA-256 `21fe604d7bf0ecbb048600589c7587e2d05e1653a14c048505f17f9ee937b9d3`.
- The regenerated frame bundle is `1,163,796` bytes with SHA-256 `b84fd044dc13bff32c27cb39e72ec196d162a48bc9fa2518650196f94cd58da0`. The prior tracked bundle was SHA-256 `57118266589b93e2571a473398e31fd8a2e7fa629b13f8dd92bca3acf47a6a44`; its semantic differences include concurrently added segmented-download handling plus the plugin source-banner path change.

### OS Services and Flow WASM

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs` -> `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/👀️file_watcher.rs` -> `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/👀️file-watcher/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🚪️native_io.rs` -> `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🚪️native-io/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs` -> `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol.rs` -> `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📡️protocol/🦀️.rs`

## Exact Consumer and Configuration Files

The following live files were changed for package-root, semantic-source, generated-output, import, manifest, workspace, or test-path closure. Several are shared files with unrelated concurrent edits; this list identifies only this lane's touched files.

```text
.dependency-cruiser.cjs
.gitignore
.storybook/main.ts
.storybook/scopes.ts
.storybook/stories/block/2d/Board.stories.tsx
.storybook/stories/block/2d/Fixtures.stories.tsx
.storybook/stories/block/3d/World.stories.tsx
.storybook/stories/block/5d/Board.stories.tsx
.storybook/stories/block/5d/World.stories.tsx
.storybook/stories/block/scene.ts
.storybook/stories/fem/2d/Model.stories.tsx
.storybook/stories/fem/2d/Results.stories.tsx
.storybook/stories/fem/2d/Viewer.stories.tsx
.storybook/stories/fem/3d/Model.stories.tsx
.storybook/stories/fem/3d/Results.stories.tsx
.storybook/stories/fem/3d/Viewer.stories.tsx
.storybook/stories/fem/scene.ts
.storybook/stories/puzzle/2d/Board.stories.tsx
.storybook/stories/puzzle/2d/Fixtures.stories.tsx
.storybook/stories/puzzle/3d/World.stories.tsx
.storybook/stories/remodel/Frames.stories.tsx
.storybook/stories/remodel/Model.stories.tsx
.storybook/stories/remodel/Modes.stories.tsx
.storybook/stories/remodel/Panels.stories.tsx
.storybook/stories/remodel/Report.stories.tsx
.storybook/stories/remodel/Viewer.stories.tsx
.storybook/stories/remodel/scene.ts
.storybook/🧪️tests/🧭️scope-resolution/🟦️.ts
bun.lock
package.json
📜️script.ts
🔒️dependencies.json
🧅️layering.json
♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/vitest.config.ts
♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/⚙️vite.config.ts
♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts
♻️mit-bestand/🧺️demonstrator/⚛️footer.tsx
✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/vitest.config.ts
✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts
🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/tsconfig.json
🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/vitest.config.ts
🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/⚙️vite.config.ts
🌎️hub/🔨️modules/🛡️admin/🧫️fixtures/🕸️build-graph/🎨️stylesheet-graph.json
🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts
🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧪️playgroundflowwasmdevstubplugin/🟦️.ts
🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️react-package-export/🟦️.ts
🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🔣️.json
🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🔢️scalar/🧬️schema/🔣️.json
🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/⭕️Ring/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎬️Scene/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🐚️ShellScope/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🧪️tests/🧩️component/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔝️Navbar/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖼️Panel/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪜️Stepper/🟦️.tsx
🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx
🧰️framework/🛍️products/💻️os/⚡️effect-backbone.ts
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/vitest.config.ts
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/vitest.config.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🔬️component-domain-laws/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⚙️browser-build/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Trunk.toml
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🌐️.html
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📋️project.json
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📥️wgpu-intake-budget/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️wgpu-module-routes/laws.json
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🩺️window-fault/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧊️wgpu-browser-boot-cache-inputs/🔣️.json
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📥️imports/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/📋️project.json
🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🎛️config/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts
🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/👁️watch-policy.json
🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🟦️.ts
🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml
```

The three target package roots and every moved source/config file named above are also changed files; the retained report itself is `📓️sol-package-topology-2026-09-12.md`.

## Taxonomy and Authority Files

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` adds `members-of-react-target` (`🎠️runtime`, `🖌️render`, `🛠️build-tooling`) and admits `👀️file-watcher` and `🚪️native-io` under module members. Its WGPU generator paths now point at the semantic plugin/frame/browser owners and its exact path arrays are unique and UTF-8 byte ordered.

The UI package relocation changed three README/license/package evidence paths. The reviewed authority chain was updated exactly in these files:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/⚖️readme-license-owner-authority/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/🗺️testing-readme-coordinates/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🟢️readme-current-source-activation/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔖️readme-current-source-revision/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🎯️reviewed-expectations/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔖️readme-current-source-revision/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/👀️readme-reviewed-fixture-inputs/🔣️.json
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json
```

Exact resulting digests:

- owner authority catalog: `b60f16acbde34545d29abfeb50a55d5db6115b2d316b33011db07da34ca7afe9`
- testing coordinates/reviewed expectations: `e821d6835b7bc0054ae7d0d3c4e7907692263b7ad947c3d7230a4c848c730320`
- reviewed manifest: `081ff80b401f1b6d3c5c94a70a0c3875e0987b03289145dd0ae60fb6c983f1ab`
- canonical revision digest: `425a28ccf110b81e10d40e387b25e3e8d1d1cb07b15eade2bb5448178cc059fd`
- current revision fixture: `5eb7fd808e24a1d95e065afd8f37a5990f30cc80b15e7e13ff59e629ccef44fa`

The historical nested-Cargo projection catalogs were not rewritten; they remain evidence for their earlier projection rather than live runtime consumers.

## Verification

Passing checks and exact output:

- `bun install --frozen-lockfile`: exit 0, Bun 1.3.14, three packages installed.
- Direct `loadTaxonomy()`: exit 0, `[DEBUG] taxonomy loaded 391`.
- Target-first filesystem scan: zero inverse `*/📦️packages/*/🎯️targets*` directories.
- Removed-path existence scan: zero stale physical paths.
- Root workspace links:
  - `@semio-tech/ui-react -> ../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript`
  - `@semio-tech/framework-renderer-react -> ../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript`
  - `@semio-tech/puzzle-5d-react -> ../../✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript`
- Isolated Nx multi-project quick run: renderer React and puzzle React targets passed. UI React reached the relocated project but failed its broader current suite as described below.
- UI React package-export focused test: one file passed, one test passed.
- Renderer React direct test: one file passed, one test passed, four skipped.
- Puzzle React direct test: one file passed, one test passed.
- Services `native cargo metadata --locked`: exit 0, dependency lock validated.
- Flow `native cargo metadata --locked`: exit 0, dependency lock validated.
- Browser boot generation plus independent freshness check: exit 0; regenerated bytes match the producer and the deleted tracked artifact.
- Frame-worker producer preview: exit 0; the declared `1,163,796` bytes exactly match the new generated leaf (`fresh=True`).

Observed broader failures, without claiming they were present before this lane:

- UI React full quick suite: 14 of 22 files passed and 130 of 155 tests passed; 25 tests currently fail, predominantly because the test i18n double lacks `i18n.exists`.
- Services Nx quick test remained silent while its native task ran for more than two minutes and was cancelled with exit 130 to keep this lane bounded. Both moved services paths were still accepted by locked Cargo metadata.
- Current-source revision authority suite: 9 passed, 2 failed on an undeclared `semanticOwnedSourceBasename` compiler helper and a missing launch-seed row.
- Reviewed fixture suite: 3 passed, 6 failed on current discovery/launch/process-isolation behavior.
- Current-source activation suite accepted its digest/schema provenance checks, then current planner assertions failed and the 15-second suite budget terminated the run.
- Testing README coordinates failed its current path-safety input before executing tests.

One WGPU failure is proven unchanged by the old tracked bytes: `generate-frame-worker` writes the new bundle, then its carrier census rejects `localStorage`. The same `globalThis.localStorage` sequence exists in the deleted HEAD bundle. The producer preview independently proves the new output is byte fresh; resolving the carrier policy or shard diagnostics behavior is outside this topology relocation.

## Target Package Discovery Audit Closure

The relocated puzzle target package now declares the exact catalog marker `semio.role = "plugin"` and `semio.id = "puzzle-5d-react"` in:

```text
✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/package.json
```

Catalog target assignment also required the existing `⚛️5d-react` semantic target name to participate in the global target registry. The following file now registers it as a TypeScript React target with the `typescript-library-entry` and `typescript-react-entry` contracts:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json
```

The catalog-backed regression in the following file discovers all three moved target packages and checks each exact id, role, owner, target, package root, and manifest path:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts
```

The empty disallowed directory `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/🧪️tests` was removed. It contained no files, so this removal has no tracked file path.

Final checks:

- Registered focused test: `bun ./📜️script.ts test quick --timeout 20000 -t 'discovers every moved target-first React package with its exact owner and package root'`; exit 0, `1 pass`, `582 filtered out`, `0 fail`, `3 expect() calls`.
- Direct strict-catalog proof called `discoverPackages(process.cwd(), loadCatalogTaxonomy())`; exit 0. It returned exactly `puzzle-5d-react` as role `plugin`, owner `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react`, target `⚛️5d-react`; `renderer-react` as role `framework`, owner `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react`, target `⚛️react`; and `ui-react` as role `framework`, owner `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react`, target `⚛️react`. Every row reported its exact owner-relative `📦️packages/🟦️typescript` package root and `package.json` manifest path.
- Directory absence proof: `test ! -d '✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/📦️packages/🟦️typescript/🧪️tests'`; exit 0.

The audit's broader config-body heuristics and the pre-existing WGPU nested-language adapter finding were intentionally left for their separately owned lanes.
