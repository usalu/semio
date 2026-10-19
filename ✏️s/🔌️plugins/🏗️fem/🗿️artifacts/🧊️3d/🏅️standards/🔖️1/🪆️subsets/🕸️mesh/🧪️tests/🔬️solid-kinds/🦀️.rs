//! 🔬️ Integration tests of the `s.fem.fem3d` solid mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;

#[path = "../../🧬️schema/🧬️mutations/🧊️create-solid/🧪️tests/🏗️hall-new-slab-d79da4/🦀️.rs"]
mod create_solid_hall_hall_new_slab;
#[path = "../../🧬️schema/🧬️mutations/🧊️create-solid/🧪️tests/🏠️appends/🦀️.rs"]
mod create_solid_appends_an_extruded_roof_slab;
#[path = "../../🧬️schema/🧬️mutations/🧊️create-solid/🧪️tests/📐️sliver-outline-316a7c/🦀️.rs"]
mod create_solid_edge_sliver_outline;
#[path = "../../🧬️schema/🧬️mutations/🧊️create-solid/🧪️tests/🚨️dangling-mat-1ebd78/🦀️.rs"]
mod create_solid_edge_dangling_mat;
#[path = "../../🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/⛓️raft-under-load-e4ea39/🦀️.rs"]
mod delete_solid_edge_raft_under_load;
#[path = "../../🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/🏗️hall-cut-apron-6c79d3/🦀️.rs"]
mod delete_solid_hall_hall_cut_apron;
#[path = "../../🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_solid_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/🚨️no-such-solid-f08d23/🦀️.rs"]
mod delete_solid_edge_no_such_solid;
#[path = "../../🧬️schema/🧬️mutations/🚫️delete-solid/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_solid_removes_the_roof_slab_and_keeps_its_material;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/⏸️same-solid-8ad12c/🦀️.rs"]
mod replace_solid_edge_same_solid;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/🏗️hall-thick-raft-cddc0f/🦀️.rs"]
mod replace_solid_hall_hall_thick_raft;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/📐️zero-height-2b131a/🦀️.rs"]
mod replace_solid_edge_zero_height;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/📚️thickens/🦀️.rs"]
mod replace_solid_thickens_the_slab_and_adds_a_mesh_layer;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/🚨️dangling-mat-9c89da/🦀️.rs"]
mod replace_solid_edge_dangling_mat;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-solid/🧪️tests/🪪️renames-apron-7bfadd/🦀️.rs"]
mod replace_solid_edge_renames_apron;
#[path = "../../🧬️schema/🧬️mutations/🩹️change-solid/🧪️tests/🔬️unit/🦀️.rs"]
mod change_solid_field_intent;
