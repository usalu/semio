//! 🔬️ Integration tests of the `s.fem.fem3d` load case mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/🌬️appends/🦀️.rs"]
mod create_load_case_appends_a_wind_case_pushing_on_the_column_head;
#[path = "../../🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/🏗️hall/🦀️.rs"]
mod create_load_case_hall_hall_snow_drift;
#[path = "../../🧬️schema/🧬️mutations/📋️create-load-case/🧪️tests/🚨️dangling/🦀️.rs"]
mod create_load_case_edge_dangling_solid;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/⛓️dead-in-combos-e73167/🦀️.rs"]
mod delete_load_case_edge_dead_in_combos;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🏗️hall/🦀️.rs"]
mod delete_load_case_hall_hall_cut_crane;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_load_case_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🚨️no-such-case-ef1fde/🦀️.rs"]
mod delete_load_case_edge_no_such_case;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-load-case/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_load_case_removes_the_wind_case_together_with_its_load;
#[path = "../../🧬️schema/🧬️mutations/🏷️change-load-case-name/🧪️tests/⏸️same-name-56ab29/🦀️.rs"]
mod change_load_case_name_same_name;
#[path = "../../🧬️schema/🧬️mutations/🏷️change-load-case-name/🧪️tests/🏗️hall/🦀️.rs"]
mod change_load_case_name_hall_hall_renames_crane;
#[path = "../../🧬️schema/🧬️mutations/🏷️change-load-case-name/🧪️tests/🏷️renames/🦀️.rs"]
mod change_load_case_name_renames_the_wind_case;
#[path = "../../🧬️schema/🧬️mutations/🏷️change-load-case-name/🧪️tests/🚨️no/🦀️.rs"]
mod change_load_case_name_no_such_case;
#[path = "../../🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/⏸️switches/🦀️.rs"]
mod change_load_case_self_weight_switches_self_weight_off_for_the_dead_case;
#[path = "../../🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/🏗️hall/🦀️.rs"]
mod change_load_case_self_weight_hall_hall_crane_sw;
#[path = "../../🧬️schema/🧬️mutations/⚖️change-load-case-self-weight/🧪️tests/🚨️sw/🦀️.rs"]
mod change_load_case_self_weight_edge_sw_no_such_case;
