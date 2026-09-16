//! 📊️ Fem3d play app panel — `results`: the results source/mode controls and the deformation
//! playback transport for the addressed results window.
//!
//! Every control is tagged with the `windowId` it speaks for (the puzzle3d Settings panel's rule), so
//! a split layout showing two results panes cannot retune the one the user is not looking at. Every
//! control also names the FIELD it changes (`{field, value}`), because the host merges a control's own
//! scalar under the single key `value` — a slider has one binding and no other way to say what moved.

use crate::editor::fem3d::modes::edit::windows::results::config::{Fem3dLoopMode, Fem3dResultsWindowConfig, Fem3dWaveform, ANIMATION_PHASE_STEP, ANIMATION_SPEED_MAXIMUM, ANIMATION_SPEED_MINIMUM};
use crate::editor::fem3d::modes::edit::windows::results::transient::Fem3dPlaybackClock;
use crate::editor::fem3d::terminology::Fem3dLabels;
use crate::editor::fem3d::{fem3d_action, ui_label};
use crate::Fem3dSnapshot;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::{ui_node_list, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PluginAssemblyError, UiAssemblyResult, UiMapBuilder, UiText, UiValue};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const PANEL_TAB_ID: &str = "fem3d.panel.results";
pub const BODY_KEY: &str = "fem3d.play.results-panel";

/// 👁️ The command every Display row binds; 🎚️ the one every Playback row binds; 🧮️ the one every
/// Analysis row binds. Named here because the panel's own laws assert on them.
pub const DISPLAY_ACTION: &str = "setResultDisplay";
pub const PLAYBACK_ACTION: &str = "setResultAnimation";
pub const ANALYSIS_ACTION: &str = "setAnalysisSettings";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(PANEL_TAB_ID.into()), label: LocalizedLabel::native("Results", "Ergebnisse"), group: PanelGroup::Display, body_key: Some(BODY_KEY.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Controls
fn error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "fem3d results panel admission failed")
}

fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| error("ui.text"))
}

/// 🎛️ The arguments one control dispatches: the field it owns, an authored value when the gesture
/// carries none of its own (a button), and the `windowId` it speaks for — which
/// `results::config::addressed_window_id` honours over the focused pane, so a split layout cannot
/// retune the results window the user is not looking at. `UiMapBuilder::push` sorts on admission
/// (`🎬️action/🦀️.rs`'s `try_insert`), so the order these are pushed in carries no meaning.
fn control_args(field: &str, value: Option<&str>, window_id: &str) -> UiAssemblyResult<UiValue> {
    let mut builder = UiMapBuilder::try_new().ok_or_else(|| error("ui.value.map"))?;
    builder.push("field".to_owned(), UiValue::Text(ui_text(field)?)).map_err(|_| error("ui.value.map.entry"))?;
    if let Some(value) = value {
        builder.push("value".to_owned(), UiValue::Text(ui_text(value)?)).map_err(|_| error("ui.value.map.entry"))?;
    }
    builder.push("windowId".to_owned(), UiValue::Text(ui_text(window_id)?)).map_err(|_| error("ui.value.map.entry"))?;
    Ok(UiValue::Map(builder.finish()))
}

fn bind<B: HasBase>(builder: B, trigger: Trigger, action: &str, args: UiValue) -> UiAssemblyResult<B> {
    let (action, args) = fem3d_action(action, Some(args))?;
    match args {
        Some(args) => builder.try_on_with(trigger, action, args).map_err(|_| error("ui.control.binding")),
        None => builder.try_on(trigger, action).map_err(|_| error("ui.control.binding")),
    }
}

fn row(id: &str, label: &str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    ui::field(ui_label(label)?).try_id(id).map_err(|_| error("ui.field.id"))?.try_child(control).map_err(|_| error("ui.field.child"))?.try_build().map_err(|_| error("ui.field"))
}

fn select_row(id: &str, label: &str, value: &str, options: &[(String, String)], action: &str, field: &str, window_id: &str) -> UiAssemblyResult<BuiltNode> {
    let mut control = ui::select(ui_text(value)?);
    for (option, option_label) in options {
        control = control.try_item(ui_text(option)?, ui_label(option_label)?).map_err(|_| error("ui.select.item"))?;
    }
    let control = bind(control.try_id(format!("{id}.control")).map_err(|_| error("ui.select.id"))?, Trigger::Change, action, control_args(field, None, window_id)?)?;
    row(id, label, control.try_build().map_err(|_| error("ui.select"))?)
}

fn slider_row(id: &str, label: &str, value: f64, minimum: f64, maximum: f64, step: f64, action: &str, field: &str, window_id: &str) -> UiAssemblyResult<BuiltNode> {
    let control = ui::slider(value).min(minimum).max(maximum).step(step);
    let control = bind(control.try_id(format!("{id}.control")).map_err(|_| error("ui.slider.id"))?, Trigger::Change, action, control_args(field, None, window_id)?)?;
    row(id, label, control.try_build().map_err(|_| error("ui.slider"))?)
}

fn number_row(id: &str, label: &str, value: f64, step: f64, action: &str, field: &str, window_id: &str) -> UiAssemblyResult<BuiltNode> {
    let control = ui::input(InputKind::Number).value(ui_text(format_number(value))?).step(step).commit(ui_text("blur")?);
    let control = bind(control.try_id(format!("{id}.control")).map_err(|_| error("ui.input.id"))?, Trigger::Change, action, control_args(field, None, window_id)?)?;
    row(id, label, control.try_build().map_err(|_| error("ui.input"))?)
}

/// 🔘️ A transport button: it carries BOTH the field and the value, because `Trigger::Activate` has no
/// scalar of its own for the host to merge.
/// ⏯️ The play/pause button of a pane whose state the panel cannot read: it names no field, and a
/// `setResultAnimation` that names nothing toggles play/pause on the window it addresses.
fn toggle_button(id: &str, label: &str, window_id: &str) -> UiAssemblyResult<BuiltNode> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| error("ui.value.map"))?;
    args.push("windowId".to_owned(), UiValue::Text(ui_text(window_id)?)).map_err(|_| error("ui.value.map.entry"))?;
    let control = ui::button(ui_label(label)?).icon(ui_text("play")?);
    let control = bind(control.try_id(id).map_err(|_| error("ui.button.id"))?, Trigger::Activate, PLAYBACK_ACTION, UiValue::Map(args.finish()))?;
    control.try_build().map_err(|_| error("ui.button"))
}

fn transport_button(id: &str, label: &str, icon: &str, field: &str, value: &str, window_id: &str) -> UiAssemblyResult<BuiltNode> {
    let control = ui::button(ui_label(label)?).icon(ui_text(icon)?);
    let control = bind(control.try_id(id).map_err(|_| error("ui.button.id"))?, Trigger::Activate, PLAYBACK_ACTION, control_args(field, Some(value), window_id)?)?;
    control.try_build().map_err(|_| error("ui.button"))
}

/// 🔢️ Trims the trailing `.0` a whole number would otherwise carry into a number input.
fn format_number(value: f64) -> String {
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}

/// 💡️ One plain text row — the panel's hint when it renders a pane it cannot read.
fn hint_row(id: &str, text: &str) -> UiAssemblyResult<BuiltNode> {
    ui::text(ui_label(text)?).try_id(id).map_err(|_| error("ui.text.id"))?.try_build().map_err(|_| error("ui.text"))
}

fn section(id: &str, label: &str, rows: impl IntoIterator<Item = UiAssemblyResult<BuiltNode>>) -> UiAssemblyResult<BuiltNode> {
    ui::section(ui_label(label)?)
        .try_id(id)
        .map_err(|_| error("ui.section.id"))?
        .default_open(true)
        .try_children(ui_node_list(rows)?)
        .map_err(|_| error("ui.section.children"))?
        .try_build()
        .map_err(|_| error("ui.section"))
}
//#endregion 🔖️Controls

//#region 🔖️Render
/// 📊️ How many result sources one select offers before the rest are dropped — the bounded option
/// list every `UiFixedList` admits.
const RESULT_SOURCE_OPTIONS: usize = 24;

/// 👁️ The wire spelling of one result mode — the `select` option value the command parses back.
fn result_mode_value(mode: crate::app_surface::ResultMode) -> &'static str {
    match mode {
        crate::app_surface::ResultMode::Static => "static",
        crate::app_surface::ResultMode::Modal => "modal",
        crate::app_surface::ResultMode::Buckling => "buckling",
    }
}

/// 📊️ The result sources one document offers: every load case, then every combination.
fn result_sources(doc: &Fem3dSnapshot) -> Vec<(String, String)> {
    doc.load_cases.iter().map(|case| (case.id.clone(), case.name.clone())).chain(doc.combinations.iter().map(|combination| (combination.id.clone(), combination.name.clone()))).take(RESULT_SOURCE_OPTIONS).collect()
}

/// 📊️ Renders the panel for ONE results window. `window` is that window's captured partition, or
/// `None` when the projection captured a different pane: the controls then show defaults, the play
/// button is a bare play/pause toggle (its state cannot be read), and a hint row says which pane to
/// focus for live readouts — a panel projection binds to the focused window, and nothing else can read
/// another window's configuration. `clock` is that window's running playback clock, so the phase
/// slider follows the animation while it plays and rests where it paused.
pub fn render(doc: &Fem3dSnapshot, window: Option<&Fem3dResultsWindowConfig>, clock: Option<&Fem3dPlaybackClock>, window_id: &str, labels: &Fem3dLabels) -> UiAssemblyResult<BuiltNode> {
    let live = window.is_some();
    let window = crate::editor::fem3d::modes::edit::windows::results::config::effective(window.unwrap_or(&Fem3dResultsWindowConfig::default()), clock);
    let animation = &window.animation;
    let sources = result_sources(doc);
    let source = window.result_source_id.clone().or_else(|| sources.first().map(|(id, _)| id.clone())).unwrap_or_default();
    let modes = [("static".to_owned(), labels.static_mode.as_str().to_owned()), ("modal".to_owned(), labels.modal.as_str().to_owned()), ("buckling".to_owned(), labels.buckling.as_str().to_owned())];
    let loops = [
        ("loop".to_owned(), labels.loop_mode.as_str().to_owned()),
        ("pingPong".to_owned(), labels.ping_pong.as_str().to_owned()),
        ("once".to_owned(), labels.once.as_str().to_owned()),
    ];
    let waveforms = [("ramp".to_owned(), labels.ramp.as_str().to_owned()), ("sine".to_owned(), labels.sine.as_str().to_owned())];
    let loop_value = match animation.loop_mode {
        Fem3dLoopMode::Loop => "loop",
        Fem3dLoopMode::PingPong => "pingPong",
        Fem3dLoopMode::Once => "once",
    };
    let waveform_value = match animation.waveform {
        Fem3dWaveform::Ramp => "ramp",
        Fem3dWaveform::Sine => "sine",
    };
    let transport = ui::row()
        .try_id("fem3d-play-results.transport.controls")
        .map_err(|_| error("ui.row.id"))?
        .try_children(ui_node_list([
            transport_button("fem3d-play-results.transport.back", &format!("{} \u{2212}", labels.step.as_str()), "skip-back", "phaseStep", &format!("{}", -ANIMATION_PHASE_STEP), window_id),
            if live {
                transport_button(
                    "fem3d-play-results.transport.play",
                    if animation.playing { labels.pause.as_str() } else { labels.play.as_str() },
                    if animation.playing { "pause" } else { "play" },
                    "playing",
                    if animation.playing { "false" } else { "true" },
                    window_id,
                )
            } else {
                toggle_button("fem3d-play-results.transport.play", &format!("{} / {}", labels.play.as_str(), labels.pause.as_str()), window_id)
            },
            transport_button("fem3d-play-results.transport.forward", &format!("{} +", labels.step.as_str()), "skip-forward", "phaseStep", &format!("{ANIMATION_PHASE_STEP}"), window_id),
        ])?)
        .map_err(|_| error("ui.row.children"))?
        .try_build()
        .map_err(|_| error("ui.row"))?;
    let mut playback: Vec<UiAssemblyResult<BuiltNode>> = Vec::new();
    if !live {
        playback.push(hint_row("fem3d-play-results.hint", labels.focus_results_hint.as_str()));
    }
    playback.push(slider_row("fem3d-play-results.phase", labels.phase.as_str(), animation.phase, 0.0, 1.0, 0.01, PLAYBACK_ACTION, "phase", window_id));
    playback.push(row("fem3d-play-results.transport", labels.playing.as_str(), transport));
    playback.push(slider_row("fem3d-play-results.speed", labels.speed.as_str(), animation.speed, ANIMATION_SPEED_MINIMUM, ANIMATION_SPEED_MAXIMUM, 0.05, PLAYBACK_ACTION, "speed", window_id));
    playback.push(select_row("fem3d-play-results.loop", labels.loop_mode.as_str(), loop_value, &loops, PLAYBACK_ACTION, "loopMode", window_id));
    playback.push(select_row("fem3d-play-results.waveform", labels.waveform.as_str(), waveform_value, &waveforms, PLAYBACK_ACTION, "waveform", window_id));
    ui::column()
        .try_id("fem3d-play-results")
        .map_err(|_| error("ui.column.id"))?
        .try_children(ui_node_list([
            section(
                "fem3d-play-results.display",
                labels.display.as_str(),
                [
                    select_row("fem3d-play-results.source", labels.source.as_str(), &source, &sources, DISPLAY_ACTION, "sourceId", window_id),
                    select_row("fem3d-play-results.mode", labels.mode.as_str(), result_mode_value(window.result_mode), &modes, DISPLAY_ACTION, "mode", window_id),
                    number_row("fem3d-play-results.mode-index", labels.mode_index.as_str(), f64::from(window.result_mode_index), 1.0, DISPLAY_ACTION, "modeIndex", window_id),
                ],
            ),
            section("fem3d-play-results.playback", labels.playback.as_str(), playback),
            section(
                "fem3d-play-results.analysis",
                labels.analysis.as_str(),
                [
                    number_row("fem3d-play-results.modal-count", labels.modal_count.as_str(), doc.analysis.modal_count as f64, 1.0, ANALYSIS_ACTION, "modalCount", window_id),
                    number_row("fem3d-play-results.buckling-count", labels.buckling_count.as_str(), doc.analysis.buckling_count as f64, 1.0, ANALYSIS_ACTION, "bucklingCount", window_id),
                    number_row("fem3d-play-results.deformation-scale", labels.deformation_scale.as_str(), doc.analysis.deformation_scale, 10.0, ANALYSIS_ACTION, "deformationScale", window_id),
                ],
            ),
        ])?)
        .map_err(|_| error("ui.column.children"))?
        .try_build()
        .map_err(|_| error("ui.column"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
