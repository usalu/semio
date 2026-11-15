//! 🗂️ Norm design cases and spec trace: the bridge from a catalogue entry to an evaluated document entity and the typed spec-to-realisation table.

#![allow(async_fn_in_trait)]

#[cfg(test)]
extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use protocol::Mutation;
use std::fmt;
use store::TextError;

pub use semio_s_plugin_norm_compliance_core::*;

#[path = "../🦀️.rs"]
pub mod design_case;

#[path = "../../🧭️spec-trace/🦀️.rs"]
pub mod spec_trace;
