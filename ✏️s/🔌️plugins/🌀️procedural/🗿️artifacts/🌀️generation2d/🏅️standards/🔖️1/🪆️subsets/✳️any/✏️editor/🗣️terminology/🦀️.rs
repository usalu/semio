//! 🗣️ Generation2d play app — the single `app_labels!` block plus the locale resolver every taxonomy
//! node reaches for. Deliberately ONE block for the whole app.

use crate::editor::generation2d::config::Generation2dConfig;

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the 2D flow app; one field per label makes every locale combination compile-checked.
    pub struct Generation2dLabels {
        sources: native_en "Sources", native_de "Quellen", reuse_en "Sources", reuse_de "Quellen";
        components: native_en "Components", native_de "Komponenten", reuse_en "Components", reuse_de "Komponenten";
        sinks: native_en "Sinks", native_de "Senken", reuse_en "Sinks", reuse_de "Senken";
        show_mode_section: native_en "Show mode", native_de "Anzeigemodus", reuse_en "Show mode", reuse_de "Anzeigemodus";
        show_prefix: native_en "Show", native_de "Anzeigen", reuse_en "Show", reuse_de "Anzeigen";
        none: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        selection: native_en "Selection", native_de "Auswahl", reuse_en "Selection", reuse_de "Auswahl";
        ids: native_en "Ids", native_de "Kennungen", reuse_en "Ids", reuse_de "Kennungen";
        schema_prefix: native_en "Schema:", native_de "Schema:", reuse_en "Schema:", reuse_de "Schema:";
        widgets_prefix: native_en "Widgets:", native_de "Elemente:", reuse_en "Widgets:", reuse_de "Elemente:";
        show_mode_prefix: native_en "Show mode:", native_de "Anzeigemodus:", reuse_en "Show mode:", reuse_de "Anzeigemodus:";
        generate_hint: native_en "Add a generation to edit input values.", native_de "Erstelle eine Generation, um Eingabewerte zu bearbeiten.", reuse_en "Add a generation to edit input values.", reuse_de "Erstelle eine Generation, um Eingabewerte zu bearbeiten.";
        preview_hint: native_en "(evaluate a generation to preview output)", native_de "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)", reuse_en "(evaluate a generation to preview output)", reuse_de "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)";
        source_slider: native_en "Slider", native_de "Schieberegler", reuse_en "Slider", reuse_de "Schieberegler";
        source_note: native_en "Note", native_de "Notiz", reuse_en "Note", reuse_de "Notiz";
        component_add: native_en "Add", native_de "Addieren", reuse_en "Add", reuse_de "Addieren";
        component_and: native_en "And", native_de "Und", reuse_en "And", reuse_de "Und";
        component_concat: native_en "Concat", native_de "Verketten", reuse_en "Concat", reuse_de "Verketten";
        sink_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        sink_export: native_en "Export", native_de "Export", reuse_en "Export", reuse_de "Export";
        window_main: native_en "Flow", native_de "Fluss", reuse_en "Flow", reuse_de "Fluss";
        window_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        window_generations: native_en "Generations", native_de "Generationen", reuse_en "Generations", reuse_de "Generationen";
        window_generate_form: native_en "Form", native_de "Formular", reuse_en "Form", reuse_de "Formular";
        window_generate_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        delete_selection: native_en "Delete selection", native_de "Auswahl löschen", reuse_en "Delete selection", reuse_de "Auswahl löschen";
    }
}

/// 🗣️ Resolves the active label set from the config-carried locale; falls back to native English.
pub fn generation2d_labels(cfg: &Generation2dConfig) -> &'static Generation2dLabels {
    semio_framework_plugin::resolve_labels_for_locale::<Generation2dLabels>(&cfg.locale)
}
//#endregion 🔖️Labels

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(generation2d_labels(&Generation2dConfig::default()).sources.as_str(), "Sources");
        assert_eq!(generation2d_labels(&Generation2dConfig { locale: "de-DE".into(), ..Generation2dConfig::default() }).sources.as_str(), "Quellen");
    }
}
//#endregion 🧪️Tests
