//! 🔬️ Integration tests of the `s.fem.fem3d` analysis-settings mutation kind against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/⏸️same/🦀️.rs"]
mod update_analysis_settings_edge_same_settings;
#[path = "../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🏗️hall/🦀️.rs"]
mod update_analysis_settings_hall_hall_more_modes;
#[path = "../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔢️doubles/🦀️.rs"]
mod update_analysis_settings_doubles_the_buckling_mode_count;
#[path = "../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🧨️zero/🦀️.rs"]
mod update_analysis_settings_edge_zero_modes;
