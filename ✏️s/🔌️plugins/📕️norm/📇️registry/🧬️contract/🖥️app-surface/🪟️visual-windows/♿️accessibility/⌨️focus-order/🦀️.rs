//! ⌨️ Focus order and keyboard shortcuts of the visual windows: outliner, viewport, properties, checks, diagrams; focus is never trapped.

use crate::viewport3d::config::DeviceClass;
use semio_s_plugin_norm_structural_scene::LocalizedCopy;

/// 🪟️ A focusable window region; ids equal the window roles of the layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Region {
    Outliner,
    Viewport,
    Properties,
    Checks,
    Diagrams,
}

pub const DESKTOP_ORDER: [Region; 5] = [Region::Outliner, Region::Viewport, Region::Properties, Region::Checks, Region::Diagrams];
pub const MOBILE_ORDER: [Region; 5] = [Region::Viewport, Region::Outliner, Region::Properties, Region::Checks, Region::Diagrams];

impl Region {
    pub fn role(self) -> &'static str {
        match self {
            Self::Outliner => "outliner",
            Self::Viewport => "viewport",
            Self::Properties => "properties",
            Self::Checks => "checks",
            Self::Diagrams => "diagrams",
        }
    }

    pub fn name(self) -> LocalizedCopy {
        match self {
            Self::Outliner => LocalizedCopy::new("Model outline", "Modellstruktur"),
            Self::Viewport => LocalizedCopy::new("3D model", "3D-Modell"),
            Self::Properties => LocalizedCopy::new("Properties", "Eigenschaften"),
            Self::Checks => LocalizedCopy::new("Checks", "Nachweise"),
            Self::Diagrams => LocalizedCopy::new("Diagrams", "Diagramme"),
        }
    }
}

/// 🔢️ Tab order of a device class; mobile and tablet keep the model first and reach the outliner as a tab (the list-based selection surface of a touch device), the tablet side pane tabs follow the same order.
pub fn focus_order(device: DeviceClass) -> &'static [Region] {
    match device {
        DeviceClass::Mobile | DeviceClass::Tablet => &MOBILE_ORDER,
        DeviceClass::Desktop => &DESKTOP_ORDER,
    }
}

/// ➡️ Where Tab or Shift+Tab goes: the neighbouring region, or out of the visual windows so focus is never trapped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusMove {
    To(Region),
    Leave { forward: bool },
}

pub fn next(device: DeviceClass, current: Region, forward: bool) -> FocusMove {
    let order = focus_order(device);
    let Some(i) = order.iter().position(|r| *r == current) else { return FocusMove::To(order[0]) };
    match (forward, i) {
        (true, i) if i + 1 < order.len() => FocusMove::To(order[i + 1]),
        (false, i) if i > 0 => FocusMove::To(order[i - 1]),
        _ => FocusMove::Leave { forward },
    }
}

/// ⌨️ A key chord; `ctrl` is the primary modifier (Command on macOS).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chord {
    pub key: &'static str,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
}

const fn chord(key: &'static str, ctrl: bool, alt: bool, shift: bool) -> Chord {
    Chord { key, ctrl, alt, shift }
}

/// 📏️ Step class of a handle nudge; Alt is fine, Shift is large, matching the nudge multipliers of the handle drag tool (x0.1, x1, x10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NudgeStep {
    Fine,
    Normal,
    Large,
}

impl NudgeStep {
    pub fn scale(self) -> f64 {
        match self {
            Self::Fine => 0.1,
            Self::Normal => 1.0,
            Self::Large => 10.0,
        }
    }
}

/// 🎯️ Where a shortcut is active: everywhere, with focus in the viewport, or with a handle focused. A chord may repeat across the viewport and handle scopes, never with the global scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    Global,
    Viewport,
    Handle,
}

/// 🧭️ What a shortcut does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShortcutAction {
    Focus(Region),
    CycleRegion { forward: bool },
    CycleSelection { forward: bool },
    EnterHandles,
    OrbitLeft,
    OrbitRight,
    OrbitUp,
    OrbitDown,
    PanLeft,
    PanRight,
    PanUp,
    PanDown,
    ZoomIn,
    ZoomOut,
    ResetView,
    NudgeHandle { dx: i8, dy: i8, step: NudgeStep },
    CycleHandle { forward: bool },
    CycleGripInEntity { forward: bool },
    ConfirmHandle,
    CancelHandle,
    ToggleSceneAsList,
    NextCheck,
    PreviousCheck,
    NextGoverningCheck,
    PreviousGoverningCheck,
    NextCombination,
    PreviousCombination,
    Undo,
    Redo,
    Copy,
    Paste,
    Duplicate,
    Delete,
    SelectAll,
    SearchCatalogue,
    ClearSelection,
    Help,
}

/// 📋️ One documented shortcut.
#[derive(Clone, Debug, PartialEq)]
pub struct Shortcut {
    pub scope: Scope,
    pub chord: Chord,
    pub action: ShortcutAction,
    pub label: LocalizedCopy,
}

fn s(scope: Scope, chord: Chord, action: ShortcutAction, en: &str, de: &str) -> Shortcut {
    Shortcut { scope, chord, action, label: LocalizedCopy::new(en, de) }
}

fn nudges() -> Vec<Shortcut> {
    let directions = [("ArrowLeft", -1, 0, "left", "links"), ("ArrowRight", 1, 0, "right", "rechts"), ("ArrowUp", 0, 1, "up", "oben"), ("ArrowDown", 0, -1, "down", "unten")];
    let steps = [(NudgeStep::Fine, false, true, " by a fine step", "einen feinen Schritt"), (NudgeStep::Normal, false, false, "", ""), (NudgeStep::Large, true, false, " by a large step", "einen großen Schritt")];
    directions
        .iter()
        .flat_map(|(key, dx, dy, en, de)| {
            steps.iter().map(move |(step, shift, alt, en_step, de_step)| {
                let de_text = if de_step.is_empty() { format!("Griff nach {de} schieben") } else { format!("Griff um {de_step} nach {de} schieben") };
                s(Scope::Handle, chord(*key, false, *alt, *shift), ShortcutAction::NudgeHandle { dx: *dx, dy: *dy, step: *step }, &format!("Nudge handle {en}{en_step}"), &de_text)
            })
        })
        .collect()
}

/// 📋️ Every documented shortcut; none collides with browser or screen-reader chords (Ctrl+W/T/N, Tab, Insert-key chords). With a handle focused the arrows nudge it (Shift large, Alt fine), otherwise they orbit; Tab and Shift+Tab walk the handles of the selection, Ctrl+Arrow (Command+Arrow on macOS) cycles the handles of the entity of the focused one.
pub fn shortcuts() -> Vec<Shortcut> {
    use Scope::{Global, Handle, Viewport};
    use ShortcutAction::*;
    let mut all = vec![
        s(Global, chord("1", false, true, false), Focus(Region::Outliner), "Focus model outline", "Modellstruktur fokussieren"),
        s(Global, chord("2", false, true, false), Focus(Region::Viewport), "Focus 3D model", "3D-Modell fokussieren"),
        s(Global, chord("3", false, true, false), Focus(Region::Properties), "Focus properties", "Eigenschaften fokussieren"),
        s(Global, chord("4", false, true, false), Focus(Region::Checks), "Focus checks", "Nachweise fokussieren"),
        s(Global, chord("5", false, true, false), Focus(Region::Diagrams), "Focus diagrams", "Diagramme fokussieren"),
        s(Global, chord("F6", false, false, false), CycleRegion { forward: true }, "Next window", "Nächstes Fenster"),
        s(Global, chord("F6", false, false, true), CycleRegion { forward: false }, "Previous window", "Vorheriges Fenster"),
        s(Global, chord("z", true, false, false), Undo, "Undo", "Rückgängig"),
        s(Global, chord("z", true, false, true), Redo, "Redo", "Wiederholen"),
        s(Global, chord("y", true, false, false), Redo, "Redo", "Wiederholen"),
        s(Global, chord("c", true, false, false), Copy, "Copy selected entities", "Ausgewählte Bauteile kopieren"),
        s(Global, chord("v", true, false, false), Paste, "Paste entities", "Bauteile einfügen"),
        s(Global, chord("d", true, false, false), Duplicate, "Duplicate selected entities", "Ausgewählte Bauteile duplizieren"),
        s(Global, chord("Delete", false, false, false), Delete, "Delete selected entities", "Ausgewählte Bauteile löschen"),
        s(Viewport, chord("a", true, false, false), SelectAll, "Select all entities", "Alle Bauteile auswählen"),
        s(Global, chord("k", true, false, false), SearchCatalogue, "Search catalogue", "Katalog durchsuchen"),
        s(Global, chord("/", false, false, false), SearchCatalogue, "Search catalogue", "Katalog durchsuchen"),
        s(Global, chord("]", false, false, false), NextCheck, "Next check", "Nächster Nachweis"),
        s(Global, chord("[", false, false, false), PreviousCheck, "Previous check", "Vorheriger Nachweis"),
        s(Global, chord("g", false, true, false), NextGoverningCheck, "Next governing check", "Nächster maßgebender Nachweis"),
        s(Global, chord("g", false, true, true), PreviousGoverningCheck, "Previous governing check", "Vorheriger maßgebender Nachweis"),
        s(Global, chord("l", false, true, false), ToggleSceneAsList, "Toggle scene as list", "Szene als Liste umschalten"),
        s(Global, chord("?", false, false, true), Help, "Show keyboard help", "Tastaturhilfe anzeigen"),
        s(Viewport, chord("PageDown", false, false, false), CycleSelection { forward: true }, "Select next entity", "Nächstes Bauteil auswählen"),
        s(Viewport, chord("PageUp", false, false, false), CycleSelection { forward: false }, "Select previous entity", "Vorheriges Bauteil auswählen"),
        s(Viewport, chord("Enter", false, false, false), EnterHandles, "Edit the handles of the selection", "Griffe der Auswahl bearbeiten"),
        s(Viewport, chord("ArrowLeft", false, false, false), OrbitLeft, "Orbit left", "Nach links orbitieren"),
        s(Viewport, chord("ArrowRight", false, false, false), OrbitRight, "Orbit right", "Nach rechts orbitieren"),
        s(Viewport, chord("ArrowUp", false, false, false), OrbitUp, "Orbit up", "Nach oben orbitieren"),
        s(Viewport, chord("ArrowDown", false, false, false), OrbitDown, "Orbit down", "Nach unten orbitieren"),
        s(Viewport, chord("ArrowLeft", false, true, false), PanLeft, "Pan left", "Nach links verschieben"),
        s(Viewport, chord("ArrowRight", false, true, false), PanRight, "Pan right", "Nach rechts verschieben"),
        s(Viewport, chord("ArrowUp", false, true, false), PanUp, "Pan up", "Nach oben verschieben"),
        s(Viewport, chord("ArrowDown", false, true, false), PanDown, "Pan down", "Nach unten verschieben"),
        s(Viewport, chord("+", false, false, false), ZoomIn, "Zoom in", "Vergrößern"),
        s(Viewport, chord("-", false, false, false), ZoomOut, "Zoom out", "Verkleinern"),
        s(Viewport, chord("0", false, false, false), ResetView, "Reset view", "Ansicht zurücksetzen"),
        s(Viewport, chord(".", false, false, false), NextCombination, "Next combination", "Nächste Kombination"),
        s(Viewport, chord(",", false, false, false), PreviousCombination, "Previous combination", "Vorherige Kombination"),
        s(Viewport, chord("Escape", false, false, false), ClearSelection, "Clear selection", "Auswahl aufheben"),
        s(Handle, chord("Tab", false, false, false), CycleHandle { forward: true }, "Next handle", "Nächster Griff"),
        s(Handle, chord("Tab", false, false, true), CycleHandle { forward: false }, "Previous handle", "Vorheriger Griff"),
        s(Handle, chord("ArrowRight", true, false, false), CycleGripInEntity { forward: true }, "Next handle of the same entity", "Nächster Griff desselben Bauteils"),
        s(Handle, chord("ArrowDown", true, false, false), CycleGripInEntity { forward: true }, "Next handle of the same entity", "Nächster Griff desselben Bauteils"),
        s(Handle, chord("ArrowLeft", true, false, false), CycleGripInEntity { forward: false }, "Previous handle of the same entity", "Vorheriger Griff desselben Bauteils"),
        s(Handle, chord("ArrowUp", true, false, false), CycleGripInEntity { forward: false }, "Previous handle of the same entity", "Vorheriger Griff desselben Bauteils"),
        s(Handle, chord("Enter", false, false, false), ConfirmHandle, "Open the value field of the handle, or apply the typed value", "Wertfeld des Griffs öffnen oder eingegebenen Wert übernehmen"),
        s(Handle, chord("Escape", false, false, false), CancelHandle, "Cancel the handle change", "Griffänderung abbrechen"),
    ];
    all.extend(nudges());
    all
}

/// 🔤️ Kebab case of the words of a debug text: `CycleRegion { forward: true }` is `cycle-region-forward-true`.
pub fn kebab(text: &str) -> String {
    let mut out = String::new();
    for token in text.split(|c: char| !c.is_alphanumeric()).filter(|token| !token.is_empty()) {
        for c in token.chars() {
            if c.is_uppercase() && !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            out.extend(c.to_lowercase());
        }
        out.push('-');
    }
    out.trim_end_matches('-').to_owned()
}

/// 🆔️ The id of the app action a shortcut runs: handle nudges are `norm.handle.arrow-left.fine`, the handle walk `norm.handle.tab.forward`, everything else `norm.shortcut.<kebab action>`; the viewport window declares exactly these actions.
pub fn shortcut_action_id(action: &ShortcutAction) -> String {
    match action {
        ShortcutAction::NudgeHandle { dx, dy, step } => {
            let direction = if *dx > 0 { "arrow-right" } else if *dx < 0 { "arrow-left" } else if *dy > 0 { "arrow-up" } else { "arrow-down" };
            format!("norm.handle.{direction}.{}", match step { NudgeStep::Fine => "fine", NudgeStep::Normal => "normal", NudgeStep::Large => "large" })
        }
        ShortcutAction::CycleHandle { forward } => format!("norm.handle.tab.{}", if *forward { "forward" } else { "backward" }),
        ShortcutAction::CycleGripInEntity { forward } => format!("norm.handle.cycle-grip.{}", if *forward { "forward" } else { "backward" }),
        ShortcutAction::ConfirmHandle => "norm.handle.enter.normal".to_owned(),
        ShortcutAction::CancelHandle => "norm.handle.escape.normal".to_owned(),
        other => format!("norm.shortcut.{}", kebab(&format!("{other:?}"))),
    }
}

/// 🔎️ The action bound to a chord in a scope; global shortcuts apply in every scope.
pub fn lookup(scope: Scope, chord: Chord) -> Option<ShortcutAction> {
    shortcuts().into_iter().find(|s| s.chord == chord && (s.scope == scope || s.scope == Scope::Global)).map(|s| s.action)
}

/// 🏷️ Printable chord for the help overlay: `Ctrl+Shift+Z`, or `⌘⇧Z` with the Command key on macOS.
pub fn chord_text(chord: Chord, mac: bool) -> String {
    let key = if chord.key.chars().count() == 1 { chord.key.to_uppercase() } else { chord.key.to_owned() };
    let (ctrl, alt, shift) = if mac { ("⌘", "⌥", "⇧") } else { ("Ctrl+", "Alt+", "Shift+") };
    format!("{}{}{}{key}", if chord.ctrl { ctrl } else { "" }, if chord.alt { alt } else { "" }, if chord.shift { shift } else { "" })
}

/// 🔄️ The entity a selection-cycling key moves to, wrapping around; with no or an unknown current id it starts at the first (forward) or last (backward) entity.
pub fn cycle_selection(ids: &[String], current: Option<&str>, forward: bool) -> Option<String> {
    let len = ids.len();
    let at = current.and_then(|c| ids.iter().position(|id| id == c));
    (len > 0).then(|| ids[match (at, forward) {
        (Some(i), true) => (i + 1) % len,
        (Some(i), false) => (i + len - 1) % len,
        (None, true) => 0,
        (None, false) => len - 1,
    }].clone())
}

/// 🔢️ The index a stepping key moves to among `count` items, wrapping around; with no current index it starts at the first (forward) or last (backward) item. Used for `view.activeCombination` (the combinations or variants of the active scene).
pub fn step_index(count: usize, current: Option<usize>, forward: bool) -> Option<usize> {
    (count > 0).then(|| match (current.filter(|i| *i < count), forward) {
        (Some(i), true) => (i + 1) % count,
        (Some(i), false) => (i + count - 1) % count,
        (None, true) => 0,
        (None, false) => count - 1,
    })
}

/// 🎯️ The check a governing-check key moves to: checks are walked from the highest utilisation down, forward goes to the next lower one and wraps; `None` utilisations (not applicable) come last. Returns the index into `utilisations`; with no current check it starts at the governing (forward) or the least utilised (backward) one.
pub fn step_governing(utilisations: &[Option<f64>], current: Option<usize>, forward: bool) -> Option<usize> {
    let mut order: Vec<usize> = (0..utilisations.len()).collect();
    order.sort_by(|a, b| match (utilisations[*a], utilisations[*b]) {
        (Some(x), Some(y)) => y.total_cmp(&x),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });
    let at = current.and_then(|c| order.iter().position(|i| *i == c));
    step_index(order.len(), at, forward).map(|i| order[i])
}
