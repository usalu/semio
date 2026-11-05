//! 🔬️ Integration tests of the `s.fem.fem2d` material mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as vcs;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🗑️drops/🦀️.rs"]
mod delete_material_drops_the_unreferenced_s235_material_from_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🚫️removes-the-30f7a2/🦀️.rs"]
mod delete_material_removes_the_unreferenced_timber_material;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/⚗️denies/🦀️.rs"]
mod replace_material_refuses_a_concrete_row_whose_modulus_was_left_at_zero;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/⛔️rejects/🦀️.rs"]
mod replace_material_rejects_replacing_a_material_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🏗️restates/🦀️.rs"]
mod replace_material_restates_steel_as_s355_in_its_original_slot;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/📉️cracks/🦀️.rs"]
mod replace_material_cracks_the_c30_37_stiffness_in_half_for_the_infill_panel;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🪪️denies/🦀️.rs"]
mod replace_material_refuses_to_regrade_the_concrete_by_renaming_it_through_a_replace;
