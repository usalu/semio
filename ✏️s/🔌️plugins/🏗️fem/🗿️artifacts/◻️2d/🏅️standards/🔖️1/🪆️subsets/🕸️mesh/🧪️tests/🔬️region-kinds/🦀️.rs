//! 🔬️ Integration tests of the `s.fem.fem2d` region mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🏢️infills-the-upper-6cc520/🦀️.rs"]
mod create_region_infills_the_upper_storey_bay_with_a_concrete_panel;
#[path = "../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/📐️denies-two-point-99954a/🦀️.rs"]
mod create_region_refuses_a_floor_slab_region_outlined_by_only_two_points;
#[path = "../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🕳️denies-loose-hole-d9efa1/🦀️.rs"]
mod create_region_refuses_an_upper_bay_panel_whose_door_opening_lies_outside_it;
#[path = "../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🚫️rejects/🦀️.rs"]
mod create_region_rejects_a_duplicate_region_id_on_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/🗺️create-region/🧪️tests/🧱️appends-a-solid-d78275/🦀️.rs"]
mod create_region_appends_a_solid_rectangular_slab;
#[path = "../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/⛔️rejects-a-missing-a83a6d/🦀️.rs"]
mod delete_region_rejects_deleting_a_region_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🚫️delete-region/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_region_removes_a_middle_row;
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
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪟️widens/🦀️.rs"]
mod replace_region_widens_the_window_opening_in_the_infill_wall_panel;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪪️denies-rename-574c91/🦀️.rs"]
mod replace_region_refuses_to_rename_the_infill_panel_through_a_replace_region;
