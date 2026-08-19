//! 🗣️ Flow play app — the single `app_labels!` block plus the locale resolvers every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale×terminology combination is compile-checked in one place.

use crate::editor::flow::config::FlowConfig;
use semio_framework_plugin::Label;

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the flow app; one field per label makes every locale×terminology combination compile-checked. `FlowConfig` carries no terminology axis, so `reuse_*` mirrors `native_*` throughout.
    pub struct FlowPlayLabels {
        widgets: native_en "Widgets", native_de "Widgets", reuse_en "Widgets", reuse_de "Widgets";
        synapses: native_en "Synapses", native_de "Synapsen", reuse_en "Synapses", reuse_de "Synapsen";
        extensions: native_en "Extensions", native_de "Erweiterungen", reuse_en "Extensions", reuse_de "Erweiterungen";
        extension_actions: native_en "Extension Actions", native_de "Erweiterungsaktionen", reuse_en "Extension Actions", reuse_de "Erweiterungsaktionen";
        sources: native_en "Sources", native_de "Quellen", reuse_en "Sources", reuse_de "Quellen";
        components: native_en "Components", native_de "Komponenten", reuse_en "Components", reuse_de "Komponenten";
        sinks: native_en "Sinks", native_de "Senken", reuse_en "Sinks", reuse_de "Senken";
        catalogue_slider: native_en "Slider", native_de "Schieberegler", reuse_en "Slider", reuse_de "Schieberegler";
        catalogue_note: native_en "Note", native_de "Notiz", reuse_en "Note", reuse_de "Notiz";
        catalogue_add: native_en "Add", native_de "Addieren", reuse_en "Add", reuse_de "Addieren";
        catalogue_and: native_en "And", native_de "Und", reuse_en "And", reuse_de "Und";
        catalogue_concat: native_en "Concat", native_de "Verketten", reuse_en "Concat", reuse_de "Verketten";
        catalogue_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        catalogue_export: native_en "Export", native_de "Exportieren", reuse_en "Export", reuse_de "Exportieren";
        extension_auto_layout: native_en "Auto Layout", native_de "Automatisches Layout", reuse_en "Auto Layout", reuse_de "Automatisches Layout";
        extension_auto_evaluate: native_en "Auto Evaluate", native_de "Automatisch Auswerten", reuse_en "Auto Evaluate", reuse_de "Automatisch Auswerten";
        extension_action_reorganize_canvas: native_en "Reorganize Canvas", native_de "Leinwand neu anordnen", reuse_en "Reorganize Canvas", reuse_de "Leinwand neu anordnen";
        extension_action_evaluate_fixture: native_en "Evaluate Fixture", native_de "Fixture auswerten", reuse_en "Evaluate Fixture", reuse_de "Fixture auswerten";
        canvas: native_en "Canvas", native_de "Leinwand", reuse_en "Canvas", reuse_de "Leinwand";
        widget: native_en "Widget", native_de "Widget", reuse_en "Widget", reuse_de "Widget";
        delete_selection: native_en "Delete selection", native_de "Auswahl löschen", reuse_en "Delete selection", reuse_de "Auswahl löschen";
        duplicate_widget: native_en "Duplicate", native_de "Duplizieren", reuse_en "Duplicate", reuse_de "Duplizieren";
        hide_preview: native_en "Hide preview", native_de "Vorschau ausblenden", reuse_en "Hide preview", reuse_de "Vorschau ausblenden";
        show_preview: native_en "Show preview", native_de "Vorschau einblenden", reuse_en "Show preview", reuse_de "Vorschau einblenden";
        add_node: native_en "Add node…", native_de "Knoten hinzufügen…", reuse_en "Add node…", reuse_de "Knoten hinzufügen…";
        reorganize: native_en "Reorganize", native_de "Neu anordnen", reuse_en "Reorganize", reuse_de "Neu anordnen";
        replace_image: native_en "Replace image…", native_de "Bild ersetzen…", reuse_en "Replace image…", reuse_de "Bild ersetzen…";
        window_main: native_en "Flow", native_de "Flow", reuse_en "Flow", reuse_de "Flow";
        window_compiled: native_en "DSL", native_de "DSL", reuse_en "DSL", reuse_de "DSL";
        window_generations: native_en "Generations", native_de "Generationen", reuse_en "Generations", reuse_de "Generationen";
        window_generate_form: native_en "Form", native_de "Formular", reuse_en "Form", reuse_de "Formular";
        window_generate_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        lod_mode: native_en "LOD Mode", native_de "LOD-Modus", reuse_en "LOD Mode", reuse_de "LOD-Modus";
        automatic: native_en "Automatic", native_de "Automatisch", reuse_en "Automatic", reuse_de "Automatisch";
        proximity_distance: native_en "Proximity Distance", native_de "Näheabstand", reuse_en "Proximity Distance", reuse_de "Näheabstand";
        grid: native_en "Grid", native_de "Raster", reuse_en "Grid", reuse_de "Raster";
        grid_visible: native_en "Visible", native_de "Sichtbar", reuse_en "Visible", reuse_de "Sichtbar";
        grid_snap: native_en "Snap", native_de "Fang", reuse_en "Snap", reuse_de "Fang";
        grid_factor: native_en "Factor", native_de "Faktor", reuse_en "Factor", reuse_de "Faktor";
        select_all: native_en "Select All", native_de "Alles auswählen", reuse_en "Select All", reuse_de "Alles auswählen";
        zoom_to_selection: native_en "Zoom to Selection", native_de "Auf Auswahl zoomen", reuse_en "Zoom to Selection", reuse_de "Auf Auswahl zoomen";
        clear_selection: native_en "Clear Selection", native_de "Auswahl aufheben", reuse_en "Clear Selection", reuse_de "Auswahl aufheben";
        no_selection: native_en "No selection", native_de "Keine Auswahl", reuse_en "No selection", reuse_de "Keine Auswahl";
        value: native_en "Value", native_de "Wert", reuse_en "Value", reuse_de "Wert";
        text: native_en "Text", native_de "Text", reuse_en "Text", reuse_de "Text";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        none_placeholder: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        widget_not_found: native_en "Widget not found", native_de "Widget nicht gefunden", reuse_en "Widget not found", reuse_de "Widget nicht gefunden";
        generation_needed: native_en "Add a generation to edit input values.", native_de "Füge eine Generation hinzu, um Eingabewerte zu bearbeiten.", reuse_en "Add a generation to edit input values.", reuse_de "Füge eine Generation hinzu, um Eingabewerte zu bearbeiten.";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub async fn flow_play_labels(cfg: &FlowConfig) -> &'static FlowPlayLabels {
    semio_framework_plugin::resolve_labels_for_locale::<FlowPlayLabels>(&cfg.locale)
}

/// 🗣️ Resolves a built-in extension's display name from its stable id; unknown ids fall back to the
/// extension's native English name as genuine runtime data (never authored UI copy).
pub async fn flow_extension_label(id: &str, name: &'static str, labels: &FlowPlayLabels) -> Label {
    match id {
        "auto-layout" => labels.extension_auto_layout.into(),
        "auto-evaluate" => labels.extension_auto_evaluate.into(),
        _ => Label::data(name),
    }
}

/// 🗣️ Resolves a built-in extension action's display title from its stable action id; unknown ids fall
/// back to the action's native English title as genuine runtime data.
pub async fn flow_extension_action_title_label(action_id: &str, title: &'static str, labels: &FlowPlayLabels) -> Label {
    match action_id {
        "flow.extension.reorganize" => labels.extension_action_reorganize_canvas.into(),
        "flow.extension.evaluate" => labels.extension_action_evaluate_fixture.into(),
        _ => Label::data(title),
    }
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(flow_play_labels(&FlowConfig::default()).synapses.as_str(), "Synapses");
        assert_eq!(flow_play_labels(&FlowConfig { locale: "de-DE".into(), ..FlowConfig::default() }).synapses.as_str(), "Synapsen");
    }

    #[test]
    async fn unknown_extension_ids_fall_back_to_runtime_data_labels() {
        let labels = flow_play_labels(&FlowConfig::default());
        assert_eq!(flow_extension_label("auto-layout", "Auto Layout", labels), labels.extension_auto_layout.into());
        assert_eq!(flow_extension_label("third-party", "Third Party", labels), Label::data("Third Party"));
        assert_eq!(flow_extension_action_title_label("flow.extension.evaluate", "Evaluate Fixture", labels), labels.extension_action_evaluate_fixture.into());
        assert_eq!(flow_extension_action_title_label("third.party", "Do Thing", labels), Label::data("Do Thing"));
    }
}
//#endregion 🧪️Tests
