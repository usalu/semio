# Renderer PNG Original Context

Paths below are relative to 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu.

- 🧊️renderer/🔁️asset-decode-jobs/🦀️.rs:299–315: RendererAssetDecodeJob::step receives original StepContext and calls probe.step() at 314. Probe step calls GlbMaterializeCursor::step(owner,pages) at 142; both intermediate signatures currently omit context.
- 🧊️renderer/🔁️asset-decode-jobs/🦀️.rs:235–241: probe.close_step calls materializer.close_step. 🧊️renderer/🦴️glb-decode/📖️materialize-readers-and-component-helpers/🦀️.rs:192–196 cancels and drops the PNG job without funded close.
- 🧊️renderer/📮️mailbox-impl/🖼️asset-decode-pump-and-reference-images/🦀️.rs:173: no-context pump_renderer_asset_decode_step closes a probe. Its caller 📮️mailbox-impl/📥️renderer-asset-steps/🦀️.rs:227–228 already has cx.
- Same mailbox file:194: no-context close_renderer_asset_probe_step is called by close_renderer_asset_step(cx) at 319–333. Browser-worker/🦀️.rs:157 and frame-presentation/🎞️present-state-and-engine-closers/🦀️.rs:252–254 pass their original cx to that close.
- Same mailbox file:299: close_component_asset_step(target) lacks cx; its production caller 🏠️os-host/🦀️.rs:289–295 is close_asset_world_step(runtime,cx), so an original context exists one layer up. Each bool/enum-only seam needs refusal propagation when threaded.
- Direct no-context test calls occur in 🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🧱️glb-materials-and-independent-decoder-jobs/🦀️.rs:82,93,145,178; 🧊️glb-world-mesh-swapchain-and-original-decoder-close/🦀️.rs:57,83,133,148; 🖼️raster-contract-upload-cache-and-asset-probe/🦀️.rs:363,403,410–413; 🗜️reference-decode-staging-and-runtime-mailbox/🦀️.rs:36,62,72,81; and 👷️decoder-worker-session-recovery-and-asset-transfer/🦀️.rs:194. Existing latter suite uses a real caller.turn context nearby at line 183.

No native compilation or edits were made. Test callsite list is from a renderer test source scan, not a compiler verdict.
