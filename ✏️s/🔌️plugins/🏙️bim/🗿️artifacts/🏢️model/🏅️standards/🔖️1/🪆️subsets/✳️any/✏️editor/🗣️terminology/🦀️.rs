//! 🗣️ BIM editor terminology: the one `app_labels!` block for the whole app (never split per window or panel), so every locale cell is compile-checked
//! in one place. No language is the default: the locale always comes from the addressed view model. `{name}` and `{count}` are named placeholders.

macro_rules! bim_labels {
    ($( $field:ident: $en:literal, $de:literal; )+) => {
        semio_framework_ui_locale::app_labels! {
            pub struct BimLabels {
                $( $field: native_en $en, native_de $de, reuse_en $en, reuse_de $de );+
            }
        }
    };
}

macro_rules! bim_labels_emit {
    (; $($rows:tt)*) => {
        bim_labels! { $($rows)* }
    };
}

#[macro_use]
#[path = "🏠️model-and-window-labels/🦀️.rs"]
mod model_and_window_labels;

#[macro_use]
#[path = "🎮️command-and-argument-labels/🦀️.rs"]
mod command_and_argument_labels;

#[macro_use]
#[path = "⚠️fault-and-utility-labels/🦀️.rs"]
mod fault_and_utility_labels;

#[macro_use]
#[path = "🔍️review-and-schedule-labels/🦀️.rs"]
mod review_and_schedule_labels;

//#region 🔖️Labels
// 🗣️ A plain `//` comment: rustdoc does not document macro invocations.
bim_label_rows_model! { bim_label_rows_commands, bim_label_rows_faults, bim_label_rows_review, bim_labels_emit; }
//#endregion 🔖️Labels

/// 🗣️ Resolves the label set of the addressed view's locale and terminology.
pub fn bim_labels(view_state: &semio_framework_plugin::ViewModel) -> &'static BimLabels {
    semio_framework_plugin::resolve_labels::<BimLabels>(view_state)
}

impl BimLabels {
    /// 🌐️ The English and German text of one label field, read through borrowed constants: `BimLabels::NATIVE_EN.field` copies the whole ~17 KiB label set into the caller's frame
    /// at `opt-level = 0`, so a table of 100+ rows spent 7 MiB of stack before the app existed (ticket `r13-stack`).
    pub fn localized(pick: fn(&Self) -> semio_framework_ui_locale::LabelText) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(pick(&Self::NATIVE_EN).as_str(), pick(&Self::NATIVE_DE).as_str())
    }

    /// 🧩️ Fills the `{name}` placeholder of a label.
    pub fn named(text: semio_framework_ui_locale::LabelText, name: &str) -> String {
        text.as_str().replace("{name}", name)
    }

    /// 🧮️ Fills the `{count}` placeholder of a label.
    pub fn counted(text: semio_framework_ui_locale::LabelText, count: usize) -> String {
        text.as_str().replace("{count}", &count.to_string())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
