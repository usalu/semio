//! 🗣️ Sequence play app — the single `app_labels!` block plus the locale resolver every taxonomy
//! node reaches for. Deliberately ONE block for the whole app (never split per window/panel): the
//! macro's value is that every locale×terminology combination is compile-checked in one place.

use crate::editor::sequence::config::SequenceConfig;

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the sequence app; one field per label makes every locale×terminology combination compile-checked.
    pub struct SequenceLabels {
        steps: native_en "Steps", native_de "Schritte", reuse_en "Steps", reuse_de "Schritte";
        flow_edges: native_en "Flow edges", native_de "Ablaufkanten", reuse_en "Flow edges", reuse_de "Ablaufkanten";
        select_prompt: native_en "Select a step in the canvas or document.", native_de "Wähle einen Schritt in der Zeichenfläche oder im Dokument aus.", reuse_en "Select a step in the canvas or document.", reuse_de "Wähle einen Schritt in der Zeichenfläche oder im Dokument aus.";
        step_not_found: native_en "Step not found", native_de "Schritt nicht gefunden", reuse_en "Step not found", reuse_de "Schritt nicht gefunden";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        params: native_en "Params", native_de "Parameter", reuse_en "Params", reuse_de "Parameter";
        id: native_en "Id", native_de "ID", reuse_en "Id", reuse_de "ID";
        step: native_en "Step", native_de "Schritt", reuse_en "Step", reuse_de "Schritt";
        action_set_state: native_en "Set state", native_de "Zustand setzen", reuse_en "Set state", reuse_de "Zustand setzen";
        action_log_print: native_en "Print log", native_de "Log ausgeben", reuse_en "Print log", reuse_de "Log ausgeben";
        action_if: native_en "If", native_de "Wenn", reuse_en "If", reuse_de "Wenn";
        action_while: native_en "While", native_de "Solange", reuse_en "While", reuse_de "Solange";
        action_add: native_en "Add", native_de "Addieren", reuse_en "Add", reuse_de "Addieren";
        add_to: native_en "Add to", native_de "Hinzufügen zu", reuse_en "Add to", reuse_de "Hinzufügen zu";
        run: native_en "Run", native_de "Ausführen", reuse_en "Run", reuse_de "Ausführen";
        stop: native_en "Stop", native_de "Stopp", reuse_en "Stop", reuse_de "Stopp";
        reorganize: native_en "Reorganize", native_de "Neu anordnen", reuse_en "Reorganize", reuse_de "Neu anordnen";
        layout: native_en "Layout", native_de "Layout", reuse_en "Layout", reuse_de "Layout";
        left_to_right: native_en "Left to right", native_de "Links nach rechts", reuse_en "Left to right", reuse_de "Links nach rechts";
        top_to_bottom: native_en "Top to bottom", native_de "Oben nach unten", reuse_en "Top to bottom", reuse_de "Oben nach unten";
        window_main: native_en "Sequence", native_de "Sequenz", reuse_en "Sequence", reuse_de "Sequenz";
        window_script: native_en "Script", native_de "Skript", reuse_en "Script", reuse_de "Skript";
        window_compiled: native_en "DSL", native_de "DSL", reuse_en "DSL", reuse_de "DSL";
        none: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        slot: native_en "slot", native_de "Slot", reuse_en "slot", reuse_de "Slot";
        slot_then: native_en "Then", native_de "Dann", reuse_en "Then", reuse_de "Dann";
        slot_else: native_en "Else", native_de "Sonst", reuse_en "Else", reuse_de "Sonst";
        slot_body: native_en "Body", native_de "Rumpf", reuse_en "Body", reuse_de "Rumpf";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub fn sequence_play_labels(cfg: &SequenceConfig) -> &'static SequenceLabels {
    semio_framework_plugin::resolve_labels_for_locale::<SequenceLabels>(&cfg.locale)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(sequence_play_labels(&SequenceConfig::default()).steps.as_str(), "Steps");
        assert_eq!(sequence_play_labels(&SequenceConfig { locale: "de-DE".into(), ..SequenceConfig::default() }).steps.as_str(), "Schritte");
    }
}
//#endregion 🧪️Tests
