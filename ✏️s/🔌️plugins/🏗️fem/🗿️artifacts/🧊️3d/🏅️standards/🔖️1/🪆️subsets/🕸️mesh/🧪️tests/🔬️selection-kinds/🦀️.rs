//! 🔬️ Integration tests of the `s.fem.fem3d` selection move against the committed fixtures and its laws.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/⚖️laws/🦀️.rs"]
mod laws;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/⏸️moves-nothing-f724d6/🦀️.rs"]
mod move_selection_an_identity_transform_moves_nothing;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🌀️turns/🦀️.rs"]
mod move_selection_refuses_a_rotation_about_the_zero_axis;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🏗️hall/🦀️.rs"]
mod move_selection_hall_stretches_the_apron_slab_to_twice_its_width;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🔁️names/🦀️.rs"]
mod move_selection_refuses_a_transform_that_names_a_node_twice;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🚨️no-such-targets-f9c98d/🦀️.rs"]
mod move_selection_no_such_targets;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🧭️lifts/🦀️.rs"]
mod move_selection_lifts_the_column_head_half_a_metre;
#[path = "../../🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🫓️flattens/🦀️.rs"]
mod move_selection_refuses_a_scale_that_flattens_the_raft;
