//! 📤️ Foreign formats written from the BIM model: one leaf per target dialect, each a `Serializer<ModelSnapshot>`.

#[path = "🏗️ifc/🦀️.rs"]
pub mod ifc;

#[path = "🧊️gltf/🦀️.rs"]
pub mod gltf;

#[path = "🎨️svg/🦀️.rs"]
pub mod svg;

#[path = "📊️csv/🦀️.rs"]
pub mod csv;

#[path = "🧾️json/🦀️.rs"]
pub mod json;

#[path = "📄️sheets/🦀️.rs"]
pub mod sheets;

#[path = "🧱️holders/🦀️.rs"]
pub mod holders;

#[path = "🌿️gbxml/🦀️.rs"]
pub mod gbxml;

#[path = "🔋️energy/🦀️.rs"]
pub mod energy;

#[path = "🦴️solver/🦀️.rs"]
pub mod solver;
