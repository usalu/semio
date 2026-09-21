# Component Close Coexistence Boundaries

Read-only source audit; no build or runtime probe was run.

## Confirmed global liveness fault

[Winit build](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:239) pumps applies and native asset/decode, then returns when [advance_component_surface_close](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:694) reports unfinished work. The return skips event draining and next-frame admission. [redraw_core](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:187) still calls present_snapshot afterwards, so B only re-presents its prior snapshot. It cannot consume new input or publish a new snapshot while A is closing.

## Required one-time admission fence

A close must first quiesce every pre-close frame that can own A. A present cursor owns AppFramePresentation and a bounded EngineCanvasPacket list; its Engine phase realizes those packets ([storage](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:14181), [realization](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:15112)). The existing initial presenter pending check is necessary.

Current close admission does not inspect a live FrameBuildHandle, even though a frame build owns EngineCanvasPackets before their presentation transition ([build resources](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:13154), [packet transfer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:15912)). No target-membership query exists. Before A's resources are detached, drain or retire this capacity-one pre-close frame build too. This is a bounded admission fence, never an external-wait barrier.

Once retirement begins, A is already rejected from new work: exact state is transferred by [retire_scene_identity](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1366), and scene intents/canvas gestures reject the retiring host ([Interpreter](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2215), [Scenes](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1836)).

## Exact coexistence contract

| Phase | Exact ownership | Rule after the admission fence |
| --- | --- | --- |
| Asset | Target filtered to A World or Icon token ([renderer](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12068)) | Awaiting external A cancellation must not block B. A remains ineligible to publish. |
| World | A host token is taken from world3d_states; only A diagnostics/watches are removed ([Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23189)) | Runtime lock contention yields; no B state is changed. |
| CPU engine | Exact EngineSurfaceToken marks closing and removes exact staged identity ([EngineCanvas](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2023)) | B may build with a distinct live token. |
| GPU engine/raster | Exact slot generation and A host raster key ([GPU close](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1483), [raster key](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:15432)) | One UI-thread step serializes close or presentation. B presentation may alternate only if its packets contain no A token. |
| Witness/ack | CPU, GPU and raster terminal checks before terminal bridge publication ([OsHost](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:429)) | A's token cannot be reused before the exact terminal acknowledgment. |

The bridge is capacity-one ([request](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:265)). It may serialize closes, but it must not serialize B input, frame construction, or presentation.

## Coordinator outcome

Use distinct close outcomes:

- AdmissionBarrier: drain the pre-close build/presentation.
- Progressed: run one A-only close unit, then continue normal B work.
- AwaitingExternal: retain exact A ownership and request its wake, then continue normal B work.
- Terminal: publish A's bridge token.

Do not close A CPU/GPU resources while a pre-close A packet is live. Do not use A Busy as a global build return once the admission fence finishes.

## Fail-first law

Extend the existing two-World fixture in [wgpu-os-host-standalone](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs:69) and its real stalled-body test at [line 407](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs:407).

1. Present A and B; request A close and finish only admission quiescence.
2. Hold A awaiting external completion.
3. Enqueue real B pointer/key input and drive one redraw.
4. Require B dispatch plus a newer B snapshot/frame generation while A remains close-pending.
5. Require A is absent from B packets/actions and its token is not reused.
6. Release A; require exact terminal acknowledgment and no residual close owner.

It fails at present because Winit returns before step 3 drains. It also rejects an unsafe removal of that return if an old A packet reaches GPU realization.

Confidence is high for the liveness failure and token isolation. The live-frame-build hazard is source-proven ownership without an execution receipt; treat it as a safety invariant and test it before relaxing the barrier.
