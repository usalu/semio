//! 🔬️ Integration tests of the `s.fem.fem2d` load case and load mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/❄️appends/🦀️.rs"]
mod create_load_case_appends_the_snow_case_over_the_roof_beam;
#[path = "../../🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/📍️appends/🦀️.rs"]
mod create_load_case_appends_a_live_case_carrying_one_nodal_load;
#[path = "../../🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/🚫️rejects/🦀️.rs"]
mod create_load_case_rejects_a_load_case_whose_udl_names_a_missing_element;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/⛔️rejects/🦀️.rs"]
mod delete_load_case_rejects_deleting_a_load_case_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_load_case_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🔗️blocks-in-use-7cdc5f/🦀️.rs"]
mod delete_load_case_refuses_to_delete_the_dead_case_three_combinations_still_weight;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🗑️drops/🦀️.rs"]
mod delete_load_case_drops_the_spare_snow_case_with_its_single_load;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_load_case_removes_the_live_case_together_with_its_loads;
#[path = "../../🧬️schema/🧬️mutations/🏷️change-load-case-name/🧪️tests/⛔️rejects/🦀️.rs"]
mod change_load_case_name_rejects_renaming_a_load_case_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🏷️change-load-case-name/🧪️tests/✏️renames/🦀️.rs"]
mod change_load_case_name_renames_the_wind_case_of_the_braced_steel_frame_to_its_code_clause;
#[path = "../../🧬️schema/🧬️mutations/🏷️change-load-case-name/🧪️tests/🏷️renames/🦀️.rs"]
mod change_load_case_name_renames_the_live_case_of_the_timber_portal_frame_to_imposed_load;
#[path = "../../🧬️schema/🧬️mutations/🏷️change-load-case-name/🧪️tests/🔁️keeps/🦀️.rs"]
mod change_load_case_name_keeps_the_dead_case_of_the_steel_frame_named_exactly_as_it_is;
#[path = "../../🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/⚖️switches/🦀️.rs"]
mod change_load_case_self_weight_switches_self_weight_on_for_the_dead_case;
#[path = "../../🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/🏋️switches/🦀️.rs"]
mod change_load_case_self_weight_switches_self_weight_on_for_the_imposed_case;
#[path = "../../🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/🔁️keeps/🦀️.rs"]
mod change_load_case_self_weight_keeps_self_weight_on_for_the_dead_case;
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
