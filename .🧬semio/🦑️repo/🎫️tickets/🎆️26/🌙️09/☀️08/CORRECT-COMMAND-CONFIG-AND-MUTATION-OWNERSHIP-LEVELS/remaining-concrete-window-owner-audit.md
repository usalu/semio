# Remaining Concrete Window Ownership Audit

This read-only audit covers the remaining Layout, Sequence, Remodel, GIS Map, Shooting, Forms, CAD, FEM, Architect, and Home config ownership. It follows the ticket checklist. Terrain camera and Generation3D preview are excluded because their source migration is already underway; the completed Flow/CAD focus-source work is not reopened.

Every field below is currently persisted via an editor app-config mutation. The replacement must be keyed by the dispatcher's **concrete window identity**. A pane, surface type, or fixed kind identifier such as sequence-main is insufficient when two windows of that kind exist.

WindowConfig means persisted local state keyed to one concrete window. WindowTransient means ephemeral state keyed to one concrete window. Long-lived command output belongs to a retained operation projection. Paths are repository-relative; line references identify the observed source.

## P0: Cross-Window Contamination Or Document-Content Effects

### CAD

The completed CAD focus migration must stay complete: selected-object, hover, and active-object interaction are already delegated outside config at ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:85-95. The remaining config schema is at 🎚️config/🧬️schema/🦀️.rs:40-85.

| Fields persisted in app config | Correct owner | Evidence |
|---|---|---|
| **camera**, **camera_building**, **camera_energy**, **camera_structure_classic** | WindowConfig for the exact Shape, Building, Energy, or Structure Classic world window | 🎮️commands/🎥️camera/🦀️.rs:27-34 receives a surface but writes pane-selected config; 🎭️modes/✏️edit/🦀️.rs:232-241 renders with that config camera. |
| **sun**, **dislocate_shape**, **dislocate_building**, **dislocate_energy**, **dislocate_structure_classic** | WindowConfig for the corresponding exact world window | 🎮️commands/🧰️utility/🦀️.rs:24-34 maps pane to a fixed window kind before it writes config. Shape declares its world surface at 🎭️modes/✏️edit/🪟️windows/🟨️shape/🦀️.rs:13-16. |
| **selected_node_ids**, **hovered_reference_id**, **engagement_input**, **engagement_step**, **engagement_pane**, **engagement_session_json**, **active_example_id**, **selected_reference_model_definition_id**, **selected_reference_id**, **last_finalized_interaction_id** | WindowTransient for the exact tree or engagement window; resumable engagement data is a retained operation projection | 🎮️commands/🤝️engagement/🦀️.rs:46-50 and :66-75 mutate input, pane, session, and step through config; 🎭️modes/✏️edit/🦀️.rs:250-303 renders the live engagement. |
| **engagement_preview_operation_json**, **engagement_preview_generation** | Retained operation projection | These identify/progress generated preview work. A window displays an operation; it does not persist the operation as a preference. |
| **contributions_json** | Host/context projection | This is supplied editor context, not state owned by a concrete window. |

CAD presence declarations for camera and engagement at 👥️presence/🧬️schema/🦀️.rs:7-19 are dormant: no non-schema CAD command producer emits them.

### Layout And Sequence

| Fields persisted in app config | Correct owner | Evidence |
|---|---|---|
| Layout **active_page_id**, **camera**, **preview_camera** | WindowConfig for the exact Blueprint or Preview window | Blueprint consumes camera at 🎭️modes/✏️edit/🪟️windows/📐️blueprint/🦀️.rs:39-45; Preview consumes preview camera at 🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs:39-45. 🎮️commands/📷️set-camera/🦀️.rs:47-54 branches by surface type and emits shared config. |
| Layout **drop_preview**, **engagement_input** | WindowTransient for the exact Blueprint/input window | 🎮️commands/🛬️canvas-drag-over/🦀️.rs:60-66 writes the drag ghost; 🎮️commands/🛫️canvas-drag-leave/🦀️.rs:15-16 clears it. |
| Sequence **camera**, **orientation** | WindowConfig for the exact main graph window; orientation is captured from that window when reorganize runs | Main graph uses camera at 🎭️modes/✏️edit/🪟️windows/📽️main/🦀️.rs:71-84. 🎮️commands/🔄️layout/🦀️.rs:32-37 reads app config before a document reorganization; :52-54 persists the orientation. |
| Sequence **last_run_json** | WindowTransient for the exact Script/results window, or a retained operation result for resumable runs | 🎮️commands/🏃️run/🦀️.rs:22-25 serializes the output into config and :38-40 clears it; 🎭️modes/✏️edit/🪟️windows/📜️script/🦀️.rs:36-43 renders it. |

The all-config schemas are Layout 🎚️config/🧬️schema/🦀️.rs:8-18 and Sequence 🎚️config/🧬️schema/🦀️.rs:8-15. Their presence copies, respectively 👥️presence/🧬️schema/🦀️.rs:8-16 and :8-12, are dormant in the audited editor command sources.

## P1: View, Draft, And Result Configs

| Editor fields persisted in app config | Correct owner | Evidence |
|---|---|---|
| Remodel **camera**, **layers**, **frame_cursor**, **report_table** | WindowConfig for the exact Model 3D, Frames, and Report window respectively | Commands write config: 📷️set-camera/🦀️.rs:16-17, 👓️set-layer-visibility/🦀️.rs:16-17, ⏱️set-frame-cursor/🦀️.rs:17-18, 📊️set-report-table/🦀️.rs:15-16. Frames consumes its cursor at 🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️.rs:95-97; Report consumes its table at 🎭️modes/🔍️analyze/🪟️windows/📊️report/🦀️.rs:121. |
| GIS Map **layer_visibility**, **camera_json**, **render_mode**, **vector_style**, **lod_mode**, **layer_stroke_scale** | WindowConfig for the exact GIS Map window | 🎭️modes/✏️edit/🪟️windows/🗺️map/🦀️.rs:80-94 consumes all six, while :42-45 exposes their measures. The view command module still writes config. |
| Shooting **camera**, **center_model**, **default_shot_format**, **default_shot_shape**, **default_asset_format** | WindowConfig for the exact Scene/Shooting window. An export command captures camera into its request when it starts. | Scene consumes label, center/refit data, and camera at 🎭️modes/✏️edit/🪟️windows/🎥️scene/🦀️.rs:73-74 and :210-221. 🎮️commands/🖨️export/🦀️.rs:36 reads the config camera. |
| Shooting **selected_shot_ids**, **camera_draft_label**, **fit_revision** | WindowTransient for the exact gallery/inspection or Scene window | 🎮️commands/🗂️selection/🦀️.rs:33 persists selection and :78-83 bumps center/refit. A revision counter is an event signal, not a persisted view preference. |
| Forms **current_step_index** | WindowConfig for the exact Try/wizard window | Try renders its current step at 🎭️modes/📝️blueprint/🪟️windows/▶️try/🦀️.rs:188-190; 🎮️commands/▶️next-step/🦀️.rs:17-23 persists navigation. |
| Forms **try_values** | WindowTransient for the exact Try/wizard window | The same Try render consumes staged values. The staged-value command leaves presently write config. **contributions_json** is host form context and must not become a window field. |
| FEM 2D and FEM 3D, each **result_source_id**, **result_mode**, **result_mode_index**, **camera** | WindowConfig for that artifact's exact Results window | The schemas are 2D 🎚️config/🧬️schema/🦀️.rs:9-18 and 3D 🎚️config/🧬️schema/🦀️.rs:9-18. 3D Results creates display from config at 🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs:34-39 and reads camera at :93. |

Remodel and Shooting have duplicate presence fields, and GIS Map has a duplicate camera presence field. Their schemas are respectively 👥️presence/🧬️schema/🦀️.rs:10-22, :8-12, and :12-14; no non-schema command producer was found in the audited editor roots. FEM presence schemas are empty.

## P2: Narrow Architect/Home Window Migrations And Non-Window Records

| Editor fields persisted in app config | Correct owner | Evidence |
|---|---|---|
| Architect **active_register**, **adjacency_kind_filter**, **graph_camera_x**, **graph_camera_y**, **graph_camera_zoom** | WindowConfig for the exact Register, Adjacency, and Graph window | Register reads active register at 🎭️modes/✏️edit/🪟️windows/📋️register/🦀️.rs:62; Adjacency reads its filter at 🪟️windows/↔️adjacency/🦀️.rs:69; Graph reads camera at 🪟️windows/🕸️graph/🦀️.rs:97. |
| Architect **search_query**, **active_report_json** | WindowTransient for the exact Search/Report result window, or retained operation result when resumable | 🎮️commands/🔍️search/🦀️.rs:22-30 persists query/history/result. 🎮️commands/🔬️analysis/🦀️.rs:75-76 persists report output. |
| Architect **last_result_json**, **last_analysis_json**, **search_history_json** | Result/history projection, pending an explicit lifecycle; not a concrete WindowConfig | 🎚️config/🦀️.rs:32-38 calls last analysis write-only. Do not create a persisted window field without a render/lifecycle owner. |
| Home **active_panel_tab** | WindowConfig for the exact Home main window | 🎮️commands/⚙️set-active-panel-tab/🦀️.rs:16 writes app config. |
| Home **directory_json**, **session_hash**, **auth_generation**, **receipt_json** | OS/session directory read model | 🎮️commands/📬️apply-directory-event-page/🦀️.rs:28-31 writes config; 🎚️config/🦀️.rs:161-190 folds directory events. These are not window state. |
| Home **client_id**, **client_name** | OS client identity | 🎮️commands/🪪️set-client/🦀️.rs:14-21 persists the identity; 🎮️commands/🏗️create-studio/🦀️.rs:39-40 consumes it. |

Architect's all-config schema is 🎚️config/🧬️schema/🦀️.rs:10-31 and its presence copy at 👥️presence/🧬️schema/🦀️.rs:10-20 is dormant. Home's presence schema is empty at 👥️presence/🧬️schema/🦀️.rs:4-7.

## Boundaries To Preserve

- No audited field is a document-state migration candidate. Shooting's explicit saved-camera action should capture the caller window camera and create its selected document record; the working camera remains window-owned.
- Locale does not occur in these candidate configs. It remains OS-wide.
- A presence declaration is dormant shared capability, not a persistence owner. Add emitted presence only for deliberate collaboration behavior.
- Implement in P0, P1, then P2 order. The language-agnostic ownership test must open two concrete instances and prove a mutation in one leaves the other unchanged.
