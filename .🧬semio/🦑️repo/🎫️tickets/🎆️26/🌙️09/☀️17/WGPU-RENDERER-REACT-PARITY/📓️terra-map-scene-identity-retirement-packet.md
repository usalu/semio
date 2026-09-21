# Map Scene Identity Retirement Packet

## Decision

Retire only gesture-local state when the retained scene identity changes. Keep the MapHost and MapSyncCache through an ordinary same-identity document refresh. The identity is the existing ScenePointerTarget tuple:

- window_id;
- window_generation, which is Ui.accessibility_generation;
- retained node id and key;
- component kind; and
- surface_id.

A UI document ingress generation, Ui tree revision, layout generation, or changed scene payload is not a gesture lifetime epoch. It must not be substituted for window_generation.

This supersedes the World-specific part of the earlier scene-pointer lifecycle report. The current production contract makes a ComponentScene surface_id equal the owning document/window id, so a separate World owner field and a mismatched-id regression would defend an unreachable state rather than repair a demonstrated route.

## Source evidence

1. `framework/modules/ui/targets/wgpu/reconcile/🦀️.rs:428-445` projects ComponentScene.surface_id from the owning document surface. The React twin says the same in `renderer/engine/elements/Interpreter/🟦️.tsx:479-486`.
2. Every production caller passes that same document surface to both render_ui_document_step and SceneEngineHosts.window_id: Shell `:4641-4645`, `:19219-19223`, `:23616-23620`, `:24002-24006`, `:24204-24208`, and `:25702-25706`. The Measures, Actions, and Search callers use their suffix surface for both values. World map-key retirement is therefore correct under the present canonical identity.
3. `framework/modules/ui/targets/wgpu/engine/🦀️.rs:1031-1041` increments accessibility_generation only when the window has no document. It always increments revision and layout_generation. `surface_generation` returns accessibility_generation at `:2380`. Thus a normal same-window republish retains the target epoch; closing and reopening the window rejects the old one.
4. `renderer/engine/elements/Interpreter/wgpu/🦀️.rs:432-467` already checks exactly that lifetime epoch plus node, key, kind, and surface_id. The retained target survives a same-identity refresh and is refused after a key, kind, surface, or reopened-window change.
5. The identity-aware production transition is reconciliation, not document lease retirement. In `ui/reconcile/🦀️.rs:988-1004`, Mount has the old retained Node before overwriting its key/spec. In `:1039-1048`, Retire has an absent old node before removing it. `finish_document` at `ui/engine/🦀️.rs:989-1044` only replaces the document; its old retained tree remains until those reconcile steps. Shell `retire_one_surface_document` at `Shell/wgpu/🦀️.rs:7573-7581` runs before every successor is known and cannot decide whether an owner survived.
6. `Scenes/wgpu/🦀️.rs:381-415` stores SceneSurfaceState solely by surface_id. Its Map state is drag, map_marquee_points, map_marquee_active, and map_last_hover_json; `:7774-7907` reads and mutates all of them. No existing removal call exists for this state map.
7. EngineCanvas also keys EngineSurface by surface_id. It owns map_host and MapSyncCache (`EngineCanvas/wgpu/🦀️.rs:1508-1555`), and `sync_tiled_map_scene` deliberately reuses both at `:2523-2557`. The cache holds persistent fixture, camera, selection, hover, theme, and tile data. It is not an active-drag cache.
8. A complete chrome walk promotes retained hits in `Shell/wgpu/🦀️.rs:13055-13069`, after reconciliation and before the new pointer authority becomes observable. It has both Shell state and InputState. This is the narrow consumer seam for an already-recorded scene retirement.

## Bounded implementation map

### 1. Produce a retirement only for a changed identity

At the Mount update, snapshot the old ComponentScene identity before changing `existing.key` or `existing.spec`. Build the incoming identity from the projected key/spec. If both identities are equal, emit nothing. If the old scene is absent from the incoming node, or any of node/key/kind/surface differs, emit the old identity.

At Retire, snapshot an old ComponentScene before `remove_detached` and emit it. The document-close path must do the same before `close_document_binding_step` frees its node. Each record is one bounded reconciliation unit; do not scan the whole state map.

The UI target cannot depend on renderer's Interpreter type. It should expose a fixed-capacity, backpressured neutral `UiRetiredComponentScene` queue containing window id, accessibility generation, node, key, kind, and surface id. Interpreter converts that row to its ScenePointerTarget at dequeue. Its capacity must match the admitted scene-owner bound (256, already used by SceneSurfaceState and EngineSurface), and reconciliation must yield rather than drop a retirement when that queue is full. The queue never carries a bare surface_id. Interpreter drains one or more bounded items into the Shell at complete hit-registry promotion.

This differs from a finish_document hook in a material way: before reconciliation the old ComponentScene still lives, so a finish hook cannot tell an unchanged host from a changed retained node. It also avoids a Shell refresh hook: old leases are intentionally retired before both unchanged and changed successors.

### 2. Retire a matching gesture, never a successor

Add an owner stamp to the active Map interaction state when pointer down begins. It must contain the exact target identity, not just surface_id or document revision. Map move, up, cancel, and hover bookkeeping must only use the matching stamp.

On a queued retirement, remove every captured pointer slot whose exact target matches, cancel pending scene intents, and call a Map-specific cancellation operation with the old identity. It must:

- clear only the matching drag, marquee points, marquee active flag, and hover witness;
- cancel MapHost interaction without synthesizing PointerUp or interactionSelect;
- allow the existing Map cancel contract to publish its camera settle only for a real pan cancellation; and
- leave a later successor's state intact if its stamped identity differs.

The current SceneSurfaceState does not carry that stamp, so a bare `surface_id` cancellation would be able to erase a successor that reused the surface id. The MapHost needs the same active-owner stamp for its interaction, because EngineSurface and MapSyncCache are also addressed only by surface_id.

### 3. Keep the persistent host on a same-identity successor

Do not close EngineSurface or reset MapSyncCache for an ordinary same node/key/kind/surface update. React retains the host session across those updates, and the native cache purposefully avoids resetting the camera, tiles, and parsed fixture on unchanged values.

On an actual document/window closure with no successor, clear the surface's SceneSurfaceState only after its active interaction has been cancelled. Then start the existing incremental EngineSurface retirement rather than deleting an entire registry map in the input path. EngineCanvas already provides generation-keyed close primitives at `EngineCanvas/wgpu/🦀️.rs:2003-2012` and progressive retirement at `:262-309`, but normal document/window retirement currently does not feed them a surface token. That is a separate closure edge: expose token lookup by canonical surface id, queue the token once, and advance the existing close operation under its frame budget. A re-opened id must wait for or receive a new token generation; it must never inherit the former MapHost camera or gesture.

## Fail-first native laws

Use the existing neutral `renderer/engine/fixtures/scene-pointer-owner/🔣️.json` and Shell's actual AppInteractionState helper in `Shell/tests/wgpu-shell-input/🦀️.rs`. Add a TiledMap sibling fixture/body rather than a source-string assertion.

1. Same identity refresh: press/drag a map, republish with the same retained node/key/kind/surface, then outside-up. Assert the one original map host receives the release; the gesture remains active until that up; no second owner exists.
2. Key replacement on the same surface: press/drag, republish with a different record key but the same surface and kind, promote the registry, then outside-up. Assert old MapHost gets cancellation, not PointerUp; no interactionSelect is published; SceneSurfaceState has no old drag/marquee; the successor receives no release.
3. Kind replacement on the same surface: begin a Map gesture, replace it with a different ComponentScene kind under the same document surface. Assert the Map gesture is cancelled exactly once and the new kind receives neither old move nor up.
4. Window close/reopen: begin a Map pan, close the owning document/window, run one bounded retirement step, then reopen the same id. Assert the former target generation cannot route input; the new host does not inherit the old local camera/drag; the old token cannot close the new generation.
5. Backpressure: fill the retirement queue to its fixed capacity, attempt one more identity replacement, and assert reconciliation remains pending until one retirement is consumed. No retirement may be silently discarded.

The already-present World fixture's `refresh` and `replacement` cases establish the corresponding target-capture distinction. Its present windowId == surfaceId values are correct canonical examples; they are not evidence for a mismatched World route.

## Confidence and limits

High confidence in the identity, reconcile seam, and same-refresh preservation: each is directly established by current source. High confidence that bare-surface Map state can survive a changed-key replacement: SceneSurfaceState is surface keyed and has no retirement path. Medium confidence in the exact Shell queue API because it has not been implemented or run; the report specifies the necessary ordering and bounded behavior rather than asserting a build result. No Cargo, browser execution, or test was run for this audit.
