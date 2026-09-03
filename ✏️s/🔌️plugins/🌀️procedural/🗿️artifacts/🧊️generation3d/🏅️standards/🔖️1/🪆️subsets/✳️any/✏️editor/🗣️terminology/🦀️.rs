//! 🗣️ Generation3d play app — the single `app_labels!` block plus the locale resolvers every taxonomy
//! node reaches for.

use crate::editor::generation3d::config::Generation3dConfig;

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the 3D flow app; one field per label makes every locale combination compile-checked.
    pub struct Generation3dLabels {
        widgets: native_en "Widgets", native_de "Elemente", reuse_en "Widgets", reuse_de "Elemente";
        schema_prefix: native_en "Schema:", native_de "Schema:", reuse_en "Schema:", reuse_de "Schema:";
        widgets_prefix: native_en "Widgets:", native_de "Elemente:", reuse_en "Widgets:", reuse_de "Elemente:";
        no_selection: native_en "No selection", native_de "Keine Auswahl", reuse_en "No selection", reuse_de "Keine Auswahl";
        id_field: native_en "Id", native_de "ID", reuse_en "Id", reuse_de "ID";
        value_field: native_en "Value", native_de "Wert", reuse_en "Value", reuse_de "Wert";
        range_field: native_en "Range", native_de "Bereich", reuse_en "Range", reuse_de "Bereich";
        widget_group: native_en "Widget", native_de "Element", reuse_en "Widget", reuse_de "Element";
        generate_hint: native_en "Add a generation to edit input values.", native_de "Erstelle eine Generation, um Eingabewerte zu bearbeiten.", reuse_en "Add a generation to edit input values.", reuse_de "Erstelle eine Generation, um Eingabewerte zu bearbeiten.";
        preview_hint: native_en "(evaluate a generation to preview output)", native_de "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)", reuse_en "(evaluate a generation to preview output)", reuse_de "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)";
        catalog_neuron: native_en "Neuron", native_de "Neuron", reuse_en "Neuron", reuse_de "Neuron";
        catalog_slider: native_en "Slider", native_de "Schieberegler", reuse_en "Slider", reuse_de "Schieberegler";
        catalog_note: native_en "Note", native_de "Notiz", reuse_en "Note", reuse_de "Notiz";
        catalog_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        window_flow: native_en "Flow", native_de "Workflow", reuse_en "Flow", reuse_de "Workflow";
        window_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        window_generations: native_en "Generations", native_de "Generationen", reuse_en "Generations", reuse_de "Generationen";
        window_generate_form: native_en "Form", native_de "Formular", reuse_en "Form", reuse_de "Formular";
        window_generate_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        delete_selection: native_en "Delete selection", native_de "Auswahl löschen", reuse_en "Delete selection", reuse_de "Auswahl löschen";
    }
}

/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub fn generation3d_labels(cfg: &Generation3dConfig) -> &'static Generation3dLabels {
    semio_framework_plugin::resolve_labels_for_locale::<Generation3dLabels>(&cfg.locale)
}

/// 🗣️ Resolves a catalogue widget kind's display label from its stable id; unknown kinds fall back to
/// the id itself.
pub fn generation3d_catalog_label(kind: &'static str, labels: &Generation3dLabels) -> &'static str {
    match kind {
        "neuron" => labels.catalog_neuron.as_str(),
        "inputSlider" => labels.catalog_slider.as_str(),
        "inputNote" => labels.catalog_note.as_str(),
        "outputPreview" => labels.catalog_preview.as_str(),
        _ => kind,
    }
}
//#endregion 🔖️Labels

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(generation3d_labels(&Generation3dConfig::default()).widgets.as_str(), "Widgets");
        assert_eq!(generation3d_labels(&Generation3dConfig { locale: "de-DE".into(), ..Generation3dConfig::default() }).widgets.as_str(), "Elemente");
    }

    #[test]
    fn unknown_catalog_kind_falls_back_to_the_id_itself() {
        let labels = generation3d_labels(&Generation3dConfig::default());
        assert_eq!(generation3d_catalog_label("bogusKind", labels), "bogusKind");
    }
}
//#endregion 🧪️Tests
