//! 🔬️ Integration tests of the `s.fem.fem2d` selection move against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/⚖️laws/🦀️.rs"]
mod laws;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/⏸️moves-nothing-f724d6/🦀️.rs"]
mod move_selection_an_identity_transform_moves_nothing;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/⛔️rejects-a-missing-f9c98d/🦀️.rs"]
mod move_selection_rejects_transforming_a_node_and_a_region_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/📏️stretches/🦀️.rs"]
mod move_selection_stretches_the_spare_panel_and_the_canopy_tip_to_twice_their_width;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🔁️denies/🦀️.rs"]
mod move_selection_refuses_a_transform_that_names_a_node_twice;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🧭️shifts-the-slab-3f8df5/🦀️.rs"]
mod move_selection_shifts_the_first_floor_slab_and_its_corner_nodes_right_and_up;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🫓️denies/🦀️.rs"]
mod move_selection_refuses_a_scale_that_flattens_the_wall_to_zero_height;
