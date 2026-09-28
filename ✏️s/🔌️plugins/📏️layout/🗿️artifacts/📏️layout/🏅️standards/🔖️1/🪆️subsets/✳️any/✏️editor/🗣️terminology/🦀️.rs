//! 🗣️ Layout play app — the single `app_labels!` block plus the locale resolvers every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale×terminology combination is compile-checked in one place.

use semio_framework_plugin::{Label, LabelText};

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the layout app; one field per label makes every locale combination compile-checked.
    pub struct LayoutLabels {
        artifact: native_en "Artifact", native_de "Artefakt", reuse_en "Artifact", reuse_de "Artefakt";
        spreads: native_en "Spreads", native_de "Doppelseiten", reuse_en "Spreads", reuse_de "Doppelseiten";
        frames: native_en "Frames", native_de "Rahmen", reuse_en "Frames", reuse_de "Rahmen";
        selected: native_en "Selected", native_de "Ausgewählt", reuse_en "Selected", reuse_de "Ausgewählt";
        parent_pages: native_en "Parent Pages", native_de "Übergeordnete Seiten", reuse_en "Parent Pages", reuse_de "Übergeordnete Seiten";
        layers: native_en "Layers", native_de "Ebenen", reuse_en "Layers", reuse_de "Ebenen";
        stories: native_en "Stories", native_de "Textflüsse", reuse_en "Stories", reuse_de "Textflüsse";
        links: native_en "Links", native_de "Verknüpfungen", reuse_en "Links", reuse_de "Verknüpfungen";
        styles: native_en "Styles", native_de "Formate", reuse_en "Styles", reuse_de "Formate";
        drop_here: native_en "Drop catalogue items here", native_de "Katalogelemente hier ablegen", reuse_en "Drop catalogue items here", reuse_de "Katalogelemente hier ablegen";
        catalogue_page: native_en "Page", native_de "Seite", reuse_en "Page", reuse_de "Seite";
        kind_rect: native_en "Rectangle", native_de "Rechteck", reuse_en "Rectangle", reuse_de "Rechteck";
        kind_text: native_en "Text Frame", native_de "Textrahmen", reuse_en "Text Frame", reuse_de "Textrahmen";
        kind_image: native_en "Image Frame", native_de "Bildrahmen", reuse_en "Image Frame", reuse_de "Bildrahmen";
        kind_png: native_en "PNG", native_de "PNG", reuse_en "PNG", reuse_de "PNG";
        kind_jpg: native_en "JPEG", native_de "JPEG", reuse_en "JPEG", reuse_de "JPEG";
        kind_gif: native_en "GIF", native_de "GIF", reuse_en "GIF", reuse_de "GIF";
        kind_bmp: native_en "BMP", native_de "BMP", reuse_en "BMP", reuse_de "BMP";
        kind_tiff: native_en "TIFF", native_de "TIFF", reuse_en "TIFF", reuse_de "TIFF";
        kind_pdf: native_en "PDF", native_de "PDF", reuse_en "PDF", reuse_de "PDF";
        kind_svg: native_en "SVG", native_de "SVG", reuse_en "SVG", reuse_de "SVG";
        kind_drawing: native_en "Drawing", native_de "Zeichnung", reuse_en "Drawing", reuse_de "Zeichnung";
        kind_dwg: native_en "DWG", native_de "DWG", reuse_en "DWG", reuse_de "DWG";
        kind_dxf: native_en "DXF", native_de "DXF", reuse_en "DXF", reuse_de "DXF";
        kind_raster: native_en "Raster", native_de "Raster", reuse_en "Raster", reuse_de "Raster";
        kind_bitmap: native_en "Bitmap", native_de "Bitmap", reuse_en "Bitmap", reuse_de "Bitmap";
        kind_cad: native_en "CAD", native_de "CAD", reuse_en "CAD", reuse_de "CAD";
        kind_map: native_en "Map", native_de "Karte", reuse_en "Map", reuse_de "Karte";
        kind_fem2d: native_en "FEM 2D", native_de "FEM 2D", reuse_en "FEM 2D", reuse_de "FEM 2D";
        kind_note: native_en "Note", native_de "Notiz", reuse_en "Note", reuse_de "Notiz";
        kind_writer: native_en "Writer", native_de "Text", reuse_en "Writer", reuse_de "Text";
        kind_forms: native_en "Forms", native_de "Formulare", reuse_en "Forms", reuse_de "Formulare";
        kind_flow: native_en "Flow", native_de "Fluss", reuse_en "Flow", reuse_de "Fluss";
        kind_equation: native_en "Equation", native_de "Gleichung", reuse_en "Equation", reuse_de "Gleichung";
        kind_puzzle2d: native_en "Puzzle 2D", native_de "Puzzle 2D", reuse_en "Puzzle 2D", reuse_de "Puzzle 2D";
        kind_block2d: native_en "Block 2D", native_de "Block 2D", reuse_en "Block 2D", reuse_de "Block 2D";
        kind_generation2d: native_en "Generation 2D", native_de "Generierung 2D", reuse_en "Generation 2D", reuse_de "Generierung 2D";
        kind_terrain: native_en "Terrain", native_de "Gelände", reuse_en "Terrain", reuse_de "Gelände";
        kind_grid2d: native_en "Grid 2D", native_de "Gitter 2D", reuse_en "Grid 2D", reuse_de "Gitter 2D";
        kind_wfc2d: native_en "WFC 2D", native_de "WFC 2D", reuse_en "WFC 2D", reuse_de "WFC 2D";
        kind_presentation: native_en "Presentation", native_de "Präsentation", reuse_en "Presentation", reuse_de "Präsentation";
        kind_sequence: native_en "Sequence", native_de "Sequenz", reuse_en "Sequence", reuse_de "Sequenz";
        kind_wires: native_en "Wires", native_de "Drähte", reuse_en "Wires", reuse_de "Drähte";
        rotation: native_en "Rotation", native_de "Drehung", reuse_en "Rotation", reuse_de "Drehung";
        print_target: native_en "Print Target", native_de "Druckziel", reuse_en "Print Target", reuse_de "Druckziel";
        data_fields: native_en "Data Fields", native_de "Datenfelder", reuse_en "Data Fields", reuse_de "Datenfelder";
        group_document: native_en "Document", native_de "Dokument", reuse_en "Document", reuse_de "Dokument";
        inspection: native_en "Inspection", native_de "Inspektion", reuse_en "Inspection", reuse_de "Inspektion";
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        pages: native_en "Pages", native_de "Seiten", reuse_en "Pages", reuse_de "Seiten";
        active_page: native_en "Active page", native_de "Aktive Seite", reuse_en "Active page", reuse_de "Aktive Seite";
        id: native_en "Id", native_de "ID", reuse_en "Id", reuse_de "ID";
        width: native_en "Width", native_de "Breite", reuse_en "Width", reuse_de "Breite";
        height: native_en "Height", native_de "Höhe", reuse_en "Height", reuse_de "Höhe";
        margin_top: native_en "Margin Top", native_de "Rand oben", reuse_en "Margin Top", reuse_de "Rand oben";
        margin_right: native_en "Margin Right", native_de "Rand rechts", reuse_en "Margin Right", reuse_de "Rand rechts";
        margin_bottom: native_en "Margin Bottom", native_de "Rand unten", reuse_en "Margin Bottom", reuse_de "Rand unten";
        margin_left: native_en "Margin Left", native_de "Rand links", reuse_en "Margin Left", reuse_de "Rand links";
        gutter: native_en "Gutter", native_de "Spaltenabstand", reuse_en "Gutter", reuse_de "Spaltenabstand";
        columns: native_en "Columns", native_de "Spalten", reuse_en "Columns", reuse_de "Spalten";
        page: native_en "Page", native_de "Seite", reuse_en "Page", reuse_de "Seite";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        x: native_en "X", native_de "X", reuse_en "X", reuse_de "X";
        y: native_en "Y", native_de "Y", reuse_en "Y", reuse_de "Y";
        fill: native_en "Fill", native_de "Füllung", reuse_en "Fill", reuse_de "Füllung";
        stroke: native_en "Stroke", native_de "Kontur", reuse_en "Stroke", reuse_de "Kontur";
        story: native_en "Story", native_de "Textfluss", reuse_en "Story", reuse_de "Textfluss";
        wrap_mode: native_en "Wrap Mode", native_de "Textumfluss", reuse_en "Wrap Mode", reuse_de "Textumfluss";
        wrap_none: native_en "None", native_de "Kein", reuse_en "None", reuse_de "Kein";
        wrap_box: native_en "Box", native_de "Rechteck", reuse_en "Box", reuse_de "Rechteck";
        wrap_contour: native_en "Contour", native_de "Kontur", reuse_en "Contour", reuse_de "Kontur";
        link_path: native_en "Link Path", native_de "Verknüpfungspfad", reuse_en "Link Path", reuse_de "Verknüpfungspfad";
        link_width: native_en "Pixel Width", native_de "Pixelbreite", reuse_en "Pixel Width", reuse_de "Pixelbreite";
        link_height: native_en "Pixel Height", native_de "Pixelhöhe", reuse_en "Pixel Height", reuse_de "Pixelhöhe";
        dpi: native_en "Resolution", native_de "Auflösung", reuse_en "Resolution", reuse_de "Auflösung";
        color_profile: native_en "Color Profile", native_de "Farbprofil", reuse_en "Color Profile", reuse_de "Farbprofil";
        group_page: native_en "Page", native_de "Seite", reuse_en "Page", reuse_de "Seite";
        group_frame: native_en "Frame", native_de "Rahmen", reuse_en "Frame", reuse_de "Rahmen";
        selection_not_found: native_en "Selection not found in document.", native_de "Auswahl im Dokument nicht gefunden.", reuse_en "Selection not found in document.", reuse_de "Auswahl im Dokument nicht gefunden.";
        preflight: native_en "Preflight", native_de "Preflight", reuse_en "Preflight", reuse_de "Preflight";
        no_issues: native_en "No issues", native_de "Keine Probleme", reuse_en "No issues", reuse_de "Keine Probleme";
        window_blueprint: native_en "Blueprint", native_de "Entwurf", reuse_en "Blueprint", reuse_de "Entwurf";
        window_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        parent: native_en "parent", native_de "übergeordnet", reuse_en "parent", reuse_de "übergeordnet";
        objects: native_en "objects", native_de "Objekte", reuse_en "objects", reuse_de "Objekte";
        chars: native_en "chars", native_de "Zeichen", reuse_en "chars", reuse_de "Zeichen";
        undo: native_en "Undo", native_de "Rückgängig", reuse_en "Undo", reuse_de "Rückgängig";
        redo: native_en "Redo", native_de "Wiederholen", reuse_en "Redo", reuse_de "Wiederholen";
        preflight_out_of_bounds: native_en "Object {} extends outside page bounds", native_de "Objekt {} liegt außerhalb der Seitengrenzen", reuse_en "Object {} extends outside page bounds", reuse_de "Objekt {} liegt außerhalb der Seitengrenzen";
        preflight_asset_missing: native_en "Linked asset missing for {}", native_de "Verknüpftes Element fehlt für {}", reuse_en "Linked asset missing for {}", reuse_de "Verknüpftes Element fehlt für {}";
        preflight_asset_modified: native_en "Linked asset modified for {}", native_de "Verknüpftes Element geändert für {}", reuse_en "Linked asset modified for {}", reuse_de "Verknüpftes Element geändert für {}";
        preflight_asset_low_resolution: native_en "Linked asset is low resolution for {}", native_de "Verknüpftes Element hat niedrige Auflösung für {}", reuse_en "Linked asset is low resolution for {}", reuse_de "Verknüpftes Element hat niedrige Auflösung für {}";
        preflight_image_empty_frame: native_en "Image frame {} has no preview", native_de "Bildrahmen {} hat keine Vorschau", reuse_en "Image frame {} has no preview", reuse_de "Bildrahmen {} hat keine Vorschau";
        preflight_text_missing_story: native_en "Text frame {} has no story", native_de "Textrahmen {} hat keinen Textfluss", reuse_en "Text frame {} has no story", reuse_de "Textrahmen {} hat keinen Textfluss";
        preflight_text_below_minimum_size: native_en "Text in {} is below minimum readable size", native_de "Text in {} ist kleiner als die Mindestlesbarkeitsgröße", reuse_en "Text in {} is below minimum readable size", reuse_de "Text in {} ist kleiner als die Mindestlesbarkeitsgröße";
        preflight_font_missing: native_en "Font {} used by {} is not available", native_de "Schriftart {} verwendet von {} ist nicht verfügbar", reuse_en "Font {} used by {} is not available", reuse_de "Schriftart {} verwendet von {} ist nicht verfügbar";
        preflight_text_overset: native_en "Text in {} overflows its frame", native_de "Text in {} läuft über den Rahmen hinaus", reuse_en "Text in {} overflows its frame", reuse_de "Text in {} läuft über den Rahmen hinaus";
        preflight_asset_rgb_in_print: native_en "Linked asset {} uses RGB in a print document", native_de "Verknüpftes Element {} verwendet RGB in einem Druckdokument", reuse_en "Linked asset {} uses RGB in a print document", reuse_de "Verknüpftes Element {} verwendet RGB in einem Druckdokument";
        locked: native_en "Locked", native_de "Gesperrt", reuse_en "Locked", reuse_de "Gesperrt";
        visible: native_en "Visible", native_de "Sichtbar", reuse_en "Visible", reuse_de "Sichtbar";
        baseline_grid: native_en "Baseline Grid", native_de "Grundlinienraster", reuse_en "Baseline Grid", reuse_de "Grundlinienraster";
        baseline_offset: native_en "Baseline Offset", native_de "Grundlinienversatz", reuse_en "Baseline Offset", reuse_de "Grundlinienversatz";
        snap_to_baseline: native_en "Snap to Baseline", native_de "An Grundlinie ausrichten", reuse_en "Snap to Baseline", reuse_de "An Grundlinie ausrichten";
        delete_page: native_en "Delete Page", native_de "Seite löschen", reuse_en "Delete Page", reuse_de "Seite löschen";
        move_earlier: native_en "Move Earlier", native_de "Nach vorne", reuse_en "Move Earlier", reuse_de "Nach vorne";
        move_later: native_en "Move Later", native_de "Nach hinten", reuse_en "Move Later", reuse_de "Nach hinten";
        paragraph_style: native_en "Paragraph Style", native_de "Absatzformat", reuse_en "Paragraph Style", reuse_de "Absatzformat";
        font_family: native_en "Font Family", native_de "Schriftfamilie", reuse_en "Font Family", reuse_de "Schriftfamilie";
        font_size: native_en "Font Size", native_de "Schriftgröße", reuse_en "Font Size", reuse_de "Schriftgröße";
        font_weight: native_en "Font Weight", native_de "Schriftstärke", reuse_en "Font Weight", reuse_de "Schriftstärke";
        leading: native_en "Leading", native_de "Zeilenabstand", reuse_en "Leading", reuse_de "Zeilenabstand";
        tracking: native_en "Tracking", native_de "Laufweite", reuse_en "Tracking", reuse_de "Laufweite";
        alignment: native_en "Alignment", native_de "Ausrichtung", reuse_en "Alignment", reuse_de "Ausrichtung";
        align_left: native_en "Left", native_de "Links", reuse_en "Left", reuse_de "Links";
        align_center: native_en "Center", native_de "Zentriert", reuse_en "Center", reuse_de "Zentriert";
        align_right: native_en "Right", native_de "Rechts", reuse_en "Right", reuse_de "Rechts";
        align_justify: native_en "Justify", native_de "Blocksatz", reuse_en "Justify", reuse_de "Blocksatz";
        inset_x: native_en "Inset X", native_de "Einzug X", reuse_en "Inset X", reuse_de "Einzug X";
        inset_y: native_en "Inset Y", native_de "Einzug Y", reuse_en "Inset Y", reuse_de "Einzug Y";
        inset_width: native_en "Inset Width", native_de "Einzugsbreite", reuse_en "Inset Width", reuse_de "Einzugsbreite";
        inset_height: native_en "Inset Height", native_de "Einzugshöhe", reuse_en "Inset Height", reuse_de "Einzugshöhe";
        thread_next: native_en "Next Frame", native_de "Nächster Rahmen", reuse_en "Next Frame", reuse_de "Nächster Rahmen";
        thread_none: native_en "None", native_de "Keiner", reuse_en "None", reuse_de "Keiner";
        group_layer: native_en "Layer", native_de "Ebene", reuse_en "Layer", reuse_de "Ebene";
        character_style: native_en "Character Style", native_de "Zeichenformat", reuse_en "Character Style", reuse_de "Zeichenformat";
        add_character_style: native_en "Add Character Style", native_de "Zeichenformat hinzufügen", reuse_en "Add Character Style", reuse_de "Zeichenformat hinzufügen";
        delete_character_style: native_en "Delete Character Style", native_de "Zeichenformat löschen", reuse_en "Delete Character Style", reuse_de "Zeichenformat löschen";
        italic: native_en "Italic", native_de "Kursiv", reuse_en "Italic", reuse_de "Kursiv";
        parent_page: native_en "Parent Page", native_de "Mustervorlage", reuse_en "Parent Page", reuse_de "Mustervorlage";
        spread: native_en "Spread", native_de "Druckbogen", reuse_en "Spread", reuse_de "Druckbogen";
        color: native_en "Color", native_de "Farbe", reuse_en "Color", reuse_de "Farbe";
        guide: native_en "Guide", native_de "Hilfslinie", reuse_en "Guide", reuse_de "Hilfslinie";
        add_guide: native_en "Add Guide", native_de "Hilfslinie hinzufügen", reuse_en "Add Guide", reuse_de "Hilfslinie hinzufügen";
        delete_guide: native_en "Delete Guide", native_de "Hilfslinie löschen", reuse_en "Delete Guide", reuse_de "Hilfslinie löschen";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from the shared OS-owned view context.
pub fn layout_labels(view_state: &semio_framework_plugin::ViewModel) -> &'static LayoutLabels {
    semio_framework_plugin::resolve_labels::<LayoutLabels>(view_state)
}

/// 🗣️ Resolves a catalogue frame kind's display label from its stable id; unknown kinds fall back to the kind id itself.
pub fn catalogue_kind_label(kind: &'static str, labels: &LayoutLabels) -> Label {
    match kind {
        "rect" => labels.kind_rect.into(),
        "text" => labels.kind_text.into(),
        "image" => labels.kind_image.into(),
        "png" => labels.kind_png.into(),
        "jpg" => labels.kind_jpg.into(),
        "gif" => labels.kind_gif.into(),
        "bmp" => labels.kind_bmp.into(),
        "tiff" => labels.kind_tiff.into(),
        "pdf" => labels.kind_pdf.into(),
        "svg" => labels.kind_svg.into(),
        "drawing" => labels.kind_drawing.into(),
        "dwg" => labels.kind_dwg.into(),
        "dxf" => labels.kind_dxf.into(),
        "raster" => labels.kind_raster.into(),
        "bitmap" => labels.kind_bitmap.into(),
        "cad" => labels.kind_cad.into(),
        "map" => labels.kind_map.into(),
        "fem2d" => labels.kind_fem2d.into(),
        "note" => labels.kind_note.into(),
        "writer" => labels.kind_writer.into(),
        "forms" => labels.kind_forms.into(),
        "flow" => labels.kind_flow.into(),
        "equation" => labels.kind_equation.into(),
        "puzzle2d" => labels.kind_puzzle2d.into(),
        "block2d" => labels.kind_block2d.into(),
        "generation2d" => labels.kind_generation2d.into(),
        "terrain" => labels.kind_terrain.into(),
        "grid2d" => labels.kind_grid2d.into(),
        "wfc2d" => labels.kind_wfc2d.into(),
        "presentation" => labels.kind_presentation.into(),
        "sequence" => labels.kind_sequence.into(),
        "wires" => labels.kind_wires.into(),
        _ => Label::data(kind),
    }
}

/// 🗣️ Fills a localized preflight message template's positional `{}` placeholders, in order, with the given values.
pub fn preflight_msg(template: LabelText, args: &[&str]) -> String {
    let mut result = template.as_str().to_string();
    for arg in args {
        result = result.replacen("{}", arg, 1);
    }
    result
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
