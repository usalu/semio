//! 🔬️ Integration tests of the `s.fem.fem2d` load mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as vcs;
#[path = "../../🧬️schema/🧬️mutations/➖️remove-load/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
mod remove_load_removes_a_middle_row;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/⛔️rejects-a-missing-7a1e1f/🦀️.rs"]
mod replace_load_rejects_replacing_a_load_the_wind_case_never_carried;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/🏋️retunes-the-live-f6fd49/🦀️.rs"]
mod replace_load_retunes_the_live_point_load_on_the_timber_portal_frame;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/👻️dangling-node-b95678/🦀️.rs"]
mod replace_load_refuses_to_move_a_wind_load_onto_a_node_the_steel_frame_never_had;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/💨️strengthens/🦀️.rs"]
mod replace_load_strengthens_the_wind_point_load_on_the_braced_steel_frame;
#[path = "../../🧬️schema/🧬️mutations/🔁️replace-load/🧪️tests/🪪️denies-rename-732de0/🦀️.rs"]
mod replace_load_refuses_to_relabel_a_wind_load_through_a_replace_load;
#[path = "../../🧬️schema/🧬️mutations/🩹️change-load/🧪️tests/🔬️unit/🦀️.rs"]
mod change_load_field_intent;
