//! 🔬️ Integration tests of the `s.fem.fem3d` material mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;

#[path = "../../🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🏗️hall/🦀️.rs"]
mod create_material_hall_hall_new_steel;
#[path = "../../🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🚨️dup/🦀️.rs"]
mod create_material_edge_dup_material_id;
#[path = "../../🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🧨️nu/🦀️.rs"]
mod create_material_edge_nu_at_a_half;
#[path = "../../🧬️schema/🧬️mutations/🌱️create-material/🧪️tests/🪙️appends-an-9fdced/🦀️.rs"]
mod create_material_appends_an_aluminium_alloy;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/⛓️glulam/🦀️.rs"]
mod delete_material_edge_glulam_in_use;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🏗️hall/🦀️.rs"]
mod delete_material_hall_hall_cut_gl32c;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_material_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🚨️no/🦀️.rs"]
mod delete_material_edge_no_such_material;
#[path = "../../🧬️schema/🧬️mutations/🗑️delete-material/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_material_removes_the_unreferenced_aluminium_alloy;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/⏸️same/🦀️.rs"]
mod replace_material_edge_same_material;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🏗️hall/🦀️.rs"]
mod replace_material_hall_hall_regrades;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/📉️softens/🦀️.rs"]
mod replace_material_softens_the_steel_shear_modulus_in_place;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🧨️negative/🦀️.rs"]
mod replace_material_edge_negative_e;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-material/🧪️tests/🪪️renames/🦀️.rs"]
mod replace_material_edge_renames_c24;
#[path = "../../🧬️schema/🧬️mutations/🩹️change-material/🧪️tests/🔬️unit/🦀️.rs"]
mod change_material_field_intent;
