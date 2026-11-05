//! 🔬️ Integration tests of the `s.fem.fem2d` region mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as vcs;
#[path = "../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🔗️blocks-in-use-7c7862/🦀️.rs"]
mod delete_region_refuses_to_delete_the_infill_panel_the_wind_case_still_presses_on;
#[path = "../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_region_removes_the_slab_and_keeps_its_material;
#[path = "../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/🧹️drops-the-spare-460714/🦀️.rs"]
mod delete_region_drops_the_spare_side_panel_region_from_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/⛔️rejects-a-missing-6e0d70/🦀️.rs"]
mod replace_region_rejects_replacing_a_region_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/👻️dangling-mat-7ef81b/🦀️.rs"]
mod replace_region_refuses_to_repour_the_infill_panel_in_a_grade_the_model_lacks;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/📐️denies/🦀️.rs"]
mod replace_region_refuses_an_infill_panel_whose_thickness_was_set_to_zero;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪜️punches-a-stair-f7b3b1/🦀️.rs"]
mod replace_region_punches_a_stair_opening_through_the_slab;
