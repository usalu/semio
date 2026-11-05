//! 🔬️ Integration tests of the `s.fem.fem2d` region mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as vcs;
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
