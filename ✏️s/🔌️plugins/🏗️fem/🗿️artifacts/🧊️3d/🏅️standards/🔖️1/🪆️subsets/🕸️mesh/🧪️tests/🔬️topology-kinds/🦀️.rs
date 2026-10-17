//! 🔬️ Integration tests of the `s.fem.fem3d` node and element mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🏗️hall-new-node-b26700/🦀️.rs"]
mod create_node_hall_hall_new_node;
#[path = "../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/📍️appends/🦀️.rs"]
mod create_node_appends_the_column_head_node_n3;
#[path = "../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🚨️dup-node-id-86f2e1/🦀️.rs"]
mod create_node_edge_dup_node_id;
#[path = "../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🏗️hall-cut-node-8350fd/🦀️.rs"]
mod delete_node_hall_hall_cut_node;
#[path = "../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_node_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚨️no-such-node-4027a8/🦀️.rs"]
mod delete_node_edge_no_such_node;
#[path = "../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_node_removes_the_column_head_node_under_a_live_frame;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-node/🧪️tests/⏸️same-node-32a2a4/🦀️.rs"]
mod replace_node_same_node;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-node/🧪️tests/🏗️hall-lifts-ridge-746bae/🦀️.rs"]
mod replace_node_hall_hall_lifts_ridge;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-node/🧪️tests/📍️lifts/🦀️.rs"]
mod replace_node_lifts_the_column_head;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-node/🧪️tests/🚨️no-such-node-166880/🦀️.rs"]
mod replace_node_no_such_node;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-node/🧪️tests/🪪️renames-node-4a2286/🦀️.rs"]
mod replace_node_renames_node;
#[path = "../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/➖️appends/🦀️.rs"]
mod create_element_appends_a_diagonal_bracing_bar;
