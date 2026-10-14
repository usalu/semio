//! 🔬️ Integration tests of the `s.fem.fem3d` load mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/➕️add-load/🧪️tests/⏸️dup-load-id-4f4a0a/🦀️.rs"]
mod add_load_edge_dup_load_id;
#[path = "../../🧬️schema/🧬️mutations/➕️add-load/🧪️tests/🏗️hall-adds-udl-e345cb/🦀️.rs"]
mod add_load_hall_hall_adds_udl;
#[path = "../../🧬️schema/🧬️mutations/➕️add-load/🧪️tests/🏠️lays/🦀️.rs"]
mod add_load_lays_an_area_pressure_over_the_roof_slab;
#[path = "../../🧬️schema/🧬️mutations/➕️add-load/🧪️tests/🚨️no-such-member-3fe6e9/🦀️.rs"]
mod add_load_edge_no_such_member;
#[path = "../../🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/➖️drops/🦀️.rs"]
mod remove_load_drops_the_trailing_member_udl_from_the_dead_case;
#[path = "../../🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/🏗️hall-cut-wind-6cf528/🦀️.rs"]
mod remove_load_hall_hall_cut_wind;
#[path = "../../🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod remove_load_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/🚨️no-such-load-5bab2d/🦀️.rs"]
mod remove_load_edge_no_such_load;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/⏸️same-load-65135e/🦀️.rs"]
mod replace_load_same_load;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/🏗️hall-retunes-wx-a635fe/🦀️.rs"]
mod replace_load_hall_hall_retunes_wx;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/🔁️retunes/🦀️.rs"]
mod replace_load_retunes_the_rafter_udl;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/🚨️dangling-node-092d4a/🦀️.rs"]
mod replace_load_dangling_node;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/🚨️no-such-case-21f1b7/🦀️.rs"]
mod replace_load_no_such_case;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/🚨️no-such-load-cc8aee/🦀️.rs"]
mod replace_load_no_such_load;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/🪪️renames-load-a535c9/🦀️.rs"]
mod replace_load_renames_load;
