//! 🔬️ Integration tests of the `s.fem.fem3d` combination mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🏗️hall/🦀️.rs"]
mod create_combination_hall_hall_new_acc;
#[path = "../../🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🔗️appends-a-8ede20/🦀️.rs"]
mod create_combination_appends_a_serviceability_combination_keyed_by_case_id;
#[path = "../../🧬️schema/🧬️mutations/🔗️create-combination/🧪️tests/🚨️dangling/🦀️.rs"]
mod create_combination_edge_dangling_term;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/✂️removes-the-182f7b/🦀️.rs"]
mod delete_combination_removes_the_serviceability_combination_and_keeps_both_cases;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/🏗️hall-cut-qp-ebd806/🦀️.rs"]
mod delete_combination_hall_hall_cut_qp;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod delete_combination_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/✂️delete-combination/🧪️tests/🚨️no/🦀️.rs"]
mod delete_combination_edge_no_such_combo;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/⏸️same/🦀️.rs"]
mod replace_combination_same_combination;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/🏗️hall/🦀️.rs"]
mod replace_combination_hall_hall_retunes_sls;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/🔁️reweights/🦀️.rs"]
mod replace_combination_reweights_the_terms;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/🚨️dangling/🦀️.rs"]
mod replace_combination_dangling_term;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/🚨️no/🦀️.rs"]
mod replace_combination_no_such_combo;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-combination/🧪️tests/🪪️renames/🦀️.rs"]
mod replace_combination_renames_combo;
