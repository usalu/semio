---
name: CAD Wgpu Premigration Parity
overview: Restore the CAD module's premigration feature set (as it existed at the `premigration` git tag, before the React/JS renderer was deleted) inside the new wgpu/Rust plugin stack, without reintroducing a React renderer.
todos:
 - id: schema-foundation
   content: Extend cad_document (cad/rs/lib.rs) CadNode/CadOp/CadScene to represent real object data (typology, transform, visible, locked) instead of the id/label/kind stub
   status: completed
 - id: undoable-edits
   content: Route addObject/patchObject/translate/rotate/scale through vcs::DocumentVcsCommand + CadOp so undo/redo covers real object edits
   status: completed
 - id: rich-patch-fields
   content: Support hidden/locked/typology patch fields and add locked flag to CadObject; extend object_inspector_group with origin/rotation/scale/hidden/locked fields
   status: completed
 - id: multi-selection-inspector
   content: Extend build_properties_panel/object_inspector_group to merge values across multi-object selection instead of only using the first selected id
   status: completed
 - id: hierarchy-chrome
   content: Wire hover_command/unhover_command, actions (context menu), is_hidden, and draggable on document tree items in build_document_tree/tree_item_with_command
   status: completed
 - id: primitive-hierarchy
   content: Add primitive-level child tree items under each object in build_document_tree (depends on schema-foundation)
   status: completed
 - id: world-references
   content: Decide and implement background/context reference-overlay handling (fixtures) separate from editable CadObjects
   status: completed
 - id: model-transformations
   content: Implement transfersTo/transfersFrom transformation toolbar + applyTransformation command linking the 4 panes via real model-definition transformations
   status: completed
 - id: save-load-actions
   content: Restore saveSelected/saveInPlay/saveCurrent/loadRawRequest toolbar actions (JSON export/import) alongside existing OBJ/GLB media export
   status: completed
 - id: interaction-engagement
   content: Replace the fixed move/rotate/scale engagement with a real per-typology Interaction state machine (prompts/picks/previews/possible-engagements) plus command-input REPL
   status: completed
isProject: false
---

# CAD Wgpu Premigration Parity

## Background

The `premigration` git tag (`f8376e8486`, `🐙ueli🎆26🌙06☀️04🚩125`) marks the commit right before CAD's renderer was rewritten. In the 12 commits since (`126`→`137`), the entire React/JS CAD renderer was deleted and replaced by a much smaller Rust/wgpu plugin:

- Deleted: [`cad/renderer/core/js/index.ts`](cad/renderer/core/js/index.ts) (2810 lines) and [`cad/renderer/react/index.tsx`](cad/renderer/react/index.tsx) (1819 lines), plus their `example-slugs.ts`, `globals.css`, and package/project/script/vitest scaffolding (25 files, −13,927 lines).
- Added: [`cad/plugin/rs/lib.rs`](cad/plugin/rs/lib.rs) (1580 lines) on top of the new [`framework/renderer/wgpu`](framework/renderer/wgpu) engine and [`framework/plugin/rs/lib.rs`](framework/plugin/rs/lib.rs) plugin SDK, plus a new placeholder schema crate [`cad/rs/lib.rs`](cad/rs/lib.rs) (`cad_document`).

The new plugin reuses the old naming/ids (window ids, body keys, surface ids) but is a much thinner reimplementation. `cad_document::CadNode`/`CadOp` only supports `AddNode`/`RenameNode`/`RemoveNode` — it is a placeholder, not the full spatial Model/Object/Primitive/Typology domain described in [`cad/AGENTS.md`](cad/AGENTS.md). Confirmed via inspection that `framework/plugin/rs/lib.rs` already defines (but CAD doesn't populate) `hover_command`/`unhover_command`/`actions`/`draggable` on tree items and `input`/`control`/`possible_engagements` on `WindowEngagement` — so most gaps below are "wire CAD into existing framework capability", not new framework work.

## Confirmed gaps vs. premigration (grouped, foundational → surface)

### 1. Document schema foundation (`cad/rs/lib.rs`, `cad_document` crate)

Currently `CadNode { id, label, kind }` + `CadOp::{AddNode, RenameNode, RemoveNode}` only — a stub disconnected from the real `CadObject` array edited by the plugin (translate/rotate/scale/addObject never touch VCS/history at all; only the separate, effectively-unused "Nodes" list is undoable). Needs to become the real substrate for objects (typology, origin/orientation/scale, visible/locked flags) so undo/redo and patch operations in `cad/plugin/rs/lib.rs` operate on actual CAD content instead of a parallel stub list.

### 2. Undoable object edits (`cad/plugin/rs/lib.rs`)

`addObject`, `patchObject`, `translateSelection`, `rotateSelection`, `scaleSelection` bypass `vcs::DocumentVcsCommand` entirely (`apply_cad_translate`/`apply_cad_rotate`/`apply_cad_scale` mutate `envelope.document` directly). Route them through `CadOp` variants and `cad_history_store`/`sync_cad_history` like `addNode`/`renameNode` already do, so `undo`/`redo` (already wired to `mod+z`/`mod+shift+z`) actually cover real edits.

### 3. Rich selection patch fields

Old `patchCadPlaySelectionTarget` supported `typology | hidden | locked | name`. Current `patchObject` (`cad/plugin/rs/lib.rs:1113-1128`) only handles `field == "label"`. Add `hidden`, `locked`, `typology` support, add a `locked: bool` field to `CadObject` (currently only `visible` exists and is never toggled), and extend `object_inspector_group` with editable fields for these plus origin/rotation/scale (old inspector exposed full transform, current only exposes Label + read-only Typology).

### 4. Multi-selection support

`build_properties_panel`/`object_inspector_group` only read `selected_object_ids.first()`. Old `cadPlaySelectionAllEqual` merged values across a multi-selection (showing a shared value or blank when they differ). Extend the inspector to operate over the full selection.

### 5. Hierarchy tree chrome (`build_document_tree`, `tree_item_with_command`)

Every tree item currently hardcodes `hover_command: None, unhover_command: None, actions: None, draggable: None, is_hidden: None`. Old `cadPlayHierarchyEntityChrome`/`cadPlayHierarchyHoverHandlers` wired: hover-to-canvas-highlight, per-item context menu / actions (e.g. delete, duplicate, hide/lock toggle), `is_hidden` reflecting the object's hidden flag, and drag reordering. Wire these using the already-defined framework fields.

### 6. Primitive-level hierarchy

Old `cadPlayPrimitiveChildTreeItem`/`cadPlayPrimitiveSlotTreeItems` showed primitive slots/children nested under each object. Current `build_document_tree` is a flat object list. Depends on item 1 (schema needs primitives to show).

### 7. World references (background/context fixtures)

Old `WorldReferenceProps`/`CadPlayReferencesByModelDefinitionId`/`buildCadPlayReferenceInspectorChildren` rendered non-editable background reference geometry (e.g. concrete-forest fixtures) per model definition, with hover/select/inspect but no ownership by the editable document. Current plugin loads the forest fixture as regular editable `CadObject`s only (`forest_play_document`) — there's no separate reference-overlay concept. Decide whether to reintroduce references as a distinct overlay in the wgpu world3d scene, or intentionally fold fixtures into editable objects (needs a decision before implementing — see Open Questions).

### 8. Model transformations / transfer toolbar

Old toolbar (`buildCadPlayToolbarTools`) had a "transfer" tool group driven by `transfersTo`/`transfersFrom` (`TransformationSpec[]`) and an `applyTransformation` command, so the Shape pane could be transformed into Building/Energy/Structure-Classic via real model-definition transformations (`spatial.shape_to_aec.*`), keeping a linked `ModelSpace`. Current plugin's 4 panes are populated from 4 independent, statically-seeded object arrays (`CAD_MODEL_INDEX_*`) with no transformation/linking mechanism at all. This is the largest missing piece and also depends on item 1 (schema needs multi-model-definition awareness).

### 9. Save / Load toolbar actions

Old toolbar had `saveSelected`, `saveInPlay` (model space), `saveCurrent`, `loadRawRequest` (File System Access API). Current plugin only exposes OBJ/GLB export via `register_os_media_export_handler` (`cad/plugin/rs/lib.rs:1362-1381`) — no JSON save-selected/save-model-space/save-current, and no load/import path at all.

### 10. Interaction-driven engagement (command REPL)

Old had a full declarative Interaction state machine per typology (prompts/picks/previews/possible-engagements, plus a text command input REPL — `PLAY_REPL_SPEC`) surfaced via `cadPlayEngagementMirror`/`cadPlayResolvePaneEngagement`. Current `cad_window_engagement` (`cad/plugin/rs/lib.rs:925-966`) only exposes 3 fixed toggle buttons (move/rotate/scale) + a status line — `input`, `control`, `possible_engagements` are always `None`, even though `WindowEngagement` supports them. This is the biggest domain-model gap relative to [`cad/AGENTS.md`](cad/AGENTS.md)'s Interaction concept, and depends on item 1.

## Open questions before implementation

- Item 7 (world references): fold fixtures into editable objects, or rebuild the reference-overlay concept in wgpu's world3d scene API?
- Item 8 (model transformations): does `framework/renderer/wgpu` / `cad_document` need new transformation-execution support, or should this reuse an existing transformation runtime from elsewhere in the repo?
- Given item 10 is the largest, should the Interaction/REPL engagement be scoped to this ticket or split into its own follow-up ticket once the schema foundation (item 1) lands?

## Process notes (per repo rules)

- Open a ticket under goal `🎯r2602🎯runningsketchpad` (existing CAD tickets like `CAD-PLAY-PER-MODEL-INTERACTIONS`, `CAD-TRANSFORM-TOOL-GUMBALL` use this goal) before starting implementation.
- Work directly in `cad/rs/lib.rs`, `cad/plugin/rs/lib.rs`, and their existing test files (extend, don't create new test files, per `cad/AGENTS.md`/repo rules).
- Given the size, recommend tackling in dependency order: (1) schema foundation → (2/3/4) undoable rich edits → (5/6) hierarchy chrome → (9) save/load → (7/8/10) larger domain features, likely as sequential follow-up tickets rather than one pass.
