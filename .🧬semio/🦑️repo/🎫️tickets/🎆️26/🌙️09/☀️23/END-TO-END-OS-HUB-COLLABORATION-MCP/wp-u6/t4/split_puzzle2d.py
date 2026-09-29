"""🙈️ (ii) verb split for puzzle2d in the overlay: setSelectionHidden/Locked + setTargetRegionHidden/Locked."""
import re, sys
from pathlib import Path

ROOT = Path(sys.argv[1]) / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any"
ED = ROOT / "✏️editor/🦀️.rs"
s = ED.read_text()

def once(old, new):
    global s
    n = s.count(old)
    assert n == 1, (old[:80], n)
    s = s.replace(old, new)

def every(old, new, count):
    global s
    n = s.count(old)
    assert n == count, (old[:80], n)
    s = s.replace(old, new)

once('    SetSelectionFlag = "setSelectionFlag",\n', '    SetSelectionFlag = "setSelectionFlag",\n    SetSelectionHidden = "setSelectionHidden",\n    SetSelectionLocked = "setSelectionLocked",\n')
once('    SetTargetRegionFlag = "setTargetRegionFlag",\n', '    SetTargetRegionFlag = "setTargetRegionFlag",\n    SetTargetRegionHidden = "setTargetRegionHidden",\n    SetTargetRegionLocked = "setTargetRegionLocked",\n')
every('\n    "setSelectionFlag",\n', '\n    "setSelectionFlag",\n    "setSelectionHidden",\n    "setSelectionLocked",\n', 2)
every('\n    "setTargetRegionFlag",\n', '\n    "setTargetRegionFlag",\n    "setTargetRegionHidden",\n    "setTargetRegionLocked",\n', 2)
once('\n            "setSelectionFlag",\n', '\n            "setSelectionFlag",\n            "setSelectionHidden",\n            "setSelectionLocked",\n')
once('\n            "setTargetRegionFlag",\n', '\n            "setTargetRegionFlag",\n            "setTargetRegionHidden",\n            "setTargetRegionLocked",\n')
for verb in ("setSelection", "setTargetRegion"):
    once(f'        ArtifactToolPublicationContract {{ tool_id: "{verb}Flag", lanes: &[ArtifactToolPublicationLane::Artifact] }},\n',
         f'        ArtifactToolPublicationContract {{ tool_id: "{verb}Flag", lanes: &[ArtifactToolPublicationLane::Artifact] }},\n'
         f'        ArtifactToolPublicationContract {{ tool_id: "{verb}Hidden", lanes: &[ArtifactToolPublicationLane::Artifact] }},\n'
         f'        ArtifactToolPublicationContract {{ tool_id: "{verb}Locked", lanes: &[ArtifactToolPublicationLane::Artifact] }},\n')
    once(f'            .action_interactive_job("{verb}Flag", InteractiveJobClassification::Migrated)\n',
         f'            .action_interactive_job("{verb}Flag", InteractiveJobClassification::Migrated)\n'
         f'            .action_interactive_job("{verb}Hidden", InteractiveJobClassification::Migrated)\n'
         f'            .action_interactive_job("{verb}Locked", InteractiveJobClassification::Migrated)\n')
once('            "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),\n',
     '            "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),\n'
     '            "setSelectionHidden" => set_selection_flag::set_selection_flag_value(ctx, args, "hidden"),\n'
     '            "setSelectionLocked" => set_selection_flag::set_selection_flag_value(ctx, args, "locked"),\n')
once('            "setTargetRegionFlag" => set_target_region_flag::set_target_region_flag(ctx, args),\n',
     '            "setTargetRegionFlag" => set_target_region_flag::set_target_region_flag(ctx, args),\n'
     '            "setTargetRegionHidden" => set_target_region_flag::set_target_region_flag_value(ctx, args, "hidden"),\n'
     '            "setTargetRegionLocked" => set_target_region_flag::set_target_region_flag_value(ctx, args, "locked"),\n')
once('    if !matches!(action, "patchInspectorNodes" | "setSelectionFlag" | "setTargetRegionFlag" | ',
     '    if !matches!(action, "patchInspectorNodes" | "setSelectionFlag" | "setSelectionHidden" | "setSelectionLocked" | "setTargetRegionFlag" | "setTargetRegionHidden" | "setTargetRegionLocked" | ')
once('    let addressed = if action == "setTargetRegionFlag" && command.args().and_then(|args| args.get("id")).is_some() {\n        1\n    } else {\n        command.args().filter(|_| action == "patchInspectorNodes")',
     '    let addressed = if matches!(action, "setTargetRegionFlag" | "setTargetRegionHidden" | "setTargetRegionLocked") && command.args().and_then(|args| args.get("id")).is_some() {\n        1\n    } else {\n        command.args().filter(|_| matches!(action, "patchInspectorNodes" | "setSelectionHidden" | "setSelectionLocked"))')
once('        let args = args.map(Value::from);\n        Puzzle2dCommand::try_from_action(',
     '        if let Some(flag) = puzzle2d_flag_value_argument(action) {\n'
     '            args.and_then(|value| value.get(flag)).and_then(dsl::DslValue::as_bool).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("puzzle2d.action.flag-value-required"), format!("action \'{action}\' requires the boolean \'{flag}\' it sets")))?;\n'
     '        }\n'
     '        let args = args.map(Value::from);\n        Puzzle2dCommand::try_from_action(')
once('''fn puzzle2d_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}
''', '''fn puzzle2d_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 🙈️ The flag a set-verb sets to exactly the boolean its arguments carry (`setSelectionHidden{hidden}`,
/// `setTargetRegionLocked{locked}`, …) — the row target's explicit next state, so a stale view sets a value and never flips one.
fn puzzle2d_flag_value_argument(action: &str) -> Option<&'static str> {
    match action {
        "setSelectionHidden" | "setTargetRegionHidden" => Some("hidden"),
        "setSelectionLocked" | "setTargetRegionLocked" => Some("locked"),
        _ => None,
    }
}

/// 🙈️ A set-verb's definition: the explicit identity a row names (or, left empty, the live selection) and the REQUIRED
/// boolean `flag` it sets — a missing value is refused, never defaulted.
fn puzzle2d_flag_value_action(id: &str, category: &str, identity: ActionArgDef, flag: &str, label: LocalizedLabel, value: LocalizedLabel) -> ActionDefinition {
    puzzle2d_internal_action(id, label, ActionKind::Mutation).with_category(category).with_args([identity, ActionArgDef::toggle(flag, value).required()])
}
''')
once('            .action_with(puzzle2d_internal_action("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).with_category("settings"))\n',
     '            .action_with(puzzle2d_internal_action("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Mutation).with_category("settings"))\n'
     '            .action_with(puzzle2d_flag_value_action("setSelectionHidden", "settings", ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs")), "hidden", LocalizedLabel::native("Set Hidden", "Verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))\n'
     '            .action_with(puzzle2d_flag_value_action("setSelectionLocked", "settings", ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs")), "locked", LocalizedLabel::native("Set Locked", "Gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))\n')
m = re.search(r'            \.action_with\(ActionDefinition \{ in_palette: false, \.\.ActionDefinition::bounded_catalog\("setTargetRegionFlag".*\n', s)
assert m
s = s[:m.end()] + (
    '            .action_with(puzzle2d_flag_value_action("setTargetRegionHidden", "targets", ActionArgDef::text("id", puzzle2d_localized(|l| l.id)), "hidden", LocalizedLabel::native("Set Target Region Hidden", "Zielbereich verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))\n'
    '            .action_with(puzzle2d_flag_value_action("setTargetRegionLocked", "targets", ActionArgDef::text("id", puzzle2d_localized(|l| l.id)), "locked", LocalizedLabel::native("Set Target Region Locked", "Zielbereich gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))\n'
) + s[m.end():]
for verb, en_noun, de_noun, de_acc in (("setSelection", "the given or selected nodes", "die angegebenen oder ausgewählten Knoten", None), ("setTargetRegion", "the given or selected target regions", "die angegebenen oder ausgewählten Zielbereiche", None)):
    m = re.search(r'            \.action_describe\("' + verb + r'Flag".*\n', s)
    assert m
    s = s[:m.end()] + (
        f'            .action_describe("{verb}Hidden", LocalizedLabel::native("Sets {en_noun} hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt {de_noun} oder zeigt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))\n'
        f'            .action_describe("{verb}Locked", LocalizedLabel::native("Sets {en_noun} locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt {de_noun} oder entsperrt sie, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))\n'
    ) + s[m.end():]
ED.write_text(s)

SEL = ROOT / "✏️editor/🎮️commands/🚩️set-selection-flag/🦀️.rs"
t = SEL.read_text()
old = '''pub fn set_selection_flag(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    let explicit'''
new = '''pub fn set_selection_flag(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    apply(ctx, args, flag, value);
}

/// 🎯️ `setSelectionHidden{hidden}`/`setSelectionLocked{locked}`: `flag` set to exactly the boolean the arguments carry —
/// the row target's explicit next state, so replaying it changes nothing. `command_from_action` already refused a
/// missing value, so there is no default to fall back to here.
pub fn set_selection_flag_value(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>, flag: &str) {
    if let Some(value) = args.and_then(|value| value.get(flag)).and_then(Value::as_bool) {
        apply(ctx, args, flag, value);
    }
}

fn apply(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>, flag: &str, value: bool) {
    let explicit'''
assert t.count(old) == 1
t = t.replace(old, new).replace("//! 🗂️ `set-selection-flag` command.", "//! 🗂️ `set-selection-flag` command, and the two set-verbs an outliner row names (`setSelectionHidden`,\n//! `setSelectionLocked`).")
SEL.write_text(t)

REG = ROOT / "✏️editor/🎮️commands/🚩️set-target-region-flag/🦀️.rs"
t = REG.read_text()
old = '''pub fn set_target_region_flag(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(Value::as_str).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(Value::as_bool).unwrap_or(true);
    let explicit'''
new = '''pub fn set_target_region_flag(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(Value::as_str).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(Value::as_bool).unwrap_or(true);
    apply(ctx, args, flag, value);
}

/// 🎯️ `setTargetRegionHidden{hidden}`/`setTargetRegionLocked{locked}`: `flag` set to exactly the boolean the arguments
/// carry — the row target's explicit next state, so replaying it changes nothing. `command_from_action` already refused
/// a missing value, so there is no default to fall back to here.
pub fn set_target_region_flag_value(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>, flag: &str) {
    if let Some(value) = args.and_then(|value| value.get(flag)).and_then(Value::as_bool) {
        apply(ctx, args, flag, value);
    }
}

fn apply(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>, flag: &str, value: bool) {
    let explicit'''
assert t.count(old) == 1
t = t.replace(old, new).replace("//! 🚩️ `set-target-region-flag` command.", "//! 🚩️ `set-target-region-flag` command, and the two set-verbs an outliner region row names (`setTargetRegionHidden`,\n//! `setTargetRegionLocked`).")
REG.write_text(t)

FX = ROOT / "🧫️fixtures/🗄️retained-jobs/🔣️.json"
f = FX.read_text()
for verb in ("setSelection", "setTargetRegion"):
    old = f'    "{verb}Flag",\n'
    assert f.count(old) == 1
    f = f.replace(old, old + f'    "{verb}Hidden",\n    "{verb}Locked",\n')
FX.write_text(f)
print("ok")
