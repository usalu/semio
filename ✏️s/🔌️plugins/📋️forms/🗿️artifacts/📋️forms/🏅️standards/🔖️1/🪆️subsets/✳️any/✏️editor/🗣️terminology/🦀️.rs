//! 🗣️ Forms play app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale combination is compile-checked in one place.

//#region 🔖️Labels
semio_framework_ui_locale::app_labels! {
    /// 🗣️ Complete UI label set for the forms app; one field per label makes every locale combination compile-checked.
    pub struct FormsLabels {
        extension_unavailable: native_en "Extension unavailable", native_de "Erweiterung nicht verfügbar", reuse_en "Extension unavailable", reuse_de "Erweiterung nicht verfügbar";
        title: native_en "Title", native_de "Titel", reuse_en "Title", reuse_de "Titel";
        steps: native_en "Steps", native_de "Schritte", reuse_en "Steps", reuse_de "Schritte";
        questions: native_en "Questions", native_de "Fragen", reuse_en "Questions", reuse_de "Fragen";
        options: native_en "Choices", native_de "Auswahlmöglichkeiten", reuse_en "Choices", reuse_de "Auswahlmöglichkeiten";
        parameters: native_en "Parameters", native_de "Parameter", reuse_en "Parameters", reuse_de "Parameter";
        unset: native_en "No default", native_de "Kein Standardwert", reuse_en "No default", reuse_de "Kein Standardwert";
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        visibility: native_en "Show When", native_de "Anzeigen wenn", reuse_en "Show When", reuse_de "Anzeigen wenn";
        rule: native_en "Rule", native_de "Regel", reuse_en "Rule", reuse_de "Regel";
        rule_constant: native_en "Fixed Value", native_de "Fester Wert", reuse_en "Fixed Value", reuse_de "Fester Wert";
        rule_answer: native_en "Answer", native_de "Antwort", reuse_en "Answer", reuse_de "Antwort";
        rule_equals: native_en "Equals", native_de "Ist gleich", reuse_en "Equals", reuse_de "Ist gleich";
        rule_answered: native_en "Has a Value", native_de "Hat einen Wert", reuse_en "Has a Value", reuse_de "Hat einen Wert";
        rule_all: native_en "All Rules Match", native_de "Alle Regeln treffen zu", reuse_en "All Rules Match", reuse_de "Alle Regeln treffen zu";
        rule_any: native_en "Any Rule Matches", native_de "Eine Regel trifft zu", reuse_en "Any Rule Matches", reuse_de "Eine Regel trifft zu";
        rule_always: native_en "Always", native_de "Immer", reuse_en "Always", reuse_de "Immer";
        rule_empty: native_en "Empty", native_de "Leer", reuse_en "Empty", reuse_de "Leer";
        choose_question: native_en "Choose a Question", native_de "Frage auswählen", reuse_en "Choose a Question", reuse_de "Frage auswählen";
        add_rule: native_en "Add Rule", native_de "Regel hinzufügen", reuse_en "Add Rule", reuse_de "Regel hinzufügen";
        value: native_en "Value", native_de "Wert", reuse_en "Value", reuse_de "Wert";
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
        error_required: native_en "An answer is required.", native_de "Eine Antwort ist erforderlich.", reuse_en "An answer is required.", reuse_de "Eine Antwort ist erforderlich.";
        error_type: native_en "Enter a value of the expected type.", native_de "Geben Sie einen Wert des erwarteten Typs ein.", reuse_en "Enter a value of the expected type.", reuse_de "Geben Sie einen Wert des erwarteten Typs ein.";
        error_range: native_en "Enter a value within the allowed range.", native_de "Geben Sie einen Wert im erlaubten Bereich ein.", reuse_en "Enter a value within the allowed range.", reuse_de "Geben Sie einen Wert im erlaubten Bereich ein.";
        error_step: native_en "Use the configured increment.", native_de "Verwenden Sie die festgelegte Schrittweite.", reuse_en "Use the configured increment.", reuse_de "Verwenden Sie die festgelegte Schrittweite.";
        error_option: native_en "Choose from the available options.", native_de "Wählen Sie aus den verfügbaren Optionen.", reuse_en "Choose from the available options.", reuse_de "Wählen Sie aus den verfügbaren Optionen.";
        error_date: native_en "Enter a valid date.", native_de "Geben Sie ein gültiges Datum ein.", reuse_en "Enter a valid date.", reuse_de "Geben Sie ein gültiges Datum ein.";
        error_color: native_en "Enter a color as #RRGGBB.", native_de "Geben Sie eine Farbe als #RRGGBB ein.", reuse_en "Enter a color as #RRGGBB.", reuse_de "Geben Sie eine Farbe als #RRGGBB ein.";
        error_vector: native_en "Enter a number for every vector component.", native_de "Geben Sie für jede Vektorkomponente eine Zahl ein.", reuse_en "Enter a number for every vector component.", reuse_de "Geben Sie für jede Vektorkomponente eine Zahl ein.";
        submitted: native_en "Your response was saved.", native_de "Ihre Antwort wurde gespeichert.", reuse_en "Your response was saved.", reuse_de "Ihre Antwort wurde gespeichert.";
        responses: native_en "Responses", native_de "Antworten", reuse_en "Responses", reuse_de "Antworten";
        answers: native_en "Answers", native_de "Feldantworten", reuse_en "Answers", reuse_de "Feldantworten";
        no_responses: native_en "No responses yet. Fill out the form to save a response.", native_de "Noch keine Antworten. Füllen Sie das Formular aus, um eine Antwort zu speichern.", reuse_en "No responses yet. Fill out the form to save a response.", reuse_de "Noch keine Antworten. Füllen Sie das Formular aus, um eine Antwort zu speichern.";
        unanswered: native_en "Not answered", native_de "Nicht beantwortet", reuse_en "Not answered", reuse_de "Nicht beantwortet";
        submitted_at: native_en "Submitted", native_de "Abgesendet", reuse_en "Submitted", reuse_de "Abgesendet";
        discard_response: native_en "Remove Response", native_de "Antwort entfernen", reuse_en "Remove Response", reuse_de "Antwort entfernen";
        export_json: native_en "Export Responses as JSON", native_de "Antworten als JSON exportieren", reuse_en "Export Responses as JSON", reuse_de "Antworten als JSON exportieren";
        export_csv: native_en "Export Answers as CSV", native_de "Feldantworten als CSV exportieren", reuse_en "Export Answers as CSV", reuse_de "Feldantworten als CSV exportieren";
        respond_again: native_en "Start a New Response", native_de "Neue Antwort beginnen", reuse_en "Start a New Response", reuse_de "Neue Antwort beginnen";
        submit: native_en "Submit", native_de "Absenden", reuse_en "Submit", reuse_de "Absenden";
        example_id: native_en "Fixture Slug", native_de "Fixture-Slug", reuse_en "Fixture Slug", reuse_de "Fixture-Slug";
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
        window_blueprint: native_en "Design", native_de "Entwurf", reuse_en "Design", reuse_de "Entwurf";
        window_try: native_en "Fill Form", native_de "Formular ausfüllen", reuse_en "Fill Form", reuse_de "Formular ausfüllen";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
pub fn forms_play_labels(view_state: &semio_framework_plugin::ViewModel) -> &'static FormsLabels {
    semio_framework_plugin::resolve_labels::<FormsLabels>(view_state)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

impl FormsLabels {
    /// ✅️ Locale-specific validation messages for stable answer error codes.
    pub fn answer_error(&self, code: &str) -> &str {
        match code {
            "required" => self.error_required.as_str(),
            "type" => self.error_type.as_str(),
            "range" => self.error_range.as_str(),
            "step" => self.error_step.as_str(),
            "option" => self.error_option.as_str(),
            "date" => self.error_date.as_str(),
            "color" => self.error_color.as_str(),
            "vector" => self.error_vector.as_str(),
            _ => self.error_type.as_str(),
        }
    }
}
