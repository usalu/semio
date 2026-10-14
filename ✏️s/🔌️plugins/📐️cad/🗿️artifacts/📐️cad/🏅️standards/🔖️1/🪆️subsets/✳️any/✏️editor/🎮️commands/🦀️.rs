//! 🎮️ CAD editor command handlers: camera, contributions, engagement, io, model definition, nodes, objects, references, sun, transform and utility.

extern crate semio_framework_os_kernel as protocol;

//#region 🧩️Modules
#[path = "🎥️camera/🦀️.rs"]
pub mod camera;
#[path = "🧩️contribution/🦀️.rs"]
pub mod contribution;
#[path = "🤝️engagement/🦀️.rs"]
pub mod engagement;
#[path = "📥️io/🦀️.rs"]
pub mod io;
#[path = "🗺️model-definition/🦀️.rs"]
pub mod model_definition;
#[path = "🕸️node/🦀️.rs"]
pub mod node;
#[path = "🧱️object/🦀️.rs"]
pub mod object;
#[path = "🖼️reference/🦀️.rs"]
pub mod reference;
#[path = "🌞️sun/🦀️.rs"]
pub mod sun;
#[path = "🔄️transform/🦀️.rs"]
pub mod transform;
#[path = "🧰️utility/🦀️.rs"]
pub mod utility;
//#endregion 🧩️Modules
