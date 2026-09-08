//! 🔌️ Norm plugin composition over independently compiled standard artifacts.

#![allow(async_fn_in_trait)]

#[path = "../../🦀️.rs"]
mod plugin;
pub use plugin::{plugin, NormApps};
semio_framework_plugin::plugin_exports!(plugin, NormApps);
