//! 🔬️ Integration tests of the `s.fem.fem2d` node and element mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as vcs;
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
