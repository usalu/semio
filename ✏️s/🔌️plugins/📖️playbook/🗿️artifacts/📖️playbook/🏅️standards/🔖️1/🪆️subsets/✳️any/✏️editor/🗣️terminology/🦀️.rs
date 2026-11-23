//! 🗣️ Playbook play app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale combination is compile-checked in one place.

//#region 🔖️Labels
semio_ui_locale::app_labels! {
    /// 🗣️ Complete UI label set for the playbook-play app; one field per label makes every locale
    /// combination compile-checked. No separate reuse-terminology concept, so reuse repeats native.
    pub struct PlaybookPlayLabels {
        window_builder: native_en "Builder", native_de "Builder", reuse_en "Builder", reuse_de "Builder";
        mode_builder: native_en "Builder", native_de "Builder", reuse_en "Builder", reuse_de "Builder";
        kind_arg: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        surface_steps: native_en "Steps", native_de "Schritte", reuse_en "Steps", reuse_de "Schritte";
        surface_source: native_en "Source", native_de "Quelle", reuse_en "Source", reuse_de "Quelle";
        surface_activity: native_en "Activity", native_de "Aktivität", reuse_en "Activity", reuse_de "Aktivität";
        surface_changes: native_en "Changes", native_de "Änderungen", reuse_en "Changes", reuse_de "Änderungen";
        surface_files: native_en "Files", native_de "Dateien", reuse_en "Files", reuse_de "Dateien";
        files_root: native_en "Playbook", native_de "Playbook", reuse_en "Playbook", reuse_de "Playbook";
        files_step: native_en "Step", native_de "Schritt", reuse_en "Step", reuse_de "Schritt";
        files_block: native_en "Block", native_de "Baustein", reuse_en "Block", reuse_de "Baustein";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
pub fn playbook_play_labels(view_state: &semio_plugin::ViewModel) -> &'static PlaybookPlayLabels {
    semio_plugin::resolve_labels::<PlaybookPlayLabels>(view_state)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests

//#endregion 🧪️Tests
