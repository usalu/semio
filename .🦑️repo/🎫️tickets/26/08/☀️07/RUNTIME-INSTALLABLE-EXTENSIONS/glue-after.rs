//! 🌊️ OS flow family glue — wires core, brep/draw builtins, and wasm SDK helpers.

extern crate self as flow_core;
extern crate self as flow_extension_brep;
extern crate self as flow_extension_draw;
extern crate self as flow_extension_wasm;
extern crate self as flow_extension_sdk;

#[path = "../../🫀️core/🦀️component.rs"]
pub mod core;
pub use core::*;

#[path = "../../🫀️core/📐️brep-geometry/🦀️component.rs"]
pub mod brep_geometry;
pub use brep_geometry::{
    dispose_geometry, export_solid_json, import_solid_json, retain_geometry_handles, tessellate_geometry,
};


#[path = "."]
pub mod extensions {

  #[path = "../../🧩️extensions/🕸️wasm/🦀️component.rs"]
  pub mod wasm;

  #[path = "../../../../../../../✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs"]
  pub mod draw;
}

pub use extensions::draw::*;
pub use extensions::wasm::*;
