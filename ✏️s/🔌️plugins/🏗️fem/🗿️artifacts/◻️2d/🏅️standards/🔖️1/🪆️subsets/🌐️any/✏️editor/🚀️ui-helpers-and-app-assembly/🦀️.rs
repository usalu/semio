//! 🚀️ UI helpers and app assembly: the utility helpers, the patch action and the app creation with its manifest stitch. See [`super`].

use super::*;

//#region 🔖️Fem2dPlayApp
//#region 🔖️UiHelpers
fn fem2d_utility(id: &str, label: LocalizedLabel, icon: &str, category: UtilityCategory) -> UtilityDefinition {
    UtilityDefinition { group: Some("Select".into()), category: Some(category), ..UtilityDefinition::new(id, label, icon) }
}

const FEM2D_CANVAS_UTILITIES: &[&str] = &["selectDirect", "selectMarquee", "selectLasso", "transformMove", "transform"];
//#endregion 🔖️UiHelpers
//#endregion 🔖️Fem2dPlayApp

//#region 🔖️Manifest
/// 🩹️ One inspector patch action — internal (not in the palette; its arguments are authored by the
/// inspector row that binds it), a document mutation on one entity.
fn fem2d_patch_action(id: &str, label: LocalizedLabel) -> semio_framework_plugin::ActionDefinition {
    semio_framework_plugin::ActionDefinition {
        in_palette: false,
        args: vec![ActionArgDef::text("id", LocalizedLabel::native("Entity", "Element")).required(), ActionArgDef::text("field", LocalizedLabel::native("Field", "Feld")).required(), ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")).required()],
        ..semio_framework_plugin::ActionDefinition::new(id, label, semio_framework_plugin::ActionKind::Mutation, "pencil")
    }
}

/// 📚️ `AppBuilder` carries no `.example(...)`: an example is declared ONCE as a definition leaf
/// (`📚️examples/🎬️demo`'s `ExampleSource`) and reaches `PluginManifest.examples` through the subset
/// root's `SubsetDeclaration.examples` — see `🪆️subsets/🌐️any/🦀️.rs`. The shell's navbar switcher
/// reads that list and dispatches `setActiveExample { exampleId }` back into this app, which is why
/// `setActiveExample`'s select option below is that same `demo::ID` and why the action is `Migrated`
/// (a `BatchOnlyPendingRewrite` classification would make the switcher a dead control). `.workflow(...)`
/// stays dropped: `WorkflowDefinition` was deleted from framework-core, with no replacement surface.
pub fn create_fem2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(semio_s_artifact_fem_2d_io::standards::v1::subsets::any::io::FEM2D_DIALECT)
            .document(["semio", "fem", "fem2d"])
            // 🔌️ The computed-results output artifact (`results:out`'s `kind_id`, see
            // `fem2d_io` above) — deliberately a different `media_type`
            // (`Computation`×`Value`) than the PORT's wire-level `Data`×`Value`.
            .artifact_kind(semio_s_artifact_fem_2d_io::standards::v1::subsets::any::io::document_artifact_kind())
            .artifact_kind(semio_s_artifact_fem_2d_io::standards::v1::subsets::any::io::computation_artifact_kind())
            .icon_id("fem-app")
            .mode(edit::MODE_ID, LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id(edit::MODE_ID)
            .window_kind(model_window::WINDOW_KIND_ID, LocalizedLabel::native("Model", "Modell"), model_window::BODY_KEY, semio_framework_ui_contract::SurfaceKind::Canvas2d, "fem-model")
            .window_kind(results_window::WINDOW_KIND_ID, LocalizedLabel::native("Results", "Ergebnisse"), results_window::BODY_KEY, semio_framework_ui_contract::SurfaceKind::Canvas2d, "bar-chart-3")
            .default_layout(create_default_layout(
                &[model_window::WINDOW_KIND_ID.into(), results_window::WINDOW_KIND_ID.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Model".into(), "Results".into()]),
            ))
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_args("addNode", vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required(),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required(),
            ])
            .mutation("addBar", LocalizedLabel::native("Add Bar", "Stab hinzufügen"))
            .action_args("addBar", vec![
                ActionArgDef::text("start", LocalizedLabel::native("Start Node", "Startknoten")).required(),
                ActionArgDef::text("end", LocalizedLabel::native("End Node", "Endknoten")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::native("Material", "Material")).required(),
                ActionArgDef::text("sectionId", LocalizedLabel::native("Section", "Querschnitt")).required(),
            ])
            .mutation("addBeam", LocalizedLabel::native("Add Beam", "Balken hinzufügen"))
            .action_args("addBeam", vec![
                ActionArgDef::text("start", LocalizedLabel::native("Start Node", "Startknoten")).required(),
                ActionArgDef::text("end", LocalizedLabel::native("End Node", "Endknoten")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::native("Material", "Material")).required(),
                ActionArgDef::text("sectionId", LocalizedLabel::native("Section", "Querschnitt")).required(),
            ])
            .mutation("addMaterial", LocalizedLabel::native("Add Material", "Material hinzufügen"))
            .action_args("addMaterial", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::number("e", LocalizedLabel::native("Young's Modulus", "Elastizitätsmodul")).default_value(&2.1e11),
            ])
            .mutation("addSection", LocalizedLabel::native("Add Section", "Querschnitt hinzufügen"))
            .action_args("addSection", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::number("area", LocalizedLabel::native("Area", "Fläche")).required(),
                ActionArgDef::number("iy", LocalizedLabel::native("Second Moment of Area", "Flächenträgheitsmoment")).required(),
            ])
            .mutation("addSupport", LocalizedLabel::native("Add Support", "Lager hinzufügen"))
            // 🛡️ `fixed` is `Vec<FemDof>`; no `ActionArgDef` control maps to a typed enum list, so the
            // staged form takes the separator-delimited spelling `fem2d_dofs` reads (`"tx,ty"`).
            .action_args("addSupport", vec![
                ActionArgDef::text("nodeId", LocalizedLabel::native("Node", "Knoten")).required(),
                ActionArgDef::text("fixed", LocalizedLabel::native("Fixed Degrees of Freedom", "Gesperrte Freiheitsgrade")).default_value(&"tx,ty"),
            ])
            .mutation("addNodalLoad", LocalizedLabel::native("Add Nodal Load", "Knotenlast hinzufügen"))
            .action_args("addNodalLoad", vec![
                ActionArgDef::text("nodeId", LocalizedLabel::native("Node", "Knoten")).required(),
                ActionArgDef::select("dof", LocalizedLabel::native("Degree of Freedom", "Freiheitsgrad"), vec![
                    ActionArgOption::new("tx", LocalizedLabel::native("Tx", "Tx")),
                    ActionArgOption::new("ty", LocalizedLabel::native("Ty", "Ty")),
                    ActionArgOption::new("rz", LocalizedLabel::native("Rz", "Rz")),
                ])
                .default_value(&"ty"),
                ActionArgDef::number("value", LocalizedLabel::native("Value", "Wert")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addMemberUdl", LocalizedLabel::native("Add Member UDL", "Streckenlast hinzufügen"))
            .action_args("addMemberUdl", vec![
                ActionArgDef::text("elementId", LocalizedLabel::native("Element", "Element")).required(),
                ActionArgDef::number("wx", LocalizedLabel::native("Wx", "Wx")).default_value(&0.0),
                ActionArgDef::number("wy", LocalizedLabel::native("Wy", "Wy")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addAreaLoad", LocalizedLabel::native("Add Area Load", "Flächenlast hinzufügen"))
            .action_args("addAreaLoad", vec![
                ActionArgDef::text("regionId", LocalizedLabel::native("Region", "Bereich")).required(),
                ActionArgDef::number("pressure", LocalizedLabel::native("Pressure", "Druck")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addRegion", LocalizedLabel::native("Add Region", "Bereich hinzufügen"))
            .action_args("addRegion", vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required(),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required(),
                ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required(),
                ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::native("Material", "Material")).required(),
                ActionArgDef::number("thickness", LocalizedLabel::native("Thickness", "Dicke")).default_value(&0.02),
                ActionArgDef::number("meshSize", LocalizedLabel::native("Mesh Size", "Netzgröße")).default_value(&0.25),
            ])
            .mutation("addLoadCase", LocalizedLabel::native("Add Load Case", "Lastfall hinzufügen"))
            .action_args("addLoadCase", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::toggle("selfWeight", LocalizedLabel::native("Self Weight", "Eigengewicht")).default_value(&false),
            ])
            // 🎯️ `terms` is `Fem2dCommand::AddCombination`'s typed `Vec<FemCombinationTerm>` — no single
            // `ActionArgDef` control maps to that shape, so the staged form declares `name` only and the
            // action bridge opens an empty combination the term rows are added to afterwards.
            .mutation("addCombination", LocalizedLabel::native("Add Combination", "Kombination hinzufügen"))
            .action_args("addCombination", vec![ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required()])
            .mutation("setSelfWeight", LocalizedLabel::native("Set Self Weight", "Eigengewicht festlegen"))
            .action_args("setSelfWeight", vec![
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")).required(),
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).required(),
            ])
            .mutation("setAnalysisSettings", LocalizedLabel::native("Set Analysis Settings", "Analyseeinstellungen festlegen"))
            .action_args("setAnalysisSettings", vec![
                ActionArgDef::number("modalCount", LocalizedLabel::native("Modal Count", "Anzahl Eigenformen")),
                ActionArgDef::number("bucklingCount", LocalizedLabel::native("Buckling Count", "Anzahl Knickformen")),
                ActionArgDef::number("deformationScale", LocalizedLabel::native("Deformation Scale", "Verformungsmaßstab")),
            ])
            .mutation("removeSelection", LocalizedLabel::native("Remove Selection", "Auswahl entfernen"))
            .action_destructive("removeSelection")
            .action_with(semio_framework_plugin::ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), semio_framework_plugin::ActionKind::View, "camera"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), semio_framework_plugin::ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            // 📚️ The option id is the bundled example's own `ExampleSource` id, because the shell's
            // navbar switcher dispatches `setActiveExample { exampleId }` straight from
            // `PluginManifest.examples` (`ShellHost`'s `dispatchActiveExample`) — a select option that
            // did not match that id could never be reached from the switcher.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new(
                    semio_s_artifact_fem_2d_io::examples::demo::ID,
                    semio_s_artifact_fem_2d_io::examples::demo::label(),
                )])
                .default_value(&semio_s_artifact_fem_2d_io::examples::demo::ID),
            ])
            .view_action("setResultDisplay", LocalizedLabel::native("Set Result Display", "Ergebnisanzeige festlegen"))
            .action_args("setResultDisplay", semio_s_fem_app_surface::result_display_action_args())
            // 🖱️ Viewport gesture vocabulary — the Canvas2d host emits these; picks become framework
            // `interactionSelect`/`interactionHover` requests, never a document mutation.
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), semio_framework_plugin::ActionKind::View, "mouse-pointer-click") })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegen"), semio_framework_plugin::ActionKind::View, "mouse-pointer") })
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), semio_framework_plugin::ActionKind::View, "mouse-pointer") })
            // 🩹️ Inspector field edits — one bounded edit per dispatch on one entity.
            .action_with(fem2d_patch_action("patchNode", LocalizedLabel::native("Patch Node", "Knoten ändern")))
            .action_with(fem2d_patch_action("patchElement", LocalizedLabel::native("Patch Element", "Element ändern")))
            .action_with(fem2d_patch_action("patchMaterial", LocalizedLabel::native("Patch Material", "Material ändern")))
            .action_with(fem2d_patch_action("patchSection", LocalizedLabel::native("Patch Section", "Querschnitt ändern")))
            .action_with(fem2d_patch_action("patchSupport", LocalizedLabel::native("Patch Support", "Lager ändern")))
            .action_with(fem2d_patch_action("patchRegion", LocalizedLabel::native("Patch Region", "Bereich ändern")))
            .action_with(fem2d_patch_action("patchLoad", LocalizedLabel::native("Patch Load", "Last ändern")))
            .action_with(fem2d_patch_action("patchLoadCase", LocalizedLabel::native("Patch Load Case", "Lastfall ändern")))
            .action_with(fem2d_patch_action("patchCombination", LocalizedLabel::native("Patch Combination", "Kombination ändern")))
            // ⏯️ Deformation playback — results-window view state, self re-armed through `DispatchAction`.
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
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("addBar", InteractiveJobClassification::Migrated)
            .action_interactive_job("addBeam", InteractiveJobClassification::Migrated)
            .action_interactive_job("addMaterial", InteractiveJobClassification::Migrated)
            .action_interactive_job("addSection", InteractiveJobClassification::Migrated)
            .action_interactive_job("addSupport", InteractiveJobClassification::Migrated)
            .action_interactive_job("addNodalLoad", InteractiveJobClassification::Migrated)
            .action_interactive_job("addMemberUdl", InteractiveJobClassification::Migrated)
            .action_interactive_job("addAreaLoad", InteractiveJobClassification::Migrated)
            .action_interactive_job("addRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("addLoadCase", InteractiveJobClassification::Migrated)
            .action_interactive_job("addCombination", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelfWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAnalysisSettings", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setResultDisplay", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchElement", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchMaterial", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchSection", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchSupport", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchRegion", InteractiveJobClassification::Migrated)
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
            // 🧰️ Canvas selection utilities — rectangle/lasso marquee and direct pick, shared with draw/note.
            .utility(fem2d_utility("selectDirect", LocalizedLabel::native("Direct Select", "Direktauswahl"), "mouse-pointer-2", UtilityCategory::Selection))
            .utility(fem2d_utility("selectMarquee", LocalizedLabel::native("Marquee Select", "Rahmenauswahl"), "square-dashed", UtilityCategory::Selection))
            .utility(fem2d_utility("selectLasso", LocalizedLabel::native("Lasso Select", "Lasso-Auswahl"), "lasso", UtilityCategory::Selection))
            .utility(fem2d_utility("transformMove", LocalizedLabel::native("Pan", "Schwenken"), "move", UtilityCategory::Utilities))
            .utility(UtilityDefinition { group: Some("Transform".into()), category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("transform", LocalizedLabel::native("Transform", "Transformieren"), "move-3d") })
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .action_args("translateSelection", vec![
                ActionArgDef::number("dx", LocalizedLabel::native("Dx", "Dx")).required(),
                ActionArgDef::number("dy", LocalizedLabel::native("Dy", "Dy")).required(),
            ])
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .action_args("rotateSelection", vec![ActionArgDef::number("angle", LocalizedLabel::native("Angle", "Winkel")).required()])
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .action_args("scaleSelection", vec![
                ActionArgDef::number("sx", LocalizedLabel::native("Sx", "Sx")).required(),
                ActionArgDef::number("sy", LocalizedLabel::native("Sy", "Sy")).required(),
            ])
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::new("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transform-Griff festlegen"), semio_framework_plugin::ActionKind::View, "settings-2") })
            .action_args("setTransformGumballFlag", vec![
                ActionArgDef::text("flag", LocalizedLabel::native("Flag", "Flagge")).required(),
                ActionArgDef::toggle("pressed", LocalizedLabel::native("Pressed", "Gedrückt")),
            ])
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", InteractiveJobClassification::Migrated)
            .window_kind_utilities(model_window::WINDOW_KIND_ID, FEM2D_CANVAS_UTILITIES.iter().map(|id| (*id).into()).collect())
            .window_kind_utilities(results_window::WINDOW_KIND_ID, FEM2D_CANVAS_UTILITIES.iter().map(|id| (*id).into()).collect())
            // 🕹️ Framework-owned hover/selection: one domain over every document entity kind, bound to
            // both Canvas2d windows; interactionSelect/interactionHover/clearSelection/selectAll auto-inject.
            .interaction(fem2d_interaction_definition())
            .window_kind_interactions(model_window::WINDOW_KIND_ID, vec![InteractionRef::new(FEM2D_INTERACTION_DOMAIN)])
            .window_kind_interactions(results_window::WINDOW_KIND_ID, vec![InteractionRef::new(FEM2D_INTERACTION_DOMAIN)])
            .panel_tab_def(artifact_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .panel_tab_def(results_panel::definition())
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("delete", "removeSelection")
            .keybinding("backspace", "removeSelection")
            .keybinding("space", "setResultAnimation")
            // 🎯️ Typed channel surface — `config_spec()`/`fem2d_io()` are this same information's single
            // source of truth, reused here rather than duplicated.
            .config(Fem2dPlayApp::config_spec())
            .io(fem2d_io())
            .action_describe("addNode", LocalizedLabel::native("Adds a node at x, y to the 2D structural model; elements, supports and loads attach to nodes.", "Fügt dem 2D-Tragwerksmodell einen Knoten an x, y hinzu; Elemente, Lager und Lasten hängen an Knoten."))
            .action_describe("addBeam", LocalizedLabel::native("Adds a beam (a bending member) between two existing nodes of the 2D model, with a material and a section.", "Fügt zwischen zwei vorhandenen Knoten des 2D-Modells einen Balken (ein biegebeanspruchtes Glied) mit Material und Querschnitt hinzu."))
            .action_describe("addRegion", LocalizedLabel::native("Adds a rectangular plate region at x, y with a width and height, material, thickness and mesh size, meshed into finite elements.", "Fügt einen rechteckigen Scheibenbereich an x, y mit Breite, Höhe, Material, Dicke und Netzweite hinzu, der in finite Elemente vernetzt wird."))
            .action_describe("patchRegion", LocalizedLabel::native("Sets one field of one plate region by id, such as its size, thickness or mesh size.", "Setzt ein Feld eines Scheibenbereichs anhand seiner Id, etwa Größe, Dicke oder Netzweite."))
            .action_describe("addBar", LocalizedLabel::native("Adds a bar (an axial-only truss member) between two existing nodes of the 2D model, with a material and a section.", "Fügt zwischen zwei vorhandenen Knoten des 2D-Modells einen Stab (ein nur normalkraftbeanspruchtes Fachwerkglied) mit Material und Querschnitt hinzu."))
            .action_describe("addMaterial", LocalizedLabel::native("Adds a named material with its Young's modulus (and, in 3D, its shear modulus) for elements to use.", "Fügt ein benanntes Material mit seinem Elastizitätsmodul (in 3D auch Schubmodul) hinzu, das Elemente verwenden können."))
            .action_describe("addSection", LocalizedLabel::native("Adds a named cross-section with its area and second moment of area for bars and beams to use.", "Fügt einen benannten Querschnitt mit Fläche und Flächenträgheitsmoment hinzu, den Stäbe und Balken verwenden können."))
            .action_describe("addSupport", LocalizedLabel::native("Adds a support to one node, fixing the given degrees of freedom (translations and rotations) against movement.", "Fügt einem Knoten ein Lager hinzu, das die angegebenen Freiheitsgrade (Verschiebungen und Verdrehungen) festhält."))
            .action_describe("addNodalLoad", LocalizedLabel::native("Adds a point load of the given value on one degree of freedom of a node, in a load case.", "Fügt eine Einzellast mit dem angegebenen Wert auf einen Freiheitsgrad eines Knotens in einem Lastfall hinzu."))
            .action_describe("addMemberUdl", LocalizedLabel::native("Adds a uniformly distributed load (wx along, wy across) on one bar or beam element, in a load case.", "Fügt eine gleichmäßig verteilte Streckenlast (wx längs, wy quer) auf ein Stab- oder Balkenelement in einem Lastfall hinzu."))
            .action_describe("addAreaLoad", LocalizedLabel::native("Adds a uniform pressure load on one plate region, in a load case.", "Fügt eine gleichmäßige Flächenlast (Druck) auf einen Scheibenbereich in einem Lastfall hinzu."))
            .action_describe("addLoadCase", LocalizedLabel::native("Adds a named load case that loads are assigned to, optionally including the structure's self weight.", "Fügt einen benannten Lastfall hinzu, dem Lasten zugeordnet werden, optional mit dem Eigengewicht des Tragwerks."))
            .action_describe("addCombination", LocalizedLabel::native("Adds a named load combination that sums weighted load cases for the analysis.", "Fügt eine benannte Lastfallkombination hinzu, die gewichtete Lastfälle für die Analyse überlagert."))
            .action_describe("setSelfWeight", LocalizedLabel::native("Turns the structure's self weight on or off in one load case.", "Schaltet das Eigengewicht des Tragwerks in einem Lastfall ein oder aus."))
            .action_describe("setAnalysisSettings", LocalizedLabel::native("Sets how many vibration modes and buckling modes the analysis computes and the scale deformations are drawn with.", "Legt fest, wie viele Eigenschwingungs- und Knickformen die Analyse berechnet und mit welchem Maßstab Verformungen gezeichnet werden."))
            .action_describe("removeSelection", LocalizedLabel::native("Deletes every selected node, element, support or load from the model, together with what depends on it.", "Löscht alle ausgewählten Knoten, Elemente, Lager oder Lasten samt allem, was davon abhängt, aus dem Modell."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 2D structural model with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte 2D-Tragwerksmodell durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
            .action_describe("setResultDisplay", LocalizedLabel::native("Chooses which analysis result the view shows: the static result of a load case or combination, or a numbered vibration or buckling mode; the model is not changed.", "Wählt, welches Analyseergebnis die Ansicht zeigt: das statische Ergebnis eines Lastfalls oder einer Kombination oder eine nummerierte Eigen- oder Knickform; das Modell ändert sich nicht."))
            .action_describe("patchNode", LocalizedLabel::native("Sets one field of one node by id, such as a coordinate.", "Setzt ein Feld eines Knotens anhand seiner Id, etwa eine Koordinate."))
            .action_describe("patchElement", LocalizedLabel::native("Sets one field of one bar or beam element by id, such as its material, section or end nodes.", "Setzt ein Feld eines Stab- oder Balkenelements anhand seiner Id, etwa Material, Querschnitt oder Endknoten."))
            .action_describe("patchMaterial", LocalizedLabel::native("Sets one field of one material by id, such as its name or Young's modulus.", "Setzt ein Feld eines Materials anhand seiner Id, etwa Name oder Elastizitätsmodul."))
            .action_describe("patchSection", LocalizedLabel::native("Sets one field of one cross-section by id, such as its area or second moment of area.", "Setzt ein Feld eines Querschnitts anhand seiner Id, etwa Fläche oder Flächenträgheitsmoment."))
            .action_describe("patchSupport", LocalizedLabel::native("Sets one field of one support by id, such as which degrees of freedom it fixes.", "Setzt ein Feld eines Lagers anhand seiner Id, etwa welche Freiheitsgrade es festhält."))
            .action_describe("patchLoad", LocalizedLabel::native("Sets one field of one load by id, such as its value, direction or load case.", "Setzt ein Feld einer Last anhand ihrer Id, etwa Wert, Richtung oder Lastfall."))
            .action_describe("patchLoadCase", LocalizedLabel::native("Sets one field of one load case by id, such as its name or whether it includes self weight.", "Setzt ein Feld eines Lastfalls anhand seiner Id, etwa Name oder ob er das Eigengewicht enthält."))
            .action_describe("patchCombination", LocalizedLabel::native("Sets one field of one load combination by id, such as its name or the factors of its load cases.", "Setzt ein Feld einer Lastfallkombination anhand ihrer Id, etwa Name oder die Faktoren ihrer Lastfälle."))
            .action_describe("translateSelection", LocalizedLabel::native("Moves the selected nodes by dx and dy, dragging the attached elements along.", "Verschiebt die ausgewählten Knoten um dx und dy und zieht die angeschlossenen Elemente mit."))
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the selected nodes by the given angle around their common centre.", "Dreht die ausgewählten Knoten um den angegebenen Winkel um ihren gemeinsamen Mittelpunkt."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the positions of the selected nodes by sx and sy about their common centre.", "Skaliert die Positionen der ausgewählten Knoten um sx und sy um ihren gemeinsamen Mittelpunkt."))
            .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
            .build_definition()
}
//#endregion 🔖️Manifest
