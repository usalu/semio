//! 🗣️ Layout play app — the single `app_labels!` block plus the locale resolvers every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale×terminology combination is compile-checked in one place. `LayoutConfig`
//! carries no terminology axis, so `reuse_*` mirrors `native_*` throughout.

use crate::editor::layout::config::LayoutConfig;
use semio_framework_plugin::{Label, LabelText};

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the layout app; one field per label makes every locale combination compile-checked.
    pub struct LayoutLabels {
        document: native_en "Document", native_de "Dokument", reuse_en "Document", reuse_de "Dokument";
        spreads: native_en "Spreads", native_de "Doppelseiten", reuse_en "Spreads", reuse_de "Doppelseiten";
        frames: native_en "Frames", native_de "Rahmen", reuse_en "Frames", reuse_de "Rahmen";
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
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from the config-carried locale; unknown locales fall back to native English.
pub async fn layout_labels(cfg: &LayoutConfig) -> &'static LayoutLabels {
    semio_framework_plugin::resolve_labels_for_locale::<LayoutLabels>(&cfg.locale)
}

/// 🗣️ Resolves a catalogue frame kind's display label from its stable id; unknown kinds fall back to the kind id itself.
pub async fn catalogue_kind_label(kind: &'static str, labels: &LayoutLabels) -> Label {
    match kind {
        "rect" => labels.kind_rect.into(),
        "text" => labels.kind_text.into(),
        "image" => labels.kind_image.into(),
        _ => Label::data(kind),
    }
}

/// 🗣️ Fills a localized preflight message template's positional `{}` placeholders, in order, with the given values.
pub async fn preflight_msg(template: LabelText, args: &[&str]) -> String {
    let mut result = template.as_str().to_string();
    for arg in args {
        result = result.replacen("{}", arg, 1);
    }
    result
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(layout_labels(&LayoutConfig::default()).frames.as_str(), "Frames");
        assert_eq!(layout_labels(&LayoutConfig { locale: "de-DE".into(), ..LayoutConfig::default() }).frames.as_str(), "Rahmen");
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_kind_label_resolves_known_kinds_and_falls_back_to_the_id() {
        let labels = layout_labels(&LayoutConfig::default());
        assert_eq!(catalogue_kind_label("rect", labels), labels.kind_rect.into());
        assert_eq!(catalogue_kind_label("bogus", labels), Label::data("bogus"));
    }

    #[semio_framework_async_macros::async_test]
    async fn preflight_msg_fills_positional_placeholders_in_order() {
        let labels = layout_labels(&LayoutConfig::default());
        assert_eq!(preflight_msg(labels.preflight_font_missing, &["Comic Sans", "frame-1"]), "Font Comic Sans used by frame-1 is not available");
    }
}
//#endregion 🧪️Tests
