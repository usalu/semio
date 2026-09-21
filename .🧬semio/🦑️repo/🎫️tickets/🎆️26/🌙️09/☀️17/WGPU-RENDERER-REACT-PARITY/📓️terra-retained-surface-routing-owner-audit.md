# Retained Surface Routing and Window Owner Audit

**Scope:** read-only source audit of the retained pointer owner boundary, the four bespoke scene surfaces, and the per-window Actions, Search, and Measures panes.

**Evidence limit:** the fresh checkpoint-17 generated logs and JSON were removed during ticket cleanup before this audit completed. The runtime observation supplied for step 21 was that an Abort action left `active_window_id` at `puzzle3d-main-top/framework.section.engagements`. The source below independently predicts that result. No native build or runtime journey was run for this audit.

## Decision

Two independent defects remain.

1. The recent `World3d` exception is only a partial scene-owner repair. `NodeGraph`, `TiledMap`, and `Board2d` are live bespoke surfaces with dedicated press, move, release, cancellation, and wheel paths, but their retained hit rows still classify as Chrome before those paths can run.
2. Measures is the only host pane normalized to a parent window. Actions and Search can become `active_window_id` and `windowId` action arguments themselves. Extending the existing Measures suffix parser with a fixed scan would still select stale focus arbitrarily and would prevent an open panel from owning keyboard focus.

The owner of a host pane and the surface that owns a focused retained node are separate identities. Preserve both.

## Scene pointer ownership

The authoritative hit registration is `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:698-710`.

| Surface kind | Registered hit | Control id | Dedicated state and dispatch |
| --- | --- | --- | --- |
| World3d | `HitKind::World3d` | `surface_id` | `world3d_states` |
| NodeGraph | `HitKind::ScrollRegion` | `{surface_id}.pane` | `node_graph_states` and node-graph pointer dispatcher |
| TiledMap | `HitKind::ScrollRegion` | `{surface_id}.map` | `tiled_map_states` and tiled-map pointer dispatcher |
| Board2d | `HitKind::ScrollRegion` | `{surface_id}.pane` | `board2d_states` and board pointer dispatcher |

Every document control registered by `register_retained_body_hits` is entered in the double-buffered `retained_hit_windows` map (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12958-12982`). Current `pointer_owner_at` gives modal and open-panel precedence, then returns Chrome for any such retained hit except `HitKind::World3d` (`:13278-13286`). Therefore:

- World3d now bypasses the retained-body Chrome result.
- NodeGraph, TiledMap, and Board2d still return Chrome because their ScrollRegion records remain in that map.
- The renderer takes the Chrome path before the dedicated scene paths. The latter consequently cannot start a capture, receive later move/up, or receive a wheel.
- The same owner predicate blocks `wheel_reaches_scene_surface` before its otherwise correct `.pane`/`.map` propagation logic (`:13130-13145`, `:13175-13180`). `handle_pointer_wheel` must likewise not dispatch a scene hit as a generic retained-document scroll before the scene wheel path sees it (`:13329-13362`).

The distinct hit kinds are an implementation detail, not a distinct ownership rule. All four are active engine surfaces; the existing `world3d_states`, `node_graph_states`, `tiled_map_states`, and `board2d_states` are the authoritative live state census (`:13151-13155`).

### Required scene repair

Publish an explicit, bounded **bespoke scene-hit provenance** together with `InputState` and `retained_hit_windows`. It should map the exact registered control id to the registered surface id and kind for the four bespoke engine kinds. The hit provenance must be staged and atomically promoted with the current hit registry, as `retained_hit_windows` already is.

The ownership order then becomes:

1. modal overlay or the opaque open-panel rectangle => Chrome;
2. a published live bespoke-scene hit => Surface;
3. any other published retained-body hit => Chrome;
4. static hit classification.

This preserves panel and modal precedence, regular retained controls, and the existing pointer-capture rule. It also removes the current `World3d` one-off as soon as the provenance map exists.

Do not use a global `ends_with(".pane")` or `ends_with(".map")` test as the ownership repair. Those spellings are legacy registration encodings, can collide with document controls, and duplicate the canonical registration decision. State-map membership can serve as a small interim proof, but published registration provenance is the exact, frame-consistent contract.

### Scene fail-first laws

Add one physical complete gesture for each of World3d, NodeGraph, TiledMap, and Board2d:

- after a complete hit publication, a centre hit inside the live surface and also in `retained_hit_windows` resolves Surface;
- primary down, move, and up invoke only that surface's dedicated route and establish then retire its capture;
- a wheel reaches the same dedicated surface route;
- a modal and an open panel covering that same coordinate resolve Chrome and invoke none of the four scene routes.

The existing `🐚️Shell/🧪️tests/🎯️wgpu-pointer-hit-ownership/🦀️.rs` is the correct native owner-boundary suite to extend. Keep its actual Shell/renderer ingress rather than testing only `pointer_hit_owner`.

## Window-owner and focus boundary

Host constructors are authoritative:

- Measures: `window_measures_surface_id` in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4289-4314` creates `{window}/framework.section.measures`.
- Actions: `window_actions_surface_id` at `:18368-18375` creates `{window}/framework.section.engagements`.
- Search: `window_search_surface_id` at `:18377-18385` creates `{window}/framework.section.engagements.search`.

The source has four callers that normalize only Measures:

| Ingress | Current behavior | Defect for Actions and Search |
| --- | --- | --- |
| `route_retained_pointer_press` down (`:13085-13095`) | Measures suffix becomes owner before `dock.sync_active_window` and `active_window_id`. | Pane identity becomes the dock and active-window identity. |
| same route release (`:13117-13121`) | `scope_action_to_window(action, window_id)`. | Guest receives pane id in `args.windowId`. |
| `dispatch_action` (`:10039-10042`) | defensive normalization recognizes Measures only. | Direct or deferred Action/Search ingress remains malformed. |
| accessibility focus (`:13391-13407`) | Measures focus activates its parent window. | Action/Search focus activates their pane surface. |

Keyboard has a fifth gap. `retained_keyboard_focus` only chooses `active_window_id` or that window's Measures surface (`:12330-12335`). `content_focus` stores a node per arbitrary retained surface but contains neither focus order nor a global focused-surface pointer (`:1344-1353`, `:12276-12294`). Adding three fixed branches would make a stale entry choose an arbitrary host pane and still leave a focused panel unrouteable.

The ordinary `Window` DOM model is capture-phase activation of its containing window. A host pane focuses its exact child surface, while the enclosing window remains the active application window. A panel remains outside that application activation model, yet a focused input in an open panel must still own keyboard input.

### Required shared owner and focus repair

1. Add one strict helper for the four host surfaces:
   - the primary body `window`;
   - `window_measures_surface_id(window)`;
   - `window_actions_surface_id(window)`;
   - `window_search_surface_id(window)`.

   It returns the concrete window instance only for these exact, currently mounted host surfaces. It must not normalize a generic `framework.section.*` string or an app-authored surface. Keep `retained_surface_is_panel` as the separate panel decision.

2. Use the helper consistently in the three owner transitions:
   - retained pointer down: sync dock and set `active_window_id` to the concrete instance;
   - retained release: call `scope_action_to_window` with that instance;
   - `dispatch_action`: defensively replace a host-pane `args.windowId` with that instance;
   - accessibility focus: activate the concrete instance for a host pane.

   A panel continues to skip dock/active-window activation.

3. Keep `content_focus: surface -> node`, but add `focused_retained_surface: Option<surface>` updated only by real `UiCommand::FocusChanged`:
   - `Some(node)`: record the node and set the tracker to that surface;
   - `None`: clear that surface and clear the tracker only if it names that surface.
   
   `retained_keyboard_focus` reads the tracked surface and its node after validating the surface is still live. This supplies the exact focused Actions/Search/Measures/body surface and allows an open panel to receive keyboard input without changing `active_window_id`.

4. On a pointer down in one host surface, clear focus for the other three host surfaces of the same parent window. If the tracker names one of those siblings, clear it as well. The dispatched retained event then supplies the new `FocusChanged`. This prevents stale Search/Measures focus but does not blur a panel or a different window merely because another window was activated.

5. Clear the tracker when a focused document is closed, retired, hidden, or replaced. The live check must accept an active dock body, a currently mounted host pane, or an open panel document only. It must reject a stale map entry.

This is a narrow state addition: focus is already event-derived, and the event's `window_id` is the actual retained surface. It avoids inferring focus from an unordered map or from owner string suffixes.

### Window-owner fail-first laws

The neutral fixture `🐚️Shell/🧫️fixtures/🪪️window-surface-owner/🔣️.json` already expresses the four cases: body, Measures, Actions, Search. Its native test route is `🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:34-85`.

For every fixture case, drive real retained pointer, accessibility, and key ingress and assert:

1. active shell window and active dock stack equal `scope-window`, never any `/framework.section.*` surface;
2. the focused keyboard surface equals the exact pressed/focused surface;
3. a row action, text commit, and Abort action all carry `args.windowId == "scope-window"`;
4. changing focus from one host surface to another clears the former focus, and key input reaches only the latter;
5. focus inside an open panel leaves `active_window_id` and dock state unchanged but routes keyboard input to that panel; closing it makes the panel ineligible for keyboard routing;
6. a panel tab remains Chrome and cannot be treated as a host-pane owner.

The React oracle belongs beside the existing mounted `WindowActionPane` contract coverage in `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`: render two real Window instances, activate an Actions/Search child via pointer capture, then assert the captured activation and emitted action use the enclosing instance id. The same shared JSON fixture should drive body, Measures, Actions, and Search. A React result reported during this audit was green for the four rows; I did not run that suite myself.

## Confidence

**High** for both source-level defects and the required ownership/focus contracts: the registration, retained map, owner predicate, and every Measures-only normalization site are direct current-source evidence.

**Medium** for the exact fresh-journey impact of NodeGraph, TiledMap, and Board2d: their starvation follows deterministically from the code paths, but the checkpoint-17 generated artifacts were unavailable for direct reinspection after cleanup. No test or native compile was run by this audit.

