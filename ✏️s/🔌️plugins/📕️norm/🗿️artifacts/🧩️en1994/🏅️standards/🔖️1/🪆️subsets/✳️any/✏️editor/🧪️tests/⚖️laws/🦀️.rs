//! ⚖️ EN 1994 editor laws: the window, mode, panel and command unit suites run against the public surface of the editor crate.

#![allow(async_fn_in_trait)]
#![cfg(test)]

pub use semio_s_artifact_norm_en1994_editor::*;

#[path = "."]
pub(crate) mod unit_tests {
    #[path = "../🧰️context/🦀️.rs"]
    pub(crate) mod context;
}

#[path = "../../🎭️modes/✏️edit/🪟️windows/🧊️viewport/🧪️tests/🔬️unit/🦀️.rs"]
mod viewport_window_tests;

#[path = "../../🎭️modes/✏️edit/🪟️windows/✅️checks/🧪️tests/🔬️unit/🦀️.rs"]
mod checks_window_tests;

#[path = "../../🎭️modes/✏️edit/🪟️windows/🧾️properties/🧪️tests/🔬️unit/🦀️.rs"]
mod properties_window_tests;

#[path = "../../🎭️modes/✏️edit/🪟️windows/📈️diagrams/🧪️tests/🔬️unit/🦀️.rs"]
mod diagrams_window_tests;

#[path = "../../🎭️modes/✏️edit/🪟️windows/🌲️outliner/🧪️tests/🔬️unit/🦀️.rs"]
mod outliner_window_tests;

#[path = "../../🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
mod edit_mode_tests;

#[path = "../../📌️panels/📚️catalogue/🧪️tests/🔬️unit/🦀️.rs"]
mod catalogue_panel_tests;

#[path = "../../📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
mod inspection_panel_tests;

#[path = "../../📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
mod document_panel_tests;

#[path = "../../🎮️commands/☑️selected-check/🧪️tests/🔬️unit/🦀️.rs"]
mod selected_check_command_tests;
