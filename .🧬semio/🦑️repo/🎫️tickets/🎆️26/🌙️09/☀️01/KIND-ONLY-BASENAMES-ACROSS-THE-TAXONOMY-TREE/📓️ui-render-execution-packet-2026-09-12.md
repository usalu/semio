# UI Runtime, Scene, and Render Execution Packet

The completed WGPU/TUI target lane does not cover the UI runtime, scene core, or render backend crates. This next packet owns these three semantic owners, including render target backends. The report snapshot below is only a starting inventory: inspect all anonymous package-root source bodies and source references as well.

Move authored implementation from language package directories to the relevant domain owner and use anonymous source leaves. Preserve exact minimal Cargo package glue, cfg/features, module visibility, macro registrations, and platform selection. Capture source identity before moving concurrent live files; validate actual referent identity after rebasing, not just that each path exists. Register narrow semantic owner/member additions in the shared schema before edits.

Do not absorb the styling generators or the general artifact-oracle/generator taxonomy families into this packet. Register any new command in the current launch source authority and its derived launch output. Use existing package Nx checks and focused tests; document a compiler/platform limitation accurately rather than rerunning broad suites indefinitely.

## Snapshot Named Leaves

- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🌉️surface.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🌍️world3d_snapshot.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🎬️scenes.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/📐️math.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/📦️pack.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🖼️canvas2d_snapshot.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/⚠️vk_error.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🌋️backend.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/💾️memory.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🔗️swapchain_support.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🗃️resources.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🗺️descriptor_layout.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust/🪟️surface.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/✨️msl.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🌐️world3d.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🌫️scene_target.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🍎️backend.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🏗️pipelines.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/📬️frame_buffers.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🗃️resources.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧭️objective_c.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust/🧱️types.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🌆️scene_target.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🌐️backend.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🎞️frame.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/📈️buffers.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🔌️gpu_context.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🔤️gpu_types.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🗃️resources.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🚦️surface_state.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🧊️surface_adapter.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🧮️gpu_uniforms.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🧵️pipelines.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/✨️hlsl.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🌐️world3d.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🌫️scene_target.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🏗️pipelines.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/📬️frame_buffers.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🗃️resources.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🧱️types.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust/🪟️backend.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/⏱️schedule.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/✨️shader_contract.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🎬️scene.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📏️layout.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📐️tessellate.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🔌️backend.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🖋️text.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🖱️dispatch.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🖼️frame.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🗃️resource.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🗺️surface.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/🧱️element.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🎛️context.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🎭️present.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🎯️dispatch.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/👥️presence.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📥️inbox.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📮️gateway.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🔄️transaction.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🕸️tracking.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🪪️entity.rs`
