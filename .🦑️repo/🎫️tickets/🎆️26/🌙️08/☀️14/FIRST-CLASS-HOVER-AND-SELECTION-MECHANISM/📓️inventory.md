# Per-App Hover/Selection Migration Inventory

Verified read-only scan. **17 plugin crates, ~24 apps.** Every listed app has a `👥️presence` facet that mirrors its config selection/hover — all of those collapse into the framework's typed `PresenceInteraction` field.

Paths are relative to `✏️s/🔌️plugins/` unless stated. Every app has `🎚️config/🦀️component.rs`, `🎚️config/🧬️schema/🦀️component.rs`, `👥️presence/🦀️component.rs` (+ its `🧬️schema/`).

| Crate | Plugin | Apps | Domain / granularities | Hierarchy for transitive |
|---|---|---|---|---|
| semio-s-plugin-writer | ✒️writer | ✒️writer | `ast` (+ editor text) | AST parent links |
| semio-s-plugin-procedural | 🌀️procedural | 🧊️3d, ◻2d | `graph` node/edge/handle | DAG parent links |
| semio-s-plugin-gis | 🌍️gis | 🧊️3d, ◻2d | `features` layer/feature | Flat |
| semio-s-plugin-shooting | 🎥️shooting | 🎥️shooting | `assets` asset | Flat |
| semio-s-plugin-process | 🏭️process | 🧊️3d | `geometry` object/face | Flat |
| semio-s-plugin-lowpoly | 💠️lowpoly | 💠️lowpoly | `mesh` object/vertex/edge/face | Flat |
| semio-s-plugin-layout | 📏️layout | 📏️layout | `elements` element | Flat |
| semio-s-plugin-cad | 📐️cad | 📐️cad | `cad` object/vertex/edge/face | Flat |
| semio-s-plugin-remodel | 📸️remodel | 📸️remodel | `assets` | Flat |
| semio-s-plugin-trinity | 🔱️trinity | 🔌️jack, ♻️rewrite | `ast`, `graph` | AST parents + var refs |
| semio-s-plugin-draw | 🖍️draw | 🖍️draw | `strokes` | Flat |
| semio-s-plugin-raster | 🖨️raster | 🖨️raster | `layers` | Flat |
| semio-s-plugin-note | 🗒️note | 🗒️note | `blocks` block | Document nesting |
| semio-s-plugin-puzzle | 🧩️puzzle | 🧊️3d, 🖐️5d, ◻2d | `vortex` object/part/kind | vortex→part→kind |
| semio-s-plugin-block | 🧱️block | 🧊️3d, 🖐️5d, ◻2d | `vortex` | vortex parents |
| semio-s-plugin-space | 🪐️space | 🪐️space | `graph` instance/media-node | node graph parents |
| semio-s-plugin-sourcing | 🪵️sourcing | 🗂️curate | `rows` object | Flat |
| (os kernel) | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite` | — | `world` surface/item | PathDelimited `surfaceId/id` |

## Heaviest migrations

**🧩️puzzle** (3 apps, ~20 selection/hover commands): `🎥️focus-selection`, `👆️set-hover`, `👆️set-kind-hover`, `👆️world-hover`, `👆️world-vortex-hover`, `🔄️{rotate,scale,translate}-selection`, `🖌️engagement-control-select`, `🖌️hover-suggestion`, `🗂️{clear-selection,select-all,select-same-kind,set-selectable-kind,set-selection,set-selection-method,set-selection-mode-default,world-select,world-vortex-select}`, `🧊️{delete-selection,duplicate-selection,set-selection-flag}`. Config: `selection: Puzzle3dSelection`, `selection_method`, `selection_mode_default`, `hovered_object_id`, `hovered_vortex_full_id`, `hovered_kind_id`; 5d adds `hovered_part_id`.

**💠️lowpoly** (richest single-app model — acceptance bar): `selection_mode`, `selection_ids: Vec<u32>`, `selection_targets_{mesh,vertex,edge,face}: bool`, `selection_keys`, `selection_method`, `selection_mode_default`, `selected_object_ids`, `hovered_object_id`, `hovered_target_{object_id,mode,id}` (flattened `LowpolyHoverTarget`). Mutations `SetSelection`, `SetSelectionTargets`, `SetSelectionKeys`, `SetSelectionMethod`, `SetSelectionModeDefault`, `SetSelectedObjectIds`, `SetHoveredObject`, `SetHoveredTarget`.

**✒️writer**: `selected_ast_ids`, `editor_selection: Option<WriterEditorSelection>`, `tree_hovered_ast_id`, `editor_hover_offset`. Commands `🗂️{select-ast-node,set-ast-hover,set-ast-selection,set-editor-selection,text-hover,text-select}`. Readers: `🎭️modes/✏️edit/🪟️windows/✒️main/🦀️component.rs`, `📌️panels/📄️artifact/🦀️component.rs`, app `🦀️component.rs`.

**📐️cad**: nested `CadHoverTarget{object_id,mode,id}` + `CadComponentSelection{targets mesh/vertex/edge/face bools, mode, ids: Vec<u32>}`.

**🌍️gis ◻2d**: JSON-string state — `feature_selection_json`, `hover_json`, `selection_method`, `selection_mode`. These become typed domain state.

## Migration notes

- **u32 ids** (lowpoly `selection_ids`, cad `CadComponentSelection.ids`, process `selected_face_id`, lowpoly `hovered_target_id`) stringify at the `InteractionTarget` boundary; each app's `interaction_topology` owns the round-trip.
- **Retained app verbs** (must keep working by reading `InteractionView`, not deleted): `delete-selection`, `duplicate-selection`, `focus-selection`, `zoom-to-selection`, `{rotate,scale,translate}-selection`, `nudge-selection*` (note: 9 variants), `select-same-kind`, `set-selection-flag`, `set-selected-opacity`, `🧬️select-generation`.
- **Editor caret/range** (writer `editor_selection`, trinity jack `editor_selection`) is editor-intrinsic and stays app-side, but moves onto the Interaction history lane so it is not undoable.
- No serialized selection/hover state was found in example/fixture DSL files in scan scope — confirm again in W5 rather than assuming.
