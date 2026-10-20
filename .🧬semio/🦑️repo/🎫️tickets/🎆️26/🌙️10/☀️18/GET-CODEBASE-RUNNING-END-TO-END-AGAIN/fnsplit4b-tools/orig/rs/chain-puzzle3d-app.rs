//! 📋️ Manifest: the typed media I/O, the interaction definition, the shipped object kinds and `create_puzzle3d_app`.

use super::*;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main;
use semio_s_artifact_puzzle_3d_editor_panels::artifact;
use semio_s_artifact_puzzle_3d_editor_panels::catalogue;
use semio_s_artifact_puzzle_3d_editor_panels::inspection;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main::utilities;
use semio_s_artifact_puzzle_3d_editor_modes as edit;
use semio_s_artifact_puzzle_3d_editor_panels::settings as settings_panel;
use semio_s_artifact_puzzle_3d_editor_terminology::Puzzle3dLabels;

//#region 🔖️Manifest
/// 🔌️ Declares puzzle3d's typed media I/O surface — the implicit document ports plus the flagship
/// `kit:in` seam: an input port accepting `Kit×Type` media tagged `kit.catalog`, fanning IN from
/// potentially many producers (`multiplicity: Many`).
///
/// 🎯️ A free function, not an inline `ArtifactApp::io()` body, because BOTH the trait method and the
/// `AppBuilder` need it: the trait method serves the runtime, while `.io(..)` on the builder is what
/// puts `document_schema` into the published `AppDefinition`. Inlining it in only the trait method
/// left the manifest's `io` empty, so a host reading the manifest could not route a document to this
/// surface at all — caught by the demonstrator bundle's `every_pane_declares_a_document_schema`.
pub fn puzzle3d_io() -> AppIo {
    ::semio_framework_async::poll::resolve_ready(
        ::semio_framework_async::poll::resolve_ready(AppIo::from_artifact(
            "puzzle.3d",
            MediaType { class: MediaClass::ThreeD, form: MediaForm::Design },
            semio_framework_plugin::ArtifactPresentation { id: "3d.puzzle".into(), name: "3D Puzzle".into(), dimension: "3d".into(), component_kind: "puzzle3d".into() },
        ))
        .with_ports(vec![MediaPortSpec {
            id: "kit:in".into(),
            label: "Kit Catalog".into(),
            direction: MediaPortDirection::In,
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            kind_id: Some("kit.catalog".into()),
            required: false,
            multiplicity: PortMultiplicity::Many,
        }]),
    )
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain declaration —
/// one granularity per previously-distinct `Puzzle3dSelection` bag (object/vortex/attraction/
/// targetVolume/reference) plus `kind` for the catalogue's rows. `Topology` hierarchy (see
/// `Puzzle3dPlayApp::interaction_topology`) makes the object→vortex-marker nesting available for a
/// future transitive hover; every other granularity is a flat root today.
fn puzzle3d_interaction_definition() -> InteractionDefinition {
    let granularity = |id: &str, label: LocalizedLabel, icon: &str| GranularityDefinition { id: id.into(), label, icon_id: icon.into() };
    InteractionDefinition {
        id: PUZZLE3D_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Vortex", "Vortex"),
        granularities: vec![
            granularity(PUZZLE3D_GRANULARITY_OBJECT, puzzle3d_localized(|l| l.object), "box"),
            granularity(PUZZLE3D_GRANULARITY_VORTEX, puzzle3d_localized(|l| l.vortex), "sparkles"),
            granularity(PUZZLE3D_GRANULARITY_ATTRACTION, puzzle3d_localized(|l| l.attraction), "link"),
            granularity(PUZZLE3D_GRANULARITY_TARGET_VOLUME, puzzle3d_localized(|l| l.target_volume), "box-select"),
            granularity(PUZZLE3D_GRANULARITY_REFERENCE, puzzle3d_localized(|l| l.reference), "image"),
            granularity(PUZZLE3D_GRANULARITY_KIND, puzzle3d_localized(|l| l.kind), "layers"),
        ],
        hierarchy: HierarchyProvider::Topology,
        hover: HoverSpec { enabled: true, transitive: false, channels: vec![PUZZLE3D_HOVER_CHANNEL.into()], broadcast: true },
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}

/// 🗂️ Fixed ceiling on the object-kind rows the `addObjectKind` arg form and the "Add Object" dialog
/// offer — the manifest is minted once per process, so this select is built eagerly and must be
/// bounded independently of how wide a catalog a future example declares.
pub const PUZZLE3D_OBJECT_KIND_OPTIONS_MAX: usize = 64;

/// 🗂️ The object kinds the "Add Object" dialog (and the standalone `addObjectKind` arg form) offers —
/// read from the declared examples' own `meta.kindCatalogs`, exactly the rows
/// `📌️panels/🛍️catalogue` renders, deduplicated across examples in catalog order. `AppDefinition`
/// is built once per process and never sees the live document, and `setActiveExample` is this
/// editor's only document source (see `📓️2026-09-09-user-feature-checklist.md` §24: there is no
/// import), so the union of the declared example catalogs IS the reachable kind set — not the single
/// literal `"Object"` option this select used to hardcode, which could not add a single real kind of
/// either example.
///
/// 🖐️ Those rows are AUTHORED below rather than derived, in the two shipped examples' own catalog
/// order, and pinned to the documents by `shipped_object_kinds_are_the_two_examples_own_catalog_rows`.
///
/// 🐛️ Deriving it dereferenced `CONCRETE_FOREST_EXAMPLE_SNAPSHOT` and `NAKAGIN_EXAMPLE_SNAPSHOT`,
/// i.e. parsed 136 395 B of authored DSL into two typed `Puzzle3dSceneSnapshot`s — on the
/// `AppDefinition` path, which is the `describe()` path AND every actor boot. Measured natively on
/// 2026-09-22 (slice PZ2, `🗑️generated/pz2-native-profile-*.txt`): `create_puzzle3d_app()` cost
/// 231 ms cold against ~1 ms with those statics warm, so ALL of it was this one select.
pub const PUZZLE3D_SHIPPED_OBJECT_KINDS: &[(&str, &str)] = &[
    ("Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Left"),
    ("Hexagonal Cut Concrete Forest Right", "Hexagonal Cut Concrete Forest Right"),
    ("Base", "Base"),
    ("Bridge", "Bridge"),
    ("Capital", "Capital"),
    ("Capsule With Balcony Backslash", "Capsule With Balcony Backslash"),
    ("Capsule With Balcony J", "Capsule With Balcony J"),
    ("Capsule With Balcony L", "Capsule With Balcony L"),
    ("Capsule With Balcony P", "Capsule With Balcony P"),
    ("Capsule With Balcony S", "Capsule With Balcony S"),
    ("Capsule With Balcony Slash", "Capsule With Balcony Slash"),
    ("First Storey Tambour", "First Storey Tambour"),
    ("Last Storey Tambour", "Last Storey Tambour"),
    ("Tambour", "Tambour"),
];

/// 🗂️ The `objectKind` select's options, mapped from [`PUZZLE3D_SHIPPED_OBJECT_KINDS`].
fn puzzle3d_object_kind_options() -> Vec<ActionArgOption> {
    PUZZLE3D_SHIPPED_OBJECT_KINDS.iter().take(PUZZLE3D_OBJECT_KIND_OPTIONS_MAX).map(|(id, label)| ActionArgOption::new(*id, LocalizedLabel::data(*label))).collect()
}

/// 🗂️ The kind the `objectKind` select stages when nothing is picked — the first catalog row, never a
/// literal id no catalog declares.
fn puzzle3d_default_object_kind(options: &[ActionArgOption]) -> String {
    options.first().map(|option| option.value.clone()).unwrap_or_default()
}

/// 🗂️ The one `objectKind` select both the standalone `addObjectKind` arg form and the "Add Object"
/// dialog declare — built twice from the same catalog so the two forms can never drift apart.
fn puzzle3d_object_kind_arg() -> ActionArgDef {
    let options = puzzle3d_object_kind_options();
    let default = puzzle3d_default_object_kind(&options);
    ActionArgDef::select("objectKind", puzzle3d_localized(|l| l.kind), options).default_value(&default)
}

/// 🎭️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (contract §2.4): `Editor::builder`
/// derives the surface id from `DIALECT`+`ROLE` (no hand-written label/id), returns `AppDefinition`
/// (not `App`), and has no `.example(...)`/`.workflow(...)` methods — the old
/// `.example(PUZZLE3D_EXAMPLE_CONCRETE_FOREST, …)` / `.example(PUZZLE3D_EXAMPLE_NAKAGIN, …)` /
/// `.workflow("puzzle3d", "Puzzle 3D", "model")` calls this builder used to end with are DROPPED
/// here, not ported (contract §7.4's `App { definition, examples }` split: `.editor::<E>(def)` only
/// takes the definition, so `AppBuilder`'s own `examples` vec is discarded either way) — flagged as a
/// known gap for the coordinator, not silently lost.
pub fn create_puzzle3d_app() -> semio_framework_plugin::AppDefinition {
    let envelope = Puzzle3dScene { scene_snapshot: default_scene_snapshot(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    Editor::builder(semio_s_artifact_puzzle_3d_model::PUZZLE3D_DIALECT)
            .document(["semio", "puzzle", "3d"])
            .artifact_kind(semio_s_artifact_puzzle_3d_model::artifact_kind())
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Aggregator"])
            .mode_def(edit::definition())
            .default_mode_id(edit::PUZZLE3D_PLAY_MODE_EDIT)
            .io(puzzle3d_io())
            .window_kind_def(main::definition(&envelope, &Puzzle3dLabels::NATIVE_EN))
            .interaction(puzzle3d_interaction_definition())
            .window_kind_interactions(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, vec![InteractionRef::new(PUZZLE3D_INTERACTION_DOMAIN)])
            .default_layout(edit::layout())
            .panel_tab_def(artifact::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            .panel_tab_def(settings_panel::definition())
            .keybinding("escape", "engagementAbort")
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("mod+d", "duplicateSelection")
            .keybinding("tab", "cycleBrushCandidate")
            .keybinding("shift+tab", "cycleBrushCandidateBack")
            .keybinding("f", "focusSelection")
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .action_with(ActionDefinition::bounded_catalog("openAddObjectDialog", puzzle3d_localized_phrase(|l| l.object, |w| format!("Add {w}…"), |w| format!("{w} hinzufügen…")), ActionKind::Shell).category("create"))
            .action_with(ActionDefinition::bounded_catalog("addObjectKind", puzzle3d_localized_phrase(|l| l.object, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")), ActionKind::Mutation).category("create").in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).category("selection"))
            .action_destructive("deleteSelection")
            .action_with(ActionDefinition::bounded_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Mutation).category("create"))
            .action_with(ActionDefinition::bounded_catalog("exportSnapshot", LocalizedLabel::native("Export", "Exportieren"), ActionKind::Shell).category("file"))
            .action_with(ActionDefinition::bounded_catalog("importSnapshot", LocalizedLabel::native("Import", "Importieren"), ActionKind::Mutation).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("openImportSnapshot", LocalizedLabel::native("Import", "Importieren"), ActionKind::Shell).category("file"))
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .mutation("worldRelocate", puzzle3d_localized_phrase(|l| l.object, |w| format!("Relocate {w}"), |w| format!("{w} verlagern")))
            .action_with(ActionDefinition::bounded_catalog("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).category("hand"))
            .action_with(puzzle3d_flag_value_action("setSelectionHidden", "hidden", LocalizedLabel::native("Set Hidden", "Verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))
            .action_with(puzzle3d_flag_value_action("setSelectionLocked", "locked", LocalizedLabel::native("Set Locked", "Gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))
            .mutation("patchInspector", LocalizedLabel::native("Patch Inspector", "Inspektor aktualisieren"))
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .mutation("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"))
            .mutation("createAttraction", puzzle3d_localized_phrase(|l| l.attraction, |w| format!("Create {w}"), |w| format!("{w} erstellen")))
            .action_with(ActionDefinition::bounded_catalog("deleteAttraction", puzzle3d_localized_phrase(|l| l.attraction, |w| format!("Delete {w}"), |w| format!("{w} löschen")), ActionKind::Mutation).category("targets").in_palette(false))
            .action_destructive("deleteAttraction")
            .mutation("addTargetVolume", puzzle3d_localized_phrase(|l| l.target_volume, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")))
            .action_with(ActionDefinition::bounded_catalog("deleteTargetVolume", LocalizedLabel::native("Delete Target Volume", "Zielvolumen löschen"), ActionKind::Mutation).category("targets").in_palette(false))
            .action_destructive("deleteTargetVolume")
            .action_with(ActionDefinition::bounded_catalog("setTargetVolumeFlag", LocalizedLabel::native("Set Target Volume Flag", "Zielvolumenmarkierung festlegen"), ActionKind::Mutation).category("targets").in_palette(false))
            .mutation("addBrushObject", puzzle3d_localized_phrase(|l| l.object, |w| format!("Add Brush {w}"), |w| format!("Pinsel-{w} hinzufügen")))
            .mutation("acceptSuggestion", LocalizedLabel::native("Accept Suggestion", "Vorschlag annehmen"))
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setProjection", LocalizedLabel::native("Set Projection", "Projektion festlegen"), ActionKind::View, "scan"))
            .action_with(ActionDefinition::new("setProjectionParam", LocalizedLabel::native("Set Projection Parameter", "Projektionsparameter festlegen"), ActionKind::View, "scan"))
            .view_action("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"))
            .action_with(ActionDefinition::bounded_catalog("selectSameKindSelection", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).category("selection"))
            .view_action("setVortexShow", puzzle3d_localized_phrase(|l| l.vortex_show, |w| format!("Set {w}"), |w| format!("{w} festlegen")))
            .view_action("setVortexDirection", puzzle3d_localized_phrase(|l| l.vortex_direction, |w| format!("Set {w}"), |w| format!("{w} festlegen")))
            .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
            .view_action("setLodAutomatic", LocalizedLabel::native("Set Lod Automatic", "Detailstufe automatisch"))
            .view_action("setLodDepthVariable", LocalizedLabel::native("Set Lod Depth Variable", "Detailstufen-Tiefe festlegen"))
            .view_action("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster anzeigen"))
            .view_action("setLodManual", LocalizedLabel::native("Set Lod Manual", "Detailstufe manuell"))
            .action_with(ActionDefinition::new("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View, "grid-3x3"))
            .view_action("setGridSpacing", LocalizedLabel::native("Set Grid Spacing", "Rasterabstand festlegen"))
            .view_action("setProximityRadius", LocalizedLabel::native("Set Proximity Radius", "Näheradius festlegen"))
            .view_action("setChunkSize", LocalizedLabel::native("Set Chunk Size", "Blockgröße festlegen"))
            .view_action("setSelectableKind", LocalizedLabel::native("Set Selectable Kind", "Auswählbare Art festlegen"))
            .action_with(ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand"))
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View, "hand"))
            .action_audience("engagementAbort", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition::new("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"), ActionKind::View, "hand"))
            .view_action("setTransformGumballFlag", LocalizedLabel::native("Set Transform Gumball Flag", "Transformieren-Griff festlegen"))
            .view_action("setVoxelDims", LocalizedLabel::native("Set Voxel Dims", "Voxel-Abmessungen festlegen"))
            .mutation("relocateTargetVolume", LocalizedLabel::native("Relocate Target Volume", "Zielvolumen verlagern"))
            .view_action("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"))
            .view_action("setBrushPlacementContactTolerance", LocalizedLabel::native("Set Brush Placement Contact Tolerance", "Pinsel-Kontakttoleranz festlegen"))
            .view_action("setObjectKindWeight", puzzle3d_localized_phrase(|l| l.object, |w| format!("Set {w} Kind Weight"), |w| format!("{w}-Art-Gewicht festlegen")))
            .view_action("setVortexKindWeight", puzzle3d_localized_phrase(|l| l.vortex, |w| format!("Set {w} Kind Weight"), |w| format!("{w}-Art-Gewicht festlegen")))
            .view_action("cycleBrushCandidate", LocalizedLabel::native("Cycle Brush Candidate", "Pinselkandidat wechseln"))
            .view_action("cycleBrushCandidateBack", LocalizedLabel::native("Cycle Brush Candidate Back", "Pinselkandidat rückwärts wechseln"))
            .action_with(ActionDefinition::bounded_catalog("openVortexSuggestions", puzzle3d_localized_phrase(|l| l.vortex, |w| format!("Open {w} Suggestions"), |w| format!("{w}-Vorschläge öffnen")), ActionKind::View).category("tools"))
            .view_action("closeVortexSuggestions", puzzle3d_localized_phrase(|l| l.vortex, |w| format!("Close {w} Suggestions"), |w| format!("{w}-Vorschläge schließen")))
            .view_action("hoverSuggestion", LocalizedLabel::native("Hover Suggestion", "Vorschlag überfahren"))
            .action_audience("hoverSuggestion", semio_framework_plugin::CapabilityAudience::Input)
            .view_action("targetBrushSuggestions", LocalizedLabel::native("Target Brush Suggestions", "Pinselvorschläge ausrichten"))
            .view_action("registerBrushMesh", LocalizedLabel::native("Register Brush Mesh", "Pinsel-Mesh registrieren"))
            .action_with(ActionDefinition::new("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .action_audience("worldPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_args("addObjectKind", vec![puzzle3d_object_kind_arg()])
            .utility(semio_s_artifact_puzzle_3d_editor_transform::definition())
            .utility(utilities::brush::definition(puzzle3d_localized(|l| l.brush)))
            .utility(utilities::volume_brush::definition(puzzle3d_localized(|l| l.volume_brush)))
            .utility(utilities::world_relocate::definition())
            .window_kind_utilities(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, vec![
                semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID.into(),
                utilities::brush::UTILITY_ID.into(),
                utilities::volume_brush::UTILITY_ID.into(),
                utilities::world_relocate::UTILITY_ID.into(),
            ])
            .tool(fill_tool::definition(puzzle3d_localized(|l| l.fill)))
            .mode_tools(edit::PUZZLE3D_PLAY_MODE_EDIT, vec![::semio_framework_async::poll::resolve_ready(ToolRef::new(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID))])
            .introduction(IntroductionDefinition {
                title: puzzle3d_localized_phrase(|l| l.window_main, |w| format!("Welcome to {w}"), |w| format!("Willkommen bei {w}")),
                steps: vec![
                    IntroductionStepDefinition::new(
                        "welcome",
                        puzzle3d_localized_phrase(|l| l.window_main, |w| format!("Welcome to {w}"), |w| format!("Willkommen bei {w}")),
                        LocalizedLabel::native(
                            "A quick tour of the viewport, utilities, and panels before you start composing.",
                            "Eine kurze Tour durch Ansicht, Hilfsmittel und Paneele, bevor Sie mit dem Zusammenfügen beginnen.",
                        ),
                    ),
                    IntroductionStepDefinition::new(
                        "viewport",
                        LocalizedLabel::native("The Viewport", "Die 3D-Ansicht"),
                        LocalizedLabel::native(
                            "This is your 3D scene — orbit, pan, and zoom to look around.",
                            "Das ist Ihre 3D-Szene — orbitieren, verschieben und zoomen Sie, um sich umzusehen.",
                        ),
                    )
                        .introduce(window_element_id(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID))
                        .interact(vec![
                            ::semio_framework_async::poll::resolve_ready(IntroductionInteraction::zoom(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, "Zoom")),
                            ::semio_framework_async::poll::resolve_ready(IntroductionInteraction::pan(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, "Pan")),
                            ::semio_framework_async::poll::resolve_ready(IntroductionInteraction::orbit(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, "Orbit")),
                        ]),
                    IntroductionStepDefinition::new(
                        "catalogue",
                        LocalizedLabel::native("The Catalogue", "Der Katalog"),
                        puzzle3d_localized_phrase(|l| l.objects, |w| format!("Browse the {w} available to place from here."), |w| format!("Durchstöbern Sie hier die verfügbaren {w}.")),
                    )
                        .introduce(panel_tab_element_id(semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID))
                        .placement(IntroductionPlacement::Right),
                    IntroductionStepDefinition::new(
                        "add-object",
                        puzzle3d_localized_phrase(|l| l.object, |w| format!("Add a {w}"), |w| format!("{w} hinzufügen")),
                        puzzle3d_localized_phrase(
                            |l| l.object,
                            |w| format!("Drag the first {w} from the catalogue into the viewport."),
                            |_w| "Ziehen Sie den ersten Eintrag per Drag-and-Drop aus dem Katalog in die 3D-Ansicht.".to_string(),
                        ),
                    )
                        .introduce(panel_tab_first_draggable_element_id(semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID))
                        .show(vec![panel_tab_element_id(semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID), window_element_id(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)])
                        .placement(IntroductionPlacement::Right)
                        .interact(vec![::semio_framework_async::poll::resolve_ready(IntroductionInteraction::action("addObjectKind", "Add an object"))]),
                    IntroductionStepDefinition::new(
                        "transform-utility",
                        puzzle3d_localized_phrase(|l| l.objects, |w| format!("Transform {w}"), |w| format!("{w} transformieren")),
                        puzzle3d_localized_phrase(
                            |l| l.objects,
                            |w| format!("Activate the Transform utility to move and rotate {w} in the scene."),
                            |w| format!("Aktivieren Sie das Transformieren-Hilfsmittel, um {w} zu verschieben und zu drehen."),
                        ),
                    )
                        .introduce(semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID)
                        .show(vec![window_element_id(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID)])
                        .interact(vec![::semio_framework_async::poll::resolve_ready(IntroductionInteraction::utility(semio_s_artifact_puzzle_3d_editor_transform::UTILITY_ID, "Activate Transform"))]),
                ],
            })
            .dialog(
                DialogDefinition::new(
                    "addObject",
                    puzzle3d_localized_phrase(|l| l.object, |w| format!("Add {w}"), |w| format!("{w} hinzufügen")),
                    ActionRef::new("addObjectKind"),
                )
                    .body(puzzle3d_localized_phrase(
                        |l| l.object,
                        |w| format!("Choose the kind of {w} to add to the scene."),
                        |_w| "Wählen Sie die Art zum Hinzufügen.".to_string(),
                    ))
                    .args(vec![puzzle3d_object_kind_arg().required()])
                    .submit_label(LocalizedLabel::native("Add", "Hinzufügen")),
            )
            .action_interactive_job("acceptSuggestion", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addBrushObject", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addObjectKind", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addTargetVolume", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("closeVortexSuggestions", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("createAttraction", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidate", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("cycleBrushCandidateBack", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteAttraction", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteTargetVolume", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("exportSnapshot", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_destructive("exportSnapshot")
            .action_interactive_job("importSnapshot", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("openImportSnapshot", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementControlSelect", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementRepeatLast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("focusSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("hoverSuggestion", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("openAddObjectDialog", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("openVortexSuggestions", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("patchInspector", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("registerBrushMesh", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("relocateTargetVolume", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("selectSameKindSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setBrushPlacementContactTolerance", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setChunkSize", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setFillCount", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSnapEnabled", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridSpacing", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setGridVisible", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodAutomatic", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodDepthVariable", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodManual", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setObjectKindWeight", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjectionParam", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setProximityRadius", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectableKind", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionFlag", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionHidden", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectionLocked", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setTargetVolumeFlag", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setTransformGumballFlag", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setVortexDirection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setVortexKindWeight", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setVortexShow", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setVoxelDims", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("targetBrushSuggestions", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerDown", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("worldRelocate", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 3D puzzle with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte 3D-Puzzle durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
            .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected object from the 3D puzzle together with its connections.", "Löscht alle ausgewählten Objekte samt ihrer Verbindungen aus dem 3D-Puzzle."))
            .action_describe("duplicateSelection", LocalizedLabel::native("Adds a copy of every selected object next to the original; nothing happens without a selection.", "Fügt neben jedem ausgewählten Objekt eine Kopie hinzu; ohne Auswahl geschieht nichts."))
            .action_describe("focusSelection", LocalizedLabel::native("Frames the camera on the selected objects; only the view changes.", "Richtet die Kamera auf die ausgewählten Objekte aus; nur die Ansicht ändert sich."))
            .action_describe("translateSelection", LocalizedLabel::native("Moves the selected objects by dx and dy.", "Verschiebt die ausgewählten Objekte um dx und dy."))
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the selected objects by the given angle.", "Dreht die ausgewählten Objekte um den angegebenen Winkel."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the selected objects by the given factor.", "Skaliert die ausgewählten Objekte um den angegebenen Faktor."))
            .action_describe("exportSnapshot", LocalizedLabel::native("Writes the whole 3D puzzle as JSON to a downloaded file named after the active example on the user's machine.", "Schreibt das gesamte 3D-Puzzle als JSON in eine heruntergeladene, nach dem aktiven Beispiel benannte Datei auf dem Rechner des Nutzers."))
            .action_describe("openImportSnapshot", LocalizedLabel::native("Opens the host's file picker for a 3D puzzle JSON file; the chosen file then replaces the whole puzzle.", "Öffnet die Dateiauswahl des Hosts für eine 3D-Puzzle-JSON-Datei; die gewählte Datei ersetzt dann das gesamte Puzzle."))
            .action_describe("importSnapshot", LocalizedLabel::native("Replaces the whole 3D puzzle with one read from an imported JSON file; the previous puzzle is discarded.", "Ersetzt das gesamte 3D-Puzzle durch eines aus einer importierten JSON-Datei; das bisherige Puzzle wird verworfen."))
            .action_describe("setSelectionFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given or selected objects.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen oder ausgewählten Objekte."))
            .action_describe("setSelectionHidden", LocalizedLabel::native("Sets the given or selected objects hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt die angegebenen oder ausgewählten Objekte oder zeigt sie, genau auf den übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("setSelectionLocked", LocalizedLabel::native("Sets the given or selected objects locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt die angegebenen oder ausgewählten Objekte oder entsperrt sie, genau auf den übergebenen Wert; eine Wiederholung ändert nichts."))
            .action_describe("acceptSuggestion", LocalizedLabel::native("Places the suggested piece chosen from the suggestion list (by index, or the highlighted one) at its connection point.", "Setzt das aus der Vorschlagsliste gewählte Teil (per Index oder das hervorgehobene) an seinem Anschlusspunkt."))
            .action_describe("selectSameKindSelection", LocalizedLabel::native("Extends the selection to every piece of the same kind as the selected one.", "Erweitert die Auswahl auf alle Teile derselben Art wie das ausgewählte."))
            .action_describe("toggleSun", LocalizedLabel::native("Switches the 3D view's sun light on or off; only the view changes.", "Schaltet das Sonnenlicht der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("setSunAzimuth", LocalizedLabel::native("Sets the compass direction the 3D view's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne der 3D-Ansicht scheint; nur die Ansicht ändert sich."))
            .action_describe("setSunElevation", LocalizedLabel::native("Sets how high the 3D view's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne der 3D-Ansicht über dem Horizont steht; nur die Ansicht ändert sich."))
            .action_describe("setSunIntensity", LocalizedLabel::native("Sets the brightness of the 3D view's sun; only the view changes.", "Legt die Helligkeit der Sonne der 3D-Ansicht fest; nur die Ansicht ändert sich."))
            .action_describe("setLodAutomatic", LocalizedLabel::native("Turns automatic level of detail in the 3D view on or off; only the view changes.", "Schaltet die automatische Detailstufe der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("setLodDepthVariable", LocalizedLabel::native("Turns depth-dependent level of detail in the 3D view on or off; only the view changes.", "Schaltet die tiefenabhängige Detailstufe der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("setLodManual", LocalizedLabel::native("Sets a fixed level of detail for the 3D view; only the view changes.", "Legt eine feste Detailstufe für die 3D-Ansicht fest; nur die Ansicht ändert sich."))
            .action_describe("setGridVisible", LocalizedLabel::native("Shows or hides the placement grid in the 3D view; only the view changes.", "Blendet das Platzierungsraster in der 3D-Ansicht ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("setGridSnapEnabled", LocalizedLabel::native("Turns snapping to the placement grid on or off for moves in the 3D view.", "Schaltet das Fangen am Platzierungsraster für Verschiebungen in der 3D-Ansicht ein oder aus."))
            .action_describe("setGridSpacing", LocalizedLabel::native("Sets the spacing of the placement grid.", "Legt den Abstand des Platzierungsrasters fest."))
            .action_describe("setProximityRadius", LocalizedLabel::native("Sets how close two pieces must come for a move or Connect Nearby to join their connection points.", "Legt fest, wie nahe sich zwei Teile kommen müssen, damit ein Verschieben oder In der Nähe verbinden ihre Anschlusspunkte verbindet."))
            .action_describe("setChunkSize", LocalizedLabel::native("Sets the size of the spatial chunks the 3D view loads and draws pieces in; only the view changes.", "Legt die Größe der räumlichen Blöcke fest, in denen die 3D-Ansicht Teile lädt und zeichnet; nur die Ansicht ändert sich."))
            .action_describe("setSelectableKind", LocalizedLabel::native("Sets whether pieces, connection points or attractions can be picked in the 3D view.", "Legt fest, ob Teile, Anschlusspunkte oder Anziehungen in der 3D-Ansicht gewählt werden können."))
            .action_describe("setTransformGumballFlag", LocalizedLabel::native("Switches one option of the transform gumball (such as translate, rotate or scale handles) on or off.", "Schaltet eine Option des Transformationsgriffs (etwa Verschiebe-, Dreh- oder Skaliergriffe) ein oder aus."))
            .action_describe("setVoxelDims", LocalizedLabel::native("Sets one dimension (width, depth or height) of the voxel volume the fill tool packs pieces into.", "Legt eine Abmessung (Breite, Tiefe oder Höhe) des Voxelvolumens fest, in das das Füllwerkzeug Teile packt."))
            .action_describe("relocateTargetVolume", LocalizedLabel::native("Moves one target volume to a new position.", "Verschiebt ein Zielvolumen an eine neue Position."))
            .action_describe("setFillCount", LocalizedLabel::native("Sets how many pieces the fill tool places into the target volume.", "Legt fest, wie viele Teile das Füllwerkzeug in das Zielvolumen setzt."))
            .action_describe("setBrushPlacementContactTolerance", LocalizedLabel::native("Sets how much overlap (0 to 1) the placement brush tolerates between a suggested piece and existing ones.", "Legt fest, wie viel Überlappung (0 bis 1) der Platzierungspinsel zwischen einem vorgeschlagenen und vorhandenen Teilen zulässt."))
            .action_describe("cycleBrushCandidate", LocalizedLabel::native("Switches the placement brush to the next suggested piece.", "Schaltet den Platzierungspinsel zum nächsten vorgeschlagenen Teil."))
            .action_describe("cycleBrushCandidateBack", LocalizedLabel::native("Switches the placement brush to the previous suggested piece.", "Schaltet den Platzierungspinsel zum vorherigen vorgeschlagenen Teil."))
            .action_describe("openVortexSuggestions", LocalizedLabel::native("Opens the list of pieces that fit at one connection point.", "Öffnet die Liste der Teile, die an einen Anschlusspunkt passen."))
            .action_describe("closeVortexSuggestions", LocalizedLabel::native("Closes the list of suggested pieces.", "Schließt die Liste der vorgeschlagenen Teile."))
            .action_describe("addTargetVolume", LocalizedLabel::native("Adds a target volume, a box region the fill tool packs pieces into.", "Fügt ein Zielvolumen hinzu, einen Quaderbereich, in den das Füllwerkzeug Teile packt."))
            .action_describe("deleteTargetVolume", LocalizedLabel::native("Deletes one target volume by id.", "Löscht ein Zielvolumen anhand seiner Id."))
            .action_describe("setTargetVolumeFlag", LocalizedLabel::native("Sets one flag (such as hidden or locked) on the given target volumes.", "Setzt eine Markierung (etwa verborgen oder gesperrt) auf den angegebenen Zielvolumen."))
            .action_describe("openAddObjectDialog", LocalizedLabel::native("Opens the Add Object dialog to pick an object kind to place.", "Öffnet den Dialog Objekt hinzufügen, um eine zu setzende Objektart zu wählen."))
            .action_describe("addObjectKind", LocalizedLabel::native("Adds an object of the given kind (a 3D block kind) to the puzzle.", "Fügt dem Puzzle ein Objekt der angegebenen Art (einer 3D-Blockart) hinzu."))
            .action_describe("worldRelocate", LocalizedLabel::native("Moves one object to a new position and connects it to nearby compatible connection points.", "Verschiebt ein Objekt an eine neue Position und verbindet es mit verträglichen Anschlusspunkten in der Nähe."))
            .action_describe("patchInspector", LocalizedLabel::native("Sets one inspector field on the given or selected objects or connection points.", "Setzt ein Inspektorfeld auf den angegebenen oder ausgewählten Objekten oder Anschlusspunkten."))
            .action_describe("createAttraction", LocalizedLabel::native("Creates an attraction that pulls two compatible connection points (vortices) of different objects together.", "Erstellt eine Anziehung, die zwei verträgliche Anschlusspunkte (Wirbel) verschiedener Objekte zusammenzieht."))
            .action_describe("deleteAttraction", LocalizedLabel::native("Deletes one attraction by id, releasing its two connection points.", "Löscht eine Anziehung anhand ihrer Id und gibt ihre zwei Anschlusspunkte frei."))
            .action_describe("addBrushObject", LocalizedLabel::native("Places an object with the placement brush at the given contact point, as the brush suggests.", "Setzt mit dem Platzierungspinsel ein Objekt am angegebenen Kontaktpunkt, wie der Pinsel es vorschlägt."))
            .action_describe("setVortexShow", LocalizedLabel::native("Sets whether connection points are shown always or only on selected objects; only the view changes.", "Legt fest, ob Anschlusspunkte immer oder nur an ausgewählten Objekten gezeigt werden; nur die Ansicht ändert sich."))
            .action_describe("setVortexDirection", LocalizedLabel::native("Sets how connection point directions are drawn; only the view changes.", "Legt fest, wie die Richtungen der Anschlusspunkte gezeichnet werden; nur die Ansicht ändert sich."))
            .action_describe("setObjectKindWeight", LocalizedLabel::native("Sets how often the fill tool and brush pick one object kind relative to the others.", "Legt fest, wie oft Füllwerkzeug und Pinsel eine Objektart im Verhältnis zu den anderen wählen."))
            .action_describe("setVortexKindWeight", LocalizedLabel::native("Sets how strongly one connection point kind is preferred when suggesting placements.", "Legt fest, wie stark eine Anschlusspunktart bei Platzierungsvorschlägen bevorzugt wird."))
            .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("setProjection", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("setProjectionParam", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("engagementRepeatLast", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("engagementControlSelect", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("targetBrushSuggestions", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("registerBrushMesh", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("setTransformGumballFlag", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("importSnapshot")
            .build_definition()
}
//#endregion 🔖️Manifest
