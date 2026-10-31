//! 🧪️ Integration tests of the `s.fem.fem2d` play app editor that drive the whole mounted app through the shared `context` harness.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "../🧰️context/🦀️.rs"]
pub mod context;
#[path = "../../🎮️commands/🩹️patch-region/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_region;
#[path = "../../🎮️commands/🩹️patch-section/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_section;
#[path = "../../🎮️commands/🩹️patch-support/🧪️tests/🔬️unit/🦀️.rs"]
mod patch_support;
#[path = "../../🎮️commands/🗂️remove-selection/🧪️tests/🔬️unit/🦀️.rs"]
mod remove_selection;
#[path = "../../🎮️commands/⏱️result-animation-tick/🧪️tests/🔬️unit/🦀️.rs"]
mod result_animation_tick;
#[path = "../../🎮️commands/📚️set-active-example/🧪️tests/🔬️unit/🦀️.rs"]
mod set_active_example;
#[path = "../../🎮️commands/🧮️set-analysis-settings/🧪️tests/🔬️unit/🦀️.rs"]
mod set_analysis_settings;
#[path = "../../🎮️commands/🎥️set-camera/🧪️tests/🔬️unit/🦀️.rs"]
mod set_camera;
#[path = "../../🎮️commands/⏯️set-result-animation/🧪️tests/🔬️unit/🦀️.rs"]
mod set_result_animation;
#[path = "../../🧪️tests/🪟️window-config-ownership/🦀️.rs"]
mod window_config_ownership;
