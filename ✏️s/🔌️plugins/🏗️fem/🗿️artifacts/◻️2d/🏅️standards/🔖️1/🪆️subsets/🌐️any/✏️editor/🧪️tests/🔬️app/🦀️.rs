//! 🧪️ Integration tests of the `s.fem.fem2d` play app editor that drive the whole mounted app through the shared `context` harness.

extern crate semio_os_kernel as store;
extern crate semio_os_kernel as vcs;

#[path = "../🧰️context/🦀️.rs"]
pub mod context;
#[path = "../../🎮️commands/📍️add-nodal-load/🧪️tests/🔬️unit/🦀️.rs"]
mod add_nodal_load;
#[path = "../../🎮️commands/⚪️add-node/🧪️tests/🔬️unit/🦀️.rs"]
mod add_node;
#[path = "../../📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
mod artifact_panel;
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod editor_unit;
#[path = "../../🎮️commands/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs"]
mod gumball;
#[path = "../../📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
mod inspection_panel;
#[path = "../../🎭️modes/✏️edit/🪟️windows/🧱️model/🧪️tests/🔬️unit/🦀️.rs"]
mod model_window;
#[path = "../../🎮️commands/🩹️patch-combination/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_combination;
#[path = "../../🎮️commands/🩹️patch-element/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_element;
#[path = "../../🎮️commands/🩹️patch-load/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_load;
#[path = "../../🎮️commands/🩹️patch-load-case/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_load_case;
#[path = "../../🎮️commands/🩹️patch-material/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_material;
#[path = "../../🎮️commands/🩹️patch-node/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_node;
