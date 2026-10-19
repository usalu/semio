//! 🔬️ Integration tests of the `s.fem.fem2d` node and element mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;

#[path = "../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/📐️braces-the-upper-d96634/🦀️.rs"]
mod create_element_braces_the_upper_storey_with_a_chs_diagonal;
#[path = "../../🧬️schema/🧬️mutations/🧩️create-element/🧪️tests/🚫️rejects/🦀️.rs"]
mod create_element_rejects_an_element_whose_start_node_is_missing;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/⛔️rejects-a-missing-611215/🦀️.rs"]
mod delete_element_rejects_deleting_an_element_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/✂️cuts-the-lower-e4a250/🦀️.rs"]
mod delete_element_cuts_the_lower_storey_bracing_diagonal;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_element_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🔗️blocks-udl-a1df8e/🦀️.rs"]
mod delete_element_refuses_to_delete_the_floor_beam_two_member_udls_still_load;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-element/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_element_removes_bar_e2_and_keeps_its_end_nodes;
#[path = "../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/♻️converts/🦀️.rs"]
mod replace_element_converts_beam_e1_into_a_bar_in_place;
#[path = "../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/⛔️rejects-a-missing-bd448c/🦀️.rs"]
mod replace_element_rejects_replacing_an_element_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🔧️regrades/🦀️.rs"]
mod replace_element_regrades_the_roof_beam_onto_the_ipe270_profile;
#[path = "../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🚫️dangling-start-cda887/🦀️.rs"]
mod replace_element_refuses_to_replace_the_brace_onto_a_start_node_that_does_not_exist;
#[path = "../../🧬️schema/🧬️mutations/♻️replace-element/🧪️tests/🪪️denies-rename-0d46d8/🦀️.rs"]
mod replace_element_refuses_to_rename_the_roof_beam_through_a_replace_element;
#[path = "../../🧬️schema/🧬️mutations/🩹️change-node/🧪️tests/🔬️unit/🦀️.rs"]
mod change_node_field_intent;
