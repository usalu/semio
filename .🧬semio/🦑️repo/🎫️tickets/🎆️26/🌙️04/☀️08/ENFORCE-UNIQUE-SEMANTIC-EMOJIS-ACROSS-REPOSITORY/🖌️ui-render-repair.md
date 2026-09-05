# UI Render Hand Repair

Scope: `🧰️framework/🔨️modules/🖱️ui/🖌️render`, including the four backend packages. The parent owns surrounding UI trees. No Git mutations, generated rename choices, migration scripts, or historical fixture rewrites.

## Reviewed decisions

Each old basename below starts with `🦀️`. The table specifies its exact replacement; extensions and module identifiers remain unchanged. Choices follow the inspected file's actual responsibility, independently checked against its siblings. The unique `🦀️.rs` Rust entry leaves, literal `Cargo.toml`, fixed `📋️project.json`, script, and already-unique fixtures remain unchanged.

| Parent under scope | Old basename | New basename | Responsibility |
| --- | --- | --- | --- |
| `📦️packages/🦀️rust` | `🦀️backend.rs` | `🔌️backend.rs` | Backend interface and null implementation |
| same | `🦀️dispatch.rs` | `🖱️dispatch.rs` | Pointer, focus and input dispatch |
| same | `🦀️element.rs` | `🧱️element.rs` | Element trait and frame arena |
| same | `🦀️frame.rs` | `🖼️frame.rs` | Atomic presented frame snapshots |
| same | `🦀️layout.rs` | `📏️layout.rs` | Intrinsic measurement and layout |
| same | `🦀️resource.rs` | `🗃️resource.rs` | Typed resource residency |
| same | `🦀️scene.rs` | `🎬️scene.rs` | Display-list scene assembly |
| same | `🦀️schedule.rs` | `⏱️schedule.rs` | Frame invalidation scheduling |
| same | `🦀️shader_contract.rs` | `✨️shader_contract.rs` | Shader families and pipeline contract |
| same | `🦀️surface.rs` | `🗺️surface.rs` | Embedded product surface placement |
| same | `🦀️tessellate.rs` | `📐️tessellate.rs` | CPU tessellation geometry |
| same | `🦀️text.rs` | `🖋️text.rs` | Text shaping and glyph atlas |
| `🎯️targets/🌋️vulkan/📦️packages/🦀️rust` | `🦀️backend.rs` | `🌋️backend.rs` | Vulkan implementation |
| same | `🦀️descriptor_layout.rs` | `🗺️descriptor_layout.rs` | Contract-to-Vulkan value mappings |
| same | `🦀️memory.rs` | `💾️memory.rs` | Device memory allocation |
| same | `🦀️resources.rs` | `🗃️resources.rs` | Resource residency and uploads |
| same | `🦀️surface.rs` | `🪟️surface.rs` | Native window surface ABI |
| same | `🦀️swapchain_support.rs` | `🔗️swapchain_support.rs` | Swapchain selection decisions |
| same | `🦀️vk_error.rs` | `⚠️vk_error.rs` | Vulkan failure classification |
| `🎯️targets/🍎️metal/📦️packages/🦀️rust` | `🦀️backend.rs` | `🍎️backend.rs` | Metal implementation |
| same | `🦀️frame_buffers.rs` | `📬️frame_buffers.rs` | Per-frame upload buffers |
| same | `🦀️msl.rs` | `✨️msl.rs` | Metal shader source |
| same | `🦀️objective_c.rs` | `🧭️objective_c.rs` | Objective-C ownership and ABI boundary |
| same | `🦀️pipelines.rs` | `🏗️pipelines.rs` | GPU pipeline construction |
| same | `🦀️resources.rs` | `🗃️resources.rs` | Resource residency |
| same | `🦀️scene_target.rs` | `🌫️scene_target.rs` | Offscreen mip-chain blur target |
| same | `🦀️types.rs` | `🧱️types.rs` | GPU memory-layout structures |
| same | `🦀️world3d.rs` | `🌐️world3d.rs` | World-space mesh and line encoding |
| `🎯️targets/🧊️webgpu/📦️packages/🦀️rust` | `🦀️backend.rs` | `🌐️backend.rs` | Browser WebGPU implementation |
| same | `🦀️buffers.rs` | `📈️buffers.rs` | Growable GPU buffers |
| same | `🦀️frame.rs` | `🎞️frame.rs` | Frame command replay |
| same | `🦀️gpu_context.rs` | `🔌️gpu_context.rs` | Device and surface construction |
| same | `🦀️gpu_types.rs` | `🔤️gpu_types.rs` | Pure GPU enum translations |
| same | `🦀️gpu_uniforms.rs` | `🧮️gpu_uniforms.rs` | Uniform numerical memory layouts |
| same | `🦀️pipelines.rs` | `🧵️pipelines.rs` | Shader render pipelines |
| same | `🦀️resources.rs` | `🗃️resources.rs` | Resource residency |
| same | `🦀️scene_target.rs` | `🌆️scene_target.rs` | Scene-color offscreen target |
| same | `🦀️surface_adapter.rs` | `🧊️surface_adapter.rs` | Owned WebGPU byte/page surface port |
| same | `🦀️surface_state.rs` | `🚦️surface_state.rs` | Surface and device state machine |
| `🎯️targets/🪟️d3d12/📦️packages/🦀️rust` | `🦀️backend.rs` | `🪟️backend.rs` | Direct3D implementation |
| same | `🦀️frame_buffers.rs` | `📬️frame_buffers.rs` | Per-frame uploads and descriptors |
| same | `🦀️hlsl.rs` | `✨️hlsl.rs` | HLSL shader source |
| same | `🦀️pipelines.rs` | `🏗️pipelines.rs` | Root signature and pipeline construction |
| same | `🦀️resources.rs` | `🗃️resources.rs` | Resource residency |
| same | `🦀️scene_target.rs` | `🌫️scene_target.rs` | Offscreen mip-chain blur target |
| same | `🦀️types.rs` | `🧱️types.rs` | GPU memory-layout structures |
| same | `🦀️world3d.rs` | `🌐️world3d.rs` | World-space mesh and line encoding |

## Verification

All 47 exact moves are applied, with their 47 Rust module mounts, direct descriptive references, and four root script frame/scene consumers repaired. All other module identifiers and behavior are unchanged. Frozen historical package-purity records remain untouched.

- Complete physical audit: 86 entries, 79 governed, zero findings. Fixed Cargo manifests are resolved with their inspected Rust package-root context. All 48 literal Rust module mounts resolve.
- `@semio-tech/ui-render-rs:test-quick`: 130 tests passed.
- `@semio-tech/ui-render-rs:boundaries`: all three forbidden dependency checks passed.
- Production `cargo check` through Nx: all four backend packages passed with all features on macOS. Vulkan/D3D12 and device-shaped browser code are platform-gated; this does not claim native Windows/Linux or browser GPU runtime validation.
- The existing WebGPU JS lifecycle test passed under Bun, producing create/resize/frame/drop and resource-limit results. It exposed one stale browser-host filename; its exact import now targets the existing UI host `🟨️.js`.
- The all-backend Rust test build is blocked by five existing Metal test calls to `Scene::finish`, which is only exposed by the core dependency under `#[cfg(test)]`. Those five calls also exist in read-only baseline `03100691d5`; no unrelated API semantics were changed.
- An additional Node run of the existing JS test exposed its pre-existing top-level use of class `Writer` before initialization. Bun passes; Node parity is not claimed. No test execution semantics were changed.
- Scoped `git diff --check` passed. No modifying Git commands were run.

The portable backend rerun excluding the independently blocked Metal tests passed: 158 tests across four binaries (core plus WebGPU/Vulkan/D3D12; platform gates apply). Evidence is under `🗑️generated/ui-render/`. Naming repair is complete for this subtree; the two independently identified pre-existing test limitations above remain explicitly reported.
