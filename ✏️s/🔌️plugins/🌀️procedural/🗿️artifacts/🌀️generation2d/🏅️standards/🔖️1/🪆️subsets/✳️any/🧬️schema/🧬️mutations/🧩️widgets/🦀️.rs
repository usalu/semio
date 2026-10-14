//! 🧩️ Generation2d widget mutation leaves: create, delete, replace and move widgets, clear their layout, set slider values and shift nodes. Every leaf is generic over the dispatch enum.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

//#region 🔖️GestureLeaves
/// 🖊️ One number as the history labels print it: English with a decimal point, German with a decimal comma.
pub fn generation2d_label_number(value: f64) -> (String, String) {
    let english = format!("{}", (value * 1_000.0).round() / 1_000.0);
    let german = english.replace('.', ",");
    (english, german)
}

/// 🧱️ The payload-intrinsic target law every relative gesture leaf states in its schema: at least one id, each once.
pub fn generation2d_targets_invariant(targets: &[String]) -> Result<(), &'static str> {
    if targets.is_empty() || targets.iter().any(String::is_empty) {
        return Err("a gesture leaf names at least one non-empty target");
    }
    if targets.iter().enumerate().any(|(at, id)| targets[..at].contains(id)) {
        return Err("a gesture leaf names each target once");
    }
    Ok(())
}

/// 🩹️ The `mutation.partial` warning of the targets a relative leaf skipped, or nothing.
pub fn generation2d_partial(skipped: Vec<String>, total: usize, reason: &str) -> Option<protocol::MutationMessage> {
    (!skipped.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {total} target(s) skipped ({reason}): {}", skipped.len(), skipped.join(", "))).at(skipped))
}
//#endregion 🔖️GestureLeaves

#[path = "."]
pub mod create_widget {
    #[path = "../🌱️create-widget/🦀️.rs"]
    mod component;
    #[path = "../🌱️create-widget/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🌱️create-widget/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod delete_widget {
    #[path = "../🗑️delete-widget/🦀️.rs"]
    mod component;
    #[path = "../🗑️delete-widget/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🗑️delete-widget/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod replace_widget {
    #[path = "../🔁️replace-widget/🦀️.rs"]
    mod component;
    #[path = "../🔁️replace-widget/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔁️replace-widget/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod move_widget {
    #[path = "../📍️move/🦀️.rs"]
    mod component;
    #[path = "../📍️move/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📍️move/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod clear_widget_layout {
    #[path = "../🧹clear-widget-layout/🦀️.rs"]
    mod component;
    #[path = "../🧹clear-widget-layout/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧹clear-widget-layout/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod change_slider_value {
    #[path = "../🎚️change-slider-value/🦀️.rs"]
    mod component;
    #[path = "../🎚️change-slider-value/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🎚️change-slider-value/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod move_nodes {
    #[path = "../🚚️move-nodes/🦀️.rs"]
    mod component;
    #[path = "../🚚️move-nodes/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🚚️move-nodes/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
