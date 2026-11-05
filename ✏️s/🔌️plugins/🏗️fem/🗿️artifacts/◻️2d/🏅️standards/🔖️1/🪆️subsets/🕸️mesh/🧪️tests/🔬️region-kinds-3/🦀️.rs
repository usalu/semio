//! 🔬️ Integration tests of the `s.fem.fem2d` region mutation kinds against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as vcs;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪟️widens/🦀️.rs"]
mod replace_region_widens_the_window_opening_in_the_infill_wall_panel;
#[path = "../../🧬️schema/🧬️mutations/🔄️replace-region/🧪️tests/🪪️denies-rename-574c91/🦀️.rs"]
mod replace_region_refuses_to_rename_the_infill_panel_through_a_replace_region;
#[path = "../../🧬️schema/🧬️mutations/🩹️change-region/🧪️tests/🔬️unit/🦀️.rs"]
mod change_region_field_intent;
