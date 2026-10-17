//! 🧪️ Aggregated unit tests of the `options` leaves, one module per leaf.

mod font_size {
use crate::editor::writer::modes::edit::windows::main::config::WriterMainWindowConfig;
use semio_framework_plugin::WindowMeasure;
use super::super::font_size::*;
use crate::editor::writer::terminology::writer_play_labels;

#[semio_framework_async_macros::async_test]
async fn the_slider_range_matches_the_command_handler_clamp() {
    let config = WriterMainWindowConfig::default();
    match measure(&config, writer_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))) {
        WindowMeasure::Slider { min, max, .. } => assert!(min == 10.0 && max == 24.0),
        other => panic!("font-size measure must be a slider, got {other:?}"),
    }
}
}

mod line_height {
use crate::editor::writer::modes::edit::windows::main::config::WriterMainWindowConfig;
use semio_framework_plugin::WindowMeasure;
use super::super::line_height::*;
use crate::editor::writer::terminology::writer_play_labels;

#[semio_framework_async_macros::async_test]
async fn the_slider_range_matches_the_command_handler_clamp() {
    let config = WriterMainWindowConfig::default();
    match measure(&config, writer_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))) {
        WindowMeasure::Slider { min, max, .. } => assert!(min == 16.0 && max == 40.0),
        other => panic!("line-height measure must be a slider, got {other:?}"),
    }
}
}

mod line_numbers {
use crate::editor::writer::modes::edit::windows::main::config::WriterMainWindowConfig;
use semio_framework_plugin::WindowMeasure;
use super::super::line_numbers::*;
use crate::editor::writer::terminology::writer_play_labels;

#[semio_framework_async_macros::async_test]
async fn the_toggle_reflects_the_configs_show_line_numbers_flag() {
    let config = WriterMainWindowConfig { editor_settings: crate::WriterEditorSettings { show_line_numbers: false, ..Default::default() }, ..WriterMainWindowConfig::default() };
    match measure(&config, writer_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))) {
        WindowMeasure::Toggle { pressed, .. } => assert!(!pressed),
        other => panic!("line-numbers measure must be a toggle, got {other:?}"),
    }
}
}

mod tab_size {
use crate::editor::writer::modes::edit::windows::main::config::WriterMainWindowConfig;
use semio_framework_plugin::WindowMeasure;
use super::super::tab_size::*;
use crate::editor::writer::terminology::writer_play_labels;

#[semio_framework_async_macros::async_test]
async fn the_slider_range_matches_the_command_handler_clamp() {
    let config = WriterMainWindowConfig::default();
    match measure(&config, writer_play_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))) {
        WindowMeasure::Slider { min, max, .. } => assert!(min == 1.0 && max == 8.0),
        other => panic!("tab-size measure must be a slider, got {other:?}"),
    }
}
}
