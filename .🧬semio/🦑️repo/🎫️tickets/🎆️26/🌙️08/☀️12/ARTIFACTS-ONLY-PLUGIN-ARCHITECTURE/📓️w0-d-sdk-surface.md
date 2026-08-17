# W0-D — SDK/API Surface & Dependency Graph Census

Scope: violation class D per the APA assignment. Read-only census, zero source edits made. Counts
below are live grep results captured 2026-08-12. Two other sessions
(UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM, SEMANTIC-MUTATIONS-OVERHAUL) are editing this tree
concurrently, so every finding below carries a grep-able anchor string alongside its line number so
a later agent can re-find it even if lines have moved.

---

## 1. Tail re-export block — `🔌️plugin/🦀️component.rs`

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (9731 lines total).

Full tail re-export block, quoted verbatim, lines 9630–9654:

```rust
9630  pub use app::testkit;
9631  pub use app::ActionFactory;
9632  pub use app::{
9633      node_graph_delete_selection_spec, selection_count_phrase, selection_domains_from_surface, ActionMeta, App, AppActionRegistry, AppBuilder, AppInstance, ArtifactBuilder, ArtifactDecomposer, ArtifactAnalyzer, ArtifactComposer, ArtifactAnalysis, ArtifactComposition, ArtifactInferrer, ArtifactSerializer, ArtifactDeserializer, DerivedArtifactSpec, DerivedArtifactParts, DerivedArtifactBuilder, DerivedArtifactAnalyzer, DerivedArtifactComposer, composer_entry_of, deserializer_entry_of, serializer_entry_of, ArtifactKindSpec, Confidence, Decomposition, DecomposeSource, ConfigView, ArtifactApp, ArtifactView, DraftView, Emit, ExampleSource, HistoryView,
9634      KeybindingSpec, MediaClass, MediaType, Menu, ModeSpec, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NodeGraphDeleteDispatch, OsMediaCapability, PanelTabSpec, PanelTreeBuilder, Plugin, PluginApp, PluginBuilder, PluginProgram, VcsArtifactApp,
9635      WindowKindSpec,
9636  };
9637  pub use app::{locale_from_str, resolve_labels, resolve_labels_for_locale, selection_ids, tree_item, tree_item_desc, tree_item_with_action, tree_item_with_action_draggable, LabelAxes};
9638  pub use engagement::{engagement_token_matches, strip_engagement_prefix};
9639  pub use host_port::{host_backbone_poll, host_backbone_send, host_backbone_status, host_now_ms, host_read_asset, register_host_backbone_channel, HostBackboneChannel};
9640  pub use plugin_runtime::{
9641      extension_activate, extension_deactivate, extension_invoke, extension_manifest, install_extension_bundle, install_plugin_bundle,
9642      plugin_attach_backbone, plugin_detach_backbone, plugin_document_pack, plugin_ingest_operations, plugin_load_document_pack, ExtensionBundle,
9643      ExtensionManifest,
9644  };
9645  pub use semio_framework::*;
9646  pub use semio_framework::{MediaForm, MediaPortDirection, MediaPortSpec};
9647  pub use world3d_host::{
9648      apply_world3d_projection_action, apply_world3d_sun_action, default_world3d_selection, export_mesh_glb_bytes, export_mesh_obj, merge_world_selection_ids, mesh_kind_from_json, world3d_camera_projection_json, world3d_default_camera,
9649      world3d_environment_json, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds, world3d_meshes_json_from_kinds_and_urls, world3d_meshes_json_from_urls, world3d_projection_action_moves_pose, world3d_projection_measures,
9650      world3d_projection_pose, world3d_projection_spec_json, world3d_scene, world3d_scene_extended, world3d_selection_json, world3d_sun_measures, SelectionSet, WorldProjectionConfig, WorldSunConfig,
9651  };
9652  // 🧩️ Declarative component model (UiNode, layouts, utilities) — moved into ui_wgpu; re-exported here so
9653  // apps keep the flat `semio_framework_plugin::*` import surface with zero Cargo.toml churn.
9654  pub use ui_wgpu::wgpu::*;
```

Anchors: `pub use app::testkit;` (block start) · `pub use semio_framework::*;` (glob #1) ·
`pub use ui_wgpu::wgpu::*;` (glob #2 / block end).

**Glob re-exports: exactly 2, both flagged for the curated-list replacement:**

| line | statement | source crate |
|---:|---|---|
| 9645 | `pub use semio_framework::*;` | `semio_framework` (the framework-core crate) |
| 9654 | `pub use ui_wgpu::wgpu::*;` | `ui_wgpu::wgpu` (declarative UiNode/layout model) |

No other glob (`::*`) re-exports exist in this tail block. Everything else in lines 9630–9651 is
already an explicit, named `pub use` list (from `app`, `engagement`, `host_port`, `plugin_runtime`,
`world3d_host`) — those are in scope for pruning to "actually used" but are not glob violations.

---

## 2. What plugins actually pull through `semio_framework_plugin::*`

Method: two greps over all of `✏️s/🔌️plugins/**/*.rs` (33 top-level plugin crates + their
`🧩️extensions/` sub-crates):

1. Bare-path uses — regex `(?<!:)\bsemio_framework_plugin::\w+` (catches inline
   `semio_framework_plugin::Foo(...)` calls and single-symbol `use semio_framework_plugin::Foo;`).
2. Grouped-use uses — regex `use\s+semio_framework_plugin::\{([^}]*)\};` (multi-line aware),
   symbols split on top-level commas, `as` aliases resolved to the original name.

Raw occurrence count (every match of the path, not deduped):
`grep -rn 'semio_framework_plugin::' ✏️s/🔌️plugins/ --include='*.rs' | wc -l` → **4050**.

After parsing every `use` statement and deduping by symbol: **302 distinct symbols** imported from
`semio_framework_plugin` across the plugin tree. This is the raw material for the curated explicit
re-export list. Full table below — `plugins` = distinct top-level plugin directories using the
symbol (max 33), `files` = distinct `.rs` files across the whole tree. Sorted by `plugins` desc, then
`files` desc, then name.

| plugins | files | symbol | plugin dirs (if ≤6, else count only) |
|---:|---:|---|---|
| 33 | 476 | `Dialect` | 33 plugins |
| 33 | 387 | `StandardId` | 33 plugins |
| 33 | 387 | `SubsetId` | 33 plugins |
| 33 | 277 | `AnalyzeSource` | 33 plugins |
| 33 | 266 | `ComposeError` | 33 plugins |
| 33 | 231 | `ArtifactBuilder` | 33 plugins |
| 33 | 216 | `IoConfidence` | 33 plugins |
| 33 | 197 | `ComposerEntry` | 33 plugins |
| 33 | 137 | `Analysis` | 33 plugins |
| 33 | 137 | `ArtifactAnalysis` | 33 plugins |
| 33 | 137 | `ArtifactComposition` | 33 plugins |
| 33 | 137 | `ComposeSource` | 33 plugins |
| 33 | 137 | `Composition` | 33 plugins |
| 33 | 137 | `derive_artifact_facets` | 33 plugins |
| 33 | 130 | `ErasedComposeSource` | 33 plugins |
| 33 | 129 | `ComposedArtifact` | 33 plugins |
| 33 | 128 | `MediaClass` | 33 plugins |
| 33 | 128 | `MediaForm` | 33 plugins |
| 33 | 128 | `MediaType` | 33 plugins |
| 33 | 109 | `ArtifactAnalyzer` | 33 plugins |
| 33 | 104 | `register_composer_entries` | 33 plugins |
| 33 | 101 | `ArtifactKindSpec` | 33 plugins |
| 33 | 94 | `composer_entry_of` | 33 plugins |
| 33 | 88 | `OsMediaCapability` | 33 plugins |
| 33 | 72 | `ArtifactInferrer` | 33 plugins |
| 33 | 41 | `Plugin` | 33 plugins |
| 32 | 526 | `LocalizedLabel` | 32 plugins |
| 32 | 174 | `ExampleSource` | 32 plugins |
| 32 | 92 | `IoPayload` | 32 plugins |
| 31 | 32 | `plugin_exports` | 31 plugins |
| 30 | 355 | `Emit` | 30 plugins |
| 30 | 354 | `Fault` | 30 plugins |
| 30 | 351 | `ArtifactView` | 30 plugins |
| 30 | 349 | `ConfigView` | 30 plugins |
| 30 | 320 | `UiNode` | 30 plugins |
| 30 | 147 | `Label` | 30 plugins |
| 30 | 99 | `testkit` | 30 plugins |
| 30 | 97 | `ui_text` | 30 plugins |
| 30 | 88 | `PluginApp` | 30 plugins |
| 30 | 65 | `ViewModel` | 30 plugins |
| 30 | 59 | `VcsArtifactApp` | 30 plugins |
| 30 | 57 | `App` | 30 plugins |
| 30 | 55 | `ArtifactApp` | 30 plugins |
| 30 | 55 | `DraftView` | 30 plugins |
| 30 | 55 | `NoDraft` | 30 plugins |
| 30 | 54 | `NoDraftMutation` | 30 plugins |
| 30 | 42 | `plugin_runtime` | 30 plugins |
| 29 | 83 | `SurfaceKind` | 29 plugins |
| 29 | 50 | `WindowLayout` | 29 plugins |
| 28 | 62 | `app_commands` | 28 plugins |
| 28 | 58 | `ModeDefinition` | 28 plugins |
| 27 | 106 | `WindowKindDefinition` | 27 plugins |
| 27 | 74 | `WindowOptions` | 27 plugins |
| 27 | 52 | `ActionDescriptor` | 27 plugins |
| 26 | 140 | `PanelGroup` | 26 plugins |
| 26 | 55 | `UiPresence` | 26 plugins |
| 26 | 53 | `create_default_layout` | 26 plugins |
| 26 | 47 | `InvocationResult` | 26 plugins |
| 26 | 34 | `app_labels` | 26 plugins |
| 25 | 138 | `PanelTabDefinition` | 25 plugins |
| 25 | 93 | `PanelTabKind` | 25 plugins |
| 25 | 47 | `FRAMEWORK_PANEL_TAB_INSPECTION_LABEL` | 25 plugins |
| 25 | 46 | `FRAMEWORK_PANEL_TAB_ARTIFACT_ID` | 25 plugins |
| 25 | 45 | `FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL` | 25 plugins |
| 25 | 34 | `ActionArgOption` | 25 plugins |
| 25 | 32 | `ActionArgDef` | 25 plugins |
| 24 | 69 | `AppIo` | 24 plugins |
| 24 | 60 | `UiTreeItemNode` | 24 plugins |
| 24 | 45 | `FRAMEWORK_PANEL_TAB_INSPECTION_ID` | 24 plugins |
| 24 | 30 | `ActionFactory` | 24 plugins |
| 23 | 49 | `Media` | 23 plugins |
| 23 | 48 | `MediaPayload` | 23 plugins |
| 23 | 47 | `MediaError` | 23 plugins |
| 23 | 41 | `FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL` | 23 plugins |
| 22 | 41 | `resolve_labels_for_locale` | 22 plugins |
| 22 | 30 | `ActionKind` | 22 plugins |
| 21 | 80 | `HistoryView` | 21 plugins |
| 21 | 45 | `PanelTreeBuilder` | 21 plugins |
| 21 | 41 | `FRAMEWORK_PANEL_TAB_CATALOGUE_ID` | 21 plugins |
| 21 | 28 | `UiInspectorFieldGroup` | 21 plugins |
| 21 | 28 | `ui_inspector_groups_to_tree` | 21 plugins |
| 21 | 27 | `ui_inspector_readonly_field` | 21 plugins |
| 20 | 29 | `UiFieldNode` | 20 plugins |
| 20 | 28 | `UiInputNode` | 20 plugins |
| 20 | 24 | `ActionDefinition` | 20 plugins |
| 19 | 43 | `MediaPortDirection` | 19 plugins |
| 19 | 37 | `tree_item_with_action` | 19 plugins |
| 19 | 27 | `ArtifactPresentation` | 19 plugins |
| 19 | 27 | `MediaPortSpec` | 19 plugins |
| 16 | 24 | `UiSectionNode` | 16 plugins |
| 16 | 24 | `ui_declarative_sections_to_tree` | 16 plugins |
| 15 | 117 | `WindowMeasure` | 15 plugins |
| 15 | 26 | `ui_stack_vertical` | 15 plugins |
| 15 | 23 | `Terminology` | 15 plugins |
| 15 | 20 | `Locale` | 15 plugins |
| 12 | 34 | `HostEffect` | 12 plugins |
| 12 | 16 | `build_world_3d_scene` | 12 plugins |
| 11 | 21 | `UtilityDefinition` | 11 plugins |
| 11 | 14 | `ui_inspector_mixed_text` | 11 plugins |
| 11 | 13 | `world3d_selection_json` | 11 plugins |
| 11 | 12 | `UiToggleNode` | 11 plugins |
| 10 | 31 | `WindowEngagement` | 10 plugins |
| 10 | 16 | `WorldSunConfig` | 10 plugins |
| 10 | 15 | `WindowEngagementStatus` | 10 plugins |
| 10 | 14 | `AppActionRegistry` | 10 plugins |
| 10 | 14 | `ContextMenuItemSpec` | 10 plugins |
| 10 | 14 | `ContextMenuRequest` | 10 plugins |
| 10 | 14 | `Menu` | 10 plugins |
| 10 | 14 | `WindowEngagementInput` | 10 plugins |
| 10 | 10 | `ui_inspector_mixed_number` | 10 plugins |
| 9 | 13 | `UiMenuRef` | 9 plugins |
| 9 | 12 | `Canvas2dScene` | 9 plugins |
| 9 | 12 | `UiSelectItem` | 9 plugins |
| 9 | 12 | `UiSelectNode` | 9 plugins |
| 8 | 22 | `MeasureSelectItem` | 8 plugins |
| 8 | 14 | `PortMultiplicity` | 8 plugins |
| 8 | 11 | `AppLabels` | 8 plugins |
| 8 | 11 | `MeshData` | 8 plugins |
| 8 | 11 | `NodeGraphScene` | 8 plugins |
| 8 | 11 | `NodeGraphViewport` | 8 plugins |
| 8 | 11 | `build_canvas_2d_scene` | 8 plugins |
| 8 | 11 | `world3d_scene` | 8 plugins |
| 8 | 10 | `UtilityCategory` | 8 plugins |
| 7 | 10 | `IconName` | 7 plugins |
| 7 | 10 | `SET_ACTIVE_UTILITY_ACTION_ID` | 7 plugins |
| 7 | 10 | `tree_item` | 7 plugins |
| 7 | 9 | `TextEditorScene` | 7 plugins |
| 7 | 9 | `build_text_editor_scene` | 7 plugins |
| 7 | 8 | `UI_INSPECTOR_MIXED_PLACEHOLDER` | 7 plugins |
| 7 | 7 | `ContextMenuSurfaceTarget` | 7 plugins |
| 7 | 7 | `IoDirection` | 7 plugins |
| 7 | 7 | `IoKey` | 7 plugins |
| 6 | 26 | `ExtensionBundle` | 🌊️flow, 🏭️process, 📐️cad, 📖️playbook, 📜️imperative, 🪵️sourcing |
| 6 | 26 | `extension_exports` | 🌊️flow, 🏭️process, 📐️cad, 📖️playbook, 📜️imperative, 🪵️sourcing |
| 6 | 10 | `mesh_from_kind` | 🏭️process, 💠️lowpoly, 📐️cad, 📖️playbook, 📸️remodel, 🧩️puzzle |
| 6 | 9 | `GlbExporter` | 🌀️procedural, 🎪️demonstrator, 🏭️process, 💠️lowpoly, 📸️remodel, 🧩️puzzle |
| 6 | 9 | `build_node_graph_scene` | 🌀️procedural, 🌊️flow, 🎬️sequence, 🔱️trinity, 🕸️dag, 🪐️space |
| 6 | 8 | `GlbImporter` | 🌀️procedural, 🎪️demonstrator, 🏭️process, 💠️lowpoly, 📐️cad, 🧩️puzzle |
| 6 | 8 | `UiButtonNode` | 🌿️vcs, 🎞️animate, 📋️forms, 📖️playbook, 🧩️puzzle, 🪐️space |
| 6 | 8 | `optional_json_to_dsl` | 🌊️flow, 🎬️sequence, 📜️imperative, 🗒️note, 🧩️puzzle, 🪐️space |
| 6 | 8 | `tree_item_desc` | 🌊️flow, 🎬️sequence, 🏭️process, 📏️layout, 🕸️dag, 🪐️space |
| 6 | 6 | `io_dispatch` | 🌍️gis, 🎥️shooting, 📏️layout, 🖍️draw, 🗒️note, 🧩️puzzle |
| 6 | 6 | `ui_inspector_mixed_toggle` | 🌍️gis, 🏛️architect, 📋️forms, 📐️cad, 🖍️draw, 🗒️note |
| 5 | 47 | `ArtifactSerializer` | 🌍️gis, 🏗️fem, 📐️cad, 📸️remodel, 🗄️stdio |
| 5 | 20 | `SelectionSet` | 🌀️procedural, 🎥️shooting, 💠️lowpoly, 📐️cad, 🧩️puzzle |
| 5 | 14 | `LabelText` | 🏭️process, 💠️lowpoly, 📏️layout, 📸️remodel, 🧩️puzzle |
| 5 | 12 | `UiTreeSectionNode` | 🌊️flow, 🎞️animate, 🏛️architect, 🕸️dag, 🧩️puzzle |
| 5 | 7 | `ConfigSpec` | 🌍️gis, 🎥️shooting, 🎬️sequence, 🏗️fem, 🏭️process |
| 5 | 7 | `selection_domains_from_surface` | 🌀️procedural, 🎬️sequence, 🔱️trinity, 🕸️dag, 🪐️space |
| 5 | 7 | `world3d_sun_measures` | 🌀️procedural, 🏭️process, 💠️lowpoly, 📐️cad, 🧩️puzzle |
| 5 | 6 | `UiTreeActionPlacement` | 🏭️process, 💠️lowpoly, 📐️cad, 🧩️puzzle, 🪵️sourcing |
| 5 | 6 | `UiTreeItemAction` | 🏭️process, 💠️lowpoly, 📐️cad, 🧩️puzzle, 🪵️sourcing |
| 5 | 6 | `merge_world_selection_ids` | 🌀️procedural, 🎥️shooting, 💠️lowpoly, 📐️cad, 🧩️puzzle |
| 5 | 6 | `world3d_mesh_id_from_url` | 🎥️shooting, 🏭️process, 📐️cad, 🧩️puzzle, 🧱️block |
| 5 | 5 | `WindowEngagementPossible` | ✒️writer, 🎥️shooting, 💠️lowpoly, 📏️layout, 📐️cad |
| 4 | 13 | `WindowEngagementSlot` | 💠️lowpoly, 📸️remodel, 🧩️puzzle, 🪐️space |
| 4 | 9 | `UiTreeNode` | 🎞️animate, 🏛️architect, 🕸️dag, 🧩️puzzle |
| 4 | 7 | `ActionMeta` | 📐️cad, 📖️playbook, 🔱️trinity, 🧩️puzzle |
| 4 | 6 | `NamedLayout` | 🌀️procedural, 🌊️flow, 💠️lowpoly, 📸️remodel |
| 4 | 6 | `NodeGraphDeleteDispatch` | 🌀️procedural, 🎬️sequence, 🔱️trinity, 🕸️dag |
| 4 | 6 | `ObjExporter` | 🌀️procedural, 🎪️demonstrator, 💠️lowpoly, 🧩️puzzle |
| 4 | 6 | `StlExporter` | 🌀️procedural, 🎪️demonstrator, 💠️lowpoly, 🧩️puzzle |
| 4 | 6 | `WindowLayoutRoot` | 📐️cad, 🔱️trinity, 🧩️puzzle, 🪵️sourcing |
| 4 | 6 | `create_named_layout` | 🌀️procedural, 🌊️flow, 💠️lowpoly, 📸️remodel |
| 4 | 6 | `node_graph_delete_selection_spec` | 🌀️procedural, 🎬️sequence, 🔱️trinity, 🕸️dag |
| 4 | 5 | `ObjImporter` | 🌀️procedural, 🎪️demonstrator, 💠️lowpoly, 🧩️puzzle |
| 4 | 5 | `StlImporter` | 🌀️procedural, 🎪️demonstrator, 💠️lowpoly, 🧩️puzzle |
| 4 | 5 | `TableScene` | 📜️imperative, 📸️remodel, 🔱️trinity, 🪵️sourcing |
| 4 | 5 | `UiNumberStepperNode` | 🏛️architect, 📋️forms, 🪐️space, 🪵️sourcing |
| 4 | 5 | `WindowLayoutAxisNode` | 📐️cad, 🔱️trinity, 🧩️puzzle, 🪵️sourcing |
| 4 | 5 | `WindowLayoutChild` | 📐️cad, 🔱️trinity, 🧩️puzzle, 🪵️sourcing |
| 4 | 5 | `WindowLayoutStackNode` | 📐️cad, 🔱️trinity, 🧩️puzzle, 🪵️sourcing |
| 4 | 5 | `apply_world3d_sun_action` | 🌀️procedural, 💠️lowpoly, 📐️cad, 🧩️puzzle |
| 4 | 5 | `build_table_scene` | 📜️imperative, 📸️remodel, 🔱️trinity, 🪵️sourcing |
| 4 | 5 | `world3d_scene_extended` | 🌍️gis, 📐️cad, 🧩️puzzle, 🧱️block |
| 4 | 4 | `ContextMenuHit` | 🌊️flow, 🔱️trinity, 🕸️dag, 🪐️space |
| 4 | 4 | `ContextMenuSelectionGroup` | 🌊️flow, 🔱️trinity, 🕸️dag, 🧩️puzzle |
| 4 | 4 | `NodeGraphEdgeRecord` | 🎬️sequence, 🏛️architect, 🔱️trinity, 🪐️space |
| 4 | 4 | `NodeGraphNodeRecord` | 🎬️sequence, 🏛️architect, 🔱️trinity, 🪐️space |
| 4 | 4 | `UiSliderNode` | 🌍️gis, 📋️forms, 📖️playbook, 🖍️draw |
| 4 | 4 | `resolve_window_actions` | 💠️lowpoly, 📏️layout, 🧩️puzzle, 🪐️space |
| 3 | 9 | `kernel` | 🌍️gis, 🖍️draw, 🧩️puzzle |
| 3 | 6 | `locale_from_str` | 🌀️procedural, 🌊️flow, 🪐️space |
| 3 | 5 | `FaultOrigin` | 📸️remodel, 🧱️block, 🪐️space |
| 3 | 5 | `UtilityRef` | 💠️lowpoly, 📐️cad, 📸️remodel |
| 3 | 4 | `FaultCode` | 📸️remodel, 🧱️block, 🪐️space |
| 3 | 4 | `WindowLayoutWindowNode` | 📐️cad, 🔱️trinity, 🪵️sourcing |
| 3 | 4 | `WorldProjectionConfig` | 📐️cad, 🧩️puzzle, 🧱️block |
| 3 | 4 | `selection_count_phrase` | 🌊️flow, 🧩️puzzle, 🪐️space |
| 3 | 4 | `world3d_default_camera` | 🏗️fem, 📖️playbook, 🪵️sourcing |
| 3 | 3 | `MeshExporter` | 🌀️procedural, 🏭️process, 📸️remodel |
| 3 | 3 | `NodeGraphHover` | 🌀️procedural, 🔱️trinity, 🪐️space |
| 3 | 3 | `NodeGraphPortRecord` | 🎬️sequence, 🏛️architect, 🔱️trinity |
| 3 | 3 | `UiStackNode` | 🌿️vcs, 🏛️architect, 📋️forms |
| 3 | 3 | `engagement_token_matches` | ✒️writer, 💠️lowpoly, 📏️layout |
| 3 | 3 | `tree_item_with_action_draggable` | 🌊️flow, 📏️layout, 🖍️draw |
| 3 | 3 | `ui_tree_stamp_presence` | 🏛️architect, 🕸️dag, 🧩️puzzle |
| 3 | 3 | `world3d_camera_json` | 🏭️process, 💠️lowpoly, 📸️remodel |
| 3 | 3 | `world3d_camera_projection_json` | 📐️cad, 🧩️puzzle, 🧱️block |
| 2 | 50 | `ArtifactDeserializer` | 📐️cad, 🗄️stdio |
| 2 | 5 | `ActionRef` | 💠️lowpoly, 🧩️puzzle |
| 2 | 5 | `ui_inspector_stepper_field` | 📐️cad, 🧩️puzzle |
| 2 | 3 | `DslValue` | 🎥️shooting, 🎬️sequence |
| 2 | 3 | `ui_inspector_vec3_group` | 📐️cad, 🧩️puzzle |
| 2 | 3 | `world3d_chunking_json` | 📐️cad, 🧩️puzzle |
| 2 | 3 | `world3d_environment_json` | 📐️cad, 🧩️puzzle |
| 2 | 2 | `BlockPaletteEntry` | 📋️forms, 📖️playbook |
| 2 | 2 | `ConfigFieldShape` | 🎥️shooting, 🎬️sequence |
| 2 | 2 | `ConfigFieldSpec` | 🎥️shooting, 🎬️sequence |
| 2 | 2 | `DwgDrawing` | 🌍️gis, 🎥️shooting |
| 2 | 2 | `MeshImporter` | 🌀️procedural, 🏭️process |
| 2 | 2 | `UiComponentSceneNode` | ➗️mathematical, 🏛️architect |
| 2 | 2 | `UiControlNode` | 🎬️sequence, 🪐️space |
| 2 | 2 | `UiGroupNode` | 📐️cad, 🧩️puzzle |
| 2 | 2 | `WindowEngagementOption` | ✒️writer, 💠️lowpoly |
| 2 | 2 | `apply_world3d_projection_action` | 📐️cad, 🧩️puzzle |
| 2 | 2 | `io_resolve` | 🗄️stdio, 🗒️note |
| 2 | 2 | `strip_engagement_prefix` | ✒️writer, 🧩️puzzle |
| 2 | 2 | `ui_inspector_mixed_select` | 🖍️draw, 🧩️puzzle |
| 2 | 2 | `world3d_meshes_json_from_kinds_and_urls` | 🎥️shooting, 🧩️puzzle |
| 2 | 2 | `world3d_meshes_json_from_urls` | 📐️cad, 🧩️puzzle |
| 2 | 2 | `world3d_projection_action_moves_pose` | 📐️cad, 🧩️puzzle |
| 2 | 2 | `world3d_projection_measures` | 📐️cad, 🧩️puzzle |
| 2 | 2 | `world3d_projection_pose` | 📐️cad, 🧩️puzzle |
| 1 | 44 | `SubsetValidator` | 🗄️stdio |
| 1 | 44 | `SubsetValidatorEntry` | 🗄️stdio |
| 1 | 44 | `register_subset_validator` | 🗄️stdio |
| 1 | 44 | `subset_validator_entry_of` | 🗄️stdio |
| 1 | 13 | `deserializer_entry_of` | 🗄️stdio |
| 1 | 13 | `serializer_entry_of` | 🗄️stdio |
| 1 | 4 | `IoError` | 🏗️fem |
| 1 | 4 | `ToolRef` | 🧩️puzzle |
| 1 | 4 | `is_de_locale` | 🧩️puzzle |
| 1 | 3 | `extension_activate` | 🌊️flow |
| 1 | 3 | `extension_invoke` | 🌊️flow |
| 1 | 3 | `extension_manifest` | 🌊️flow |
| 1 | 3 | `install_extension_bundle` | 🌊️flow |
| 1 | 2 | `Board2dScene` | 🧩️puzzle |
| 1 | 2 | `FRAMEWORK_HISTORY_BODY_KEY` | 🧩️puzzle |
| 1 | 2 | `SET_ACTIVE_TOOL_ACTION_ID` | 🧩️puzzle |
| 1 | 2 | `TableCell` | 🪵️sourcing |
| 1 | 2 | `ToolDefinition` | 🧩️puzzle |
| 1 | 2 | `UiInspectorMixedText` | 🧩️puzzle |
| 1 | 2 | `VirtualFileSystemScene` | 🪐️space |
| 1 | 2 | `build_board2d_scene` | 🧩️puzzle |
| 1 | 2 | `build_paint_2d_scene` | 🖨️raster |
| 1 | 2 | `build_virtual_file_system_scene` | 🪐️space |
| 1 | 2 | `table_row_json` | 🪵️sourcing |
| 1 | 2 | `world3d_default_selection_json` | 🏗️fem |
| 1 | 1 | `ActionArgControl` | 🧩️puzzle |
| 1 | 1 | `BlockListScene` | 🏛️architect |
| 1 | 1 | `ContextMenuTextContext` | ✒️writer |
| 1 | 1 | `DialogDefinition` | 🧩️puzzle |
| 1 | 1 | `DwgGeometry` | 🌍️gis |
| 1 | 1 | `FRAMEWORK_PANEL_TAB_PARAMETERS_LABEL` | 🪐️space |
| 1 | 1 | `GraphTimelineScene` | 🌿️vcs |
| 1 | 1 | `IconRenderExportItem` | 🎥️shooting |
| 1 | 1 | `IconRenderScene` | 🎥️shooting |
| 1 | 1 | `InkCanvasScene` | 🗒️note |
| 1 | 1 | `IntroductionDefinition` | 🧩️puzzle |
| 1 | 1 | `IntroductionInteraction` | 🧩️puzzle |
| 1 | 1 | `IntroductionPlacement` | 🧩️puzzle |
| 1 | 1 | `IntroductionStepDefinition` | 🧩️puzzle |
| 1 | 1 | `NoConfig` | 📖️playbook |
| 1 | 1 | `NoConfigMutation` | 📖️playbook |
| 1 | 1 | `NoPresence` | 📖️playbook |
| 1 | 1 | `NoPresenceMutation` | 📖️playbook |
| 1 | 1 | `NodeGraphFindItem` | 🪐️space |
| 1 | 1 | `NodeGraphOperatorRecord` | 🪐️space |
| 1 | 1 | `Paint2dScene` | 🖨️raster |
| 1 | 1 | `TiledMapScene` | 🌍️gis |
| 1 | 1 | `TopicContribution` | 📋️forms |
| 1 | 1 | `UiTextNode` | 📋️forms |
| 1 | 1 | `WindowEngagementControl` | 🏭️process |
| 1 | 1 | `World3dScene` | 🎥️shooting |
| 1 | 1 | `app` | 📖️playbook |
| 1 | 1 | `build_graph_timeline_scene` | 🌿️vcs |
| 1 | 1 | `build_icon_render_scene` | 🎥️shooting |
| 1 | 1 | `build_ink_canvas_scene` | 🗒️note |
| 1 | 1 | `build_tiled_map_scene` | 🌍️gis |
| 1 | 1 | `create_stack_layout` | 📜️imperative |
| 1 | 1 | `create_tab_stack_layout` | 🪐️space |
| 1 | 1 | `create_window_layout` | 🧩️puzzle |
| 1 | 1 | `host_now_ms` | 🪐️space |
| 1 | 1 | `io` | 🗄️stdio |
| 1 | 1 | `io_dialects_for` | 🗒️note |
| 1 | 1 | `mesh_from_indexed_with_face_groups` | 🏭️process |
| 1 | 1 | `panel_tab_element_id` | 🧩️puzzle |
| 1 | 1 | `panel_tab_first_draggable_element_id` | 🧩️puzzle |
| 1 | 1 | `selection_ids` | 🧩️puzzle |
| 1 | 1 | `testkit::assert_undo_redo_round_trip` | 🎬️sequence |
| 1 | 1 | `text_identifier_bounds_at` | 🔱️trinity |
| 1 | 1 | `text_identifier_occurrences_json` | 🔱️trinity |
| 1 | 1 | `to_dsl_value` | 🎥️shooting |
| 1 | 1 | `ui_external_slot` | 📋️forms |
| 1 | 1 | `ui_image` | 📋️forms |
| 1 | 1 | `ui_import_drop_zone` | 📸️remodel |
| 1 | 1 | `ui_inspector_all_equal` | 🪐️space |
| 1 | 1 | `ui_inspector_mixed_slider` | 🖍️draw |
| 1 | 1 | `ui_inspector_toggle_field` | 🧩️puzzle |
| 1 | 1 | `window_element_id` | 🧩️puzzle |
| 1 | 1 | `world3d_meshes_json_from_kinds` | 🏗️fem |

Total distinct symbols: **302**.

---

## 3. Cargo dependency census — every plugin/extension crate

Source: all 63 `Cargo.toml` under `✏️s/🔌️plugins/**/📦️packages/🦀️rust/Cargo.toml`
(`find ✏️s/🔌️plugins/ -path '*/📦️packages/🦀️rust/Cargo.toml' | wc -l` → **63**).
`[dependencies]`, `[dev-dependencies]`, and any `[target.'cfg(...)'.dependencies]` sections were all
parsed (a target-cfg section matters — see 🧩️puzzle below, whose `semio-framework-os` dep is
cfg-gated, not unconditional).

### 3.1 Matrix — framework-crate dependencies per crate

`os` column = depends on the **host crate `semio-framework-os`** (forbidden by APA). `•!` marks it.

| crate (Cargo.toml dir, relative to `✏️s/🔌️plugins/`) | os-kernel | **os (HOST)** | plugin-sdk | core | 3d/2d | other framework | third-party (non-workspace, notable) |
|---|:-:|:-:|:-:|:-:|:-:|---|---|
| ✒️writer | • | •! | • |  |  | framework_editor, schema | — |
| ➗️mathematical | • |  | • | • |  | math, schema, ui_wgpu | — |
| 🌀️procedural | • | •! | • | • | • | flow, infinite_canvas, schema, ui_styling, ui_wgpu | wasm-bindgen |
| 🌊️flow | • |  | • |  |  | flow, infinite_canvas, schema, ui_wgpu | — |
| 🌊️flow/🧩️extensions/🏗️bim |  |  | • | • |  | flow_extension_sdk, neural_engine | — |
| 🌊️flow/🧩️extensions/📃️list |  |  | • | • |  | flow_extension_sdk, neural_engine | — |
| 🌊️flow/🧩️extensions/📐️brep |  |  | • | • | • | flow_extension_sdk, neural_engine | base64 |
| 🌊️flow/🧩️extensions/📖️dictionary |  |  | • | • |  | flow_extension_sdk, neural_engine | — |
| 🌊️flow/🧩️extensions/📝️text |  |  | • | • |  | flow_extension_sdk, neural_engine | — |
| 🌊️flow/🧩️extensions/🔤️primitive |  |  | • | • |  | flow_extension_sdk, neural_engine | — |
| 🌊️flow/🧩️extensions/🖍️draw |  |  | • | • | • | flow_extension_sdk, neural_engine | — |
| 🌊️flow/🧩️extensions/🧠️logic |  |  | • | • |  | flow_extension_sdk, neural_engine | — |
| 🌊️flow/🧩️extensions/🧮️math |  |  | • | • |  | flow_extension_sdk, neural_engine | — |
| 🌍️gis | • | •! | • | • |  | framework_surface, schema | wasm-bindgen |
| 🌿️vcs | • |  | • |  |  | schema | — |
| 🎞️animate | • | •! | • | • |  | framework_hash, math, schema | base64, blake3, comemo, ecow, fontdb, image, kurbo, pollster, typst, typst-assets, typst-svg, usvg, vello, wgpu |
| 🎥️shooting | • | •! | • | • |  | schema | wasm-bindgen |
| 🎪️demonstrator | • | •! | • |  | • | schema | cad, gis, procedural, process, puzzle, sourcing |
| 🎬️sequence | • |  | • | • |  | infinite_canvas, math, neural_engine, schema | js-sys, wasm-bindgen, wasm-bindgen-futures, web-sys |
| 🏗️fem | • |  | • | • |  | math, schema | spade |
| 🏛️architect | • |  | • |  |  | math, schema | blake3 |
| 🏭️process | • | •! | • | • | • | schema | base64, blake3, wasm-bindgen |
| 🏭️process/🧩️extensions/🔩️metal |  |  | • | • |  | — | wasm-bindgen |
| 🏭️process/🧩️extensions/🤖️robotic |  |  | • | • |  | — | wasm-bindgen |
| 🏭️process/🧩️extensions/🧱️concrete |  |  | • | • |  | — | wasm-bindgen |
| 🏭️process/🧩️extensions/🪵️wood |  |  | • | • |  | — | wasm-bindgen |
| 💠️lowpoly | • | •! | • |  | • | schema | base64, png |
| 💡️reasoning | • |  | • |  |  | infinite_canvas, schema | — |
| 📋️forms | • |  | • | • |  | flow, schema | blake3 |
| 📏️layout | • | •! | • | • |  | infinite_canvas, schema | base64, fontique, image, js-sys, parley, png, serde-wasm-bindgen, sha2, swash, wasm-bindgen, wasm-bindgen-futures, web-sys, zip |
| 📐️cad | • | •! | • | • | • | math, schema, ui_wgpu | base64 |
| 📐️cad/🧩️extensions/🏛️aec-building-structure |  |  | • | • |  | — | wasm-bindgen |
| 📐️cad/🧩️extensions/🏢️aec-building |  |  | • | • |  | — | wasm-bindgen |
| 📐️cad/🧩️extensions/📐️spatial-shape |  |  | • | • |  | — | wasm-bindgen |
| 📐️cad/🧩️extensions/🔥️aec-building-energy |  |  | • | • |  | — | wasm-bindgen |
| 📕️norm | • |  | • |  |  | schema | fem |
| 📖️playbook | • |  | • | • |  | flow, schema, ui_wgpu | — |
| 📖️playbook/🧩️extensions/🌀️procedural | • |  | • | • |  | flow | — |
| 📜️imperative | • |  | • |  |  | neural_engine, schema | — |
| 📜️imperative/🧩️extensions/🎮️control |  |  | • | • |  | neural_engine | wasm-bindgen |
| 📜️imperative/🧩️extensions/📝️text |  |  | • | • |  | neural_engine | wasm-bindgen |
| 📜️imperative/🧩️extensions/📣️effect |  |  | • | • |  | neural_engine | wasm-bindgen |
| 📜️imperative/🧩️extensions/🧠️logic |  |  | • | • |  | neural_engine | wasm-bindgen |
| 📜️imperative/🧩️extensions/🧮️math |  |  | • | • |  | neural_engine | wasm-bindgen |
| 📸️remodel | • | •! | • | • |  | math, schema | base64, png |
| 🔋️energy | • |  | • |  |  | math, schema | — |
| 🔱️trinity | • | •! | • | • |  | infinite_canvas, math, schema | js-sys, wasm-bindgen, wasm-bindgen-futures, web-sys |
| 🔱️trinity/🔨️modules/🔌️jack/🐚️shell | • |  |  |  |  | — | — |
| 🔱️trinity/🔨️modules/🔌️jack/🧠️lsp |  |  |  |  |  | — | wasm-bindgen |
| 🕸️dag | • |  | • |  |  | infinite_canvas, math, schema, ui_wgpu | — |
| 🖍️draw | • | •! | • | • |  | schema, semio_s_2d | base64, image |
| 🖍️draw/🔄️fsm/✨️macros |  |  |  |  |  | — | proc-macro2, quote, syn |
| 🖍️draw/🔄️fsm |  |  |  |  |  | — | js-sys, wasm-bindgen |
| 🖨️raster | • | •! | • | • |  | schema | base64, blake3 |
| 🗄️stdio | • |  | • |  |  | schema | — |
| 🗒️note | • | •! | • | • |  | schema | blake3 |
| 🧩️puzzle | • | •!(cfg-gated) | • | • |  | hash, infinite_canvas, math, schema | blake3, js-sys, nalgebra, parry3d, wasm-bindgen-futures, web-sys |
| 🧱️block | • |  | • |  |  | schema | — |
| 🪐️space | • | •! | • | • |  | infinite_canvas, schema | base64 |
| 🪵️sourcing | • |  | • | • |  | schema | wasm-bindgen |
| 🪵️sourcing/🧩️extensions/🧱️slabs |  |  | • | • |  | — | wasm-bindgen |
| 🪵️sourcing/🧩️extensions/🪟️windows |  |  | • | • |  | — | wasm-bindgen |
| 🪵️sourcing/🧩️extensions/🪵️beams |  |  | • | • |  | — | wasm-bindgen |


### 3.2 Crates depending on the HOST crate `semio-framework-os` (APA-forbidden)

`grep -n '^semio-framework-os = ' <every Cargo.toml>` plus a manual check of `[target.'cfg(...)'.dependencies]`
sections found **17 crates** (all top-level plugins, no extension sub-crate declares it) with a
`semio-framework-os` dependency:

`✒️writer` (workspace=true), `🌀️procedural`, `🌍️gis`, `🎪️demonstrator`, `🏭️process`, `📏️layout`,
`📐️cad`, `🎥️shooting`, `🎞️animate`, `💠️lowpoly`, `📸️remodel` (workspace=true), `🗒️note` (workspace=true),
`🔱️trinity` (workspace=true), `🖍️draw`, `🧩️puzzle` (**cfg-gated**: only under
`[target.'cfg(not(all(target_arch = "wasm32", target_env = "p2")))'.dependencies]` — anchor is that
literal section header string in `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml`), `🪐️space`
(feature `os-host-full`), `🖨️raster`.

For each, exact `semio_framework_os::<symbol>` uses found by
`grep -rn 'semio_framework_os::' ✏️s/🔌️plugins/<dir>/ --include='*.rs'` (bare-path hits only; each
crate's full `use semio_framework_os` block was also manually checked for brace-grouped imports —
none exist beyond what's below):

| plugin | symbols used from `semio_framework_os::` | count |
|---|---|---:|
| ✒️writer | **none found** — dependency declared, zero `semio_framework_os::` path uses in any `.rs` file (UNVERIFIED reason; possibly unused dep / re-exported via workspace default-features / used only by a build script) | 0 |
| 🌀️procedural | `register_mesh_dwg_import_handler` | 1 |
| 🌍️gis | `DwgColor`, `DwgEntity` | 2 |
| 🎪️demonstrator | `register_2d_export_handlers`, `register_dwg_import_handler`, `register_mesh_dwg_export_handler`, `register_mesh_dwg_import_handler`, `register_mesh_exporter`, `register_mesh_importer`, `register_solid_exporter`, `register_solid_importer` | 8 |
| 🏭️process | `register_mesh_dwg_import_handler` | 1 |
| 📏️layout | `DwgColor`, `DwgDrawing`, `DwgEntity`, `DwgGeometry` | 4 |
| 📐️cad | **none found** — dependency declared, zero `semio_framework_os::` path uses in any `.rs` file (UNVERIFIED reason; possibly unused dep / re-exported via workspace default-features / used only by a build script) | 0 |
| 🎥️shooting | `rasterize_svg_to_png_base64` | 1 |
| 🎞️animate | `dwg_drawing_to_svg`, `rasterize_svg_to_png_base64`, `title_card_svg` | 3 |
| 💠️lowpoly | `register_mesh_dwg_export_handler`, `register_mesh_dwg_import_handler`, `register_mesh_exporter`, `register_mesh_importer` | 4 |
| 📸️remodel | `OsMediaExportResult` | 1 |
| 🗒️note | `svg_to_dwg_bytes` | 1 |
| 🔱️trinity | **none found** — dependency declared, zero `semio_framework_os::` path uses in any `.rs` file (UNVERIFIED reason; possibly unused dep / re-exported via workspace default-features / used only by a build script) | 0 |
| 🖍️draw | **none found** — dependency declared, zero `semio_framework_os::` path uses in any `.rs` file (UNVERIFIED reason; possibly unused dep / re-exported via workspace default-features / used only by a build script) | 0 |
| 🧩️puzzle | `register_2d_export_handlers`, `register_dwg_import_handler`, `register_mesh_dwg_export_handler`, `register_mesh_dwg_import_handler`, `register_mesh_exporter`, `register_mesh_importer` | 6 |
| 🪐️space | `APP_REGISTRATIONS`, `DwgDrawing`, `OS_HOME_VFS_ROOT_ID`, `OS_SPACE_SCHEMA`, `OsBackbonePort`, `OsMediaCapability`, `OsMediaExportResult`, `OsParameter`, `OsParameterFieldBinding`, `OsParameterType`, `OsSpaceCatalogEntry`, `OsWorkflowCamera`, `SpaceKind`, `SpaceVisibility`, `VcsError`, `Workflow`, `WorkflowNode`, `WorkflowSnapshot`, `delete_os_space`, `dwg_to_bytes`, `empty_space_snapshot`, `empty_workflow_snapshot`, `export_os_app_instance_media_kind`, `host`, `import_os_app_instance_media_kind`, `import_os_space_from_dsl`, `list_os_space_catalog_entries`, `media_accept_filter_kinds`, `open_file_space_backbone`, `open_folder_space_backbone`, `os_parameter_types_compatible`, `os_workflow_to_flow_fixture`, `register_app_io`, `register_dwg_import_handler`, `validate_workflow`, `workflow` | 36 |
| 🖨️raster | `DwgColor`, `DwgDrawing`, `DwgEntity`, `DwgGeometry`, `rasterize_svg_to_png_base64` | 5 |

Union of every `semio_framework_os::` symbol actually used anywhere in `✏️s/🔌️plugins/` (this is the
candidate list for what the SDK must re-export instead, so plugins can drop the host dependency):

`APP_REGISTRATIONS`, `DwgColor`, `DwgDrawing`, `DwgEntity`, `DwgGeometry`, `OS_HOME_VFS_ROOT_ID`, `OS_SPACE_SCHEMA`, `OsBackbonePort`, `OsMediaCapability`, `OsMediaExportResult`, `OsParameter`, `OsParameterFieldBinding`, `OsParameterType`, `OsSpaceCatalogEntry`, `OsWorkflowCamera`, `SpaceKind`, `SpaceVisibility`, `VcsError`, `Workflow`, `WorkflowNode`, `WorkflowSnapshot`, `delete_os_space`, `dwg_drawing_to_svg`, `dwg_to_bytes`, `empty_space_snapshot`, `empty_workflow_snapshot`, `export_os_app_instance_media_kind`, `host`, `import_os_app_instance_media_kind`, `import_os_space_from_dsl`, `list_os_space_catalog_entries`, `media_accept_filter_kinds`, `open_file_space_backbone`, `open_folder_space_backbone`, `os_parameter_types_compatible`, `os_workflow_to_flow_fixture`, `rasterize_svg_to_png_base64`, `register_2d_export_handlers`, `register_app_io`, `register_dwg_import_handler`, `register_mesh_dwg_export_handler`, `register_mesh_dwg_import_handler`, `register_mesh_exporter`, `register_mesh_importer`, `register_solid_exporter`, `register_solid_importer`, `svg_to_dwg_bytes`, `title_card_svg`, `validate_workflow`, `workflow`

(50 distinct symbols across all 17 os-dependent crates.)

Note: `🪐️space` is the dominant consumer by far (36 of the union's symbols) and is also the only
crate using the `os-host-full` feature flag — it is functioning as a de facto second host-adjacent
crate (home/studio backbone wiring, `OsSpaceCatalogEntry`, VFS root id) rather than a plugin in the
APA sense. Flag for W1/W2 as a structural outlier, not just a re-export gap.

---


## 4. WIT host interface — `📜️wit/📜️world.wit`

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit`
(117 lines). `interface host` (lines 73–106) is what both `world plugin-world` (line 108–111) and
`world extension-world` (113–116) `import`. It has exactly **17 import funcs**. Host implementation
lives in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` (1211 lines), which has
**two separate `Host` impls**: `impl semio::framework::host::Host for HostState` (plugin-world,
line 329, anchor `impl semio::framework::host::Host for HostState {`) and
`impl extension_bindings::semio::framework::host::Host for ExtensionHostState` (extension-world,
line 887, anchor `impl extension_bindings::semio::framework::host::Host for ExtensionHostState {`).

| wit func | plugin-world impl (`HostState`, line) | plugin-world gating | extension-world impl (`ExtensionHostState`, line) | extension-world gating |
|---|---|---|---|---|
| `log` | 330 | implemented, ungated — `eprintln!`, no capability check | 888 | implemented, ungated — same |
| `now-ms` | 334 | implemented, ungated — wall clock read | 892 | implemented, ungated — same |
| `read-artifact` | 338 | **unimplemented** — always `Err("read-document not implemented")` | 896 | **unimplemented** — always `Err(...)` |
| `write-artifact` | 342 | **unimplemented** — always `Err("write-document not implemented")` | 900 | **unimplemented** — always `Err(...)` |
| `open-window` | 346 | **unimplemented** — always `Err("open-window not implemented")` | 904 | **unimplemented** — always `Err(...)` |
| `invoke-action` | 350 | **unimplemented** — always `Err("invoke-action not implemented")` | 908 | **unimplemented** — always `Err(...)` |
| `read-asset` | 354 | **unimplemented (stub)** — always `Err("unknown handle {handle}")` regardless of handle; no real asset-handle registry backs it | 912 | **unimplemented (stub)** — identical always-`Err` body |
| `network-fetch` | 358 | **unimplemented** — always `Err("network-fetch not implemented")` | 916 | **unimplemented** — always `Err(...)` |
| `write-blob` | 362 | implemented, ungated (capability-wise) — checks `self.blob_store` is `Some`, no `Rights` check | 920 | **unimplemented** — always `Err("write-blob not implemented for extension host")` |
| `read-blob` | 367 | implemented, ungated (capability-wise) — checks `self.blob_store` is `Some`, no `Rights` check | 924 | **unimplemented (stub)** — always `Err("blob not found")` |
| `backbone-send` | 372 | **capability-checked** — `self.has_backbone_access(Rights::Write)` | 928 | **unimplemented** — always `Err("backbone unavailable")` |
| `backbone-poll` | 380 | **capability-checked** — `self.has_backbone_access(Rights::Read)` | 932 | **unimplemented** — always `Err("backbone unavailable")` |
| `backbone-status` | 388 | implemented, ungated — reports map membership, no `Rights` check | 936 | implemented, ungated — hardcoded `"detached"` |
| `engine-derive` | 392 | **capability-checked** — `self.has_engine_access(Rights::Invoke)` | 940 | **unimplemented** — always `Err(...)` |
| `engine-read` | 413 | **capability-checked** — `self.has_engine_access(Rights::Read)` | 952 | **unimplemented** — always `Err(...)` |
| `io-dialects` | 403 | implemented, ungated — only checks `self.io_router.is_some()`, no `Rights`/capability check | 944 | **unimplemented** — always `Err(...)` |
| `io-compose` | 408 | implemented, ungated — only checks `self.io_router.is_some()`, no `Rights`/capability check | 948 | **unimplemented** — always `Err(...)` |

Summary: of 17 host funcs, plugin-world (`HostState`) has **4 capability-checked**
(`backbone-send`, `backbone-poll`, `engine-derive`, `engine-read`; note `engine-read`'s impl is
physically at line 413, after `io-compose`, not in declaration order), **7 implemented-and-ungated**
(`log`, `now-ms`, `write-blob`, `read-blob`, `backbone-status`, `io-dialects`, `io-compose`), and
**5 unimplemented** (`read-artifact`, `write-artifact`, `open-window`, `invoke-action`,
`network-fetch`) plus **1 unimplemented-as-permanent-stub** (`read-asset`, always errors regardless
of handle). 4 + 7 + 5 + 1 = 17. Extension-world (`ExtensionHostState`) is far more locked down: only
`log`/`now-ms`/`backbone-status` are implemented (all ungated), everything else — including the 4
plugin-world capability-checked funcs — is a hard `Err`, i.e. extensions currently cannot reach
backbone or engine IO at all.

Anchors for re-finding gating logic if lines move: `fn backbone_send(&mut self` /
`has_backbone_access(Rights::Write)`, `fn engine_derive(&mut self` / `has_engine_access(Rights::Invoke)`,
`fn engine_read(&mut self` / `has_engine_access(Rights::Read)`, `fn write_blob(&mut self` /
`no host blob store registered`.

---


## 5. `HostEffect` variants — `🎠️kernel/🦀️component.rs`

File: `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` (638 lines). `pub enum HostEffect` starts at
line 247 (anchor `pub enum HostEffect {`), ends line 387 (closing `}` before the
`IconRenderExportItem` struct). **22 variants total.** The `Rights`/`ArtifactKind`/`Scope`/
`CapabilityRequirement` types it would be graded against live in the same file: `Rights` at line 85,
`ArtifactKind` at 95, `Scope` at 108, `CapabilityRequirement` struct at 118.

`Rights` variants: `Read`, `Write`, `Invoke`, `Open`. `ArtifactKind` variants: `Document`,
`Projection`, `Window`, `Asset`, `Network`, `Backbone`, `Engine`. `Scope` variants: `Instance`,
`App`, `Plugin`, `Global`.

Construction-site counts are from `grep -rn 'HostEffect::<Variant>\b' ✏️s/🔌️plugins/ --include='*.rs'`
run separately per variant (full file:line dump saved alongside this report, see note at bottom of
this section).

| line | variant | fields (abridged) | sites in plugins | constructing plugins (dedup) | proposed `CapabilityRequirement` |
|---:|---|---|---:|---|---|
| 248 | `OpenWindow` | `{ kind, params }` | 0 | — (none; only host-side/kernel constructs this today) | Window / Open / Instance |
| 249 | `CloseWindow` | `{ window }` | 0 | — (none) | Window / Write / Instance |
| 250 | `Notify` | `{ message }` | 4 | 📸️remodel (2), 🪐️space (2) | Window / Write / Instance |
| 254 | `ClipboardWrite` | `{ fragment }` | 2 | 🧩️puzzle (2) | Document / Write / Global |
| 255 | `RequestSync` | `(unit)` | 0 | — (none) | Backbone / Invoke / Instance |
| 257 | `Navigate` | `{ uri }` | 13 | 🪐️space (13) | Window / Write / Global |
| 261 | `LoadDocument` | `{ pack, spr }` | 91 | 🏗️fem (19), 🏭️process (11), 🪵️sourcing (8), 💠️lowpoly (8), 🎞️animate (8), 🗒️note (7), 💡️reasoning (6), 🪐️space (6), 🔱️trinity (4), 📐️cad (4), 🎥️shooting (4), 🏛️architect (3), 🖍️draw (3) | Document / Write / Instance |
| 264 | `OpenExternalUrl` | `{ url }` | 2 | 🌍️gis (2) | Network / Open / Global |
| 266 | `SetPanel` | `{ panel_json }` | 0 | — (none) | Window / Write / Instance |
| 268 | `DownloadMediaExport` | `{ filename, mime_type, data, encoding? }` | 36 | 📏️layout (8), 🪐️space (8), 🎞️animate (6), 📐️cad (4), 🏭️process (2), 🗒️note (2), 🏛️architect (2), 🎥️shooting (2), 📸️remodel (1), 📋️forms (1) | Asset / Write / Instance |
| 276 | `IconRenderExport` | `{ items }` | 3 | 🎥️shooting (3) | Asset / Write / Instance |
| 281 | `RequestFileOpen` | `{ accept, read_as?, import_action, multiple }` | 16 | 🎥️shooting (5), 🪐️space (4), 📐️cad (2), 🗒️note (2), 📸️remodel (1), 🏭️process (1), 🏛️architect (1) | Asset / Open / Instance |
| 297 | `RequestMediaFrames` | `{ accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload?, args? }` | 1 | 📸️remodel (1) | Asset / Open / Instance |
| 316 | `SpawnPluginInstance` | `{ plugin_id, app_id, os_instance_id?, label?, document_json? }` | 0 | — (none) | Window / Open / Global |
| 327 | `OpenPluginInstance` | `{ plugin_id, app_id, os_instance_id? }` | 2 | 🪐️space (2) | Window / Open / Global |
| 335 | `SetActiveUtility` | `{ window_id, utility_id }` | 16 | 🧩️puzzle (11), 🖍️draw (3), 🏭️process (2) | Window / Write / Instance |
| 339 | `SetActiveTool` | `{ tool_id }` | 9 | 🧩️puzzle (9) | Window / Write / Instance |
| 342 | `OpenDialog` | `{ dialog_id, args? }` | 2 | 🧩️puzzle (2) | Window / Open / Instance |
| 352 | `DispatchAction` | `{ action, args?, delay_ms }` | 10 | 🌀️procedural (7), 🌊️flow (2), 🧩️puzzle (1) | Window / Invoke / Instance |
| 365 | `ReplayShellCommand` | `{ action_id, args? }` | 0 | — (none) | Window / Invoke / Global |
| 372 | `PatchWorld3dChrome` | `{ selection_json, vortices_json?, document_selected_ids, document_highlighted_ids? }` | 3 | 🧩️puzzle (3) | Window / Write / Instance |
| 381 | `InvokeExtension` | `{ extension_id, capability, request_json, response_action }` | 4 | 🌀️procedural (3), 🌊️flow (1) | Engine / Invoke / Plugin |

Totals: 22 variants, **6 never constructed by any plugin** (`OpenWindow`, `CloseWindow`, `RequestSync`,
`SetPanel`, `SpawnPluginInstance`, `ReplayShellCommand` — kernel/host-only today), 16 constructed
somewhere, 258 total construction sites summed across all variants. `LoadDocument` alone is 91 sites (35% of all effect construction) — it
is the dominant escape-hatch effect and the highest-value target for a dedicated capability gate.
`🧩️puzzle` is the single most prolific effect-constructing plugin (`ClipboardWrite`,
`SetActiveUtility`×11, `SetActiveTool`×9, `OpenDialog`, `DispatchAction`, `PatchWorld3dChrome`×3).

`CapabilityRequirement` proposals above are best-fit against the existing 7×4×4 enum combination and
are PROPOSED, not authoritative — `ArtifactKind` has no "Shell"/"UI" member so window-chrome effects
(`Notify`, `Navigate`, `SetPanel`, `SetActiveUtility`, `SetActiveTool`, `OpenDialog`,
`PatchWorld3dChrome`, `CloseWindow`, `ReplayShellCommand`) are all mapped onto `ArtifactKind::Window`
as the closest fit; W1/W2 should confirm whether that enum needs a new member before this becomes
policy.

Per-variant file:line construction sites (all 258, raw grep output) are saved at
`.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/📓️w0-d-hosteffect-sites.txt`
for a later wave to consume directly without re-running the grep.

---


## 6. Global registration functions reachable from plugin code

Every name from the assignment list, plus siblings found while grepping. `def crate` is the crate
owning the `fn` (not the plugin re-exporting it through the tail block in §1). Call-site counts are
`grep -rn '\b<fn>(' ✏️s/🔌️plugins/ --include='*.rs' | wc -l` (raw occurrences, including the
definition's own doc-tests if any live under `✏️s/`; none did here).

| function | defining file : line | defining crate | signature | call sites in `✏️s/🔌️plugins/` |
|---|---|---|---|---:|
| `register_language` | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs:525` | semio-framework-os-kernel (dsl module) | `pub fn register_language(spec: LanguageSpec)` | 433 |
| `register_document_codec` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:629` | semio-framework-os-kernel (store module) | `pub fn register_document_codec(codec: ArtifactCodec)` | 54 |
| `register_composer_entries` | `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:311` | semio-framework (core, io module) | `pub fn register_composer_entries(entries: &'static [ComposerEntry])` | 109 |
| `set_io_fallback_dispatcher` | `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:392` | semio-framework (core, io module) | `pub fn set_io_fallback_dispatcher<F>(hook: F) where F: Fn(&IoKey, &[ErasedComposeSource]) -> Option<Result<ComposedArtifact, ComposeError>> + Send + Sync + 'static` | 0 |
| `register_subset_validator` | `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:480` | semio-framework (core, io module) | `pub fn register_subset_validator(entry: &'static SubsetValidatorEntry)` | 44 |
| `register_format_descriptors` | `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:678` | semio-framework (core, io module) | `pub fn register_format_descriptors(rows: Vec<FormatDescriptor>)` | 1 |
| `register_artifact_schema_descriptor` | `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:264` | semio-framework-schema | `pub fn register_artifact_schema_descriptor(descriptor: ArtifactSchemaDescriptor)` | 107 |
| `register_artifact_inference_descriptor` | `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:376` | semio-framework-schema | `pub fn register_artifact_inference_descriptor(descriptor: ArtifactInferenceDescriptor)` | 71 |
| `register_app_schema_descriptor` | `🧰️framework/🔨️modules/🧬️schema/🦀️component.rs:497` | semio-framework-schema | `pub fn register_app_schema_descriptor(descriptor: AppSchemaDescriptor)` | 39 |
| `register_dialect_migration` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:676` | semio-framework-os-kernel (store module) | `pub fn register_dialect_migration(migration: DialectMigration)` | 1 |
| `register_document_codec_for_app` | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:6510` | semio-framework-plugin | `pub fn register_document_codec_for_app<A: ArtifactApp>(schema: impl Into<String>)` — inherent assoc fn, thin wrapper calling `store::register_document_codec` | 44 |
| `register_studio_port` | `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🦀️component.rs:110` | **not a framework SDK fn** — plugin-local, `pub(crate)` in `🪐️space` | `pub(crate) fn register_studio_port(space_id: &str, port: Arc<dyn OsBackbonePort>)` | 3 |
| `register_linked_flow_extension_installer` | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️component.rs:79` | semio-framework-os-flow | `pub fn register_linked_flow_extension_installer(extension_id: impl Into<String>, install: LinkedFlowExtensionInstall)` | 7 |

Notes:
- `set_io_fallback_dispatcher` — genuinely 0 call sites in `✏️s/🔌️plugins/` (checked both the
  `fn(`-suffixed form and the bare identifier with no trailing paren); only ever set from the
  host/OS boot path today, not plugin code. Confirms it's a host-owned hook, not part of the
  plugin-facing registrar surface — worth excluding from the APA registrar census unless a future
  plugin needs it.
- `register_document_codec_for_app` is **always** called through its generic turbofish form —
  `semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<SomeApp>(SCHEMA)` — so a
  naive `register_document_codec_for_app(` grep undercounts it (turbofish `::<...>` sits between the
  name and the paren). The 44 above is the correct count, from a bare-identifier grep
  (`grep -rn register_document_codec_for_app ✏️s/🔌️plugins/ --include='*.rs' | wc -l`, 42 distinct
  files). Every call site fully-qualifies through `plugin_runtime::`, never through the flat
  `semio_framework_plugin::` tail-block re-export (line 9640–9644 in §1) — i.e. plugins reach it via
  the module path, not the re-export, even though the re-export also makes it available.

`register_studio_port` is **not** a framework-defined registrar at all — it's a `pub(crate)` fn
private to the `🪐️space` plugin's own `🏠️home` app, included here only because the assignment named
it explicitly. It registers into a plugin-local `shared_studio_ports()` static, not any
framework-owned registry. Flag for W1/W2: if a plugin needs a *framework* studio-port registrar,
one doesn't currently exist under that name anywhere in `🧰️framework/`.

No additional sibling registrars (`register_*`) turned up beyond the assignment's list and the ones
already covered in §1/§2 (`register_host_backbone_channel` at plugin tail-block line 9639,
`register_composer_entries`, `register_subset_validator` — all already listed above or in §1).

---

## Appendix — scratch files produced by this census

- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE/📓️w0-d-hosteffect-sites.txt`
  — full raw `grep -rn` file:line dump for all 22 `HostEffect::*` variants (§5 backing data).
