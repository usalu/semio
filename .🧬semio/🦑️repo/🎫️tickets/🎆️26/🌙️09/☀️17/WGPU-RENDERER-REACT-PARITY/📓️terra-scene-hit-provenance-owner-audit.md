# Retained Scene-Hit Provenance Audit

**Scope.** Read-only source audit after the paired retained-scene registry was introduced. No Cargo, browser run, or production edit was performed. Line references describe the source observed on 2026-09-21; concurrent implementation work may move them.

## Finding

The new `RetainedSceneHit` chain correctly makes the **Shell ownership gate** source-aware, but it does not yet make the **native dedicated dispatcher** source-aware. A valid retained `NodeGraph`, `TiledMap`, or `Board2d` hit now changes `pointer_owner_at` from `Chrome` to `Surface`; the renderer then scans every live bespoke state whose rectangle contains the point. Two overlapping live surfaces therefore receive the same press, move, release, or wheel application. This is a live implementation gap, not a stale fixture issue.

The old suffix authority remains a second bypass: any raw `ScrollRegion` named `*.pane` or `*.map` can still be considered a scene wheel hit without a paired record.

Confidence: **high** for both source paths. Runtime reproduction was intentionally not performed.

## Authoritative data path

1. [`input/🦀️.rs`](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs) defines `RetainedHitRegistration { node, rect, overlay, scene, kind, control_id, ... }` at about lines 655–679. `retained_hit_registration` records `Some(RetainedSceneHit { surface_id, kind })` only for `World3d`, `NodeGraph`, `TiledMap`, and `Board2d` component nodes (about lines 791–797). Its canonical kind/id mapping remains World=`World3d`/id, graph+board=`ScrollRegion`/`.pane`, map=`ScrollRegion`/`.map` (about lines 712–721).
2. The UI engine produces those registrations in a separate `Hits` walk after retained painting and publishes the completed vector at [`engine/🦀️.rs`](../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs) about lines 1706–1726. This is the right source of scene provenance.
3. [`Interpreter/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs) `register_clipped_retained_hit_targets` (about lines 557–578) preserves the optional scene record and returns the **clipped emitted rect**. It omits clipped-out non-overlay registrations. This is the correct seam for a paired registry.
4. [`Shell/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs) stages `retained_hit_windows` and `retained_scene_hits` together (about lines 13018–13027), swaps both before `InputState::publish_hits` (13034–13037), and validates the current hit by control id, clipped rect, body/window owner, canonical kind, and live bespoke state (12993–13005). `pointer_owner_at` consumes that validation (13339–13347). This addresses the former Graph/Map/Board `Chrome` classification defect.

## Required repair boundary

Represent the result of validation as one internal, immutable route value, for example:

```rust
ScenePointerTarget {
    window_id: String,
    surface_id: String,
    kind: SurfaceKind,
    rect: Rect,
    input_generation: u64,
}
```

`ShellState::scene_target_at(x, y, input, theme)` must return it only from the currently resolved, paired hit after the existing validation. `pointer_owner_at` derives `Surface` from `Some(target)`; it must not independently infer a scene from a suffix or from a rectangle scan.

The native renderer must use that target in all dedicated ingress:

- press: claim and call only the target's World/Graph/Map/Board entry point;
- move: use the captured target while a sequence is active, then use the current target only for hover without capture;
- release: take the captured target and call only its `*_pointer_up_into` path;
- cancel: retain the existing exact `cancel_scene_pointer` route;
- wheel: resolve one fresh target per notch and call only that target's `*_wheel_into` path.

No `values().filter(|state| state.bounds.contains(...))` loop may choose a bespoke target after that point. Bounds remain a final live/geometry guard inside the selected target, not selection authority.

### Current fan-out sites

[`renderer/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs) has the production fan-out:

- pointer release: about lines 15789–15843 loops map, board, world, then graph by bounds;
- pointer press: about lines 15869–15953 loops world, graph, map, and board by bounds;
- move: about lines 15975–16041 loops all four kinds by bounds;
- wheel frame phases: `WheelWorld3d`, `WheelGraph`, `WheelMap`, and `WheelBoard` each scan the corresponding state map by bounds (about lines 13375–13580).

A `PointerHitOwner::Surface` is only a two-way layer choice. It is insufficient to identify which surface receives a dedicated interaction.

### Capture lifecycle

[`Interpreter/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs) already stores `(pointer_id, window_id, window_generation, surface_id, kind)` in `ScenePointerOwners` (about lines 430–503), and cancel removes/returns the exact owner only after checking the originating document generation.

The renderer currently calls `release_scene_pointer(pointer_id)` **before** its release scans (renderer about line 15743). Add a `take_scene_pointer_owner(pointer_id)` operation with the same generation check as cancellation. On release, take first, dispatch exactly that owner, then do not re-hit-test a dedicated target. A stale/missing owner performs no scene release. This preserves DOM pointer-capture semantics for a press that leaves or crosses an overlapping surface.

## World3d origin gap

Graph/Map/Board validation checks `state.window_id == paired.window_id`. World validation at Shell about line 13001 checks only `surface_id` and bounds because `World3dState` exposes no independent origin window. The scene painter does have the origin at [`Scenes/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs) 2648–2660 and registers it in `EngineSurfaceRegistration { surface_id, window_id, ... }` at [`EngineCanvas/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs) 2058–2065 and 2400–2402.

Store that window identity with World3d's live shell state and require it in `retained_scene_hit_is_live`. Better still, make `(window_id, surface_id)` the identity at every state registry boundary. The source does not establish global uniqueness of a `surface_id`; treating it as globally unique would be an unstated policy and would not prove exact ownership.

The immediate map entry may use `World3dState`'s existing paint-derived window field only if it is explicitly the live owning-window contract. Its current `tool_run_trace_window_id` name and purpose do not establish that contract.

## Remove suffix authority

`scroll_region_is_scene_surface` at Shell about lines 13191–13193 is still referenced by:

- `wheel_propagates_to_scene_surface` (13195–13205),
- `wheel_reaches_scene_surface` (13236–13240), and
- `handle_pointer_wheel` (13421–13422).

A raw, unmapped `HitTarget { kind: ScrollRegion, control_id: "forged.pane" }` is not classified as chrome by the static classifier. It therefore reaches the suffix branch even though no canonical retained component node minted a scene record. Replace all three with `scene_target_at`; only a validated target may suppress retained scrolling and enter a dedicated wheel route. Update older tests that create bare `.pane`/`.map` targets to publish the paired record through the real registration bridge.

## Fail-first regression laws

Add the following to a renderer-owned test module, because the defect is in the production native dispatch, not only Shell classification.

1. **Exact press:** attach two real bespoke hosts with overlapping `Rect`s and distinct `window_id`, `surface_id`, and `controller_id`. Publish a canonical paired hit for A. Drive `AppInteractionState::handle_pointer_button(pointer, point, true, Primary, modifiers)`. Assert A's real host/action effect occurs and B emits no action/mutation.
2. **Captured release:** press A, then publish/hit B and release at B's point. Assert only A's actual `*_pointer_up_into` effect occurs; B receives no press or release. The exact `ScenePointerOwners` entry is consumed once. Repeat cancellation and assert the existing exact cancel only reaches A.
3. **Exact wheel:** use the same overlap with one canonical target per notch. Drive the frame wheel application route and assert only the selected host publishes its camera/interaction action. A raw `.pane`/`.map` hit with no paired provenance must publish no bespoke action.
4. **Provenance rejection matrix:** each of kind mismatch, clipped-rect mismatch, owner-window mismatch, stale source generation, missing live state, a normal retained body row, and an unmapped suffix row resolves Chrome / no `ScenePointerTarget`.
5. **World origin:** identical `surface_id` from a different window must be rejected, or the fixture must explicitly prove the system rejects the duplicate before it can register. Do not leave this implied by a map key.
6. **Duplicate control id:** a scene registration and an ordinary retained registration sharing a control id cannot be represented safely by a `HashMap<String, ...>` alone. Reject the collision at staging or key the paired entry by an unambiguous hit identity. The test must prove a non-scene row cannot borrow a scene record merely because `control_id`, rect, and kind coincide.

Use actual host effects, not call counters. The existing attachment seam is `engine_canvas::sync_node_graph_scene`, `sync_tiled_map_scene`, and `sync_board2d_scene` in [`EngineCanvas/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs) 2352, 2523, and 2768. The real fixture constructors and production attach/teardown ladder are in [`wgpu-engine-surfaces/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs) 31–166; node-graph's real down/move/up effect oracle is in [`node-graph-gestures/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🫳️node-graph-gestures/🦀️.rs).

For the native entry test, add a focused renderer test module next to the existing `wheel_application_point_tests` declaration in [`renderer/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs) about lines 9516–9520. It can construct the actual private `AppInteractionState`; [`wgpu-renderer-async-boundary/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs) 960–987 is the existing complete field initializer. Extract that initializer to a test-only `scene_interaction()` helper rather than duplicating it. This drives `AppInteractionState::handle_pointer_button` itself and catches the current post-gate fan-out.

## Existing test limitation

The newly added Shell law `retained_engine_hit_provenance_reaches_each_dedicated_pointer_and_wheel_route` in [`engine-surface-retention/🦀️.rs`](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs) currently proves `pointer_owner_at == Surface`, wheel admission, retained wheel non-consumption, and a forged owner rejection. It does **not** invoke `AppInteractionState` or observe a Graph/Map/Board/World mutation/action, so it cannot prove that the exact source reaches the dedicated route. Keep it as the Shell-layer law and add the renderer law above.

