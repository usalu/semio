//! 🧱️ Create layout app: the manifest stitch that builds the layout app definition from the per node definitions. See [`super`].

use super::*;

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_layout_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(semio_s_artifact_layout_layout_model::LAYOUT_DIALECT)
            .artifact_kind(ArtifactKindSpec {
                id: "2d.layout".into(),
                label: LocalizedLabel::native("Layout", "Layout"),
                source_format: "layout.layout".into(),
                component_kind: "layout".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                schema: "layout.layout".into(),
                export_formats: vec![],
                import_formats: vec![],
                export_stdio_kinds: vec!["stdio.svg".into(), "stdio.png".into()],
                import_stdio_kinds: vec!["stdio.svg".into(), "stdio.png".into()],
            })
            .document(["semio", "layout"])
            .icon_id("layout")
            .mode_def(edit::definition())
            .default_mode_id(edit::LAYOUT_PLAY_MODE_EDIT)
            .window_kind_def(blueprint::definition())
            .window_kind_def(preview::definition())
            .utility(blueprint::select::definition())
            .utility(blueprint::transform::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(preflight_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("addFrame", LocalizedLabel::native("Add Frame", "Rahmen hinzufügen"))
            .mutation("addPage", LocalizedLabel::native("Add Page", "Seite hinzufügen"))
            .action_args("addFrame", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("rect", LocalizedLabel::native("Rectangle", "Rechteck")),
                    ActionArgOption::new("text", LocalizedLabel::native("Text Frame", "Textrahmen")),
                    ActionArgOption::new("image", LocalizedLabel::native("Image Frame", "Bildrahmen")),
                ]).default_value(&"rect"),
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")),
            ])
            .shell_action("exportPng", LocalizedLabel::native("Export Png", "Png exportieren"))
            .shell_action("exportSvg", LocalizedLabel::native("Export Svg", "Svg exportieren"))
            .shell_action("exportPdf", LocalizedLabel::native("Export Pdf", "Pdf exportieren"))
            .shell_action("exportPackage", LocalizedLabel::native("Export Package", "Paket exportieren"))
            .action_with(layout_internal_action("patchPage", LocalizedLabel::native("Patch Page", "Seite aktualisieren"), ActionKind::Mutation))
            .action_with(layout_internal_action("patchFrame", LocalizedLabel::native("Patch Frame", "Rahmen aktualisieren"), ActionKind::Mutation))
            .action_with(layout_internal_action("patchDocument", LocalizedLabel::native("Patch Document", "Dokument aktualisieren"), ActionKind::Mutation))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection") })
            .action_destructive("deleteSelection")
            .action_destructive("exportPng")
            .action_destructive("exportSvg")
            .action_destructive("exportPdf")
            .action_destructive("exportPackage")
            .action_with(layout_internal_action("canvasDrop", LocalizedLabel::native("Canvas Drop", "Ablegen auf Leinwand"), ActionKind::Mutation))
            .action_with(layout_internal_action("setActivePage", LocalizedLabel::native("Set Active Page", "Aktive Seite festlegen"), ActionKind::View))
            .action_with(layout_internal_action("focusPreflightIssue", LocalizedLabel::native("Focus Preflight Issue", "Preflight-Problem fokussieren"), ActionKind::View))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::View, "mouse-pointer") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegen"), ActionKind::View, "mouse-pointer") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), ActionKind::View, "mouse-pointer") })
            .action_with(layout_internal_action("canvasDragOver", LocalizedLabel::native("Canvas Drag Over", "Ziehen über Leinwand"), ActionKind::View))
            .action_with(layout_internal_action("canvasDragLeave", LocalizedLabel::native("Canvas Drag Leave", "Ziehen verlässt Leinwand"), ActionKind::View))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_with(layout_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Shell))
            .action_describe("addPage", LocalizedLabel::native("Appends a new page to the layout document.", "Fügt dem Layoutdokument eine neue Seite hinzu."))
            .action_use_when("addPage", vec!["add a page".into(), "new page in the layout".into()])
            .action_describe("addFrame", LocalizedLabel::native("Places a new frame on a page — the box that holds an image, a text block or a nested artifact.", "Platziert einen neuen Rahmen auf einer Seite — den Kasten für Bild, Textblock oder eingebettetes Artefakt."))
            .action_use_when("addFrame", vec!["add a frame".into(), "place an image box".into(), "add a text frame".into()])
            .action_describe("patchPage", LocalizedLabel::native("Sets one named property of one page — its size, orientation, margins or name.", "Setzt eine benannte Eigenschaft einer Seite — Größe, Ausrichtung, Ränder oder Name."))
            .action_use_when("patchPage", vec!["make the page A4".into(), "change the page margins".into()])
            .action_describe("patchFrame", LocalizedLabel::native("Sets one named property of one frame — its position, size, content binding or style.", "Setzt eine benannte Eigenschaft eines Rahmens — Position, Größe, Inhaltsbindung oder Stil."))
            .action_use_when("patchFrame", vec!["move a frame".into(), "resize this frame".into()])
            .action_describe("patchDocument", LocalizedLabel::native("Sets one named property of the layout document — its name, print target, or data fields.", "Setzt eine benannte Eigenschaft des Layoutdokuments — Name, Druckziel oder Datenfelder."))
            .action_use_when("patchDocument", vec!["rename the layout".into(), "set the print target".into()])
            .action_describe("deleteSelection", LocalizedLabel::native("Removes the selected pages or frames from the layout.", "Entfernt die ausgewählten Seiten oder Rahmen aus dem Layout."))
            .action_describe("exportPdf", LocalizedLabel::native("Renders the whole layout to a downloadable PDF.", "Rendert das gesamte Layout in eine herunterladbare PDF-Datei."))
            .action_use_when("exportPdf", vec!["export the document as pdf".into(), "print this layout to pdf".into()])
            .action_describe("exportPng", LocalizedLabel::native("Renders the layout to downloadable PNG raster images.", "Rendert das Layout in herunterladbare PNG-Rasterbilder."))
            .action_describe("exportSvg", LocalizedLabel::native("Renders the layout to a downloadable SVG vector document.", "Rendert das Layout in ein herunterladbares SVG-Vektordokument."))
            .action_describe("exportPackage", LocalizedLabel::native("Bundles the layout and every asset it references into one downloadable package.", "Bündelt das Layout und alle referenzierten Assets in ein herunterladbares Paket."))
            .action_describe("setActivePage", LocalizedLabel::native("Brings one page of the layout into view.", "Holt eine Seite des Layouts in die Ansicht."))
            .action_describe("focusPreflightIssue", LocalizedLabel::native("Scrolls to and highlights one reported preflight issue.", "Springt zu einem gemeldeten Preflight-Problem und hebt es hervor."))
            .action_audience("canvasDrop", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("canvasDragOver", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("canvasDragLeave", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("canvasPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("canvasPointerMove", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("canvasPointerUp", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("setActivePage", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusPreflightIssue", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasDragOver", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasDragLeave", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportPng", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportSvg", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportPdf", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportPackage", InteractiveJobClassification::Migrated)
            .action_interactive_job("addFrame", InteractiveJobClassification::Migrated)
            .action_interactive_job("addPage", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchPage", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchFrame", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchDocument", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasDrop", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
            .action_with(layout_internal_action("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Mutation))
            .action_with(layout_internal_action("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Mutation))
            .action_with(layout_internal_action("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Mutation))
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_describe("translateSelection", LocalizedLabel::native("Moves the selected frames by a delta in page units.", "Verschiebt die ausgewählten Rahmen um einen Versatz in Seiteneinheiten."))
            .action_use_when("translateSelection", vec!["move the selected frames".into(), "shift this frame to the right".into()])
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the selected frames by an angle in radians about the centroid of their centres.", "Dreht die ausgewählten Rahmen um einen Winkel im Bogenmaß um den Schwerpunkt ihrer Mittelpunkte."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the selected frames by a factor per axis about the centroid of their centres.", "Skaliert die ausgewählten Rahmen um einen Faktor je Achse um den Schwerpunkt ihrer Mittelpunkte."))
            .window_kind_action_refs(LAYOUT_PLAY_WINDOW_BLUEPRINT, vec![
                "addFrame".into(), "addPage".into(), "patchPage".into(), "patchFrame".into(), "patchDocument".into(), "translateSelection".into(), "rotateSelection".into(), "scaleSelection".into(),
            ])
            .interaction(InteractionDefinition {
                id: LAYOUT_INTERACTION_ELEMENTS.into(),
                label: LocalizedLabel::native("Elements", "Elemente"),
                granularities: vec![GranularityDefinition { id: LAYOUT_GRANULARITY_ELEMENT.into(), label: LocalizedLabel::native("Element", "Element"), icon_id: "square".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(LAYOUT_PLAY_WINDOW_BLUEPRINT, vec![InteractionRef::new(LAYOUT_INTERACTION_ELEMENTS)])
            .io(semio_s_artifact_layout_layout_engine::layout_io())
            .build_definition()
}
//#endregion 🔖️Manifest
