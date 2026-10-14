//! 🔬️ Integration tests of the `s.fem.fem2d` material mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/⚗️denies/🦀️.rs"]
mod create_material_refuses_an_elastomeric_bearing_at_the_incompressible_poisson_limit;
#[path = "../../🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🏗️adds/🦀️.rs"]
mod create_material_adds_the_c25_30_material_for_the_ground_slab;
#[path = "../../🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🚫️rejects/🦀️.rs"]
mod create_material_rejects_a_duplicate_material_id_on_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🧱️appends/🦀️.rs"]
mod create_material_appends_concrete_c30;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/⛔️rejects/🦀️.rs"]
mod delete_material_rejects_deleting_a_material_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_material_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🔗️blocks/🦀️.rs"]
mod delete_material_refuses_to_delete_the_s355_grade_seven_members_are_made_of;
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
