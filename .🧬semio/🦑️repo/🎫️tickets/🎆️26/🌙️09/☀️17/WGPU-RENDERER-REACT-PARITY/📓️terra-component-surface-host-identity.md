# Component Host Identity and Retirement Packet

Status: read-only source audit on 2026-09-21. No build, browser run, or source change was performed for this packet. Assertions about the existing Native102 and React receipts below are parent-provided evidence; the source findings are independently inspected.

## Decision

`surfaceHostIdentityV1` is the public document address. It is deliberately **not** a retained component-host address. The WGPU renderer needs a second, runtime-only identity for every mounted `Component::Surface`:

```text
SceneHostIdV1 = (document_generation, node.index, node.generation, component_generation)
```

Serialize that tuple into a fixed, checked `host_id` no longer than the existing 256-byte internal registry limit. It must be generated only at a retained mount, never supplied by the document, and must not appear in the UI contract, JSON payload, DOM attributes, or action arguments.

Keep `UiComponentSceneNode.surface_id` unchanged. It is the owning document's public `surfaceId`, and all dispatched payloads continue to publish exactly that value. `host_id` selects local CPU/GPU state; `surface_id` addresses the producer.

The preferred implementation is a runtime-only `UiComponentSceneNode.host_id: String` (`serde`/value skipped), stamped by WGPU document reconciliation after it has the allocated `NodeId` and final component generation. This is safer than recomputing a string independently at each state access. `ScenePointerTarget` and `UiRetiredComponentScene` carry the same stamped value and live validation compares it as part of the presented node identity.

## Why a document address cannot be the host key

The React contract is explicit: `surfaceHostIdentityV1(surface, key, recordId)` returns the owning document surface as `surfaceId`, with the authored leaf key separately in `paneId` ([Interpreter/🟦️.tsx](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:479)). The Rust projection has the same rule at [reconcile/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:461).

The tree walker emits every component-scene leaf recursively, not one scene per document ([scene_slots/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📨️scene_slots/🦀️.rs:210)). Thus sibling surfaces are valid and intentionally share the document `surface_id`.

The current mount implementation has exactly the wrong cardinality: `SCENE_STATE[owner.surface_id]` holds one `mount_owner` and rejects a different target ([Scenes/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1135)). A second sibling therefore faults on its first paint. The same document-key collision exists in `ensure_engine_surface(&scene.surface_id, …)` for NodeGraph, TiledMap, Board2d, Paint2d, and TextEditor ([EngineCanvas/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1946)).

`pane_id` alone is insufficient: it is authored data and does not advance when an existing retained node is replaced. The document generation plus arena node generation plus component generation make an old capture, staged raster, or close acknowledgement incapable of naming a successor.

## Required ownership split

| Owner family | Internal key after this repair | Public wire address | Exact current seam |
| --- | --- | --- | --- |
| Generic scene state, list scroll/selection, Canvas2d camera, Ink state, Map local state | `host_id` | `scene.surface_id` | `SCENE_STATE`, camera deadlines, and `SceneListTransfer` in [Scenes/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1014) |
| Canvas gestures and catalogue hover | captured `host_id` plus pointer and document generation | Canvas action `surfaceId = scene.surface_id` | `CanvasGestureSlots` and `CanvasCatalogueHover` at [Scenes/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1601) |
| Canvas image/raster upload queue | `host_id` carried at reservation, checkout, and completion | image producer/action still uses canonical `surfaceId` | `PENDING_RASTER_STATE` and `PendingRasterCheckedOut` at [Scenes/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:767) |
| NodeGraph, TiledMap, Board2d, Paint2d, TextEditor CPU host and staged texture | `host_id` | each action builder receives the original scene and keeps its `surface_id` | `ENGINE_SURFACES`, `STAGED_ENGINE_SCENES`, and raster-key calls in [EngineCanvas/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1946) and [EngineCanvas/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2987) |
| Shell retained hit/controller mirrors for NodeGraph, Map, Board | map key `host_id`; value retains canonical `surface_id` for publication | `surface_id` | `mirror_engine_surface_states` currently uses registration `surface_id` as the map key at [Shell/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7514) |
| World3d and IconRender world state | `host_id` | World action/status payload retains canonical `surface_id` | `render_world3d_surface_step` presently stores `world3d_states[scene.surface_id]` at [Scenes/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3085) |
| Text editor popups/focus and Ink focus/clipboard | `ScenePointerTarget.host_id`, not a raw surface string | Text/Ink action `surfaceId` stays canonical | `FOCUSED_TEXT_EDITOR` starts at [Interpreter/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1496); Ink focus/clipboard starts at [Interpreter/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1557) |

This means every parameter now named `surface_id` in an engine/local-state lookup must be classified. Rename its local argument to `host_id` if it selects `ENGINE_SURFACES`, `SCENE_STATE`, `TEXT_EDITOR_UI`, a raster queue, a map/board host, a staged texture, or a shell mirror. Do not rename an action payload field, document parser, producer operation, or React contract field.

## Minimal data path

1. Expose `NodeId`'s index and arena generation as owned scalar accessors. At a component mount or replacement, reconciliation stamps `host_id` from the current global document generation, those scalars, and the resulting component generation. The initial and replacement branches must stamp after the generation is known. An unchanged same-key/same-kind component preserves its host id.
2. Copy the stamped value into `UiRetiredComponentScene`; `ScenePointerTarget::from`, live-target comparison, scene captures, queued intents, focus records, and close records preserve it. `UiComponentSceneNode` serialization/value conversion skips it.
3. Change `FrameworkSceneHost::paint_slot_step` to pass the just-built `ScenePointerTarget` into `render_component_scene_step`. Pass that target, or its `host_id`, through the local-state and EngineCanvas helpers. Test-only direct paint helpers construct an explicit target/host id; no empty-string compatibility path is valid.
4. Make each EngineCanvas sync function accept the host key. `ensure_engine_surface`, `stage_engine_scene_paint`, `engine_raster_key`, all host mutation/query functions, and `EngineSurfaceRegistration` use that key. Registration needs both `host_id` and canonical `surface_id`, plus the exact CPU token produced by `ensure_engine_surface`.
5. Change Shell `NodeGraphSurface`, `TiledMapSurface`, and `Board2dSurface` to retain both values. Hit testing/cancel/wheel/context/drop/catalogue passes `host_id` to EngineCanvas; actions continue using its stored canonical surface id and controller. This includes the paths currently at Shell lines 12528, 12962, 13430, 14753, and 24772.
6. Key World3d, IconRender, `SCENE_STATE`, `PENDING_RASTER_STATE`, `TEXT_EDITOR_UI`, deadlines, Canvas gesture/hover, and `SceneListTransfer` by host id. The methods that construct action descriptors take a scene or target and use `scene.surface_id` only at the final payload boundary.
7. A component replacement retires only its old host id. A document close queues every host under its document token, with no re-admission of an identical host id until the old CPU/GPU and local-state owners acknowledge terminal retirement. A sibling with a different host id is never removed or blocked by that component replacement.

## External CPU/GPU close integration

Do not adapt the whole-realm scanner into a per-document scan. Reuse its per-token protocol.

`PairedEngineSurfaceClose` already demonstrates the required sequence: determine whether CPU and GPU own the same token, begin each, drive CPU one bounded step, drive GPU one bounded step, then require both terminal witnesses ([os-host/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:204)). It also handles a CPU-only or GPU-only token without inventing the missing half. The external document-close owner should hold exactly one registered host token at a time and use those phases, rather than scanning 256 slots.

The CPU `EngineSurfaceRetirement` is already granular over graph, map, editor, raster, board, their sync caches, pending events, claims, and scalar data ([EngineCanvas/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:358)). Its token must be registered at attach time, not looked up by a document string after close has begun.

The new `SceneSurfaceRetirement` and `AdmittedSurfaceMap::take_exact_to_retirement` are the correct local-owner pattern: they reserve capacity while draining, then acknowledge the exact retirement ([Scenes/🦀️.rs](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:217)). Its map key must change from document `surface_id` to `host_id`; otherwise the new cursor still takes a sibling's state. The UI close cursor must not progress ordinary paint/reconcile for the closing document until every emitted component retirement has completed the local owner and paired engine acknowledgement. It may continue unrelated document work subject to existing fixed queue credits.

## Component coverage and remaining cleanup work

| Kind | Current owner collision / close gap | Required target |
| --- | --- | --- |
| Canvas2d | Scene state, settled camera, gesture, catalogue hover, raster queue are document-keyed. | Host-key all local data; close silently discards a matching gesture and drains only that host raster queue. |
| World3d | Shell `world3d_states`, status, hit and retirement use document key. | Host-key state and status; preserve canonical `surfaceId` in public status/actions. |
| NodeGraph | Engine CPU/GPU/stage registry and Shell mirror collide. | Host-key paired engine token and mirror; pointer, wheel, context, catalogue use host key. |
| TextEditor | Engine editor and unbounded `TEXT_EDITOR_UI` collide; focus stores no generation. | Host-key editor/UI state and full target focus; close clears only that focus/popup. |
| Table | Scene selection/scroll/click and list-transfer source collide. | Host-key state and transfer source identity. |
| Paint2d | Raster engine, marquee and pending raster lane collide. | Host-key engine/raster/marquee/async completion. |
| VirtualFileSystem | Scene expand/selection/scroll and list-lane state collide. | Host-key state. |
| TiledMap | Engine map/tiles, Shell mirror and Scene map interaction owner collide. | Host-key all map host/cache/interaction state; tile resource request must carry the host key separately from action wire id. |
| Board2d | Engine board, event/claim queues and Shell mirror collide. | Host-key all board state and cancellation. |
| IconRender | Synthetic World3d pass can share document world state. | Give the synthetic world a host key derived from IconRender's mounted host. |
| InkCanvas | Scene ink state plus editor/clipboard focus/streams collide. | Host-key ink local state and exact focused target/address. |
| GraphTimeline, BlockList, DiffView, EventFeed | Generic scene state/list rendering uses document key; BlockList also has transfer state. | Host-key generic state; BlockList transfer uses host identity. |

## Fail-first laws

Add a schema-first neutral fixture, for example `🔬️scene-host-identity`, whose one document has a single canonical surface id and two sibling `Component::Surface` records with distinct explicit keys. It must declare canonical `surfaceId` once and expected distinct runtime hosts without making the runtime host id a public document value.

1. **Two Canvas2d siblings:** mount cameras A and B with distinct authored cameras. Wheel/pan A. Require A's local viewport alone changes, B remains at its authored viewport, and every published action still has the same canonical `surfaceId`. Remove A and require B continues to paint and receive input. This fails today at the second `mount_scene_identity` call.
2. **Two engine siblings:** mount NodeGraph and TextEditor in the same document. Require both engine stages/raster keys are distinct, a graph gesture cannot modify editor text, and typing into the editor cannot modify graph interaction. This fails today because `ENGINE_SURFACES` stores one record under the document id.
3. **Replacement ABA:** capture a Canvas/Map/Graph host, replace that component while retaining the same document, then resolve an old upload/close/capture. Require the old host becomes terminal without an action and the successor retains its distinct host state. Require its token/generation is different from the old one.
4. **Document close paired acknowledgement:** mount all five engine-backed kinds across one or more documents, close through the real UI document close queue, and advance one close unit at a time. Before UI close completion, require each CPU and GPU token terminal; after completion, re-open using the old canonical document surface id and require new tokens. No stale token may close the successor.
5. **Capacity rotation:** run `ENGINE_SURFACE_CAPACITY + 1` mount/close cycles through the actual Shell document path, including at least one NodeGraph, TiledMap, Board2d, Paint2d, and TextEditor host. Require the last live host stages and the prior ones have no CPU/GPU owner. This must supplement the existing 257 Canvas scene-state law because it exercises the separate Engine registry.
6. **Focus/clipboard:** focus one of two TextEditors/Ink canvases with the same document surface id, remove its node, then deliver a key, paste resolution, and clipboard-stream completion. Require no action targets the survivor. Focus the survivor and require its input still works.

The React counterpart should mount the same two records through the actual Interpreter and assert two host elements share `data-surface-id` while their DOM component keys and interactions remain independent. It validates the canonical public address; it must not prescribe or expose WGPU's internal host key.

## Confidence and limits

High confidence: sibling `surface_id` sharing, current `mount_scene_identity` refusal, EngineCanvas collision, and Shell mirror collision are direct source facts. High confidence that the existing paired CPU/GPU retirement can be factored token-by-token because its implementation already distinguishes CPU-only/GPU-only ownership and validates both terminal witnesses.

The exact names of the new fixture files and host-id formatting are implementation choices. The report does not claim that an existing runtime fixture has already mounted two siblings; the existing source contract and recursive scene-slot traversal establish that such a document is legal and must work.
