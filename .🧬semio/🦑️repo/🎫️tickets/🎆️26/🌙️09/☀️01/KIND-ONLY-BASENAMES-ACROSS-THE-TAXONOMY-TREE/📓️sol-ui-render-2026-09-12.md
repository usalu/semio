# UI Runtime, Scene, and Render Taxonomy Repair

Date: 2026-09-12

## Scope and result

This lane repaired the implementation topology owned by UI runtime, scene, render, and the Vulkan, Metal, WebGPU, and D3D12 render targets. It moved 63 semantic Rust filenames and one semantic JavaScript package body to schema-registered domain directories with anonymous implementation leaves. The seven Rust package roots remain as declaration-only Cargo glue and mount the domain-owned Rust leaves with explicit paths. Cfg gates, module names, visibility, public reexports, platform selection, and the WebGPU ABI registration remain in those package roots.

The final owned shape contains no semantic implementation filename below these language package directories. The former WebGPU JavaScript package body now shares the `🧊️surface-adapter` domain directory with its Rust implementation.

## Schema-first contract

The taxonomy schema at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` gained contextual kinds `ui-runtime`, `ui-scene`, `vulkan-target`, `metal-target`, `webgpu-target`, and `d3d12-target`; `ui-render` already existed. The corresponding owner-local registries are:

- `members-of-ui-runtime`: `♻️reconcile`, `🎛️context`, `🎭️present`, `🎯️dispatch`, `📮️gateway`, `🕸️tracking`.
- `members-of-ui-scene`: `🌉️surface`, `🌍️world3d-snapshot`, `📐️math`, `📦️pack`, `🖼️canvas2d-snapshot`.
- `members-of-ui-render`: `⏱️schedule`, `✨️shader-contract`, `🎬️scene`, `📏️layout`, `📐️tessellate`, `🔌️backend`, `🖋️text`, `🖱️dispatch`, `🗃️resource`, `🗺️surface`, `🧱️element`.
- `members-of-vulkan-target`: `⚠️vk-error`, `🌋️backend`, `🔗️swapchain-support`, `🗃️resources`, `🗺️descriptor-layout`, `🪟️surface`.
- `members-of-metal-target`: `✨️msl`, `🌐️world3d`, `🌫️scene-target`, `🍎️backend`, `🏗️pipelines`, `📬️frame-buffers`, `🗃️resources`, `🧭️objective-c`, `🧱️types`.
- `members-of-webgpu-target`: `🌆️scene-target`, `🌐️backend`, `🎞️frame`, `📈️buffers`, `🔌️gpu-context`, `🔤️gpu-types`, `🗃️resources`, `🚦️surface-state`, `🧊️surface-adapter`, `🧮️gpu-uniforms`, `🧵️pipelines`.
- `members-of-d3d12-target`: `✨️hlsl`, `🌐️world3d`, `🌫️scene-target`, `🏗️pipelines`, `📬️frame-buffers`, `🗃️resources`, `🧱️types`, `🪟️backend`.

Actual classifier validation showed seven moved domain names already resolve through reusable existing kinds, so they were deliberately omitted from the new owner-local registry member lists: runtime `👥️presence`, `📥️inbox`, `🔄️transaction`, and `🪪️entity`; scene `🎬️scenes`; render `🖼️frame`; Vulkan `💾️memory`. This avoids overlapping contextual and global kind resolution. The other 56 new domain directories resolve through the owner-local registries.

The language-neutral kind-only-basename test was driven red first: the focused census exited 1 with 63 named Rust implementation leaves and missing contextual registrations. After the schema and moves, `bun nx run @semio-tech/repo-lib:test-kind-only-basename` passed 6 tests, 42 expectations, with no failures. That target exercises the portable JSON fixture and independent Ajv, fast-glob, and TOML oracles in addition to target-first discovery.

## Complete move map

| Previous package-owned source | Domain-owned anonymous leaf |
| --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🎛️context.rs` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎛️context/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🎭️present.rs` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎭️present/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🎯️dispatch.rs` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎯️dispatch/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/👥️presence.rs` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/👥️presence/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📥️inbox.rs` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📥️inbox/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📮️gateway.rs` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📮️gateway/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🔄️transaction.rs` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🔄️transaction/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🕸️tracking.rs` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🕸️tracking/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🪪️entity.rs` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🪪️entity/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🌉️surface.rs` | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🌉️surface/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🌍️world3d_snapshot.rs` | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🌍️world3d-snapshot/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🎬️scenes.rs` | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/📐️math.rs` | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/📦️pack.rs` | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️pack/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🖼️canvas2d_snapshot.rs` | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🖼️canvas2d-snapshot/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/⏱️schedule.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/⏱️schedule/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/✨️shader_contract.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/✨️shader-contract/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🎬️scene.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎬️scene/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📏️layout.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/📏️layout/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📐️tessellate.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/📐️tessellate/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🔌️backend.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🔌️backend/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🖋️text.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖋️text/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🖱️dispatch.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖱️dispatch/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🖼️frame.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖼️frame/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🗃️resource.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🗃️resource/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🗺️surface.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🗺️surface/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🧱️element.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧱️element/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/⚠️vk_error.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/⚠️vk-error/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🌋️backend.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/🌋️backend/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/💾️memory.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/💾️memory/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🔗️swapchain_support.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/🔗️swapchain-support/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🗃️resources.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/🗃️resources/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🗺️descriptor_layout.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/🗺️descriptor-layout/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🪟️surface.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/🪟️surface/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/✨️msl.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/✨️msl/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🌐️world3d.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/🌐️world3d/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🌫️scene_target.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/🌫️scene-target/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🍎️backend.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/🍎️backend/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🏗️pipelines.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/🏗️pipelines/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/📬️frame_buffers.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📬️frame-buffers/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🗃️resources.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/🗃️resources/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧭️objective_c.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/🧭️objective-c/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧱️types.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/🧱️types/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🌆️scene_target.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🌆️scene-target/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🌐️backend.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🌐️backend/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🎞️frame.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🎞️frame/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/📈️buffers.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📈️buffers/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🔌️gpu_context.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🔌️gpu-context/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🔤️gpu_types.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🔤️gpu-types/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🗃️resources.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🗃️resources/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🚦️surface_state.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🚦️surface-state/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🧊️surface_adapter.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧊️surface-adapter/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🧮️gpu_uniforms.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧮️gpu-uniforms/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🧵️pipelines.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧵️pipelines/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/✨️hlsl.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/✨️hlsl/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🌐️world3d.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/🌐️world3d/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🌫️scene_target.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/🌫️scene-target/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🏗️pipelines.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/🏗️pipelines/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/📬️frame_buffers.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📬️frame-buffers/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🗃️resources.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/🗃️resources/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🧱️types.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/🧱️types/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🪟️backend.rs` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/🪟️backend/🦀️.rs` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🟨️javascript/🟨️.js` | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧊️surface-adapter/🟨️.js` |

## Package glue retained

These are the only retained Rust package root bodies in the lane. The actual Rust package classifier reports each as `declaration`; the largest contains 32 statements.

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🦀️.rs`

## Consumer changes

Each retained package root above now mounts its semantic modules from `../../<domain>/🦀️.rs` while preserving the existing module identifier, visibility, cfg attributes, and reexports.

The following moved Rust leaves required relative path rebasing for tests, runtime siblings, font bytes, or the WebGPU schema. These are the exact internal consumers observed after the move:

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎭️present/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🎯️dispatch/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/👥️presence/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📥️inbox/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📮️gateway/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🔄️transaction/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🕸️tracking/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🪪️entity/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🌉️surface/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🌍️world3d-snapshot/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️pack/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🖼️canvas2d-snapshot/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/⏱️schedule/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/✨️shader-contract/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎬️scene/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📏️layout/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📐️tessellate/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🔌️backend/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖋️text/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖱️dispatch/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖼️frame/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🗃️resource/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🗺️surface/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧱️element/🦀️.rs`
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

The path transformations were:

- owner-level test mounts: `../../🧪️` to `../🧪️`;
- runtime reconcile owner siblings: `../../📤️output`, `../../♻️retirement`, `../../🩹️patch`, and `../../🚪️handback` to `../...`;
- render font assets: `../../../../🖼️assets` to `../../../🖼️assets`;
- target-backend test mounts: `../../../../🧪️` to `../../../🧪️`;
- WebGPU surface schema: `../../../../🧬️schema` to `../../../🧬️schema`;
- WebGPU JavaScript host import: `../../../../../🖥️host/.../🟨️.js` to `../../../../🖥️host/.../🟨️.js`.

These exact non-package consumers were updated:

- `📜️script.ts`
- `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️interactivity-mounted-prepared-render/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🖼️webgpu-surface/🟨️.js`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔣️icon-name/🧾️value/🦀️.rs`

The root script constants now identify the domain-owned frame, scene, world snapshot, canvas snapshot, reconcile, and present leaves. The scene fixture and mesh/test consumers point to the new coordinates. The WGPU target and its icon-name documentation now identify the coordinated generated icon leaf `🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs`, which exists at the completed producer coordinate. No executable command was added, so no launch registration changed.

## Source and referent identity

Live SHA-256 values were captured before any move for the 63 semantic Rust files, seven Rust package roots, and the JavaScript package body. After all moves, a normalized byte comparison reversed only the listed relative-path substitutions and passed all 71 captured files:

`[DEBUG] final normalized source identity checked for 71 implementation/glue files`

A separate canonical-path comparison resolved each old literal relative to its old source and each rebased literal relative to its new source. It proved the same referent for 71 external literals and all 63 Rust package mounts:

`[DEBUG] referent identity checked for 71 external literals and 63 moved module mounts`

The final existence sweep parsed the live tree and found every referent:

`[DEBUG] final referent existence checked for 162 Rust path/includes and 2 relative JavaScript imports`

The JavaScript move received an additional host import and test-consumer check:

`[DEBUG] JavaScript external referent identity=true testConsumer=true`

### Captured pre-move hashes

| Live pre-move source | SHA-256 |
| --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🌉️surface.rs` | `ccaa29ddcc5077afab6f86c45fc256ea03ad31d267aaeea0e10db39adb2bc7d5` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🌍️world3d_snapshot.rs` | `9d467aa797e51b6578ce031a5d62344837a2843919570bb4fe82569eb2a765cf` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🎬️scenes.rs` | `94d307a41a77a81e54ab9499cf42e91acc1ec3e61b884b47b95de0244abe1efb` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/📐️math.rs` | `f0db67bdd877a24795977c737f40c2645ac27b480504e91da5a82927aca5b1b3` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/📦️pack.rs` | `d6ca253f4a2125d916072350b67ab99a982cf8e72b43aee4bc4899896aef71e6` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🖼️canvas2d_snapshot.rs` | `1b506d507961ca83e69dbfc557cd79457b998026aedeecaf3d070f096f5f9f09` |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️.rs` | `d999428e401e9ce841c2d774268d8c38a654b2a90bef4375ae6ff53325245838` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/⚠️vk_error.rs` | `99ad3d41a5b68372365d923f2858f8a40b80865d8c37ecb04715835061f49f4d` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🌋️backend.rs` | `79e4ca1c858bcfddcdba3be063616096a5016dc10cf3516d1c7e1a6c868a396a` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/💾️memory.rs` | `2576dcca2121cd7a88c31d2cdfd3c75f5c53f05be9c8829b720ed3a8efa55f65` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🔗️swapchain_support.rs` | `7295190afa0186dd1512adcd83afefce3cf1397d5094de3f6ff855d0f4f13523` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🗃️resources.rs` | `6416c61dbb1953074de03d9cf26d902991d9319436d1d3f0a7ce5998a73c237c` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🗺️descriptor_layout.rs` | `d0f8cf13aa07c3e15b871c5c38c52434a914a6024d74321e477d9b1f475a4ca9` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🦀️.rs` | `1162cad381c2d76c49b865dc18727e69ccf643ea05ee7fea5570fa382eb8095a` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🪟️surface.rs` | `3ae307562bfca04387d555219a68e153e72ad6f13c7aa712b80a2fb2bc07eef1` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/✨️msl.rs` | `7bf324f7962c6eb4c353e2e1df5f45e081a47f7dab52f73ad8320e7075cc3a6e` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🌐️world3d.rs` | `e14e02895b50afbe5c5dfc38a83bc3408fd2b42f84295f90e0d3cb463770a620` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🌫️scene_target.rs` | `45bdfbfffecfc19dd9d54b7d6dde6eae88af2c3fba2ff25c53498a76f97cde4f` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🍎️backend.rs` | `770c905d59b7eeea0586183469b84e547e0aedac6420b6386e30354be8ac8112` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🏗️pipelines.rs` | `cada3dab95c9364fbe4bdcf971e456258e9ee15fc7f396196ef74e1f36c699b7` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/📬️frame_buffers.rs` | `232a19e2a77898fa9bbf98cb1f84c5c80565e7aba5d73e867680be76ce917891` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🗃️resources.rs` | `dda7b073953154841a712dbf5f5593550aff5ce940f4018ff15d1dc38f94f1b1` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🦀️.rs` | `0d6496a5f621b58c1e975ea93c0575ab797b7bd239e742cca3184440965a4545` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧭️objective_c.rs` | `4e35278ac6473a975324cf1a2ea29be96fb2cb3ea5398fae49d01fe0a88dc0c3` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧱️types.rs` | `769e30a07ace947d455c30fc1267e587c792cd88ae0bbb7086563745ca3178c2` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🌆️scene_target.rs` | `075ee41f6a39c100e2c98d462b1c031cced3fb5ab2d66b5a6eebd9a39c3a6f24` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🌐️backend.rs` | `e5436e7e16143680033021c28b50c3c6d2e4aebe6475aaf318684bf01e34ea57` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🎞️frame.rs` | `6e41bba66edfc1d24fb7d4e42dfb5132900e292a8f673bde417eca541578c0fe` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/📈️buffers.rs` | `8960f0a57ada5bd276163a9286bbbd3598417215e6180e184f2fab1596f0b04c` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🔌️gpu_context.rs` | `00e88620746ec4f38eddc86bb3eaf62e1ff4f79a01afc275d6b1b088f3838a36` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🔤️gpu_types.rs` | `58b81a1bfc9c3dff68209a07c7cc582b5f50d5ba5f86c8d96e0d66c32e2e24e1` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🗃️resources.rs` | `a9d483d53a49663405f95171b4ba671d6efdee928f8fd2621142ab9ee5de9de8` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🦀️.rs` | `051aaa7cc9213a596e84a09bb495dbefbd63d20833dad8bfc673b975b17c2c8f` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🧊️surface_adapter.rs` | `1e0da4a92c6ac49fbf20d39d0c62d0340a428c2538f3484b5b9cedf4ea59f9bc` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🧮️gpu_uniforms.rs` | `86384919552d46ad1a0b5b3503e7ba1c6cd8fe6fc48e6c763abfeb515bf73ec5` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🧵️pipelines.rs` | `af843f7f82b3363692ec217f2cbfba2029088719eb12fd6289f2d88a87a3b3ea` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🚦️surface_state.rs` | `30c22bcde999099e3de849d43b7cd06177488ff9c96865288da0bb340a329284` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🟨️javascript/🟨️.js` | `9504edc24efeaae1fad0e3ffabfe7ad8cfa54c553b4fb2fe15eb8c2fa5e8da46` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/✨️hlsl.rs` | `83ff1141b44d0dbc93c3f958d20d5509f8e3cbe80f72987842f91b9fcfde7554` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🌐️world3d.rs` | `179b1e78f6f54b012baa5ca2a3218485c4a418d951810e7327fc0e8e64b57d4f` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🌫️scene_target.rs` | `09b052380a10e66b341487f5e91f9515f79daa20b4ef5a3ae244b0a72e07112e` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🏗️pipelines.rs` | `d5d81c8b77d30faab8417fa36c9f4c71d26935b0b1896ff29d15aa0497cee070` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/📬️frame_buffers.rs` | `8a20cd6dd1588a96c14a51ca7f1851b5792141a59847e4c5ff6a4c65c63cff83` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🗃️resources.rs` | `fb9232666afda35d2644e31bb21ea9350b4e117a2ca7a47b2e1f5b14202fa514` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🦀️.rs` | `722c2adc35d12abf14a757ecf9ff523c1aaad91f5cccea2ce8518ef0521a6db0` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🧱️types.rs` | `46f53ddcb026f357fc78ba5bd8837087ed17f48c0ffdbe06d29288699be7785f` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🪟️backend.rs` | `f12c0741877e43a624dd645c859a4218925b5f2d6a1979d8e9a3dd1564dfe42f` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/⏱️schedule.rs` | `dcf857ea6ad7a22112f20637e392ecd93b7a2cd6bcb2bfd0add038d5ef214238` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/✨️shader_contract.rs` | `7ef5d8eedfd01aafccea058fbf20e678a06ff2b9c7a6647cd6f0914268660393` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🎬️scene.rs` | `424f31357f9091475e501e2abf028b50cce30b0d49921f693378e268b91501d4` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📏️layout.rs` | `2db24744efaa3616fa6c89abb018eba07a27757148a9061640e88763c43665f8` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📐️tessellate.rs` | `1e764323a542245ddcb7e2d1cbcc5f8c01bf6a6376edebb3d89ac52a48310118` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🔌️backend.rs` | `b7e654f21d6fac182d652e06ad985e7daa87653d833cd5df4eed0f0a07fbb429` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🖋️text.rs` | `a84571842145da61514bfe4ce7f6f7c6926bf3cae9102b160a0a3993928e177d` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🖱️dispatch.rs` | `7e10927834c68d15f0d6925b2c3a94cfefbd08faeed97dd810c641163c7f3ae3` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🖼️frame.rs` | `cafdf07fc9f0649a97eca1da0d97d2e682fe29b249fe87a1bbd4028ab72776f0` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🗃️resource.rs` | `23183ae95e3dceeae07b13068cb3305a719c2bc49e1bc23449c9822251182c22` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🗺️surface.rs` | `d9dcf6a883819ed5848dbf8f650d5d6b806aca24b02133030e64047584039d7b` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🦀️.rs` | `062b2a1c2a807b03ec38d47747904ec194edda721fa161e9c09830f2754bb4f6` |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🧱️element.rs` | `701cffa32b290ddb1e386c520c8c7cdb7015071d4892138e8485b9043d1fe7a1` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs` | `c8791cdd9a2fb90a323db15ec0dd6e7aa4c530a622525434d5c2a33c0da0eafe` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🎛️context.rs` | `c2423bb9ad4953ae7bdbf4e1b35e72ccc7a07f5cb3c38931991e4128615c543d` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🎭️present.rs` | `cc84fbcf1ce28e11960d7e8fbed9e9a1059cbbe2bb02d12af52d467b8ba54940` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🎯️dispatch.rs` | `aff137846ed79053fe8bad3b3defefa227a760fb424155d88f142eb86a48d1cb` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/👥️presence.rs` | `e2ed939a39338e34a8d528ca96b14dea2c8dd380c8f995ecf84020c46f394519` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📥️inbox.rs` | `936486cbd1f44b6205bab6fdc6e70883f3a9b87cfb296a13078f606822f80e72` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📮️gateway.rs` | `8bd6e63dc6b3b34525309b8fb1a4f3fef67b5ea327db9957f06651ec028e9c9f` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🔄️transaction.rs` | `32b098906edf6cf15e4708ea5ca36fd3cac8ee750e78742d2ef1b4ef4d152e94` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🕸️tracking.rs` | `1b85eee94331c3bd6a8248f8ea6863502dfe6a332af278ec9b6f9b1a6304ff34` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️.rs` | `eee7b8623ffa6e285ae09483e0fb3d903e9944e8b236fbfac0e08fd9f21297ac` |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🪪️entity.rs` | `13bb462c6e06b5d376673b99fdfee61649e9c0989b41974b6dc360e2b870ff1c` |

## Scoped taxonomy inventory

The bounded `inventoryTaxonomy({ repoRoot, scope, progress })` run completed on the live tree before a subsequent concurrent schema change. Its owner counts and lane result were:

- runtime: 110 entries; zero moved-domain violations; 19 ambient unresolved directories;
- scene: 40 entries; zero moved-domain violations; 8 ambient unresolved directories;
- render before moving the JavaScript body: 210 entries and one owned `package-implementation-file` finding;
- render after moving the JavaScript body: 209 entries; zero moved-domain violations; 39 ambient unresolved directories.

The ambient unresolved directories were outside the newly registered implementation domains and were not expanded into this bounded lane. Exact paths follow.

### Runtime ambient unresolved paths

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🌲️tree`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🔬️runtime-tree-retirement`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🧪️tests/📤️output`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📤️output/🧪️tests/🔬️unit`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️dispatch-unit`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️entity-unit`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️gateway-unit`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️inbox-unit`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️presence-unit`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️present-unit`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️reconcile-unit`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️surface-ownership`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️tracking-unit`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️transaction-unit`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🚪️handback`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🚪️handback/🧪️tests/🚪️handback`

### Scene ambient unresolved paths

- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️canvas2d-snapshot-value-round-trip`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️math-unit`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️pack-unit`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️scenes-unit`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️scenes-value-round-trip`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️surface-unit`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️world3d-snapshot-unit`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes`

### Render ambient unresolved paths

- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️backend-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-packages-rust-backend-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-packages-rust-frame-buffers-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️d3d12-packages-rust-pipelines-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️dispatch-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️element-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️frame-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️interactivity-mounted-prepared-render`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️layout-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-packages-rust-backend-drawable-size`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-packages-rust-backend-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-packages-rust-frame-buffers-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-packages-rust-objective-c-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️resource-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️scene-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️schedule-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️shader-contract-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️surface-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️tessellate-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️text-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-packages-rust-descriptor-layout-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-packages-rust-memory-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-packages-rust-resources-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-packages-rust-surface-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-packages-rust-swapchain-support-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️vulkan-packages-rust-vk-error-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-backend-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-buffers-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-frame-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-gpu-context-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-gpu-types-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-gpu-uniforms-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-resources-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-scene-target-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-surface-adapter-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️webgpu-packages-rust-surface-state-unit`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🖼️webgpu-surface`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧫️fixtures/🍎️metal`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧫️fixtures/🧊️webgpu`

A later attempt to repeat the scoped inventory currently stops before inventory at schema validation with this diagnostic from the live concurrent schema state:

`generatorContracts["playground-session"]: previewTarget must be the exact owner preview-generated target or an explicit same-project preview-* invocation`

This report therefore retains the exact results and path lists from the completed scoped runs above and makes no inference about the origin or baseline status of the later diagnostic.

## Verification

- `bun nx run @semio-tech/repo-lib:test-kind-only-basename`: passed, 6 tests and 42 expectations.
- `bun nx run @semio-tech/ui-runtime-rs:test-quick`: passed, 122 tests and 2 skipped; the language-neutral surface ownership oracle passed 40 checks.
- `bun nx run @semio-tech/ui-scene-rs:test-quick`: passed, 117 tests.
- `bun nx run @semio-tech/ui-render-rs:test-quick`: passed, 130 tests. The JavaScript A2 and surface lifecycle oracle emitted its expected JSON. A repeat after the JavaScript move also passed.
- Registered render `boundaries`: passed; no `wgpu`, `winit`, or `semio-framework-actor` boundary finding.
- WebGPU native `cargo test --all-features --lib`: passed, 28 tests.
- Vulkan `cargo check --all-features --target x86_64-unknown-linux-gnu`: passed with warnings.
- WebGPU `cargo check --all-features --target wasm32-unknown-unknown`: passed.
- D3D12 `cargo check --all-features --target x86_64-pc-windows-msvc`: passed with warnings.
- Metal `cargo check --all-features` on macOS: passed with one unnecessary-unsafe warning.

The first render quick-test attempt observed a missing generated styling token module while another lane was moving that generated output. The retry after the producer completed passed as recorded above.

## Current Metal test limitation

Metal `cargo test --all-features --lib` currently fails to compile five assertions in `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️metal-packages-rust-backend-unit/🦀️.rs` at lines 45, 51, 67, 73, and 90. Each is Rust error E0599 because `Scene::finish` is absent. The rebased test mount is found and compiled, and the Metal library check passes. This is the observed current result only; no claim is made about when that test mismatch originated.

## Reference closure and exclusions

A final active-reference search found no runtime, scene, render, or backend source/config/test consumer pointing at the former package-owned semantic source coordinates. The frozen `remaining-package-purity-authority` fixture still records historical coordinates and is owned by the shared package-body enforcement lane, so it was left unchanged. Generated styling/assets/plugin producer topology and general artifact-oracle work were outside this lane.
