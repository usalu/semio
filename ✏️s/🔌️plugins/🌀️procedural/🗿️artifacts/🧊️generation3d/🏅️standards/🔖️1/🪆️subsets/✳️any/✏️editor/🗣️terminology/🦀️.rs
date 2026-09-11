//! 🗣️ Generation3d play app — the single `app_labels!` block plus the locale resolvers every taxonomy
//! node reaches for.

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
        graph_nodes: native_en "Nodes", native_de "Knoten", reuse_en "Nodes", reuse_de "Knoten";
        graph_wires: native_en "Wires", native_de "Leitungen", reuse_en "Wires", reuse_de "Leitungen";
        graph_input_port: native_en "Input", native_de "Eingang", reuse_en "Input", reuse_de "Eingang";
        graph_output_port: native_en "Output", native_de "Ausgang", reuse_en "Output", reuse_de "Ausgang";
        graph_empty: native_en "(no nodes)", native_de "(keine Knoten)", reuse_en "(no nodes)", reuse_de "(keine Knoten)";
        graph_unwired: native_en "(no wires)", native_de "(keine Leitungen)", reuse_en "(no wires)", reuse_de "(keine Leitungen)";
        status_ok: native_en "Evaluated", native_de "Ausgewertet", reuse_en "Evaluated", reuse_de "Ausgewertet";
        status_stale: native_en "Stale", native_de "Veraltet", reuse_en "Stale", reuse_de "Veraltet";
        status_queued: native_en "Queued", native_de "In Warteschlange", reuse_en "Queued", reuse_de "In Warteschlange";
        status_computing: native_en "Computing", native_de "Berechnet", reuse_en "Computing", reuse_de "Berechnet";
        status_error: native_en "Error", native_de "Fehler", reuse_en "Error", reuse_de "Fehler";
        status_blocked: native_en "Blocked", native_de "Blockiert", reuse_en "Blocked", reuse_de "Blockiert";
    }
}

/// 🗣️ Resolves the active label set from the shared view model.
pub fn generation3d_labels(view_state: &semio_framework_plugin::ViewModel) -> &'static Generation3dLabels {
    semio_framework_plugin::resolve_labels::<Generation3dLabels>(view_state)
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
