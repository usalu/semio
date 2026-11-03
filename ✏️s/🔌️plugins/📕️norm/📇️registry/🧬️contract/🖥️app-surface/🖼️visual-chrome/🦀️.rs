//! 🖼️ App-surface visual chrome: the 2D diagrams window, accessibility, themes, device profiles, the window manifest and the interaction domain generator, layered over the foundation and the 3D viewport crates.

#![allow(async_fn_in_trait, unused_imports)]

include!("../📋️prelude/🦀️.rs");

pub use semio_s_plugin_norm_app_surface_foundation::*;
pub use semio_s_plugin_norm_app_surface_viewport3d as viewport3d;

#[path = "../🪟️visual-windows/📈️diagrams2d/🦀️.rs"]
pub mod diagrams2d;

#[path = "../🪟️visual-windows/♿️accessibility/🦀️.rs"]
pub mod accessibility;

#[path = "../🪟️visual-windows/📱️device-profiles/🦀️.rs"]
pub mod device_profiles;

#[path = "../🪟️visual-windows/🎨️themes/🦀️.rs"]
pub mod themes;

#[path = "../🕹️direct-manipulation/🧭️domain/🦀️.rs"]
mod interaction_domain;
pub use interaction_domain::*;

#[path = "../🪟️manifest-and-media/🦀️.rs"]
mod manifest_and_media;
pub use manifest_and_media::*;
