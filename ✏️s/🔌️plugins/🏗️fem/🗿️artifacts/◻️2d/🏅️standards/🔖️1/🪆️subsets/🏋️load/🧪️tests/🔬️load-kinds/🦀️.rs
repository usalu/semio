//! 🔬️ Integration tests of the `s.fem.fem2d` load mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/➕️add-load/🧪️tests/👻️dangling-node-8d113b/🦀️.rs"]
mod add_load_refuses_to_push_a_wind_load_at_a_node_the_frame_does_not_have;
#[path = "../../🧬️schema/🧬️mutations/➕️add-load/🧪️tests/💨️pushes-a-wind-load-5c3f1e/🦀️.rs"]
mod add_load_pushes_a_wind_point_load_onto_the_first_floor_of_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/➕️add-load/🧪️tests/📏️appends/🦀️.rs"]
mod add_load_appends_a_member_udl_to_the_dead_case;
#[path = "../../🧬️schema/🧬️mutations/➕️add-load/🧪️tests/🚫️rejects-a-missing-4271bc/🦀️.rs"]
mod add_load_rejects_adding_a_load_to_a_load_case_that_does_not_exist;
#[path = "../../🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/⛔️rejects-a-missing-1a8a80/🦀️.rs"]
mod remove_load_rejects_removing_a_load_the_dead_case_never_carried;
#[path = "../../🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/✂️strips-the-roof-udl-0c1b3c/🦀️.rs"]
mod remove_load_strips_the_trailing_roof_udl_from_the_dead_case;
#[path = "../../🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/➖️strips/🦀️.rs"]
mod remove_load_strips_the_trailing_member_udl_from_the_dead_case;
#[path = "../../🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod remove_load_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/⛔️rejects-a-missing-7a1e1f/🦀️.rs"]
mod replace_load_rejects_replacing_a_load_the_wind_case_never_carried;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/🏋️retunes-the-live-f6fd49/🦀️.rs"]
mod replace_load_retunes_the_live_point_load_on_the_timber_portal_frame;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/👻️dangling-node-b95678/🦀️.rs"]
mod replace_load_refuses_to_move_a_wind_load_onto_a_node_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/💨️strengthens/🦀️.rs"]
mod replace_load_strengthens_the_wind_point_load_on_the_braced_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/🪪️denies-rename-732de0/🦀️.rs"]
mod replace_load_refuses_to_relabel_a_wind_load_through_a_replace_load;
