//! 📜️ Viewer manifest: the runtime command, the example options and the viewer constructor with its manifest stitch. See [`super`].

use super::*;

//#region 🔖️Manifest
/// ⏱️ One hidden runtime command of the evaluation chain. Never in the palette, never a `View`
/// action a window declares: a user does not dispatch a tick, the app does.
fn generation3d_view_runtime_command(id: &str, label: LocalizedLabel, args: &[&str]) -> semio_framework_plugin::CommandDefinition {
    let mut definition = semio_framework_plugin::CommandDefinition {
        in_palette: false,
        ..semio_framework_plugin::CommandDefinition::bounded_catalog(id, label, "runtime", ActionKind::View).with_args(args.iter().map(|arg| semio_framework_plugin::ActionArgDef::text(*arg, LocalizedLabel::native(*arg, *arg))).collect::<Vec<_>>())
    };
    definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
    definition
}

/// 🎨️ The bundled examples this dialect offers, as the picker's declared argument options — the
/// SAME eight ids `editor::generation3d::examples()` registers onto the manifest, named through the
/// surface-neutral `🧬️schema` constants so this read-only surface never reaches into `::editor::`
/// (`policyViewerPurityBreaches`).
fn generation3d_view_example_options() -> Vec<semio_framework_plugin::ActionArgOption> {
    use semio_s_artifact_procedural_3d_model as generation3d_schema;
    use semio_framework_plugin::ActionArgOption;
    vec![
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_HEX_COLUMN, LocalizedLabel::native("Hexagonal Mushroom Column", "Sechseckige Pilzsäule")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_MESH_WORKBENCH, LocalizedLabel::native("Mesh Workbench", "Netzwerkstatt")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE, LocalizedLabel::native("Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS, LocalizedLabel::native("Sphere Cut With Torus", "Kugel mit Torus geschnitten")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_BOX_FILLET, LocalizedLabel::native("Box Fillet Preview", "Kantenrundung Vorschau")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, LocalizedLabel::native("Sphere Box Fuse", "Kugel und Quader vereinen")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, LocalizedLabel::native("Face Sweep Extrude", "Fläche extrudieren")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, LocalizedLabel::native("Rectangle Wire Preview", "Rechteck-Draht Vorschau")),
        ActionArgOption::new(generation3d_schema::PROCEDURAL_EXAMPLE_BOX_SHELL, LocalizedLabel::native("Box Shell Preview", "Hohlkörper Vorschau")),
    ]
}

pub fn create_generation3d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(GENERATION3D_DIALECT)
        .document(["semio", "procedural", "3d"])
        .icon_id("workflow")
        .mode_def(view::definition())
        .default_mode_id(view::GENERATION3D_VIEW_MODE_VIEW)
        .window_kind_def(preview::definition())
        .default_layout(view::layout())
        // 👁️ Ephemeral view actions — the only kind a read-only surface may declare. Every one is
        // config-only; none reaches the document.
        .action_with(ActionDefinition::new("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"), ActionKind::View, "eye"))
        .action_with(ActionDefinition::new("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"), ActionKind::View, "layers"))
        .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
        .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
        .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
        .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
        .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
        // 🎨️ The navbar example picker's verb. `ActionKind::View`, not `Mutation` like the sibling
        // surface's: loading an example into a READ-ONLY surface writes this surface's config, never
        // the document — and `ShellHost` refuses a `mutation`-kind action on a viewer session outright
        // (`🏛️ShellHost/🟦️.tsx`'s read-only gate), so a `Mutation` row here would show the picker and
        // then silently swallow every pick.
        .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::View, "panel-left"))
        .action_interactive_job("setShowMode", InteractiveJobClassification::Migrated)
        .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
        .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
        .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
        .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
        .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
        .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
        .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
        .action_args("setActiveExample", vec![semio_framework_plugin::ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), generation3d_view_example_options()).required()])
        // 📤️ The reader's own io verb. `ActionKind::View` for the same reason `setActiveExample` is:
        // `ShellHost` refuses a `Mutation`-kind action on a viewer session outright, and an export
        // really does mutate nothing — it hands the shell a `DownloadMediaExport` and returns.
        .action_with(ActionDefinition::new("exportDocument", LocalizedLabel::native("Export Artifact", "Artefakt exportieren"), ActionKind::View, "download"))
        .action_interactive_job("exportDocument", InteractiveJobClassification::Migrated)
        .action_destructive("exportDocument")
        .action_args("exportDocument", vec![semio_framework_plugin::ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), semio_s_artifact_procedural_3d_io::document_io::export_format_options()).required().default_value(&"stl")])
        .keybinding("mod+shift+e", "exportDocument")
        // ⏯️ The read-only preview evaluation run: declared so the framework owns its lifecycle and
        // injects its reserved actions, referenced by no mode because the preview surface starts it.
        .tool(preview_eval::preview_eval_tool_definition())
        // 🧩️ Not a view action: a hidden host COMMAND, because the host pushes it and no user ever
        // invokes it. It publishes nothing at all, not even the config lane the seven above write.
        .command({
            let mut definition = semio_framework_plugin::CommandDefinition {
                in_palette: false,
                ..semio_framework_plugin::CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([
                    semio_framework_plugin::ActionArgDef::text("json", LocalizedLabel::native("Contributions Page", "Beiträge-Seite")),
                    semio_framework_plugin::ActionArgDef::text("page", LocalizedLabel::native("Page", "Seite")),
                    semio_framework_plugin::ActionArgDef::text("pageCount", LocalizedLabel::native("Page Count", "Seitenanzahl")),
                ])
            };
            definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
            definition
        })
        // ⏱️ The runtime evaluation chain — three hidden host COMMANDS, never user actions: the app
        // self-dispatches them through `Effect::DispatchAction`, and `reactor::extension_response_args`
        // addresses the two resolves. A command the manifest does not declare is dropped by
        // `ShellHost`'s own `declaredAction` gate before `plugin.handleAction` is ever called, so an
        // undeclared tick would stall the whole chain silently
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        .command(generation3d_view_runtime_command("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), &["windowId", "windowKindId"]))
        .command(generation3d_view_runtime_command("flowEvalResolve", LocalizedLabel::native("Resolve Flow Evaluation", "Flow-Auswertung auflösen"), &["windowId", "windowKindId", "nodeHash", "output"]))
        .command(generation3d_view_runtime_command("flowTessellateResolve", LocalizedLabel::native("Resolve Tessellation", "Tessellierung auflösen"), &["windowId", "windowKindId", "nodeHash", "output"]))
        .command(generation3d_view_runtime_command("flowEvalRelease", LocalizedLabel::native("Release Preview Kernel Work", "Vorschau-Kernelarbeit freigeben"), &["windowId", "windowKindId"]))
        .command(generation3d_view_runtime_command("flowTessellateCancelResolve", LocalizedLabel::native("Resolve Preview Cancellation", "Vorschauabbruch aufnehmen"), &["windowId", "windowKindId", "output", "ok"]))
        // 🕹️ First-class hover/selection over the same flow-graph widget DAG the sibling surface
        // declares — read-only, but a viewer still hovers, selects and inspects. Selection stays
        // `broadcast: true` so a co-viewer sees what this one is looking at.
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
        .window_kind_interactions(preview::WINDOW_KIND_ID, vec![InteractionRef::new("graph")])
        // 📇️ Window-scoped action ownership (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). This viewer has
        // exactly one window, and that window's own chrome dispatches every verb it declares —
        // `setShowMode`/`setLodMode` and the sun group from `preview_window_measures`, `setCamera`
        // from the world host's viewport gesture. Stopping the evaluation is the framework's
        // `toolRunAbort`, injected with the `previewEval` run. Declaring them explicitly is what keeps
        // `WindowKindDefinition.actions` a statement about this window rather than a copy of the app list
        // (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5334-5338`). Asserted by
        // `every_emitted_action_is_declared_on_the_preview_window_kind`.
        .window_kind_action_refs(preview::WINDOW_KIND_ID, vec![
            "setShowMode".into(),
            "setLodMode".into(),
            "setCamera".into(),
            "toggleSun".into(),
            "setSunAzimuth".into(),
            "setSunElevation".into(),
            "setSunIntensity".into(),
        ])
        .action_describe("setShowMode", LocalizedLabel::native("Sets what the 3D viewer shows of the generated result; only the view changes.", "Legt fest, was der 3D-Betrachter vom erzeugten Ergebnis zeigt; nur die Ansicht ändert sich."))
        .action_describe("setLodMode", LocalizedLabel::native("Sets the level of detail the 3D viewer draws with; only the view changes.", "Legt die Detailstufe fest, mit der der 3D-Betrachter zeichnet; nur die Ansicht ändert sich."))
        .action_describe("toggleSun", LocalizedLabel::native("Switches the 3D viewer's sun light on or off; only the view changes.", "Schaltet das Sonnenlicht des 3D-Betrachters ein oder aus; nur die Ansicht ändert sich."))
        .action_describe("setSunAzimuth", LocalizedLabel::native("Sets the compass direction the 3D viewer's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne des 3D-Betrachters scheint; nur die Ansicht ändert sich."))
        .action_describe("setSunElevation", LocalizedLabel::native("Sets how high the 3D viewer's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne des 3D-Betrachters über dem Horizont steht; nur die Ansicht ändert sich."))
        .action_describe("setSunIntensity", LocalizedLabel::native("Sets the brightness of the 3D viewer's sun; only the view changes.", "Legt die Helligkeit der Sonne des 3D-Betrachters fest; nur die Ansicht ändert sich."))
        .action_describe("setActiveExample", LocalizedLabel::native("Shows one of the bundled generator examples in this viewer instead of the document; the document itself is not changed.", "Zeigt in diesem Betrachter eines der mitgelieferten Generatorbeispiele statt des Dokuments; das Dokument selbst ändert sich nicht."))
        .action_describe("exportDocument", LocalizedLabel::native("Writes the 3D result shown in the viewer in the chosen format to a downloaded file on the user's machine.", "Schreibt das im Betrachter gezeigte 3D-Ergebnis im gewählten Format in eine heruntergeladene Datei auf dem Rechner des Nutzers."))
        .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
        .build_definition()
}
//#endregion 🔖️Manifest
