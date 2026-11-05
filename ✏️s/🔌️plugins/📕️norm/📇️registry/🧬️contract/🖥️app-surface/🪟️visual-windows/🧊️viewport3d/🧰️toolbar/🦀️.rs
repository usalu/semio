//! 🧰️ The viewport toolbar and window panel: field picker and playback transport above the `world-3d` surface, the one place every Eurocode viewport window renders its dynamics. See the viewport3d crate.

use crate::viewport3d::config::{Lang, Viewport3dWindowConfig};
use crate::viewport3d::fields;
use crate::lane_overlays::{apply, LaneOverlays};
use crate::ViewLane;
use crate::viewport3d::lowering::{lower_dynamic, ViewportInputs};
use crate::viewport3d::measure::MeasureState;
use crate::viewport3d::parts::BasicTessellator;
use crate::viewport3d::playback::{self, PlaybackState, ACTION_SEEK, ACTION_SELECT_TRACK, ACTION_SET_ACTIVE_FIELD, ACTION_SET_SPEED, ACTION_STEP, ACTION_TOGGLE_PLAY};
use semio_s_plugin_norm_structural_scene::{Handle, LocalizedCopy, StructuralScene};
use crate::viewport3d::window::render_viewport3d;
use crate::*;

pub const VIEWPORT_TOOLBAR_ID: &str = "norm-viewport-toolbar";
pub const VIEWPORT_FIELD_SECTION_ID: &str = "norm-viewport-fields";
pub const VIEWPORT_PLAYBACK_SECTION_ID: &str = "norm-viewport-playback";
pub const VIEWPORT_INSERT_SECTION_ID: &str = "norm-viewport-insert";
pub const PLAYBACK_SPEEDS: [f64; 3] = [0.5, 1.0, 2.0];

/// 🎛️ Everything a viewport window renders from besides the scene: the persisted local-only configuration, the ephemeral transport state and the live lanes (grips, open drag, insert ghost, other users).
#[derive(Clone, Debug, Default)]
pub struct ViewportView {
    pub config: Viewport3dWindowConfig,
    pub playback: PlaybackState,
    pub lanes: LaneOverlays,
    pub lane: ViewLane,
}

impl ViewportView {
    /// 🎞️ The view with the live lane folded in: the lane's colour field and speed replace the configured ones, and a lane track that the scene has becomes the transport state at its snapped value; an unknown or absent track keeps the explicit playback.
    pub fn resolved(&self, scene: &StructuralScene) -> Self {
        let mut config = self.config.clone();
        config.active_field = self.lane.active_field.clone().or(config.active_field);
        config.playback_speed = self.lane.speed;
        let playback = match self.lane.track.as_deref().and_then(|id| playback::find(&scene.tracks, id)) {
            Some(track) => {
                let selected = PlaybackState { track: Some(track.id.clone()), playing: self.lane.playing, clock: self.lane.clock, ..PlaybackState::default() };
                let at = playback::reduce(&selected, &scene.tracks, self.lane.speed, &playback::PlaybackCommand::Seek(self.lane.value.unwrap_or(track.min)));
                PlaybackState { playing: self.lane.playing, clock: self.lane.clock, ..at }
            }
            None => self.playback.clone(),
        };
        Self { config, playback, ..self.clone() }
    }
}

/// 🌐️ The text of a localized copy in the shell locale.
pub fn text_of(copy: &LocalizedCopy, locale: Locale) -> String {
    if locale == Locale::De { copy.de.clone() } else { copy.en.clone() }
}

fn mark(on: bool) -> &'static str {
    if on { "●" } else { "○" }
}

pub const VIEWPORT_ACT_TOOL: &str = "viewportAct";

fn row(controller_id: &'static str, id: String, label: String, action: &str, text: Option<&str>, number: Option<f64>) -> UiAssemblyResult<BuiltNode> {
    let mut args = vec![("action", ui_value_text(action)?)];
    if let Some(text) = text { args.push(("text", ui_value_text(text)?)); }
    if let Some(number) = number { args.push(("number", UiValue::Number(number))); }
    tree_item_with_action(id, norm_ui_label(label)?, None, norm_action(controller_id, VIEWPORT_ACT_TOOL, Some(action_args_map(args)?))?)
}

/// ➕ The armed tool of the insert palette: the spec of the ghost that follows the cursor.
fn armed_spec(view: &ViewportView) -> Option<&str> {
    match view.lanes.frame.transient.ghost.as_ref().map(|ghost| &ghost.source) {
        Some(crate::GhostSource::Insert { spec, .. }) => Some(spec.as_str()),
        _ => None,
    }
}

/// ➕ One row per insert tool (the armed one marked) and a row that disarms the tool; only live editors have a palette.
fn insert_rows(view: &ViewportView, locale: Locale, controller_id: &'static str) -> UiAssemblyResult<Vec<BuiltNode>> {
    if view.lanes.palette.is_empty() {
        return Ok(Vec::new());
    }
    let armed = armed_spec(view);
    let mut rows = Vec::new();
    for entry in &view.lanes.palette {
        let label = if locale == Locale::De { &entry.label_de } else { &entry.label_en };
        let args = action_args_map(vec![("insertSpec", ui_value_text(&entry.id)?)])?;
        rows.push(tree_item_with_action(format!("norm-viewport.insert.{}", entry.id), norm_ui_label(format!("{} {} {label}", mark(armed == Some(entry.id.as_str())), chrome("Insert", "Einfügen", locale)))?, None, norm_action(controller_id, "previewRemedy", Some(args))?)?);
    }
    rows.push(tree_item_with_action("norm-viewport.insert.off".to_string(), norm_ui_label(format!("{} {}", mark(armed.is_none()), chrome("Select", "Auswählen", locale)))?, None, norm_action(controller_id, "clearPreview", None)?)?);
    Ok(rows)
}

/// 🕹️ The gesture bindings of the world surface of a live editor: pointer phases (`delta` streams, `commit` releases, `abort` cancels) drive the handle drag through `setFields`; an armed insert tool also binds hover (ghost at the snapped cursor) and drop (one placement). The host merges the pick id (`handle`), the drag parameter (`s`, `origin`), the modifiers and the cursor (`at`) under the static arguments.
fn gesture_bindings(view: &ViewportView, controller_id: &'static str) -> UiAssemblyResult<Vec<ui::ActionBinding>> {
    let mut bindings = Vec::new();
    let mut bind = |trigger: ui::Trigger, action: &str, args: Vec<(&str, UiValue)>| -> UiAssemblyResult<()> {
        let (action, args) = norm_action(controller_id, action, Some(action_args_map(args)?))?;
        bindings.push(ui::ActionBinding { trigger, action, args, capability: None });
        Ok(())
    };
    for (trigger, phase) in [(ui::Trigger::Delta, "stream"), (ui::Trigger::Commit, "commit"), (ui::Trigger::Abort, "abort")] {
        bind(trigger, "setFields", vec![("phase", ui_value_text(phase)?)])?;
    }
    if let Some(spec) = armed_spec(view) {
        bind(ui::Trigger::HoverPreview, "previewRemedy", vec![("insertSpec", ui_value_text(spec)?)])?;
        bind(ui::Trigger::Drop, "setFields", vec![("placeSpec", ui_value_text(spec)?)])?;
    }
    Ok(bindings)
}

fn field_rows(scene: &StructuralScene, config: &Viewport3dWindowConfig, locale: Locale, controller_id: &'static str) -> UiAssemblyResult<Vec<BuiltNode>> {
    let choices = fields::available(scene);
    if choices.is_empty() {
        return Ok(Vec::new());
    }
    let active = config.active_field.as_deref();
    let mut rows = vec![row(controller_id, "norm-viewport.field.utilisation".into(), format!("{} {}", mark(active.is_none()), chrome("Utilisation", "Ausnutzung", locale)), ACTION_SET_ACTIVE_FIELD, Some(""), None)?];
    for choice in choices {
        rows.push(row(controller_id, format!("norm-viewport.field.{}", choice.id), format!("{} {}", mark(active == Some(choice.id.as_str())), text_of(&choice.title, locale)), ACTION_SET_ACTIVE_FIELD, Some(choice.id.as_str()), None)?);
    }
    Ok(rows)
}

fn playback_rows(scene: &StructuralScene, config: &Viewport3dWindowConfig, state: &PlaybackState, locale: Locale, controller_id: &'static str) -> UiAssemblyResult<Vec<BuiltNode>> {
    if scene.tracks.is_empty() {
        return Ok(Vec::new());
    }
    let mut rows = vec![row(controller_id, "norm-viewport.track.none".into(), format!("{} {}", mark(state.track.is_none()), chrome("Static view", "Statische Ansicht", locale)), ACTION_SELECT_TRACK, Some(""), None)?];
    for track in &scene.tracks {
        rows.push(row(controller_id, format!("norm-viewport.track.{}", track.id), format!("{} {}", mark(state.track.as_deref() == Some(track.id.as_str())), text_of(&track.label, locale)), ACTION_SELECT_TRACK, Some(track.id.as_str()), None)?);
    }
    let Some(track) = state.track.as_deref().and_then(|id| playback::find(&scene.tracks, id)) else { return Ok(rows) };
    let readout = text_of(&playback::readout(track, state.value), locale);
    let transport = if state.playing { chrome("Pause", "Anhalten", locale) } else { chrome("Play", "Abspielen", locale) };
    rows.push(row(controller_id, "norm-viewport.play".into(), format!("{transport} · {readout}"), ACTION_TOGGLE_PLAY, None, None)?);
    for (id, en, de, direction) in [("back", "Step back", "Schritt zurück", -1), ("forward", "Step forward", "Schritt vor", 1)] {
        let target = playback::reduce(state, &scene.tracks, config.playback_speed, &playback::PlaybackCommand::Step(direction)).value;
        rows.push(row(controller_id, format!("norm-viewport.step-{id}"), chrome(en, de, locale), ACTION_STEP, None, Some(target))?);
    }
    for (n, key) in track.keyframes.iter().enumerate() {
        let current = (key.at - state.value).abs() < track.step / 2.0;
        rows.push(row(controller_id, format!("norm-viewport.seek.{n}"), format!("{} {}", mark(current), text_of(&key.caption, locale)), ACTION_SEEK, None, Some(key.at))?);
    }
    for speed in PLAYBACK_SPEEDS {
        rows.push(row(controller_id, format!("norm-viewport.speed.{speed}"), format!("{} {} ×{}", mark((config.playback_speed - speed).abs() < 1e-9), chrome("Speed", "Geschwindigkeit", locale), speed), ACTION_SET_SPEED, None, Some(speed))?);
    }
    Ok(rows)
}

/// 🧰️ The toolbar tree: a field section (utilisation plus every scene field) and a playback section (static view, tracks, transport, keyframes, speeds); a section with nothing to choose is omitted, and the whole toolbar is `None` for a static scene without fields.
pub fn render_toolbar(scene: &StructuralScene, view: &ViewportView, locale: Locale, controller_id: &'static str) -> UiAssemblyResult<Option<BuiltNode>> {
    let fields = field_rows(scene, &view.config, locale, controller_id)?;
    let playback = playback_rows(scene, &view.config, &view.playback, locale, controller_id)?;
    let insert = insert_rows(view, locale, controller_id)?;
    if fields.is_empty() && playback.is_empty() && insert.is_empty() {
        return Ok(None);
    }
    let mut tree = PanelTreeBuilder::new(VIEWPORT_TOOLBAR_ID)?;
    if !fields.is_empty() {
        tree = tree.section(VIEWPORT_FIELD_SECTION_ID, Some(norm_ui_label(chrome("Field", "Feld", locale))?), true, ui_node_list(fields.into_iter().map(Ok))?)?;
    }
    if !playback.is_empty() {
        tree = tree.section(VIEWPORT_PLAYBACK_SECTION_ID, Some(norm_ui_label(chrome("Playback", "Wiedergabe", locale))?), true, ui_node_list(playback.into_iter().map(Ok))?)?;
    }
    if !insert.is_empty() {
        tree = tree.section(VIEWPORT_INSERT_SECTION_ID, Some(norm_ui_label(chrome("Insert", "Einfügen", locale))?), true, ui_node_list(insert.into_iter().map(Ok))?)?;
    }
    tree.build().map(Some)
}

/// ✋️ The id of the touch tree of the viewport panel: the Edit menu and the nudge buttons.
pub const VIEWPORT_TOUCH_ID: &str = "norm-viewport-touch";

/// ✋️ The nudge buttons of the grips of the selection, what tap-to-nudge devices edit a handle with: per grip a minus and a plus row that nudge the bound field by one schema step (Shift and Alt scale are the keyboard's). A row carries the flat gesture that commits the nudged parameter in one `setFields` (the pointer drag's commit, freed from snapping so a step never snaps back), and says the current value as its description; a grip at its limit gets a row that says so. `focused` narrows the list to one grip.
pub fn nudge_rows(handles: &[Handle], focused: Option<&str>, locale: Locale, controller_id: &'static str) -> UiAssemblyResult<Vec<BuiltNode>> {
    use crate::accessibility::published_nodes::rest_parameter;
    use crate::accessibility::screen_reader::handle_value_text;
    use crate::direct_manipulation::{nudge, Modifiers};
    let lang = Lang::of(locale);
    let mut rows = Vec::new();
    for handle in handles.iter().filter(|handle| focused.map_or(true, |id| id == handle.id)) {
        let at = rest_parameter(handle);
        let name = text_of(&handle.accessible_name, locale);
        let value = handle_value_text(handle, at, lang);
        for (id, sign, direction) in [("minus", "\u{2212}", -1.0), ("plus", "+", 1.0)] {
            let to = nudge(handle, at, direction, Modifiers::NONE).s;
            let row_id = format!("norm-viewport.nudge.{}.{id}", handle.id);
            let label = norm_ui_label(format!("{sign} {name}"))?;
            rows.push(if (to - at).abs() > f64::EPSILON {
                let args = action_args_map(vec![("handle", ui_value_text(&handle.id)?), ("s", UiValue::Number(to)), ("origin", UiValue::Number(at)), ("phase", ui_value_text("commit")?), ("ctrl", UiValue::Bool(true))])?;
                tree_item_with_action(row_id, label, Some(value.clone()), norm_action(controller_id, "setFields", Some(args))?)?
            } else {
                tree_item_desc(row_id, label, Some(chrome("At the limit", "Am Grenzwert", locale)))?
            });
        }
    }
    Ok(rows)
}

/// 🖐️ The touch controls of the viewport panel of a live editor: the Edit menu while the view lane holds it open ([`render_touch_menu`] over the entries the published selection, clip and governing check allow, with a row that closes it; its accessibility nodes are published for the screen reader; the shell hook closes it on an outside tap or Escape) and, on a device whose profile asks for nudge buttons, the nudge rows of the selected grips. The tree is a touch tree: every row is at least 44 px tall. `None` when neither applies.
pub fn render_touch_controls(view: &ViewportView, device: Option<crate::viewport3d::config::DeviceClass>, domain_id: &str, locale: Locale, controller_id: &'static str) -> UiAssemblyResult<Option<BuiltNode>> {
    let nudges = match device {
        Some(class) if crate::device_profiles::device_profile(class).nudge_buttons => nudge_rows(&view.lanes.handles, view.lanes.focused.as_deref(), locale, controller_id)?,
        _ => Vec::new(),
    };
    crate::note_touch_menu(domain_id, view.lane.menu);
    if !view.lane.menu && nudges.is_empty() {
        return Ok(None);
    }
    let mut tree = PanelTreeBuilder::new(VIEWPORT_TOUCH_ID)?.presentation(semio_ui_contract::TreePresentation::Touch);
    if view.lane.menu {
        let context = crate::host_context();
        let entries = crate::touch_menu(context.ids.len(), context.clip_json.is_some(), crate::host_focus());
        crate::publish_touch_menu_access(domain_id, &entries, Lang::of(locale));
        let close = row(controller_id, "norm-viewport.touch.close".into(), chrome("Close menu", "Menü schließen", locale), VIEW_ACTION_MENU, None, Some(0.0))?;
        tree = tree.section(format!("{VIEWPORT_TOUCH_ID}.edit"), None, true, ui_node_list([crate::render_touch_menu(&entries, controller_id, locale)?, close].into_iter().map(Ok))?)?;
    }
    if !nudges.is_empty() {
        tree = tree.section(format!("{VIEWPORT_TOUCH_ID}.nudge"), Some(norm_ui_label(chrome("Handles", "Griffe", locale))?), true, ui_node_list(nudges.into_iter().map(Ok))?)?;
    }
    tree.build().map(Some)
}

/// ⌨️ The id of the keyboard help section of the viewport panel.
pub const VIEWPORT_HELP_SECTION_ID: &str = "norm-viewport-help";

/// ⌨️ The keyboard help overlay: a closed section listing every documented shortcut with its scope, its label in the shell language and its chord (`⌘⇧Z` style on macOS), generated from the one shortcut table so a new shortcut cannot be missing.
pub fn render_shortcut_help(locale: Locale, mac: bool) -> UiAssemblyResult<BuiltNode> {
    use crate::accessibility::focus_order::{chord_text, shortcuts, Scope};
    let all = shortcuts();
    let mut groups = Vec::new();
    for (scope, id, en, de) in [(Scope::Global, "global", "Everywhere", "Überall"), (Scope::Viewport, "viewport", "3D model", "3D-Modell"), (Scope::Handle, "handle", "Handle", "Griff")] {
        let rows = all.iter().enumerate().filter(|(_, shortcut)| shortcut.scope == scope).map(|(n, shortcut)| tree_item_desc(format!("norm-viewport.help.{n}"), norm_ui_label(text_of(&shortcut.label, locale))?, Some(chord_text(shortcut.chord, mac))));
        groups.push(tree_group(format!("{VIEWPORT_HELP_SECTION_ID}.{id}"), norm_ui_label(chrome(en, de, locale))?, true, ui_node_list(rows)?)?);
    }
    PanelTreeBuilder::new(VIEWPORT_HELP_SECTION_ID)?.section(format!("{VIEWPORT_HELP_SECTION_ID}.list"), Some(norm_ui_label(chrome("Keyboard shortcuts", "Tastenkürzel", locale))?), false, ui_node_list(groups.into_iter().map(Ok))?)?.build()
}

/// 🎥️ The lowered viewport of a resolved view with the live lanes applied; while the view lane holds a check (a click or a stepping shortcut selected it) or the frame toggle (a double tap) the camera frames the selected entities in the configured preset direction instead of the whole scene.
pub fn lowered_viewport(scene: &StructuralScene, view: &ViewportView, selected: &[String], lang: Lang) -> crate::viewport3d::lowering::LoweredViewport {
    let measure = MeasureState::default();
    let inputs = ViewportInputs { config: &view.config, selected, hovered: None, measure: &measure, lang };
    let mut lowered = apply(lower_dynamic(scene, &inputs, &view.playback, &BasicTessellator::default()), scene, &view.lanes);
    if view.lane.check.is_some() || view.lane.frame {
        let ids: Vec<String> = selected.iter().filter_map(|address| match crate::viewport3d::picking::resolve(scene, address) { Some(crate::viewport3d::picking::PickTarget::Entity { entity, .. }) => Some(entity.id), _ => None }).collect();
        lowered.camera = crate::viewport3d::camera::fit_selection(scene, &ids, view.config.preset);
    }
    lowered
}

/// 🧊️ The viewport panel of a window: lowers the scene at the transport state and stacks the toolbar above the `world-3d` surface.
pub fn render_viewport_panel(body_key: &str, scene: &StructuralScene, view: &ViewportView, selected: &[String], domain_id: &str, controller_id: &'static str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let lang = Lang::of(locale);
    let view = &view.resolved(scene);
    publish_viewport_access(domain_id, scene, selected, &view.lanes.handles, lang);
    let lowered = lowered_viewport(scene, view, selected, lang);
    let mut surface = render_viewport3d(body_key, &lowered, domain_id, view.config.sun)?;
    if crate::controller_is_live(controller_id) && controller_id.ends_with("#editor") {
        for binding in gesture_bindings(view, controller_id)? {
            surface.bindings.try_push(binding).map_err(|_| ui_error("ui.node.binding"))?;
        }
    }
    let mut children: Vec<BuiltNode> = render_toolbar(scene, view, locale, controller_id)?.into_iter().collect();
    if crate::controller_is_live(controller_id) && controller_id.ends_with("#editor") {
        children.extend(render_touch_controls(view, crate::device_profiles::current_device(), domain_id, locale, controller_id)?);
    }
    children.push(render_shortcut_help(locale, cfg!(target_os = "macos"))?);
    children.push(surface);
    ui::column().try_children(children).map_err(|_| ui_error("ui.node.children"))?.try_build().map_err(|_| ui_error("ui.node.build"))
}
