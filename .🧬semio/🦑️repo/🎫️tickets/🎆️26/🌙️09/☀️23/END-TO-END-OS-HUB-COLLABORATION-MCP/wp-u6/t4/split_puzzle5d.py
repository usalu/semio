"""🙈️ (ii) verb split for puzzle5d in the overlay: setSelectionHidden/Locked + setTargetVolumeHidden/Locked."""
import re, sys
from pathlib import Path

ROOT = Path(sys.argv[1]) / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any"
ED = ROOT / "✏️editor/🦀️.rs"
s = ED.read_text()

def once(old, new):
    global s
    n = s.count(old)
    assert n == 1, (old[:90], n)
    s = s.replace(old, new)

def after_line(pattern, lines):
    global s
    ms = list(re.finditer(pattern, s))
    assert len(ms) == 1, (pattern, len(ms))
    m = ms[0]
    s = s[:m.end()] + "".join(lines) + s[m.end():]

once('    SetSelectionFlag = "setSelectionFlag",\n', '    SetSelectionFlag = "setSelectionFlag",\n    SetSelectionHidden = "setSelectionHidden",\n    SetSelectionLocked = "setSelectionLocked",\n')
once('    SetTargetVolumeFlag = "setTargetVolumeFlag",\n', '    SetTargetVolumeFlag = "setTargetVolumeFlag",\n    SetTargetVolumeHidden = "setTargetVolumeHidden",\n    SetTargetVolumeLocked = "setTargetVolumeLocked",\n')
once('\n    "setSelectionFlag",\n', '\n    "setSelectionFlag",\n    "setSelectionHidden",\n    "setSelectionLocked",\n')
once('\n    "setTargetVolumeFlag",\n', '\n    "setTargetVolumeFlag",\n    "setTargetVolumeHidden",\n    "setTargetVolumeLocked",\n')
once('\n            "setSelectionFlag",\n', '\n            "setSelectionFlag",\n            "setSelectionHidden",\n            "setSelectionLocked",\n')
once('\n            "setTargetVolumeFlag",\n', '\n            "setTargetVolumeFlag",\n            "setTargetVolumeHidden",\n            "setTargetVolumeLocked",\n')
for verb in ("setSelection", "setTargetVolume"):
    once(f'        ArtifactToolPublicationContract {{ tool_id: "{verb}Flag", lanes: &[ArtifactToolPublicationLane::Artifact] }},\n',
         f'        ArtifactToolPublicationContract {{ tool_id: "{verb}Flag", lanes: &[ArtifactToolPublicationLane::Artifact] }},\n'
         f'        ArtifactToolPublicationContract {{ tool_id: "{verb}Hidden", lanes: &[ArtifactToolPublicationLane::Artifact] }},\n'
         f'        ArtifactToolPublicationContract {{ tool_id: "{verb}Locked", lanes: &[ArtifactToolPublicationLane::Artifact] }},\n')
    once(f'            .action_interactive_job("{verb}Flag", InteractiveJobClassification::Migrated)\n',
         f'            .action_interactive_job("{verb}Flag", InteractiveJobClassification::Migrated)\n'
         f'            .action_interactive_job("{verb}Hidden", InteractiveJobClassification::Migrated)\n'
         f'            .action_interactive_job("{verb}Locked", InteractiveJobClassification::Migrated)\n')
once('        "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),\n',
     '        "setSelectionFlag" => set_selection_flag::set_selection_flag(ctx, args),\n'
     '        "setSelectionHidden" => set_selection_flag::set_selection_flag_value(ctx, args, "hidden"),\n'
     '        "setSelectionLocked" => set_selection_flag::set_selection_flag_value(ctx, args, "locked"),\n')
once('        "setTargetVolumeFlag" => set_target_volume_flag::set_target_volume_flag(ctx, args),\n',
     '        "setTargetVolumeFlag" => set_target_volume_flag::set_target_volume_flag(ctx, args),\n'
     '        "setTargetVolumeHidden" => set_target_volume_flag::set_target_volume_flag_value(ctx, args, "hidden"),\n'
     '        "setTargetVolumeLocked" => set_target_volume_flag::set_target_volume_flag_value(ctx, args, "locked"),\n')
once('''    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.map(dsl::os_pack::json::from_dsl_value);''', '''    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        if let Some(flag) = puzzle5d_flag_value_argument(action) {
            args.and_then(|value| value.get(flag)).and_then(dsl::DslValue::as_bool).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("puzzle5d.action.flag-value-required"), format!("action '{action}' requires the boolean '{flag}' it sets")))?;
        }
        let args = args.map(dsl::os_pack::json::from_dsl_value);''')
once('''fn puzzle5d_part_kind_arg() -> ActionArgDef {''', '''/// 🙈️ The flag a set-verb sets to exactly the boolean its arguments carry (`setSelectionHidden{hidden}`,
/// `setTargetVolumeLocked{locked}`, …) — the row target's explicit next state, so a stale view sets a value and never flips one.
fn puzzle5d_flag_value_argument(action: &str) -> Option<&'static str> {
    match action {
        "setSelectionHidden" | "setTargetVolumeHidden" => Some("hidden"),
        "setSelectionLocked" | "setTargetVolumeLocked" => Some("locked"),
        _ => None,
    }
}

/// 🙈️ A set-verb's definition: the explicit identity a row names (or, left empty, the live selection) and the REQUIRED
/// boolean `flag` it sets — a missing value is refused, never defaulted.
fn puzzle5d_flag_value_action(id: &str, identity: Vec<ActionArgDef>, flag: &str, label: LocalizedLabel, value: LocalizedLabel) -> ActionDefinition {
    let mut args = identity;
    args.push(ActionArgDef::toggle(flag, value).required());
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation).with_category("settings").with_args(args) }
}

fn puzzle5d_part_kind_arg() -> ActionArgDef {''')
after_line(r'            \.action_with\(ActionDefinition::bounded_catalog\("setSelectionFlag".*\n', [
    '            .action_with(puzzle5d_flag_value_action("setSelectionHidden", vec![ActionArgDef::text("entity", LocalizedLabel::native("Entity", "Entität")), ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs"))], "hidden", LocalizedLabel::native("Set Hidden", "Verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))\n',
    '            .action_with(puzzle5d_flag_value_action("setSelectionLocked", vec![ActionArgDef::text("entity", LocalizedLabel::native("Entity", "Entität")), ActionArgDef::text_list("ids", LocalizedLabel::native("Ids", "IDs"))], "locked", LocalizedLabel::native("Set Locked", "Gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))\n',
])
after_line(r'            \.action_with\(ActionDefinition::bounded_catalog\("setTargetVolumeFlag".*\n', [
    '            .action_with(puzzle5d_flag_value_action("setTargetVolumeHidden", vec![ActionArgDef::text("id", LocalizedLabel::native("Id", "ID"))], "hidden", LocalizedLabel::native("Set Target Volume Hidden", "Zielvolumen verborgen festlegen"), LocalizedLabel::native("Hidden", "Verborgen")))\n',
    '            .action_with(puzzle5d_flag_value_action("setTargetVolumeLocked", vec![ActionArgDef::text("id", LocalizedLabel::native("Id", "ID"))], "locked", LocalizedLabel::native("Set Target Volume Locked", "Zielvolumen gesperrt festlegen"), LocalizedLabel::native("Locked", "Gesperrt")))\n',
])
for verb, en, de in (("setSelection", "the given or selected parts", "die angegebenen oder ausgewählten Teile"), ("setTargetVolume", "the given target volume", "das angegebene Zielvolumen")):
    after_line(r'    \.action_describe\("' + verb + r'Flag".*\n', [
        f'    .action_describe("{verb}Hidden", LocalizedLabel::native("Sets {en} hidden or shown, to exactly the value passed; repeating it changes nothing.", "Verbirgt {de} oder zeigt {"sie" if verb == "setSelection" else "es"}, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))\n',
        f'    .action_describe("{verb}Locked", LocalizedLabel::native("Sets {en} locked or unlocked, to exactly the value passed; repeating it changes nothing.", "Sperrt {de} oder entsperrt {"sie" if verb == "setSelection" else "es"}, genau nach dem übergebenen Wert; eine Wiederholung ändert nichts."))\n',
    ])
ED.write_text(s)

SEL = ROOT / "✏️editor/🎮️commands/🚩️set-selection-flag/🦀️.rs"
t = SEL.read_text()
old = '''pub fn set_selection_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    let entity'''
new = '''pub fn set_selection_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    apply(ctx, args, flag, value);
}

/// 🎯️ `setSelectionHidden{hidden}`/`setSelectionLocked{locked}`: `flag` set to exactly the boolean the arguments carry —
/// the row target's explicit next state, so replaying it changes nothing. `command_from_action` already refused a
/// missing value, so there is no default to fall back to here.
pub fn set_selection_flag_value(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>, flag: &str) {
    if let Some(value) = args.and_then(|value| value.get(flag)).and_then(Value::as_bool) {
        apply(ctx, args, flag, value);
    }
}

fn apply(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>, flag: &str, value: bool) {
    let entity'''
assert t.count(old) == 1
t = t.replace(old, new).replace("//! 🚩️ `set-selection-flag` command.", "//! 🚩️ `set-selection-flag` command, and the two set-verbs a document-tree part row names (`setSelectionHidden`,\n//! `setSelectionLocked`).")
SEL.write_text(t)

VOL = ROOT / "✏️editor/🎮️commands/🚩️set-target-volume-flag/🦀️.rs"
t = VOL.read_text()
old = '''/// 🚩️ The outliner's show/hide and lock/unlock toggles for one target volume. An unknown flag name
/// writes nothing, so a stale row action cannot corrupt the other flag.
pub fn set_target_volume_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let id = args.and_then(|value| value.get("id")).and_then(Value::as_str).unwrap_or("");
    let flag = args.and_then(|value| value.get("flag")).and_then(Value::as_str).unwrap_or("");
    let value = args.and_then(|value| value.get("value")).and_then(Value::as_bool).unwrap_or(false);
    if let'''
new = '''/// 🚩️ One flag of one target volume by name. An unknown flag name writes nothing, so a stale caller cannot corrupt the
/// other flag.
pub fn set_target_volume_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(Value::as_str).unwrap_or("");
    let value = args.and_then(|value| value.get("value")).and_then(Value::as_bool).unwrap_or(false);
    apply(ctx, args, flag, value);
}

/// 🎯️ The outliner's show/hide and lock/unlock toggles for one target volume — `setTargetVolumeHidden{hidden}`/
/// `setTargetVolumeLocked{locked}` set `flag` to exactly the boolean the arguments carry, the row target's explicit next
/// state, so replaying one changes nothing. `command_from_action` already refused a missing value.
pub fn set_target_volume_flag_value(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>, flag: &str) {
    if let Some(value) = args.and_then(|value| value.get(flag)).and_then(Value::as_bool) {
        apply(ctx, args, flag, value);
    }
}

fn apply(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>, flag: &str, value: bool) {
    let id = args.and_then(|value| value.get("id")).and_then(Value::as_str).unwrap_or("");
    if let'''
assert t.count(old) == 1
t = t.replace(old, new).replace("//! 🚩️ `set-target-volume-flag` command.", "//! 🚩️ `set-target-volume-flag` command, and the two set-verbs an outliner target-volume row names\n//! (`setTargetVolumeHidden`, `setTargetVolumeLocked`).")
VOL.write_text(t)

FX = ROOT / "🧫️fixtures/🗄️retained-jobs/🔣️.json"
f = FX.read_text()
for verb in ("setSelection", "setTargetVolume"):
    old = f'    "{verb}Flag",\n'
    assert f.count(old) == 1, verb
    f = f.replace(old, old + f'    "{verb}Hidden",\n    "{verb}Locked",\n')
FX.write_text(f)
print("ok")
