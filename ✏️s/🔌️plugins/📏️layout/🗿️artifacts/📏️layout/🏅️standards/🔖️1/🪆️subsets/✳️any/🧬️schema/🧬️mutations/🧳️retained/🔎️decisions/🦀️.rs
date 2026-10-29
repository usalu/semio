//! 🔎️ Original Layout decisions and bounded source cursors.
#[path="👁️source.rs"]pub mod source;
pub use source::*;
#[path="../🎚️simple/🦀️.rs"]pub mod simple;
#[path="../🔎️target/🦀️.rs"]pub mod target;
#[path="../📄️document/🎚️decision.rs"]pub mod document_decision;
#[path="../📄️document/⚖️equality/🦀️.rs"]pub mod document_equality;
#[path="../🧾️data/🗺️plan.rs"]pub mod data_plan;
#[path="../🧾️data/🎚️decision.rs"]pub mod data_decision;
#[path="../📎️overrides/🗺️plan.rs"]pub mod override_plan;
#[path="../📎️overrides/🎚️decision.rs"]pub mod override_decision;
#[path="../📖️story/🗺️plan.rs"]pub mod story_plan;
#[path="../📖️story/📝️text/🦀️.rs"]pub mod story_text;
#[path="../📖️story/🧷️source.rs"]pub mod story_source;
#[path="../🖼️frame/🔎️decision/🦀️.rs"]pub mod frame_decision;
#[path="../🖼️frame/👥️batch/🗺️plan/🦀️.rs"]pub mod batch_plan;
#[path="../🖼️frame/👥️batch/⚠️message/🦀️.rs"]pub mod batch_message;
#[path="../🗄️parent/🌱️creation/🦀️.rs"]pub mod parent_creation;
