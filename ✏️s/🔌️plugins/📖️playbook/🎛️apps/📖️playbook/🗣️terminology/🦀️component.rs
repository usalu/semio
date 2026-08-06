//! 🗣️ Playbook play app — the single `app_labels!` block plus the locale resolver every taxonomy node
//! reaches for. Deliberately ONE block for the whole app (never split per window/panel): the macro's
//! value is that every locale combination is compile-checked in one place.

use crate::apps::playbook::config::PlaybookConfig;

//#region 🔖️Labels
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the playbook-play app; one field per label makes every locale
    /// combination compile-checked. No separate reuse-terminology concept, so reuse repeats native.
    pub struct PlaybookPlayLabels {
        window_builder: native_en "Builder", native_de "Builder", reuse_en "Builder", reuse_de "Builder";
        mode_builder: native_en "Builder", native_de "Builder", reuse_en "Builder", reuse_de "Builder";
        kind_arg: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
    }
}
//#endregion 🔖️Labels

//#region 🔖️Resolvers
/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
pub fn playbook_play_labels(cfg: &PlaybookConfig) -> &'static PlaybookPlayLabels {
    semio_framework_plugin::resolve_labels_for_locale::<PlaybookPlayLabels>(&cfg.locale)
}
//#endregion 🔖️Resolvers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_resolve_native_english_and_german_from_the_config_locale() {
        assert_eq!(playbook_play_labels(&PlaybookConfig::default()).kind_arg.as_str(), "Kind");
        assert_eq!(playbook_play_labels(&PlaybookConfig { locale: "de-DE".into(), ..PlaybookConfig::default() }).kind_arg.as_str(), "Art");
    }
}
//#endregion 🧪️Tests
