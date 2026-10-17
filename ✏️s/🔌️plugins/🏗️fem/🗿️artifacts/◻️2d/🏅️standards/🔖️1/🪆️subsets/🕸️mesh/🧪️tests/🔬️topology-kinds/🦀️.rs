//! 🔬️ Integration tests of the `s.fem.fem2d` node and element mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🏢️appends-the-canopy-226a20/🦀️.rs"]
mod create_node_appends_the_canopy_strut_head_node_to_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/📍️appends-node-n3/🦀️.rs"]
mod create_node_appends_node_n3;
#[path = "../../🧬️schema/🧬️mutations/⚪️create-node/🧪️tests/🚫️rejects-a-duplicate-eb0df0/🦀️.rs"]
mod create_node_rejects_a_duplicate_node_id_on_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/⛔️rejects-a-missing-429801/🦀️.rs"]
mod delete_node_rejects_deleting_a_node_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_node_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🗑️drops-the-spare-6c285d/🦀️.rs"]
mod delete_node_drops_the_spare_canopy_node_from_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_node_removes_node_n3_without_cascading_to_its_support;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-node/🧪️tests/⛔️rejects-a-missing-334de5/🦀️.rs"]
mod replace_node_rejects_replacing_a_node_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-node/🧪️tests/📍️widens-the-canopy-553d69/🦀️.rs"]
mod replace_node_widens_the_canopy_tip_of_the_steel_frame_by_half_a_metre;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-node/🧪️tests/🕹️raises-the-ridge-e53b00/🦀️.rs"]
mod replace_node_raises_the_ridge_node_of_the_timber_portal_frame_by_600_millimetres;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-node/🧪️tests/🪪️denies-rename-e69720/🦀️.rs"]
mod replace_node_refuses_to_relabel_a_steel_frame_node_through_a_replace_node;
#[path = "../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/➖️appends/🦀️.rs"]
mod create_element_appends_bar_e2_between_n2_and_n3;
