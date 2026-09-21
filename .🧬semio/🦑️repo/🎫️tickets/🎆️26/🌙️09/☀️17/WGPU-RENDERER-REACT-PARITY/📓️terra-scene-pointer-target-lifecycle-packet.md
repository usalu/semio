# Scene Pointer Target Lifecycle Packet

## Scope

Read-only source audit on 2026-09-21. No build, browser run, or runtime mutation was performed.

## Correction

The former section "World3d lacks authoritative host ownership" is superseded and must not be implemented. Current canonical production projection makes `ComponentScene.surface_id` the owning document/window id, and every production `SceneEngineHosts.window_id` caller receives that same id. The equal ids in the World fixture therefore reflect the real contract, not a masking fixture. See `📓️terra-map-scene-identity-retirement-packet.md` for the source evidence and the corrected bounded Map cache-retirement work.

The current exact chain is real: UI scene registration records canonical surface identity; Interpreter mints ScenePointerTarget as window, surface, kind, node, key, and document generation; its clipped hit rows and Shell's owning-document map are promoted together; then Shell resolves one target after modal/panel precedence. Renderer carries that target through down, captured move/outside-up, and the WheelStart to WheelScene handoff.

Authoritative paths:

- 🖱️ui/🎯targets/🧊wgpu/🦀️.rs:788-804
- 🗣️Interpreter/🎯targets/🧊wgpu/🦀️.rs:432-467, 577-596
- 🐚️Shell/🎯targets/🧊wgpu/🦀️.rs:13012-13045, 13334-13345
- renderer/🧊wgpu/🦀️.rs:15616-15805

The old generic bounds fanout is gone. The following receiver and retirement gaps remain.

## Confirmed residuals

### Map cancellation commits normal pointer-up

Confidence: confirmed.

Shell handle_pointer_cancel_for invokes tiled_map_pointer_up_into for the captured Map at 🐚️Shell/🎯targets/🧊wgpu/🦀️.rs:12428-12447. The MapMarquee branch of 🎞️Scenes/🎯targets/🧊wgpu/🦀️.rs:7838-7904 writes interactionSelect, so cancellation can make a selection.

React's onPointerCancel at 🧭️TiledMapHost/🟦️.tsx:1183-1193 calls pointerUpScreen only for the map-session/pan side, clears left/middle state, resets the marquee, and does not call emitFeatureSelection. Native's combined up helper cannot represent cancellation.

Repair: add tiled_map_pointer_cancel_into next to map down/move/up.

- MapPan: issue host PointerUp once, mirror its camera action, then clear drag and marquee state.
- MapMarquee: clear only drag and marquee state; do not hit-test or write selection.
- No drag: no action.

Route only the exact cancelled target. Do not manufacture a successful up or cancel peers.

Fail-first law: through real AppInteractionState button/move/cancel, cancelled upper middle-pan yields one upper setCamera, no lower action, no selection; cancelled upper primary marquee yields no interactionSelect, clears tiled_map_drag_active, and accepts a following new down. A second cancel and old outside-up are inert.

Extend 🧑‍🎨engine/🧫️fixtures/🛑️scene-pointer-cancellation schema-first with distinct windowId and surfaceId, expected pan camera count, and zero primary-marquee selection. Its React oracle must prove no emitFeatureSelection, not merely that pointerUpScreen ran.

### Map hover has no leave route

Confidence: confirmed in native source; React support is present.

AppInteractionState handle_pointer_move clears World and Board peer hover but no Map hover at renderer/🧊wgpu/🦀️.rs:15724-15805. Map hover is cached as map_last_hover_json and emitted at 🎞️Scenes/🎯targets/🧊wgpu/🦀️.rs:7818-7833, so it survives entering another retained scene or chrome. React observes Map pointer movement at window scope in 🧭️TiledMapHost/🟦️.tsx:1106-1125, 1220-1228 and resolves a null hit after leaving.

Repair: add tiled_map_pointer_leave_into. If cached hover is non-null, emit one interactionHover with targets [] in the map's declared domain and store the null witness. Before selected-target move dispatch, call it only for a previously hovered map that is not the selected target.

This is hover retirement, not gesture fanout: it must not send down/up/wheel/cancel, change a drag, or emit peer selection.

Law: hover upper Map, move to World, Graph, or blank chrome, then move again. The first transition emits one upper empty hover; later moves emit none; the lower Map emits none. Captured drag move/outside-up still reaches only upper Map.

### World3d lacks authoritative host ownership

Confidence: confirmed; current fixture masks it.

Graph, Map, and Board retain window_id and compare it to ScenePointerTarget.window_id in every direct route. World instead carries optional tool_run_trace_window_id at ♾️infinite/🌍️world/🦀️.rs:1781-1784, overwritten by paint at 🎞️Scenes/🎯targets/🧊wgpu/🦀️.rs:2652-2661. World hit admission, button/wheel, move, and cancellation look up only surface_id at 🐚️Shell/🎯targets/🧊wgpu/🦀️.rs:13013-13027, renderer/🧊wgpu/🦀️.rs:15616-15805, and 🐚️Shell/🎯targets/🧊wgpu/🦀️.rs:12434-12439.

retire_closed_world3d_windows treats world3d_states map keys as window ids at 🐚️Shell/🎯targets/🧊wgpu/🦀️.rs:7325-7360, although paint keys that map by scene.surface_id. The scene-pointer-owner fixture uses equal windowId and surfaceId, so the test cannot detect this mismatch.

Repair: promote or rename the trace field to window_id identity, letting trace publication read it.

- Bind the host window on first World paint; later paint accepts only that window and rejects a rebinding.
- Require state.window_id equal target.window_id in every World direct route; an ownerless state cannot route.
- Retire World state by surface_id map key and clear shell projections by stored window_id.

This reuses the existing optional owned string and adds no capacity.

Law: publish two distinct window/surface pairs and try to attach the first surface under the second window. Reject the mismatch; no press, captured move/outside-up, wheel, or cancel reaches it. Closing the actual owner retires its state and old target.

### Per-pointer Interpreter ownership conflicts with a global app capture slot

Confidence: confirmed.

ScenePointerOwners is pointer_id keyed and has capacity 16 at 🗣️Interpreter/🎯targets/🧊wgpu/🦀️.rs:421-505. Shell PointerCapture is a single Option of PointerHitOwner at 🐚️Shell/🎯targets/🧊wgpu/🦀️.rs:3091-3120; app down state, button, and last coordinates are also singular. Pointer two overwrites pointer one's capture and pointer two release/cancel clears it.

Choose one coherent contract: refuse a second pointer before an Interpreter claim, or use fixed per-pointer app slots for layer owner, down state, button, last position, and modifiers with the same cap as the target table. The existing target table and browser pointer capture are per pointer.

Law: capture pointer 1 on upper scene, press/release pointer 2 on chrome and then another scene. Pointer 2 cannot change pointer 1 owner, coordinates, or later outside-up recipient. Each release/cancel retires only its own slot.

### Stale targets cannot dispatch but consume the fixed grant

Confidence: confirmed.

captured_scene_pointer filters stale targets at 🗣️Interpreter/🎯targets/🧊wgpu/🦀️.rs:492-494, and release/cancel removes only its own pointer entry. claim_scene_pointer_owner removes only the same pointer before testing its 16-item capacity at lines 470-480. Replacing a document or closing a window before release/cancel leaves a stale distinct-pointer slot that cannot receive input but can block new admission.

Repair: prune non-live owners before capacity test. Do not deliver cancellation to a replaced or closed document.

Law: claim 16 distinct ids, replace or close their documents through the actual lifecycle, then claim a fresh 17th id. The fresh target is admitted and the only receiver producing an effect.

### Target invalidation does not retire per-surface gesture caches

Confidence: cache lifetime confirmed; visible consequence needs the proposed regression.

🎞️Scenes/🎯targets/🧊wgpu/🦀️.rs:943-952 keeps SCENE_STATE keyed only by surface_id. Its Map drag and map_last_hover_json are reused by scene_state(surface_id), and the module has no removal or retirement path for SCENE_STATE. Shell document replacement retires UiDocumentLease through retire_one_surface_document at 🐚️Shell/🎯targets/🧊wgpu/🦀️.rs:7573-7595, but does not cancel its old ScenePointerTarget or clear that cache.

Target liveness correctly blocks old pointer up from entering the replacement. It does not clear old Map marquee, pan, hover, or Map-host state, all addressed by surface_id.

Repair boundary: at retained document generation/key/kind/surface replacement or removal, retire the old target before publishing its successor:

1. remove old pointer-owner slot;
2. cancel old receiver by old window/surface/kind/generation, never successor lookup;
3. clear its old cache: Map marquee without selection, Map pan by the cancellation rule, Graph/Board through their existing cancel APIs;
4. only then accept a new target.

This must run at the document replacement boundary while old identity still exists, not in the liveness query or as a synthetic up.

Law: begin Map marquee or pan, replace same window document with a different node key, then send old-pointer move/up/cancel. No successor selection/up occurs; old drag/hover cache is gone; fresh target begins exactly one new gesture. Repeat removal and Graph/Board cancellation.

## Receiver-effect tests

Use the private real AppInteractionState seam in 🐚️Shell/🧪tests/🔬️wgpu-shell-input/🦀️.rs. It paints retained documents, publishes the actual registry, and drives production button/move/wheel/cancel.

- World: upper queue only receives expected press/move/up/wheel; lower queue remains unchanged; cancel calls only upper relocate cancel.
- NodeGraph: only upper controller receives graph interaction/edit/camera rows; cancel changes upper host only and no successful edit occurs.
- TiledMap: only upper receives setCamera, hover, or selection as warranted; MapPan cancel has one camera effect and MapMarquee cancel has zero selection.
- Board2d: only upper receives applyBoardEvents, camera, or hover; cancel releases upper claim and a new upper down succeeds.

Exactly one owner applies to press, captured move, outside-up, wheel, and cancel. It permits one hover-retirement clear for a previously hovered peer. Board's existing leave helper correctly guards with board_pointer_inside at ⚙️EngineCanvas/🎯targets/🧊wgpu/🦀️.rs:5138-5168, so never-entered overlap emits nothing.

## Fixture and tests

Evolve 🧑‍🎨engine/🧫️fixtures/🪪️scene-pointer-owner schema-first with distinct windowId/surfaceId, kind, z order, overlap points, receiver effect expectations, and replacement/removal rows. Keep 🧑‍🎨engine/🧪tests/🧲️engine-surface-retention/🟦️.ts as independent Chromium topology/capture oracle. Native tests must observe concrete receiver actions and host state.

The previous source-string ingress anchors are already behavioral in the sampled tree: wgpu-pointer-hit-ownership, wgpu-shell-chrome-parity, and wgpu-wheel-and-escape-routing call retained_world_sequence_probe rather than looking for over_world. Do not replace them with new string needles.

## Execution order

1. Split Map cancel from up and add Map hover retirement.
2. Promote/enforce World owner identity and correct World retirement.
3. Retire old target/cache at document replacement and prune stale target slots.
4. Make app capture coherent with the existing per-pointer target contract.
5. Add real receiver effect plus replacement/removal laws while retaining Chromium as topology oracle.
