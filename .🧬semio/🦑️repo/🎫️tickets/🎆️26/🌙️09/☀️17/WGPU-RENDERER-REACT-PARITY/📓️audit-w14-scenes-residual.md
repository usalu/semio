# W14 — Residual scene/engine-canvas gap audit (all 15 SurfaceKinds), re-verified against the live tree

Read-only audit, no source edits, no builds, no sub-agents. Written 2026-09-18 ~21:56 local. The tree
is being edited concurrently by other ticket workers (W13c/W13e integration loops still running per
`📓️status.md`); every citation below was grepped against the tree AS IT STANDS NOW, not copied from an
earlier report. Baseline read in full: `📓️status.md`, `📓️audit-scenes-engine-canvas.md` (Wave-0), plus
w1a/w1b/w1f/w2a/w2b/w2f/w3d/w5c/w8b/w9b/w10a/w12b/w13e in full. This report only lists what is **still**
a gap, or newly found — it does not re-list what those reports already closed and I re-confirmed closed.

## 0. Headline: the Wave-0 P0 ("11 of 15 kinds paint nothing") is CLOSED

Re-verified live in `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1658-1668` and
`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2967-2976`:

```rust
// render_component_scene_step, 🎞️Scenes/…/🦀️.rs:1658
if cursor.phase() >= ENGINE_PHASE {
    match scene.component_kind {
        SurfaceKind::World3d    => return render_world3d_surface_step(...),
        SurfaceKind::Canvas2d   => return render_canvas_2d_step(...),
        SurfaceKind::InkCanvas  => return render_ink_canvas_step(...),
        SurfaceKind::IconRender => return render_icon_render_step(...),
        kind if scene_kind_is_list(kind) => return render_list_scene_step(...),  // Table/VFS/GraphTimeline/DiffView/EventFeed/BlockList
        _ => {}
    }
}
// falls through to the phase 4-8 ladder, which calls sync_engine_scene:
match scene.component_kind {
    SurfaceKind::NodeGraph  => sync_node_graph_scene(...),
    SurfaceKind::TiledMap   => sync_tiled_map_scene(...),
    SurfaceKind::Board2d    => sync_board2d_scene(...),
    SurfaceKind::Paint2d    => sync_paint2d_scene(...),
    SurfaceKind::TextEditor => sync_text_editor_scene(...),
    _ => false,
}
```

4 (direct) + 5 (engine-canvas vello) + 6 (list) = all 15 `SurfaceKind`s now reach a real paint path in
production. Surface context menus (W2b), TextEditor popups (W2b), the three dead asset pipelines
(terrain/map-tile/ui-image, W1f+W2a), World3d camera wire shape (W1f/W8b), TiledMap selection verbs
(W1f), Board2d keyboard+domain_id (W2a), CAD typology/picking engine + engagement preview (W2f), the
World3d mesh/textured GPU passes + one-frame-per-boot presenter bug (W5c/W7b), orthographic camera +
lighting + grid + mesh-style table (W8b/W9b), world interaction journal parity (W10a), and the
`registerBrushMesh` twin (W12b) are all confirmed landed by direct grep, not just report-reading.
**Do not re-dispatch packets for any of the above.** What follows is what is left.

## 1. Cross-cutting residuals (apply to many kinds)

| # | Item | Status | Evidence |
|---|---|---|---|
| C1 | **Pointer events carry no modifier field.** `UiEvent::PointerDown/Up/Move/Scroll` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:46-65`) still have no `modifiers: EventModifiers` field — only `KeyDown`/`KeyUp` do. | **P1, confirmed still open** | Read live: `PointerDown{x,y,button}`, `PointerMove{x,y}`, `Scroll{x,y,delta_x,delta_y}` — no modifiers anywhere. Blocks shift/ctrl multi-select for VFS (`vfs_selection_for_click(…, false, false)` hardcoded), Table, BlockList, NodeGraph, and every "merge mode from modifiers" call that currently can only ever resolve `replace`. |
| C2 | **Row drag sources are attached but not dispatchable.** `TableScene.row_drag_mime` is read into `drag_data` at paint time (`🎞️Scenes/…/🦀️.rs:2369`) but no drag-start/drop wiring consumes it; `TableScene::drop_action_json` and the BlockList palette drag (`PALETTE_DRAG_MIME`) have no route at all. | **P1, confirmed still open** | grep for `drop_action_json` / palette-drag route: zero production hits outside the read of `row_drag_mime` itself. |
| C3 | **World3d production relocate drag does not exist.** All three pointer handlers that would drive it (`handle_world3d_pointer_move/_button/_drag`) are still `#[cfg(test)]` in `♾️infinite/🌍️world/🦀️.rs:11860` onward; the one live `worldRelocate` action id (`:11973`) is inside that same `#[cfg(test)]` block. Production goes through `WorldInteractionAuthority` instead, which has no `Relocate` plan kind. | **P1, confirmed still open** (W2a §10 item 1, re-verified) | `grep -n "fn handle_world3d_pointer_button" -B3` → `#[cfg(test)]` on the line above, still. Escape-cancels-relocate and the vortex suggestion menu are therefore both still meaningless (nothing to cancel). |
| C4 | **Paint2d marquee/lasso overlay and navigator "you are here" are unwired.** `RasterHost::marquee_hits_json` exists; no call site in `⚙️EngineCanvas/…/🦀️.rs` reads it (grep: 0 hits). `navigator_fit_camera_json` is applied (`⚙️EngineCanvas/…/🦀️.rs:2852`) but `navigator_viewport_overlay_json` and navigator pan/wheel are absent. | **P1, confirmed still open** | Direct grep, this session. |
| C5 | **Two of the two "dead pipeline" fixes from W1f/W2a hold; one wire-drift item from that same family is unresolved:** NodeGraph's own `interactionSelect` omits `surfaceId` while every other surface's now includes it (W1f §"remaining gaps" item 7) — not re-checked this session but no report claims it fixed; harmless (guest reads `domainId`) but still a live spelling drift between wgpu surfaces. | **P2, not re-verified, presumed open** | Carried forward from W1f's own report; no later packet claims to have closed it. |

## 2. Per-kind gap tables

Legend: **wgpu** column is full / partial (what's missing) / absent. All line numbers are from the
live tree at audit time.

### World3d

| Category | wgpu | Notes |
|---|---|---|
| Paint (mesh/textured/line passes, lighting, grid) | **full** | W5c + W8b landed the GPU encoders, R3F-equivalent light rig, adaptive far plane, single-band grid. |
| Camera (orbit/pan/zoom/fit, orthographic projection, `setCamera` wire shape) | **full** | W1f + W8b + W9b: `CameraProjection3d`, pixel frustum, `zoom`/`up` on the wire, projection-template selection reaching the live camera (`liveCamera` diagnostics confirm it numerically to 0.007%). |
| Interaction journal (hover/select/orbit/pan/zoom/pick/context-menu order & `origin`) | **full** | W10a: matches React's measured journal row-for-row, with `origin: gesture` provenance. |
| GLB mesh lane (url-declared meshes, frame rotation, transport URL) | **full** | W3d. |
| Terrain DEM tiles | **full** | W1f: rides the bounded asset pipeline now. |
| **Relocate drag / Escape-cancel-relocate / vortex suggestion menu** | **absent (production)** | C3 above. `#[cfg(test)]`-only. |
| `registerBrushMesh` twin | **full** | W12b. |
| Sub-object pick-target LANE (face/edge/vertex/handle) — generic `World3dScene` wire | **absent** | `World3dScene` has no pick-target lane; a click resolves to the object only. W2f §5 item 1 (framework-owned hand-off), re-confirmed: no new lane found this session. |
| CAD: curved edges tessellation | **partial (degrades to endpoints)** | `CadEdgeCurve` still carries only `kind` (`…/🗺️geometry-import/🦀️.rs:61`), confirmed unchanged. |
| CAD: per-instance locked-opacity dimming | **absent** | No `WORLD_LOCKED_OPACITY_SCALE` application found on the wire-instance side; `target_style`'s locked arm exists and is tested but the instance lane carries no per-instance opacity field to apply it to (W2f §5 item 5, re-confirmed unfixed). |
| Mesh style table: `celebrated` conic spin, `highlighted`/`disabled` wire producers | **partial** | Table exists (`resolve_mesh_style`, W9b) but `WORLD3D_SHADER` has no conic term/flag bit, and `World3dSceneInstanceEntry` only carries `provisional` — `celebrating`/`highlighted`/`disabled` are unreachable from any producer today. |
| Grid: shader plane vs line geometry | **partial (line geometry, clamped at 512 divisions, invisible-by-fade)** | W9b §3: deliberately not started; scoped with exact sites. Vulkan backend has no World3d module at all. |
| Reference underlay `mem::take` regression (W9b §7 item 5) | **appears fixed** | The specific `take_reference_underlay_upload`/`mem::take` pattern W9b flagged no longer exists under that name; `reference_underlay_upload` (`🌍️world/🦀️.rs:10136`) now has a doc comment "no longer does" at :10131, suggesting a later packet fixed it. **Not independently live-verified this session** — recommend a quick boot check before closing. |
| Locked/pick screen-space picking, catalogue drag/drop, hover cursor | **full (parity confirmed)** | W1f. |

### TiledMap

| Category | wgpu | Notes |
|---|---|---|
| Pan/zoom/fit, LOD/chunking | **full** | Shared `MapHost` core. |
| Selection/hover verbs (`interactionSelect`/`interactionHover`) | **full** | W1f fixed the `setFeatureSelection`/`setHover` → generic verb drift; guest evidence cited. |
| Map tile fetch (raster/vector) | **full** | W2a: `reserve_map_tile_fetch(es)` + `apply_map_tile_bytes` now both called in production (`🧊️renderer/🦀️.rs:10567,10607`). No prefetch ring yet (W2a's own noted follow-up, low severity). |
| Context menu | **full** | W1f: `tiled_map_context_menu_target` + Shell wiring. |
| Wheel-zoom reachability | **full (confirmed live)** | W1f. |
| Labels/captions (interactive DOM hover popup) | **partial (UX-level only, colors match, no popup)** | Unchanged from Wave-0 — likely acceptable, not re-flagged as urgent. |
| **gis2d playground: boots ready, presents literally nothing (uniform background, no chrome/map pixel) for 420s, worker hangs silently after `ui-doc ingress`** | **P0 — live-boot defect, NOT a paint-dispatch gap** | W13e §3: reproduced 3× including a 420s run; suspects named (`MapHost::new()`, `sync_map_json`, `fit_world_camera` — all non-yielding inside the first window paint); hand-off instructions given (freeze-dispatch technique). Not yet re-probed this session — treat as open until re-verified. |

### Board2d

| Category | wgpu | Notes |
|---|---|---|
| Mount/lease, camera, placement/brush tools, grid/LOD, labels, context menu | **full** | Wave-0 OK items, unchanged. |
| Keyboard (Escape cancels area-select, Tab/shift+Tab brush-cycle, capture-phase precedence) | **full** | W2a §4.1. |
| `domain_id`-scoped hover | **full** | W2a §4.2. |
| Event-table drift (transient/flush-now sets) | **full** | W2a §4.3. |
| Selection-verb model (bespoke `BoardEventKind::Select` vs React's generic `interactionSelect`) | **unchanged/deliberate, documented** | W2a §"gaps": explicit decision to keep bespoke since React itself dispatches no generic `interactionSelect` for Board2d either — not a real divergence, correctly left. |
| Hover-row republish-per-move cost on the `BoardHost::plan_pointer` path | **P2, still open** | W2a §"gaps": needs a `🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` push-guard; not this ticket's file list, small. |
| Third Escape listener (cross-pane suggestion-popup dismissal) | **absent** | W2a, unchanged. |

### Paint2d

| Category | wgpu | Notes |
|---|---|---|
| Mount/lease via shared `RasterHost`, pan/zoom/fit (basic) | **full** | W1b: real vello-texture host, shared compositor with React. |
| Selection/hover dispatch (`interactionHover`/`interactionSelect` on `layers` domain) | **full** | W1b §4. |
| Scene lanes (paging past 32 KiB) | **full** | W2a §6c. |
| IconRender pending/error state pattern reused | n/a | n/a for this kind. |
| **Marquee/lasso overlay + `marqueeHitsJson` commit** | **absent** | C4 above; single-point pick only. |
| **Navigator "you are here" + navigator-drives-content-camera** | **absent** | C4 above. |
| Context menu | **full** | W2b: `paint2d_context_menu_target`. |

### InkCanvas

| Category | wgpu | Notes |
|---|---|---|
| Rendering (strokes, tables, images, grid, selection chrome) | **full** | W1b recovered `render_ink_canvas` from pre-2026-09-08 history, now production. |
| Wire field-name drift (`documentJson` vs `snapshotJson`) | **full (fixed)** | W2a §2: both encoders now agree on `documentJson`, pinned by a law; repo-wide grep for the field shows zero `snapshotJson` remaining (only an unrelated machine-derive name). |
| Camera, marquee compute, context menu | **full** | W1b/W2b. |
| Keyboard | **not independently re-checked this session** | Wave-0 flagged as P1-absent; no later report claims it landed. Presumed still open — recommend a quick grep before another packet is spun up blind. |

### TextEditor

| Category | wgpu | Notes |
|---|---|---|
| Buffer/gutter/selection/carets, semantic-token highlighting via `EditorHost` | **full** | W1b. |
| Keyboard editing, F2 rename, Ctrl/Cmd+Space + alt-click completions | **full** | W1b + W2b (completions/rename state machines, popup paint). |
| Context menu | **full, by design routed through Shell's own chrome, not a promoted test helper** | W2b §2.3 — deliberate, documented decision; not a gap. |
| `multiSpanReplace` unshifted-span offsets after the first character of a length change | **shared defect, both renderers, out of scope for a parity fix** | W2b §6 item 3 — React itself has the bug; flagged for the React lane, not wgpu. |
| Rename input is a painted box, not the real retained `Input` widget (no IME) | **P2, documented follow-up** | W2b §6 item 6. |

### Table

| Category | wgpu | Notes |
|---|---|---|
| Paint (columns/rows/sort/cells/steppers) | **full** | W1a. |
| Row press → `selectRow`/`interactionSelect`, header sort, stepper delta merge | **full** | W1a §5. |
| Scroll | **full** | W1a. |
| Context menu | **full** | W2b: `table_context_menu_target`. |
| **Row drag source / drop target** | **absent (inert)** | C2 above — attached at paint, never dispatched. |
| Modifier-qualified multi-select | **blocked on C1** | Same root cause as VFS/BlockList. |

### VirtualFileSystem

| Category | wgpu | Notes |
|---|---|---|
| Paint (tree/list rows, expand/collapse), scroll | **full** | W1a: `render_vfs` recovered from `860e015bf6`. |
| Row press → `selectRows`, double-click → `openInstance`/`exportMedia`/`navigateVirtualFileSystemNode` | **full** | W1a §5, with geometry fixed to use theme metrics + visible-row indexing (was hardcoded `22.0`/`24.0` before this ticket). |
| Wire tag rename (`virtualFileSystem` → `virtual-file-system`) | **full (fixed everywhere, law added)** | W2a §3: wgpu `SurfaceKind`, schema projection, both generated TS unions, all four stale docstrings, and a new golden law walking all 15 variant pairs. |
| Context menu | **full** | W2b: `vfs_context_menu_target`, expansion-aware. |
| **File-type glyph table** | **partial (3 hardcoded kinds: folder/instance/file-text) vs React's ~40-entry extension table** | `vfs_glyph_icon`, `🎞️Scenes/…/🦀️.rs:6934-6944` — confirmed live, unchanged since Wave-0. Low severity (cosmetic), still real. |
| Modifier-qualified selection (shift/ctrl) | **blocked on C1** | `vfs_selection_for_click(…, false, false)` still hardcoded — confirmed, not re-checked line number this session but C1 makes it structurally impossible regardless. |

### IconRender

| Category | wgpu | Notes |
|---|---|---|
| Chrome paint (frame border, status text), GLB delegate paint | **full** | W1b. |
| Mesh transport URL resolution | **full** | W1b: `mesh_asset_transport_url`, the Rust twin of `meshAssetTransportUrl`. |
| Pending/error/ready state | **full** | W2a §8: `IconRenderStatus{Ready,Rendering,Failed}`. |
| Pointer/wheel | n/a | React's `IconRenderHost` has no handlers either — parity. |
| Localized labels for status text | **P2, hard-coded English, documented** | W2a §8 — own packet, not urgent. |
| Context menu resolver | **full (answers empty, by design — React binds none either)** | W2b §6 item 5 — parity, not a gap. |

### GraphTimeline

| Category | wgpu | Notes |
|---|---|---|
| Paint (rows/lanes/avatars), row select → `checkoutCheckpoint` | **full** | W1a. |
| Context menu | **full** | W2b. |
| Scroll | **full** | W1a wired it into the list-scene reservation model. |

### DiffView

| Category | wgpu | Notes |
|---|---|---|
| Paint (unified + split modes), diff algorithm (LCS w/ documented size-cap divergence) | **full** | W1a. |
| Highlighting model (text-tint only, no row wash) | **full, matches React's intent exactly** | W1a §5 item 2. |
| Context menu | **full** | W2b (in the "whole-surface, no hits" family). |
| **Line-number gutter (`beforeNo`/`afterNo`)** | **absent** | Confirmed live: no `before_no`/`gutter` match in the Scenes file. Wave-0's own note said this needs `unchanged_lines_stay_full_brightness_not_dimmed` reworked first — still true. |

### BlockList

| Category | wgpu | Notes |
|---|---|---|
| Paint, add/remove/move step & block actions | **full** | W1a: `block_list_plan` shared paint/hit geometry, drift-pinned by test. |
| Reordering model (click-based, not drag) | **deliberate divergence, documented** | Not a gap — no established cross-frame drag-position-tracking primitive. |
| Context menu | **full** | W2b. |
| **Native + driver-arm palette drag-drop** | **absent** | C2 above (same family as Table's row drag). |

### EventFeed

| Category | wgpu | Notes |
|---|---|---|
| Paint, follow/auto-scroll-to-bottom, tone→color mapping | **full** | W1a. |
| Row click → `activateAction {surfaceId, id}` | **full (field-name drift fixed)** | W1a §5 item 1 — was `entryId`, now `id`, pinned by a test asserting `entryId` is absent. |
| Per-row context menu | **full** | W2b: `event_feed_context_menu_target`. |
| Scroll | **full** | W1a. |

### NodeGraph

| Category | wgpu | Notes |
|---|---|---|
| Mount, engine-selection predicate, hit-test/drag-connect/minimap/pan-zoom/marquee | **full** | Wave-0 OK, shared native crates. |
| Catalogue drag/drop onto canvas | **full** | Wave-0 P2 resolved — test exists, dedicated `wgpu-catalogue-workflow-drop`. |
| Context menu (WP-10 item, was "not verified") | **full — now confirmed** | `node_graph_context_menu_target`, `⚙️EngineCanvas/…/🦀️.rs:5240-5255`; W2b §1.3 lists it explicitly. |
| Keyboard: Escape clears selection, node/edge delete | **full — confirmed live this session** | `KeyAction::Delete => host.note_delete_forward()` (`:3057`) and further delete-arms at `:5619-5644`. WP-10's "inconclusive" is now resolved: it IS wired. |
| Per-window view state (WP-10 item) | **not independently re-checked this session** | Low priority per Wave-0's own framing; no report claims it was checked. |

## 3. New finding this session

**W9-family diagnostics say the CAD World3d pick-target lane, curved-edge tessellation and locked
dimming are still the only load-bearing GAPS left in the CAD/World3d story** — everything else W2f
scoped (typology engine, picking engine, engagement preview) is landed and green. This is consistent
across three independent re-greps (picking engine file, geometry-import file, world instance wire) done
this session, not carried over unverified from the W2f report.

**generation3d and gis2d are NOT part of the 15-SurfaceKind wire-parity scope** (they're playgrounds
exercising NodeGraph/World3d/TiledMap), but W13e's live-boot findings are worth carrying forward because
they're P0-severity and freshly discovered: node/port LABELS unpainted in the flow graph (NodeGraph,
generic — likely affects every NodeGraph-hosting playground, not just generation3d), and the gis2d
420-second silent hang (TiledMap). Neither is re-verified live this session (no rebuild/activation was
done — this was a read-only audit) but both are recent (same day) and specific enough to hand off cold.

## 4. Top 10 packets to dispatch

1. **P0 — Investigate the gis2d TiledMap boot hang** (W13e §3 hand-off: instrument
   `MapHost::new()`/`sync_map_json`/`fit_world_camera`, `⚙️EngineCanvas/…/🦀️.rs:2446,2485,2509`; freeze-dispatch
   technique from `feedback-freeze-dispatch-to-read-guest-trap`). Live-boot P0, not a paint-dispatch gap,
   but it means TiledMap presents literally nothing on at least one real playground today.
2. **P1 — Add `EventModifiers` to `UiEvent::PointerDown/Up/Move/Scroll`** (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:46-65`).
   Unblocks shift/ctrl multi-select for VFS, Table, BlockList, NodeGraph and every merge-mode-from-modifiers
   call in one packet — the single highest-leverage remaining fix (C1). W2a already flagged it as
   "wants its own packet, wide but mechanical."
3. **P1 — World3d production relocate drag.** Needs a new `WorldFlatActionKind::Relocate`,
   `plan_world3d_relocate`, a ray-pick purpose arm and the dispatch arms (`♾️infinite/🌍️world/🦀️.rs`,
   near the existing `#[cfg(test)]` mirror at `:11860`). Unblocks Escape-cancel and makes the vortex
   suggestion menu meaningful. W2a's own next-step framing, re-confirmed still open.
4. **P1 — Row/palette drag-and-drop** for Table (`drop_action_json`) and BlockList (native +
   driver-arm palette). Depends on #2 for modifier-qualified drops but the drag-start/drop route itself
   is a separate wire-up (C2).
5. **P1 — Paint2d marquee/lasso overlay + navigator "you are here" + navigator-driven content camera**
   (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`, near `navigator_fit_camera_json` at `:2852`). Both items
   were scoped but not landed by W2a (C4).
6. **P1 — `World3dScene` sub-object pick-target lane.** New wire lane (id/kind/point/points/typology/
   style) + readers on both `🌐️World3dHost` and `♾️infinite/🌍️world`, consumed by the picking engine
   W2f already ported and left half-wired (W2f §5 item 1). This is the biggest remaining CAD gap and a
   framework-level seam other World3d-hosted plugins would also benefit from.
7. **P2 — CAD curved-edge tessellation + per-instance locked-opacity dimming.** Widen the `CadEdgeCurve`
   import struct (or read the real `Brep` handle) for arc/circle/ellipse/nurbs; add a per-instance
   opacity field to the World3d instance wire so `WORLD_LOCKED_OPACITY_SCALE` has something to multiply
   (W2f §5 items 2 and 5).
8. **P2 — DiffView line-number gutter.** Needs `unchanged_lines_stay_full_brightness_not_dimmed`
   reworked first (it currently asserts `theme.text_muted` is absent from the glyph palette), then port
   React's `beforeNo`/`afterNo` gutter column.
9. **P2 — Node/port labels in the NodeGraph paint** (surfaced live by W13e on generation3d; likely a
   generic NodeGraph gap, not playground-specific — worth a quick check on the puzzle3d/other NodeGraph
   boots before scoping it as its own packet).
10. **P2 — Mesh-style wire completion**: publish `celebrating`/`highlighted`/`disabled` on
    `World3dSceneInstanceEntry` (currently only `provisional`) so W9b's already-landed style table has
    real producers for its other rows; add the conic-spin flag bit to `WORLD3D_SHADER` + MSL/HLSL
    mirrors for the `celebrated` animation.
