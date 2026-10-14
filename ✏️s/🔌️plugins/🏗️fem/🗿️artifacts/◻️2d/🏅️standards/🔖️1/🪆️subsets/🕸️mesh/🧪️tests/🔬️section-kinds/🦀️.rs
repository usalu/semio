//! 🔬️ Integration tests of the `s.fem.fem2d` section mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/⚗️denies-zero-area-58b5ca/🦀️.rs"]
mod create_section_refuses_an_ipe_100_profile_whose_area_was_left_at_zero;
#[path = "../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/➕️adds-the-hea220-dfdf34/🦀️.rs"]
mod create_section_adds_the_hea220_profile_to_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/📐️appends/🦀️.rs"]
mod create_section_appends_the_ipe300_profile;
#[path = "../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🚫️rejects/🦀️.rs"]
mod create_section_rejects_a_duplicate_section_id_on_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/⛔️rejects-a-missing-dbd0a4/🦀️.rs"]
mod delete_section_rejects_deleting_a_section_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/✂️drops-the-spare-dcf609/🦀️.rs"]
mod delete_section_drops_the_unreferenced_ipe200_section_from_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_section_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🔗️blocks-in-use-0a6a3c/🦀️.rs"]
mod delete_section_refuses_to_delete_the_heb_200_profile_four_columns_still_carry;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚫️removes-the-spare-1c235a/🦀️.rs"]
mod delete_section_removes_the_spare_hollow_section;
#[path = "../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⚗️denies-zero-iy-404e31/🦀️.rs"]
mod replace_section_refuses_a_roof_beam_profile_whose_second_moment_was_left_at_zero;
#[path = "../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⛔️rejects/🦀️.rs"]
mod replace_section_rejects_replacing_a_section_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/💪️stiffens/🦀️.rs"]
mod replace_section_stiffens_ipe200_with_a_reinforced_profile;
#[path = "../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🛠️thickens/🦀️.rs"]
mod replace_section_thickens_the_chs_brace_wall_to_five_millimetres;
#[path = "../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🪪️denies-rename-1d02dd/🦀️.rs"]
mod replace_section_refuses_to_rename_the_roof_beam_profile_through_a_replace_section;
