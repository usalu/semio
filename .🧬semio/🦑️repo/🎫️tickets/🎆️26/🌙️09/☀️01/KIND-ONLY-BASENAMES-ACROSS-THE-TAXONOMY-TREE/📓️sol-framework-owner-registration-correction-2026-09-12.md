# Framework Owner Registration Correction

## Result

The 24 semantic source-owner directories introduced by the framework source batch now have exact contextual directory kinds. The correction registers each of the 23 render test owners only beneath the existing `tests` kind and registers the Vite implementation owner only beneath the existing `builder` kind. It adds no arbitrary test-slug rule, emoji-wide admission, member exemption, path exclusion, or compatibility identity.

The portable framework-source fixture now enumerates every corrected owner. Its test ties those rows to the actual owner paths and validates the live ancestor chains:

- `🖌️render` under `members-of-modules` → `ui-render`
- `🧪️tests` under `ui-render` → `tests`
- each of the 23 exact render owners under `tests` → its exact `ui-render-test-…` kind
- `🎨️styling` under `members-of-modules` → `styling`
- `🏗️builder` under `styling` → `builder`
- `🌐️vite` under `builder` → `ui-styling-builder-vite`

## Exact Owner Registrations

| Owner path | Directory kind | Parent kind |
| --- | --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-backend-unit` | `ui-render-test-d3d12-backend-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-frame-buffers-unit` | `ui-render-test-d3d12-frame-buffers-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-pipelines-unit` | `ui-render-test-d3d12-pipelines-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-backend-drawable-size` | `ui-render-test-metal-backend-drawable-size` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-backend-unit` | `ui-render-test-metal-backend-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-frame-buffers-unit` | `ui-render-test-metal-frame-buffers-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-objective-c-unit` | `ui-render-test-metal-objective-c-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-descriptor-layout-unit` | `ui-render-test-vulkan-descriptor-layout-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-memory-unit` | `ui-render-test-vulkan-memory-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-resources-unit` | `ui-render-test-vulkan-resources-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-surface-unit` | `ui-render-test-vulkan-surface-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-swapchain-support-unit` | `ui-render-test-vulkan-swapchain-support-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-vk-error-unit` | `ui-render-test-vulkan-vk-error-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-backend-unit` | `ui-render-test-webgpu-backend-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-buffers-unit` | `ui-render-test-webgpu-buffers-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-frame-unit` | `ui-render-test-webgpu-frame-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-gpu-context-unit` | `ui-render-test-webgpu-gpu-context-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-gpu-types-unit` | `ui-render-test-webgpu-gpu-types-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-gpu-uniforms-unit` | `ui-render-test-webgpu-gpu-uniforms-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-resources-unit` | `ui-render-test-webgpu-resources-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-scene-target-unit` | `ui-render-test-webgpu-scene-target-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-surface-adapter-unit` | `ui-render-test-webgpu-surface-adapter-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-surface-state-unit` | `ui-render-test-webgpu-surface-state-unit` | `tests` |
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite` | `ui-styling-builder-vite` | `builder` |

Every kind uses an anchored exact slug pattern and the owner's existing emoji. All 23 render kinds require `parentKindIds: ["tests"]`; the Vite kind requires `parentKindIds: ["builder"]`.

## TDD and Validation

The portable fixture was extended first. Before the taxonomy registrations, the direct test failed at the first new row:

- command: `bun test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️framework-source-topology/🟦️.ts`
- result: 3 pass, 1 fail, 254 expectations, 151 ms
- failure: `🔬️d3d12-backend-unit` expected `ui-render-test-d3d12-backend-unit`, received `null`

After registration and owner-path/ancestor assertions:

- direct portable test: 4 pass, 0 fail, 329 expectations, 122 ms
- registered route: `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/repo-lib:test-framework-source-topology --skip-nx-cache`
- registered result: 4 pass, 0 fail, 329 expectations; Bun test 135 ms, Nx target 376 ms, outer invocation 30.2 s
- strict loader: `loadCatalogTaxonomy` followed by `validateTaxonomy` returned no problems

Scoped inventory used the permanent root route:

- `bun ./📜️script.ts clean taxonomy inventory --scope '🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests' --format json`: 79 entries, all 23 corrected owner paths present, zero corrected-owner violations, 14 ambient violations
- `bun ./📜️script.ts clean taxonomy inventory --scope '🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder' --format json`: 8 entries, corrected Vite owner present, zero violations

## Ambient Render Findings

The 14 findings outside this correction remain assigned to the contextual-directory closure lane. This report makes no baseline or causal claim for them:

- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️backend-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️dispatch-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️element-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️frame-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️interactivity-mounted-prepared-render`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️layout-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️resource-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️scene-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️schedule-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️shader-contract-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️surface-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️tessellate-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️text-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🖼️webgpu-surface`

## Exact Files

Updated:

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧱️framework-source-topology/🔣️.json`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️framework-source-topology/🟦️.ts`

Created and retained:

- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-framework-owner-registration-correction-2026-09-12.md`

No source, package, launch, Storybook, session, Metal, or WGPU bytes were changed by this registration-only correction. The packet explicitly did not require another full Storybook build. Disposable command logs and inventories under `🗑️generated/sol-framework-owner-registration` were removed after their results were retained here.
