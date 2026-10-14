//! 🧪️ Kind fixture laws of `change-slider-value`, `drag-transforms`, `rotate-transforms`, `scale-transforms`, `move-nodes`: each committed scenario applied, inverted and diffed through the mutation union.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🎚️change-slider-value/🧪️tests/🎚️sets/🦀️.rs"]
mod change_slider_value_sets;
#[path = "../../✋️drag-transforms/🧪️tests/✋️drags/🦀️.rs"]
mod drag_transforms_drags;
#[path = "../../🔃️rotate-transforms/🧪️tests/🔃️turns/🦀️.rs"]
mod rotate_transforms_turns;
#[path = "../../📏️scale-transforms/🧪️tests/📏️scales/🦀️.rs"]
mod scale_transforms_scales;
#[path = "../../🚚️move-nodes/🧪️tests/🚚️shifts/🦀️.rs"]
mod move_nodes_shifts;
