# Native135 World Lifecycle and Retained-Surface Fixture Triage

## Evidence

Read-only review of [Native135 receipt](./🗑️generated/astra-runtime/renderer-native135-full/failures.json), [run log](./🗑️generated/astra-runtime/renderer-native135-full/run.log), the [World lifecycle law](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🪟️window-lifecycle-template-drag/🦀️.rs:115), and the [cross-kind hit law](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs:20). No test was run.

## World close failure is incomplete fixture ownership

The World lifecycle failure is a fixture setup defect, not evidence that production close fails.

The test directly inserts two World3d states but never inserts the required host-to-window rows in world3d_window_ids. Production registration is the writer of that map in [mirror_engine_surface_states](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7535). Close retirement enumerates exactly that map at [Shell lines 7366–7370](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7366). Without the row, the direct fixture state is intentionally ineligible; this exactly explains the failed assertion that the old entry was removed.

Production does remove the active state before input, transfer it into the bounded retirement queue, reserve its admission credit, and clear the window-scoped projections at [Shell lines 7369–7399](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7369). It releases that reservation only after the retained state reaches terminal empty at [Shell lines 23178–23201](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:23178).

Correct the fixture by using component-host IDs for state-map keys and inserting exact host-to-window rows before calling sync. Assert deletion by host ID, stale token rejection, per-window map removal, and a fresh token on re-admission. This matches the current contract: component host IDs own renderer state; window IDs own dock and projection state.

## Cross-kind retained-hit failure manufactures an invalid target

The engine-surface retention test correctly drives the real self-less retention function, but it then fabricates an arena NodeId and ScenePointerTarget, uses the authored surface ID as host ID, injects it into staging maps, and asks Shell to promote it ([test lines 45–70](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs:45)).

That target is not an accepted UI ComponentScene. Shell correctly rejects it: [retained_scene_hit_is_live](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12931) calls [scene_pointer_target_is_live](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:706), which requires the accepted tree to match node, key, host, component generation, and document ID. The temporary test arena cannot satisfy that. The observed Chrome result is therefore correct.

Do not weaken the live-target check or add a test-only bypass.

Factor the test into two laws:

1. Keep the neutral fixture and SurfaceMirror path for state retention only. Assert each expected graph, Map, or Board state row's stored window, controller, and bounds. This continues to prove creation, no-repaint retention, refresh, and close removal through mirror_engine_surface_states.

2. Put pointer and wheel assertions in an actual retained-document path. Existing [paint_tree_pointer_documents](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:769) paints real ComponentScene records, registers clipped targets, seals, and ACKs. [retained_world_sequence_probe](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:299) is the working World pattern. Add Graph, Map, and Board rows through that same path before asserting pointer_owner_at or wheel routing.

## Related stale host-key assertion

The Display test at [Shell input line 1558](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:1558) assumes world3d_states is keyed by window ID main-2. It is host-keyed after the migration. After acknowledgement, resolve the published ScenePointerTarget and assert state-map membership by target.host_id, while independently asserting target.window_id equals main-2.

## Confidence

High. Each failed assertion follows an exact absent fixture owner or an intentionally invalid fabricated target. This audit found no contradicted production World close sequence.


