//! 🗣️ Forms play app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale combination is compile-checked in one place.

use crate::editor::forms::config::FormsConfig;

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the forms app; one field per label makes every locale combination compile-checked.
    pub struct FormsLabels {
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        required: native_en "Required", native_de "Erforderlich", reuse_en "Required", reuse_de "Erforderlich";
        description: native_en "Description", native_de "Beschreibung", reuse_en "Description", reuse_de "Beschreibung";
        placeholder: native_en "Placeholder", native_de "Platzhalter", reuse_en "Placeholder", reuse_de "Platzhalter";
        default: native_en "Default", native_de "Standard", reuse_en "Default", reuse_de "Standard";
        min: native_en "Min", native_de "Min", reuse_en "Min", reuse_de "Min";
        max: native_en "Max", native_de "Max", reuse_en "Max", reuse_de "Max";
        step_field: native_en "Step", native_de "Schrittweite", reuse_en "Step", reuse_de "Schrittweite";
        unit: native_en "Unit", native_de "Einheit", reuse_en "Unit", reuse_de "Einheit";
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        text: native_en "Text", native_de "Text", reuse_en "Text", reuse_de "Text";
        src: native_en "Src", native_de "Quelle", reuse_en "Src", reuse_de "Quelle";
        accept: native_en "Accept", native_de "Akzeptierte Dateien", reuse_en "Accept", reuse_de "Akzeptierte Dateien";
        yes: native_en "Yes", native_de "Ja", reuse_en "Yes", reuse_de "Ja";
        no: native_en "No", native_de "Nein", reuse_en "No", reuse_de "Nein";
        option: native_en "Option", native_de "Option", reuse_en "Option", reuse_de "Option";
        remove: native_en "Remove", native_de "Entfernen", reuse_en "Remove", reuse_de "Entfernen";
        add_option: native_en "Add Option", native_de "Option hinzufügen", reuse_en "Add Option", reuse_de "Option hinzufügen";
        remove_option: native_en "Remove Option", native_de "Option entfernen", reuse_en "Remove Option", reuse_de "Option entfernen";
        add_vector_field: native_en "Add Vector Field", native_de "Vektorfeld hinzufügen", reuse_en "Add Vector Field", reuse_de "Vektorfeld hinzufügen";
        vector_field_label_suffix: native_en "label", native_de "Bezeichnung", reuse_en "label", reuse_de "Bezeichnung";
        vector_field_value_suffix: native_en "value", native_de "Wert", reuse_en "value", reuse_de "Wert";
        add_step: native_en "Add Step", native_de "Schritt hinzufügen", reuse_en "Add Step", reuse_de "Schritt hinzufügen";
        add_text_question: native_en "Add Text Question", native_de "Textfrage hinzufügen", reuse_en "Add Text Question", reuse_de "Textfrage hinzufügen";
        question: native_en "Question", native_de "Frage", reuse_en "Question", reuse_de "Frage";
        selected: native_en "selected", native_de "ausgewählt", reuse_en "selected", reuse_de "ausgewählt";
        no_steps_in_form: native_en "No steps in this form.", native_de "Keine Schritte in diesem Formular.", reuse_en "No steps in this form.", reuse_de "Keine Schritte in diesem Formular.";
        form_fallback_title: native_en "Form", native_de "Formular", reuse_en "Form", reuse_de "Formular";
        step_progress: native_en "Step", native_de "Schritt", reuse_en "Step", reuse_de "Schritt";
        back: native_en "Back", native_de "Zurück", reuse_en "Back", reuse_de "Zurück";
        next: native_en "Next", native_de "Weiter", reuse_en "Next", reuse_de "Weiter";
        submit: native_en "Submit", native_de "Absenden", reuse_en "Submit", reuse_de "Absenden";
        fixture_slug: native_en "Fixture Slug", native_de "Fixture-Slug", reuse_en "Fixture Slug", reuse_de "Fixture-Slug";
        no_steps_tree_item: native_en "(no steps)", native_de "(keine Schritte)", reuse_en "(no steps)", reuse_de "(keine Schritte)";
        actions: native_en "Actions", native_de "Aktionen", reuse_en "Actions", reuse_de "Aktionen";
        kind_text: native_en "Text", native_de "Text", reuse_en "Text", reuse_de "Text";
        kind_long_text: native_en "Long Text", native_de "Langtext", reuse_en "Long Text", reuse_de "Langtext";
        kind_number: native_en "Number", native_de "Zahl", reuse_en "Number", reuse_de "Zahl";
        kind_slider: native_en "Slider", native_de "Schieberegler", reuse_en "Slider", reuse_de "Schieberegler";
        kind_boolean: native_en "Boolean", native_de "Boolescher Wert", reuse_en "Boolean", reuse_de "Boolescher Wert";
        kind_single: native_en "Single Select", native_de "Einzelauswahl", reuse_en "Single Select", reuse_de "Einzelauswahl";
        kind_multi: native_en "Multi Select", native_de "Mehrfachauswahl", reuse_en "Multi Select", reuse_de "Mehrfachauswahl";
        kind_date: native_en "Date", native_de "Datum", reuse_en "Date", reuse_de "Datum";
        kind_color: native_en "Color", native_de "Farbe", reuse_en "Color", reuse_de "Farbe";
        kind_image: native_en "Image", native_de "Bild", reuse_en "Image", reuse_de "Bild";
        kind_file: native_en "File", native_de "Datei", reuse_en "File", reuse_de "Datei";
        kind_vector: native_en "Vector", native_de "Vektor", reuse_en "Vector", reuse_de "Vektor";
        kind_note: native_en "Note", native_de "Notiz", reuse_en "Note", reuse_de "Notiz";
        window_blueprint: native_en "Blueprint", native_de "Entwurf", reuse_en "Blueprint", reuse_de "Entwurf";
        window_try: native_en "Try", native_de "Testen", reuse_en "Try", reuse_de "Testen";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub fn forms_play_labels(cfg: &FormsConfig) -> &'static FormsLabels {
    semio_framework_plugin::resolve_labels_for_locale::<FormsLabels>(&cfg.locale)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(forms_play_labels(&FormsConfig::default()).kind_boolean.as_str(), "Boolean");
        assert_eq!(forms_play_labels(&FormsConfig { locale: "de-DE".into(), ..FormsConfig::default() }).kind_boolean.as_str(), "Boolescher Wert");
    }
}
//#endregion 🧪️Tests
