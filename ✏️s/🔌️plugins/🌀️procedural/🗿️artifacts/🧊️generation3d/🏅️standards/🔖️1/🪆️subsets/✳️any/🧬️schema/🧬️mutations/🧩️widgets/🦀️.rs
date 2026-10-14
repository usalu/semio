//! 🧩️ Generation3d widget mutation leaves: create, update, delete and move widgets, drop their position, set slider values and inputs and shift nodes. Every leaf is generic over the dispatch enum.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

//#region 🔖️GestureLeaves
/// 🖊️ One number as the history labels print it: English with a decimal point, German with a decimal comma.
pub fn generation3d_label_number(value: f64) -> (String, String) {
    let english = format!("{}", (value * 1_000.0).round() / 1_000.0);
    let german = english.replace('.', ",");
    (english, german)
}

/// 🧺️ `count` items as the history labels name them, English and German.
pub fn generation3d_label_items(count: usize, english: &str, german: &str) -> (String, String) {
    (format!("{count} {english}"), format!("{count} {german}"))
}

/// 🧱️ The payload-intrinsic target law every relative gesture leaf states in its schema: at least one id, each once.
pub fn generation3d_targets_invariant(targets: &[String]) -> Result<(), &'static str> {
    if targets.is_empty() || targets.iter().any(String::is_empty) {
        return Err("a gesture leaf names at least one non-empty target");
    }
    if targets.iter().enumerate().any(|(at, id)| targets[..at].contains(id)) {
        return Err("a gesture leaf names each target once");
    }
    Ok(())
}

/// 🩹️ The `mutation.partial` warning of the targets a relative leaf skipped, or nothing.
pub fn generation3d_partial(skipped: Vec<String>, total: usize, reason: &str) -> Option<protocol::MutationMessage> {
    (!skipped.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {total} target(s) skipped ({reason}): {}", skipped.len(), skipped.join(", "))).at(skipped))
}

/// 🪡️ Merges native parameter entries and explicitly retires the temporary patch owner.
pub fn generation3d_with_params(widget: &semio_framework_artifact_flow_flow::Widget, entries: Vec<(&str, semio_framework_artifact_flow_flow::neural::Value)>) -> semio_framework_artifact_flow_flow::Widget {
    use semio_framework_artifact_flow_flow::neural::{ColdRetire, Dictionary};
    let mut patch = Dictionary::new();
    for (key, entry) in entries { patch = patch.insert(key, entry); }
    let semio_framework_artifact_flow_flow::Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } = widget else {
        patch.retire_cold();
        return widget.clone();
    };
    let merged = params.merge(&patch);
    patch.retire_cold();
    semio_framework_artifact_flow_flow::Widget::Neuron { id: id.clone(), neuron_kind: neuron_kind.clone(), params: merged, input_ports: input_ports.clone(), output_ports: output_ports.clone(), preview: *preview }
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
pub mod update_widget {
    #[path = "../🩹update-widget/🦀️.rs"]
    mod component;
    #[path = "../🩹update-widget/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🩹update-widget/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod delete_widget {
    #[path = "../❌delete-widget/🦀️.rs"]
    mod component;
    #[path = "../❌delete-widget/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../❌delete-widget/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod move_widget {
    #[path = "../📍️move-widget/🦀️.rs"]
    mod component;
    #[path = "../📍️move-widget/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📍️move-widget/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod delete_widget_position {
    #[path = "../🧹️delete-widget-position/🦀️.rs"]
    mod component;
    #[path = "../🧹️delete-widget-position/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧹️delete-widget-position/↩️inverse/🦀️.rs"]
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
pub mod change_widget_input {
    #[path = "../🎛️change-widget-input/🦀️.rs"]
    mod component;
    #[path = "../🎛️change-widget-input/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🎛️change-widget-input/↩️inverse/🦀️.rs"]
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
