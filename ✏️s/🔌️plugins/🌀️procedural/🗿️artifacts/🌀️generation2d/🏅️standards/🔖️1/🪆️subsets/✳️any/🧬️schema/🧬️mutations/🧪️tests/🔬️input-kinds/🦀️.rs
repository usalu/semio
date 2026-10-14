//! 🧪️ Kind fixture laws of `change-slider-value`, `change`, `update-camera`, `move-nodes`: each committed scenario applied, inverted and diffed through the mutation union.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🎛️update-camera/🧪️tests/📷️pans/🦀️.rs"]
mod update_camera_pans_and_zooms_the_graph_camera;
#[path = "../../🎚️change-slider-value/🧪️tests/🎚️sets/🦀️.rs"]
mod change_slider_value_sets;
#[path = "../../🚚️move-nodes/🧪️tests/🚚️shifts/🦀️.rs"]
mod move_nodes_shifts;
#[path = "../../🔢️change/🧪️tests/📏️raises/🦀️.rs"]
mod change_raises_the_height_answer_in_generation_1;
