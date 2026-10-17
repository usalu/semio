//! 🧾️ Manifest of the GIS 2D play app: the layer argument options and the app definition that stitches every command, panel and window. See [`super`].

use crate::editor::gis2d::commands::{example, features, inference, view};
use crate::editor::gis2d::modes::edit;
use crate::editor::gis2d::modes::edit::windows::map;
use crate::editor::gis2d::panels::{artifact as document_panel, catalogue as catalogue_panel, inspection as inspection_panel};
use crate::standards::v1::subsets::any::artifact_declaration::artifact_kind;
use semio_framework::InteractiveJobClassification;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::Editor;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;

//#region 🔖️Manifest
/// 🗣️ The manifest-time localized names of [`GIS_MAP_LAYER_IDS`], in the same order — the static twin
/// of the runtime `gis2d_layer_label`, because `ActionArgOption` is built once at manifest time and
/// cannot reach the view state's resolved label set. `gis2d_layer_arg_options_cover_every_layer` pins
/// the two tables together.
pub const GIS_MAP_LAYER_ARG_NAMES: &[(&str, &str)] = &[
    ("Raster", "Raster"),
    ("Water", "Wasser"),
    ("Land", "Land"),
    ("Roads", "Straßen"),
    ("Buildings", "Gebäude"),
    ("Borders", "Grenzen"),
    ("Labels", "Beschriftungen"),
    ("Positions", "Positionen"),
    ("Position Labels", "Positionsbeschriftungen"),
    ("Routes", "Routen"),
    ("Regions", "Regionen"),
];

/// 🗂️ The staged map-layer choice shared by `toggleLayerVisibility` and `setLayerStrokeScale`.
fn gis2d_layer_arg() -> ActionArgDef {
    let options = GIS_MAP_LAYER_IDS.iter().zip(GIS_MAP_LAYER_ARG_NAMES).map(|((id, _, _), (english, german))| ActionArgOption::new(*id, LocalizedLabel::native(*english, *german))).collect();
    ActionArgDef::select("layerId", LocalizedLabel::native("Layer", "Ebene"), options).default_value(&"positions")
}

/// 🗂️ The staged document-collection choice every per-feature editing verb takes.
fn gis2d_collection_arg() -> ActionArgDef {
    let options = features::GIS_MAP_FEATURE_COLLECTIONS.iter().map(|(id, english, german, _)| ActionArgOption::new(*id, LocalizedLabel::native(*english, *german))).collect();
    ActionArgDef::select("collection", LocalizedLabel::native("Collection", "Sammlung"), options).default_value(&"positions")
}

/// 🎯️ The staged feature address — empty addresses the collection's newest entry, so the rail can
/// dispatch `moveFeature`/`renameFeature`/`deleteFeature` right after an `addFeature` without typing.
fn gis2d_feature_id_arg() -> ActionArgDef {
    ActionArgDef::text("featureId", LocalizedLabel::native("Feature", "Objekt")).default_value(&"")
}

pub fn create_gis2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::GISMAP_DIALECT).document(["semio", "gis", "2d"])
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Verfolgen"])
            .artifact_kind(artifact_kind())
            // 🔌️ Typed workflow ports (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe) — same
            // constructor fns `gis2d_io()` embeds, so `AppIo.all_ports()` and these declarations can
            // never drift apart. `map:out`'s `2d.map` kind is declared above; `features:in` pins no kind.
            .media_input(gis2d_features_in_port())
            .media_output(gis2d_map_out_port())
            .icon_id("gis2d")
            .mode_def(edit::definition())
            .default_mode_id(edit::GIS2D_PLAY_MODE_EDIT)
            .window_kind_def(map::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // 🕹️ The framework-owned "features" interaction domain (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — covers both the document tree's
            // layer selection (granularity "layer") and the map's feature pick/marquee selection
            // (granularity "feature"); auto-injects interactionSelect/interactionHover/clearSelection/
            // selectAll/setSelectionMode/setInteractionGranularity, replacing every deleted bespoke
            // setSelection/setFeatureSelection/setHover/setSelectionMethod/setSelectionMode/
            // clearSelection/selectAll action below.
            .interaction(InteractionDefinition {
                id: "features".into(),
                label: LocalizedLabel::native("Features", "Objekte"),
                granularities: vec![
                    GranularityDefinition { id: "layer".into(), label: LocalizedLabel::native("Layer", "Ebene"), icon_id: "layers".into() },
                    GranularityDefinition { id: "feature".into(), label: LocalizedLabel::native("Feature", "Objekt"), icon_id: "crosshair".into() },
                ],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(map::GIS2D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("features")])
            // ✏️ Mutation actions — flow through the document store with true inverses. `setActiveExample`
            // replaces document content by diffing every collection into batched create/delete/
            // replace-data operations (never a whole-document snapshot swap — that vocabulary is
            // retired by the taxonomy), so it is a Mutation, not a View action.
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .mutation("patchPositions", LocalizedLabel::native("Patch Positions", "Positionen aktualisieren"))
            .mutation("patchRoutes", LocalizedLabel::native("Patch Routes", "Routen aktualisieren"))
            .mutation("patchRoute", LocalizedLabel::native("Patch Route", "Route aktualisieren"))
            // 🗺️ Per-feature editing vocabulary — the map editor's own add / move / rename / delete,
            // each fully argument-staged so the Actions rail can dispatch it, and each routed through
            // the collection's authored create/replace-data/delete leaves, so undo and redo are the
            // leaves' own inverses rather than a bespoke history.
            .mutation("addFeature", LocalizedLabel::native("Add Feature", "Objekt hinzufügen"))
            .mutation("moveFeature", LocalizedLabel::native("Move Feature", "Objekt verschieben"))
            .mutation("renameFeature", LocalizedLabel::native("Rename Feature", "Objekt umbenennen"))
            .mutation("deleteFeature", LocalizedLabel::native("Delete Feature", "Objekt löschen"))
            .action_destructive("deleteFeature")
            // 👁️ View actions — mutate ephemeral config state (camera, render config, layer
            // visibility, stroke weights), never the document.
            .view_action("toggleLayerVisibility", LocalizedLabel::native("Toggle Layer Visibility", "Ebenensichtbarkeit umschalten"))
            .action_with(ActionDefinition { category: Some("view".into()), ..ActionDefinition::bounded_catalog("fitWorld", LocalizedLabel::native("Fit World", "Welt einpassen"), ActionKind::View) })
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .view_action("setRenderMode", LocalizedLabel::native("Set Render Mode", "Darstellungsmodus festlegen"))
            .view_action("setVectorStyle", LocalizedLabel::native("Set Vector Style", "Vektorstil festlegen"))
            .action_with(ActionDefinition::new("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"), ActionKind::View, "layers"))
            .action_with(ActionDefinition { category: Some("view".into()), ..ActionDefinition::bounded_catalog("focusFeature", LocalizedLabel::native("Focus Feature", "Objekt fokussieren"), ActionKind::View) })
            .view_action("setLayerStrokeScale", LocalizedLabel::native("Set Layer Stroke Scale", "Ebenenstrichstärke festlegen"))
            // 🌐️ Shell action — opens the picked feature's source URL through the host.
            .action_with(ActionDefinition { category: Some("open".into()), ..ActionDefinition::new("openSource", LocalizedLabel::native("Open Source", "Quelle öffnen"), ActionKind::Shell, "hard-drive") })
            // 💡️ Shell action — asks the host to open its own ephemeral inference port and offer a
            // reviewable bounds region. It never writes the document; only the hub's server-stamped
            // approval command can.
            .action_with(ActionDefinition { category: Some("inference".into()), ..ActionDefinition::bounded_catalog("proposeBoundsRegion", LocalizedLabel::native("Propose Bounds Region", "Begrenzungsregion vorschlagen"), ActionKind::Shell) })
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchPositions", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchRoutes", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchRoute", InteractiveJobClassification::Migrated)
            .action_interactive_job("addFeature", InteractiveJobClassification::Migrated)
            .action_interactive_job("moveFeature", InteractiveJobClassification::Migrated)
            .action_interactive_job("renameFeature", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteFeature", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleLayerVisibility", InteractiveJobClassification::Migrated)
            .action_interactive_job("fitWorld", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setRenderMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("setVectorStyle", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusFeature", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLayerStrokeScale", InteractiveJobClassification::Migrated)
            .action_interactive_job("openSource", InteractiveJobClassification::Migrated)
            .action_interactive_job("proposeBoundsRegion", InteractiveJobClassification::Migrated)
            // 📝️ Argument schemas for the discrete-choice actions so the command palette can stage them
            // and the registry validates the vocabulary. The arg id matches the key each handler reads.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), example::example_arg_options()).default_value(&example::DEFAULT_EXAMPLE_ID),
            ])
            .action_args("addFeature", vec![
                gis2d_collection_arg(),
                ActionArgDef::text("label", LocalizedLabel::native("Label", "Bezeichnung")).default_value(&""),
                ActionArgDef::number("lon", LocalizedLabel::native("Longitude", "Längengrad")).default_value(&0.0),
                ActionArgDef::number("lat", LocalizedLabel::native("Latitude", "Breitengrad")).default_value(&0.0),
                ActionArgDef::number("span", LocalizedLabel::native("Span", "Ausdehnung")).default_value(&GIS2D_DEFAULT_FEATURE_SPAN),
            ])
            .action_args("moveFeature", vec![
                gis2d_collection_arg(),
                gis2d_feature_id_arg(),
                ActionArgDef::number("lon", LocalizedLabel::native("Longitude", "Längengrad")).default_value(&0.0),
                ActionArgDef::number("lat", LocalizedLabel::native("Latitude", "Breitengrad")).default_value(&0.0),
            ])
            .action_args("renameFeature", vec![
                gis2d_collection_arg(),
                gis2d_feature_id_arg(),
                ActionArgDef::text("label", LocalizedLabel::native("Label", "Bezeichnung")).default_value(&"Renamed Feature"),
            ])
            .action_args("deleteFeature", vec![gis2d_collection_arg(), gis2d_feature_id_arg()])
            .action_args("toggleLayerVisibility", vec![gis2d_layer_arg()])
            .action_args("setLayerStrokeScale", vec![
                gis2d_layer_arg(),
                ActionArgDef::slider("value", LocalizedLabel::native("Stroke Scale", "Strichstärke"), 0.25, 4.0).default_value(&1.0),
            ])
            .action_args("setRenderMode", vec![
                ActionArgDef::select("value", LocalizedLabel::native("Render Mode", "Darstellungsmodus"), vec![
                    ActionArgOption::new("image", LocalizedLabel::native("Image", "Bild")),
                    ActionArgOption::new("vector", LocalizedLabel::native("Vector", "Vektor")),
                    ActionArgOption::new("combined", LocalizedLabel::native("Combined", "Kombiniert")),
                ]).default_value(&"combined"),
            ])
            .action_args("setVectorStyle", vec![
                ActionArgDef::select("value", LocalizedLabel::native("Vector Style", "Vektorstil"), vec![
                    ActionArgOption::new("colored", LocalizedLabel::native("Colored", "Farbig")),
                    ActionArgOption::new("figureGround", LocalizedLabel::native("Figure Ground", "Figur-Grund")),
                    ActionArgOption::new("invertedFigure", LocalizedLabel::native("Inverted Figure", "Invertierte Figur")),
                ]).default_value(&"colored"),
            ])
            .action_args("setLodMode", vec![
                ActionArgDef::select("value", LocalizedLabel::native("LOD Mode", "LOD-Modus"), map::options::lod_mode::lod_arg_options()).default_value(&semio_framework_surface::tiled_map::MAP_LOD_MODE_AUTOMATIC),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .io(gis2d_io())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder::build_definition` has no `.example(...)`/
            // `.workflow(...)`. The subset's own `📚️examples/*` facets are the replacement surface —
            // `example::example_catalogue()` reads them, and the `setActiveExample` arg options above
            // are projected from it, so the palette and `set_active_example` can never drift apart.
            .interactive_jobs(InteractiveJobClassification::Migrated)
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole map with one of the plugin's bundled map examples, by example id.", "Ersetzt die gesamte Karte durch eines der mitgelieferten Kartenbeispiele, anhand der Beispiel-Id."))
            .action_describe("patchPositions", LocalizedLabel::native("Replaces the coordinates of the map's features from a JSON position list, writing only the positions that change.", "Ersetzt die Koordinaten der Kartenobjekte aus einer JSON-Positionsliste und schreibt nur die sich ändernden Positionen."))
            .action_describe("patchRoutes", LocalizedLabel::native("Sets one named field (such as name or colour) on several routes at once.", "Setzt ein benanntes Feld (etwa Name oder Farbe) auf mehreren Routen gleichzeitig."))
            .action_describe("patchRoute", LocalizedLabel::native("Sets one named field of one route, such as its name, colour or waypoints.", "Setzt ein benanntes Feld einer Route, etwa Name, Farbe oder Wegpunkte."))
            .action_describe("addFeature", LocalizedLabel::native("Adds a new point, line or area feature with a label at longitude and latitude to a collection of the map.", "Fügt einer Sammlung der Karte ein neues Punkt-, Linien- oder Flächenobjekt mit Bezeichnung an Länge und Breite hinzu."))
            .action_describe("moveFeature", LocalizedLabel::native("Moves one map feature of a collection to new longitude and latitude.", "Verschiebt ein Kartenobjekt einer Sammlung an neue Länge und Breite."))
            .action_describe("renameFeature", LocalizedLabel::native("Renames one map feature of a collection.", "Benennt ein Kartenobjekt einer Sammlung um."))
            .action_describe("deleteFeature", LocalizedLabel::native("Removes one feature by id from a collection of the map.", "Entfernt ein Objekt anhand seiner Id aus einer Sammlung der Karte."))
            .action_describe("openSource", LocalizedLabel::native("Opens the selected feature's source URL through the host.", "Öffnet die Quell-URL des ausgewählten Objekts über den Host."))
            .action_describe("proposeBoundsRegion", LocalizedLabel::native("Asks the inference service to propose a bounds region for human review; it never writes the map itself.", "Bittet den Inferenzdienst, eine Begrenzungsregion zur menschlichen Prüfung vorzuschlagen; die Karte selbst wird nie geschrieben."))
            .action_audience("toggleLayerVisibility", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("fitWorld", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("setRenderMode", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("setVectorStyle", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("setLodMode", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("focusFeature", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("setLayerStrokeScale", semio_framework_plugin::CapabilityAudience::Chrome)
            .build_definition()
}
//#endregion 🔖️Manifest
