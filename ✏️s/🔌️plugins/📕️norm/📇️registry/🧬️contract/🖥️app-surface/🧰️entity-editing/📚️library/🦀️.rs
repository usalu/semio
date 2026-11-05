//! 🧰️ Pure entity editing operations and their event batches.

#![allow(async_fn_in_trait, unused_imports)]

include!("../../📋️prelude/🦀️.rs");

use semio_s_plugin_norm_app_surface_foundation::*;
use semio_s_plugin_norm_app_surface_live_core::*;
use semio_s_plugin_norm_app_surface_visual_chrome::*;
#[cfg(test)]
use semio_s_plugin_norm_app_surface_panels::*;
use semio_s_plugin_norm_app_surface_viewport3d as viewport3d;

#[path = "../🦀️.rs"]
mod operations;
pub use operations::*;
