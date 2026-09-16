# ◻️ Puzzle 2d ↔ 🧊️ puzzle 3d feature-parity survey (2026-09-16, source-measured)

Reference: the 3d ticket's executable checklist `26/09/02/PUZZLE-3D-END-TO-END/📓️2026-09-09-user-feature-checklist.md`
(25 sections) and its battery `🔍️browser-probe.ts` (74/25 PASS, 0 faults on 2026-09-13). Sources read today:
`◻️2d/…/✏️editor/🦀️.rs` (3,949 lines, 30 retained ids), `🎭️modes/✏️edit/🦀️.rs`, `🪟️windows/👁️overview`,
`🛠️tools/🪣️fill`, `📌️panels/{🛍️catalogue,🔍️inspection}`, `🎮️commands/🎲️apply-board-events`, the React
`🖥️Board2dHost/🟦️.tsx` (1,052 lines) and the 3d editor's `Puzzle3dInteractionSnapshot` / `render_with_request_context`.

## Architecture difference that shapes every row
3d: the React `World3dHost` picks/hovers/marquees and dispatches framework `interactionSelect`/`interactionHover`;
the guest reads the framework-owned `vortex` domain via `Puzzle3dInteractionSnapshot::from_interaction` inside
`render_with_request_context`. 2d: a wasm-pack `BoardSession` engine (wgpu) owns all pointer input and emits
board events (`camera`, `select`, `nodeMove`, `nodeDragEnd`, `brushPlace`, `edgeCreate`, `edgeDelete`, `nodeDelete`,
`brushCandidates`) that `Board2dHost` coalesces into ONE guest verb `applyBoardEvents {eventsJson}`; the host also
dispatches `setCamera`, `setSelection`, `addNode` (catalogue drop). The guest paints `selection_json = "[]"` always
(`🎭️modes/✏️edit/🦀️.rs` `puzzle2d_board_scene`) and `render_with_request_context` ignores its `_interaction`.

## Row-by-row (3d checklist § → 2d state)
| § | 3d feature | 2d today (source) | gap class |
|---|---|---|---|
| 1 | 2 window instances render | 3 kinds (overview interactive, detail, selection) via `SurfaceKind::Canvas2d` → `Board2dHost` | verify at runtime |
| 2 | orbit/pan/zoom → `setCamera` per window | pan/zoom in the engine → `camera` board event + `setCamera`; `runtime_camera` shared, pane framing derived | verify |
| 3–4 | projection/grid/LOD/vortex/sun/select options | LOD per pane (`setLodModeForPane`), brush options (`setBrushNodeSize`, `setBrushKindWeights`), grid snap/factor verbs exist but NO measure renders them (`window_measures` = lod + brush only) | **add grid measure group** |
| 5 | example switcher + empty → add kind | `setActiveExample` Work (8-step stages), boot default EMPTY board, `addNode` | verify Nakagin (180 nodes) loads |
| 6 | click/marquee/same-kind/all/clear selection painted | engine paints its own local selection; guest never echoes framework selection back (`selection_json="[]"`); `selectSameKind` is a `NoopPuzzleCommandWork` (host-only) → **dead** | **thread `InteractionView` into render; real selectSameKind** |
| 7 | hover flag | `hovered_id: None` always | **thread hover** |
| 8 | transform gumball translate/rotate/scale | engine drag-move → `nodeMove`/`nodeDragEnd` → `patch_inspector_nodes(x,y)`; no rotate/scale verbs | **add `rotateSelection`/`scaleSelection`/`translateSelection` (programmatic + engagement)** |
| 9 | brush utility + candidate picker | `brushOpenSlot/CycleCandidate/SetCandidateIndex/CommitSlot/CancelSlot` + `brushCandidates` board event; measures in `☑️options/🖌️brush` | verify |
| 10 | volume brush (target volumes) | no 2d analogue (no target regions) | decide: target-region brush (rect) — OUT unless cheap |
| 11 | relocate utility | drag-move covers it | n/a |
| 12 | fill tool: count, cancel, weights, run panel | `ToolRunDefinition` with run/revalidate jobs (`⏳️precompute/🪣️fill` 1,614 lines), count measure ONLY — no distribution weights group, `settings.config` = `/fillCount` only | **add node/handle kind weight group to fill measures + settings reads** |
| 13 | vortex suggestions popup (open/hover/accept/close) | brush slot family (open/cycle/commit/cancel) is the analogue; no hover-to-preview verb | verify; add `hoverSuggestion`-like if the engine supports it |
| 14 | engagement bar `fill <n>`, `brush`, `zoom` | `engagementInput/Submit/Abort/ControlSelect` + placeholder "select, brush, clear" | verify parser |
| 15 | context menu per selection kind | `puzzle2d_context_menu_items` exists (reads `request.surface.selection`) | verify; adopt authoritative interaction selection |
| 16 | inspection panel per entity (`patchInspector`) | ALWAYS document summary (doc comment says framework gap — no longer true, `render_with_request_context` carries `interaction`) | **rewrite: per-node id/kind/x/y/radius editable rows via `patchInspectorNodes`** |
| 17 | outliner with hide/lock row icons | `📌️panels/🗿️artifact` 83 lines — check hide/lock rows | verify/extend |
| 18 | catalogue rows add + drag/drop | `📌️panels/🛍️catalogue`: nodes draggable, click → `addNode {kind}`; `Board2dHost` drop → `addNode` | verify |
| 19 | settings panel (4 steppers) | none | **add `📌️panels/⚙️settings`** (grid factor, snap, suggestion offset, brush node size) |
| 20 | history undo/redo/checkpoint | framework | verify on 2d |
| 21 | copy/cut/paste | framework | verify |
| 22 | delete/duplicate/focus | `deleteSelection` (Del), `duplicateSelection`, `focusSelection` | verify re-select of clones |
| 23 | add-object dialog | none (`addNode` via palette with a static `node` option) | **make `addNode` args enumerate live node kinds** |
| 24 | import/export fixture | `import-media` reserved route only; 3d has `exportFixture`/`importFixture`/`openImportFixture` | **add export/import fixture verbs** |
| 25 | EN/DE | `puzzle2d_labels` fail-closed since 09-06 | verify |

## Extra 3d-only verbs with no 2d analogue (deliberately out): sun, projection, voxel dims, target volumes,
world-relocate, register-brush-mesh, chunk size, proximity radius.

## Runtime gate first
Nothing above has ever been driven in a browser for 2d (09-06 status stops before step 5). Order: boot → battery
probe (port of the 3d `🔍️browser-probe.ts`) → fix what is red → then the gap rows marked **bold**.
