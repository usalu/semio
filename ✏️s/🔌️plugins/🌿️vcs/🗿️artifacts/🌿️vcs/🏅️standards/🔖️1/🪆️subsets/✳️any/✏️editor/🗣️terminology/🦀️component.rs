//! 🗣️ VCS play app — the single `app_labels!` block plus the locale resolver every taxonomy node reaches
//! for. Deliberately ONE block for the whole app (never split per window/panel): the macro's value is
//! that every locale×terminology combination is compile-checked in one place.

use crate::editor::vcs::config::VcsDemoConfig;

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the VCS app; one field per label makes every locale combination compile-checked.
    pub struct VcsPlayLabels {
        document: native_en "Document", native_de "Dokument", reuse_en "Document", reuse_de "Dokument";
        actions: native_en "Actions", native_de "Aktionen", reuse_en "Actions", reuse_de "Aktionen";
        counter: native_en "Counter", native_de "Zähler", reuse_en "Counter", reuse_de "Zähler";
        commit: native_en "Commit", native_de "Commit", reuse_en "Commit", reuse_de "Commit";
        branch: native_en "Branch", native_de "Branch", reuse_en "Branch", reuse_de "Branch";
        undo: native_en "Undo", native_de "Rückgängig", reuse_en "Undo", reuse_de "Rückgängig";
        redo: native_en "Redo", native_de "Wiederholen", reuse_en "Redo", reuse_de "Wiederholen";
        title: native_en "Title", native_de "Titel", reuse_en "Title", reuse_de "Titel";
        status: native_en "Status", native_de "Status", reuse_en "Status", reuse_de "Status";
        notes: native_en "Notes", native_de "Notizen", reuse_en "Notes", reuse_de "Notizen";
        tags: native_en "Tags", native_de "Schlagwörter", reuse_en "Tags", reuse_de "Schlagwörter";
        alternatives: native_en "Alternatives", native_de "Alternativen", reuse_en "Alternatives", reuse_de "Alternativen";
        no_checkpoints: native_en "(no checkpoints)", native_de "(keine Checkpoints)", reuse_en "(no checkpoints)", reuse_de "(keine Checkpoints)";
        checkpoints: native_en "checkpoints", native_de "Checkpoints", reuse_en "checkpoints", reuse_de "Checkpoints";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub async fn vcs_play_labels(cfg: &VcsDemoConfig) -> &'static VcsPlayLabels {
    semio_framework_plugin::resolve_labels_for_locale::<VcsPlayLabels>(&cfg.locale)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(vcs_play_labels(&VcsDemoConfig::default()).commit.as_str(), "Commit");
        assert_eq!(vcs_play_labels(&VcsDemoConfig { locale: "de-DE".into(), ..VcsDemoConfig::default() }).undo.as_str(), "Rückgängig");
    }
}
//#endregion 🧪️Tests
