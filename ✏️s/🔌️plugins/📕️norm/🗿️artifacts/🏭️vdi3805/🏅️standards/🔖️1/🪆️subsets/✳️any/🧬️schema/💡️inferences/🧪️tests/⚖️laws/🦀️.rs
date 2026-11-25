//! ⚖️ VDI 3805 inference laws: determinism and defaults of the inference, the published sheet scopes, the part 22 and 29 corrections and the typed sheet routing, run against the public API of the inferences crate.

#![cfg(test)]

extern crate semio_os_kernel as protocol;

pub use semio_s_artifact_norm_vdi3805_inferences::*;
use semio_s_artifact_norm_vdi3805_model::*;
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, CheckStatus, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

#[path = "../🔬️unit/🦀️.rs"]
mod unit;
