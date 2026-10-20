//! 📐️ Generation3d artifact — snapshot re-exports, widget id helper, and artifact kind.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;





pub use semio_s_artifact_procedural_3d_editor::Generation3dPlayApp;

//#region 🔖️Declaration
/// 🧾️ Defines s.generation3d's immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.procedural.generation3d")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.procedural.generation3d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.procedural.generation3d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.procedural.generation3d.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.procedural.generation3d.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.procedural.generation3d@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.procedural.generation3d@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.composer.txt")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.txt@utf-8/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.txt@utf-8/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"generation.3d:generation3d")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "generation.3d")?)?
                .claim(ArtifactIdentityClaim::codec_extension("generation.3d", "generation3d")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"3D Generation")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "3D Generation")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.procedural.generation3d.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"3D Generierung")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "3D Generierung")?)?,
        )
}

/// 🔖️ Assembles s.generation3d's typed runtime declaration.
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(semio_s_artifact_procedural_3d_model::generation3d_artifact_schema_descriptor())
        .inferences([semio_s_artifact_procedural_3d_inferences::generation3d_artifact_inference_descriptor()])
        .inference_services([semio_s_artifact_procedural_3d_geometry_service::geometry_inference_service()])
        .composers(semio_s_artifact_procedural_3d_io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_procedural_3d_editor::Generation3dPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration

//#region 🔖️Manifest
/// 🗂️ The generation3d editor's app definition: the composition of its actions, commands, windows, modes and panels.
pub mod manifest {
    use semio_s_artifact_procedural_3d_editor_commands::add_generation;
    use semio_s_artifact_procedural_3d_add_widget as add_widget;
    use semio_s_artifact_procedural_3d_editor_commands::cycle_lod_mode;
    use semio_s_artifact_procedural_3d_editor_commands::cycle_show_mode;
    use semio_s_artifact_procedural_3d_delete_selection as delete_selection;
    use semio_s_artifact_procedural_3d_export_document as export_document;
    use semio_s_artifact_procedural_3d_editor_commands::flow_eval_release;
    use semio_s_artifact_procedural_3d_editor_commands::flow_eval_resolve;
    use semio_s_artifact_procedural_3d_flow_eval_tick as flow_eval_tick;
    use semio_s_artifact_procedural_3d_editor_commands::flow_tessellate_cancel_resolve;
    use semio_s_artifact_procedural_3d_editor_commands::flow_tessellate_resolve;
    use semio_s_artifact_procedural_3d_import_document as import_document;
    use semio_s_artifact_procedural_3d_editor_commands::import_document_request;
    use semio_s_artifact_procedural_3d_node_graph_edit as node_graph_edit;
    use semio_s_artifact_procedural_3d_editor_commands::node_graph_viewport;
    use semio_s_artifact_procedural_3d_editor_commands::patch_flow_widgets;
    use semio_s_artifact_procedural_3d_editor_commands::remove_generation;
    use semio_s_artifact_procedural_3d_editor_commands::remove_widget;
    use semio_s_artifact_procedural_3d_editor_commands::rename_generation;
    use semio_s_artifact_procedural_3d_editor_commands::reorganize;
    use semio_s_artifact_procedural_3d_rotate_selection as rotate_selection;
    use semio_s_artifact_procedural_3d_scale_selection as scale_selection;
    use semio_s_artifact_procedural_3d_editor_commands::select_generation;
    use semio_s_artifact_procedural_3d_editor_commands::set_active_example;
    use semio_s_artifact_procedural_3d_editor_commands::set_camera;
    use semio_s_artifact_procedural_3d_set_contributions as set_contributions;
    use semio_s_artifact_procedural_3d_editor_commands::set_lod_mode;
    use semio_s_artifact_procedural_3d_editor_commands::set_show_mode;
    use semio_s_artifact_procedural_3d_editor_commands::set_sun_azimuth;
    use semio_s_artifact_procedural_3d_editor_commands::set_sun_elevation;
    use semio_s_artifact_procedural_3d_editor_commands::set_sun_intensity;
    use semio_s_artifact_procedural_3d_editor_commands::toggle_sun;
    use semio_s_artifact_procedural_3d_editor_commands::translate_selection;
    use semio_s_artifact_procedural_3d_editor_commands::update_generation_values;
    use semio_s_artifact_procedural_3d_set_widget_input as set_widget_input;
    use semio_s_artifact_procedural_3d_mode_edit::windows::flow as flow_window;
    use semio_s_artifact_procedural_3d_mode_edit::windows::preview as edit_preview;
    use semio_s_artifact_procedural_3d_mode_generate::windows::form;
    use semio_s_artifact_procedural_3d_mode_generate::windows::generations;
    use semio_s_artifact_procedural_3d_mode_generate::windows::preview as generate_preview;
    use semio_s_artifact_procedural_3d_mode_edit as edit;
    use semio_s_artifact_procedural_3d_mode_generate as generate;
    use semio_s_artifact_procedural_3d_editor_panels::catalogue as catalogue_panel;
    use semio_s_artifact_procedural_3d_editor_panels::artifact as artifact_panel;
    use semio_s_artifact_procedural_3d_editor_panels::inspection as inspection_panel;
    use semio_s_artifact_procedural_3d_model::artifact_kind;
    use semio_framework_plugin::ActionArgDef;
    use semio_framework_plugin::ActionArgOption;
    use semio_framework_plugin::ActionDefinition;
    use semio_framework_plugin::ActionKind;
    use semio_framework_plugin::CommandDefinition;
    use semio_framework_plugin::Editor;
    use semio_framework_plugin::GranularityDefinition;
    use semio_framework_plugin::HierarchyProvider;
    use semio_framework_plugin::HoverSpec;
    use semio_framework_plugin::InteractionDefinition;
    use semio_framework_plugin::InteractionRef;
    use semio_framework_plugin::InteractiveJobClassification;
    use semio_framework_ui_locale::LocalizedLabel;
    use semio_framework_plugin::MergeMode;
    use semio_framework_plugin::SelectionMethod;
    use semio_framework_plugin::SelectionMode;
    use semio_framework_plugin::SelectionSpec;
    use semio_framework_plugin::UtilityDefinition;
    use semio_s_artifact_procedural_3d_mode_edit::windows::preview::PreviewInteractionMarks;
    use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_selection_json;
    use semio_s_artifact_procedural_3d_editor::*;

    fn categorized_action(id: &str, label: LocalizedLabel, kind: ActionKind, category: &str) -> ActionDefinition {
        ActionDefinition::bounded_catalog(id, label, kind).with_category(category)
    }
    /// 🧵️ Classifies the internal flow continuation as backed by its bounded first-step factory.
    fn migrated_command(mut definition: CommandDefinition) -> CommandDefinition {
        definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
        definition
    }
    pub fn create_generation3d_app() -> semio_framework_plugin::AppDefinition {
        let builder = Editor::builder(semio_s_artifact_procedural_3d_model::GENERATION3D_DIALECT);
    let builder = create_generation3d_app_part_1(builder);
    let builder = create_generation3d_app_part_2(builder);
    let builder = create_generation3d_app_part_3(builder);
    let builder = create_generation3d_app_part_4(builder);
    let builder = create_generation3d_app_part_5(builder);
        builder.build_definition()
    }

/// 🧱️ The manifest slice 1 of 5 of `create_generation3d_app`: its builder calls.
fn create_generation3d_app_part_1(builder: semio_framework_plugin::app::EditorBuilder) -> semio_framework_plugin::app::EditorBuilder {
    builder.document(["semio", "procedural", "3d"])
                .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), "runtime", ActionKind::View) }))
                .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalResolve", LocalizedLabel::native("Resolve Flow Evaluation", "Flow-Auswertung aufnehmen"), "runtime", ActionKind::View) }))
                .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowTessellateResolve", LocalizedLabel::native("Resolve Preview Tessellation", "Vorschau-Tessellierung aufnehmen"), "runtime", ActionKind::View) }))
                .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalRelease", LocalizedLabel::native("Release Preview Kernel Work", "Vorschau-Kernelarbeit freigeben"), "runtime", ActionKind::View) }))
                .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowTessellateCancelResolve", LocalizedLabel::native("Resolve Preview Cancellation", "Vorschauabbruch aufnehmen"), "runtime", ActionKind::View) }))
                .command(migrated_command(CommandDefinition {
                    in_palette: false,
                    ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([
                        ActionArgDef::text("json", LocalizedLabel::native("Contributions Page", "Beiträge-Seite")),
                        ActionArgDef::text("page", LocalizedLabel::native("Page", "Seite")),
                        ActionArgDef::text("pageCount", LocalizedLabel::native("Page Count", "Seitenanzahl")),
                    ])
                }))
                .artifact_kind(artifact_kind())
                .icon_id("workflow")
                .terminology("reuse")
                .terminology_document("reuse", ["Entwerfen mit Bestand", "Generator"])
                .mode_def(edit::definition())
                .mode_def(generate::definition())
                .default_mode_id(edit::GENERATION_3D_PLAY_MODE_EDIT)
                .mode_layout(generate::GENERATION_3D_PLAY_MODE_GENERATE, generate::GENERATION_3D_PLAY_LAYOUT_GENERATE)
                .window_kind_def(flow_window::definition())
                .window_kind_def(edit_preview::definition())
                .window_kind_def(generations::definition())
                .window_kind_def(form::definition())
                .window_kind_def(generate_preview::definition())
                .tool(semio_s_artifact_procedural_3d_preview_eval::preview_eval_tool_definition())
                .default_layout(edit::layout())
                .named_layout(generate::layout())
                .panel_tab_def(artifact_panel::definition())
                .panel_tab_def(catalogue_panel::definition())
                .panel_tab_def(inspection_panel::definition())
                .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
                .action_with(categorized_action("editMeshSelection", LocalizedLabel::native("Edit Mesh Selection", "Netzauswahl bearbeiten"), ActionKind::Mutation, "methods"))
                .action_with(categorized_action("knifeMeshSelection", LocalizedLabel::native("Knife Cut Selected Face", "Ausgewählte Fläche schneiden"), ActionKind::Mutation, "methods"))
                .mutation("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"))
                .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
                .action_destructive("deleteSelection")
                .action_with(categorized_action("removeWidget", LocalizedLabel::native("Remove Widget", "Element entfernen"), ActionKind::Mutation, "targets"))
                .action_destructive("removeWidget")
                .action_with(categorized_action("addWidget", LocalizedLabel::native("Add Widget", "Element hinzufügen"), ActionKind::Mutation, "create"))
                .action_with(categorized_action("patchFlowWidgets", LocalizedLabel::native("Patch Flow Widgets", "Flow-Elemente aktualisieren"), ActionKind::Mutation, "methods"))
                .action_with(categorized_action("setWidgetInput", LocalizedLabel::native("Edit Widget Input", "Elementeingabe bearbeiten"), ActionKind::Mutation, "methods"))
                .action_with(categorized_action("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "transform"))
                .action_with(categorized_action("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Mutation, "transform"))
                .action_with(categorized_action("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Mutation, "transform"))
                .action_with(categorized_action("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Mutation, "transform"))
                .action_with(categorized_action("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::Mutation, "create"))
                .action_with(categorized_action("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"), ActionKind::Mutation, "targets"))
                .action_destructive("removeGeneration")
                .action_with(categorized_action("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"), ActionKind::Mutation, "methods"))
                .action_with(categorized_action("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"), ActionKind::Mutation, "methods"))
                .shell_action("importDocumentRequest", LocalizedLabel::native("Import Artifact…", "Artefakt importieren…"))
                .shell_action("exportDocument", LocalizedLabel::native("Export Artifact", "Artefakt exportieren"))
                .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importDocument", LocalizedLabel::native("Import Artifact File", "Artefaktdatei importieren"), ActionKind::Mutation) })
                .action_with(ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Set Viewport", "Ansicht festlegen"), ActionKind::View, "camera"))
                .action_with(ActionDefinition::new("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"), ActionKind::View, "layers"))
                
}

/// 🪵️ The manifest slice 2 of 5 of `create_generation3d_app`: its builder calls.
fn create_generation3d_app_part_2(builder: semio_framework_plugin::app::EditorBuilder) -> semio_framework_plugin::app::EditorBuilder {
    builder.view_action("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"))
                .action_with(ActionDefinition::new("cycleShowMode", LocalizedLabel::native("Cycle Show Mode", "Anzeigemodus wechseln"), ActionKind::View, "eye"))
                .action_with(ActionDefinition::new("cycleLodMode", LocalizedLabel::native("Cycle Lod Mode", "LOD-Modus wechseln"), ActionKind::View, "layers"))
                .action_with(ActionDefinition::new("selectNextNode", LocalizedLabel::native("Select Next Node", "Nächsten Knoten auswählen"), ActionKind::View, "arrow-down"))
                .action_with(ActionDefinition::new("selectPreviousNode", LocalizedLabel::native("Select Previous Node", "Vorherigen Knoten auswählen"), ActionKind::View, "arrow-up"))
                .action_with(ActionDefinition::new("selectUpstreamNode", LocalizedLabel::native("Select Upstream Node", "Vorgelagerten Knoten auswählen"), ActionKind::View, "arrow-left"))
                .action_with(ActionDefinition::new("selectDownstreamNode", LocalizedLabel::native("Select Downstream Node", "Nachgelagerten Knoten auswählen"), ActionKind::View, "arrow-right"))
                .action_with(ActionDefinition::new("activateSelection", LocalizedLabel::native("Open Node Ports", "Knotenanschlüsse öffnen"), ActionKind::View, "circle"))
                .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
                .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
                .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
                .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
                .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
                .view_action("selectGeneration", LocalizedLabel::native("Set Generation", "Generation auswählen"))
                .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
                .action_destructive("setActiveExample")
                .action_interactive_job("editMeshSelection", InteractiveJobClassification::Migrated)
                .action_interactive_job("knifeMeshSelection", InteractiveJobClassification::Migrated)
                .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
                .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
                .action_interactive_job("removeWidget", InteractiveJobClassification::Migrated)
                .action_interactive_job("addWidget", InteractiveJobClassification::Migrated)
                .action_interactive_job("patchFlowWidgets", InteractiveJobClassification::Migrated)
                .action_interactive_job("setWidgetInput", InteractiveJobClassification::Migrated)
                .action_interactive_job("reorganize", InteractiveJobClassification::Migrated)
                .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
                .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
                .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
                .action_interactive_job("addGeneration", InteractiveJobClassification::Migrated)
                .action_interactive_job("removeGeneration", InteractiveJobClassification::Migrated)
                .action_interactive_job("renameGeneration", InteractiveJobClassification::Migrated)
                .action_interactive_job("updateGenerationValues", InteractiveJobClassification::Migrated)
                .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
                .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
                .action_interactive_job("setShowMode", InteractiveJobClassification::Migrated)
                .action_interactive_job("cycleShowMode", InteractiveJobClassification::Migrated)
                .action_interactive_job("cycleLodMode", InteractiveJobClassification::Migrated)
                .action_interactive_job("selectNextNode", InteractiveJobClassification::Migrated)
                .action_interactive_job("selectPreviousNode", InteractiveJobClassification::Migrated)
                .action_interactive_job("selectUpstreamNode", InteractiveJobClassification::Migrated)
                .action_interactive_job("selectDownstreamNode", InteractiveJobClassification::Migrated)
                .action_interactive_job("activateSelection", InteractiveJobClassification::Migrated)
                .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
                .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
                .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
                .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
                .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
                .action_interactive_job("selectGeneration", InteractiveJobClassification::Migrated)
                .action_interactive_job("flowEvalTick", InteractiveJobClassification::Migrated)
                .action_interactive_job("flowEvalResolve", InteractiveJobClassification::Migrated)
                .action_interactive_job("flowTessellateResolve", InteractiveJobClassification::Migrated)
                .action_interactive_job("flowTessellateCancelResolve", InteractiveJobClassification::Migrated)
                .action_interactive_job("setContributions", InteractiveJobClassification::Migrated)
                .action_interactive_job("importDocumentRequest", InteractiveJobClassification::Migrated)
                .action_interactive_job("importDocument", InteractiveJobClassification::Migrated)
                
}

/// 🔩️ The manifest slice 3 of 5 of `create_generation3d_app`: its builder calls.
fn create_generation3d_app_part_3(builder: semio_framework_plugin::app::EditorBuilder) -> semio_framework_plugin::app::EditorBuilder {
    builder.action_interactive_job("exportDocument", InteractiveJobClassification::Migrated)
                .action_destructive("exportDocument")
                .action_args("editMeshSelection", vec![
                    ActionArgDef::select("operation", LocalizedLabel::native("Operation", "Operation"), vec![
                        ActionArgOption::new("extrude", LocalizedLabel::native("Extrude Faces", "Flächen extrudieren")),
                        ActionArgOption::new("inset", LocalizedLabel::native("Inset Faces", "Flächen einziehen")),
                        ActionArgOption::new("subdivide", LocalizedLabel::native("Subdivide Faces", "Flächen unterteilen")),
                        ActionArgOption::new("flip", LocalizedLabel::native("Flip Faces", "Flächen umkehren")),
                        ActionArgOption::new("deleteFaces", LocalizedLabel::native("Delete Faces", "Flächen löschen")),
                        ActionArgOption::new("moveVertices", LocalizedLabel::native("Move Vertices", "Eckpunkte verschieben")),
                        ActionArgOption::new("loopCut", LocalizedLabel::native("Cut Edge Loops", "Kantenschleifen schneiden")),
                        ActionArgOption::new("bevel", LocalizedLabel::native("Bevel Edges", "Kanten abschrägen")),
                        ActionArgOption::new("dissolveEdges", LocalizedLabel::native("Dissolve Edges", "Kanten auflösen")),
                        ActionArgOption::new("dissolveVertices", LocalizedLabel::native("Dissolve Vertices", "Eckpunkte auflösen")),
                        ActionArgOption::new("mergeVertices", LocalizedLabel::native("Merge Vertices", "Eckpunkte zusammenführen")),
                        ActionArgOption::new("moveProportional", LocalizedLabel::native("Move Proportionally", "Proportional verschieben")),
                        ActionArgOption::new("snapVertices", LocalizedLabel::native("Snap Vertices to Grid", "Eckpunkte am Raster ausrichten")),
                        ActionArgOption::new("filletEdges",LocalizedLabel::native("Fillet B-Rep Edges","B-Rep-Kanten verrunden")),
                        ActionArgOption::new("chamferEdges",LocalizedLabel::native("Chamfer B-Rep Edges","B-Rep-Kanten fasen")),
                        ActionArgOption::new("shell",LocalizedLabel::native("Shell B-Rep Faces","B-Rep-Flächen aushöhlen")),
                        ActionArgOption::new("draft",LocalizedLabel::native("Draft B-Rep Faces","B-Rep-Flächen mit Formschräge versehen")),
                        ActionArgOption::new("defeature",LocalizedLabel::native("Remove B-Rep Features","B-Rep-Formelemente entfernen")),
                    ]).required().default_value(&"extrude"),
                    ActionArgDef::number("amount", LocalizedLabel::native("Distance / Inset", "Abstand / Einzug")).default_value(&0.1),
                    ActionArgDef { schema: semio_framework::ArgSchema::number(Some(1.0), Some(256.0), Some(1.0), true), ..ActionArgDef::number("cuts", LocalizedLabel::native("Loop Cuts", "Schleifenschnitte")).default_value(&1) },
                    ActionArgDef::number("dx", LocalizedLabel::native("Move X", "Verschieben X")).default_value(&0.0),
                    ActionArgDef::number("dy", LocalizedLabel::native("Move Y", "Verschieben Y")).default_value(&0.0),
                    ActionArgDef::number("dz", LocalizedLabel::native("Move Z", "Verschieben Z")).default_value(&0.0),
                    ActionArgDef::number("width", LocalizedLabel::native("Bevel Width", "Fasenbreite")).default_value(&0.1),
                    ActionArgDef { schema: semio_framework::ArgSchema::number(Some(1.0), Some(64.0), Some(1.0), true), ..ActionArgDef::number("segments", LocalizedLabel::native("Bevel Segments", "Fasensegmente")).default_value(&1) },
                    ActionArgDef::select("mergeMode", LocalizedLabel::native("Merge Mode", "Zusammenführungsmodus"), vec![
                        ActionArgOption::new("first", LocalizedLabel::native("First Vertex", "Erster Eckpunkt")),
                        ActionArgOption::new("center", LocalizedLabel::native("Center", "Mittelpunkt")),
                        ActionArgOption::new("distance", LocalizedLabel::native("By Distance", "Nach Abstand")),
                    ]).default_value(&"center"),
                    ActionArgDef::number("tolerance", LocalizedLabel::native("Merge Tolerance", "Zusammenführungstoleranz")).default_value(&0.0001),
                    ActionArgDef::number("radius", LocalizedLabel::native("Proportional Radius", "Proportionaler Radius")).default_value(&1.0),
                    ActionArgDef::number("grid", LocalizedLabel::native("Grid Spacing", "Rasterabstand")).default_value(&1.0),
                    ActionArgDef::vector("center", LocalizedLabel::native("Proportional Center", "Proportionaler Mittelpunkt"), 3).default_value(&[0.0, 0.0, 0.0]),
                    ActionArgDef::number("angle",LocalizedLabel::native("Draft Angle (rad)","Formschrägenwinkel (rad)")).default_value(&0.1),
                    ActionArgDef::vector("pullDirection",LocalizedLabel::native("Pull Direction","Entformungsrichtung"),3).default_value(&[0.0,0.0,1.0]),
                    ActionArgDef::vector("neutralPoint",LocalizedLabel::native("Neutral Point","Neutralpunkt"),3).default_value(&[0.0,0.0,0.0]),
                ])
                .action_args("knifeMeshSelection", vec![
                    ActionArgDef::vector("start", LocalizedLabel::native("Cut Start", "Schnittanfang"), 3).required().default_value(&[0.0, -1.0, 0.0]),
                    ActionArgDef::vector("end", LocalizedLabel::native("Cut End", "Schnittende"), 3).required().default_value(&[0.0, 1.0, 0.0]),
                ])
                .action_args("addWidget", vec![
                    ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                        ActionArgOption::new("neuron", LocalizedLabel::native("Neuron", "Neuron")),
                        ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
                        ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
                        ActionArgOption::new("outputPreview", LocalizedLabel::native("Preview", "Vorschau")),
                    ]).default_value(&"inputSlider"),
                    ActionArgDef::text("neuronKind", LocalizedLabel::native("Operator", "Operator")),
                    ActionArgDef::text("format", LocalizedLabel::native("Export Format", "Exportformat")),
                    ActionArgDef::text("action", LocalizedLabel::native("Action", "Aktion")),
                ])
                .action_args("setWidgetInput", vec![
                    ActionArgDef::select("facet", LocalizedLabel::native("Widget field", "Elementfeld"), vec![ActionArgOption::new("port", LocalizedLabel::native("Operator input", "Operatoreingang")), ActionArgOption::new("variableName", LocalizedLabel::native("Variable name", "Variablenname")), ActionArgOption::new("variableSchema", LocalizedLabel::native("Variable type", "Variablentyp")), ActionArgOption::new("exportFormat", LocalizedLabel::native("Export format", "Exportformat")), ActionArgOption::new("meshSource", LocalizedLabel::native("Mesh source", "Netzquelle"))]),
                    ActionArgDef::text("widgetId", LocalizedLabel::native("Widget", "Element")).required(),
                    ActionArgDef::text("channel", LocalizedLabel::native("Input", "Eingabe")).required(),
                    ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).required(),
                    ActionArgDef::text("component", LocalizedLabel::native("Coordinate", "Koordinate")),
                    ActionArgDef::select("operation", LocalizedLabel::native("List Operation", "Listenoperation"), vec![ActionArgOption::new("set",LocalizedLabel::native("Edit Item","Eintrag bearbeiten")),ActionArgOption::new("add",LocalizedLabel::native("Add Item","Eintrag hinzufügen")),ActionArgOption::new("remove",LocalizedLabel::native("Remove Item","Eintrag entfernen")),ActionArgOption::new("move",LocalizedLabel::native("Move Item","Eintrag verschieben"))]),
                    ActionArgDef::number("index", LocalizedLabel::native("Item Index", "Eintragsindex")),
                    ActionArgDef::number("destination", LocalizedLabel::native("Destination Index", "Zielindex")),
                    ActionArgDef::text_list("path", LocalizedLabel::native("Mesh field", "Netzfeld")),
                ])
                .action_args("exportDocument", vec![
                    ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), semio_s_artifact_procedural_3d_io::document_io::export_format_options()).required().default_value(&"stl"),
                    ActionArgDef::text("widgetId", LocalizedLabel::native("Export output", "Exportausgabe")),
                ])
                .action_args("setActiveExample", vec![
                    ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                        ActionArgOption::new(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_HEX_COLUMN, LocalizedLabel::native("Hexagonal Mushroom Column", "Sechseckige Pilzsäule")),
                        ActionArgOption::new(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_MESH_WORKBENCH, LocalizedLabel::native("Mesh Workbench", "Netzwerkstatt")),
                        ActionArgOption::new(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_RECT_EXTRUDE, LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen")),
                        ActionArgOption::new(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_SPHERE_TORUS, LocalizedLabel::native("Sphere Cut With Torus", "Kugel mit Torus geschnitten")),
                        ActionArgOption::new(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_BOX_FILLET, LocalizedLabel::native("Box Fillet Preview", "Kantenrundung Vorschau")),
                        ActionArgOption::new(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, LocalizedLabel::native("Sphere Box Fuse", "Kugel und Quader vereinen")),
                        ActionArgOption::new(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, LocalizedLabel::native("Face Sweep Extrude", "Fläche extrudieren")),
                        ActionArgOption::new(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, LocalizedLabel::native("Rectangle Wire Preview", "Rechteck-Draht Vorschau")),
                        ActionArgOption::new(semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_BOX_SHELL, LocalizedLabel::native("Box Shell Preview", "Hohlkörper Vorschau")),
                    ]).required(),
                ])
                .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", LocalizedLabel::native("Move", "Verschieben"), "move") })
                .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw") })
                .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2") })
                .window_kind_utilities(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into()])
                .window_kind_utilities(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into()])
                
}

/// ⚙️ The manifest slice 4 of 5 of `create_generation3d_app`: its builder calls.
fn create_generation3d_app_part_4(builder: semio_framework_plugin::app::EditorBuilder) -> semio_framework_plugin::app::EditorBuilder {
    builder.window_kind_action_refs(
                    flow_window::GENERATION_3D_PLAY_WINDOW_MAIN,
                    vec![
                        "nodeGraphEdit".into(),
                        "nodeGraphViewport".into(),
                        "setLodMode".into(),
                        "selectNextNode".into(),
                        "selectPreviousNode".into(),
                        "selectUpstreamNode".into(),
                        "selectDownstreamNode".into(),
                        "activateSelection".into(),
                    ],
                )
                .window_kind_action_refs(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec![
                    "editMeshSelection".into(),
                    "knifeMeshSelection".into(),
                    "setCamera".into(),
                    "setShowMode".into(),
                    "toggleSun".into(),
                    "setSunAzimuth".into(),
                    "setSunElevation".into(),
                    "setSunIntensity".into(),
                    "translateSelection".into(),
                    "rotateSelection".into(),
                    "scaleSelection".into(),
                ])
                .window_kind_action_refs(generations::GENERATION_3D_PLAY_WINDOW_GENERATIONS, vec!["addGeneration".into(), "selectGeneration".into(), "renameGeneration".into(), "removeGeneration".into()])
                .window_kind_action_refs(form::GENERATION_3D_PLAY_WINDOW_GENERATE_FORM, vec!["updateGenerationValues".into()])
                .window_kind_action_refs(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW, vec![
                    "setCamera".into(),
                    "setShowMode".into(),
                    "toggleSun".into(),
                    "setSunAzimuth".into(),
                    "setSunElevation".into(),
                    "setSunIntensity".into(),
                    "translateSelection".into(),
                    "rotateSelection".into(),
                    "scaleSelection".into(),
                ])
                .interaction(InteractionDefinition {
                    id: "graph".into(),
                    label: LocalizedLabel::native("Graph", "Graph"),
                    granularities: vec![
                        GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
                        GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
                        GranularityDefinition { id: "handle".into(), label: LocalizedLabel::native("Handle", "Griff"), icon_id: "move".into() },
                    ],
                    hierarchy: HierarchyProvider::Topology,
                    hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                    selection: SelectionSpec {
                        modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                        methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                        merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                        transitive: false,
                        broadcast: true,
                    },
                })
                .interaction(InteractionDefinition {
                    id: selection::DOMAIN.into(),
                    label: LocalizedLabel::native("Geometry", "Geometrie"),
                    granularities: vec![
                        GranularityDefinition { id: "object".into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "box".into() },
                        GranularityDefinition { id: "vertex".into(), label: LocalizedLabel::native("Vertex", "Eckpunkt"), icon_id: "circle".into() },
                        GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
                        GranularityDefinition { id: "face".into(), label: LocalizedLabel::native("Face", "Fläche"), icon_id: "square".into() },
                    ],
                    hierarchy: HierarchyProvider::Flat,
                    hover: HoverSpec::default(),
                    selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle], merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive], transitive: false, broadcast: true },
                })
                .window_kind_interactions(flow_window::GENERATION_3D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("graph")])
                .window_kind_interactions(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW, vec![InteractionRef::new("graph"), InteractionRef::new(selection::DOMAIN)])
                .window_kind_interactions(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW, vec![InteractionRef::new("graph")])
                .keybinding("mod+shift+m", "editMeshSelection")
                .keybinding("mod+shift+k", "knifeMeshSelection")
                .keybinding("mod+z", "undo")
                .keybinding("mod+shift+z", "redo")
                .keybinding("mod+alt+l", "reorganize")
                .keybinding("mod+o", "importDocumentRequest")
                .keybinding("mod+shift+e", "exportDocument")
                .keybinding("delete,backspace", "deleteSelection")
                .keybinding("mod+shift+g", "addGeneration")
                .keybinding("mod+alt+d", "cycleShowMode")
                .keybinding("mod+alt+k", "cycleLodMode")
                .keybinding("arrowdown", "selectNextNode")
                .keybinding("arrowup", "selectPreviousNode")
                
}

/// 🧲️ The manifest slice 5 of 5 of `create_generation3d_app`: its builder calls.
fn create_generation3d_app_part_5(builder: semio_framework_plugin::app::EditorBuilder) -> semio_framework_plugin::app::EditorBuilder {
    builder.keybinding("arrowleft", "selectUpstreamNode")
                .keybinding("arrowright", "selectDownstreamNode")
                .config(Generation3dPlayApp::config_spec())
                .io(::semio_framework_async::poll::resolve_ready(generation3d_io()))
                .action_describe("addGeneration", LocalizedLabel::native("Adds a new generation, a named set of input values for the 3D generator, to the generation list and selects it.", "Fügt der Generationsliste eine neue Generation hinzu, einen benannten Satz von Eingabewerten für den 3D-Generator, und wählt sie aus."))
                .action_describe("selectGeneration", LocalizedLabel::native("Selects the generation with the given id, whose input values the generator then evaluates and shows.", "Wählt die Generation mit der angegebenen Id aus, deren Eingabewerte der Generator dann auswertet und zeigt."))
                .action_describe("renameGeneration", LocalizedLabel::native("Renames one generation of the generation list.", "Benennt eine Generation der Generationsliste um."))
                .action_describe("removeGeneration", LocalizedLabel::native("Removes one generation by id from the generation list, with its input values.", "Entfernt eine Generation anhand ihrer Id samt ihrer Eingabewerte aus der Generationsliste."))
                .action_describe("updateGenerationValues", LocalizedLabel::native("Sets the value one input question takes in a generation (the selected one when no id is given) and re-evaluates the result.", "Setzt den Wert, den eine Eingabefrage in einer Generation annimmt (ohne Id in der ausgewählten), und wertet das Ergebnis neu aus."))
                .action_describe("addWidget", LocalizedLabel::native("Adds a new widget of the given kind (an input or an operator of the 3D generator graph) to the canvas.", "Fügt der Fläche ein neues Widget der angegebenen Art hinzu (eine Eingabe oder einen Operator des 3D-Generatorgraphen)."))
                .action_describe("removeWidget", LocalizedLabel::native("Removes one widget by id from the generator graph together with its connections.", "Entfernt ein Widget anhand seiner Id samt seiner Verbindungen aus dem Generatorgraphen."))
                .action_describe("reorganize", LocalizedLabel::native("Lays out every widget of the generator graph automatically from left to right, overwriting their manual positions.", "Ordnet alle Widgets des Generatorgraphen automatisch von links nach rechts an und überschreibt ihre manuellen Positionen."))
                .action_describe("setShowMode", LocalizedLabel::native("Sets what the editor shows, such as the generator graph or the generated result; only the view changes.", "Legt fest, was der Editor zeigt, etwa den Generatorgraphen oder das erzeugte Ergebnis; nur die Ansicht ändert sich."))
                .action_describe("editMeshSelection", LocalizedLabel::native("Edits the faces, edges, or vertices selected in the preview by inserting an adjustable mesh widget; downstream geometry and analysis follow the edit.", "Bearbeitet die in der Vorschau ausgewählten Flächen, Kanten oder Eckpunkte mit einem einstellbaren Netz-Widget; nachgelagerte Geometrie und Analyse folgen der Änderung."))
                .action_describe("knifeMeshSelection", LocalizedLabel::native("Cuts one selected mesh face along the line through two points. The cut remains editable and downstream measurements update.", "Schneidet eine ausgewählte Netzfläche entlang der Geraden durch zwei Punkte. Der Schnitt bleibt einstellbar und nachgelagerte Messungen werden aktualisiert."))
                .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 3D generator with one of the plugin's bundled examples, by example id.", "Ersetzt den gesamten 3D-Generator durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
                .action_describe("setLodMode", LocalizedLabel::native("Sets the level of detail the 3D preview draws the generated result with; only the view changes.", "Legt die Detailstufe fest, mit der die 3D-Vorschau das erzeugte Ergebnis zeichnet; nur die Ansicht ändert sich."))
                .action_describe("selectNextNode", LocalizedLabel::native("Moves the graph selection to the next widget in the generator graph.", "Bewegt die Auswahl im Generatorgraphen zum nächsten Widget."))
                .action_describe("selectPreviousNode", LocalizedLabel::native("Moves the graph selection to the previous widget in the generator graph.", "Bewegt die Auswahl im Generatorgraphen zum vorherigen Widget."))
                .action_describe("selectUpstreamNode", LocalizedLabel::native("Moves the graph selection to a widget feeding the selected one.", "Bewegt die Auswahl im Generatorgraphen zu einem Widget, das das ausgewählte speist."))
                .action_describe("selectDownstreamNode", LocalizedLabel::native("Moves the graph selection to a widget fed by the selected one.", "Bewegt die Auswahl im Generatorgraphen zu einem Widget, das vom ausgewählten gespeist wird."))
                .action_describe("activateSelection", LocalizedLabel::native("Opens the ports of the selected widget in the graph window, as pressing Enter on it does; only the view changes.", "Öffnet die Ports des ausgewählten Widgets im Graphfenster, wie ein Druck auf die Eingabetaste; nur die Ansicht ändert sich."))
                .action_describe("toggleSun", LocalizedLabel::native("Switches the sun light of the 3D preview on or off; only the view changes.", "Schaltet das Sonnenlicht der 3D-Vorschau ein oder aus; nur die Ansicht ändert sich."))
                .action_describe("setSunAzimuth", LocalizedLabel::native("Sets the compass direction the 3D preview's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne der 3D-Vorschau scheint; nur die Ansicht ändert sich."))
                .action_describe("setSunElevation", LocalizedLabel::native("Sets how high the 3D preview's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne der 3D-Vorschau über dem Horizont steht; nur die Ansicht ändert sich."))
                .action_describe("setSunIntensity", LocalizedLabel::native("Sets the brightness of the 3D preview's sun; only the view changes.", "Legt die Helligkeit der Sonne der 3D-Vorschau fest; nur die Ansicht ändert sich."))
                .action_describe("translateSelection", LocalizedLabel::native("Moves the given or selected generated objects by dx, dy and dz through a translate transform in the generator graph; one gumball drag is one undo step.", "Verschiebt die angegebenen oder ausgewählten erzeugten Objekte über eine Verschiebe-Transformation im Generatorgraphen um dx, dy und dz; ein Gumball-Zug ist ein Rückgängig-Schritt."))
                .action_describe("rotateSelection", LocalizedLabel::native("Rotates the given or selected generated objects around an axis by an angle through a rotate transform in the generator graph.", "Dreht die angegebenen oder ausgewählten erzeugten Objekte über eine Dreh-Transformation im Generatorgraphen um eine Achse und einen Winkel."))
                .action_describe("scaleSelection", LocalizedLabel::native("Scales the given or selected generated objects by per-axis factors through a scale transform in the generator graph.", "Skaliert die angegebenen oder ausgewählten erzeugten Objekte über eine Skalier-Transformation im Generatorgraphen um Faktoren je Achse."))
                .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected widget from the generator graph together with its connections.", "Löscht alle ausgewählten Widgets samt ihrer Verbindungen aus dem Generatorgraphen."))
                .action_describe("patchFlowWidgets", LocalizedLabel::native("Sets one numeric field (such as a slider value) on several widgets at once; one drag of the control is one undo step.", "Setzt ein Zahlenfeld (etwa einen Schiebereglerwert) auf mehreren Widgets zugleich; ein Zug des Bedienelements ist ein Rückgängig-Schritt."))
                .action_describe("importDocumentRequest", LocalizedLabel::native("Opens the host's file picker for a 3D artifact file; the chosen file is then imported as the generator document.", "Öffnet die Dateiauswahl des Hosts für eine 3D-Artefaktdatei; die gewählte Datei wird dann als Generatordokument importiert."))
                .action_describe("exportDocument", LocalizedLabel::native("Writes the generated 3D result in the chosen format to a downloaded file on the user's machine.", "Schreibt das erzeugte 3D-Ergebnis im gewählten Format in eine heruntergeladene Datei auf dem Rechner des Nutzers."))
                .action_describe("importDocument", LocalizedLabel::native("Replaces the generator document with one read from an imported 3D artifact file.", "Ersetzt das Generatordokument durch eines aus einer importierten 3D-Artefaktdatei."))
                .action_describe("cycleShowMode", LocalizedLabel::native("Switches the editor to the next show mode in turn (such as graph, result or wireframe); only the view changes.", "Wechselt den Editor reihum in den nächsten Anzeigemodus (etwa Graph, Ergebnis oder Drahtgitter); nur die Ansicht ändert sich."))
                .action_describe("cycleLodMode", LocalizedLabel::native("Switches the 3D preview to the next level of detail in turn; only the view changes.", "Wechselt die 3D-Vorschau reihum zur nächsten Detailstufe; nur die Ansicht ändert sich."))
                .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input)
                .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome)
                .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
                .action_destructive("reorganize")
                .action_destructive("importDocument")
                
}


}
//#endregion 🔖️Manifest









