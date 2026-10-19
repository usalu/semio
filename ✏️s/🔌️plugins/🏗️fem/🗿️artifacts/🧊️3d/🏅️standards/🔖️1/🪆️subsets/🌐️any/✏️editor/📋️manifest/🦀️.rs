//! 📋️ Manifest of the FEM 3D editor: the inspector patch action and the app manifest stitch of `create_fem3d_app`. See [`super`].

use super::*;

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node. fem3d's mode/windows are all scalar
/// (`.mode(..)`/`.window_kind(..)`) declarations — no `_def` passthrough exists for them since no
/// `ModeDefinition`/`WindowKindDefinition` object is built anywhere (see `modes::edit`'s doc comment).
///
/// 🚧️ SDK GAP (contract §2.4, `App { definition, examples }` split): `EditorBuilder` has no
/// `.example(...)`/`.workflow(...)` methods — the pre-migration chain's trailing
/// `.example(examples::demo::ID, LocalizedLabel::native("Family House", "Einfamilienhaus"),
/// semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::FEM3D_EXAMPLE_TEXT, "file")` and `.workflow("fem3d", "FEM 3D",
/// "structure")` calls are dropped here, not ported. `setActiveExample`'s handler loads the same
/// `FEM3D_EXAMPLE_TEXT` fixture directly.
/// 🩹️ One inspector patch action — internal (not in the palette; its arguments are authored by the
/// inspector row that binds it), a document mutation on one entity.
fn fem3d_patch_action(id: &str, label: LocalizedLabel) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition {
        in_palette: false,
        args: vec![ActionArgDef::text("id", LocalizedLabel::native("Entity", "Element")).required(), ActionArgDef::text("field", LocalizedLabel::native("Field", "Feld")).required(), ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).required()],
        ..semio_framework_plugin::ActionDefinition::new(id, label, semio_framework_plugin::ActionKind::Mutation, "pencil")
    }
}

pub fn create_fem3d_app() -> AppDefinition {
    Editor::builder(semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::FEM3D_DIALECT)
            .document(["semio", "fem", "fem3d"])
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Statik"])
            .artifact_kind(semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::document_artifact_kind())
            .artifact_kind(semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::computation_artifact_kind())
            .icon_id("fem-app")
            .mode(edit::MODE_ID, LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id(edit::MODE_ID)
            .window_kind(window_model::FEM3D_WINDOW_MODEL, LocalizedLabel::native("Model", "Modell"), window_model::FEM3D_BODY_MODEL, semio_framework_ui_contract::SurfaceKind::World3d, "fem-model")
            .window_kind(window_results::FEM3D_WINDOW_RESULTS, LocalizedLabel::native("Results", "Ergebnisse"), window_results::FEM3D_BODY_RESULTS, semio_framework_ui_contract::SurfaceKind::World3d, "bar-chart-3")
            .default_layout(create_default_layout(
                &[window_model::FEM3D_WINDOW_MODEL.into(), window_results::FEM3D_WINDOW_RESULTS.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Model".into(), "Results".into()]),
            ))
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_args("addNode", vec![
                ActionArgDef::number("x", LocalizedLabel::data("X")).required(),
                ActionArgDef::number("y", LocalizedLabel::data("Y")).required(),
                ActionArgDef::number("z", LocalizedLabel::data("Z")).required(),
            ])
            .mutation("addBar", LocalizedLabel::native("Add Bar", "Stab hinzufügen"))
            .mutation("addFrame", LocalizedLabel::native("Add Frame", "Rahmen hinzufügen"))
            .mutation("addMaterial", LocalizedLabel::native("Add Material", "Material hinzufügen"))
            .mutation("addSection", LocalizedLabel::native("Add Section", "Querschnitt hinzufügen"))
            .mutation("addSupport", LocalizedLabel::native("Add Support", "Lager hinzufügen"))
            .mutation("addNodalLoad", LocalizedLabel::native("Add Nodal Load", "Knotenlast hinzufügen"))
            .action_args("addNodalLoad", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .mutation("addMemberUdl", LocalizedLabel::native("Add Member UDL", "Streckenlast hinzufügen"))
            .action_args("addMemberUdl", vec![ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall"))])
            .mutation("addAreaLoad", LocalizedLabel::native("Add Area Load", "Flächenlast hinzufügen"))
            .action_args("addAreaLoad", vec![
                ActionArgDef::text("solidId", LocalizedLabel::native("Solid", "Volumenkörper")).required(),
                ActionArgDef::number("pressure", LocalizedLabel::native("Pressure", "Druck")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addSolid", LocalizedLabel::native("Add Solid", "Volumenkörper hinzufügen"))
            .action_args("addSolid", vec![
                ActionArgDef::number("x", LocalizedLabel::data("X")).required(),
                ActionArgDef::number("y", LocalizedLabel::data("Y")).required(),
                ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required(),
                ActionArgDef::number("depth", LocalizedLabel::native("Depth", "Tiefe")).required(),
                ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::data("Material")).required(),
                ActionArgDef::number("baseZ", LocalizedLabel::native("Base Z", "Basis Z")).default_value(&0.0),
                ActionArgDef::number("layers", LocalizedLabel::native("Layers", "Schichten")).default_value(&1),
                ActionArgDef::number("meshSize", LocalizedLabel::native("Mesh Size", "Netzgröße")).default_value(&0.5),
                ActionArgDef::select("axis", LocalizedLabel::native("Axis", "Achse"), vec![ActionArgOption::new("x", LocalizedLabel::data("X")), ActionArgOption::new("y", LocalizedLabel::data("Y")), ActionArgOption::new("z", LocalizedLabel::data("Z"))]).default_value(&"z"),
            ])
            .mutation("addLoadCase", LocalizedLabel::native("Add Load Case", "Lastfall hinzufügen"))
            .action_args("addLoadCase", vec![
                ActionArgDef::text("name", LocalizedLabel::data("Name")).required(),
                ActionArgDef::toggle("selfWeight", LocalizedLabel::native("Self Weight", "Eigengewicht")).default_value(&false),
            ])
            .mutation("addCombination", LocalizedLabel::native("Add Combination", "Kombination hinzufügen"))
            .action_args("addCombination", vec![
                ActionArgDef::text("name", LocalizedLabel::data("Name")).required(),
                ActionArgDef::text("terms", LocalizedLabel::native("Terms", "Terme")).required(),
            ])
            .mutation("setSelfWeight", LocalizedLabel::native("Set Self Weight", "Eigengewicht festlegen"))
            .action_args("setSelfWeight", vec![
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")).required(),
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).required(),
            ])
            .mutation("setAnalysisSettings", LocalizedLabel::native("Set Analysis Settings", "Analyseeinstellungen festlegen"))
            .action_args("setAnalysisSettings", vec![
                ActionArgDef::number("modalCount", LocalizedLabel::native("Modal Count", "Anzahl Moden")),
                ActionArgDef::number("bucklingCount", LocalizedLabel::native("Buckling Count", "Anzahl Beulmoden")),
                ActionArgDef::number("deformationScale", LocalizedLabel::native("Deformation Scale", "Verformungsmaßstab")),
            ])
            .mutation("removeSelection", LocalizedLabel::native("Remove Selection", "Auswahl entfernen"))
            .action_destructive("removeSelection")
            .action_with(semio_framework_plugin::ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), semio_framework_plugin::ActionKind::View, "camera"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), semio_framework_plugin::ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new(semio_s_artifact_fem_3d_io::examples::concrete_forest::ID, semio_s_artifact_fem_3d_io::examples::concrete_forest::label()), ActionArgOption::new(semio_s_artifact_fem_3d_io::examples::demo::ID, LocalizedLabel::native("Default", "Standard")), ActionArgOption::new(semio_s_artifact_fem_3d_io::examples::house::ID, semio_s_artifact_fem_3d_io::examples::house::label())]).default_value(&semio_s_artifact_fem_3d_io::examples::concrete_forest::ID),
            ])
            .view_action("setResultDisplay", LocalizedLabel::native("Set Result Display", "Ergebnisanzeige festlegen"))
            .action_args("setResultDisplay", semio_s_fem_app_surface::result_display_action_args())
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("addBar", InteractiveJobClassification::Migrated)
            .action_interactive_job("addFrame", InteractiveJobClassification::Migrated)
            .action_interactive_job("addMaterial", InteractiveJobClassification::Migrated)
            .action_interactive_job("addSection", InteractiveJobClassification::Migrated)
            .action_interactive_job("addSupport", InteractiveJobClassification::Migrated)
            .action_interactive_job("addNodalLoad", InteractiveJobClassification::Migrated)
            .action_interactive_job("addMemberUdl", InteractiveJobClassification::Migrated)
            .action_interactive_job("addAreaLoad", InteractiveJobClassification::Migrated)
            .action_interactive_job("addSolid", InteractiveJobClassification::Migrated)
            .action_interactive_job("addLoadCase", InteractiveJobClassification::Migrated)
            .action_interactive_job("addCombination", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelfWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAnalysisSettings", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setResultDisplay", InteractiveJobClassification::Migrated)
            .action_with(fem3d_patch_action("patchNode", LocalizedLabel::native("Patch Node", "Knoten ändern")))
            .action_with(fem3d_patch_action("patchElement", LocalizedLabel::native("Patch Element", "Element ändern")))
            .action_with(fem3d_patch_action("patchMaterial", LocalizedLabel::native("Patch Material", "Material ändern")))
            .action_with(fem3d_patch_action("patchSection", LocalizedLabel::native("Patch Section", "Querschnitt ändern")))
            .action_with(fem3d_patch_action("patchSupport", LocalizedLabel::native("Patch Support", "Lager ändern")))
            .action_with(fem3d_patch_action("patchSolid", LocalizedLabel::native("Patch Solid", "Volumenkörper ändern")))
            .action_with(fem3d_patch_action("patchLoad", LocalizedLabel::native("Patch Load", "Last ändern")))
            .action_with(fem3d_patch_action("patchLoadCase", LocalizedLabel::native("Patch Load Case", "Lastfall ändern")))
            .action_with(fem3d_patch_action("patchCombination", LocalizedLabel::native("Patch Combination", "Kombination ändern")))
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("setResultAnimation", LocalizedLabel::native("Set Result Animation", "Ergebnisanimation festlegen"), semio_framework_plugin::ActionKind::View, "play") })
            .action_args("setResultAnimation", vec![
                ActionArgDef::slider("phase", LocalizedLabel::native("Phase", "Phase"), 0.0, 1.0),
                ActionArgDef::toggle("playing", LocalizedLabel::native("Playing", "Läuft")),
                ActionArgDef::number("speed", LocalizedLabel::native("Speed", "Geschwindigkeit")),
                ActionArgDef::select("loopMode", LocalizedLabel::native("Loop", "Schleife"), vec![
                    ActionArgOption::new("loop", LocalizedLabel::native("Loop", "Schleife")),
                    ActionArgOption::new("pingPong", LocalizedLabel::native("Ping-Pong", "Ping-Pong")),
                    ActionArgOption::new("once", LocalizedLabel::native("Once", "Einmal")),
                ]),
                ActionArgDef::select("waveform", LocalizedLabel::native("Waveform", "Wellenform"), vec![ActionArgOption::new("ramp", LocalizedLabel::native("Ramp", "Rampe")), ActionArgOption::new("sine", LocalizedLabel::native("Sine", "Sinus"))]),
            ])
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("resultAnimationTick", LocalizedLabel::native("Result Animation Tick", "Ergebnisanimation Takt"), semio_framework_plugin::ActionKind::View, "timer") })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("focusEntity", LocalizedLabel::native("Focus Entity", "Element fokussieren"), semio_framework_plugin::ActionKind::View, "focus") })
            .action_args("focusEntity", vec![ActionArgDef::text("id", LocalizedLabel::native("Entity", "Element")).required()])
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .action_args("translateSelection", vec![
                ActionArgDef::number("dx", LocalizedLabel::native("Dx", "Dx")).required(),
                ActionArgDef::number("dy", LocalizedLabel::native("Dy", "Dy")).required(),
                ActionArgDef::number("dz", LocalizedLabel::native("Dz", "Dz")).required(),
            ])
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .action_args("rotateSelection", vec![
                ActionArgDef::number("ax", LocalizedLabel::native("Axis X", "Achse X")).default_value(&0.0),
                ActionArgDef::number("ay", LocalizedLabel::native("Axis Y", "Achse Y")).default_value(&0.0),
                ActionArgDef::number("az", LocalizedLabel::native("Axis Z", "Achse Z")).default_value(&1.0),
                ActionArgDef::number("angle", LocalizedLabel::native("Angle", "Winkel")).required(),
            ])
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .action_args("scaleSelection", vec![
                ActionArgDef::number("sx", LocalizedLabel::native("Sx", "Sx")).required(),
                ActionArgDef::number("sy", LocalizedLabel::native("Sy", "Sy")).required(),
                ActionArgDef::number("sz", LocalizedLabel::native("Sz", "Sz")).required(),
            ])
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transform-Griff festlegen"), semio_framework_plugin::ActionKind::View, "settings-2") })
            .action_args("setTransformGumballFlag", vec![
                ActionArgDef::text("flag", LocalizedLabel::native("Flag", "Flagge")).required(),
                ActionArgDef::toggle("pressed", LocalizedLabel::native("Pressed", "Gedrückt")),
            ])
            .action_interactive_job("patchNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchElement", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchMaterial", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchSection", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchSupport", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchSolid", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchLoad", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchLoadCase", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchCombination", InteractiveJobClassification::Migrated)
            .action_interactive_job("setResultAnimation", InteractiveJobClassification::Migrated)
            .action_interactive_job("resultAnimationTick", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusEntity", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", InteractiveJobClassification::Migrated)
            .utility(UtilityDefinition { group: Some("Transform".into()), category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new(FEM3D_UTILITY_TRANSFORM, LocalizedLabel::native("Transform", "Transformieren"), "move-3d") })
            .window_kind_utilities(window_model::FEM3D_WINDOW_MODEL, vec![FEM3D_UTILITY_TRANSFORM.into()])
            .interaction(fem3d_interaction_definition())
            .window_kind_interactions(window_model::FEM3D_WINDOW_MODEL, vec![InteractionRef::new(FEM3D_INTERACTION_DOMAIN)])
            .window_kind_interactions(window_results::FEM3D_WINDOW_RESULTS, vec![InteractionRef::new(FEM3D_INTERACTION_DOMAIN)])
            .panel_tab_def(artifact_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .panel_tab_def(results_panel::definition())
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("delete", "removeSelection")
            .keybinding("backspace", "removeSelection")
            .keybinding("space", "setResultAnimation")
            .config(Fem3dPlayApp::config_spec())
            .io(fem3d_io())
            .action_describe("addNode", LocalizedLabel::native("Adds a node at x, y, z to the 3D structural model; elements, supports and loads attach to nodes.", "Fügt dem 3D-Tragwerksmodell einen Knoten an x, y, z hinzu; Elemente, Lager und Lasten hängen an Knoten."))
            .action_describe("addFrame", LocalizedLabel::native("Adds a frame member (a 3D beam carrying bending and torsion) between two existing nodes, with a material, a section and a roll angle.", "Fügt zwischen zwei vorhandenen Knoten ein Rahmenglied (einen 3D-Balken für Biegung und Torsion) mit Material, Querschnitt und Rollwinkel hinzu."))
            .action_describe("addSolid", LocalizedLabel::native("Adds a box-shaped solid at x, y with a width, depth and height, a material, base elevation, layer count, mesh size and axis, meshed into finite elements.", "Fügt einen quaderförmigen Volumenkörper an x, y mit Breite, Tiefe, Höhe, Material, Basishöhe, Schichtanzahl, Netzweite und Achse hinzu, der in finite Elemente vernetzt wird."))
            .action_describe("patchSolid", LocalizedLabel::native("Sets one field of one solid by id, such as its size, material or mesh size.", "Setzt ein Feld eines Volumenkörpers anhand seiner Id, etwa Größe, Material oder Netzweite."))
            .action_describe("addBar", LocalizedLabel::native("Adds a bar (an axial-only truss member) between two existing nodes of the 3D model, with a material and a section.", "Fügt zwischen zwei vorhandenen Knoten des 3D-Modells einen Stab (ein nur normalkraftbeanspruchtes Fachwerkglied) mit Material und Querschnitt hinzu."))
            .action_describe("addMaterial", LocalizedLabel::native("Adds a named material with its Young's modulus (and, in 3D, its shear modulus) for elements to use.", "Fügt ein benanntes Material mit seinem Elastizitätsmodul (in 3D auch Schubmodul) hinzu, das Elemente verwenden können."))
            .action_describe("addSection", LocalizedLabel::native("Adds a named cross-section with its area and second moment of area for bars and beams to use.", "Fügt einen benannten Querschnitt mit Fläche und Flächenträgheitsmoment hinzu, den Stäbe und Balken verwenden können."))
            .action_describe("addSupport", LocalizedLabel::native("Adds a support to one node, fixing the given degrees of freedom (translations and rotations) against movement.", "Fügt einem Knoten ein Lager hinzu, das die angegebenen Freiheitsgrade (Verschiebungen und Verdrehungen) festhält."))
            .action_describe("addNodalLoad", LocalizedLabel::native("Adds a point load of the given value on one degree of freedom of a node, in a load case.", "Fügt eine Einzellast mit dem angegebenen Wert auf einen Freiheitsgrad eines Knotens in einem Lastfall hinzu."))
            .action_describe("addMemberUdl", LocalizedLabel::native("Adds a uniformly distributed load (wx along, wy across) on one bar or beam element, in a load case.", "Fügt eine gleichmäßig verteilte Streckenlast (wx längs, wy quer) auf ein Stab- oder Balkenelement in einem Lastfall hinzu."))
            .action_describe("addAreaLoad", LocalizedLabel::native("Adds a uniform pressure load on one solid, in a load case.", "Fügt eine gleichmäßige Flächenlast (Druck) auf einen Volumenkörper in einem Lastfall hinzu."))
            .action_describe("addLoadCase", LocalizedLabel::native("Adds a named load case that loads are assigned to, optionally including the structure's self weight.", "Fügt einen benannten Lastfall hinzu, dem Lasten zugeordnet werden, optional mit dem Eigengewicht des Tragwerks."))
            .action_describe("addCombination", LocalizedLabel::native("Adds a named load combination that sums weighted load cases for the analysis.", "Fügt eine benannte Lastfallkombination hinzu, die gewichtete Lastfälle für die Analyse überlagert."))
            .action_describe("setSelfWeight", LocalizedLabel::native("Turns the structure's self weight on or off in one load case.", "Schaltet das Eigengewicht des Tragwerks in einem Lastfall ein oder aus."))
            .action_describe("setAnalysisSettings", LocalizedLabel::native("Sets how many vibration modes and buckling modes the analysis computes and the scale deformations are drawn with.", "Legt fest, wie viele Eigenschwingungs- und Knickformen die Analyse berechnet und mit welchem Maßstab Verformungen gezeichnet werden."))
            .action_describe("removeSelection", LocalizedLabel::native("Deletes every selected node, element, support or load from the model, together with what depends on it.", "Löscht alle ausgewählten Knoten, Elemente, Lager oder Lasten samt allem, was davon abhängt, aus dem Modell."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 3D structural model with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte 3D-Tragwerksmodell durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
            .action_describe("setResultDisplay", LocalizedLabel::native("Chooses which analysis result the view shows: the static result of a load case or combination, or a numbered vibration or buckling mode; the model is not changed.", "Wählt, welches Analyseergebnis die Ansicht zeigt: das statische Ergebnis eines Lastfalls oder einer Kombination oder eine nummerierte Eigen- oder Knickform; das Modell ändert sich nicht."))
            .action_describe("patchNode", LocalizedLabel::native("Sets one field of one node by id, such as a coordinate.", "Setzt ein Feld eines Knotens anhand seiner Id, etwa eine Koordinate."))
            .action_describe("patchElement", LocalizedLabel::native("Sets one field of one bar or beam element by id, such as its material, section or end nodes.", "Setzt ein Feld eines Stab- oder Balkenelements anhand seiner Id, etwa Material, Querschnitt oder Endknoten."))
            .action_describe("patchMaterial", LocalizedLabel::native("Sets one field of one material by id, such as its name or Young's modulus.", "Setzt ein Feld eines Materials anhand seiner Id, etwa Name oder Elastizitätsmodul."))
            .action_describe("patchSection", LocalizedLabel::native("Sets one field of one cross-section by id, such as its area or second moment of area.", "Setzt ein Feld eines Querschnitts anhand seiner Id, etwa Fläche oder Flächenträgheitsmoment."))
            .action_describe("patchSupport", LocalizedLabel::native("Sets one field of one support by id, such as which degrees of freedom it fixes.", "Setzt ein Feld eines Lagers anhand seiner Id, etwa welche Freiheitsgrade es festhält."))
            .action_describe("patchLoad", LocalizedLabel::native("Sets one field of one load by id, such as its value, direction or load case.", "Setzt ein Feld einer Last anhand ihrer Id, etwa Wert, Richtung oder Lastfall."))
            .action_describe("patchLoadCase", LocalizedLabel::native("Sets one field of one load case by id, such as its name or whether it includes self weight.", "Setzt ein Feld eines Lastfalls anhand seiner Id, etwa Name oder ob er das Eigengewicht enthält."))
            .action_describe("patchCombination", LocalizedLabel::native("Sets one field of one load combination by id, such as its name or the factors of its load cases.", "Setzt ein Feld einer Lastfallkombination anhand ihrer Id, etwa Name oder die Faktoren ihrer Lastfälle."))
            .action_describe("translateSelection", LocalizedLabel::native("Moves the given or selected nodes and solids by dx, dy and dz as one editable history step, dragging the attached elements, supports and loads along.", "Verschiebt die angegebenen oder ausgewählten Knoten und Volumenkörper um dx, dy und dz als einen bearbeitbaren Verlaufsschritt und zieht die angeschlossenen Elemente, Lager und Lasten mit."))
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the given or selected nodes and solids by an angle around the axis ax, ay, az through their common centre, as one editable history step.", "Dreht die angegebenen oder ausgewählten Knoten und Volumenkörper um einen Winkel um die Achse ax, ay, az durch ihren gemeinsamen Mittelpunkt, als einen bearbeitbaren Verlaufsschritt."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the given or selected nodes and solids by sx, sy and sz about their common centre, as one editable history step.", "Skaliert die angegebenen oder ausgewählten Knoten und Volumenkörper um sx, sy und sz um ihren gemeinsamen Mittelpunkt, als einen bearbeitbaren Verlaufsschritt."))
            .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
            .build_definition()
}
//#endregion 🔖️Manifest
