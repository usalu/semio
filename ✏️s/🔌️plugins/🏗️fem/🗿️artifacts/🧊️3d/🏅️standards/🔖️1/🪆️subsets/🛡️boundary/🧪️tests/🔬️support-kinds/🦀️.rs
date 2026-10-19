//! 🔬️ Integration tests of the `s.fem.fem3d` support mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;

#[path = "../../🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🏗️hall/🦀️.rs"]
mod create_support_hall_hall_new_pin;
#[path = "../../🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🔒️clamps/🦀️.rs"]
mod create_support_clamps_the_column_base_in_all_six_dofs;
#[path = "../../🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🚨️dangling/🦀️.rs"]
mod create_support_edge_dangling_node;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🏗️hall/🦀️.rs"]
mod delete_support_hall_hall_cut_pin;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_support_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🔓️releases/🦀️.rs"]
mod delete_support_releases_the_pinned_node_n2;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🚨️no/🦀️.rs"]
mod delete_support_edge_no_such_support;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/⏸️same/🦀️.rs"]
mod replace_support_edge_same_support;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🏗️hall/🦀️.rs"]
mod replace_support_hall_hall_fixes_base;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🔄️frees/🦀️.rs"]
mod replace_support_frees_the_three_rotations_at_the_column_base;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🚨️dangling/🦀️.rs"]
mod replace_support_edge_dangling_node;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🪪️renames/🦀️.rs"]
mod replace_support_edge_renames_pin;
#[path = "../../🧬️schema/🧬️mutations/🩹️change-support/🧪️tests/🔬️unit/🦀️.rs"]
mod change_support_field_intent;
