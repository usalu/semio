//! ✏️ WFC 3D editor — the `ArtifactEditor` for `s.wfc.wfc3d@1/*`: two windows over one document, the
//! `wfc-graph` slot canvas (copied verbatim from `wfc2d`, so both artifacts present ONE window kind
//! to the shell) and the read-only `wfc-3d-preview` World3d pane.
//!
//! Every command maps 1:1 onto a real `Wfc3dMutation` builder from the schema tree — no synthetic
//! "set field" indirection, because the domain's own mutations are already exactly this granular.
//! The command families that are NOT document mutations (`change-camera`, `change-active-tile`) go to
//! the per-pane config instead: a camera is per-window view state, and the solve is an INFERENCE, so
//! letting either reach the document lanes would make a derived value look persisted and pollute undo.
//!
//! Collection inserts always land at the collection's CANONICAL SORTED position
//! (`canonical_*_index`), so a delete followed by its inverse puts the row back exactly where it was.

#![allow(clippy::result_large_err)]

use crate::editor::wfc3d::config::{wfc3d_active_tile_id, Wfc3dConfig, Wfc3dConfigMutation};
use crate::editor::wfc3d::modes::edit;
use crate::editor::wfc3d::modes::edit::tools::{drag, fill};
use crate::editor::wfc3d::modes::edit::windows::{graph, preview};
use crate::editor::wfc3d::transient::{Wfc3dTransient, Wfc3dTransientMutation};
use crate::mutations::{change_seed, change_tile_media, change_tile_weight, connect_slots, create_rule, create_slot, create_tile, delete_rule, delete_slot, delete_tile, disconnect_slots, move_slot, pin_slot, resize_slot, unpin_slot};
use crate::schema::snapshot::{canonical_edge_index, canonical_insertion_index, canonical_rule_index, canonical_slot_index, canonical_tile_index, GraphRule, Slot3d, SlotEdge, Tile, TileMedia3d};
use crate::{Wfc3dMutation, Wfc3dSnapshot, WFC3D_DIALECT, WFC3D_DOCUMENT_SCHEMA};
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::ToolRunJob;
use semio_framework_plugin::ToolRunJobRequest;
use semio_framework_plugin::ToolRunView;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_tool_machine::{node_graph_edit_rows, NodeDragRecord, NodeGraphEditRow};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_2d_compute::EngineHandles;

#[path = "🎛️command-vocabulary/🦀️.rs"]
mod command_vocabulary;
pub use command_vocabulary::*;

#[path = "🧵️retained-commands/🦀️.rs"]
mod retained_commands;
pub use retained_commands::*;

#[path = "🔗️graph-view-and-editor/🦀️.rs"]
mod graph_view_and_editor;
pub use graph_view_and_editor::*;

#[path = "🧰️examples-and-graph-edit/🦀️.rs"]
mod examples_and_graph_edit;
pub use examples_and_graph_edit::*;

//#region 🔖️Manifest
/// 🀄️ One app-level catalogue action. `build_definition` pushes every action NO window kind owns
/// explicitly onto EVERY window kind, which is exactly what makes these reachable from either pane
/// and what stops `undeclaredActionDiagnostic` dropping them.
fn wfc3d_action(id: &'static str, en: &'static str, de: &'static str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), kind)
}

/// 🀄️ The tile, rule and pane-state verbs the graph window does not own. `TOOL_JOB_IDS ∩
/// migrated` must EQUAL the proof rows exactly, so every retained id is declared here or on a
/// window kind — one missing row boots the app with `interactive-job.catalog-incomplete`.
/// 🕸️ The two verbs the NodeGraph canvas itself dispatches: a released drag/wire gesture and a
/// settled camera. Both are canvas vocabulary, not palette entries.
/// 🎨️ The navbar example picker dispatches `setActiveExample { exampleId }` at boot and on every
/// pick; it belongs to no window, so it is declared centrally and `build_definition`'s
/// unowned-action fallback attaches it to both window kinds.
/// 🕹️ ONE selection/hover domain over the slot nodes. Declaring it is what makes the framework
/// inject `interactionSelect`/`interactionHover`/`clearSelection` onto every window kind — the
/// shell drops an action no window kind declares, so without this domain every pointer move
/// over the graph canvas was refused `undeclared-action`.
pub fn create_wfc3d_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(WFC3D_DIALECT)
        .document(["semio", "wfc", "3d"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("network")
        .mode_def(edit::definition())
        .default_mode_id(edit::WFC_3D_EDIT_MODE_ID)
        .tool(fill::definition())
        .mode_tools(edit::WFC_3D_EDIT_MODE_ID, vec![::semio_framework_async::poll::resolve_ready(ToolRef::new(fill::TOOL_ID))])
        .window_kind_def(graph::definition())
        .window_kind_def(preview::definition())
        .default_layout(edit::layout())
        .window_kind_actions(preview::WFC_3D_PREVIEW_WINDOW, vec![wfc3d_action("solve", "Solve", "Lösen", ActionKind::Mutation)])
        .interaction(InteractionDefinition {
            id: WFC_3D_INTERACTION_GRAPH.into(),
            label: LocalizedLabel::native("Slots", "Slots"),
            granularities: vec![GranularityDefinition { id: "slot".into(), label: LocalizedLabel::native("Slot", "Slot"), icon_id: "square-dashed".into() }],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
        })
        .window_kind_interactions(graph::WFC_GRAPH_WINDOW, vec![InteractionRef::new(WFC_3D_INTERACTION_GRAPH)])
        .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
        .action_destructive("setActiveExample")
        .action_destructive("delete-slot")
        .action_args("setActiveExample", vec![
            ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                ActionArgOption::new(crate::examples::two_room_corridor::ID, crate::examples::two_room_corridor::label()),
                ActionArgOption::new(crate::examples::wall_roof_facade_strip::ID, crate::examples::wall_roof_facade_strip::label()),
                ActionArgOption::new(crate::examples::tower_stack::ID, crate::examples::tower_stack::label()),
            ])
            .required()
            .default_value(&crate::examples::two_room_corridor::ID),
        ])
        .action_interactive_job("setActiveExample", semio_framework::InteractiveJobClassification::Migrated)
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"), ActionKind::Mutation) })
        .action_interactive_job("nodeGraphEdit", semio_framework::InteractiveJobClassification::Migrated)
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(WFC_3D_NODE_GRAPH_VIEWPORT, LocalizedLabel::native("Graph Viewport", "Graph-Ansicht"), ActionKind::View) })
        .action_interactive_job(WFC_3D_NODE_GRAPH_VIEWPORT, semio_framework::InteractiveJobClassification::Migrated)
        .action_with(wfc3d_action("create-tile", "Create Tile", "Kachel erstellen", ActionKind::Mutation))
        .action_with(wfc3d_action("delete-tile", "Delete Tile", "Kachel löschen", ActionKind::Mutation))
        .action_destructive("delete-tile")
        .action_with(wfc3d_action("change-tile-weight", "Change Tile Weight", "Kachelgewicht ändern", ActionKind::Mutation))
        .action_with(wfc3d_action("change-tile-media", "Reset Tile Media", "Kachelmedium zurücksetzen", ActionKind::Mutation))
        .action_with(wfc3d_action("create-rule", "Create Rule", "Regel erstellen", ActionKind::Mutation))
        .action_with(wfc3d_action("delete-rule", "Delete Rule", "Regel löschen", ActionKind::Mutation))
        .action_destructive("delete-rule")
        .action_with(wfc3d_action("change-camera", "Change Camera", "Kamera ändern", ActionKind::View))
        .action_with(wfc3d_action("change-active-tile", "Arm Tile", "Kachel aktivieren", ActionKind::View))
        .action_interactive_job("create-tile", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("delete-tile", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("change-tile-weight", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("change-tile-media", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("create-rule", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("delete-rule", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("change-camera", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("change-active-tile", semio_framework::InteractiveJobClassification::Migrated)
        .action_interactive_job("solve", semio_framework::InteractiveJobClassification::Migrated)
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(fill::COMMIT_FILL_ACTION_ID, LocalizedLabel::native("Commit Fill", "Füllen übernehmen"), ActionKind::Mutation) })
        .action_interactive_job(fill::COMMIT_FILL_ACTION_ID, semio_framework::InteractiveJobClassification::Migrated)
        .action_args("create-tile", vec![
            ActionArgDef::text("id", LocalizedLabel::native("Id", "Id")).required(),
            ActionArgDef::text("label", LocalizedLabel::native("Label", "Bezeichnung")),
            ActionArgDef::number("weight", LocalizedLabel::native("Weight", "Gewicht")).required().default_value(&1.0),
        ])
        .action_args("delete-tile", vec![ActionArgDef::text("id", LocalizedLabel::native("Id", "Id")).required()])
        .action_args("change-tile-weight", vec![
            ActionArgDef::text("tileId", LocalizedLabel::native("Tile", "Kachel")).required(),
            ActionArgDef::number("weight", LocalizedLabel::native("Weight", "Gewicht")).required().default_value(&1.0),
        ])
        .action_args("change-tile-media", vec![ActionArgDef::text("tileId", LocalizedLabel::native("Tile", "Kachel")).required()])
        .action_args("create-rule", vec![
            ActionArgDef::text("id", LocalizedLabel::native("Id", "Id")).required(),
            ActionArgDef::text("tileAId", LocalizedLabel::native("Tile A", "Kachel A")).required(),
            ActionArgDef::text("tileBId", LocalizedLabel::native("Tile B", "Kachel B")).required(),
            ActionArgDef::text("relation", LocalizedLabel::native("Relation", "Relation")),
        ])
        .action_args("delete-rule", vec![ActionArgDef::text("id", LocalizedLabel::native("Id", "Id")).required()])
        .action_args("change-active-tile", vec![ActionArgDef::text("tileId", LocalizedLabel::native("Tile", "Kachel")).required()])
        .default_layout(edit::layout())
        .action_describe("change-seed", LocalizedLabel::native("Sets the random seed the solve starts from; the same seed, slots and rules always produce the same assignment.", "Legt den Zufallsstartwert des Lösers fest; derselbe Startwert mit denselben Slots und Regeln ergibt immer dieselbe Belegung."))
        .action_describe("solve", LocalizedLabel::native("Starts the fill tool run that assigns a tile to every slot of the 3D slot graph along its adjacency edges; the result is a preview and does not change the document.", "Startet den Füll-Werkzeuglauf, der jedem Slot des 3D-Slotgraphen entlang seiner Nachbarschaftskanten eine Kachel zuweist; das Ergebnis ist eine Vorschau und ändert das Dokument nicht."))
        .action_describe("commit-fill", LocalizedLabel::native("Receives the result of a finished fill run for the preview window; nothing is written to the document.", "Nimmt das Ergebnis eines fertigen Füll-Laufs für das Vorschaufenster entgegen; ins Dokument wird nichts geschrieben."))
        .action_describe("create-slot", LocalizedLabel::native("Adds a new slot, a 3D box the solve assigns one tile to, with the given id, position and size; refused when the id is taken.", "Fügt einen neuen Slot hinzu, ein 3D-Feld, dem der Löser eine Kachel zuweist, mit der angegebenen Id, Position und Größe; eine vergebene Id wird abgelehnt."))
        .action_describe("delete-slot", LocalizedLabel::native("Removes a slot by id together with every adjacency edge connected to it.", "Entfernt einen Slot anhand seiner Id samt aller mit ihm verbundenen Nachbarschaftskanten."))
        .action_describe("move-slot", LocalizedLabel::native("Moves a slot so its lower corner sits at the given x, y and optional z; its size and connections stay the same.", "Verschiebt einen Slot so, dass seine untere Ecke bei x, y und optional z liegt; Größe und Verbindungen bleiben erhalten."))
        .action_describe("resize-slot", LocalizedLabel::native("Sets a slot's width, height and optional depth; the preview scales the assigned tile into this box, and a zero or negative size is refused.", "Legt Breite, Höhe und optional Tiefe eines Slots fest; die Vorschau skaliert die zugewiesene Kachel in dieses Feld, eine Größe von null oder weniger wird abgelehnt."))
        .action_describe("connect-slots", LocalizedLabel::native("Adds an adjacency edge with the given id and relation between two slots, so the solve must pick tiles the rules allow side by side there.", "Fügt zwischen zwei Slots eine Nachbarschaftskante mit der angegebenen Id und Relation hinzu, sodass der Löser dort nur laut Regeln verträgliche Kacheln wählt."))
        .action_describe("disconnect-slots", LocalizedLabel::native("Removes one adjacency edge by id, so its two slots no longer constrain each other.", "Entfernt eine Nachbarschaftskante anhand ihrer Id, sodass sich ihre beiden Slots nicht mehr gegenseitig einschränken."))
        .action_describe("pin-slot", LocalizedLabel::native("Fixes one slot to the given tile, or to the armed tile when none is given; the solve must keep it.", "Legt einen Slot auf die angegebene Kachel fest, ohne Angabe auf die gewählte Kachel; der Löser muss sie beibehalten."))
        .action_describe("unpin-slot", LocalizedLabel::native("Releases the tile pin of one slot so the solve may choose any tile for it again.", "Löst die Kachelanheftung eines Slots, sodass der Löser wieder jede Kachel dafür wählen darf."))
        .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole document (slots, edges, tiles, rules and seed) with one of the plugin's bundled 3D slot-graph examples, by example id.", "Ersetzt das gesamte Dokument (Slots, Kanten, Kacheln, Regeln und Startwert) durch eines der mitgelieferten 3D-Slotgraph-Beispiele, anhand der Beispiel-Id."))
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧩️ArtifactItems
pub mod artifact_items {
pub use crate::schema::snapshot::WFC3D_DOCUMENT_SCHEMA;
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};

//#region 🔖️Dialect
/// 🪪️ The canonical surface-id coordinate for this artifact's ONE subset (`✳️any`) —
/// `s.wfc.wfc3d@1/*`. Lives at the ARTIFACT root (not under `✏️editor`/`👁️viewer`) so a viewer file
/// can read it without importing through the sibling editor module. `artifact_kind` matches
/// `Wfc3dSnapshot`/`Wfc3dDiff`'s own `#[artifact_schema(id = "s.wfc.wfc3d")]` attribute and
/// `WFC3D_DOCUMENT_SCHEMA`'s literal value; it is NOT `artifact_kind()`'s OS-level `"3d.wfc3d"`
/// kind id, which is a different, unrelated namespace.
pub const WFC3D_DIALECT: Dialect = Dialect { artifact_kind: WFC3D_DOCUMENT_SCHEMA, standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
pub use crate::schema::snapshot::WFC3D_DOCUMENT_SCHEMA;
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`. `dimension`/`media_*` follow the 3d/mesh family: the
/// editor's preview window is a `World3d` surface placing one mesh instance per solved slot.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.wfc3d".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("3D", "3D"),
        source_format: WFC3D_DOCUMENT_SCHEMA.into(),
        component_kind: "wfc3d".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        schema: WFC3D_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.txt".into()],
        import_stdio_kinds: vec!["stdio.txt".into()],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems2
