//! 🧭️ Manifest of the CAD play app: the Dislocate utility, the interaction definition and the app assembly. See [`super`].

use super::*;
use semio_s_artifact_cad_cad_kit::CAD_DISLOCATE_UTILITY_ID;
use semio_s_artifact_cad_cad_kit::CAD_INTERACTION_DOMAIN;
use semio_s_artifact_cad_cad_kit::cad_reference_flag_action;
use semio_s_artifact_cad_cad_kit::cad_io;

//#region 🔖️Manifest
/// 🧰️ The window-scoped CAD Dislocate utility, whose Move and Rotate handles are utility options.
pub fn cad_dislocate_utility() -> UtilityDefinition {
    UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new(CAD_DISLOCATE_UTILITY_ID, LocalizedLabel::native("Dislocate", "Versetzen"), "move-3d") }
}

/// 🧰️ The single Dislocate utility ref exposed independently by each world-3d window.
pub fn cad_dislocate_utility_refs() -> Vec<semio_framework_plugin::UtilityRef> {
    vec![CAD_DISLOCATE_UTILITY_ID.into()]
}

/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): the `"cad"` mesh interaction domain —
/// whole objects (`"object"`, the default granularity) plus component-level vertex/edge/face
/// picking, all `u32` ids stringified at the `InteractionTarget` boundary (round-tripped back to
/// `u32` inside command handlers, e.g. `🎮️commands/🔄️transform`). CAUTION: NOT the same thing as
/// `crate::standards::v1::subsets::any::io::InteractionSpec` (a CAD-artifact DSL
/// type for engagement statecharts, `🗿️artifacts/📐️cad/…/🎬️interaction-spec/🦀️.rs`) —
/// unrelated, pre-existing, untouched by this migration.
pub fn cad_interaction_definition() -> semio_framework_plugin::InteractionDefinition {
    use semio_framework_plugin::{GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, MergeMode, SelectionMethod, SelectionMode, SelectionSpec};
    InteractionDefinition {
        id: CAD_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Mesh", "Netz"),
        granularities: vec![
            GranularityDefinition { id: "object".into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "box".into() },
            GranularityDefinition { id: "vertex".into(), label: LocalizedLabel::native("Vertex", "Eckpunkt"), icon_id: "circle-dot".into() },
            GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
            GranularityDefinition { id: "face".into(), label: LocalizedLabel::native("Face", "Fläche"), icon_id: "square".into() },
        ],
        hierarchy: HierarchyProvider::Flat,
        hover: HoverSpec::default(),
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
            transitive: false,
            broadcast: true,
        },
    }
}

pub fn create_cad_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(semio_s_artifact_cad_cad_model::snapshot::CAD_DIALECT).document(["semio", "cad"])
            .command({
                let mut definition = CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) };
                definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
                definition
            })
            .artifact_kind(artifact_kind())
            .icon_id("box")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Koordinator"])
            .mode_def(edit::definition())
            .default_mode_id(edit::CAD_PLAY_MODE_EDIT)
            .window_kind_def(shape::definition())
            .window_kind_def(building::definition())
            .window_kind_def(energy::definition())
            .window_kind_def(structure_classic::definition())
            .default_layout(edit::layout())
            .mutation("addObject", LocalizedLabel::native("Add Object", "Objekt hinzufügen"))
            .mutation("patchObject", LocalizedLabel::native("Patch Object", "Objekt aktualisieren"))
            .mutation("patchSelection", LocalizedLabel::native("Patch Selection", "Auswahl aktualisieren"))
            .action_with(ActionDefinition::bounded_catalog("deleteObject", LocalizedLabel::native("Delete Object", "Objekt löschen"), ActionKind::Mutation).category("actions"))
            .action_with(ActionDefinition::bounded_catalog("duplicateObject", LocalizedLabel::native("Duplicate Object", "Objekt duplizieren"), ActionKind::Mutation).category("create"))
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .mutation("renameNode", LocalizedLabel::native("Rename Node", "Knoten umbenennen"))
            .action_with(ActionDefinition::bounded_catalog("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Mutation).category("transform"))
            .action_with(ActionDefinition::bounded_catalog("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Mutation).category("transform"))
            .action_with(ActionDefinition::bounded_catalog("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Mutation).category("transform"))
            .mutation("applyTransformation", LocalizedLabel::native("Apply Transformation", "Transformation anwenden"))
            .mutation("importCadFile", LocalizedLabel::native("Import CAD File", "CAD-Datei importieren"))
            .action_with(ActionDefinition::bounded_catalog("patchCadPlayReference", LocalizedLabel::native("Patch Reference", "Referenz aktualisieren"), ActionKind::Mutation).in_palette(false))
            .action_with(cad_reference_flag_action("setReferenceHidden", "hidden", LocalizedLabel::native("Set Reference Hidden", "Referenz verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(cad_reference_flag_action("setReferenceLocked", "locked", LocalizedLabel::native("Set Reference Locked", "Referenz gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .action_with(ActionDefinition::bounded_catalog("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation).in_palette(false))
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setProjection", LocalizedLabel::native("Set Projection", "Projektion festlegen"), ActionKind::View, "scan"))
            .action_with(ActionDefinition::new("setProjectionParam", LocalizedLabel::native("Set Projection Parameter", "Projektionsparameter festlegen"), ActionKind::View, "scan"))
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_with(ActionDefinition::bounded_catalog("setNodeSelection", LocalizedLabel::native("Set Node Selection", "Knotenauswahl festlegen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("setReferenceSelection", LocalizedLabel::native("Set Reference Selection", "Referenzauswahl festlegen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("referenceHover", LocalizedLabel::native("Reference Hover", "Überfahren (Referenz)"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand").in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("engagementPossibleSelect", LocalizedLabel::native("Engagement Possible Select", "Eingabeoption auswählen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View, "hand").in_palette(false))
            .action_with(ActionDefinition::new("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View, "mouse-pointer").in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("worldPointerMove", LocalizedLabel::native("World Pointer Move", "Welt-Zeiger bewegt"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::bounded_catalog("setDislocateOption", LocalizedLabel::native("Set Dislocate Option", "Versetzen-Option festlegen"), ActionKind::View).in_palette(false))
            .shell_action("saveSelected", LocalizedLabel::native("Save Selected", "Auswahl speichern"))
            .shell_action("saveInPlay", LocalizedLabel::native("Save In Play", "Im Play speichern"))
            .shell_action("saveCurrent", LocalizedLabel::native("Save Current", "Aktuelles speichern"))
            .shell_action("loadRawRequest", LocalizedLabel::native("Load Raw Request", "Rohdaten laden"))
            .action_args("saveCurrent", vec![ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![
                ActionArgOption::new("step", LocalizedLabel::native("STEP", "STEP")),
                ActionArgOption::new("obj", LocalizedLabel::native("OBJ", "OBJ")),
                ActionArgOption::new("stl", LocalizedLabel::native("STL", "STL")),
            ]).default_value(&"step")])
            .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                ActionArgOption::new(CAD_EXAMPLE_FOREST_LEFT, LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Sechseckig geschnittener Betonwald links")),
            ]).required()])
            .action_describe("addObject", LocalizedLabel::native("Adds a new geometric object to the CAD model.", "Fügt dem CAD-Modell ein neues geometrisches Objekt hinzu."))
            .action_use_when("addObject", vec!["add a box".into(), "create an object".into(), "place a solid in the model".into()])
            .action_describe("patchObject", LocalizedLabel::native("Sets one named property of one CAD object — its name, material, dimensions or placement.", "Setzt eine benannte Eigenschaft eines CAD-Objekts — Name, Material, Abmessungen oder Platzierung."))
            .action_describe("patchSelection", LocalizedLabel::native("Sets one named property on every currently selected CAD object.", "Setzt eine benannte Eigenschaft auf allen aktuell ausgewählten CAD-Objekten."))
            .action_describe("deleteObject", LocalizedLabel::native("Removes one object from the CAD model by id.", "Entfernt ein Objekt anhand seiner Id aus dem CAD-Modell."))
            .action_describe("duplicateObject", LocalizedLabel::native("Copies one CAD object and inserts the copy alongside the original.", "Kopiert ein CAD-Objekt und fügt die Kopie neben dem Original ein."))
            .action_describe("addNode", LocalizedLabel::native("Adds a node to the model tree — the grouping level CAD objects hang from.", "Fügt dem Modellbaum einen Knoten hinzu — die Gruppierungsebene, an der CAD-Objekte hängen."))
            .action_describe("renameNode", LocalizedLabel::native("Renames one node of the model tree.", "Benennt einen Knoten des Modellbaums um."))
            .action_describe("translateSelection", LocalizedLabel::native("Moves the selected objects by a delta along x, y and z.", "Verschiebt die ausgewählten Objekte um einen Versatz entlang x, y und z."))
            .action_use_when("translateSelection", vec!["move the selection".into(), "shift these objects 2 metres east".into()])
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the selected objects by an angle around a given axis.", "Dreht die ausgewählten Objekte um einen Winkel um eine angegebene Achse."))
            .action_use_when("rotateSelection", vec!["rotate the selection 90 degrees".into()])
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the selected objects by a factor per axis.", "Skaliert die ausgewählten Objekte um einen Faktor je Achse."))
            .action_describe("applyTransformation", LocalizedLabel::native("Bakes the staged transformation into the target pane's model; refused until its derivation rules exist.", "Schreibt die vorbereitete Transformation fest in das Modell des Zielbereichs; abgelehnt, bis ihre Ableitungsregeln existieren."))
            .action_describe("importCadFile", LocalizedLabel::native("Replaces the whole CAD scene with a spatial scene file, or inserts a single STEP, OBJ or STL object into the addressed pane's model.", "Ersetzt die gesamte CAD-Szene durch eine räumliche Szenendatei oder fügt ein einzelnes STEP-, OBJ- oder STL-Objekt in das Modell des angesprochenen Bereichs ein."))
            .action_use_when("importCadFile", vec!["import a step file".into(), "load this geometry".into()])
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole model with one of the plugin's declared playground examples.", "Ersetzt das gesamte Modell durch eines der deklarierten Beispiele des Plugins."))
            .action_describe("saveCurrent", LocalizedLabel::native("Exports the current model to a downloadable CAD file in the chosen format.", "Exportiert das aktuelle Modell als herunterladbare CAD-Datei im gewählten Format."))
            .action_use_when("saveCurrent", vec!["export the model as step".into(), "download this as an obj".into()])
            .action_describe("saveSelected", LocalizedLabel::native("Exports only the selected objects to a downloadable CAD file.", "Exportiert nur die ausgewählten Objekte als herunterladbare CAD-Datei."))
            .action_describe("saveInPlay", LocalizedLabel::native("Writes the current model back into the playground document it was opened from.", "Schreibt das aktuelle Modell in das Playground-Dokument zurück, aus dem es geöffnet wurde."))
            .action_describe("loadRawRequest", LocalizedLabel::native("Asks the host to open a raw geometry file and load it into the model.", "Fordert den Host auf, eine Rohgeometriedatei zu öffnen und in das Modell zu laden."))
            .action_describe("patchCadPlayReference", LocalizedLabel::native("Sets one named property of one reference in the model's play definition — its target, placement or parameter.", "Setzt eine benannte Eigenschaft einer Referenz in der Play-Definition des Modells — Ziel, Platzierung oder Parameter."))
            .action_describe("setReferenceHidden", LocalizedLabel::native("Sets one reference overlay hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt eine Referenzüberlagerung oder zeigt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("setReferenceLocked", LocalizedLabel::native("Sets one reference overlay locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt eine Referenzüberlagerung oder entsperrt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("setProjection", LocalizedLabel::native("Switches the viewport between perspective and the orthographic projections.", "Schaltet das Ansichtsfenster zwischen Perspektive und den orthografischen Projektionen um."))
            .action_describe("toggleSun", LocalizedLabel::native("Turns the scene's sun light on or off.", "Schaltet das Sonnenlicht der Szene ein oder aus."))
            .action_describe("setSunAzimuth", LocalizedLabel::native("Sets the sun's compass direction, in degrees.", "Legt die Himmelsrichtung der Sonne in Grad fest."))
            .action_describe("setSunElevation", LocalizedLabel::native("Sets the sun's height above the horizon, in degrees.", "Legt die Höhe der Sonne über dem Horizont in Grad fest."))
            .action_describe("setSunIntensity", LocalizedLabel::native("Sets how bright the scene's sun light is.", "Legt fest, wie hell das Sonnenlicht der Szene ist."))
            .action_destructive("deleteObject")
            .action_destructive("setActiveExample")
            .action_destructive("saveCurrent")
            .action_destructive("saveSelected")
            .action_destructive("saveInPlay")
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("engagementPossibleSelect", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("engagementRepeatLast", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("engagementAbort", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("worldPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("worldPointerMove", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("referenceHover", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("setProjectionParam", semio_framework_plugin::CapabilityAudience::Chrome)
            .utility(cad_dislocate_utility())
            .window_kind_utilities(shape::WINDOW_KIND_ID, cad_dislocate_utility_refs())
            .window_kind_utilities(building::WINDOW_KIND_ID, cad_dislocate_utility_refs())
            .window_kind_utilities(energy::WINDOW_KIND_ID, cad_dislocate_utility_refs())
            .window_kind_utilities(structure_classic::WINDOW_KIND_ID, cad_dislocate_utility_refs())
            .interaction(cad_interaction_definition())
            .window_kind_interactions(shape::WINDOW_KIND_ID, vec![semio_framework_plugin::InteractionRef::new(CAD_INTERACTION_DOMAIN)])
            .window_kind_interactions(building::WINDOW_KIND_ID, vec![semio_framework_plugin::InteractionRef::new(CAD_INTERACTION_DOMAIN)])
            .window_kind_interactions(energy::WINDOW_KIND_ID, vec![semio_framework_plugin::InteractionRef::new(CAD_INTERACTION_DOMAIN)])
            .window_kind_interactions(structure_classic::WINDOW_KIND_ID, vec![semio_framework_plugin::InteractionRef::new(CAD_INTERACTION_DOMAIN)])
            .panel_tab_def(document::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            .config(CadPlayApp::config_spec())
            .io(cad_io())
            .action_interactive_job("addObject", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchObject", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteObject", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateObject", InteractiveJobClassification::Migrated)
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("renameNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("applyTransformation", InteractiveJobClassification::Migrated)
            .action_interactive_job("importCadFile", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchCadPlayReference", InteractiveJobClassification::Migrated)
            .action_interactive_job("setReferenceHidden", InteractiveJobClassification::Migrated)
            .action_interactive_job("setReferenceLocked", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjectionParam", InteractiveJobClassification::Migrated)
            .action_interactive_job("setDislocateOption", InteractiveJobClassification::Migrated)
            .action_interactive_job("setNodeSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setReferenceSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("referenceHover", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementPossibleSelect", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementRepeatLast", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerMove", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("saveSelected", InteractiveJobClassification::Migrated)
            .action_interactive_job("saveInPlay", InteractiveJobClassification::Migrated)
            .action_interactive_job("saveCurrent", InteractiveJobClassification::Migrated)
            .action_interactive_job("loadRawRequest", InteractiveJobClassification::Migrated)
            .build_definition()
}
//#endregion 🔖️Manifest
