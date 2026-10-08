//! 📤️ Foreign formats written from the BIM model: one leaf per target dialect, each a `Serializer<ModelSnapshot>`.

#[path = "🏗️ifc/🦀️.rs"]
pub mod ifc;

#[path = "🧊️gltf/🦀️.rs"]
pub mod gltf;

#[path = "🎨️svg/🦀️.rs"]
pub mod svg;
