# Presented Scene-Capture Rebase Audit

## Scope

Read-only audit on 2026-09-21 after the presented-input tree/router split. No production edit or test run was performed. This report concerns capture continuity through a **GPU-acknowledged ordinary same-host refresh**. It does not relax replacement or close invalidation.

## Confirmed failure

**Confidence: high.** The UI correctly maintains two independent arenas and explicitly remaps the router's captured node from presented to candidate before presentation:

- `Ui::acknowledge_presented_input` swaps candidate `tree/router` with `presented_tree/presented_router` only after the matching witness ([UI engine:1127-1153](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs)).
- `EventRouter::transfer_interaction_from` maps presented `NodeId` through the stable document binding into the candidate tree, and accepts it only when `interaction_identity_matches` ([UI events:1119-1148](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs)).
- That identity checks key, component generation, widget kind, and component `host_id` ([UI tree:394-405](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs)).

The renderer's separate `SCENE_POINTER_OWNERS` cache does not receive the remap. `ScenePointerTarget` includes an arena-local `node`, while `same_component_host` intentionally omits that field for resource ownership ([Interpreter WGPU:559-575](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)). Yet `scene_pointer_target_is_live` requires the captured node to resolve in the current presented tree exactly ([643-655](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)). After an ordinary ACK, the current presented tree is the candidate arena, so an otherwise identical captured target can no longer resolve.

This is a behavioral loss, not merely stale cache retention:

- `captured_scene_pointer` filters the old target through that exact liveness predicate ([675-677](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)); `release_scene_pointer` does the same after removing it ([679-684](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)).
- Shell checks published ownership using complete `PartialEq`, which includes the old node, before dispatch ([Shell WGPU:13560-13572](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs)).
- `AppInteractionState` resolves a captured pointer for move ([renderer WGPU:15690-15697](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs)) and releases it before special-surface up ([15612-15638](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🦀️.rs)). World3d, NodeGraph, TiledMap, and Board2d therefore lose captured move/up after the ACK. World’s state/cache itself remains correctly host-owned, which isolates the defect to dispatch identity.

`candidate_scene_target_in` already constructs the staged candidate target ([Interpreter WGPU:635-639](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)); staged hit registration already uses it ([776-779](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)). The missing operation is rebinding existing capture to that exact candidate node at acknowledgement.

## Bounded production seam

Do **not** change `scene_pointer_target_is_live` into a host-only boolean. Downstream event and scene dispatch need a live candidate `NodeId`; a boolean would admit an old arena node to new-tree operations. Do **not** scan every document for each captured pointer either.

Extend the existing document binding with a node-owned optional protocol `UiNodeId` and an O(1) reverse table. Expose one narrowly scoped UI query for a sealed witness:

```rust
candidate_scene_node_for_presented_node(witness, window_id, presented_node) -> Option<NodeId>
```

It must use the same relation already used by `EventRouter`: presented node → protocol `UiNodeId` → candidate node. The result is valid only if the window participates in `witness`, its candidate is sealed and ready, and the source/candidate nodes satisfy `interaction_identity_matches`. The query must not expose arena IDs in a wire value or diagnostic API.

In Interpreter's `acknowledge_presented_input` wrapper ([3972-3974](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs)), prepare the at-most-16 `SCENE_POINTER_OWNERS` replacements under the same UI borrow, then acknowledge UI, then replace only the successful cache entries with targets produced through `candidate_scene_target_in`. Require `old.same_component_host(&candidate)` before replacement. This preserves exact `window_id`, window generation, `host_id`, wire `surface_id`, kind, key, and component generation while changing only the arena-local node.

The Shell promotion then swaps staged scene-hit targets at the same acknowledgement boundary ([Shell WGPU:13250-13272](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs)). A captured target and published target remain exact-equal after the rebase.

### Required non-participation and invalidation rules

1. A captured owner in a window absent from the witness remains untouched. Its presented tree does not swap, so rebasing or cancelling it would manufacture a regression for an unchanged/hidden window.
2. If the participating candidate has no mapping, or key/kind/component-generation/host identity differs, drop the capture without a successful up. The existing replacement law remains authoritative.
3. A failed, stale, or aborted witness must mutate neither the UI router capture nor `SCENE_POINTER_OWNERS`.
4. Keep the fixed `SCENE_POINTER_OWNER_CAPACITY = 16`; map only occupied slots. No per-document walk or unbounded copy belongs in the ACK path.

## Fail-first law

Extend the real `scene-pointer-owner` fixture and its native/Chromium use with this sequence:

1. Present document records `A + B`; acknowledge them.
2. Press physical pointer one on overlapping World `B` and verify `B` owns capture.
3. Reconcile and prepare `A + C + B`, where `C` is a distinct inserted record. Assert the stable protocol identity of `B` is unchanged while its candidate arena `NodeId` differs from the captured node. This makes the rebase necessary without relying on an arbitrary allocator coincidence.
4. Acknowledge the prepared frame, move the captured pointer outside every surface, then release it.
5. Assert that only `B` receives the move/release terminal effects; `A` and `C` receive zero. Assert capture is retired.

The existing Shell entry point is `retained_world_sequence_probe` ([wgpu-shell-input:94](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:94)); its current `refresh` and `replacement` scenarios are the correct neighboring cases ([26-33](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs)). The browser counterpart should execute the same neutral sequence against keyed React World hosts: inserting sibling `C` cannot cancel DOM pointer capture from keyed `B`, while replacing `B` must not emit a successful terminal action.

Canvas2d’s generic event route already has the UI router’s protocol-node rebase. Its separately registered `SCENE_POINTER_OWNERS` still needs the same acknowledgment update for ownership accounting, but this audit only proves direct lost terminal routing for the four `AppInteractionState` bespoke kinds named above. Treat any Canvas terminal regression as a separate real-route law rather than inferring it from the World result.
