//! 🎮️ Lowpoly editor command handlers.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

//#region 🧩️Modules
#[path = "➕️add-primitive/🦀️.rs"]
pub mod add_primitive;
#[path = "🎥️camera/🦀️.rs"]
pub mod camera;
#[path = "👁️chrome/🦀️.rs"]
pub mod chrome;
#[path = "💬️engagement/🦀️.rs"]
pub mod engagement;
#[path = "📄️document/🦀️.rs"]
pub mod document;
#[path = "📤️media/🦀️.rs"]
pub mod media;
#[path = "🗑️object/🦀️.rs"]
pub mod object;
#[path = "🔷️mesh-edit/🦀️.rs"]
pub mod mesh_edit;
#[path = "🖌️paint/🦀️.rs"]
pub mod paint;
#[path = "✏️patch-object/🦀️.rs"]
pub mod patch_object;
#[path = "🗂️selection/🦀️.rs"]
pub mod selection;
#[path = "🌞️sun/🦀️.rs"]
pub mod sun;
#[path = "🧲️transform/🦀️.rs"]
pub mod transform;
#[path = "🧰️utility/🦀️.rs"]
pub mod utility;
#[path = "🧵️uv/🦀️.rs"]
pub mod uv;
//#endregion 🧩️Modules
