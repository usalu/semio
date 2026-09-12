# Framework Source Ownership Batch — 2026-09-12

Workspace root: `/Users/ueli/Documents/semio`. Every path below is repository-relative to that root, so the exact absolute identity is the workspace root joined with the displayed path.

## Result

The batch moves named implementation leaves into semantic owners and leaves anonymous language-kind leaves behind. Package entries are glue, Cargo manifests bind domain crate roots, and corresponding Rust, TypeScript, JavaScript, JSON and CSS implementations remain adjacent when they implement the same concern. Generated token and palette outputs were not reassigned.

The production Storybook preview now treats `import.meta.vitest` as `undefined`, so Rollup erases inline native test imports before it follows them into OS build and repository tooling. Native Vitest routes keep providing their own `import.meta.vitest` definition and remain functional. The styling runtime facade contains only theme exports; filesystem, workspace, playground and Vite orchestration live under `🎨️styling/🏗️builder/🌐️vite/🟦️.ts`. The iframe header helper is browser-pure under `🎨️styling/🌐️iframe/🟦️.ts`.

A pure TypeScript UI limit companion at `🧬️contract/🛡️limits/🟦️.ts` gives React and WGPU the same constants without importing `UiDocumentStore` and React into the WGPU browser source graph.

## Portable contract and TDD

The schema, fixture and test at `🧬️schema/🧱️framework-source-topology/🔣️.json`, `🧫️fixtures/🧱️framework-source-topology/🔣️.json` and `🧪️tests/🧱️framework-source-topology/🟦️.ts` define the source move identities, package-body projections, Cargo bindings, render test owners, contextual semantic directories, browser-pure entries and production inline-test elision. AJV validates the portable contract, `@iarna/toml` independently parses the native bindings, and Bun's native browser compiler validates the browser leaves.

Red proof initially reported every old source still present and every semantic leaf absent. Final direct proof: 4 tests passed, 0 failed, 253 expectations. The registered Nx route `@semio-tech/repo-lib:test-framework-source-topology` and launch entry `🧹clean🧩️taxonomy🧪️framework-source-topology` are wired through the existing repository dispatcher.

Pre-move SHA-256 evidence captured 56 inputs. Of the 52 one-to-one moved/test-owner identities, 40 had a direct preimage in the live workspace snapshot: 29 remained byte-identical and 11 changed only for path/reference repair. Twelve split contract bodies did not have a standalone old-file preimage; their original package-body hash was retained in the snapshot before extraction.

## Source move map

| Previous source | Semantic implementation | Kind |
|---|---|---|
| `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts` | `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts` | typescript |
| `🧰️framework/🔨️modules/🖱️ui/🌐️globals-ui.css` | `🧰️framework/🔨️modules/🖱️ui/🌐️globals/🎨️.css` | css |
| `🧰️framework/🔨️modules/🖱️ui/🌓️theme.css` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🎨️.css` | css |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui.css` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css` | css |
| `🧰️framework/🔨️modules/🖱️ui/🧵️.css` | `🧰️framework/🔨️modules/🖱️ui/🧵️styles/🎨️.css` | css |
| `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🪆️slot.tsx` | `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🪆️slot/🟦️.tsx` | typescript-jsx |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/📡️interactive-jobs.ts` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/📡️interactive-jobs/🟦️.ts` | typescript |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/📐️layout.ts` | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/📐️layout/🟦️.ts` | typescript |
| `🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🌐️delivery.ts` | `🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🌐️delivery/🟦️.ts` | typescript |
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📡️event.rs` | `🧰️framework/🔨️modules/🖱️ui/🖥️host/📡️event/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📥️enqueue.rs` | `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️enqueue/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🔌️backend_alias.rs` | `🧰️framework/🔨️modules/🖱️ui/🖥️host/🔌️backend/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🪟️window.rs` | `🧰️framework/🔨️modules/🖱️ui/🖥️host/🪟️window/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🧬️schema/⚛️component.rs` | `🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator.rs` | `🧰️framework/🔨️modules/🧬️schema/✅️validator/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/♿️accessibility.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎨️style.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎨️style/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/👥️presence.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/👥️presence/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📃️document.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📐️layout.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📐️layout/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🗺️surface.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🗺️surface/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🛡️limits.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🛡️limits/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🧩️component.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🪢️text_edit.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🪢️text-edit/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/📃️document.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/📃️document/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧱️component.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/🧱️component/🦀️.rs` | rust |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌐️favicon.json` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌐️favicon/🔣️.json` | json |

## Package body extraction map

| Package entry | Semantic body | Glue limit |
|---|---|---|
| `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts` | `🧰️framework/🔨️modules/🎭️actor/🧵️turn-scheduler/🟦️.ts` | ≤ 3 lines |
| `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🟦️.ts` | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🟦️.ts` | ≤ 3 lines |
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🟨️javascript/🟨️.js` | `🧰️framework/🔨️modules/🖱️ui/🖥️host/🪟️window/🟨️.js` | ≤ 3 lines |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts` | ≤ 3 lines |

Additional split identities:

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts` is the browser facade; native build/dev orchestration is `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`, and iframe policy is `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌐️iframe/🟦️.ts`.
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🛡️limits/🟦️.ts` is the browser-pure TypeScript limit companion to `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🛡️limits/🦀️.rs`.

## Native crate bindings

| Manifest | Domain crate root | Substantive semantic body |
|---|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🔨️modules/🖱️ui/🖥️host/🦀️.rs` via `../../🦀️.rs` | `🧰️framework/🔨️modules/🖱️ui/🖥️host/🪟️window/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🦀️.rs` via `../../🦀️.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🦀️.rs` via `../../🦀️.rs` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌗️mixing/🦀️.rs` |
| `🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🔨️modules/🔄️machine/✨️derive/🦀️.rs` via `../../🦀️.rs` | `🧰️framework/🔨️modules/🔄️machine/✨️derive/⚙️expansion/🦀️.rs` |
| `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs` via `../../🦀️.rs` | `🧰️framework/🔨️modules/🌱️value/✨️derive/⚙️expansion/🦀️.rs` |
| `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/Cargo.toml` | `🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️.rs` via `../../🦀️.rs` | `🧰️framework/🔨️modules/🧬️schema/✨️derive/⚙️expansion/🦀️.rs` |

The exact crate-root relocations are:

| Previous package root | Domain crate root |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️.rs` | `🧰️framework/🔨️modules/🖱️ui/🖥️host/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️.rs` | `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🦀️.rs` | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🦀️.rs` |
| `🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust/🦀️.rs` | `🧰️framework/🔨️modules/🔄️machine/✨️derive/🦀️.rs` |
| `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/🦀️.rs` | `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs` |
| `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/🦀️.rs` | `🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️.rs` |

The three derive crates keep macro package identities and public exports while their substantive expansion bodies live under the same `⚙️expansion` concern. Host, contract and styling crates likewise retain native package identities with domain-owned roots.

## Render test-owner map

| Implementation-encoded test owner | Semantic test owner |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-packages-rust-backend-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-backend-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-packages-rust-frame-buffers-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-frame-buffers-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-packages-rust-pipelines-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-pipelines-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-packages-rust-backend-drawable-size` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-backend-drawable-size` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-packages-rust-backend-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-backend-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-packages-rust-frame-buffers-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-frame-buffers-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-packages-rust-objective-c-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-objective-c-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-packages-rust-descriptor-layout-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-descriptor-layout-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-packages-rust-memory-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-memory-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-packages-rust-resources-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-resources-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-packages-rust-surface-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-surface-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-packages-rust-swapchain-support-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-swapchain-support-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-packages-rust-vk-error-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-vk-error-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-backend-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-backend-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-buffers-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-buffers-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-frame-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-frame-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-gpu-context-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-gpu-context-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-gpu-types-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-gpu-types-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-gpu-uniforms-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-gpu-uniforms-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-resources-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-resources-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-scene-target-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-scene-target-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-surface-adapter-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-surface-adapter-unit` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-surface-state-unit` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-surface-state-unit` |

The render `Scene::finish` support changed from test-only visibility to `cfg(any(test, feature = "backend-testing"))`, matching the native backend feature that exercises it. Metal all-features evidence passed 10 tests.

## Contextual taxonomy members

| Directory | Parent kind | Exact contextual kind |
|---|---|---|
| `📡️interactive-jobs` | `members-of-elements` | `ui-ports-interactive-jobs` |
| `📐️layout` | `members-of-elements` | `ui-diagram-layout` |
| `♿️accessibility` | `schema` | `ui-contract-accessibility` |
| `📐️layout` | `schema` | `ui-contract-layout` |
| `🧩️component` | `schema` | `ui-contract-component` |
| `🪢️text-edit` | `schema` | `ui-contract-text-edit` |
| `🛡️limits` | `schema` | `ui-contract-limits` |
| `📃️document` | `ui-contract-typed-retirement` | `ui-contract-typed-document` |
| `🧱️component` | `ui-contract-typed-retirement` | `ui-contract-typed-component` |
| `🪆️slot` | `members-of-modules` | `ui-class-name-slot` |
| `🌐️delivery` | `members-of-assets` | `asset-resolver-delivery` |
| `⚙️expansion` | `members-of-members-of-modules` | `derive-expansion` |
| `⚙️expansion` | `members-of-schema` | `derive-expansion` |
| `⚛️component` | `schema` | `schema-component` |
| `🌐️favicon` | `styling` | `ui-styling-favicon` |
| `🌐️iframe` | `styling` | `ui-styling-iframe` |
| `🌗️mixing` | `styling` | `ui-styling-mixing` |
| `🖌️ui` | `styling` | `ui-styling-ui` |
| `🔌️backend` | `members-of-members-of-modules` | `ui-host-backend` |
| `🪟️window` | `members-of-members-of-modules` | `ui-host-window` |

The two new UI element owners, `Ports/📡️interactive-jobs` and `Diagram/📐️layout`, contribute their exact contextual member identities. The focused inventory reduced the element findings from 21 to 19, with zero findings for these two owners. Focused inventories for actor, assets, host, schema, styling, derive and every newly registered owner also returned zero owned findings. Ambient findings outside this batch remain assigned separately.

## Runtime and compiler evidence

- Portable topology: 4 passed, 0 failed, 253 expectations.
- Actor moved-source tests: 2 files, 30 passed, 0 failed. Native import and package selection remained unchanged.
- Styling suite: 45 passed, 0 failed, 1,440 expectations; it includes the PostCSS oracle and moved CSS source identity.
- Schema crate: 28 unit tests, 1 schema-export test and 1 module-compile test passed.
- Host focused event suite: 19 passed. Host focused window suite: 17 passed.
- Metal all-features suite: 10 passed, including headless backend construction/readback paths.
- WGPU retained-UI intake budget: 4 passed, 0 failed, 13 expectations.
- Direct TypeScript import smoke loaded actor, assets, styling browser facade and builder exports successfully.
- UI-scoped Storybook: `STORYBOOK_SCOPE=ui bunx storybook build --output-dir <ticket>/🗑️generated/sol-framework-source/storybook-static-ui-green` transformed 1,555 modules and completed successfully. Vite reported 2m41s build time; the output transcript ran from 16:23:30 to 16:26:56 (3m26s observed command elapsed time). This validates the completed browser graph, rather than only the absence of the earlier Node error.

The Storybook browser boundary had two distinct causal edges. First, the original styling facade directly imported playground/workspace/build tooling; extracting the builder removed that edge. Second, the UI runtime reached the framework package/kernel/actor shard client, whose `if (import.meta.vitest)` dynamic test registrations pulled actor test modules into the production graph; those test modules imported the OS plugin build package, repository process helpers and `🗂️workspaces`. Production `import.meta.vitest = undefined` removes this second edge by compile-time elimination. Native Vitest still injects the sentinel on its own route.

The unscoped Storybook build now clears the former styling/Node failure and reaches the separately owned puzzle scope. It stops after 113 transformed modules on unresolved `@semio-tech/puzzle-5d-rs`, imported by `✏️s/🔌️plugins/🧩️puzzle/📖️stories/🎭️5d-timeline/🧪️.story.tsx`. The declared package is `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/📦️packages/🦀️rust/📋️project.json` with Nx name `@semio-tech/puzzle-5d-rs`; it exposes only build/check/test targets. Its permanent producer `📜️script.ts` calls `runArtifactRustPackageMain`, whose build route produces native Cargo artifacts under `dist/build`. The package directory currently contains Cargo.toml, project.json and the router, with no JavaScript package manifest or wasm package target/output for the Storybook import. This is a concrete binding/producer prerequisite for the separately owned metadata/package lane; this batch did not rebuild or redesign the puzzle plugin.

## Bounded limits and negative evidence

- The combined contract action filter ran 15 tests: 14 passed and `credited_alias_keeps_pages_live_after_original_handle_is_lost` failed because the queued UI value retirement arena was poisoned by suite ordering. The exact failing test passes 1/1 in isolation, so the source move itself compiles and runs; the global-state ordering defect remains outside this source taxonomy batch.
- The broad host run compiled the moved sources and ran 79 tests, with 70 passing and 9 admission/root tests failing on state assertions. Focused event/window behavior is green; no unchanged-baseline claim is made for the broad failures.
- `check-wgpu` compiled the complete declared browser source closure after the new path/order entries and initially stopped only on the separately owned frame-worker output being stale. The WGPU owner then regenerated `.vscode/launch.json`, the catalog and the canonical frame-worker output from the settled inputs; root generation/check and package generation/check all passed. The portable topology gate was rerun after that regeneration and still passed 4/4 with 253 expectations, including one exact launch entry in each authority.
- `bun ./📜️script.ts verify interactivity` reached path loading after old source identities were repaired, then stopped on current repository-wide launch/descriptor caps and the Puzzle fill envelope self-test (seven reported ownership/credit assertions). This run is retained here as a limit, not reported as a pass.
- Browser asset URL warnings in the successful UI-scoped Storybook build identify runtime-served absolute asset paths; they did not prevent bundle completion.

## Exact source/support file list

The move, extraction, native-binding and test-owner tables above are the exact old/new identity list, including removals. The following are the exact repo-relative source/support files directly authored or changed for schema, dispatch, consumer references, manifests, browser closure, contextual taxonomy or native test inclusion. Concurrent changes inside shared files are preserved and are not separately attributed.

- `.storybook/main.ts`
- `.storybook/📖️stories/🧭️coordination/🟦️.ts`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- `♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts`
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/vitest.config.ts`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/⚙️vite.config.ts`
- `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🎨️.css`
- `🌎️hub/🔨️modules/🛡️admin/🧫️fixtures/🕸️build-graph/🎨️stylesheet-graph.json`
- `📜️script.ts`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/vitest.config.ts`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🎭️actor/📬️mailbox/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/🛑️cancel-job-reply/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/🧪️tests/📨️effect-wire-routes/🟦️.ts`
- `🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/📨️effect-wire-routes/🔣️.json`
- `🧰️framework/🔨️modules/🎭️actor/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/🔄️machine/✨️derive/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/components.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌐️iframe/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🛂️adapters.manifest.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧫️fixtures/🔁️animation-scope/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧪️tests/🧩️slot/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎬️scene/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/⚠️vk-error/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/💾️memory/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/🔗️swapchain-support/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/🗃️resources/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/🗺️descriptor-layout/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/🪟️surface/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/🍎️backend/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📬️frame-buffers/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/🧭️objective-c/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🌆️scene-target/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🌐️backend/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🎞️frame/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📈️buffers/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🔌️gpu-context/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🔤️gpu-types/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🗃️resources/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🚦️surface-state/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧊️surface-adapter/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧮️gpu-uniforms/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/🏗️pipelines/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📬️frame-buffers/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/🪟️backend/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🟨️javascript/🟨️.js`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️document-tree-reconcile/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🛡️limits/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧬️typegen-export/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕸️Diagram/🧪️tests/🧩️component/🟦️.tsx`
- `🧰️framework/🔨️modules/🖼️assets/🎯️concepts/🟦️.ts`
- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🔨️modules/🖼️assets/🔍️resolver/🟦️.ts`
- `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🔨️modules/🧬️schema/✨️derive/🦀️.rs`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts`
- `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🏷️entity-kinds/🟦️.ts`
- `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🧩️module-compile/🦀️.rs`
- `🧰️framework/🔨️modules/🧬️schema/🧫️fixtures/✅️draft07-validation-vectors.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/vitest.config.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/vitest.config.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️framework-source-topology/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️framework-source-topology/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️framework-source-topology/🔣️.json`

The 22 render implementation leaves that include the renamed semantic test owners are listed above as support files; their language-independent test identities are the 23 old/new rows in the render map.
