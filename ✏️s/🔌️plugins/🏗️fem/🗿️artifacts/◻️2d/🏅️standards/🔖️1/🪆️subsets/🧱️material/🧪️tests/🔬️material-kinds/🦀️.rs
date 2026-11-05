//! 🔬️ Integration tests of the `s.fem.fem2d` material mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as vcs;
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
