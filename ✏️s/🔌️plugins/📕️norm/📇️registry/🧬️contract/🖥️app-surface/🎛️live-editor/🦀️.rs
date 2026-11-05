//! 🎛️ App-surface live editor: the retained norm editor and its command jobs, the live editor wiring of gestures and lanes, and the retained-disposition and live-acceptance oracles, layered over every other app-surface crate.

#![allow(async_fn_in_trait, unused_imports)]

include!("../📋️prelude/🦀️.rs");

pub use semio_s_plugin_norm_app_surface_foundation::*;
pub use semio_s_plugin_norm_app_surface_live_core::*;
pub use semio_s_plugin_norm_app_surface_panels::*;
pub use semio_s_plugin_norm_app_surface_viewport3d as viewport3d;
pub use semio_s_plugin_norm_app_surface_visual_chrome::*;

/// 🧵️ Retained norm command output with empty application config and draft lanes.
pub type NormRetainedCommandResult<M> = Result<Emit<M, NoConfigMutation, semio_plugin::NoDraftMutation>, Fault>;

pub use semio_s_plugin_norm_app_surface_entity_editing as entity_editing;

#[path = "../🫧️live-lanes/🎛️editor-wiring/🦀️.rs"]
mod editor_wiring;
pub use editor_wiring::*;

#[path = "../🏭️command-and-tool-job-macros/🦀️.rs"]
mod command_and_tool_job_macros;

#[path = "../🧵️retained-commands/🦀️.rs"]
mod retained_commands;
pub use retained_commands::*;

#[path = "../👁️viewer-surface/🦀️.rs"]
mod viewer_surface;
pub use viewer_surface::*;

#[cfg(any(test, feature = "compliance-testing"))]
//#region 🧵️RetainedDispositionOracle
#[path = "../🧪️tests/🔬️retained-disposition-oracle/🦀️.rs"]
pub mod retained_disposition_oracle;
//#endregion 🧵️RetainedDispositionOracle

#[cfg(any(test, feature = "compliance-testing"))]
#[path = "../🧪️tests/🔬️live-acceptance/🦀️.rs"]
mod live_acceptance;

#[cfg(any(test, feature = "compliance-testing"))]
#[path = "../🧪️tests/⏱️latency-bench/🦀️.rs"]
pub mod latency_bench;

pub use semio_s_plugin_norm_inspector_forms::*;

#[path = "../🪺️inspector-forms/plan.rs"]
mod inspector_form_plan;
pub use inspector_form_plan::*;

#[cfg(any(test, feature = "compliance-testing"))]
#[path = "../🪺️inspector-forms/🧪️tests/🧮️compiled/🦀️.rs"]
mod compiled_input_oracle;
#[cfg(any(test, feature = "compliance-testing"))]
pub use compiled_input_oracle::*;
