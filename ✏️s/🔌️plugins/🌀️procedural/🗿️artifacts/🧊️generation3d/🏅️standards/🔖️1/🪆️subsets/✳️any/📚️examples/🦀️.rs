//! 📚️ Generation3d bundled examples: one module per authored example document.
extern crate semio_framework_os_kernel as protocol;

#[path = "🥽️mesh-workbench/🦀️.rs"]
pub mod art_generation3d_mesh_workbench;
#[path = "📐️box-fillet-preview/🦀️.rs"]
pub mod art_generation3d_box_fillet_preview;
#[path = "🐚️box-shell-preview/🦀️.rs"]
pub mod art_generation3d_box_shell_preview;
#[path = "🧹️face-sweep-extrude/🦀️.rs"]
pub mod art_generation3d_face_sweep_extrude;
#[path = "🍄️hexagonal-mushroom-column/🦀️.rs"]
pub mod art_generation3d_hexagonal_mushroom_column;
#[path = "📦️rectangle-extrude-volume/🦀️.rs"]
pub mod art_generation3d_rectangle_extrude_volume;
#[path = "🪢️rectangle-wire-preview/🦀️.rs"]
pub mod art_generation3d_rectangle_wire_preview;
#[path = "🧲️sphere-box-fuse/🦀️.rs"]
pub mod art_generation3d_sphere_box_fuse;
#[path = "🍩️sphere-cut-with-torus/🦀️.rs"]
pub mod art_generation3d_sphere_cut_with_torus;
#[cfg(test)]
#[path = "📐️box-fillet-preview/🧪️tests/🧩️example/🦀️.rs"]
mod box_fillet_preview_tests;
#[cfg(test)]
#[path = "🐚️box-shell-preview/🧪️tests/🧩️example/🦀️.rs"]
mod box_shell_preview_tests;
#[cfg(test)]
#[path = "🧹️face-sweep-extrude/🧪️tests/🧩️example/🦀️.rs"]
mod face_sweep_extrude_tests;
#[cfg(test)]
#[path = "🍄️hexagonal-mushroom-column/🧪️tests/🧩️example/🦀️.rs"]
mod hexagonal_mushroom_column_tests;
#[cfg(test)]
#[path = "📦️rectangle-extrude-volume/🧪️tests/🧩️example/🦀️.rs"]
mod rectangle_extrude_volume_tests;
#[cfg(test)]
#[path = "🪢️rectangle-wire-preview/🧪️tests/🧩️example/🦀️.rs"]
mod rectangle_wire_preview_tests;
#[cfg(test)]
#[path = "🧲️sphere-box-fuse/🧪️tests/🧩️example/🦀️.rs"]
mod sphere_box_fuse_tests;
#[cfg(test)]
#[path = "🍩️sphere-cut-with-torus/🧪️tests/🧩️example/🦀️.rs"]
mod sphere_cut_with_torus_tests;
