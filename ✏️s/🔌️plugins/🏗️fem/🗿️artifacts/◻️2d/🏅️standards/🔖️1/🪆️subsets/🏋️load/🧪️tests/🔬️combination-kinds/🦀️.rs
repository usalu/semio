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
#[path = "../../🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/🗑️drops/🦀️.rs"]
mod delete_combination_drops_the_spare_uls_combination_from_the_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/⚖️reweights/🦀️.rs"]
mod replace_combination_adds_a_snow_term_to_the_uls_combination_of_the_timber_portal_frame;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/⛔️rejects/🦀️.rs"]
mod replace_combination_rejects_replacing_a_combination_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/👻️dangling/🦀️.rs"]
mod replace_combination_refuses_a_uls_term_weighting_a_case_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/👻️self-term-0f54d1/🦀️.rs"]
mod replace_combination_refuses_a_uls_term_that_weights_the_combination_itself;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/🔗️adds/🦀️.rs"]
mod replace_combination_adds_a_wind_term_to_the_uls_combination_of_the_braced_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/🪪️denies/🦀️.rs"]
mod replace_combination_refuses_to_relabel_a_combination_through_a_replace_combination;
#[path = "../../🧬️schema/🧬️mutations/🩹️change-combination-term/🧪️tests/🔬️unit/🦀️.rs"]
mod change_combination_term_field_intent;
#[path = "../../🧬️schema/🧬️mutations/🩹️change-combination-name/🧪️tests/🔬️unit/🦀️.rs"]
mod change_combination_name_field_intent;
