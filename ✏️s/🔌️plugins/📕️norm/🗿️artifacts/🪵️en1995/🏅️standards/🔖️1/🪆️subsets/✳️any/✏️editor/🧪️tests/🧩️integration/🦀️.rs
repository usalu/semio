//! 🧩️ Editor unit tests of every window, panel and command, run against the public editor surface.

pub use semio_s_artifact_norm_en1995_editor::*;

#[path = "../../🎭️modes/✏️edit/🧪️tests/🔬️unit/🦀️.rs"]
mod tests_1;

#[path = "../../🎭️modes/✏️edit/🪟️windows/✅️checks/🧪️tests/🔬️unit/🦀️.rs"]
mod tests_2;

#[path = "../../🎭️modes/✏️edit/🪟️windows/🌲️outliner/🧪️tests/🔬️unit/🦀️.rs"]
mod tests_3;

#[path = "../../🎭️modes/✏️edit/🪟️windows/📈️diagrams/🧪️tests/🔬️unit/🦀️.rs"]
mod tests_4;

#[path = "../../🎭️modes/✏️edit/🪟️windows/🧊️viewport/🧪️tests/🔬️unit/🦀️.rs"]
mod tests_5;

#[path = "../../🎭️modes/✏️edit/🪟️windows/🧾️properties/🧪️tests/🔬️unit/🦀️.rs"]
mod tests_6;

#[path = "../../🎮️commands/☑️selected-check/🧪️tests/🔬️unit/🦀️.rs"]
mod tests_7;

#[path = "../../📌️panels/📚️catalogue/🧪️tests/🔬️unit/🦀️.rs"]
mod tests_8;

#[path = "../../📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs"]
mod tests_9;

#[path = "../../📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs"]
mod tests_10;

#[path = "../📚️assets/🦀️.rs"]
mod tests_11;

#[path = "../🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
