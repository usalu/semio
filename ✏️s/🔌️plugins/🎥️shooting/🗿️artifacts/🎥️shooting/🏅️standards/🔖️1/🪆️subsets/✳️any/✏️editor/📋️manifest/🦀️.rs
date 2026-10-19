//! 📋️ Manifest of the shooting editor: the app manifest stitch of `create_shooting_app`. See [`super`].

use super::*;

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_shooting_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::SHOOTING_DIALECT)
            .document(["semio", "shooting"])
            .artifact_kind(crate::artifact_kind())
            .artifact_kind(semio_framework_plugin::ArtifactKindSpec {
                id: "2d.image".into(),
                label: LocalizedLabel::native("2D Image", "2D-Bild"),
                source_format: "2d.image".into(),
                component_kind: "image".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
                schema: "2d.image".into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec!["stdio.png".into()],
        import_stdio_kinds: vec!["stdio.png".into()],
    })
            .media_output(shooting_photos_out_port())
            .icon_id("camera")
            .mode_def(edit::definition())
            .default_mode_id(edit::SHOOTING_PLAY_MODE_EDIT)
            .window_kind_def(scene_window::definition())
            .window_kind_def(icon_window::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importSnapshotJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Mutation) })
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .mutation("setActiveShot", LocalizedLabel::native("Set Active Shot", "Aktive Aufnahme festlegen"))
            .mutation("setActiveAsset", LocalizedLabel::native("Set Active Asset", "Aktives Objekt festlegen"))
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .mutation("setShotCamera", LocalizedLabel::native("Set Shot Camera", "Aufnahmekamera festlegen"))
            .mutation("saveCamera", LocalizedLabel::native("Save Camera", "Kamera speichern"))
            .view_action("loadSavedCamera", LocalizedLabel::native("Load Saved Camera", "Gespeicherte Kamera laden"))
            .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::Mutation, "sun"))
            .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::Mutation, "sun"))
            .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::Mutation, "sun"))
            .mutation("setAmbientIntensity", LocalizedLabel::native("Set Ambient Intensity", "Umgebungslichtintensität festlegen"))
            .mutation("setMaterialRoughness", LocalizedLabel::native("Set Material Roughness", "Materialrauheit festlegen"))
            .mutation("setShadowEnabled", LocalizedLabel::native("Set Shadow Enabled", "Schatten aktivieren"))
            .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::Mutation, "sun"))
            .mutation("setActiveShotLabel", LocalizedLabel::native("Set Active Shot Label", "Bezeichnung der aktiven Aufnahme festlegen"))
            .mutation("setActiveShotFormat", LocalizedLabel::native("Set Active Shot Format", "Format der aktiven Aufnahme festlegen"))
            .mutation("setActiveShotShape", LocalizedLabel::native("Set Active Shot Shape", "Form der aktiven Aufnahme festlegen"))
            .mutation("patchShots", LocalizedLabel::native("Patch Shots", "Aufnahmen aktualisieren"))
            .mutation("patchAssets", LocalizedLabel::native("Patch Assets", "Objekte aktualisieren"))
            .mutation("addShot", LocalizedLabel::native("Add Shot", "Aufnahme hinzufügen"))
            .mutation("addAsset", LocalizedLabel::native("Add Asset", "Objekt hinzufügen"))
            .mutation("importAsset", LocalizedLabel::native("Import Asset", "Objekt importieren"))
            .mutation("resetSnapshot", LocalizedLabel::native("Reset Fixture", "Vorgabe zurücksetzen"))
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .view_action("setShotSelection", LocalizedLabel::native("Set Shot Selection", "Aufnahmeauswahl festlegen"))
            .view_action("setCenterModel", LocalizedLabel::native("Set Center Model", "Modellzentrierung festlegen"))
            .action_with(ActionDefinition::new("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .action_audience("worldPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .view_action("worldPointerMove", LocalizedLabel::native("World Pointer Move", "Welt-Zeiger bewegt"))
            .action_audience("worldPointerMove", semio_framework_plugin::CapabilityAudience::Input)
            .interaction(InteractionDefinition {
                id: SHOOTING_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Assets", "Objekte"),
                granularities: vec![GranularityDefinition { id: "asset".into(), label: LocalizedLabel::native("Asset", "Objekt"), icon_id: "box".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(SHOOTING_PLAY_WINDOW_SCENE, vec![InteractionRef::new(SHOOTING_INTERACTION_DOMAIN)])
            .shell_action("saveDownload", LocalizedLabel::native("Save Download", "Download speichern"))
            .shell_action("importAssetRequest", LocalizedLabel::native("Import Asset Request", "Objekt-Importanfrage"))
            .shell_action("exportActiveShot", LocalizedLabel::native("Export Active Shot", "Aktive Aufnahme exportieren"))
            .shell_action("exportAllShots", LocalizedLabel::native("Export All Shots", "Alle Aufnahmen exportieren"))
            .action_interactive_job("importSnapshotJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveShot", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveAsset", InteractiveJobClassification::Migrated)
            .action_interactive_job("setShotCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("saveCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAmbientIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setMaterialRoughness", InteractiveJobClassification::Migrated)
            .action_interactive_job("setShadowEnabled", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveShotLabel", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveShotFormat", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveShotShape", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchShots", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchAssets", InteractiveJobClassification::Migrated)
            .action_interactive_job("addShot", InteractiveJobClassification::Migrated)
            .action_interactive_job("addAsset", InteractiveJobClassification::Migrated)
            .action_interactive_job("importAsset", InteractiveJobClassification::Migrated)
            .action_interactive_job("resetSnapshot", InteractiveJobClassification::Migrated)
            .action_destructive("resetSnapshot")
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("loadSavedCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCenterModel", InteractiveJobClassification::Migrated)
            .action_interactive_job("setShotSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerMove", InteractiveJobClassification::Migrated)
            .action_interactive_job("saveDownload", InteractiveJobClassification::Migrated)
            .action_destructive("saveDownload")
            .action_interactive_job("importAssetRequest", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportActiveShot", InteractiveJobClassification::Migrated)
            .action_destructive("exportActiveShot")
            .action_interactive_job("exportAllShots", InteractiveJobClassification::Migrated)
            .action_destructive("exportAllShots")
            .action_args("addShot", vec![
                ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![ActionArgOption::new("svg", LocalizedLabel::native("SVG", "SVG")), ActionArgOption::new("png", LocalizedLabel::native("PNG", "PNG"))]).default_value(&"png"),
                ActionArgDef::select("shape", LocalizedLabel::native("Shape", "Form"), vec![ActionArgOption::new("rectangle", LocalizedLabel::native("Rectangle", "Rechteck")), ActionArgOption::new("ellipse", LocalizedLabel::native("Ellipse", "Ellipse"))]).default_value(&"rectangle"),
            ])
            .action_args("addAsset", vec![
                ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![ActionArgOption::new("glb", LocalizedLabel::native("GLB", "GLB"))]).default_value(&"glb"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(SHOOTING_EXAMPLE_DEFAULT_ID, LocalizedLabel::native("Default Base Icon", "Standard-Basissymbol")),
                    ActionArgOption::new(
                        SHOOTING_EXAMPLE_HEXAGONAL_CUT_CONCRETE_FOREST_LEFT,
                        LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Sechseckig geschnittener Betonwald links"),
                    ),
                ]).required(),
            ])
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", LocalizedLabel::native("Move", "Verschieben"), "move") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2") })
            .window_kind_utilities(SHOOTING_PLAY_WINDOW_SCENE, vec!["move".into(), "rotate".into(), "scale".into()])
            .config(ShootingPlayApp::config_spec())
            .io(shooting_io())
            .action_describe("importSnapshotJson", LocalizedLabel::native("Replaces the whole shooting document (shots, assets, cameras and scene lighting) with one parsed from the given JSON; invalid JSON changes nothing.", "Ersetzt das gesamte Shooting-Dokument (Aufnahmen, Objekte, Kameras und Szenenlicht) durch eines aus dem angegebenen JSON; ungültiges JSON ändert nichts."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole shooting document with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte Shooting-Dokument durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
            .action_describe("setActiveShot", LocalizedLabel::native("Makes the shot with the given id the active one that shot edits and Export Active Shot apply to.", "Macht die Aufnahme mit der angegebenen Id zur aktiven, auf die Aufnahmeänderungen und Aktive Aufnahme exportieren wirken."))
            .action_describe("setActiveAsset", LocalizedLabel::native("Makes the asset (3D model) with the given id the active one, or clears the active asset.", "Macht das Objekt (3D-Modell) mit der angegebenen Id zum aktiven oder hebt das aktive Objekt auf."))
            .action_describe("setShotCamera", LocalizedLabel::native("Replaces the camera pose stored on one shot, which decides how that shot frames the scene when rendered.", "Ersetzt die in einer Aufnahme gespeicherte Kamerapose, die bestimmt, wie diese Aufnahme die Szene beim Rendern zeigt."))
            .action_describe("saveCamera", LocalizedLabel::native("Saves the current viewport camera into the document's catalogue of saved cameras under the given label (Camera N when blank).", "Speichert die aktuelle Ansichtskamera unter der angegebenen Bezeichnung (Kamera N, wenn leer) im Katalog gespeicherter Kameras des Dokuments."))
            .action_describe("loadSavedCamera", LocalizedLabel::native("Moves the viewport camera to one saved camera by id; only the view changes, not the document.", "Setzt die Ansichtskamera auf eine gespeicherte Kamera anhand ihrer Id; nur die Ansicht ändert sich, nicht das Dokument."))
            .action_describe("setSunAzimuth", LocalizedLabel::native("Sets the compass direction (azimuth, in degrees) the scene's sun shines from.", "Legt die Himmelsrichtung (Azimut, in Grad) fest, aus der die Sonne der Szene scheint."))
            .action_describe("setSunElevation", LocalizedLabel::native("Sets how high above the horizon (elevation, in degrees) the scene's sun stands.", "Legt fest, wie hoch über dem Horizont (Höhe, in Grad) die Sonne der Szene steht."))
            .action_describe("setSunIntensity", LocalizedLabel::native("Sets the brightness of the scene's sun light.", "Legt die Helligkeit des Sonnenlichts der Szene fest."))
            .action_describe("setAmbientIntensity", LocalizedLabel::native("Sets the brightness of the scene's ambient light.", "Legt die Helligkeit des Umgebungslichts der Szene fest."))
            .action_describe("setMaterialRoughness", LocalizedLabel::native("Sets the surface roughness applied to the scene's materials, from glossy to matte.", "Legt die auf die Materialien der Szene angewandte Oberflächenrauheit fest, von glänzend bis matt."))
            .action_describe("setShadowEnabled", LocalizedLabel::native("Turns cast shadows in the scene on or off.", "Schaltet Schlagschatten in der Szene ein oder aus."))
            .action_describe("toggleSun", LocalizedLabel::native("Switches the scene's sun light on or off.", "Schaltet das Sonnenlicht der Szene ein oder aus."))
            .action_describe("setActiveShotLabel", LocalizedLabel::native("Renames the active shot to the given label.", "Benennt die aktive Aufnahme in die angegebene Bezeichnung um."))
            .action_describe("setActiveShotFormat", LocalizedLabel::native("Sets the output format (PNG or SVG) the active shot renders to.", "Legt das Ausgabeformat (PNG oder SVG) fest, in das die aktive Aufnahme gerendert wird."))
            .action_describe("setActiveShotShape", LocalizedLabel::native("Sets the crop shape of the active shot, rectangle or ellipse, which the rendered icon is cut to.", "Legt die Zuschnittform der aktiven Aufnahme fest, Rechteck oder Ellipse, auf die das gerenderte Icon zugeschnitten wird."))
            .action_describe("patchShots", LocalizedLabel::native("Sets one named field (such as label, format or shape) on every shot with the given ids.", "Setzt ein benanntes Feld (etwa Bezeichnung, Format oder Form) auf allen Aufnahmen mit den angegebenen Ids."))
            .action_describe("patchAssets", LocalizedLabel::native("Sets one named field on every asset (3D model) with the given ids.", "Setzt ein benanntes Feld auf allen Objekten (3D-Modellen) mit den angegebenen Ids."))
            .action_describe("addShot", LocalizedLabel::native("Adds a new shot with the given format and shape, framed by the current viewport camera.", "Fügt eine neue Aufnahme mit dem angegebenen Format und der Form hinzu, gerahmt von der aktuellen Ansichtskamera."))
            .action_describe("addAsset", LocalizedLabel::native("Adds a new placeholder asset of the given format to the scene.", "Fügt der Szene ein neues Platzhalterobjekt des angegebenen Formats hinzu."))
            .action_describe("importAsset", LocalizedLabel::native("Imports a GLB model (data-URL payload, optional name) as a new asset in the scene.", "Importiert ein GLB-Modell (Data-URL-Inhalt, optionaler Name) als neues Objekt in die Szene."))
            .action_describe("resetSnapshot", LocalizedLabel::native("Resets the whole shooting document to the default scene; every shot, asset and saved camera is discarded.", "Setzt das gesamte Shooting-Dokument auf die Standardszene zurück; alle Aufnahmen, Objekte und gespeicherten Kameras werden verworfen."))
            .action_describe("translateSelection", LocalizedLabel::native("Moves the given assets by dx, dy and dz; one gumball drag is one history step whose offset stays editable.", "Verschiebt die angegebenen Objekte um dx, dy und dz; ein Gumball-Zug ist ein Verlaufsschritt, dessen Versatz bearbeitbar bleibt."))
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the given assets by an angle around the axis ax, ay, az; one gumball drag is one history step whose rotation stays editable.", "Dreht die angegebenen Objekte um einen Winkel um die Achse ax, ay, az; ein Gumball-Zug ist ein Verlaufsschritt, dessen Drehung bearbeitbar bleibt."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the given assets by sx, sy and sz; one gumball drag is one history step whose factors stay editable.", "Skaliert die angegebenen Objekte um sx, sy und sz; ein Gumball-Zug ist ein Verlaufsschritt, dessen Faktoren bearbeitbar bleiben."))
            .action_describe("setShotSelection", LocalizedLabel::native("Selects the given shots in the gallery and document tree; only the editor's view state changes.", "Wählt die angegebenen Aufnahmen in Galerie und Dokumentbaum aus; nur der Ansichtszustand des Editors ändert sich."))
            .action_describe("setCenterModel", LocalizedLabel::native("Sets whether the viewport keeps the model centred; only the view changes.", "Legt fest, ob die Ansicht das Modell zentriert hält; nur die Ansicht ändert sich."))
            .action_describe("saveDownload", LocalizedLabel::native("Writes the whole shooting document as text to a downloaded shooting.shooting.ops file on the user's machine.", "Schreibt das gesamte Shooting-Dokument als Text in eine heruntergeladene Datei shooting.shooting.ops auf dem Rechner des Nutzers."))
            .action_describe("importAssetRequest", LocalizedLabel::native("Opens the host's file picker for a GLB model; the chosen file is then imported as a new asset.", "Öffnet die Dateiauswahl des Hosts für ein GLB-Modell; die gewählte Datei wird dann als neues Objekt importiert."))
            .action_describe("exportActiveShot", LocalizedLabel::native("Renders the active shot and writes it as a PNG or SVG file named after the shot to the user's machine.", "Rendert die aktive Aufnahme und schreibt sie als PNG- oder SVG-Datei mit dem Namen der Aufnahme auf den Rechner des Nutzers."))
            .action_describe("exportAllShots", LocalizedLabel::native("Renders every shot and writes each as a PNG or SVG file named after its shot to the user's machine.", "Rendert alle Aufnahmen und schreibt jede als PNG- oder SVG-Datei mit dem Namen ihrer Aufnahme auf den Rechner des Nutzers."))
            .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("importSnapshotJson")
            .build_definition()
}
//#endregion 🔖️Manifest
