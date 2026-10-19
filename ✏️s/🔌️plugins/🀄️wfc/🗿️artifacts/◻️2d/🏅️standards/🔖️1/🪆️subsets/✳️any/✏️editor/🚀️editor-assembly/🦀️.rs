//! 🚀️ Editor assembly: the action and example argument helpers and the editor creation with its manifest stitch. See [`super`].

use super::*;

//#region 🔖️Manifest
/// 🀄️ One app-level catalogue action. `build_definition` pushes every action NO window kind owns
/// explicitly onto EVERY window kind, which is exactly what makes these reachable from either pane
/// and what stops `undeclaredActionDiagnostic` dropping them.
fn wfc2d_action(id: &'static str, en: &'static str, de: &'static str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), kind)
}

/// 🗃️ The example picker's own argument form — one option per `crate::examples::sources()` row, in
/// picker order, defaulting to the document the editor boots on.
fn wfc2d_example_arg() -> Vec<ActionArgDef> {
    vec![ActionArgDef::select(
        "exampleId",
        LocalizedLabel::native("Example", "Beispiel"),
        vec![
            ActionArgOption::new(crate::examples::two_room_corridor::ID, crate::examples::two_room_corridor::label()),
            ActionArgOption::new(crate::examples::wall_roof_facade_strip::ID, crate::examples::wall_roof_facade_strip::label()),
            ActionArgOption::new(crate::examples::hex_ring::ID, crate::examples::hex_ring::label()),
            ActionArgOption::new(crate::examples::terrain_ring::ID, crate::examples::terrain_ring::label()),
        ],
    )
    .required()
    .default_value(&crate::examples::two_room_corridor::ID)]
}

/// 📝️ Staged argument forms. An entity-naming verb with no form dispatches an EMPTY id the
/// instant its palette row is activated, which `dispatch`'s guards now refuse by name — the
/// form is what makes the row usable rather than merely honest.
/// 🕹️ Declaring ONE interaction domain is what injects the framework's own
/// `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll` verbs. Without it the
/// NodeGraph canvas' every node click is dropped as `undeclared-action`.
pub fn create_wfc2d_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(WFC_2D_DIALECT)
        .document(["semio", "wfc", "2d"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("network")
        .mode_def(edit::definition())
        .default_mode_id(edit::WFC_2D_EDIT_MODE_ID)
        .tool(fill::definition())
        .window_kind_def(graph::definition())
        .window_kind_def(preview::definition())
        .default_layout(edit::layout())
        .window_kind_actions(preview::WFC_2D_PREVIEW_WINDOW, vec![wfc2d_action("solve", "Solve", "Lösen", ActionKind::Mutation)])
        .interaction(InteractionDefinition {
            id: "slot".into(),
            label: LocalizedLabel::native("Slots", "Slots"),
            granularities: vec![GranularityDefinition { id: "slot".into(), label: LocalizedLabel::native("Slot", "Slot"), icon_id: "square-dashed".into() }],
            hierarchy: HierarchyProvider::Flat,
            hover: HoverSpec::default(),
            selection: SelectionSpec { modes: vec![SelectionMode::Single, SelectionMode::Multiple], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: false },
        })
        .window_kind_interactions(graph::WFC_GRAPH_WINDOW, vec![InteractionRef::new("slot")])
        .action_with(ActionDefinition::new(WFC_2D_SET_ACTIVE_EXAMPLE, LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
        .action_args(WFC_2D_SET_ACTIVE_EXAMPLE, wfc2d_example_arg())
        .action_destructive("delete-slot")
        .action_destructive(WFC_2D_SET_ACTIVE_EXAMPLE)
        .action_with(wfc2d_action("create-tile", "Create Tile", "Kachel erstellen", ActionKind::Mutation))
        .action_with(wfc2d_action("delete-tile", "Delete Tile", "Kachel löschen", ActionKind::Mutation))
        .action_destructive("delete-tile")
        .action_with(wfc2d_action("change-tile-weight", "Change Tile Weight", "Kachelgewicht ändern", ActionKind::Mutation))
        .action_with(wfc2d_action("change-tile-media", "Clear Tile Media", "Kachelmedium leeren", ActionKind::Mutation))
        .action_with(wfc2d_action("create-rule", "Create Rule", "Regel erstellen", ActionKind::Mutation))
        .action_with(wfc2d_action("delete-rule", "Delete Rule", "Regel löschen", ActionKind::Mutation))
        .action_destructive("delete-rule")
        .action_with(wfc2d_action("change-camera", "Change Camera", "Kamera ändern", ActionKind::View))
        .action_with(wfc2d_action("change-active-tile", "Arm Tile", "Kachel aktivieren", ActionKind::View))
        .action_with(wfc2d_action(WFC_2D_NODE_GRAPH_EDIT, "Edit Graph", "Graph bearbeiten", ActionKind::Mutation))
        .action_with(ActionDefinition { in_palette: false, ..wfc2d_action(fill::COMMIT_FILL_ACTION_ID, "Commit Fill", "Füllen übernehmen", ActionKind::Mutation) })
        .action_args("create-tile", vec![
            ActionArgDef::text("id", LocalizedLabel::native("Tile", "Kachel")).required(),
            ActionArgDef::text("label", LocalizedLabel::native("Label", "Bezeichnung")),
            ActionArgDef::number("weight", LocalizedLabel::native("Weight", "Gewicht")).default_value(&1.0),
        ])
        .action_args("delete-tile", vec![ActionArgDef::text("id", LocalizedLabel::native("Tile", "Kachel")).required()])
        .action_args("change-tile-weight", vec![
            ActionArgDef::text("tileId", LocalizedLabel::native("Tile", "Kachel")).required(),
            ActionArgDef::number("weight", LocalizedLabel::native("Weight", "Gewicht")).default_value(&1.0),
        ])
        .action_args("change-tile-media", vec![ActionArgDef::text("tileId", LocalizedLabel::native("Tile", "Kachel")).required()])
        .action_args("create-rule", vec![
            ActionArgDef::text("id", LocalizedLabel::native("Rule", "Regel")).required(),
            ActionArgDef::text("tileAId", LocalizedLabel::native("Tile A", "Kachel A")).required(),
            ActionArgDef::text("tileBId", LocalizedLabel::native("Tile B", "Kachel B")).required(),
            ActionArgDef::text("relation", LocalizedLabel::native("Relation", "Relation")),
            ActionArgDef::toggle("allowed", LocalizedLabel::native("Allowed", "Erlaubt")).default_value(&true),
        ])
        .action_args("delete-rule", vec![ActionArgDef::text("id", LocalizedLabel::native("Rule", "Regel")).required()])
        .action_args("change-active-tile", vec![ActionArgDef::text("tileId", LocalizedLabel::native("Tile", "Kachel")).required()])
        .interactive_jobs(InteractiveJobClassification::Migrated)
        .action_describe("change-seed", LocalizedLabel::native("Sets the random seed the solve starts from; the same seed, slots and rules always produce the same assignment.", "Legt den Zufallsstartwert des Lösers fest; derselbe Startwert mit denselben Slots und Regeln ergibt immer dieselbe Belegung."))
        .action_describe("solve", LocalizedLabel::native("Starts the fill tool run that assigns a tile to every slot of the 2D slot graph along its adjacency edges; the result is a preview and does not change the document.", "Startet den Füll-Werkzeuglauf, der jedem Slot des 2D-Slotgraphen entlang seiner Nachbarschaftskanten eine Kachel zuweist; das Ergebnis ist eine Vorschau und ändert das Dokument nicht."))
        .action_describe("commit-fill", LocalizedLabel::native("Receives the result of a finished fill run for the preview window; nothing is written to the document.", "Nimmt das Ergebnis eines fertigen Füll-Laufs für das Vorschaufenster entgegen; ins Dokument wird nichts geschrieben."))
        .action_describe("create-slot", LocalizedLabel::native("Adds a new slot, a 2D box the solve assigns one tile to, with the given id, position and size; refused when the id is taken.", "Fügt einen neuen Slot hinzu, ein 2D-Feld, dem der Löser eine Kachel zuweist, mit der angegebenen Id, Position und Größe; eine vergebene Id wird abgelehnt."))
        .action_describe("delete-slot", LocalizedLabel::native("Removes a slot by id together with every adjacency edge connected to it.", "Entfernt einen Slot anhand seiner Id samt aller mit ihm verbundenen Nachbarschaftskanten."))
        .action_describe("move-slot", LocalizedLabel::native("Moves a slot so its lower corner sits at the given x, y; its size and connections stay the same.", "Verschiebt einen Slot so, dass seine untere Ecke bei x, y liegt; Größe und Verbindungen bleiben erhalten."))
        .action_describe("resize-slot", LocalizedLabel::native("Sets a slot's width and height; the preview scales the assigned tile into this box, and a zero or negative size is refused.", "Legt Breite und Höhe eines Slots fest; die Vorschau skaliert die zugewiesene Kachel in dieses Feld, eine Größe von null oder weniger wird abgelehnt."))
        .action_describe("connect-slots", LocalizedLabel::native("Adds an adjacency edge with the given id and relation between two slots, so the solve must pick tiles the rules allow side by side there.", "Fügt zwischen zwei Slots eine Nachbarschaftskante mit der angegebenen Id und Relation hinzu, sodass der Löser dort nur laut Regeln verträgliche Kacheln wählt."))
        .action_describe("disconnect-slots", LocalizedLabel::native("Removes one adjacency edge by id, so its two slots no longer constrain each other.", "Entfernt eine Nachbarschaftskante anhand ihrer Id, sodass sich ihre beiden Slots nicht mehr gegenseitig einschränken."))
        .action_describe("pin-slot", LocalizedLabel::native("Fixes one slot to the given tile, or to the armed tile when none is given; the solve must keep it.", "Legt einen Slot auf die angegebene Kachel fest, ohne Angabe auf die gewählte Kachel; der Löser muss sie beibehalten."))
        .action_describe("unpin-slot", LocalizedLabel::native("Releases the tile pin of one slot so the solve may choose any tile for it again.", "Löst die Kachelanheftung eines Slots, sodass der Löser wieder jede Kachel dafür wählen darf."))
        .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole document (slots, edges, tiles, rules and seed) with one of the plugin's bundled 2D slot-graph examples, by example id.", "Ersetzt das gesamte Dokument (Slots, Kanten, Kacheln, Regeln und Startwert) durch eines der mitgelieferten 2D-Slotgraph-Beispiele, anhand der Beispiel-Id."))
        .action_describe("create-tile", LocalizedLabel::native("Adds a new placeable tile with the given id, label and weight to the tile catalogue; refused when the id is taken.", "Fügt dem Kachelkatalog eine neue setzbare Kachel mit der angegebenen Id, Bezeichnung und Gewichtung hinzu; eine vergebene Id wird abgelehnt."))
        .action_describe("delete-tile", LocalizedLabel::native("Removes a tile from the catalogue together with every rule that names it and every slot pinned to it.", "Entfernt eine Kachel aus dem Katalog samt aller Regeln, die sie nennen, und aller Slots, die auf sie angeheftet sind."))
        .action_describe("change-tile-weight", LocalizedLabel::native("Sets how strongly the solve favours one tile when several fit a slot; a higher weight makes it more frequent.", "Legt fest, wie stark der Löser eine Kachel bevorzugt, wenn mehrere in einen Slot passen; ein höheres Gewicht macht sie häufiger."))
        .action_describe("change-tile-media", LocalizedLabel::native("Clears the look of one tile back to the plain default box; its id, weight and rules stay unchanged.", "Setzt das Aussehen einer Kachel auf den schlichten Standardkörper zurück; Id, Gewicht und Regeln bleiben unverändert."))
        .action_describe("create-rule", LocalizedLabel::native("Allows or forbids tile B next to tile A under a named relation; along each edge the solve only places tile pairs the rules allow.", "Erlaubt oder verbietet Kachel B neben Kachel A unter einer benannten Relation; entlang jeder Kante setzt der Löser nur erlaubte Kachelpaare."))
        .action_describe("delete-rule", LocalizedLabel::native("Removes one adjacency rule by id, withdrawing the permission or ban it stated.", "Entfernt eine Nachbarschaftsregel anhand ihrer Id und nimmt die Erlaubnis oder das Verbot zurück, das sie ausdrückte."))
        .action_describe("change-active-tile", LocalizedLabel::native("Arms the tile that a pin gesture in this window assigns; only the editor's view state changes, not the document.", "Wählt die Kachel, die eine Anheften-Geste in diesem Fenster zuweist; nur der Ansichtszustand des Editors ändert sich, nicht das Dokument."))
        .action_audience("change-camera", semio_framework_plugin::CapabilityAudience::Chrome)
        .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input)
        .build_definition()
}
//#endregion 🔖️Manifest
