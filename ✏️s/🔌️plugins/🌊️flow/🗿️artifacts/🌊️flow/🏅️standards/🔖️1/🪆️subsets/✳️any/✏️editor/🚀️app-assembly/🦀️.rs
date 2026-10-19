//! 🚀️ App assembly: the app creation with its manifest stitch. See [`super`].

use super::*;

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_flow_app() -> AppDefinition {
    Editor::builder(crate::FLOW_DIALECT)
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), "runtime", ActionKind::View) })
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalResolve", LocalizedLabel::native("Resolve Flow Evaluation", "Flow-Auswertung auflösen"), "runtime", ActionKind::View) })
        .command(CommandDefinition {
            in_palette: false,
            ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([
                ActionArgDef::text("json", LocalizedLabel::native("Contributions Page", "Beiträge-Seite")),
                ActionArgDef::text("page", LocalizedLabel::native("Page", "Seite")),
                ActionArgDef::text("pageCount", LocalizedLabel::native("Page Count", "Seitenanzahl")),
            ])
        })
        .document(["semio", "flow"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("flow")
        .mode_def(edit::definition())
        .mode_def(generate::definition())
        .default_mode_id(edit::FLOW_PLAY_MODE_EDIT)
        .window_kind_def(main::definition())
        .window_kind_def(compiled::definition())
        .window_kind_def(generations::definition())
        .window_kind_def(form::definition())
        .window_kind_def(preview::definition())
        .default_layout(edit::layout())
        .named_layout(generate::layout())
        .panel_tab_def(document_panel::definition())
        .panel_tab_def(catalogue_panel::definition())
        .panel_tab_def(inspection_panel::definition())
        .mutation("addWidget", LocalizedLabel::native("Add Widget", "Widget hinzufügen"))
        .mutation("removeWidget", LocalizedLabel::native("Remove Widget", "Widget entfernen"))
        .action_destructive("removeWidget")
        .mutation("duplicateWidget", LocalizedLabel::native("Duplicate Widget", "Widget duplizieren"))
        .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
        .action_destructive("deleteSelection")
        .mutation("disconnect", LocalizedLabel::native("Disconnect", "Trennen"))
        .mutation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Anschlüsse verbinden"))
        .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
        .action_with(ActionDefinition::new("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "rotate-cw").with_category("transform"))
        .mutation("patchFlowWidgets", LocalizedLabel::native("Patch Widgets", "Widgets aktualisieren"))
        .mutation("renameFlowWidget", LocalizedLabel::native("Rename Widget", "Widget umbenennen"))
        .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
        .mutation("spotlightCommit", LocalizedLabel::native("Spotlight Commit", "Spotlight bestätigen"))
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("runExtensionAction", LocalizedLabel::native("Run Extension Action", "Erweiterungsaktion ausführen"), ActionKind::Mutation) })
        .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Beispiel setzen"))
        .action_with(ActionDefinition::new("evaluate", LocalizedLabel::native("Evaluate", "Auswerten"), ActionKind::View, "hash"))
        .action_with(ActionDefinition::bounded_catalog("focusSelection", LocalizedLabel::native("Zoom to Selection", "Auf Auswahl zoomen"), ActionKind::View).with_category("view"))
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"), ActionKind::View, "camera") })
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"), ActionKind::View, "layers") })
        .action_with(flow_internal_action("setProximityDistance", LocalizedLabel::native("Set Proximity Distance", "Näheabstand festlegen"), ActionKind::View))
        .action_with(flow_internal_action("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster sichtbar"), ActionKind::View))
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View, "grid-3x3") })
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View, "grid-3x3") })
        .action_with(flow_internal_action("contextMenuAt", LocalizedLabel::native("Context Menu At", "Kontextmenü an Position"), ActionKind::View))
        .action_with(flow_internal_action("setPreviewOff", LocalizedLabel::native("Set Preview Off", "Vorschau deaktivieren"), ActionKind::View).with_category("view"))
        .action_with(flow_internal_action("openSpotlight", LocalizedLabel::native("Open Spotlight", "Spotlight öffnen"), ActionKind::View).with_category("create"))
        .action_with(flow_internal_action("replaceImage", LocalizedLabel::native("Replace Image", "Bild ersetzen"), ActionKind::View).with_category("actions"))
        .action_with(flow_internal_action("setCatalogueSections", LocalizedLabel::native("Set Catalogue Sections", "Katalogabschnitte festlegen"), ActionKind::View))
        .action_with(flow_internal_action("toggleExtension", LocalizedLabel::native("Toggle Extension", "Erweiterung umschalten"), ActionKind::View))
        .action_args("addWidget", vec![ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
            ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
            ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
        ])
        .default_value(&"inputSlider")])
        .action_args("removeWidget", vec![ActionArgDef::text("widgetId", LocalizedLabel::native("Widget Id", "Widget-ID"))])
        .action_args("duplicateWidget", vec![ActionArgDef::text("widgetId", LocalizedLabel::native("Widget Id", "Widget-ID"))])
        .action_args("disconnect", vec![ActionArgDef::text("synapseId", LocalizedLabel::native("Synapse Id", "Synapsen-ID"))])
        .action_args(
            "connectMediaPorts",
            vec![
                ActionArgDef::text("sourceNodeId", LocalizedLabel::native("Source Widget Id", "Quell-Widget-ID")),
                ActionArgDef::text("sourcePortId", LocalizedLabel::native("Source Port", "Quellanschluss")),
                ActionArgDef::text("targetNodeId", LocalizedLabel::native("Target Widget Id", "Ziel-Widget-ID")),
                ActionArgDef::text("targetPortId", LocalizedLabel::native("Target Port", "Zielanschluss")),
            ],
        )
        .action_args("moveMediaNode", vec![ActionArgDef::text("nodeId", LocalizedLabel::native("Widget Id", "Widget-ID")), ActionArgDef::number("x", LocalizedLabel::native("X", "X")), ActionArgDef::number("y", LocalizedLabel::native("Y", "Y"))])
        .action_args(
            "patchFlowWidgets",
            vec![
                ActionArgDef::text_list("widgetIds", LocalizedLabel::native("Widget Ids", "Widget-IDs")),
                ActionArgDef::select("field", LocalizedLabel::native("Field", "Feld"), vec![ActionArgOption::new("value", LocalizedLabel::native("Slider Value", "Reglerwert")), ActionArgOption::new("text", LocalizedLabel::native("Note Text", "Notiztext"))]),
                ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")),
            ],
        )
        .action_args("renameFlowWidget", vec![ActionArgDef::text("oldId", LocalizedLabel::native("Widget Id", "Widget-ID")), ActionArgDef::text("value", LocalizedLabel::native("New Id", "Neue ID"))])
        .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new(crate::examples::demo::ID, LocalizedLabel::native("Demo", "Demo"))])])
        .action_interactive_job("addWidget", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("removeWidget", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("duplicateWidget", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("disconnect", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("connectMediaPorts", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("moveMediaNode", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("reorganize", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("patchFlowWidgets", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("renameFlowWidget", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphEdit", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("spotlightCommit", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("runExtensionAction", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_destructive("setActiveExample")
        .action_interactive_job("evaluate", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("focusSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphViewport", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setLodMode", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setProximityDistance", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setGridVisible", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setGridSnapEnabled", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setGridFactor", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("contextMenuAt", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setPreviewOff", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("openSpotlight", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("replaceImage", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setCatalogueSections", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("toggleExtension", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("addGeneration", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("removeGeneration", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_destructive("removeGeneration")
        .action_interactive_job("selectGeneration", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("renameGeneration", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("updateGenerationValues", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("flowEvalTick", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("flowEvalResolve", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setContributions", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .keybinding("mod+z", "undo")
        .keybinding("mod+shift+z", "redo")
        .keybinding("delete,backspace", "deleteSelection")
        .interaction(InteractionDefinition {
            id: FLOW_INTERACTION_GRAPH.into(),
            label: LocalizedLabel::native("Graph", "Graph"),
            granularities: vec![
                GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
                GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "spline".into() },
                GranularityDefinition { id: "handle".into(), label: LocalizedLabel::native("Handle", "Anfasser"), icon_id: "move".into() },
            ],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: SelectionSpec {
                modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                transitive: false,
                broadcast: true,
            },
        })
        .window_kind_interactions(FLOW_PLAY_WINDOW_MAIN, vec![InteractionRef::new(FLOW_INTERACTION_GRAPH)])
        .config(FlowPlayApp::config_spec())
        .action_describe("addWidget", LocalizedLabel::native("Adds a new widget of the given kind (an input such as a slider or note, or an operator, optionally a specific neuron kind) to the flow canvas at x, y.", "Fügt der Flow-Fläche an x, y ein neues Widget der angegebenen Art hinzu (eine Eingabe wie Schieberegler oder Notiz oder einen Operator, optional eine bestimmte Neuronart)."))
        .action_describe("removeWidget", LocalizedLabel::native("Removes one widget by id from the flow graph together with every synapse attached to it.", "Entfernt ein Widget anhand seiner Id samt aller angeschlossenen Synapsen aus dem Flow-Graphen."))
        .action_describe("duplicateWidget", LocalizedLabel::native("Copies one widget by id into a new widget next to it, connected from the original, as one undoable edit; an unknown id is refused.", "Kopiert ein Widget anhand seiner Id in ein neues, vom Original aus verbundenes Widget daneben, als eine rückgängig machbare Änderung; eine unbekannte Id wird abgelehnt."))
        .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected widget and synapse from the flow graph.", "Löscht alle ausgewählten Widgets und Synapsen aus dem Flow-Graphen."))
        .action_describe("disconnect", LocalizedLabel::native("Removes one synapse (a connection between two ports) by id from the flow graph.", "Entfernt eine Synapse (eine Verbindung zwischen zwei Ports) anhand ihrer Id aus dem Flow-Graphen."))
        .action_describe("connectMediaPorts", LocalizedLabel::native("Connects an output port of one widget to an input port of another with a new synapse; an unknown widget or an incompatible connection is refused.", "Verbindet einen Ausgangsport eines Widgets mit einem Eingangsport eines anderen durch eine neue Synapse; ein unbekanntes Widget oder eine unverträgliche Verbindung wird abgelehnt."))
        .action_describe("moveMediaNode", LocalizedLabel::native("Moves one widget to the canvas position x, y; consecutive moves of the same widget merge into one edit.", "Verschiebt ein Widget an die Position x, y der Fläche; aufeinanderfolgende Verschiebungen desselben Widgets werden zu einer Änderung zusammengefasst."))
        .action_describe("reorganize", LocalizedLabel::native("Lays out every widget of the flow graph automatically from left to right, overwriting their manual positions.", "Ordnet alle Widgets des Flow-Graphen automatisch von links nach rechts an und überschreibt ihre manuellen Positionen."))
        .action_describe("patchFlowWidgets", LocalizedLabel::native("Sets one field on several widgets at once: a slider's value or a note's text.", "Setzt ein Feld auf mehreren Widgets zugleich: den Wert eines Schiebereglers oder den Text einer Notiz."))
        .action_describe("renameFlowWidget", LocalizedLabel::native("Renames a widget's id from the old id to a new one and updates every synapse that referenced it; a taken or empty id changes nothing.", "Benennt die Id eines Widgets von der alten in eine neue um und aktualisiert alle Synapsen, die darauf verweisen; eine vergebene oder leere Id ändert nichts."))
        .action_describe("runExtensionAction", LocalizedLabel::native("Runs one automation of an installed flow extension, such as auto-layout (Reorganize) or auto-evaluate (Evaluate), when that automation is enabled.", "Führt eine Automatisierung einer installierten Flow-Erweiterung aus, etwa automatisches Anordnen (Neu anordnen) oder automatisches Auswerten (Auswerten), sofern sie aktiviert ist."))
        .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole flow graph with the bundled demo graph, or with an empty graph for an empty id; other ids change nothing.", "Ersetzt den gesamten Flow-Graphen durch den mitgelieferten Demo-Graphen, bei leerer Id durch einen leeren Graphen; andere Ids ändern nichts."))
        .action_describe("evaluate", LocalizedLabel::native("Evaluates every widget whose result is not computed yet and shows the results on the canvas; the graph itself is not changed.", "Wertet alle Widgets aus, deren Ergebnis noch nicht berechnet ist, und zeigt die Ergebnisse auf der Fläche; der Graph selbst ändert sich nicht."))
        .action_describe("focusSelection", LocalizedLabel::native("Zooms the main window onto the selected widgets; only the view changes.", "Zoomt das Hauptfenster auf die ausgewählten Widgets; nur die Ansicht ändert sich."))
        .action_describe("addGeneration", LocalizedLabel::native("Adds a new parameter generation, a named set of input values, to the generate mode's list; generations live in the window, not in the document.", "Fügt der Liste des Generieren-Modus eine neue Parametergeneration hinzu, einen benannten Satz von Eingabewerten; Generationen liegen im Fenster, nicht im Dokument."))
        .action_describe("removeGeneration", LocalizedLabel::native("Removes one parameter generation by id from the generate mode's list; the flow graph is not changed.", "Entfernt eine Parametergeneration anhand ihrer Id aus der Liste des Generieren-Modus; der Flow-Graph ändert sich nicht."))
        .action_describe("renameGeneration", LocalizedLabel::native("Renames one parameter generation of the generate mode's list.", "Benennt eine Parametergeneration in der Liste des Generieren-Modus um."))
        .action_describe("updateGenerationValues", LocalizedLabel::native("Sets the value one input question takes in a parameter generation (the selected one when no id is given) and re-evaluates its preview.", "Setzt den Wert, den eine Eingabefrage in einer Parametergeneration annimmt (ohne Id in der ausgewählten), und wertet ihre Vorschau neu aus."))
        .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input)
        .action_audience("spotlightCommit", semio_framework_plugin::CapabilityAudience::Input)
        .action_destructive("reorganize")
        .build_definition()
}
//#endregion 🔖️Manifest
