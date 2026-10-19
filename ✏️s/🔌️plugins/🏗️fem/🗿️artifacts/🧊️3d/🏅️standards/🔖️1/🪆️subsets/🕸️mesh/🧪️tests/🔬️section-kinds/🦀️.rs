//! 🔬️ Integration tests of the `s.fem.fem3d` section mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;

#[path = "../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🏗️hall-new-beam-251a92/🦀️.rs"]
mod create_section_hall_hall_new_beam;
#[path = "../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🔳️appends/🦀️.rs"]
mod create_section_appends_a_square_hollow_profile;
#[path = "../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🚨️dup-section-id-a76686/🦀️.rs"]
mod create_section_edge_dup_section_id;
#[path = "../../🧬️schema/🧬️mutations/📐️create-section/🧪️tests/🧨️zero-area-475a19/🦀️.rs"]
mod create_section_edge_zero_area;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/⛓️purlin-in-use-99eb01/🦀️.rs"]
mod delete_section_edge_purlin_in_use;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🏗️hall-cut-shs-d44b9e/🦀️.rs"]
mod delete_section_hall_hall_cut_shs;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_section_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚨️no-such-section-50d29b/🦀️.rs"]
mod delete_section_edge_no_such_section;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-section/🧪️tests/🚫️removes/🦀️.rs"]
mod delete_section_removes_the_spare_square_hollow_profile;
#[path = "../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/⏸️same-section-d1d013/🦀️.rs"]
mod replace_section_edge_same_section;
#[path = "../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🌀️raises/🦀️.rs"]
mod replace_section_raises_the_torsion_constant_of_hea200;
#[path = "../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🏗️hall/🦀️.rs"]
mod replace_section_hall_hall_deep_purlin;
#[path = "../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🧨️negative-iy-d4e0a8/🦀️.rs"]
mod replace_section_edge_negative_iy;
#[path = "../../🧬️schema/🧬️mutations/📏️replace-section/🧪️tests/🪪️renames-purlin-dfe160/🦀️.rs"]
mod replace_section_edge_renames_purlin;
#[path = "../../🧬️schema/🧬️mutations/🩹️change-section/🧪️tests/🔬️unit/🦀️.rs"]
mod change_section_field_intent;
