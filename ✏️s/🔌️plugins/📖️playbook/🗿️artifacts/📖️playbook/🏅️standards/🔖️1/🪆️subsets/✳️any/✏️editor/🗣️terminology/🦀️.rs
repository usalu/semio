//! 🗣️ Playbook play app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale combination is compile-checked in one place.

//#region 🔖️Labels
semio_framework_ui_locale::app_labels! {
    /// 🗣️ Complete UI label set for the playbook-play app; one field per label makes every locale
    /// combination compile-checked. No separate reuse-terminology concept, so reuse repeats native.
    pub struct PlaybookPlayLabels {
        window_builder: native_en "Builder", native_de "Builder", reuse_en "Builder", reuse_de "Builder";
        mode_builder: native_en "Builder", native_de "Builder", reuse_en "Builder", reuse_de "Builder";
        kind_arg: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        files_root: native_en "Playbook", native_de "Playbook", reuse_en "Playbook", reuse_de "Playbook";
        files_step: native_en "Step", native_de "Schritt", reuse_en "Step", reuse_de "Schritt";
        files_block: native_en "Block", native_de "Baustein", reuse_en "Block", reuse_de "Baustein";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
pub fn playbook_play_labels(view_state: &semio_framework_plugin::ViewModel) -> &'static PlaybookPlayLabels {
    semio_framework_plugin::resolve_labels::<PlaybookPlayLabels>(view_state)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
