# Explore: puzzle 2d (drag, selection, history) for non-destructive history editing

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Read-only audit, 2026-09-30. No source file edited, no build or test run.
Every statement is grounded in code that was read; where I only inferred behaviour from code (no runtime proof) it says
"by code reading". Nothing here claims a test passes.

Path aliases used below (all repo-root relative):

- `P`   = `✏️s/🔌️plugins/🧩️puzzle`
- `A2D` = `P/🗿️artifacts/◻️2d`
- `ANY` = `A2D/🏅️standards/🔖️1/🪆️subsets/✳️any`
- `ED`  = `ANY/✏️editor` (the play app; `ED/🦀️.rs` is the 5709-line app file)
- `MUT` = `ANY/🧬️schema/🧬️mutations`
- `FIX` = `ANY/🧫️fixtures/🧬️mutations`
- `PLUG` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (45k lines, VcsArtifactApp, Emit, dispatch; a peer is editing it, my `PLUG` line numbers were read at about 01:40 and had already drifted by ~5 lines by 01:50, treat them as +-10)
- `BH`  = `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` (board engine, 13.7k lines)
- `B2H` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx` (React board host)

## 0. Ten-second picture

- "Puzzle 2d" is the `◻️2d` artifact of the `🧩️puzzle` plugin (crate `semio-s-artifact-puzzle-2d`, kind `s.puzzle.puzzle2d`, standard 1, one subset `✳️any`).
  There is no separate "app" tree: the editor app (`s.puzzle.puzzle2d@1/*#editor`) and the read-only viewer live under `ANY/✏️editor` and `ANY/👁️viewer`.
- The 33 document mutations are all absolute final-state setters (`move-node {id,newX,newY}`). No mutation records an offset, a target set, or a gesture.
- Selection is NOT a document mutation and NOT window state. It is the framework-owned interaction domain `vortex`, persisted in a separate store on
  `HistoryLane::Interaction`. Reducers read it ambiently.
- A board drag is computed entirely client-side in the board engine (`BoardHost`); the guest sees one buffered event batch (`applyBoardEvents`) whose
  `nodeDragEnd` row carries only final absolute positions. The guest patches a JSON scratch fixture and DIFFS before/after into `MoveNode` (+ `ConnectHandles`) mutations.
  All of one dispatch's mutations are published as ONE `Edit`.
- Therefore, at commit time the tool's real inputs (which ids, which offset) are already lost. That is the core gap for editable history (section 5).

## 1. Artifact schema

### 1.1 Identity and layout

- Artifact root `A2D/🦀️.rs` (1857 lines): `PUZZLE_2D_SCHEMA = "puzzle.2d.fixture"` (L28); document records L30-540; `PUZZLE2D_DIALECT` (`s.puzzle.puzzle2d`, std `1`, subset `ANY`, L553);
  `artifact_kind()` L559 (`2d.puzzle`, MeshOnly media, export/import stdio kinds dwg/dxf/json/pdf/png/svg); `definition()` L613 (capability rows: standard, schema, inference, composers native+svg/pdf/png/json/dwg/dxf, 5 grammars, codec);
  `artifact::<PA>()` L662 (declaration tree root); `pilot_languages()` L670 (5 `LanguageSpec`s: `puzzle.puzzle2d`, `.op`, `.diff`, `2d.pack`, `2d.spr`); module mount tree L740-1857 (`#[path]` per leaf; editor mount L1573, viewer mount L1820, both behind `feature = "component-app-assembly"`).
- Subset policy `ANY/🔣️.json`: `subsetPolicy: "single"`, `✳️any` is the only subset.
- Manifest `A2D/🛂️manifest.json`: default puzzle manifest (`portModel: ported`, `directedness: directed`, one port kind, one wire kind, one edge kind, no node kinds).
- Directory taxonomy under `ANY`: `🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations,💡️inferences}` (each with `🔣️.json`, `🔗️.graphql`, `🦀️.rs`, `🟦️.ts`, `🛰️.proto`, plus `💾️binary/` and `📝️text/` codec facets), `🚪️io`, `🔮️oracles`, `✏️editor`, `👁️viewer`, `📚️examples`, `🧫️fixtures`, `🧪️tests`.

### 1.2 Snapshot (`ANY/🧬️schema/📸️snapshot/🦀️.rs`, `Puzzle2dSnapshot`, dsl id `puzzle.puzzle2d`)

Fields (all `#[state(artifact)]`): `schema: String`, `camera: Puzzle2dCamera`, `nodes: Vec<Puzzle2dNode>`, `edges: Vec<Puzzle2dEdge>`, `targetRegions: Vec<Puzzle2dTargetRegion>` (skipped when empty), `meta: Puzzle2dMeta`.
Record types in `A2D/🦀️.rs`:

- `Puzzle2dNode` L105: `id, nodeKind?, shape?, x, y, radius?, width?, height?, text?, iconKind?, root?, scale?, visible?, locked?, anchor: Fixed|Derived, handles: Vec<Puzzle2dHandle>`.
- `Puzzle2dHandle` L53: `id, handleKind?, angle, radius?, color?, iconKind?, scale?, visible?, locked?`.
- `Puzzle2dEdge` L178: `id, source(handle id), target(handle id), edgeKind?, gap, shift, rise, rotation, turn, tilt, x, y, sourceTip?, targetTip?, visible?, locked?`.
- `Puzzle2dTargetRegion` L472: `id, x, y, width, height, label?, hidden, locked` (fill-constraining rectangle; `bounds()` normalizes negative extents).
- `Puzzle2dMeta` L528: `manifestId?, kindCompatibility: Vec<{source,target,bidirectional,important,specificity}>, kindCatalogs?` (`Puzzle2dKindCatalogs` L445 = node/handle/edge/wire kind tables).
- `Puzzle2dCamera` L36 exists in the snapshot but there is deliberately no camera mutation: the camera is per-window config (see 2.1). Documents "only seed" a new window (`ED/🪟️window/🦀️.rs` L382 `document_seed`).

### 1.3 Diff (`ANY/🧬️schema/🔺️diff/🦀️.rs`)

`Puzzle2dDiff` L11: sparse `{artifact?, schema?, camera?, nodes?: Puzzle2dNodesDelta, edges?: Puzzle2dEdgesDelta, targetRegions?: Puzzle2dTargetRegionsDelta, meta?}`.
Each `*Delta` = `{added: Vec<T>, removed: Vec<id>, patched: Vec<{id, patch:{replacement: Option<T>}}>, reordered: Option<Vec<id>>}`; a patch is a WHOLE-ITEM replacement built from the base at diff time.
Consequence: a mutation's diff depends on the base it is applied to, so replaying a stored mutation on a changed base is well defined (it re-derives), as long as the target exists.

### 1.4 Mutations (`MUT/🦀️.rs`)

- `enum Puzzle2dMutation` L36: 33 single-field tuple variants, `#[value(tag="mutation", rename_all="camelCase")]`, derives `dsl::Mutations` (generates `Mutation<Puzzle2dSnapshot>` + `SemanticMutation`), `dsl::DslEnum`. `KINDS` L77 lists the 33 kebab names. `puzzle2d_snapshot_mutations(before, after)` L152 diffs two typed snapshots into a mutation list (this is the function every scratch-fixture edit funnels through, see 2.3). `apply_puzzle2d_mutation` L322, `inverse_puzzle2d_mutation` L329, `puzzle2d_document_delta_operations` L396 (JSON before/after, errors on undecodable side), `Puzzle2dPlaySnapshot` L409-569 (typed authority + lazily materialised `serde_json::Value`).
- All 33 leaves declare `invertibility: explicit-mutation`, `diffParticipation: detect`, `composition: atomic`, `outcomeClasses: [applied, no-op, rejected]` (checked by script over every leaf `🔣️.json`). Inverses are computed from the base snapshot (`↩️inverse/🦀️.rs` per leaf).
- Leaf anatomy: `MUT/<emoji><slug>/{🔣️.json (descriptor), 🧬️schema/🔣️.json (payload JSON schema), 🦀️.rs (payload struct + builder + MutationKind impl), 🔺️diff/🦀️.rs, ↩️inverse/🦀️.rs, 🧪️tests/<vector>/🦀️.rs}`. Binary tags 0..32 (next free = 33), text opcode = the kebab kind, e.g. `move-node <id> <x> <y>` (`MUT/📝️text/📖️.grammar.semio` L18).
- Outcomes: `MutationOutcome` with `MutationMessage {level: Severity, code, message, target, op_index}` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` L953/L1066). Puzzle 2d uses `mutation.target-missing` (error, e.g. `📍move-node/🔺️diff/🦀️.rs`) and `mutation.no-op` (warning). The frozen code set is 7 (`target-missing`, `no-op`, `partial`, `clamped`, `duplicate-id`, `invariant`, `cascade`; no per-plugin codes; ticket `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/📋️contract-freeze.md` C2), levels `Info < Warning < Error < Fatal`, `MergePolicy {LaissezFaire, Normal, Vigilant}` decides what blocks.

All 33 kinds, payload fields (Rust snake_case; JSON is camelCase), binary tag:

| tag | kind | payload |
|---|---|---|
| 0 | create-node | node: Puzzle2dNode, index?: usize |
| 1 | delete-node | id (inverse = create-node + one connect-handles per attached edge) |
| 2 | move-node | id, newX, newY (ABSOLUTE final position) |
| 3 | replace-node-geometry | id, newShape?, newRadius?, newWidth?, newHeight? |
| 4 | change-node-kind | id, newNodeKind? |
| 5 | edit-node-text | id, newText? |
| 6 | change-node-icon | id, newIconKind? |
| 7 | scale-node | id, newScale? |
| 8 | change-node-visible | id, newVisible? |
| 9 | change-node-locked | id, newLocked? |
| 10 | change-node-root | id, newRoot? |
| 11 | change-node-anchor | id, newAnchor (Fixed/Derived) |
| 12 | add-node-handle | nodeId, handle, index? |
| 13 | remove-node-handle | nodeId, handleId |
| 14 | replace-node-handle | nodeId, handleId, newHandle |
| 15 | connect-handles | id, source, target, edgeKind?, gap, shift, rise, rotation, turn, tilt, x, y, sourceTip?, targetTip? |
| 16 | disconnect-handles | id |
| 17 | replace-edge-geometry | id, newGap, newShift, newRise, newRotation, newTurn, newTilt, newX, newY |
| 18 | change-edge-kind | id, newEdgeKind? |
| 19 | change-edge-tips | id, newSourceTip?, newTargetTip? |
| 20 | change-edge-visible | id, newVisible? |
| 21 | change-edge-locked | id, newLocked? |
| 22 | change-manifest-id | newManifestId? |
| 23 | connect-kind-compatibility | source, target, bidirectional, important, specificity |
| 24 | disconnect-kind-compatibility | source, target |
| 25 | replace-kind-catalogs | newCatalogs? |
| 26 | create-target-region | targetRegion, index? |
| 27 | delete-target-region | id |
| 28 | move-target-region | id, newX, newY (ABSOLUTE) |
| 29 | resize-target-region | id, newWidth, newHeight |
| 30 | edit-target-region-label | id, newLabel? |
| 31 | change-target-region-hidden | id, newHidden |
| 32 | change-target-region-locked | id, newLocked |

Files that enumerate the kinds and must change when a kind is added (grep for `resize-target-region`, the newest kind): `A2D/🦀️.rs` (mod tree), `ANY/🔮️oracles/🔣️.json`, `MUT/🔣️.json` ("33 kinds" in its description and 33 `$ref`s), `MUT/🦀️.rs` (enum, `KINDS`, re-exports, `puzzle2d_snapshot_mutations`), `MUT/🟦️.ts`, `MUT/📝️text/{📖️.grammar.semio,🔤️.ebnf,🅰️.g4}`, `MUT/💾️binary/📡️.protocol.semio`, `FIX/🔣️.json`, `ANY/🧪️tests/◻️mutate-puzzle-2d-1/{🥒️.feature,🦀️.rs,🐍️.py}`, `ANY/🧪️tests/🕸️third-party-puzzle-2d-1/{🥒️.feature,🐍️.py}`. Generated facet leaves (`🔗️.graphql`, `🛰️.proto`) did not name the kind in that grep.

### 1.5 Inferences (`ANY/🧬️schema/💡️inferences/`)

One inference: `Puzzle2dInference { flat_position }` (schema id `s.puzzle.puzzle2d.inference`, reads `nodes`, `edges`); `🎛️flat-position/🦀️.rs` runs `fastened_layout_snapshot` (BFS: `Fixed` nodes keep x/y, `Derived` nodes resolve from edge params) and returns `positions: {nodeId: {x,y}}`. Uncached, whole-snapshot. It is a derived read model, never a mutation input.

### 1.6 IO (`ANY/🚪️io/🦀️.rs`, 205 lines)

Import stdio kinds: json, txt (L4). Export: dwg, dxf, json, pdf, png, svg, txt (L7). `Puzzle2dComposerComposition` (import from native/json/txt dialects), export `ComposerEntry`s per target dialect, `puzzle2d_board_drawing(snapshot)` (L~180: nodes/edges/regions as one diagram, shared by all page exports). The editor also has `exportFixture`/`importFixture`/`openImportFixture` actions (whole-document replace goes through `store::ArtifactStore::reset`, deliberately not a mutation; see the note at `MUT/🦀️.rs` L23-30).

## 2. The app (editor)

### 2.1 Structure

- Mode `edit` (`ED/🎭️modes/✏️edit/🦀️.rs`, `PUZZLE2D_PLAY_MODE_EDIT`); tools `[fill]`; triptych layout `row [50,25,25]` of the three windows. Viewer mode `view` (`ANY/👁️viewer/🎭️modes/👁️view`): one full-pane `board` window.
- Windows (`ED/🎭️modes/✏️edit/🪟️windows/`): `👁️overview` (`2d-overview`, body `puzzle2d.play.overview`, the ONLY interactive pane, `interactive: pane == overview` in `puzzle2d_board_scene`), `🔍️detail` (`2d-detail`), `🎯️selection` (`2d-selection`). Detail/selection are non-interactive mirrors that render the same fixture with different pane cameras. All three declare `InteractionRef("vortex")`.
- Utilities (overview only, `🪟️windows/👁️overview/🪛️utilities/`): `select` (default pointer utility: pick + rectangle/lasso marquee + gumball), `brush` (`brush`), `areaBrush` (paints target regions). Active utility is per window (`puzzle2d_active_utility`, `ED/🦀️.rs` L271).
- Window measures/options (`🎭️modes/✏️edit/☑️options/`): `🔭️lod`, `🌐️grid`, `🎯️select` (which granularities a pick may reach), `🔄️transform` (gumball `move`/`rotate` flags), `🖌️brush`; plus `areaBrush` extent sliders.
- Tool (mode-level, `🛠️tools/🪣️fill`): `fill`, a mutating framework tool run (`ToolRunRebasePolicy::Revalidate`, `ToolRunReconfigurePolicy::Resume`, trace `Placement2d`, jobs `puzzle2d.fill.run`/`puzzle2d.fill.revalidate`). Its typed placement mutations go to the tool-run ledger and finalize publishes them as ONE `Edit` (`TOOL_RUN_GROUP_ID_PREFIX = "toolRun:"`, `🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs` L56).
- Panels (`ED/📌️panels/`): `🗿️artifact` (document tree / layers, virtualised, rows are `vortex` pick targets), `🛍️catalogue` (kind rows, drag sources), `🔍️inspection` (properties of the selection), `⚙️settings`.
- Actions: 55 `Puzzle2dCommand` variants (`ED/🦀️.rs` L1782-1841, macro `puzzle2d_command_variants!`): `addNode, setActiveExample, deleteSelection, duplicateSelection, forceLayout, focusSelection, selectSameKind, createEdge, deleteEdge, proximityConnect, setProximityRadius, setSelectionFlag/Hidden/Locked, patchInspectorNodes, redrawHandles, reorganize, applyBoardEvents, setFillCount, acceptSuggestion, setCamera, engagementInput/Submit/Abort/ControlSelect/RepeatLast, setLodModeForPane, setGridSnapEnabled/Factor/Visible, setSelectableKind, setBrushPlacementContactTolerance/OverlapBudget, openAddNodeDialog, setBrushKindWeights/NodeSize, setSuggestionOffset, setTransformGumballFlag, addTargetRegion, deleteTargetRegion, relocateTargetRegion, setTargetRegionFlag/Hidden/Locked, setAreaBrushSize, cycleBrushCandidate(+Back), targetBrushSuggestions, hoverSuggestion, openHandleSuggestions, closeHandleSuggestions, lodScaleJson, translateSelection, rotateSelection, scaleSelection, exportFixture, importFixture, openImportFixture`. Plus framework-injected `setActiveTool`, `setActiveUtility`, undo/redo/copy/cut/paste/checkpoint verbs and the six reserved interaction verbs (`interactionSelect`, `interactionHover`, `clearSelection`, `selectAll`, `setSelectionMode`, `setInteractionGranularity`; `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` L1312-1325).
  Registration/kind (Mutation vs View vs Shell): `create_puzzle2d_app()` `ED/🦀️.rs` L5425-5680; retained-tool ids L2085; publication lanes per verb `PUBLICATION_CONTRACTS` L2305-2366 (Artifact, Config, WindowConfig, WindowTransient, Interaction, HostOnly).
- Commands directory: 50 `ED/🎮️commands/<emoji><slug>/🦀️.rs` arms (55 action variants because several share an arm, e.g. `setSelectionHidden|Locked`, `reorganize`, inline `openAddNodeDialog`) (one file per verb; `apply-board-events`, `translate-selection`, `proximity-connect`, `patch-inspector`, ...). Commands are pure reducers over `Puzzle2dActionCtx` (L1858).
- State classes: persisted-shared = the document snapshot (VCS store); persisted-local = `Puzzle2dConfig` (`ED/🎚️config/🦀️.rs`: kind weights, fill count, contact tolerance, overlap budget), per-window `Puzzle2dWindowConfig` (`ED/🪟️window/🦀️.rs` L15: camera, LOD, grid, snap, suggestion offset, proximity radius, area-brush extent, gumball flags, selectable kinds) and the selection (see 2.2); ephemeral-local = `Puzzle2dWindowTransient` (L182: engagement input, brush candidates, suggestion menu) and hover; ephemeral-shared = `Puzzle2dPresence` (`ED/👥️presence/🦀️.rs`, camera only). Nothing gesture-shaped (no drag state) is in any of them.

### 2.2 Selection representation

- Framework interaction domain `vortex`, declared by `puzzle2d_interaction_definition()` (`ED/🦀️.rs` L249): granularities `node`, `edge`, `handle` (a `targetRegion` granularity const exists at L81 but is not declared in the definition; unknown ids default to `node` in `puzzle2d_selection_targets` L189), hierarchy `Flat`, hover channel `pointer` (broadcast, not transitive), modes `Multiple|Single`, methods `Pick|Rectangle`, merges `Replace|Additive|Subtractive|Invertive`.
- Not in the document, not in `Puzzle2dConfig`, not in window config/transient. Stored in the plugin's `interaction_store` (own artifact store) via `InteractionConfigMutation::set_state`, dispatched on `HistoryLane::Interaction` (`PLUG` L28071-28107; lane doc `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` L2626-2650: "persisted, replayable SIDE history ... default Undo/Redo skip past them"). So selection is event-sourced but in a different lane/store than the document and never in document history rows (interaction verbs log as `ActionKind::Interaction`, kept out of the history panel, `PLUG` L28158-28170 doc). Hover is in an ephemeral map only.
- Shape: `InteractionState { selection: {domain -> DomainSelection {granularity, ids, anchor_id}}, hover, active_mode, active_granularity }` (`🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` L2371-2448, doc: "Own persisted-local selection (Interaction history lane) + ephemeral-local hover").
- Writers: (a) client `interactionSelect` (renderer), (b) app-initiated `Emit.interaction_writes: Vec<InteractionWrite{domain, targets, merge}>` (`PLUG` L12492; applied AFTER the emit's document mutations, `apply_interaction_writes` L28133). 2d uses (b): `board_selection_write` (`ED/🎮️commands/🎲️apply-board-events/🦀️.rs` L253) takes the LAST `select` row of a batch (`ids`, replace) and `puzzle2d_selection_write` (L211) classifies ids by document membership.
- Readers: guest commands get `ctx.selection: &DomainSelection` and `ctx.selected_ids()` (`ED/🦀️.rs` L1871/L1885); `Puzzle2dInteractionSnapshot` (L141) for rendering; the engine gets it echoed back through `Board2dScene.selection_json` (`ED/🎭️modes/✏️edit/🦀️.rs` `puzzle2d_board_scene`) into `BoardHost::set_selection_ids(_silent)`.
- Client engine also keeps its own OPTIMISTIC selection (`BH` L8702-8770 `set_selection_ids*`), `B2H` `setLocalSelectionJson` L895.

### 2.3 Drag/move gesture, pointer event to mutation

Numbered pipeline (line refs; `BH` = board engine, `B2H` = React host, `ED` = guest):

1. DOM: `B2H` L1336 `onPointerDown` -> `session.pointerDownScreen(...)` (wasm `BoardHost`). The wgpu renderer routes through the same `BoardHost` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` L4056-4064, L5564).
2. `BH::pointer_down_screen` L12480: on a draggable, unlocked node hit (L12556-12582) the engine (a) merges the pick into its selection and emits a `select` event via `set_selection_ids_gestured` (L8748) unless the node already belongs to a multi-selection group, (b) enters `Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair }` (L12579). `start_positions` = current x/y of the whole dragged group.
3. `BH::pointer_move_screen` L12618 (`DragNodes` arm L12658-12711): computes one common `(dx, dy)` from the pointer, optionally grid-snapped (`snap_world_pair`), writes new x/y into the ENGINE's own node table (`n.x = ...`, `bump_content_scene_generation`) and pushes one `nodeMove {id,x,y}` event per node. This is the live preview: engine-local, never sent to the guest as state. A single-node drag also tracks a `proximity_pair` for a live link preview.
4. `B2H::onPointerMove` L1356-1375: `drainAndMaybeFlush` (L912) -> `drainIntoBuffer` (L875) appends rows; `coalesceBoard2dEvents` (L314-344, unit-tested in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` L5887-6033) drops transient rows (`preselect, brushPreview, linkCompatibleNodes, linkTargetRing, transformPreview, hover`, L310), keeps the last `camera`, coalesces `nodeMove` to one row per id and DROPS them all when a `nodeDragEnd` is present; `select` (and others, L311) set `flushNow`. Sibling panes preview through `collectPuzzle2dLiveMirrorMutations` (L366) + `pushPuzzle2dLiveMirrorMutations`, i.e. client-side imperative mirroring, not guest state. The wgpu host has a Rust port of the coalescer (`coalesce_owned_board_events`, wgpu `🦀️.rs` L5164) and posts `applyBoardEvents` with an `eventsJson` string (L5254-5260).
5. `BH::pointer_up_screen` L12790 (`DragNodes` arm L12853-12863): commits a proximity link if one was tracked (`try_commit_link_edge`, event `ProximityConnect`) and calls `push_node_drag_end_events` (L8038) -> ONE `nodeDragEnd { moves: [{id,x,y}, ...] }` row with the FINAL ABSOLUTE positions of every dragged node. Escape/leave/cancel (`pointer_cancel_screen` L12928) restores `start_positions` in the engine and emits nothing.
6. `B2H::onPointerUp` L1378-1391: `settleGestureEnd` + `dispatchBufferedEvents` (L903) -> `dispatch("applyBoardEvents", {eventsJson})`.
7. Guest: `Puzzle2dCommand::ApplyBoardEvents` -> `puzzle2d_dispatch_emit` (`ED/🦀️.rs` L2877-3013). This rebuilds a fresh scene and a fresh `BoardHost` from the document every dispatch (L2896), runs the arm, then computes the document delta by DIFFING the scratch JSON fixture (L2994 `puzzle2d_document_delta_operations(before, &scene.fixture)`), appends any explicit `artifact_mutations`, and derives config/window-config/window-transient mutations by inequality (L3003-3006).
8. `apply_board_events_from_json` (`ED/🎮️commands/🎲️apply-board-events/🦀️.rs` L95-235): `nodeDragEnd` (L116-146): refuse the whole batch if any moved id is locked (`puzzle2d_addresses_locked_entity`, returns `refused_locked`, one notice), else `patch_inspector_nodes(fixture, [id], "x"/"y", value)` per move (L1000), then `puzzle2d_proximity_connect(fixture, dropped, runtime.proximity_radius)` (`ED/🎮️commands/🔗️proximity-connect/🦀️.rs` L105): for each moved node, nearest compatible open handle pair within radius becomes a new edge `{id: new_edge_id(), source: peer, target: moved}`, capped 8 per node and 64 per gesture (`PUZZLE2D_PROXIMITY_CONNECT_MAX/GESTURE_MAX`, `ED/🦀️.rs` L89-93). Edge ids are minted from a process-global atomic counter (`new_node_id`, `ED/🦀️.rs` L694-698; collisions re-minted, L1245-1262).
9. Delta -> mutations: `puzzle2d_snapshot_mutations` (`MUT/🦀️.rs` L152): per node with changed x/y a `move-node(id, x, y)`; new edges become `connect-handles`. Order: deletes, then per-node moves, then edge creates. Interaction write: `board_selection_write` adds the `select` row's ids as `InteractionWrite::Replace` (`apply_board_events` L266-281).
10. Publication: the migrated tool ladder drains the whole artifact lane into ONE staged `Edit` (one ledger slot, one undo step): `PLUG` `publish_mounted_typed_operation_unit` L30670 (doc block "ONE gesture is ONE batched publication"), per item prepared by `Puzzle2dArtifactStorePreparation` (`ED/🦀️.rs` L2565-2731: computes `mutation.inverse(base)` and the post root, stamps `MutationMeta` with `undo_policy: ExactBaseOnly`, `group_id: None`, `dependencies: []`). Interaction writes are applied afterwards to the other store (`apply_interaction_writes`, `PLUG` L28133).

Quirk (by code reading, NOT runtime-verified): a drag that starts on a not-yet-selected node emits `select` at pointer-down (step 2). `select` is in `PUZZLE2D_FLUSH_NOW_EVENT_NAMES` (`B2H` L311), and rows drained on the first pointer-move give `flushNow = true` with `nodeMove` still coalesced in, so `dispatchBufferedEvents` sends `[select, nodeMove(first tick)]` mid-gesture. `nodeMove` is folded into the fixture by `apply_board_events_from_json` (L161), so that first tick can commit a `move-node` Edit; the release then commits a second Edit (`nodeDragEnd`). `settleGestureEnd`'s own comment (`B2H` L976-983) describes this "early mid-gesture flush". Verify at runtime by counting history rows after one drag on an unselected node (probe recipe in section 3.4).

### 2.4 Preview during drag

Preview is entirely client engine state: `BoardHost` mutates its own node/handle table (`BH` L12692-12708; rotate ring: `update_transform_drag` L13153 rewrites positions AND handle angles and emits only transient `transformPreview`, "Emits nothing - a rotate is ONE document edit, on release"). Peer panes mirror via JS-side `pushPuzzle2dLiveMirrorMutations`. There is no guest transient state for drags (`Puzzle2dWindowTransient` L182 holds no gesture). A `BoardEventKind` payload cap of 16 KiB per event (`BH` L1590) and batch cap 256 events (`PUZZLE2D_BOARD_EVENT_BATCH_LIMIT`, `ED/🦀️.rs` L2738) bound what one release can carry (a 180-node `nodeDragEnd` is roughly 12 KiB by my estimate; not measured).

### 2.5 What is committed on release, and how history rows appear

- Committed: `move-node` per dragged node (absolute), `connect-handles` per proximity pair, all in one `Edit` whose `forwards` is that list; `inverse` is frozen at commit (per-item `inverse(base)`); selection lands separately in the interaction store.
- History row: one `CommandLogEntry` per dispatch (`record_command`, `PLUG` L26646; `dispatch_emit_inner` L27284). Row fields `CommandView {seq, action_id: "applyBoardEvents", label, kind: Mutation, edit_id, op_lines, op_count, applied, revertible}` (`build_history_view` L26822). `label` = the action label ("Apply Board Events" / "Board-Ereignisse anwenden") because the emit sets no `description`; `op_lines` = `print_op()` of the newest 8 forwards, e.g. `move-node <id> <x> <y>` (`HISTORY_ROW_OPERATION_PREVIEW = 8`, L12040). A `select`-only flush produces a row with `edit_id: None` ("Without Operations" filter): confirmed for the migrated ladder by `record_settled_typed_operation_command` (`PLUG` ~L26105-26125) and `record_typed_operation_lane` (~L26065-26095), which call `record_command(verb, kind, None, ...)` and so fall back to the action label. The ledger capacity is 64 edits (`ARTIFACT_HISTORY_LEDGER_CAPACITY`; law `sequential_small_edits_honour_the_fixed_edit_ledger_ceiling`, `ED/🧪️tests/🔬️unit/🦀️.rs` ~L475).
- Streamed programmatic transforms (`translateSelection`/`rotateSelection`/`scaleSelection`, keyboard nudges, gumball) use `Emit.coalesce_key` (`puzzle2d-gesture-translate|rotate|scale`, `ED/🦀️.rs` L2862) so each tick `AmendLast`s the same Edit (`PLUG` L27414; law `transform_gesture_ticks_coalesce_into_one_undo_step`, `ED/🧪️tests/🔬️unit/🦀️.rs` ~L445). `applyBoardEvents` never sets a coalesce key.

### 2.6 Every input path converges on the same absolute mutation

`nodeDragEnd`/`nodeMove` (board), `translateSelection {dx,dy,step}` (palette arg form `ED/🦀️.rs` L5548, keyboard, gumball; `ED/🎮️commands/🚀️translate-selection/🦀️.rs`: ambient selection, moves selected nodes AND target regions, then proximity-connect), engagement HUD text `move <dx> <dy>` (`ED/🎮️commands/📨️engagement-submit/🦀️.rs` L60), inspector `patchInspectorNodes {ids?, field, value|delta}` (`ED/🎮️commands/🩹️patch-inspector/🦀️.rs`; explicit `ids` else selection; absolute `value` wins over `delta`), `relocateTargetRegion`/`regionMove`. All are folded into the scratch JSON and diffed into `move-node`/`move-target-region`. The offset and target set never survive.

## 3. Tests, fixtures, run recipe

### 3.1 Language-agnostic mutation corpus (per kind)

- Fixture quintets (JSON): `FIX/<emoji><kind>/<emoji><vector>/{📸️snapshot/⬅️before/🔣️.json, 📸️snapshot/➡️after/🔣️.json, 🦠️mutation/🔣️.json, 🔺️diff/🔣️.json (or 🚫️.absent for refusals), 🎯️outcome/🔣️.json {"status":"applied|no-op|rejected"}}`. 33 kind dirs + the `FIX/🔣️.json` index, 90 vector dirs (ls-counted); derived `.op/.spr/.dsl/.pack/.patch.semio` encodings come from `fixtures generate` (see header of `MUT/📍move-node/🧪️tests/📍️moves-node-a/🦀️.rs`). Example: `📍move-node/📍️moves-node-a/🦠️mutation/🔣️.json` = `{"id":"node-a","mutation":"moveNode","newX":5.0,"newY":7.0}`, diff = whole-node replacement patch.
- Rust leaf tests: `MUT/<leaf>/🧪️tests/<vector>/🦀️.rs` (92 dirs, grep-counted), each asserting apply-to-committed-after, inverse restores before, canonical JSON, declared outcome, produced diff equals committed diff, committed diff applies (see `📍️moves-node-a` for the 7-test template).
- Cross-implementation cases (Gherkin + independent implementations): `ANY/🧪️tests/◻️mutate-puzzle-2d-1/{🥒️.feature, 🦀️.rs, 🐍️.py}` (Python second implementation, exhaustive per-kind tables), `ANY/🧪️tests/🕸️third-party-puzzle-2d-1/{🥒️.feature, 🐍️.py}` (networkx graph cascade, shapely geometry, jsonschema, jsonpatch/deepdiff), `ANY/🧪️tests/🌐️third-party-puzzle-2d-1/{🥒️.feature, 🟦️.ts}` (graphology, jsonschema, fast-json-patch). Oracle registry `ANY/🔮️oracles/🔣️.json`. Status log claims all three green on 2026-09-06/09-17 (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/📓️status.md`); not re-run here.
- Other artifact tests: `A2D/🧪️tests/🔬️unit/🦀️.rs` (5), `ANY/🧬️schema/💡️inferences/*/🧪️tests`, `ANY/🚪️io/🧪️tests`, `ANY/👁️viewer/🧪️tests`.

### 3.2 Editor and engine tests (Rust)

`ED/🧪️tests/🔬️unit/🦀️.rs` (1675 lines, ~64 tests; harness `context::app_with_registry`, `dispatch`, `select_id`, `load_example`): includes `dragging_a_node_keeps_the_painted_board_parseable` (feeds `applyBoardEvents [nodeMove, nodeDragEnd]`, ~L1004), `board_select_row_reaches_the_inspector_and_the_board`, `transform_gesture_ticks_coalesce_into_one_undo_step`, the 64-slot ledger law, undo/redo, pack round trips. Also `ED/🧪️tests/🔬️locks` (6), `🔬️clipboard` (5), `🤖️agent-lane` (1), `ED/⚙️engine/🎲️board-host/🧪️tests` (35), `⚙️engine/🖌️brush/🧪️tests` (1607 lines), `⚙️engine/📐️layout` (815), `⚙️engine/🔗️linking` (924), `ED/🎮️commands/🔗️proximity-connect/🧪️tests` (5), `ED/⏳️precompute/🪣️fill/🧪️tests`, panel and window unit tests. No test asserts how many Edits or history rows one drag produces (grep found none), and none asserts the ordering select-then-move.

### 3.3 Framework and host tests touching the gesture

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (coalescer, hover, granularity) - TS/vitest.
- `♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`, `📺️renderer/.../⚙️EngineCanvas/🧪️tests/{🧊️wgpu-standalone,🔬️wgpu-board2d-engine}/🦀️.rs` (engine drag events, `nodeDragEnd`).
- Plugin-level `P/🧪️tests/*` (`🔬️surface`, `🪪️session-factory`, `🎚️renderer-contract`, `◻️storybook-2d`, interactivity fill laws), stories `P/📖️stories/🎭️2d-board`, `🎭️2d-fixtures` (contain `nodeDragEnd` scenarios).

### 3.4 Build and run

- Dev uses `.vscode/launch.json` only. Entries (launch.json line numbers; note the file currently has staged AND unstaged peer edits, `MM`): `🛠️dev🧩️puzzle◻️2d⚛️react` L2186 (`bun nx run workspace:dev -- puzzle2d`; env `S_OS_PORT=6012`, `SEMIO_PLUGIN=puzzle2d`, `SEMIO_RENDERER=react`, `SEMIO_APP=s.puzzle.puzzle2d@1/*#editor`), `🛠️dev🧩️puzzle◻️2d🧊️wgpu🌐️wasm` L2208 (port 6112, renderer wgpu), `🛠️dev🧩️puzzle◻️2d🧊️wgpu🖥️native` L2230 (`bun nx run @semio-tech/framework-renderer-wgpu:native -- puzzle2d`), `🛠️dev📖️storybook🧩️puzzle◻️2d` L2963 (`bun nx run workspace:dev-storybook-puzzle-2d`, port 6010). Seed `.vscode/🧩️launch.seed.jsonc`: generator row `puzzle2d {namePrefix "🧩️puzzle◻️2d", order 220, wgpuOrder 220.1}` L5021, native row L1192 (`@generated:puzzle2d:react|wgpu` markers L1189-1190), storybook L1633.
- Registration: `P/📦️packages/🦀️rust/Cargo.toml` `[[package.metadata.semio.playground]] variant="puzzle2d"`, ports react 6012 / wgpu 6112, `devContribution` `P/🧑‍💻dev/🧬️schema/🔣️.json`, browser entry `P/🧑‍💻dev/🚀️entry/🟦️.ts` (`bootFrameworkOsDev` + `PUZZLE_BOARD_SESSION_FACTORIES`). Root `📜️script.ts` `dev` (L243-262) resolves to nx `dev|serve-puzzle2d-react-dev`; the target `serve-puzzle2d-react-dev` exists in `.nx/workspace-data`. Older recipe from memory and the 09-06 ticket: `bun nx run @semio-tech/framework-os-dev:activate-puzzle2d-react-dev` (wasm-pack engine ~2-6 min + component ~1-4 min), then serve under `🔁️serve-supervisor.sh` (Vite exits silently after headless runs), open `http://127.0.0.1:6012/?plugin=puzzle2d`. I did not re-verify the `activate-` target name.
- Browser battery: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/PUZZLE-2D-END-TO-END/🔍️browser-probe.ts` (2682 lines, Playwright): `bun 🔍️browser-probe.ts --only=drag-node --port=6012` (step `drag-node` at L1000 drags the first node +80,+40 and asserts `data-board-positions-json` of `window:2d-overview` moved; it does not count edits). Other steps: `locked-node-refuses-drag`, rotate-handle drag, proximity drop.
- Rust tests (never run in this audit): `bun nx run @semio-tech/puzzle-2d-rs:test` (`P/🗿️artifacts/◻️2d/📦️packages/🦀️rust/📋️project.json`, crate `semio-s-artifact-puzzle-2d`), `bun nx run @semio-tech/puzzle-plugin:test-quick|test-long|test-exhaustive|fixtures-lint|canonical-architecture` (`P/📦️packages/🦀️rust/📋️project.json`, cargo test of `semio-s-plugin-puzzle`). Editor tests are behind `feature = "component-app-assembly"` (`A2D/🦀️.rs` L1571), so a filtered run is `cargo test -p semio-s-artifact-puzzle-2d --features component-app-assembly <filter>` (Cargo.toml features/dev-deps read; command not executed).

## 4. Comparison (how generic must the solution be)

| aspect | puzzle 2d | puzzle 3d | puzzle 5d | draw (`✏️s/🔌️plugins/🖍️draw`) |
|---|---|---|---|---|
| gesture host | client `BoardHost` (wasm/wgpu), engine-local preview | World3d host, client gumball + `WorldGumballTransformPreviewStore` (ticket `☀️13/PUZZLE-3D-TRANSFORM-GUMBALL-PREVIEW`) | part drag (ticket `☀️23/PUZZLE-5D-PART-DRAG-PREVIEW-IN-BOTH-WINDOWS`), embeds a 2d board window with the same `apply-board-events` | guest-side statechart per window (`fsm::statechart!` at `🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs` L433: idle, moving_layer, marqueeing, shape_dragging, drafting) with per-window transient scratch |
| commit vocabulary | absolute `move-node {id,newX,newY}` | absolute `move-object {id,newOrigin:[f64;3]}` (+ rotate/scale-object) | `move-part2d`, `move-part3d` | absolute `update-layer-transform {layerId, DrawingTransform}` computed in `finish_layer_move` L1367 |
| delta command over ambient selection | `translateSelection {dx,dy,step}` | `translateSelection {dx,dy,dz}` as a staged `Puzzle3dScaleWork` state machine (`🧊️3d/.../✏️editor/🦀️.rs` L4528-4740; stages ObjectSelection, VolumeSelection, Objects, Volumes, Complete) | same family | `nudge_selection::plan_selection` |
| selection | `vortex` domain (node/edge/handle) | domain with object/vortex/attraction/targetVolume/reference/kind | same | `DRAWING_INTERACTION_DOMAIN`, driven by `Effect::ReplayShellCommand{interactionSelect}` |
| coalescing | translate/rotate/scale keys; board drag none | `gumball-translate|rotate|scale` (L3576) | same | `Emit::commit(mutations, "Move selection")` |
| derived follow-ups on drop | proximity auto-connect (new `connect-handles`) | attraction re-derive (`puzzle3d_rederive_moved_attractions` L2062, `resolve_puzzle3d_attractions`) | grips/fasteners | none seen |

Conclusions: (1) All four turn a DELTA gesture over an AMBIENT selection into ABSOLUTE per-entity final-state mutations, losing targets and offset; the gap is framework-wide, so the transaction/input mechanism cannot be puzzle-specific. (2) 2d is a good reference implementation because its whole chain is small and already isolated (`apply_board_events_from_json` is the single choke point where board events become document edits). (3) Two state-machine mechanisms already exist and are candidates for "tools are state machines": the framework machine crate (`🧰️framework/🔨️modules/🔄️machine/🦀️.rs`, `statechart!` derive in `🔄️machine/✨️derive`, XState-parity static tables, `Host`, `persist`, `run_conformance`) and draw's copy (`🖱️canvas-pointer-down/🔄️fsm`, crate `semio-s-plugin-draw-fsm`); `BoardHost`'s `Interaction` enum (`BH` `enum Interaction` variants None, Pan, DragNodes, SelectionPending, Selection, LinkAtSourceHandle, LinkDragSnap, LinkTargetNode, ExternalLinkPreview) is a hand-rolled equivalent inside a 13.7k-line file.

## 5. What must change (2d-specific, plus the framework seams it exposes)

### 5.1 Facts that drive the design

1. Lost inputs: `move-node`/`move-target-region` are absolute; the drag's target set and offset exist only transiently in the engine and the `select` interaction write. Editing an absolute downstream op upstream is masked (last writer wins), so editing "the drag" upstream of a later absolute `move-node` on the same node would appear to do nothing. Gesture-level mutations must be relative/parametric to be meaningfully editable.
2. Selection lives in another store/lane (`Interaction`), applied after the document lane, and reducers read it ambiently (`ctx.selected_ids()`). Replaying a document Edit at another point in time cannot re-read "the selection back then".
3. The tool (board drag) is a client-side state machine (`BH` `Interaction`) whose terminal output today is `select` + `nodeMove`* + `nodeDragEnd` rows with no gesture identity; the `select` row is flushed separately (flush-now) so one gesture can already be two dispatches (quirk in 2.3).
4. Mutations are derived by DIFFING a mutable JSON scratch fixture (`puzzle2d_document_delta_operations`). The tool never authors mutations; the diff decides their kind. There is no place to attach inputs.
5. `Edit.inverse` is computed eagerly at commit against the exact base and `undo_policy: ExactBaseOnly`, `MutationMeta.group_id/dependencies` are unused for 2d. After an upstream edit, downstream inverses are stale and must be recomputed during replay.
6. Reducers mint ids from a process-global counter (`new_node_id`), so re-running a tool is not deterministic; replaying stored mutations is (ids are stored in `connect-handles`/`create-node`).
7. Derived follow-ups (proximity `connect-handles`) depend on geometry at drop time. After an offset edit they are either kept as recorded (stable, may become semantically odd) or must be re-derived (needs the tool again, contradicting "tools are not editable").
8. Replay-report vocabulary already exists: `MutationOutcome`/`MutationMessage` (Info/Warning/Error/Fatal), `MergePolicy`, `mutation.partial` (survivors only) which fits "translate over a target set where some ids are gone or locked", `op_index` stamping for batch replay, `MutationMeta.dependencies` for data dependencies.
9. UI-element metadata vocabulary already exists for ACTION ARGS: `ActionArgDef {schema: ArgSchema::Number{min,max,step,integer,unit}|String|..., presentation: Slider|IconSelect|Multiline|Hidden, required, default}` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` L158-345; `translateSelection` already declares `dx`/`dy` number args at `ED/🦀️.rs` L5548). Missing: snap points, an id-list/selection-picker presentation, and any per-field metadata on mutation payloads (`MutationLeafDescriptor` has 14 static fields and none of it; payload JSON schemas already carry custom `x-semio-state` / `x-semio-derived` keys, so `x-semio-input` is the natural extension point).
10. Bounded capacities: 64-edit ledger, `PUZZLE_COMMAND_WORK_ITEMS` 4096, decoded items 512, board event 16 KiB, batch 256, gesture edge budget 64, selection batch 1024. Replay of a tail is bounded by the ledger; the transaction payload (ids + offset) is much smaller than today's per-node absolute `nodeDragEnd`.

### 5.2 Changes by layer

Framework (artifact-agnostic, cross-cutting; other explorers own the deep mapping):

- A transaction container that carries ORDERED steps with typed inputs, spans document and interaction lanes, and publishes as ONE `Edit` with `MutationMeta.group_id` (precedents: tool-run finalize `group_id "toolRun:<n>"`, `TransactionPrepare` `group_id = txn_id` at `PLUG` L33716; neither covers the interaction store).
- A mutation input schema: each leaf publishes inputs with UI metadata (reuse `ArgSchema`/`ArgPresentation`, add snap points and a selection-picker presentation), surfaced next to the payload JSON schema.
- A replay engine that folds from the edited position, re-diffs each downstream mutation against the new base (`Mutation::diff`), recomputes each `Edit.inverse`, stamps `op_index`, aggregates `MutationMessage`s and applies the `MergePolicy`.

Puzzle 2d artifact (schema-first; every new leaf follows the leaf anatomy and file checklist in 1.4):

- New relative, parametric leaf(s): `translate-nodes { targets: Vec<id>, dx, dy }` (nodes and target regions; ids classified by document membership like `puzzle2d_selection_targets`), with `diff()` reading base positions (works on any base), `inverse` from base, `mutation.partial` for missing/locked members, `mutation.target-missing` when none remain. Keep absolute `move-node`/`move-target-region` as setters for inspector absolute edits. Analogous relative forms for rotate (about centroid) and scale are the same shape (`Puzzle2dTransform`, `ED/🦀️.rs` L1028); their inputs would be `angle`, `factor`.
- The "editable select-like input" is `targets` on that leaf, filled at yield time from the interaction state, data-bound so an edit changes the moved set (former members return to their base positions because they are no longer lowered). Alternative (heavier): a separate recorded `select` step in the same transaction with `MutationMeta.dependencies` pointing from the translate to it; needs selection admitted into the shared document history although it is persisted-local state. Recommend the first; decide explicitly.
- Fixtures/tests for each new leaf: quintet vectors under `FIX/`, Rust leaf tests, rows in `◻️mutate-puzzle-2d-1` (Rust + Python) and the third-party cases (TS/Python), text grammar (`📖️.grammar.semio`, `🔤️.ebnf`, `🅰️.g4`), binary protocol tag 33+, `KINDS`, oracles registry, `🧬️mutations/🔣️.json` union. A new language-agnostic family is needed for replay: `before snapshot + ordered transaction + edited input -> after snapshot + per-op outcomes`, verified against a third-party library per AGENTS.md (e.g. jsonpatch/deepdiff plus networkx as in the existing cases).

Puzzle 2d app and board engine:

- Formalize the drag as a state machine whose terminal event is a semantic gesture record (`{gestureId, kind: translate, targets, offset, selectionChange?, dropProximityPairs?}`) instead of `nodeDragEnd{absolute moves}`; the engine keeps the live preview local. It lives once in `BH` (shared by React/wasm and wgpu hosts) but both coalescers must learn the new row: TS `coalesceBoard2dEvents` (`B2H` L314) and Rust `coalesce_owned_board_events` (wgpu `🦀️.rs` L5164) (multi-implementation rule).
- Tag `select` and the terminal row with the same gesture id and exempt them from flush-now while `pointer_lane_in_flight()` (`BH` L8827), so one gesture is ONE dispatch and one transaction. Decide cancel semantics (`pointer_cancel_screen` L12928 restores positions; should an already-derived selection change also be dropped?).
- `apply_board_events_from_json` stops mutating the scratch fixture for drags; it returns an `Emit` whose `artifact_mutations` contain the relative leaf (plus the recorded proximity `connect-handles` as ordinary follow-up steps with ids minted at yield time) and whose transaction metadata carries the inputs. `translateSelection`, the HUD `move dx dy`, the keyboard nudge, and `patchInspectorNodes {delta}` should reuse the same leaf so all input paths yield editable history.
- Ambient reads must go: replay must not consult `ctx.selection`, hover, config, window state or the id counter.
- History rows: label from the mutation label (`MutationKind::label`, e.g. "Move N nodes by (dx, dy)") instead of "Apply Board Events"; `op_lines` become the one relative op.
- Locked entities: today a lock refuses the whole gesture (`refuse_when_locked`, `nodeDragEnd` arm). In replay this must become an outcome message (partial vs error), a policy decision.
- `Derived` anchor nodes render from the `flat-position` inference; a translate of a derived node changes stored x/y but not the resolved position, so the input UI/validation should flag or exclude them.

### 5.3 Open decisions for the coordinator

1. Selection as a literal input of the leaf (recommended) vs a separate recorded `select` step (needs cross-lane atomicity).
2. Whether proximity `connect-handles` stay as recorded follow-ups (recommended; replay may warn when the pair is no longer near) or the tool re-yields them.
3. Cancel semantics for a gesture that already changed selection.
4. Where the drag state machine lives: engine-side (recommended, latency) with a guest `Emit`, versus guest-side like draw (a round trip per pointer event).
5. Which `Severity` blocks acceptance per `MergePolicy` (frozen code table: `target-missing` is Error, `duplicate-id`/`invariant` Fatal) and whether "fatal must be resolved" maps to `MergePolicy::Normal`.
6. Alternatives/finalize: the store already has `CreateAlternative`, `SwitchAlternative`, `CheckoutCheckpoint` (`🏪️store/🦀️.rs` L2947-2966); confirm they are the vehicle for "new alternative vs overwrite".

### 5.4 Suggested slicing

1. Framework: transaction + input metadata + replay engine (owner: framework explorers' conclusions).
2. 2d schema: `translate-nodes` leaf (+ fixtures, oracles, codecs).
3. 2d engine/app: gesture record + one-dispatch flush + reducer change; TS and Rust coalescers.
4. Runtime proof: extend the probe with an edit-count and history-row assertion for one drag, then an edit-input-and-replay step.

## 6. Reuse inventory (already in the repo)

`Edit{forwards, inverse, mutation_meta{group_id, dependencies, undo_policy, semantic_kind, label}}` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` L1488); `MutationOutcome`/`MutationMessage`/`MergePolicy`; `ArtifactCommand::{Apply, AmendLast, ApplyInLane, CreateAlternative, SwitchAlternative, CheckoutCheckpoint, CommitCheckpoint}`; `HistoryLane::{Document, Interaction}`; tool-run ledger + group id; transaction group undo (`transaction_undo/redo`, `PLUG` L33733-33755); `EphemeralEmit {presence, transient, window_transient}`; `InteractionWrite`; `ActionArgDef/ArgSchema/ArgPresentation`; framework `🔄️machine` statechart crate; `BoardHost::pointer_lane_in_flight`.

## 7. Unverified and risks

- Not run: any cargo/nx/vitest/Playwright command. All test and boot claims are from reading code and older ticket logs.
- The "first-tick flush commits an early Edit" quirk (2.3) is a code-reading inference; the coalescer is unit-tested, the dispatch behaviour is not measured.
- Peers are concurrently renaming committed fixture vector directories (`rejects-*` scenario dirs staged as renames in `git status`) and editing `.vscode/launch.json` and the puzzle `📜️script.ts`; expect churn if a leaf is added.
- `ED/🦀️.rs` grows to 5.7k lines and is mounted by `#[path]`; adding leaves means touching the 1.4 file list, so serialise that slice.
- Older memory notes (2026-09-16 boot recipe, 198 stale editor tests) may be stale; the 09-17/09-29 tickets moved parts (for example `☀️29/PUZZLE-2D-NODE-ICON-FOLLOWS-DRAG` touched the engine icon cache only, not the drag protocol).
