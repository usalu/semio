//! 🔬️ Integration tests of the `s.fem.fem2d` load case mutation kinds against the committed fixtures, the dispatch enum and the central apply.

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
