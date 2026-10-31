//! 🔬️ Integration tests of the `s.fem.fem2d` combination mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
#[path = "../../🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/➕️appends/🦀️.rs"]
mod create_combination_appends_the_six_ten_a_combination_over_dead_and_snow;
#[path = "../../🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🔗️appends/🦀️.rs"]
mod create_combination_appends_an_uls_combination_over_both_cases;
#[path = "../../🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🚫️rejects/🦀️.rs"]
mod create_combination_rejects_a_combination_term_naming_a_load_case_that_is_absent;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/⛔️rejects/🦀️.rs"]
mod delete_combination_rejects_deleting_a_combination_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/✂️removes/🦀️.rs"]
mod delete_combination_removes_the_uls_combination_and_keeps_both_cases;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_combination_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/🔗️blocks-in-use-0b898b/🦀️.rs"]
mod delete_combination_refuses_to_delete_an_uls_combination_a_design_envelope_nests;
