//! 🔬️ Integration tests of the `s.fem.fem2d` analysis-settings mutation kind against the committed fixtures, the dispatch enum and the central apply.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🎚️raises/🦀️.rs"]
mod update_analysis_settings_raises_the_mode_counts_and_tightens_the_deformation_scale;
#[path = "../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔁️keeps/🦀️.rs"]
mod update_analysis_settings_keeps_the_analysis_settings_exactly_as_they_are;
#[path = "../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🔢️doubles/🦀️.rs"]
mod update_analysis_settings_doubles_the_modal_count_and_halves_the_deformation_scale;
#[path = "../../🧬️schema/🧬️mutations/🎛️update-analysis-settings/🧪️tests/🚫️denies/🦀️.rs"]
mod update_analysis_settings_refuses_an_analysis_configured_to_extract_zero_modal_modes;
