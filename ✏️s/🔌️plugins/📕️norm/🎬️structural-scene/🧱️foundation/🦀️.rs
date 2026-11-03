//! 🧱️ Structural scene foundation: the scene schema and the diagram kit that every lowering builds on; re-exported by `semio-s-plugin-norm-structural-scene`.

extern crate semio_framework_value as value;
extern crate semio_framework_value_derive as value_derive;
extern crate semio_s_plugin_norm_compliance as norm;

#[path = "../🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

#[path = "../📈️diagram-kit/🦀️.rs"]
pub mod diagram_kit;

/// 🎨️ The one tone of a utilisation ratio, decided by the shared [`norm::within_limit`] so the colour can never disagree with the check status: error beyond the limit, warning above 0.8, info otherwise.
pub fn utilisation_tone(ratio: f64) -> Tone {
    if !norm::within_limit(ratio) { Tone::Error } else if ratio > 0.8 { Tone::Warning } else { Tone::Info }
}

/// 🎨️ The one scene status of a utilisation ratio, decided by the shared [`norm::within_limit`]: fail beyond the limit, warning above `near`, pass otherwise.
pub fn utilisation_status(ratio: f64, near: f64) -> CheckStatus {
    if !norm::within_limit(ratio) { CheckStatus::Fail } else if ratio > near { CheckStatus::Warning } else { CheckStatus::Pass }
}
