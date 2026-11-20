//! 📌️ Layout editor panels: the catalogue, document, inspection and preflight panels.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as schema;

fn link_presentation_key(panel: &str, id: &str, position: usize) -> semio_plugin::UiAssemblyResult<semio_plugin::UiText> {
    semio_plugin::UiText::try_format(format_args!("{panel}.link.{id}"))
        .or_else(|| semio_plugin::UiText::try_format(format_args!("{panel}.long-link.{position}")))
        .ok_or_else(|| semio_plugin::PluginAssemblyError::new("ui.fixed-capacity", "layout link row key admission failed"))
}

//#region 🪢️TaxonomyMounts
#[path = "🛍️catalogue/🦀️.rs"]
pub mod catalogue;
#[path = "🗿️artifact/🦀️.rs"]
pub mod document;
#[path = "🔍️inspection/🦀️.rs"]
pub mod inspection;
#[path = "🚦️preflight/🦀️.rs"]
pub mod preflight;
//#endregion 🪢️TaxonomyMounts

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod config {
    #[path = "../🎚️config/🧬️schema/🦀️.rs"]
    pub mod schema;
}
#[path = "../📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "../📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
#[path = "."]
pub mod presence {
    #[path = "../👥️presence/🧬️schema/🦀️.rs"]
    pub mod schema;
}
#[path = "../🗣️terminology/🦀️.rs"]
pub mod terminology;
//#endregion 🪢️TaxonomyMounts

//#region 🪢️TaxonomyMounts
#[path = "../🎮️commands/🎯️focus-preflight-issue/🦀️.rs"]
pub mod focus_preflight_issue;
//#endregion 🪢️TaxonomyMounts
