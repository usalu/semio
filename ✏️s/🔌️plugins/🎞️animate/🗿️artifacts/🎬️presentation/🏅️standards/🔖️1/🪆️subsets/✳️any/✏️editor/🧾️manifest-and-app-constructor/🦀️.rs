//! 🧾️ Manifest and app constructor: the figure tile frame argument fields and the animate presentation app constructor with its manifest stitch. See [`super`].

use super::*;

//#region 🔖️Manifest
/// 🖼️ The four fields of `FigureTileFrame`, as the declared record shape of every `#[dsl(block)]`
/// frame argument (`setFrame.frame`, `setSource.source.frame`) — one source for both declarations so
/// the published input schema cannot drift from the struct `command_from_action` decodes into.
fn figure_tile_frame_arg_fields() -> Vec<ActionArgDef> {
    vec![
        ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required().default_value(&0.0),
        ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required().default_value(&0.0),
        ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required().default_value(&1.0),
        ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required().default_value(&1.0),
    ]
}

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_animate_presentation_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::ANIMATE_DIALECT)
            .document(["semio", "animate"])
            .artifact_kind(crate::artifact_kind())
            .icon_id("animate")
            .mode_def(main::definition())
            .default_mode_id(main::PRESENTATION_PLAY_MODE_MAIN)
            .window_kind_def(tile_editor::definition())
            .default_layout(main::layout())
            .panel_tab_def(artifact::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            // ✏️ Document-mutating: dispatched as VCS operations with a true inverse.
            .mutation("seedGrid", LocalizedLabel::native("Seed Grid", "Raster erzeugen"))
            .mutation("addTile", LocalizedLabel::native("Add Tile", "Kachel hinzufügen"))
            .mutation("deleteTile", LocalizedLabel::native("Delete Tile", "Kachel löschen"))
            .action_destructive("deleteTile")
            .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .action_destructive("deleteSelection")
            .mutation("renameTiles", LocalizedLabel::native("Rename Tiles", "Kacheln umbenennen"))
            .mutation("patchTileCrops", LocalizedLabel::native("Patch Tile Crops", "Kachelzuschnitte aktualisieren"))
            .mutation("setSource", LocalizedLabel::native("Set Source", "Quelle festlegen"))
            .mutation("setFrame", LocalizedLabel::native("Set Frame", "Rahmen festlegen"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .mutation("clearTiles", LocalizedLabel::native("Clear Tiles", "Kacheln leeren"))
            .action_destructive("clearTiles")
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            // 🐚️ Host side-effect — exports the generated tile-morph prompt to the user (no document mutation).
            .action_with(semio_framework_plugin::ActionDefinition::new("copyPrompt", LocalizedLabel::native("Copy Prompt", "Prompt kopieren"), ActionKind::Shell, "copy"))
            .action_with(semio_framework_plugin::ActionDefinition::new("exportVideoFromDeck", LocalizedLabel::native("Export Video From Deck", "Video aus Deck exportieren"), ActionKind::Shell, "download"))
            // 👁️ Ephemeral view state — engagement draft, locale. Selection/hover are framework-owned
            // now (see `.interaction(...)` below): interactionSelect/interactionHover/clearSelection/
            // selectAll/setSelectionMode/setInteractionGranularity auto-inject, never declared here
            // (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
            .action_with(semio_framework_plugin::ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand"))
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .action_audience("canvasPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .view_action("noMutation", LocalizedLabel::native("No Operation", "Keine Aktion"))
            // 🎛️ Declared arg schemas for palette-parametric actions (materialized before dispatch).
            .action_args("seedGrid", vec![
                ActionArgDef::number("rows", LocalizedLabel::native("Rows", "Zeilen")).required().default_value(&2),
                ActionArgDef::number("columns", LocalizedLabel::native("Columns", "Spalten")).required().default_value(&2),
            ])
            // 🧱️ `setSource`/`setFrame` decode `#[dsl(block)]` payloads (`source`, `frame`) — the arg
            // ids and the record shapes are the reducer's, so an agent reading the published input
            // schema sends what `command_from_action` actually decodes.
            .action_args("setSource", vec![
                ActionArgDef::object(
                    "source",
                    LocalizedLabel::native("Source", "Quelle"),
                    vec![
                        ActionArgDef::text("src", LocalizedLabel::native("Source URL", "Quell-URL")).required(),
                        ActionArgDef::text("kind", LocalizedLabel::native("Kind", "Art")).required().default_value(&"image"),
                        ActionArgDef::object("frame", LocalizedLabel::native("Frame", "Rahmen"), figure_tile_frame_arg_fields()).required(),
                    ],
                )
                .required(),
            ])
            .action_args("setFrame", vec![
                ActionArgDef::object("frame", LocalizedLabel::native("Frame", "Rahmen"), figure_tile_frame_arg_fields()).required(),
            ])
            .action_args("exportVideoFromDeck", vec![ActionArgDef::text("scene", LocalizedLabel::native("Presentation Scene (JSON)", "Präsentationsszene (JSON)"))])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new("demo", LocalizedLabel::native("Demo", "Demo"))])
                    .required()
                    .default_value(&"demo"),
            ])
            // 🎛️ App-scope command — see `🎮️commands/🌱️seed-grid::reset_grid`'s doc comment for why this
            // isn't `seedGrid`/`clearTiles`.
            .app_command("resetGrid", LocalizedLabel::native("Reset to Default Grid", "Auf Standardraster zurücksetzen"), "document", ActionKind::Mutation)
            .action_interactive_job("seedGrid", InteractiveJobClassification::Migrated)
            .action_interactive_job("addTile", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteTile", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("renameTiles", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchTileCrops", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSource", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFrame", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearTiles", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("resetGrid", InteractiveJobClassification::Migrated)
            .action_destructive("resetGrid")
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("noMutation", InteractiveJobClassification::Migrated)
            .action_interactive_job("copyPrompt", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportVideoFromDeck", InteractiveJobClassification::Migrated)
            .action_destructive("exportVideoFromDeck")
            // 🕹️ The framework-owned "tiles" interaction domain (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — covers both the document panel
            // tree (`.interaction_domain("tiles")`) and the tile-editor canvas's pick selection;
            // auto-injects interactionSelect/interactionHover/clearSelection/selectAll/setSelectionMode/
            // setInteractionGranularity, replacing the deleted bespoke `setSelectedIds` view action.
            .interaction(InteractionDefinition {
                id: PRESENTATION_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Tiles", "Kacheln"),
                granularities: vec![GranularityDefinition { id: PRESENTATION_INTERACTION_GRANULARITY.into(), label: LocalizedLabel::native("Tile", "Kachel"), icon_id: "square".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(tile_editor::PRESENTATION_PLAY_WINDOW_MAIN, vec![InteractionRef::new(PRESENTATION_INTERACTION_DOMAIN)])
            .config(AnimatePresentationPlayApp::config_spec())
            .io(presentation_io())
            // 🚧️ SDK GAP (contract §2.4): no `.example(...)`/`.workflow(...)` on `EditorBuilder` — the
            // old `crate::examples::art_presentation_demo::source()` app-level example registration and the
            // no-op `.workflow("animate", "Animate", "deck")` call are dropped here (not silently:
            // reported in this packet's migration notes). The subset's own `📚️examples/🎬️demo` facet
            // (`crate::examples::...`, real content, pre-existing) is the modern,
            // role-agnostic replacement surface for this.
            .action_describe("seedGrid", LocalizedLabel::native("Replaces every tile of the deck with a fresh grid of the given rows and columns cut from the source image; the previous tiles are discarded.", "Ersetzt alle Kacheln des Decks durch ein neues Raster mit den angegebenen Zeilen und Spalten aus dem Quellbild; die bisherigen Kacheln werden verworfen."))
            .action_describe("addTile", LocalizedLabel::native("Adds one new tile cropping the given region of the source image (a small square near the top left when none is given) and selects it.", "Fügt eine neue Kachel hinzu, die den angegebenen Bereich des Quellbilds ausschneidet (ohne Angabe ein kleines Quadrat oben links), und wählt sie aus."))
            .action_describe("deleteTile", LocalizedLabel::native("Deletes one tile by id from the deck; its crop and name are gone unless the edit is undone.", "Löscht eine Kachel anhand ihrer Id aus dem Deck; Zuschnitt und Name sind fort, sofern die Änderung nicht rückgängig gemacht wird."))
            .action_describe("deleteSelection", LocalizedLabel::native("Deletes every currently selected tile from the deck.", "Löscht alle aktuell ausgewählten Kacheln aus dem Deck."))
            .action_describe("renameTiles", LocalizedLabel::native("Gives every tile with the given ids the same new name; an empty name changes nothing.", "Gibt allen Kacheln mit den angegebenen Ids denselben neuen Namen; ein leerer Name ändert nichts."))
            .action_describe("patchTileCrops", LocalizedLabel::native("Sets one crop coordinate (x, y, width or height, as a fraction of the source image) on every tile with the given ids.", "Setzt eine Zuschnittkoordinate (x, y, Breite oder Höhe, als Anteil des Quellbilds) auf allen Kacheln mit den angegebenen Ids."))
            .action_describe("setSource", LocalizedLabel::native("Sets the source image the tiles are cut from; choosing a different image discards every existing tile.", "Legt das Quellbild fest, aus dem die Kacheln geschnitten werden; ein anderes Bild verwirft alle vorhandenen Kacheln."))
            .action_describe("setFrame", LocalizedLabel::native("Moves and resizes the frame the source image is shown in within the presentation (x, y, width, height).", "Verschiebt und skaliert den Rahmen, in dem das Quellbild in der Präsentation erscheint (x, y, Breite, Höhe)."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole presentation with the bundled demo deck; any other example id changes nothing.", "Ersetzt die gesamte Präsentation durch das mitgelieferte Demo-Deck; jede andere Beispiel-Id ändert nichts."))
            .action_describe("clearTiles", LocalizedLabel::native("Removes every tile from the deck and keeps only the source image.", "Entfernt alle Kacheln aus dem Deck und behält nur das Quellbild."))
            .action_describe("copyPrompt", LocalizedLabel::native("Writes a tile-morph prompt describing the source image and every tile to a downloaded tile-morph-prompt.md file on the user's machine.", "Schreibt einen Tile-Morph-Prompt, der das Quellbild und jede Kachel beschreibt, in eine heruntergeladene Datei tile-morph-prompt.md auf dem Rechner des Nutzers."))
            .action_describe("exportVideoFromDeck", LocalizedLabel::native("Renders the deck (one slide per tile), or the presentation scene given as JSON in scene, to an H.264 MP4 video on this device and downloads it; its progress and a cancel control are in the Task Manager.", "Rendert das Deck (eine Folie pro Kachel) oder die als JSON in scene übergebene Präsentationsszene auf diesem Gerät als H.264-MP4-Video und lädt es herunter; Fortschritt und Abbruch stehen im Task-Manager."))
            .action_describe("resetGrid", LocalizedLabel::native("Replaces every tile of the deck with the default 3 by 5 grid cut from the source image; the previous tiles are discarded.", "Ersetzt alle Kacheln des Decks durch das Standardraster von 3 mal 5 aus dem Quellbild; die bisherigen Kacheln werden verworfen."))
            .action_audience("noMutation", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_destructive("seedGrid")
            .action_destructive("setSource")
            .action_destructive("copyPrompt")
            .build_definition()
}
//#endregion 🔖️Manifest
