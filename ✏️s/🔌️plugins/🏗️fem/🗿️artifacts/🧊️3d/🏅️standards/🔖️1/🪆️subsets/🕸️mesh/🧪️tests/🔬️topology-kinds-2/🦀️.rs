//! 🔬️ Integration tests of the `s.fem.fem3d` node and element mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🏗️hall-new-tie-074a69/🦀️.rs"]
mod create_element_hall_hall_new_tie;
#[path = "../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🚨️dangling-start-ab4132/🦀️.rs"]
mod create_element_edge_dangling_start;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/⛓️rafter-under-udl-e0342d/🦀️.rs"]
mod delete_element_edge_rafter_under_udl;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🏗️hall-cut-tie-c268d4/🦀️.rs"]
mod delete_element_hall_hall_cut_tie;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_element_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚨️no-such-element-eb788c/🦀️.rs"]
mod delete_element_edge_no_such_element;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_element_removes_the_bracing_bar_and_leaves_the_frame;
#[path = "../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/⏸️same-element-61adb2/🦀️.rs"]
mod replace_element_edge_same_element;
#[path = "../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🏗️hall-strut-d0e4b7/🦀️.rs"]
mod replace_element_hall_hall_strut;
#[path = "../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🔄️rolls/🦀️.rs"]
mod replace_element_rolls_the_column_about_its_own_axis;
#[path = "../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🚨️dangling-sec-70b168/🦀️.rs"]
mod replace_element_edge_dangling_sec;
#[path = "../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🪪️renames-brace-219be2/🦀️.rs"]
mod replace_element_edge_renames_brace;
