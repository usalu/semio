"""🎨️ Authors the per-kind `applied` and `no-op` specification vectors for os.config's nine UI-preference leaves, the
scenario test that runs the implementation against each, and the test mount in each leaf. The vectors are
hand-specified here; the committed scenario tests are what prove them against the implementation."""
import json, os, copy
root = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/"
EMPTY = {"appearance": None, "layout": None, "driverId": None, "customDrivers": {}, "locale": None, "terminology": None, "themeId": None, "customThemes": {}, "keybindingOverrides": {}}
DRIVER = {"driverId": "studio", "label": "Studio", "config": {"scale": 1.25}}
THEME = {"themeId": "contrast", "label": "Contrast", "config": {"colors": {"surface": "#000000", "text": "#ffffff"}, "radius": 4}}
FULL = {"appearance": "dark", "layout": "desktop", "driverId": "studio", "customDrivers": {"studio": DRIVER}, "locale": "de", "terminology": "reuse", "themeId": "contrast", "customThemes": {"contrast": THEME}, "keybindingOverrides": {"edit.undo": "Meta+Z"}}
KINDS = [
    ("🌗️set-appearance", "set_appearance", "SetAppearance", {"mutation": "setAppearance", "appearance": "dark"}, lambda s: s.update(appearance="dark"), "appearance", "the appearance"),
    ("📐️set-layout", "set_layout", "SetLayout", {"mutation": "setLayout", "layout": "desktop"}, lambda s: s.update(layout="desktop"), "layout", "the layout"),
    ("🕹️set-driver", "set_driver", "SetDriver", {"mutation": "setDriver", "driverId": "studio"}, lambda s: s.update(driverId="studio"), "driver", "the driver"),
    ("🚗️set-custom-driver", "set_custom_driver", "SetCustomDriver", {"mutation": "setCustomDriver", "driverId": "studio", "driver": DRIVER}, lambda s: s["customDrivers"].update(studio=DRIVER), "custom-driver", "the studio driver"),
    ("🗣️set-locale", "set_locale", "SetLocale", {"mutation": "setLocale", "locale": "de"}, lambda s: s.update(locale="de"), "locale", "the locale"),
    ("📖️set-terminology", "set_terminology", "SetTerminology", {"mutation": "setTerminology", "terminology": "reuse"}, lambda s: s.update(terminology="reuse"), "terminology", "the terminology"),
    ("🖼️set-theme", "set_theme", "SetTheme", {"mutation": "setTheme", "themeId": "contrast"}, lambda s: s.update(themeId="contrast"), "theme", "the theme"),
    ("🎨️set-custom-theme", "set_custom_theme", "SetCustomTheme", {"mutation": "setCustomTheme", "themeId": "contrast", "theme": THEME}, lambda s: s["customThemes"].update(contrast=THEME), "custom-theme", "the contrast theme"),
    ("⌨️set-keybinding-override", "set_keybinding_override", "SetKeybindingOverride", {"mutation": "setKeybindingOverride", "controlId": "edit.undo", "keys": "Meta+Z"}, lambda s: s["keybindingOverrides"].update({"edit.undo": "Meta+Z"}), "keybinding", "the undo keybinding"),
]
TEST = '''//! 🧪️ `{kind}` fixture — `{scenario}`.
//!
//! {story}
//!
//! 🎚️ `UiPreferencesDiff` is a whole-record diff whose `apply` ignores `base`, so the committed
//! `🔺️diff` is the full post-op preferences record. Source of truth is the committed JSON quintet in
//! `../../🧫️fixtures/{scenario}/`.

use crate::opening_config::mutations::UiPreferencesConfigMutation;
use crate::opening_config::{{UiPreferences, UiPreferencesDiff}};

const BEFORE: &str = include_str!("../../🧫️fixtures/{scenario}/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../🧫️fixtures/{scenario}/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../🧫️fixtures/{scenario}/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../🧫️fixtures/{scenario}/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../🧫️fixtures/{scenario}/🎯️outcome/🔣️.json");

fn before() -> UiPreferences {{
    dsl::os_pack::json::from_json_str(BEFORE).expect("before preferences decode")
}}
fn expected_after() -> UiPreferences {{
    dsl::os_pack::json::from_json_str(AFTER).expect("after preferences decode")
}}
fn mutation() -> UiPreferencesConfigMutation {{
    dsl::os_pack::json::from_json_str(MUTATION).expect("{kind} mutation decodes")
}}
fn json_value<T: dsl::ToValue>(value: &T) -> serde_json::Value {{
    serde_json::from_str(&dsl::os_pack::json::to_json_string(value)).expect("canonical JSON parses in the independent serde_json oracle")
}}

/// ▶️ {applies}
#[test]
fn applies_to_committed_after() {{
    let base = before();
    let outcome = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &base);
    let applied = protocol::MutationDiff::apply(outcome.diff(), &base).expect("{kind} applies to its committed before-preferences");
    assert_eq!(applied, expected_after(), "{kind}/{slug}: the applied preferences differ from the committed after-snapshot");
}}

/// 🎯️ {declared}
#[test]
fn declared_outcome_holds() {{
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("{status}"), "{kind}/{slug}: this fixture declares a {status} outcome");
    let produced = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &before());
{outcome_check}
}}

/// 🔺️ The produced whole-record diff is the committed `🔺️diff`.
#[test]
fn produces_committed_diff() {{
    let outcome = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &before());
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(json_value(outcome.diff()), committed, "{kind}/{slug}: produced diff differs from the committed 🔺️diff");
    let decoded: UiPreferencesDiff = dsl::os_pack::json::from_json_str(DIFF).expect("committed diff decodes as UiPreferencesDiff");
    assert_eq!(protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies"), expected_after(), "{kind}/{slug}: the committed diff does not carry before to after");
}}

/// ↩️ The inverse restores the committed before-preferences.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let forward = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("forward {kind} applies");
    for step in <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::inverse(&mutation(), &base) {{
        let undo = <UiPreferencesConfigMutation as protocol::Mutation<UiPreferences>>::diff(&step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("{kind} inverse step applies");
    }}
    assert_eq!(snapshot, base, "{kind}/{slug}: the inverse did not restore the before-preferences");
}}

/// 🔣️ The committed preference records and payload are canonical: decode → encode is a fixed point.
#[test]
fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: UiPreferences = dsl::os_pack::json::from_json_str(text).expect("preferences decode");
        let original: serde_json::Value = serde_json::from_str(text).expect("preferences reparse");
        assert_eq!(json_value(&decoded), original, "{kind}/{slug}: committed {{label}} JSON is not canonical");
    }}
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("payload reparses");
    assert_eq!(json_value(&mutation()), original, "{kind}/{slug}: committed payload JSON is not canonical");
}}
'''
APPLIED_CHECK = '''    assert!(produced.messages().is_empty(), "{kind}/{slug}: a changed preference emits no diagnostics");'''
NOOP_CHECK = '''    assert_eq!(produced.worst_level(), Some(protocol::Severity::Warning), "{kind}/{slug}: an unchanged preference is a warned no-op, never a refusal");
    assert_eq!(produced.messages().iter().map(|message| message.code.0.clone()).collect::<Vec<_>>(), vec!["mutation.no-op".to_string()], "{kind}/{slug}: the only diagnostic is mutation.no-op");
    assert_eq!(protocol::MutationDiff::apply(produced.diff(), &before()).expect("no-op applies"), before(), "{kind}/{slug}: a no-op leaves the preferences untouched");'''
def dump(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(json.dumps(value, ensure_ascii=False, indent=2) + "\n")
for leaf, module, variant, payload, setter, field, noun in KINDS:
    kind = leaf.split("️", 1)[-1] if "️" in leaf else leaf
    kind = "".join(c for c in leaf if c.isascii()).lstrip("-")
    after_applied = copy.deepcopy(EMPTY); setter(after_applied)
    scenarios = [
        (f"✏️sets-{field}", "applied", EMPTY, after_applied, {"status": "applied"}, f"Setting {noun} on untouched preferences changes exactly that preference and nothing else.", f"Setting {noun} writes it into the record and leaves every other preference as it was.", f"The declared outcome is `applied` with no diagnostics: {noun} was not set before.", APPLIED_CHECK),
        (f"🟰️keeps-{field}", "no-op", FULL, FULL, {"status": "no-op", "messages": [{"level": "warning", "code": "mutation.no-op"}]}, f"Setting {noun} to the value the preferences already hold is the leaf's `mutation.no-op` guard: a `no-op` outcome whose whole-record diff restates the unchanged record.", f"Re-setting {noun} to its current value leaves the preferences exactly as they were.", f"The declared outcome is `no-op` with one `warning`-level `mutation.no-op`: {noun} already holds the requested value.", NOOP_CHECK),
    ]
    mounts = []
    for scenario, status, before, after, outcome, story, applies, declared, check in scenarios:
        fx = f"{root}{leaf}/🧫️fixtures/{scenario}/"
        dump(fx + "📸️snapshot/⬅️before/🔣️.json", before)
        dump(fx + "📸️snapshot/➡️after/🔣️.json", after)
        dump(fx + "🦠️mutation/🔣️.json", payload)
        dump(fx + "🔺️diff/🔣️.json", after)
        dump(fx + "🎯️outcome/🔣️.json", outcome)
        slug = scenario.split("️", 1)[-1]
        slug = "".join(c for c in scenario if c.isascii()).lstrip("-")
        text = TEST.format(kind=kind, scenario=scenario, slug=slug, story=story, applies=applies, declared=declared, status=status, outcome_check=check.format(kind=kind, slug=slug))
        tp = f"{root}{leaf}/🧪️tests/{scenario}/🦀️.rs"
        os.makedirs(os.path.dirname(tp), exist_ok=True)
        open(tp, "w", encoding="utf-8").write(text)
        mounts.append(f'#[cfg(test)]\n#[path = "🧪️tests/{scenario}/🦀️.rs"]\nmod tests_{slug.replace("-", "_")};\n')
    lp = f"{root}{leaf}/🦀️.rs"
    src = open(lp, encoding="utf-8").read()
    if "//#region 🧪️Tests" not in src:
        src = src.rstrip("\n") + "\n\n//#region 🧪️Tests\n" + "\n".join(mounts) + "//#endregion 🧪️Tests\n"
        open(lp, "w", encoding="utf-8").write(src)
    print(kind, [s[0] for s in scenarios])
