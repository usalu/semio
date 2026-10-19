//! 🔬️ Integration tests of the `s.fem.fem2d` support mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;

#[path = "../../🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🔻️props/🦀️.rs"]
mod create_support_props_the_canopy_tip_on_a_vertical_roller;
#[path = "../../🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🚫️rejects/🦀️.rs"]
mod create_support_rejects_a_support_on_a_node_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🛡️create-support/🧪️tests/🛞️adds/🦀️.rs"]
mod create_support_adds_a_vertical_roller_at_node_n2;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/⛔️rejects/🦀️.rs"]
mod delete_support_rejects_deleting_a_support_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_support_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🔓️releases-the-82b34f/🦀️.rs"]
mod delete_support_releases_the_roller_at_node_n2;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-support/🧪️tests/🕊️frees/🦀️.rs"]
mod delete_support_frees_the_roof_level_lateral_tie_of_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/⛔️rejects/🦀️.rs"]
mod replace_support_rejects_replacing_a_support_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/👻️dangling/🦀️.rs"]
mod replace_support_refuses_to_move_the_roof_tie_onto_a_node_that_does_not_exist;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🔒️upgrades/🦀️.rs"]
mod replace_support_upgrades_the_roller_at_n2_to_a_full_fixity;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🔩️pins/🦀️.rs"]
mod replace_support_pins_the_left_column_base_by_releasing_its_rotation;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-support/🧪️tests/🪪️denies/🦀️.rs"]
mod replace_support_refuses_to_rename_the_roof_tie_support_through_a_replace_support;
#[path = "../../🧬️schema/🧬️mutations/🩹️change-support/🧪️tests/🔬️unit/🦀️.rs"]
mod change_support_field_intent;
