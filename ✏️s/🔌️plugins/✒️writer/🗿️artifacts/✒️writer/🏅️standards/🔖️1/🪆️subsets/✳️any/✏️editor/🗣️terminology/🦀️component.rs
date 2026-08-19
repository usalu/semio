//! 🗣️ Writer play app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale combination is compile-checked in one place.

use crate::editor::writer::config::WriterConfig;

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the writer app; one field per label makes every locale combination compile-checked.
    pub struct WriterPlayLabels {
        document: native_en "Document", native_de "Dokument", reuse_en "Document", reuse_de "Dokument";
        empty_query: native_en "(empty query)", native_de "(leere Abfrage)", reuse_en "(empty query)", reuse_de "(leere Abfrage)";
        language: native_en "Language", native_de "Sprache", reuse_en "Language", reuse_de "Sprache";
        jack_description: native_en "jack — Cypher-inspired trinity query language", native_de "jack — von Cypher inspirierte Trinity-Abfragesprache", reuse_en "jack — Cypher-inspired trinity query language", reuse_de "jack — von Cypher inspirierte Trinity-Abfragesprache";
        camera: native_en "Camera", native_de "Kamera", reuse_en "Camera", reuse_de "Kamera";
        diagnostics: native_en "Diagnostics", native_de "Diagnosen", reuse_en "Diagnostics", reuse_de "Diagnosen";
        format: native_en "Format", native_de "Formatieren", reuse_en "Format", reuse_de "Formatieren";
        lint: native_en "Lint", native_de "Prüfen", reuse_en "Lint", reuse_de "Prüfen";
        line_numbers: native_en "Line numbers", native_de "Zeilennummern", reuse_en "Line numbers", reuse_de "Zeilennummern";
        font_size: native_en "Font size", native_de "Schriftgröße", reuse_en "Font size", reuse_de "Schriftgröße";
        line_height: native_en "Line height", native_de "Zeilenhöhe", reuse_en "Line height", reuse_de "Zeilenhöhe";
        tab_size: native_en "Tab size", native_de "Tabulatorgröße", reuse_en "Tab size", reuse_de "Tabulatorgröße";
        engagement_placeholder: native_en "Format, lint, line numbers", native_de "Format, prüfen, Zeilennummern", reuse_en "Format, lint, line numbers", reuse_de "Format, prüfen, Zeilennummern";
        editor_mode_status: native_en "Text editor", native_de "Texteditor", reuse_en "Text editor", reuse_de "Texteditor";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub async fn writer_play_labels(cfg: &WriterConfig) -> &'static WriterPlayLabels {
    semio_framework_plugin::resolve_labels_for_locale::<WriterPlayLabels>(&cfg.locale)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(writer_play_labels(&WriterConfig::default()).document.as_str(), "Document");
        assert_eq!(writer_play_labels(&WriterConfig { locale: "de-DE".into(), ..WriterConfig::default() }).document.as_str(), "Dokument");
    }
}
//#endregion 🧪️Tests
