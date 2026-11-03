//! 📐️ Layout preferences: defaults, sanitising, validation, user commands and the mobile bottom sheet; the tree itself is built by [`visual_layout_with`](crate::visual_layout_with).
//!
//! 🔗️ Persisted as a local-only preference under [`STORAGE_KEY`]; see `visual-editor-architecture.md` §8.3 and §8.4.

use super::*;

pub const STORAGE_KEY: &str = "norm.visual.layout";
pub const PERSISTENCE: &str = "persistedLocalOnly";
pub const ROLES: [&str; 5] = ["outliner", "viewport", "properties", "checks", "diagrams"];
pub const MOBILE_ROLES: [&str; 5] = ["viewport", "outliner", "properties", "checks", "diagrams"];
pub const SIDE_ROLES: [&str; 4] = ["outliner", "properties", "checks", "diagrams"];
pub const COLUMN_RANGE: (f64, f64) = (0.08, 0.75);
pub const SPLIT_RANGE: (f64, f64) = (0.2, 0.8);
pub const TABLET_VIEWPORT_RANGE: (f64, f64) = (0.3, 0.8);
pub const SHEET_RANGE: (f64, f64) = (0.25, 0.9);
pub const SHEET_PEEK: f64 = 0.12;
pub const SHEET_FULL: f64 = 0.92;

fn owned(roles: &[&str]) -> Vec<String> {
    roles.iter().map(|r| (*r).to_owned()).collect()
}

impl Default for DockPrefs {
    fn default() -> Self {
        Self { left: 0.22, centre: 0.52, right: 0.26, left_split: 0.5, right_split: 0.5, hidden: Vec::new() }
    }
}

impl Default for TabletPrefs {
    fn default() -> Self {
        Self { viewport: 0.6, side_tabs: owned(&SIDE_ROLES), side_active: "properties".into() }
    }
}

impl Default for MobilePrefs {
    fn default() -> Self {
        Self { tabs: owned(&MOBILE_ROLES), active: "viewport".into(), sheet: SheetState::Collapsed, sheet_height: 0.5 }
    }
}

impl Default for LayoutPrefs {
    fn default() -> Self {
        Self { dock: DockPrefs::default(), tablet: TabletPrefs::default(), mobile: MobilePrefs::default() }
    }
}

fn within(value: f64, (lo, hi): (f64, f64)) -> bool {
    value.is_finite() && (lo..=hi).contains(&value)
}

fn clamped(value: f64, (lo, hi): (f64, f64), fallback: f64) -> f64 {
    if value.is_finite() { value.clamp(lo, hi) } else { fallback }
}

fn permutation(tabs: &[String], all: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for tab in tabs.iter().filter(|t| all.contains(&t.as_str())) {
        if !out.contains(tab) {
            out.push(tab.clone());
        }
    }
    let missing: Vec<String> = all.iter().filter(|r| !out.iter().any(|o| o.as_str() == **r)).map(|r| (*r).to_owned()).collect();
    out.extend(missing);
    out
}

fn is_permutation(tabs: &[String], all: &[&str]) -> bool {
    tabs.len() == all.len() && all.iter().all(|r| tabs.iter().any(|t| t.as_str() == *r))
}

/// 🧼 Coerces any stored preference into a valid one: shares clamped, tab orders completed and deduplicated, unknown, duplicate and viewport entries dropped from the hidden list, an unknown active tab replaced by the first.
pub fn sanitised(prefs: &LayoutPrefs) -> LayoutPrefs {
    let d = LayoutPrefs::default();
    let mut hidden: Vec<String> = Vec::new();
    for role in prefs.dock.hidden.iter().filter(|r| ROLES.contains(&r.as_str()) && r.as_str() != "viewport") {
        if !hidden.contains(role) {
            hidden.push(role.clone());
        }
    }
    let side_tabs = permutation(&prefs.tablet.side_tabs, &SIDE_ROLES);
    let tabs = permutation(&prefs.mobile.tabs, &MOBILE_ROLES);
    LayoutPrefs {
        dock: DockPrefs {
            left: clamped(prefs.dock.left, COLUMN_RANGE, d.dock.left),
            centre: clamped(prefs.dock.centre, COLUMN_RANGE, d.dock.centre),
            right: clamped(prefs.dock.right, COLUMN_RANGE, d.dock.right),
            left_split: clamped(prefs.dock.left_split, SPLIT_RANGE, 0.5),
            right_split: clamped(prefs.dock.right_split, SPLIT_RANGE, 0.5),
            hidden,
        },
        tablet: TabletPrefs {
            viewport: clamped(prefs.tablet.viewport, TABLET_VIEWPORT_RANGE, d.tablet.viewport),
            side_active: if side_tabs.contains(&prefs.tablet.side_active) { prefs.tablet.side_active.clone() } else { side_tabs[0].clone() },
            side_tabs,
        },
        mobile: MobilePrefs {
            active: if tabs.contains(&prefs.mobile.active) { prefs.mobile.active.clone() } else { tabs[0].clone() },
            tabs,
            sheet: prefs.mobile.sheet,
            sheet_height: clamped(prefs.mobile.sheet_height, SHEET_RANGE, d.mobile.sheet_height),
        },
    }
}

/// 🧪️ Structural rule violations as `path: rule` lines; empty when the preference can be applied unchanged.
pub fn validate_layout(prefs: &LayoutPrefs) -> Vec<String> {
    let mut out = Vec::new();
    let mut rule = |path: &str, ok: bool, text: &str| if !ok { out.push(format!("{path}: {text}")) };
    let (dock, tablet, mobile) = (&prefs.dock, &prefs.tablet, &prefs.mobile);
    for (path, value) in [("dock.left", dock.left), ("dock.centre", dock.centre), ("dock.right", dock.right)] {
        rule(path, within(value, COLUMN_RANGE), "column-share-out-of-range");
    }
    for (path, value) in [("dock.leftSplit", dock.left_split), ("dock.rightSplit", dock.right_split)] {
        rule(path, within(value, SPLIT_RANGE), "split-share-out-of-range");
    }
    rule("dock.hidden", dock.hidden.iter().enumerate().all(|(i, r)| ROLES.contains(&r.as_str()) && r != "viewport" && !dock.hidden[..i].contains(r)), "hidden-window-unknown-or-duplicate");
    rule("tablet.viewport", within(tablet.viewport, TABLET_VIEWPORT_RANGE), "viewport-share-out-of-range");
    rule("tablet.sideTabs", is_permutation(&tablet.side_tabs, &SIDE_ROLES), "side-tabs-not-a-permutation");
    rule("tablet.sideActive", tablet.side_tabs.contains(&tablet.side_active), "side-active-not-a-tab");
    rule("mobile.tabs", is_permutation(&mobile.tabs, &MOBILE_ROLES), "tabs-not-a-permutation");
    rule("mobile.active", mobile.tabs.contains(&mobile.active), "active-not-a-tab");
    rule("mobile.sheetHeight", within(mobile.sheet_height, SHEET_RANGE), "sheet-height-out-of-range");
    out
}

/// 🕹️ A user change of the layout; the result of [`apply_layout`] is persisted as the new local-only preference.
#[derive(Clone, Debug, PartialEq)]
pub enum LayoutCommand {
    Reset,
    ResizeColumns { left: f64, centre: f64, right: f64 },
    Split { left_column: bool, share: f64 },
    SetHidden { role: String, hidden: bool },
    ViewportShare(f64),
    MoveTab { mobile: bool, role: String, to: usize },
    Activate { mobile: bool, role: String },
    Sheet(SheetState),
    SheetHeight(f64),
}

/// 🧩️ The sanitised preference after one user command.
pub fn apply_layout(prefs: &LayoutPrefs, command: &LayoutCommand) -> LayoutPrefs {
    let mut next = sanitised(prefs);
    match command {
        LayoutCommand::Reset => next = LayoutPrefs::default(),
        LayoutCommand::ResizeColumns { left, centre, right } => (next.dock.left, next.dock.centre, next.dock.right) = (*left, *centre, *right),
        LayoutCommand::Split { left_column: true, share } => next.dock.left_split = *share,
        LayoutCommand::Split { left_column: false, share } => next.dock.right_split = *share,
        LayoutCommand::SetHidden { role, hidden } => {
            next.dock.hidden.retain(|r| r != role);
            if *hidden {
                next.dock.hidden.push(role.clone());
            }
        }
        LayoutCommand::ViewportShare(share) => next.tablet.viewport = *share,
        LayoutCommand::MoveTab { mobile, role, to } => {
            let tabs = if *mobile { &mut next.mobile.tabs } else { &mut next.tablet.side_tabs };
            if let Some(from) = tabs.iter().position(|t| t == role) {
                let tab = tabs.remove(from);
                tabs.insert((*to).min(tabs.len()), tab);
            }
        }
        LayoutCommand::Activate { mobile: true, role } => next.mobile.active = role.clone(),
        LayoutCommand::Activate { mobile: false, role } => next.tablet.side_active = role.clone(),
        LayoutCommand::Sheet(state) => next.mobile.sheet = *state,
        LayoutCommand::SheetHeight(height) => next.mobile.sheet_height = *height,
    }
    sanitised(&next)
}

/// 🪟️ The bottom sheet that shows the properties over the model.
#[derive(Clone, Debug, PartialEq)]
pub struct SheetSpec {
    pub role: &'static str,
    pub state: SheetState,
    pub height: f64,
    pub min_handle_px: u32,
}

/// 🪟️ The properties bottom sheet: mobile only, over the model tab, and only while something is selected; collapsed it peeks, half uses the preferred height, full nearly fills the screen.
pub fn bottom_sheet(device: DeviceClass, prefs: &LayoutPrefs, has_selection: bool) -> Option<SheetSpec> {
    let prefs = sanitised(prefs);
    (device == DeviceClass::Mobile && has_selection && prefs.mobile.active == "viewport").then(|| SheetSpec {
        role: "properties",
        state: prefs.mobile.sheet,
        height: match prefs.mobile.sheet {
            SheetState::Collapsed => SHEET_PEEK,
            SheetState::Half => prefs.mobile.sheet_height,
            SheetState::Full => SHEET_FULL,
        },
        min_handle_px: device_profile(device).min_target_px,
    })
}

/// 📡️ Everything the windows adapt to for one device signal.
#[derive(Clone, Debug, PartialEq)]
pub struct DeviceView {
    pub class: DeviceClass,
    pub layout_id: String,
    pub profile: DeviceProfileSpec,
    pub sheet: Option<SheetSpec>,
}

/// 📡️ The device class, the id of its named layout, its profile and the bottom sheet for a host signal; a shell calls this on every resize and switches to `layout_id`.
pub fn device_view(signal: &DeviceSignal, prefs: &LayoutPrefs, has_selection: bool) -> DeviceView {
    let class = classify(signal.width_px, signal.touch);
    DeviceView { class, layout_id: crate::visual_layout_id(class), profile: device_profile(class), sheet: bottom_sheet(class, prefs, has_selection) }
}
