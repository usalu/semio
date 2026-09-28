#!/usr/bin/env python3
"""🗂️ S18 prepared patch (window 3): the Rust twin of the event-sourced named layouts (session 14 item 4).

React commits a saved layout as ONE `setNamedLayout { appId, layoutId, layout | null }` event in the
`os.config.ui-preferences` log (persistedShared). Rust does not know the event: the wgpu shell's log decode is
all-or-nothing, so ONE React-saved layout made it read an EMPTY log — and its next preference write replaced the
stored log with its own events, losing every React-written preference (theme, locale, … and the layouts).

This patch, idempotent (`--dry-run` reports, applies nothing):
  R1 `UserNamedLayout` + `UiPreferences.named_layouts` (config facet, schema `🎨️ui-preferences/🔣️.json`);
  R2 leaf `🧬️mutations/🗂️set-named-layout/` (authority, payload schema, TS re-export, Rust leaf, two fixture
     quintets + laws, the sibling leaves' shape);
  R3/R4 the leaf joins the cohesive ui-preferences module and the `UiPreferencesConfigMutation` aggregate;
  R5 every committed UiPreferences snapshot gains `"namedLayouts": {}` (canonical-JSON laws);
  R6 the wgpu shell projects user layouts from the replayed preferences and saves them as `setNamedLayout`
     events (no more flat `namedLayouts` projection in the `semio.os.config` document — React dropped it);
     its laws follow.
New directory → R10 registers `🗂️set-named-layout` in the taxonomy in window 3.
usage: [S18_PATCH_ROOT=<overlay>] python3 s18-named-layout-rust-twin.py [--dry-run]
"""
import json
import os
import pathlib
import sys

ROOT = pathlib.Path(os.environ.get("S18_PATCH_ROOT", "/Users/ueli/Documents/semio"))
CONFIG = ROOT / "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema"
MUTATIONS = CONFIG / "🧬️mutations"
LEAF = MUTATIONS / "🗂️set-named-layout"
SHELL = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell"
WGPU = SHELL / "🎯️targets/🧊️wgpu/🦀️.rs"
DRY = "--dry-run" in sys.argv
problems: list[str] = []
changes: list[str] = []


def edit(path: pathlib.Path, old: str, new: str) -> None:
    text = path.read_text(encoding="utf-8")
    if (old in new and new in text) or (old not in new and old not in text and new in text):
        return
    if text.count(old) != 1:
        problems.append(f"{path.relative_to(ROOT)}: anchor found {text.count(old)}× — {old[:90]!r}")
        return
    changes.append(f"edit {path.relative_to(ROOT)}: {old.splitlines()[0][:80]!r}")
    if not DRY:
        path.write_text(text.replace(old, new), encoding="utf-8")


def create(path: pathlib.Path, content: str) -> None:
    if path.exists() and path.read_text(encoding="utf-8") == content:
        return
    if path.exists():
        problems.append(f"{path.relative_to(ROOT)} exists with other content")
        return
    changes.append(f"create {path.relative_to(ROOT)}")
    if not DRY:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")


def dump(value: object) -> str:
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


# R1 — the config facet
edit(
    CONFIG / "🦀️.rs",
    """pub struct UiTheme {
    pub theme_id: String,
    pub label: String,
    #[value(with = "json_value_bridge")]
    pub config: JsonValue,
}
""",
    """pub struct UiTheme {
    pub theme_id: String,
    pub label: String,
    #[value(with = "json_value_bridge")]
    pub config: JsonValue,
}

/// 🗂️ One window layout the user saved — its label and the arrangement it restores; the layout id is its key in
/// [`UiPreferences::named_layouts`]. The arrangement stays the schema's `WindowLayout` JSON here: the renderer that
/// restores it owns the typed form.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, schemars::JsonSchema, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UserNamedLayout {
    pub label: String,
    #[value(with = "json_value_bridge")]
    pub layout: JsonValue,
}
""",
)
edit(
    CONFIG / "🦀️.rs",
    """    pub keybinding_overrides: HashMap<String, String>,
}
""",
    """    pub keybinding_overrides: HashMap<String, String>,
    pub named_layouts: HashMap<String, HashMap<String, UserNamedLayout>>,
}
""",
)

# R2 — the leaf
create(
    LEAF / "🦀️.rs",
    """//! 🗂️ Saves, replaces or removes one of the user's named window layouts of one app.
use super::super::super::{UiPreferences, UiPreferencesDiff, UserNamedLayout};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::HashMap;
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetNamedLayout {
    pub app_id: String,
    pub layout_id: String,
    pub layout: Option<UserNamedLayout>,
}
pub fn set_named_layout(app_id: impl Into<String>, layout_id: impl Into<String>, layout: Option<UserNamedLayout>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetNamedLayout(SetNamedLayout { app_id: app_id.into(), layout_id: layout_id.into(), layout })
}

impl SetNamedLayout {
    fn saved<'a>(&self, base: &'a UiPreferences) -> Option<&'a UserNamedLayout> {
        base.named_layouts.get(&self.app_id).and_then(|layouts| layouts.get(&self.layout_id))
    }
}

impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for SetNamedLayout {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "named-layout", kind: "set-named-layout", record: "Set" };
    fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
        if self.saved(base) == self.layout.as_ref() {
            return protocol::MutationOutcome::new(UiPreferencesDiff(base.clone())).warn("mutation.no-op", "named layout already has the requested value.");
        }
        let mut next = base.clone();
        let mut app_layouts: HashMap<String, UserNamedLayout> = next.named_layouts.remove(&self.app_id).unwrap_or_default();
        match &self.layout {
            Some(layout) => {
                app_layouts.insert(self.layout_id.clone(), layout.clone());
            }
            None => {
                app_layouts.remove(&self.layout_id);
            }
        }
        if !app_layouts.is_empty() {
            next.named_layouts.insert(self.app_id.clone(), app_layouts);
        }
        protocol::MutationOutcome::new(UiPreferencesDiff(next))
    }
    fn inverse(&self, base: &UiPreferences) -> Vec<UiPreferencesConfigMutation> {
        vec![set_named_layout(self.app_id.clone(), self.layout_id.clone(), self.saved(base).cloned())]
    }
    fn label(&self) -> protocol::LocalizedLabel {
        match self.layout {
            Some(_) => protocol::LocalizedLabel::native(&format!("Save layout {:?}", self.layout_id), &format!("Layout {:?} speichern", self.layout_id)),
            None => protocol::LocalizedLabel::native(&format!("Remove layout {:?}", self.layout_id), &format!("Layout {:?} entfernen", self.layout_id)),
        }
    }
    fn target(&self) -> Vec<String> {
        vec!["named-layouts".to_string(), self.app_id.clone(), self.layout_id.clone()]
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-named-layout/🦀️.rs"]
mod tests_sets_named_layout;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-named-layout/🦀️.rs"]
mod tests_keeps_named_layout;
//#endregion 🧪️Tests
""",
)
create(LEAF / "🟦️.ts", 'export { setNamedLayout } from "../🟦️.ts";\nexport type { SetNamedLayout } from "../🟦️.ts";\n')
authority = json.loads((MUTATIONS / "⌨️set-keybinding-override/🔣️.json").read_text(encoding="utf-8"))
authority.update({"owner": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🗂️set-named-layout", "semanticKind": "set-named-layout", "displayName": "Set Named Layout", "emoji": "🗂️", "aggregateVariant": "SetNamedLayout"})
create(LEAF / "🔣️.json", json.dumps(authority, ensure_ascii=False, separators=(",", ":")) + "\n")
aggregate = json.loads((MUTATIONS / "🎨️ui-preferences/🧬️schema/🔣️.json").read_text(encoding="utf-8"))
defs = aggregate["$defs"]
payload = json.loads(json.dumps(defs["SetNamedLayout"]))
payload["required"] = [key for key in payload["required"] if key != "mutation"]
payload["properties"].pop("mutation", None)
payload_schema = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "https://json.schemas.assets.semio-tech.com/os/config/mutation/set-named-layout/schema.json",
    "title": "SetNamedLayout",
    **payload,
    "definitions": {name: defs[name] for name in ["UserNamedLayout", "WindowLayout", "WindowLayoutAxisNode", "WindowLayoutStackNode", "WindowLayoutWindowNode"]},
}
create(LEAF / "🧬️schema/🔣️.json", json.dumps(payload_schema, ensure_ascii=False).replace("#/$defs/", "#/definitions/") + "\n")

EMPTY = {"appearance": None, "layout": None, "driverId": None, "customDrivers": {}, "locale": None, "terminology": None, "themeId": None, "customThemes": {}, "keybindingOverrides": {}, "namedLayouts": {}}
WIDE = {"label": "Wide", "layout": {"root": {"kind": "stack", "children": [{"kind": "window", "windowKindId": "main"}]}}}
SAVED = {**EMPTY, "namedLayouts": {"draw": {"user-1": WIDE}}}
for case, before, mutation, after, outcome in [
    ("✏️sets-named-layout", EMPTY, {"mutation": "setNamedLayout", "appId": "draw", "layoutId": "user-1", "layout": WIDE}, SAVED, {"status": "applied"}),
    ("🟰️keeps-named-layout", SAVED, {"mutation": "setNamedLayout", "appId": "draw", "layoutId": "user-1", "layout": WIDE}, SAVED, {"status": "no-op", "messages": [{"level": "warning", "code": "mutation.no-op"}]}),
]:
    base = LEAF / "🧫️fixtures" / case
    create(base / "📸️snapshot/⬅️before/🔣️.json", dump(before))
    create(base / "📸️snapshot/➡️after/🔣️.json", dump(after))
    create(base / "🔺️diff/🔣️.json", dump(after))
    create(base / "🦠️mutation/🔣️.json", dump(mutation))
    create(base / "🎯️outcome/🔣️.json", dump(outcome))
for sibling, ours, doc_old, doc_new in [
    ("✏️sets-keybinding", "✏️sets-named-layout", "Setting the undo keybinding on untouched preferences changes exactly that preference and nothing else.", "Saving a layout for `draw` on untouched preferences files it under the app and the layout id and changes nothing else."),
    ("🟰️keeps-keybinding", "🟰️keeps-named-layout", None, None),
]:
    text = (MUTATIONS / "⌨️set-keybinding-override/🧪️tests" / sibling / "🦀️.rs").read_text(encoding="utf-8")
    text = text.replace(sibling, ours).replace("set-keybinding-override", "set-named-layout").replace(sibling.split("️", 1)[1], ours.split("️", 1)[1])
    text = text.replace("the undo keybinding", "the saved layout").replace("undo keybinding", "saved layout").replace("keybinding", "named layout")
    if doc_old:
        text = text.replace(doc_old, doc_new)
    create(LEAF / "🧪️tests" / ours / "🦀️.rs", text)

# R3/R4 — the cohesive module and the aggregate
edit(
    MUTATIONS / "🎨️ui-preferences/🦀️.rs",
    """#[path = "../🗣️set-locale/🦀️.rs"]
pub mod set_locale;
""",
    """#[path = "../🗣️set-locale/🦀️.rs"]
pub mod set_locale;
#[path = "../🗂️set-named-layout/🦀️.rs"]
pub mod set_named_layout;
""",
)
edit(MUTATIONS / "🎨️ui-preferences/🦀️.rs", "pub use set_locale::*;\n", "pub use set_locale::*;\npub use set_named_layout::*;\n")
edit(
    MUTATIONS / "🦀️.rs",
    "pub use super::ui_preferences::{\n    set_appearance, set_custom_driver, set_custom_theme, set_driver, set_keybinding_override, set_layout, set_locale, set_terminology, set_theme,",
    "pub use super::ui_preferences::{\n    set_appearance, set_custom_driver, set_custom_theme, set_driver, set_keybinding_override, set_layout, set_locale, set_named_layout, set_terminology, set_theme, SetNamedLayout,",
)
edit(MUTATIONS / "🦀️.rs", "    SetKeybindingOverride(SetKeybindingOverride),\n}\n//#endregion 🔖️UiPreferences", "    SetKeybindingOverride(SetKeybindingOverride),\n    SetNamedLayout(SetNamedLayout),\n}\n//#endregion 🔖️UiPreferences")

# R5 — committed UiPreferences snapshots gain the new field
for path in sorted(MUTATIONS.glob("*/🧫️fixtures/*/**/🔣️.json")):
    if path.is_relative_to(LEAF):
        continue
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict) or "keybindingOverrides" not in value or "namedLayouts" in value:
        continue
    migrated = {}
    for key, entry in value.items():
        migrated[key] = entry
        if key == "keybindingOverrides":
            migrated["namedLayouts"] = {}
    changes.append(f"fixture {path.relative_to(ROOT)}")
    if not DRY:
        path.write_text(dump(migrated), encoding="utf-8")

# R6 — the wgpu shell
edit(
    WGPU,
    "    mutations::{set_appearance, set_custom_driver, set_custom_theme, set_driver, set_keybinding_override, set_layout, set_locale, set_terminology, set_theme, UiPreferencesConfigMutation},\n    UiAppearance as OsUiAppearance, UiChromeLayout as OsUiChromeLayout, UiDriver as OsUiDriver, UiLocale as OsUiLocale, UiPreferences, UiTheme as OsUiTheme, UI_PREFERENCES_CONFIG_SCHEMA,",
    "    mutations::{set_appearance, set_custom_driver, set_custom_theme, set_driver, set_keybinding_override, set_layout, set_locale, set_named_layout, set_terminology, set_theme, UiPreferencesConfigMutation},\n    UiAppearance as OsUiAppearance, UiChromeLayout as OsUiChromeLayout, UiDriver as OsUiDriver, UiLocale as OsUiLocale, UiPreferences, UiTheme as OsUiTheme, UserNamedLayout, UI_PREFERENCES_CONFIG_SCHEMA,",
)
old_block_start = "/// 🖥️ The key React's `NamedLayoutStore` files a saved layout under:"
old_block_end = """fn save_named_layouts_to_store(app_id: Option<&str>, layouts: &[ui_wgpu::wgpu::NamedLayout]) {
    let mut config = os_shell_config_document();
    write_named_layouts_in(&mut config, app_id, layouts);
    write_os_shell_config_document(&config);
}
"""
wgpu_text = WGPU.read_text(encoding="utf-8")
if old_block_start in wgpu_text and old_block_end in wgpu_text:
    start = wgpu_text.index(old_block_start)
    end = wgpu_text.index(old_block_end) + len(old_block_end)
    edit(
        WGPU,
        wgpu_text[start:end],
        """/// 🖥️ The app a saved layout is filed under — React's Display host `focusedApp?.id ?? "framework-os"`
/// (`🏛️ShellHost/🟦️.tsx`, `displayLayoutAppId`).
pub(crate) const NAMED_LAYOUT_STORE_FALLBACK_APP_ID: &str = "framework-os";

fn named_layout_store_app_id(app_id: Option<&str>) -> String {
    app_id.filter(|id| !id.is_empty()).unwrap_or(NAMED_LAYOUT_STORE_FALLBACK_APP_ID).to_string()
}

/// 🖥️ This app's USER layouts, projected from the replayed `os.config.ui-preferences` preferences — React's
/// `displayUserLayouts`: `namedLayouts[appId]` sorted by layout id, each `origin: "user"`. A saved arrangement this
/// renderer cannot type is left out rather than failing the roster.
fn user_named_layouts(preferences: &UiPreferences, app_id: Option<&str>) -> Vec<ui_wgpu::wgpu::NamedLayout> {
    let mut layouts: Vec<ui_wgpu::wgpu::NamedLayout> = preferences
        .named_layouts
        .get(&named_layout_store_app_id(app_id))
        .map(|saved| {
            saved
                .iter()
                .filter_map(|(id, entry)| Some(ui_wgpu::wgpu::NamedLayout { id: id.clone(), label: entry.label.clone(), icon_id: None, layout: serde_json::from_value(entry.layout.clone()).ok()?, origin: "user".to_string(), group_path: None }))
                .collect()
        })
        .unwrap_or_default();
    layouts.sort_by(|left, right| left.id.cmp(&right.id));
    layouts
}

/// 🖥️ The `setNamedLayout` events that turn `preferences`' saved layouts of this app into `layouts` — one per
/// added, changed or removed user layout, none for an unchanged roster and never one for a builtin (React's
/// `onSaveUserLayout`/`onDeleteUserLayout`, one event per gesture).
fn named_layout_events(preferences: &UiPreferences, app_id: Option<&str>, layouts: &[ui_wgpu::wgpu::NamedLayout]) -> Vec<UiPreferencesConfigMutation> {
    let key = named_layout_store_app_id(app_id);
    let before = preferences.named_layouts.get(&key).cloned().unwrap_or_default();
    let after: HashMap<String, UserNamedLayout> = layouts
        .iter()
        .filter(|layout| layout.origin == "user")
        .filter_map(|layout| Some((layout.id.clone(), UserNamedLayout { label: layout.label.clone(), layout: serde_json::to_value(&layout.layout).ok()? })))
        .collect();
    let ids: std::collections::BTreeSet<&String> = before.keys().chain(after.keys()).collect();
    ids.into_iter().filter(|id| before.get(*id) != after.get(*id)).map(|id| set_named_layout(key.clone(), id.clone(), after.get(id).cloned())).collect()
}

/// 🖥️ The persisted user layouts for `app_id`, off the replayed preference log.
fn load_named_layouts_from_store(app_id: Option<&str>) -> Vec<ui_wgpu::wgpu::NamedLayout> {
    user_named_layouts(&read_ui_preferences(), app_id)
}

/// 🖥️ Appends the events that make `layouts` this app's saved roster to the preference log.
fn save_named_layouts_to_store(app_id: Option<&str>, layouts: &[ui_wgpu::wgpu::NamedLayout]) {
    commit_ui_preferences_write(|log| {
        let events = named_layout_events(&replay_ui_preferences(log), app_id, layouts);
        log.events.extend(events);
    });
}
""",
    )
elif "fn named_layout_events(" not in wgpu_text:
    problems.append("wgpu: named-layout helper block anchors not found")
edit(WGPU, '        "preferences": {},\n        "namedLayouts": {},\n        "dockLayouts": { "apps": {} },', '        "preferences": {},\n        "dockLayouts": { "apps": {} },')
edit(WGPU, "/// back — so a sibling projection (`dockLayouts`, `dockUi`, `namedLayouts`, `windowPanes`) is never", "/// back — so a sibling projection (`dockLayouts`, `dockUi`, `windowPanes`) is never")
edit(
    WGPU,
    """    /// @emoji 🖥️ Layouts this session saved through `framework.display.save` — React's `userLayouts`,
    /// the `origin: "user"` half of its `namedLayouts` roster, round-tripped through
    /// [`ShellState::load_persisted_named_layouts`]/[`ShellState::persist_named_layouts`] (the
    /// `NamedLayoutStore` twin). The app's own `named_layouts` supply the `origin: "builtin"` half.""",
    """    /// @emoji 🖥️ Layouts the user saved through `framework.display.save` — React's `displayUserLayouts`, the
    /// `origin: "user"` half of the roster, projected from the `os.config.ui-preferences` log and written back as
    /// `setNamedLayout` events ([`ShellState::load_persisted_named_layouts`]/[`ShellState::persist_named_layouts`]).
    /// The app's own `named_layouts` supply the `origin: "builtin"` half.""",
)
edit(
    WGPU,
    """    /// 🖥️ Restores this app's SAVED layouts from the shared `semio.os.config` document — React's
    /// `NamedLayoutStore` constructor, which reads its persisted rows the moment the store is built
    /// (`🏛️ShellHost/🟦️.tsx:3507`, re-memoized on every `session.app.id` change, exactly as this is
    /// re-run from [`Self::load_persisted_dock`] on every app change).
    ///
    /// 🩸️ What this replaces: NOTHING read `namedLayouts`. W5a made the whole `semio.os.config`
    /// document round-trip through the storage door — a layout saved in the React shell survived a
    /// switch to wgpu on disk — but no wgpu lane consumed the projection, so a saved layout was
    /// preserved and ignored (`📓️w5a-browser-prefs-persistence.md` §7.2).""",
    """    /// 🖥️ Restores this app's SAVED layouts from the replayed `os.config.ui-preferences` log — React's Display
    /// host projection, re-run from [`Self::load_persisted_dock`] on every app change as React re-memoizes it on
    /// every focused-app change.""",
)
edit(
    WGPU,
    """    /// 🖥️ Writes this app's saved layouts back — `NamedLayoutStore::save`/`remove`, both of which end
    /// in one whole-array rewrite. Called by whatever mutates [`ShellState::user_layouts`].""",
    """    /// 🖥️ Commits this app's saved-layout roster as `setNamedLayout` events — React's `onSaveUserLayout`/
    /// `onDeleteUserLayout`. Called by whatever mutates [`ShellState::user_layouts`].""",
)
door = SHELL / "🧪️tests/🚪️wgpu-host-door-remainder/🦀️.rs"
door_text = door.read_text(encoding="utf-8")
law_start = "/// ⚖️ LAW (item 7 / audit row 21): a SAVED layout round-trips through the one `semio.os.config`"
law_end = """    assert!(mixed["dockLayouts"]["apps"].is_object() && mixed["windowPanes"]["apps"].is_object(), "🗄️ the sibling projections survive: {mixed}");
}
"""
if law_start in door_text and law_end in door_text:
    edit(
        door,
        door_text[door_text.index(law_start) : door_text.index(law_end) + len(law_end)],
        """/// ⚖️ LAW (item 7 / audit row 21, event-sourced since ticket 26/09/23 S18): a SAVED layout is one `setNamedLayout`
/// event in the `os.config.ui-preferences` log — filed under the app React's Display host files it under
/// (`"framework-os"` without a session), projected back sorted by id with `origin: "user"`, never written for a
/// builtin — and the event React commits decodes here, so one React-saved layout can no longer empty this
/// renderer's whole preference log.
#[test]
fn saved_named_layouts_are_ui_preference_events() {
    let layout = ui_wgpu::wgpu::create_stack_layout(&["main".to_string()], None);
    let user = ui_wgpu::wgpu::NamedLayout { id: "user-1".into(), label: "Mine".into(), icon_id: None, layout: layout.clone(), origin: "user".into(), group_path: None };
    let builtin = ui_wgpu::wgpu::NamedLayout { id: "builtin-1".into(), label: "Theirs".into(), icon_id: None, layout: layout.clone(), origin: "builtin".into(), group_path: None };

    let events = named_layout_events(&UiPreferences::default(), Some("puzzle3d"), &[user.clone(), builtin]);
    assert_eq!(events.len(), 1, "🖥️ one event per saved user layout, none for a builtin: {events:?}");
    let mut preferences = UiPreferences::default();
    for event in &events {
        apply_ui_preferences_config_mutation(&mut preferences, event).expect("a saved layout applies");
    }
    assert_eq!(user_named_layouts(&preferences, Some("puzzle3d")), vec![user.clone()], "and projects back whole");
    assert!(user_named_layouts(&preferences, Some("other-app")).is_empty(), "another app's layouts are another app's");
    assert!(named_layout_events(&preferences, Some("puzzle3d"), std::slice::from_ref(&user)).is_empty(), "🖥️ an unchanged roster appends nothing");
    for event in named_layout_events(&preferences, Some("puzzle3d"), &[]) {
        apply_ui_preferences_config_mutation(&mut preferences, &event).expect("a removal applies");
    }
    assert!(user_named_layouts(&preferences, Some("puzzle3d")).is_empty(), "🖥️ removing the last layout removes it");

    let sessionless = named_layout_events(&UiPreferences::default(), None, std::slice::from_ref(&user));
    assert!(matches!(sessionless.as_slice(), [UiPreferencesConfigMutation::SetNamedLayout(set)] if set.app_id == NAMED_LAYOUT_STORE_FALLBACK_APP_ID), "🖥️ the sessionless layer is React's `framework-os`: {sessionless:?}");

    let react_event = serde_json::json!({ "mutation": "setNamedLayout", "appId": "draw", "layoutId": "user-7", "layout": { "label": "Wide", "layout": serde_json::to_value(&layout).expect("layout json") } });
    let decoded = decode_ui_preferences_config_mutation_json(&react_event.to_string()).expect("🗄️ the event React commits decodes in this renderer");
    let mut from_react = UiPreferences::default();
    apply_ui_preferences_config_mutation(&mut from_react, &decoded).expect("and applies");
    assert_eq!(user_named_layouts(&from_react, Some("draw")).iter().map(|entry| entry.label.as_str()).collect::<Vec<_>>(), vec!["Wide"], "🗄️ a React-saved layout is this renderer's user layout");
}
""",
    )
elif "fn saved_named_layouts_are_ui_preference_events" not in door_text:
    problems.append("door law anchors not found")
edit(
    door,
    """//! 4. `namedLayouts` round-trips through the `semio.os.config` document React's `NamedLayoutStore`
//!    owns, with React's own `origin: "user"` filter; `windowPanes` stays consumer-less on both sides
//!    (row 21);""",
    """//! 4. a saved layout is a `setNamedLayout` event of the `os.config.ui-preferences` log both renderers replay,
//!    with React's own `origin: "user"` projection; `windowPanes` stays consumer-less on both sides (row 21);""",
)
edit(SHELL / "🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs", '            "namedLayouts": { "draw": [{ "id": "wide" }] },\n            "dockLayouts": { "apps": {} },', '            "dockLayouts": { "apps": { "draw": { "version": 1 } } },')
edit(SHELL / "🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs", '    assert_eq!(config["dockLayouts"]["apps"], serde_json::json!({}));\n    assert_eq!(config["namedLayouts"]["draw"][0]["id"], "wide", "preference writes must preserve sibling projections");', '    assert_eq!(config["dockLayouts"]["apps"]["draw"]["version"], 1, "preference writes must preserve sibling projections");')
edit(SHELL / "🧪️tests/🗄️browser-prefs-persistence/🦀️.rs", '        "namedLayouts": {},\n        "dockLayouts": { "apps": {} },', '        "dockLayouts": { "apps": {} },')
edit(SHELL / "🧪️tests/🗄️browser-prefs-persistence/🦀️.rs", '    for projection in ["namedLayouts", "dockLayouts", "dockUi", "windowPanes"] {', '    for projection in ["dockLayouts", "dockUi", "windowPanes"] {')

# R7 — an unreadable stored log is a typed refusal that blocks writes, never an empty log that a write replaces
edit(
    WGPU,
    """fn decode_ui_preferences_event_log(raw: &str) -> Option<UiPreferencesEventLog> {
    let value: Value = serde_json::from_str(raw).ok()?;
    if value.get("version").and_then(Value::as_u64) != Some(1) {
        return None;
    }
    let events = value.get("events")?.as_array()?.iter().map(|event| decode_ui_preferences_config_mutation_json(&event.to_string())).collect::<Result<Vec<_>, _>>().ok()?;
    Some(UiPreferencesEventLog { version: 1, events })
}
""",
    """/// 🚫️ A stored preference log this renderer cannot read whole — unparsable, another version, or carrying an event
/// it does not know (one a newer renderer wrote). Typed, never read as an empty log: it refuses every write, since a
/// rewrite from this renderer's own events would destroy what it could not read, while reads replay `readable`, the
/// events it did decode.
#[derive(Clone, Debug)]
struct UiPreferencesLogRefusal {
    reason: &'static str,
    event_index: Option<usize>,
    readable: UiPreferencesEventLog,
}

fn decode_ui_preferences_event_log(raw: &str) -> Result<UiPreferencesEventLog, UiPreferencesLogRefusal> {
    let refuse = |reason: &'static str, event_index: Option<usize>, events: Vec<UiPreferencesConfigMutation>| UiPreferencesLogRefusal { reason, event_index, readable: UiPreferencesEventLog { version: 1, events } };
    let Ok(value) = serde_json::from_str::<Value>(raw) else {
        return Err(refuse("json", None, Vec::new()));
    };
    if value.get("version").and_then(Value::as_u64) != Some(1) {
        return Err(refuse("version", None, Vec::new()));
    }
    let Some(stored) = value.get("events").and_then(Value::as_array) else {
        return Err(refuse("events", None, Vec::new()));
    };
    let mut events = Vec::with_capacity(stored.len());
    let mut undecodable = None;
    for (index, event) in stored.iter().enumerate() {
        match decode_ui_preferences_config_mutation_json(&event.to_string()) {
            Ok(mutation) => events.push(mutation),
            Err(_) => {
                undecodable.get_or_insert(index);
            }
        }
    }
    match undecodable {
        None => Ok(UiPreferencesEventLog { version: 1, events }),
        Some(index) => Err(refuse("event", Some(index), events)),
    }
}
""",
)
edit(
    WGPU,
    """fn read_ui_preferences_event_log() -> UiPreferencesEventLog {
    prefs_get(UI_PREFERENCES_CONFIG_SCHEMA).and_then(|raw| decode_ui_preferences_event_log(&raw)).unwrap_or_else(|| UiPreferencesEventLog { version: 1, events: Vec::new() })
}
""",
    """/// ✍️ The ONE write rule over a stored preference log: an absent log is an empty one, a stored one is decoded
/// whole or the write refuses (the stored text is never replaced by a log this renderer could not read); `append`
/// adds this write's events, and a write that added none stores nothing.
fn ui_preferences_log_write(stored: Option<&str>, append: impl FnOnce(&mut UiPreferencesEventLog)) -> Result<Option<String>, UiPreferencesLogRefusal> {
    let mut log = match stored {
        None => UiPreferencesEventLog { version: 1, events: Vec::new() },
        Some(raw) => decode_ui_preferences_event_log(raw)?,
    };
    let before = log.events.len();
    append(&mut log);
    Ok((log.events.len() > before).then(|| encode_ui_preferences_event_log(&log)))
}

/// 💾️ Commits one preference write through [`ui_preferences_log_write`]: stored, or one refusal line and the stored
/// log left exactly as it was.
fn commit_ui_preferences_write(append: impl FnOnce(&mut UiPreferencesEventLog)) {
    match ui_preferences_log_write(prefs_get(UI_PREFERENCES_CONFIG_SCHEMA).as_deref(), append) {
        Ok(Some(encoded)) => prefs_set(UI_PREFERENCES_CONFIG_SCHEMA, &encoded),
        Ok(None) => {}
        Err(refusal) => ShellState::debug_log(&format!("[wgpu-shell] preference write refused: the stored log is unreadable ({}{}) and stays untouched", refusal.reason, refusal.event_index.map(|index| format!(" at event {index}")).unwrap_or_default())),
    }
}

/// 📖️ The stored log for READING: an unreadable one still answers the events this renderer decoded.
fn read_ui_preferences_event_log() -> UiPreferencesEventLog {
    match prefs_get(UI_PREFERENCES_CONFIG_SCHEMA) {
        None => UiPreferencesEventLog { version: 1, events: Vec::new() },
        Some(raw) => decode_ui_preferences_event_log(&raw).unwrap_or_else(|refusal| refusal.readable),
    }
}
""",
)
persist_head = "fn persist_ui_preferences(state: &ShellState, previous: Option<&UiPrefsSnapshot>) {\n    let mut log = read_ui_preferences_event_log();\n"
persist_tail = "    prefs_set(UI_PREFERENCES_CONFIG_SCHEMA, &encode_ui_preferences_event_log(&log));\n}\n\nimpl ShellState {\n    /// 💾️ Loads persisted uiPrefs"
wgpu_now = WGPU.read_text(encoding="utf-8")
if persist_head in wgpu_now and persist_tail in wgpu_now:
    body_start = wgpu_now.index(persist_head) + len(persist_head)
    body_end = wgpu_now.index(persist_tail, body_start)
    body = wgpu_now[body_start:body_end]
    indented = "".join(("    " + line) if line.strip() else line for line in body.splitlines(keepends=True))
    edit(
        WGPU,
        persist_head + body + persist_tail,
        "fn persist_ui_preferences(state: &ShellState, previous: Option<&UiPrefsSnapshot>) {\n    commit_ui_preferences_write(|log| {\n" + indented + "    });\n}\n\nimpl ShellState {\n    /// 💾️ Loads persisted uiPrefs",
    )
elif "    commit_ui_preferences_write(|log| {\n        let mut log" not in wgpu_now and "fn persist_ui_preferences(state: &ShellState, previous: Option<&UiPrefsSnapshot>) {\n    commit_ui_preferences_write(|log| {" not in wgpu_now:
    problems.append("wgpu: persist_ui_preferences anchors not found")
prefs_test = SHELL / "🧪️tests/🗄️browser-prefs-persistence/🦀️.rs"
prefs_text = prefs_test.read_text(encoding="utf-8")
law = """
/// ⚖️ LAW (ticket 26/09/23 S18 §14b): a stored preference log this renderer cannot read whole — here one event a
/// newer renderer wrote — is a typed refusal, never an empty log: the write refuses and the stored text stays
/// byte-identical, while reads still replay every event this renderer understands.
#[test]
fn a_preference_log_with_an_unknown_event_is_never_overwritten() {
    let stored = serde_json::json!({ "version": 1, "events": [{ "mutation": "setAppearance", "appearance": "dark" }, { "mutation": "setSomethingNewer", "value": 1 }] }).to_string();
    let refusal = decode_ui_preferences_event_log(&stored).expect_err("an unknown event refuses the whole log");
    assert_eq!((refusal.reason, refusal.event_index), ("event", Some(1)));
    assert_eq!(replay_ui_preferences(&refusal.readable).appearance, Some(OsUiAppearance::Dark), "reads keep what this renderer understands");

    let mut store = MemoryPrefsStore::default();
    prefs_set_in(&mut store, UI_PREFERENCES_CONFIG_SCHEMA, &stored);
    let written = ui_preferences_log_write(prefs_get_from(&store, UI_PREFERENCES_CONFIG_SCHEMA).as_deref(), |log| log.events.push(set_locale(Some(OsUiLocale::De))));
    assert!(written.is_err(), "the write refuses");
    assert_eq!(prefs_get_from(&store, UI_PREFERENCES_CONFIG_SCHEMA).as_deref(), Some(stored.as_str()), "the stored log is byte-identical");
    for broken in ["not json", r#"{"version":2,"events":[]}"#, r#"{"version":1}"#] {
        assert!(ui_preferences_log_write(Some(broken), |log| log.events.push(set_locale(None))).is_err(), "{broken} refuses too");
    }

    let fresh = ui_preferences_log_write(None, |log| log.events.push(set_locale(Some(OsUiLocale::De)))).expect("an absent log is an empty log").expect("one event written");
    assert_eq!(replay_ui_preferences(&decode_ui_preferences_event_log(&fresh).expect("the written log decodes")).locale, Some(OsUiLocale::De));
    assert_eq!(ui_preferences_log_write(Some(&fresh), |_| {}).expect("readable"), None, "a write that adds nothing stores nothing");
}
"""
if "fn a_preference_log_with_an_unknown_event_is_never_overwritten" not in prefs_text:
    changes.append(f"append law {prefs_test.relative_to(ROOT)}")
    if not DRY:
        prefs_test.write_text(prefs_text.rstrip("\n") + "\n" + law, encoding="utf-8")

for line in changes:
    print(("WOULD " if DRY else "") + line)
for line in problems:
    print("PROBLEM " + line)
print(f"{'dry-run' if DRY else 'applied'}: {len(changes)} change(s), {len(problems)} problem(s)")
sys.exit(1 if problems else 0)
