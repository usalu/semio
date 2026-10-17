//! 📋️ Lowpoly manifest: the utility declarations and the app manifest stitch of `create_lowpoly_app`.

use super::*;

//#region 🔖️Manifest
/// 🧰️ One transform/paint utility declaration (id/label/icon reused verbatim from the retired
/// `utilities()` impl).
fn lowpoly_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new(id, label, icon) }
}

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding/utility declarations (which have no dedicated `_def` passthrough)
/// stay written out inline.
///
/// 🚧️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4: `EditorBuilder` has no
/// `.example(...)`/`.workflow(...)` methods (`App { definition, examples }` split — `.editor::<E>(def)`
/// only takes the definition, examples always end up empty). The old
/// `.example("default", …, &default_example, "file")` / `.workflow("lowpoly", "Lowpoly", "mesh")` tail
/// calls this app used to make are DROPPED here, not ported — the subset's own `📚️examples/🌲️hexagonal-cut-concrete-forest-left`
/// facet is the intended replacement mechanism per the pilot's report, not confirmed with the
/// coordinator by this packet.
pub fn create_lowpoly_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(semio_s_artifact_lowpoly_lowpoly_model::snapshot::LOWPOLY_DIALECT)
            .document(["semio", "lowpoly"])
            .artifact_kind(artifact_kind())
            .icon_id("shapes")
            .mode_def(edit::definition())
            .mode_def(paint_mode::definition())
            .default_mode_id(edit::LOWPOLY_PLAY_MODE_EDIT)
            .window_kind_def(edit::windows::model::definition())
            .window_kind_def(paint_mode::windows::uv::definition())
            .window_kind_action_refs(
                edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN,
                edit::windows::model::LOWPOLY_MAIN_ACTIONS.iter().map(|id| ActionRef::from(*id)).collect(),
            )
            .window_kind_action_refs(
                paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV,
                paint_mode::windows::uv::LOWPOLY_UV_ACTIONS.iter().map(|id| ActionRef::from(*id)).collect(),
            )
            .default_layout(edit::layout())
            .named_layout(paint_mode::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .panel_tab_def(layers_panel::definition())
            // 🔧️ Document-mutating operations — dispatched as VCS operations with true inverses.
            .mutation("addPrimitive", LocalizedLabel::native("Add Primitive", "Primitive hinzufügen"))
            .mutation("patchObject", LocalizedLabel::native("Patch Object", "Objekt aktualisieren"))
            .mutation("extrude", LocalizedLabel::native("Extrude", "Extrudieren"))
            .mutation("inset", LocalizedLabel::native("Inset", "Einziehen"))
            .mutation("bevel", LocalizedLabel::native("Bevel", "Fasen"))
            .mutation("loopCut", LocalizedLabel::native("Loop Cut", "Schleifenschnitt"))
            .mutation("subdivide", LocalizedLabel::native("Subdivide", "Unterteilen"))
            .mutation("triangulate", LocalizedLabel::native("Triangulate", "Triangulieren"))
            .mutation("mirror", LocalizedLabel::native("Mirror", "Spiegeln"))
            .mutation("decimate", LocalizedLabel::native("Decimate", "Dezimieren"))
            .mutation("flipFaces", LocalizedLabel::native("Flip Faces", "Flächen umkehren"))
            .mutation("merge", LocalizedLabel::native("Merge", "Zusammenführen"))
            .mutation("dissolve", LocalizedLabel::native("Dissolve", "Auflösen"))
            .mutation("snap", LocalizedLabel::native("Snap", "Einrasten"))
            .mutation("toggleSmooth", LocalizedLabel::native("Toggle Smooth", "Glättung umschalten"))
            .mutation("unwrapActive", LocalizedLabel::native("Unwrap", "Abwickeln"))
            .mutation("markUvSeam", LocalizedLabel::native("Mark Seam", "Naht markieren"))
            .mutation("clearSeam", LocalizedLabel::native("Clear Seam", "Naht entfernen"))
            .action_destructive("clearSeam")
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .mutation("addPaintLayer", LocalizedLabel::native("Add Paint Layer", "Malebene hinzufügen"))
            .mutation("paintFill", LocalizedLabel::native("Paint Fill", "Füllen malen"))
            .mutation("fillBucket", LocalizedLabel::native("Fill Bucket", "Fülleimer"))
            .mutation("importSnapshotJson", LocalizedLabel::native("Import Snapshot Json", "Snapshot-JSON importieren"))
            .mutation("replaceSnapshotJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"))
            // 📤️ Shell effects: a mesh download, a file-open request, and the import the shell answers it with.
            .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .action_destructive("deleteSelection")
            .mutation("duplicateObject", LocalizedLabel::native("Duplicate Object", "Objekt duplizieren"))
            .shell_action("exportMesh", LocalizedLabel::native("Export Mesh", "Mesh exportieren"))
            .shell_action("loadMeshRequest", LocalizedLabel::native("Load Mesh…", "Mesh laden…"))
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("importMeshFile", LocalizedLabel::native("Import Mesh File", "Mesh-Datei importieren"), semio_framework_plugin::ActionKind::Mutation) })
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            // 👁️ Ephemeral view state — selection, camera, hover and the eyedropper sample.
            .view_action("setActiveObject", LocalizedLabel::native("Set Active Object", "Aktives Objekt festlegen"))
            .view_action("setActivePaintLayer", LocalizedLabel::native("Set Active Paint Layer", "Aktive Malebene festlegen"))
            .view_action("setUtilityParam", LocalizedLabel::native("Set Utility Param", "Werkzeugparameter festlegen"))
            .action_with(semio_framework_plugin::ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), semio_framework_plugin::ActionKind::View, "hand"))
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .view_action("toggleShowEdges", LocalizedLabel::native("Toggle Show Edges", "Kantenanzeige umschalten"))
            .action_with(semio_framework_plugin::ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), semio_framework_plugin::ActionKind::View, "sun"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), semio_framework_plugin::ActionKind::View, "sun"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), semio_framework_plugin::ActionKind::View, "sun"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), semio_framework_plugin::ActionKind::View, "sun"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), semio_framework_plugin::ActionKind::View, "camera"))
            .action_with(semio_framework_plugin::ActionDefinition::new("paintStroke", LocalizedLabel::native("Paint Stroke", "Malstrich"), semio_framework_plugin::ActionKind::Mutation, "paintbrush"))
            .action_with(semio_framework_plugin::ActionDefinition::new("paintAt", LocalizedLabel::native("Paint At", "Malen bei"), semio_framework_plugin::ActionKind::Mutation, "paintbrush"))
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), semio_framework_plugin::ActionKind::Mutation, "mouse-pointer"))
            .action_audience("canvasPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"), semio_framework_plugin::ActionKind::Mutation, "mouse-pointer"))
            .action_audience("canvasPointerMove", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), semio_framework_plugin::ActionKind::Mutation, "mouse-pointer"))
            .action_audience("canvasPointerUp", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("paintSample", LocalizedLabel::native("Paint Sample", "Farbe aufnehmen"), semio_framework_plugin::ActionKind::View, "paintbrush"))
            // 📝️ Staged argument forms for the P1 actions — the panel form seeds from these defaults and
            // stages typed overrides read out of `args`; `config.utility_params_json` remains the live backing store.
            .action_args("extrude", vec![ActionArgDef::slider("extrudeDistance", LocalizedLabel::native("Extrude Distance", "Extrusionsabstand"), 0.01, 2.0).default_value(&0.25)])
            .action_args("inset", vec![ActionArgDef::number("insetAmount", LocalizedLabel::native("Inset Amount", "Einzugsbetrag")).default_value(&0.1)])
            // 🧲️ Numeric transform entry from the Actions pane — the gumball drag fills the same fields.
            .action_args("translateSelection", vec![
                ActionArgDef::number("dx", LocalizedLabel::native("Move X", "Verschieben X")).default_value(&0.0),
                ActionArgDef::number("dy", LocalizedLabel::native("Move Y", "Verschieben Y")).default_value(&0.0),
                ActionArgDef::number("dz", LocalizedLabel::native("Move Z", "Verschieben Z")).default_value(&0.0),
            ])
            .action_args("rotateSelection", vec![
                ActionArgDef::number("angle", LocalizedLabel::native("Angle", "Winkel")).default_value(&0.0),
                ActionArgDef::number("ax", LocalizedLabel::native("Axis X", "Achse X")).default_value(&0.0),
                ActionArgDef::number("ay", LocalizedLabel::native("Axis Y", "Achse Y")).default_value(&1.0),
                ActionArgDef::number("az", LocalizedLabel::native("Axis Z", "Achse Z")).default_value(&0.0),
            ])
            .action_args("scaleSelection", vec![
                ActionArgDef::number("sx", LocalizedLabel::native("Scale X", "Skalieren X")).default_value(&1.0),
                ActionArgDef::number("sy", LocalizedLabel::native("Scale Y", "Skalieren Y")).default_value(&1.0),
                ActionArgDef::number("sz", LocalizedLabel::native("Scale Z", "Skalieren Z")).default_value(&1.0),
            ])
            .action_args("exportMesh", vec![ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![
                ActionArgOption::new("obj", LocalizedLabel::native("OBJ", "OBJ")),
                ActionArgOption::new("ply", LocalizedLabel::native("PLY", "PLY")),
                ActionArgOption::new("stl", LocalizedLabel::native("STL", "STL")),
            ]).required().default_value(&"obj")])
            .action_args("bevel", vec![
                ActionArgDef::number("bevelAmount", LocalizedLabel::native("Bevel Amount", "Fasenbetrag")).default_value(&0.05),
                ActionArgDef::number("bevelSegments", LocalizedLabel::native("Bevel Segments", "Fasensegmente")).default_value(&1),
            ])
            .action_args("loopCut", vec![ActionArgDef::number("loopCuts", LocalizedLabel::native("Loop Cuts", "Schleifenschnitte")).default_value(&1)])
            .action_args("decimate", vec![ActionArgDef::slider("decimateRatio", LocalizedLabel::native("Decimate Ratio", "Dezimierungsverhältnis"), 0.05, 1.0).default_value(&0.5)])
            .action_args("mirror", vec![ActionArgDef::select("axis", LocalizedLabel::native("Axis", "Achse"), vec![
                ActionArgOption::new("x", LocalizedLabel::native("X", "X")),
                ActionArgOption::new("y", LocalizedLabel::native("Y", "Y")),
                ActionArgOption::new("z", LocalizedLabel::native("Z", "Z")),
            ]).default_value(&"x")])
            .action_args("addPrimitive", vec![ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                ActionArgOption::new("box", LocalizedLabel::native("Cube", "Würfel")),
                ActionArgOption::new("plane", LocalizedLabel::native("Plane", "Ebene")),
                ActionArgOption::new("cylinder", LocalizedLabel::native("Cylinder", "Zylinder")),
                ActionArgOption::new("cone", LocalizedLabel::native("Cone", "Kegel")),
                ActionArgOption::new("ico_sphere", LocalizedLabel::native("Ico Sphere", "Ikokugel")),
            ]).default_value(&"box")])
            .action_args("markUvSeam", vec![ActionArgDef::toggle("seam", LocalizedLabel::native("Seam", "Naht")).default_value(&true)])
            // 🧰️ Transform gumball + paint utilities — exclusive per-window active utility is host-owned (never a
            // document operation). Selection method/merge/kind live as an always-visible Select window-options group
            // (mirrors puzzle 3d); the transform group defaults to "move", paint bridges into `config.paint_utility`.
            .utility(lowpoly_utility("move", LocalizedLabel::native("Move", "Verschieben"), "move", "transform"))
            .utility(lowpoly_utility("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw", "transform"))
            .utility(lowpoly_utility("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2", "transform"))
            .utility(lowpoly_utility("brush", LocalizedLabel::native("Brush", "Pinsel"), "paintbrush", "paint"))
            .utility(lowpoly_utility("eraser", LocalizedLabel::native("Eraser", "Radierer"), "eraser", "paint"))
            .utility(lowpoly_utility("fill", LocalizedLabel::native("Fill", "Füllen"), "paint-bucket", "paint"))
            .utility(lowpoly_utility("eyedropper", LocalizedLabel::native("Eyedropper", "Pipette"), "pipette", "paint"))
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "mesh" interaction domain —
            // object/vertex/edge/face granularities (u32 component ids stringify at the `InteractionTarget`
            // boundary, see `🧭️view/🦀️.rs`'s `🔖️MeshDomain` region), Flat hierarchy (a mesh has no
            // parent/child structure of its own to derive a topology from), all five merges + all three
            // pick methods per the migration's acceptance bar. Scoped to the Model window only — the UV
            // window paints textures, it never selects mesh components.
            .interaction(InteractionDefinition {
                id: MESH_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Mesh", "Netz"),
                granularities: vec![
                    GranularityDefinition { id: "object".into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "box".into() },
                    GranularityDefinition { id: "vertex".into(), label: LocalizedLabel::native("Vertex", "Eckpunkt"), icon_id: "circle".into() },
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
            })
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // ⌨️ The modeller's hands: delete what is selected, duplicate the active object, extrude/inset it.
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("mod+d", "duplicateObject")
            .keybinding("e", "extrude")
            .keybinding("i", "inset")
            .config(LowpolyPlayApp::config_spec())
            .io(lowpoly_io())
            .action_interactive_job("addPrimitive", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchObject", InteractiveJobClassification::Migrated)
            .action_interactive_job("extrude", InteractiveJobClassification::Migrated)
            .action_interactive_job("inset", InteractiveJobClassification::Migrated)
            .action_interactive_job("bevel", InteractiveJobClassification::Migrated)
            .action_interactive_job("loopCut", InteractiveJobClassification::Migrated)
            .action_interactive_job("subdivide", InteractiveJobClassification::Migrated)
            .action_interactive_job("triangulate", InteractiveJobClassification::Migrated)
            .action_interactive_job("mirror", InteractiveJobClassification::Migrated)
            .action_interactive_job("decimate", InteractiveJobClassification::Migrated)
            .action_interactive_job("flipFaces", InteractiveJobClassification::Migrated)
            .action_interactive_job("merge", InteractiveJobClassification::Migrated)
            .action_interactive_job("dissolve", InteractiveJobClassification::Migrated)
            .action_interactive_job("snap", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSmooth", InteractiveJobClassification::Migrated)
            .action_interactive_job("unwrapActive", InteractiveJobClassification::Migrated)
            .action_interactive_job("markUvSeam", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearSeam", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("addPaintLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("paintFill", InteractiveJobClassification::Migrated)
            .action_interactive_job("fillBucket", InteractiveJobClassification::Migrated)
            .action_interactive_job("importSnapshotJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("replaceSnapshotJson", InteractiveJobClassification::Migrated)
            .action_destructive("replaceSnapshotJson")
            .action_interactive_job("exportMesh", InteractiveJobClassification::Migrated)
            .action_destructive("exportMesh")
            .action_interactive_job("loadMeshRequest", InteractiveJobClassification::Migrated)
            .action_interactive_job("importMeshFile", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateObject", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveObject", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActivePaintLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("setUtilityParam", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleShowEdges", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("paintSample", InteractiveJobClassification::Migrated)
            .action_interactive_job("paintStroke", InteractiveJobClassification::Migrated)
            .action_interactive_job("paintAt", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
            .action_describe("addPrimitive", LocalizedLabel::native("Adds a new primitive mesh object of the given kind (a box, sphere, cylinder, cone or plane) to the scene and makes it the active object.", "Fügt der Szene ein neues Grundkörper-Netzobjekt der angegebenen Art (Quader, Kugel, Zylinder, Kegel oder Ebene) hinzu und macht es zum aktiven Objekt."))
            .action_describe("patchObject", LocalizedLabel::native("Sets one scalar field of one object by id, its name or whether it is shaded smooth, from a JSON value.", "Setzt ein skalares Feld eines Objekts anhand seiner Id, Name oder ob es glatt schattiert wird, aus einem JSON-Wert."))
            .action_describe("extrude", LocalizedLabel::native("Extrudes the selected faces of the active object outwards by the given distance, creating new side faces.", "Extrudiert die ausgewählten Flächen des aktiven Objekts um den angegebenen Abstand nach außen und erzeugt neue Seitenflächen."))
            .action_describe("inset", LocalizedLabel::native("Insets the selected faces of the active object by the given amount, creating a smaller face inside each with a ring of faces around it.", "Zieht die ausgewählten Flächen des aktiven Objekts um den angegebenen Betrag ein und erzeugt in jeder eine kleinere Fläche mit einem Flächenring darum."))
            .action_describe("bevel", LocalizedLabel::native("Bevels the selected edges of the active object by the given amount with the given number of segments.", "Fast die ausgewählten Kanten des aktiven Objekts um den angegebenen Betrag mit der angegebenen Segmentanzahl."))
            .action_describe("loopCut", LocalizedLabel::native("Cuts the given number of edge loops across the selected faces of the active object.", "Schneidet die angegebene Anzahl von Kantenschleifen quer durch die ausgewählten Flächen des aktiven Objekts."))
            .action_describe("subdivide", LocalizedLabel::native("Subdivides the selected faces of the active object into smaller faces.", "Unterteilt die ausgewählten Flächen des aktiven Objekts in kleinere Flächen."))
            .action_describe("triangulate", LocalizedLabel::native("Splits the selected faces of the active object into triangles.", "Zerlegt die ausgewählten Flächen des aktiven Objekts in Dreiecke."))
            .action_describe("mirror", LocalizedLabel::native("Mirrors the active object's selected geometry across the given axis (x, y or z).", "Spiegelt die ausgewählte Geometrie des aktiven Objekts an der angegebenen Achse (x, y oder z)."))
            .action_describe("decimate", LocalizedLabel::native("Reduces the face count of the active object's selection to the given ratio, simplifying its shape.", "Reduziert die Flächenzahl der Auswahl des aktiven Objekts auf das angegebene Verhältnis und vereinfacht damit ihre Form."))
            .action_describe("flipFaces", LocalizedLabel::native("Flips the normals of the selected faces of the active object so they face the other way.", "Kehrt die Normalen der ausgewählten Flächen des aktiven Objekts um, sodass sie in die andere Richtung zeigen."))
            .action_describe("merge", LocalizedLabel::native("Merges the selected vertices of the active object into one.", "Verschmilzt die ausgewählten Punkte des aktiven Objekts zu einem."))
            .action_describe("dissolve", LocalizedLabel::native("Dissolves the selected elements of the active object, removing them while keeping the surrounding surface closed.", "Löst die ausgewählten Elemente des aktiven Objekts auf, entfernt sie also und hält die umgebende Fläche geschlossen."))
            .action_describe("snap", LocalizedLabel::native("Snaps the selected vertices of the active object to a grid with the snap tool's spacing.", "Rastet die ausgewählten Punkte des aktiven Objekts an einem Raster mit dem Abstand des Einrast-Werkzeugs ein."))
            .action_describe("toggleSmooth", LocalizedLabel::native("Switches the active object between smooth and flat shading.", "Schaltet das aktive Objekt zwischen glatter und flacher Schattierung um."))
            .action_describe("unwrapActive", LocalizedLabel::native("Computes a new UV unwrap of the active object along its marked seams, replacing its previous texture coordinates.", "Berechnet eine neue UV-Abwicklung des aktiven Objekts entlang seiner markierten Nähte und ersetzt die bisherigen Texturkoordinaten."))
            .action_describe("markUvSeam", LocalizedLabel::native("Marks (or with seam false unmarks) the given or selected edges of the active object as UV seams for the next unwrap.", "Markiert die angegebenen oder ausgewählten Kanten des aktiven Objekts als UV-Nähte für die nächste Abwicklung (mit seam false wird die Markierung entfernt)."))
            .action_describe("clearSeam", LocalizedLabel::native("Removes every UV seam mark from the active object.", "Entfernt alle UV-Nahtmarkierungen vom aktiven Objekt."))
            .action_describe("translateSelection", LocalizedLabel::native("Moves the mesh selection — the selected vertices, edges or faces of the active object, else every selected object — by dx, dy and dz as one undoable edit whose offset stays editable in history.", "Verschiebt die Netzauswahl — die ausgewählten Punkte, Kanten oder Flächen des aktiven Objekts, sonst jedes ausgewählte Objekt — um dx, dy und dz als eine rückgängig machbare Änderung, deren Versatz im Verlauf bearbeitbar bleibt."))
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the mesh selection by an angle in radians around the axis ax, ay, az through its centre as one undoable edit whose angle and axis stay editable in history.", "Dreht die Netzauswahl um einen Winkel im Bogenmaß um die Achse ax, ay, az durch ihre Mitte als eine rückgängig machbare Änderung, deren Winkel und Achse im Verlauf bearbeitbar bleiben."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the mesh selection by sx, sy and sz about its centre as one undoable edit whose factors stay editable in history.", "Skaliert die Netzauswahl um sx, sy und sz um ihre Mitte als eine rückgängig machbare Änderung, deren Faktoren im Verlauf bearbeitbar bleiben."))
            .action_describe("addPaintLayer", LocalizedLabel::native("Adds a new, named paint layer to the given or active object's texture.", "Fügt der Textur des angegebenen oder aktiven Objekts eine neue, benannte Malebene hinzu."))
            .action_describe("paintAt", LocalizedLabel::native("Paints one dab of the active brush at the texture point u, v of the given or active object; a host drag streams its dabs into one stroke that the release writes as one undoable edit whose brush and dabs stay editable in history.", "Malt einen Tupfer des aktiven Pinsels am Texturpunkt u, v des angegebenen oder aktiven Objekts; ein Ziehen des Hosts sammelt seine Tupfer zu einem Strich, den das Loslassen als eine rückgängig machbare Änderung schreibt, deren Pinsel und Tupfer im Verlauf bearbeitbar bleiben."))
            .action_describe("paintFill", LocalizedLabel::native("Flood-fills the region of the active paint layer around the texture point u, v (the layer's centre when none is given) with the brush colour.", "Füllt den Bereich der aktiven Malebene um den Texturpunkt u, v (ohne Angabe die Ebenenmitte) mit der Pinselfarbe."))
            .action_describe("fillBucket", LocalizedLabel::native("Bucket-fills the connected area of the active paint layer at u, v (its centre when omitted) with the brush colour, exactly as Paint Fill does.", "Füllt die zusammenhängende Fläche der aktiven Malebene bei u, v (ohne Angabe ihre Mitte) mit der Pinselfarbe, genau wie Füllen malen."))
            .action_describe("exportMesh", LocalizedLabel::native("Writes the scene's meshes in the chosen format to a downloaded file on the user's machine.", "Schreibt die Netze der Szene im gewählten Format in eine heruntergeladene Datei auf dem Rechner des Nutzers."))
            .action_describe("loadMeshRequest", LocalizedLabel::native("Opens the host's file picker for a mesh file; the chosen file then replaces the whole document.", "Öffnet die Dateiauswahl des Hosts für eine Netzdatei; die gewählte Datei ersetzt dann das gesamte Dokument."))
            .action_describe("importMeshFile", LocalizedLabel::native("Replaces the whole document with the meshes read from an imported mesh file; the previous scene is discarded.", "Ersetzt das gesamte Dokument durch die Netze aus einer importierten Netzdatei; die bisherige Szene wird verworfen."))
            .action_describe("deleteSelection", LocalizedLabel::native("Deletes what the mesh selection holds: whole objects when objects are selected, otherwise the selected faces of the active object.", "Löscht, was die Netzauswahl enthält: ganze Objekte, wenn Objekte ausgewählt sind, sonst die ausgewählten Flächen des aktiven Objekts."))
            .action_describe("duplicateObject", LocalizedLabel::native("Copies the given or active object and adds the copy to the scene.", "Kopiert das angegebene oder aktive Objekt und fügt die Kopie der Szene hinzu."))
            .action_describe("importSnapshotJson", LocalizedLabel::native("Replaces the whole lowpoly document with one parsed from the given snapshot JSON; invalid JSON changes nothing.", "Ersetzt das gesamte Lowpoly-Dokument durch eines aus dem angegebenen Snapshot-JSON; ungültiges JSON ändert nichts."))
            .action_describe("replaceSnapshotJson", LocalizedLabel::native("Loads a test fixture given as JSON as the whole lowpoly document, replacing the current scene; invalid JSON changes nothing.", "Lädt eine als JSON übergebene Test-Fixture als gesamtes Lowpoly-Dokument und ersetzt die aktuelle Szene; ungültiges JSON ändert nichts."))
            .action_describe("setActiveObject", LocalizedLabel::native("Makes the object with the given id the active one that mesh edits apply to; only the editor's view state changes.", "Macht das Objekt mit der angegebenen Id zum aktiven, auf das Netzänderungen wirken; nur der Ansichtszustand des Editors ändert sich."))
            .action_describe("setActivePaintLayer", LocalizedLabel::native("Makes the given paint layer the one painting writes to; only the editor's view state changes.", "Macht die angegebene Malebene zu der, auf die gemalt wird; nur der Ansichtszustand des Editors ändert sich."))
            .action_describe("setUtilityParam", LocalizedLabel::native("Sets one parameter of the armed tool (such as extrude distance or brush size) from a JSON value; only the tool setting changes.", "Setzt einen Parameter des gewählten Werkzeugs (etwa Extrusionsabstand oder Pinselgröße) aus einem JSON-Wert; nur die Werkzeugeinstellung ändert sich."))
            .action_describe("toggleShowEdges", LocalizedLabel::native("Shows or hides the mesh edges in the model window; only the view changes.", "Blendet die Netzkanten im Modellfenster ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("toggleSun", LocalizedLabel::native("Switches the model window's sun light on or off; only the view changes.", "Schaltet das Sonnenlicht des Modellfensters ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("setSunAzimuth", LocalizedLabel::native("Sets the compass direction the model window's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne des Modellfensters scheint; nur die Ansicht ändert sich."))
            .action_describe("setSunElevation", LocalizedLabel::native("Sets how high the model window's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne des Modellfensters über dem Horizont steht; nur die Ansicht ändert sich."))
            .action_describe("setSunIntensity", LocalizedLabel::native("Sets the brightness of the model window's sun; only the view changes.", "Legt die Helligkeit der Sonne des Modellfensters fest; nur die Ansicht ändert sich."))
            .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("paintStroke", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("paintAt", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("paintSample", semio_framework_plugin::CapabilityAudience::Input)
            .action_destructive("importSnapshotJson")
            .action_destructive("importMeshFile")
            .build_definition()
}
//#endregion 🔖️Manifest
