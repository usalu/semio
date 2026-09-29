#!/usr/bin/env python3
"""🎬️ S19 set `sequence-args` (guest, T6 round 4+): G12 p33-5 — sequence `moveStep`/`connectSteps`/`setStepParams`/… answered
"Sequence command does not match its exact retained route or payload envelope" (INTERNAL). Root cause (source + native
census `s14-s19-logs/census-t7-1.txt`: the agent lane refused `sequence.retained.tool-mismatch` for 8 verbs): those verbs
declared NO arguments, and the action bridge read their step ids with `unwrap_or_default()` — an agent following the
published (empty) input schema sent nothing, the bridge silently built an empty id, and only the retained route's
envelope check refused it, under a misleading name. Now every step/edge verb declares the arguments its bridge reads
(`removeStep`/`setStepCollapsed` `id`, `moveStep` `nodeId`+`x`+`y`, `connectSteps` `sourceNodeId`+`targetNodeId`,
`disconnectSteps` `fromId`+`toId`, `setStepParams` `id`+`params`, `addStepToSlot` `owner`+`slotName` (+ optional `kind`,
`x`, `y`)), and the bridge refuses a missing id or coordinate by name (`app.command.invalid-args`, the forms bridge's code)
instead of defaulting it. Laws: `every_agent_facing_verb_bridges_from_its_declared_arguments_alone`,
`a_step_verb_without_its_step_is_refused_by_name`. usage: s19-sequence-args.py [--dry-run|--write|--revert]"""
import importlib.util
import os
import sys

spec = importlib.util.spec_from_file_location("s19_setlib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "s19_setlib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)

EDITOR = "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
TESTS = "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"

BRIDGE_OLD = '''        let json_arg = |key: &str, fallback: &str| args.and_then(|value| value.get(key)).map_or_else(|| fallback.to_string(), dsl::json::to_json_string);
'''
BRIDGE_NEW = '''        let json_arg = |key: &str, fallback: &str| args.and_then(|value| value.get(key)).map_or_else(|| fallback.to_string(), dsl::json::to_json_string);
        let refused = |name: &str| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.invalid-args"), format!("sequence action '{action}' needs its '{name}' argument"));
        let required_text = |keys: &[&str]| text_arg(keys).filter(|value| !value.is_empty()).ok_or_else(|| refused(keys[0]));
        let required_number = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_f64).ok_or_else(|| refused(key));
'''
ARMS = [
    ('''                owner: text_arg(&["owner"]).unwrap_or_default(),
                slot_name: text_arg(&["slotName", "slot_name", "slot"]).unwrap_or_default(),''',
     '''                owner: required_text(&["owner"])?,
                slot_name: required_text(&["slotName", "slot_name", "slot"])?,'''),
    ('''"removeStep" => Ok(SequenceCommand::RemoveStep(remove_step::RemoveStep { id: text_arg(&["id", "stepId", "value"]).unwrap_or_default() })),''',
     '''"removeStep" => Ok(SequenceCommand::RemoveStep(remove_step::RemoveStep { id: required_text(&["id", "stepId", "value"])? })),'''),
    ('''"moveStep" => Ok(SequenceCommand::MoveStep(move_step::MoveStep { node_id: text_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(), x: number_arg(&["x"]), y: number_arg(&["y"]) })),''',
     '''"moveStep" => Ok(SequenceCommand::MoveStep(move_step::MoveStep { node_id: required_text(&["nodeId", "node_id", "id"])?, x: required_number("x")?, y: required_number("y")? })),'''),
    ('''"connectSteps" => Ok(SequenceCommand::ConnectSteps(connect_steps::ConnectSteps { source_node_id: text_arg(&["sourceNodeId", "source_node_id", "from"]).unwrap_or_default(), target_node_id: text_arg(&["targetNodeId", "target_node_id", "to"]).unwrap_or_default() })),''',
     '''"connectSteps" => Ok(SequenceCommand::ConnectSteps(connect_steps::ConnectSteps { source_node_id: required_text(&["sourceNodeId", "source_node_id", "from"])?, target_node_id: required_text(&["targetNodeId", "target_node_id", "to"])? })),'''),
    ('''"disconnectSteps" => Ok(SequenceCommand::DisconnectSteps(disconnect_steps::DisconnectSteps { from_id: text_arg(&["fromId", "from_id", "from"]).unwrap_or_default(), to_id: text_arg(&["toId", "to_id", "to"]).unwrap_or_default() })),''',
     '''"disconnectSteps" => Ok(SequenceCommand::DisconnectSteps(disconnect_steps::DisconnectSteps { from_id: required_text(&["fromId", "from_id", "from"])?, to_id: required_text(&["toId", "to_id", "to"])? })),'''),
    ('''"setStepParams" => Ok(SequenceCommand::SetStepParams(set_step_params::SetStepParams { id: text_arg(&["id", "stepId"]).unwrap_or_default(), params_json: json_arg("params", "null") })),''',
     '''"setStepParams" => Ok(SequenceCommand::SetStepParams(set_step_params::SetStepParams { id: required_text(&["id", "stepId"])?, params_json: json_arg("params", "null") })),'''),
    ('''"setStepCollapsed" => Ok(SequenceCommand::SetStepCollapsed(set_step_collapsed::SetStepCollapsed { id: text_arg(&["id", "stepId", "value"]).unwrap_or_default() })),''',
     '''"setStepCollapsed" => Ok(SequenceCommand::SetStepCollapsed(set_step_collapsed::SetStepCollapsed { id: required_text(&["id", "stepId", "value"])? })),'''),
]

L = "LocalizedLabel::native"
KINDS = f'''vec![
                    ActionArgOption::new("state.set", {L}("Set State", "Zustand setzen")),
                    ActionArgOption::new("log.print", {L}("Print", "Ausgeben")),
                    ActionArgOption::new("control.if", {L}("If", "Wenn")),
                    ActionArgOption::new("control.while", {L}("While", "Solange")),
                    ActionArgOption::new("math.add", {L}("Add", "Addieren")),
                ]'''
STEP = f'ActionArgDef::text("id", {L}("Step", "Schritt")).required()'
MANIFEST_ANCHOR = '''            .action_args("setOrientation", vec![
                ActionArgDef::select("orientation", LocalizedLabel::native("Orientation", "Ausrichtung"), vec![
                    ActionArgOption::new("leftRight", LocalizedLabel::native("Left to Right", "Links nach rechts")),
                    ActionArgOption::new("topBottom", LocalizedLabel::native("Top to Bottom", "Oben nach unten")),
                ]).required(),
            ])
'''
ARGS = {
    "addStepToSlot": [f'ActionArgDef::text("owner", {L}("Control Step", "Steuerschritt")).required()', f'ActionArgDef::text("slotName", {L}("Slot", "Slot")).required()', f'ActionArgDef::select("kind", {L}("Kind", "Art"), {KINDS}).default_value(&"log.print")', f'ActionArgDef::number("x", {L}("X", "X"))', f'ActionArgDef::number("y", {L}("Y", "Y"))'],
    "removeStep": [STEP],
    "moveStep": [f'ActionArgDef::text("nodeId", {L}("Step", "Schritt")).required()', f'ActionArgDef::number("x", {L}("X", "X")).required()', f'ActionArgDef::number("y", {L}("Y", "Y")).required()'],
    "connectSteps": [f'ActionArgDef::text("sourceNodeId", {L}("From Step", "Von Schritt")).required()', f'ActionArgDef::text("targetNodeId", {L}("To Step", "Zu Schritt")).required()'],
    "disconnectSteps": [f'ActionArgDef::text("fromId", {L}("From Step", "Von Schritt")).required()', f'ActionArgDef::text("toId", {L}("To Step", "Zu Schritt")).required()'],
    "setStepParams": [STEP, f'ActionArgDef::any("params", {L}("Parameters", "Parameter")).required()'],
    "setStepCollapsed": [STEP],
}
MANIFEST_NEW = MANIFEST_ANCHOR + "".join(f'            .action_args("{verb}", vec![\n' + "".join(f"                {arg},\n" for arg in args) + "            ])\n" for verb, args in ARGS.items())


def editor(text):
    text = lib.replace_once(BRIDGE_OLD, BRIDGE_NEW)(text)
    for old, new in ARMS:
        text = lib.replace_once(old, new)(text)
    return lib.replace_once(MANIFEST_ANCHOR, MANIFEST_NEW)(text)


LAW = '''/// 🤖️ LAW: an agent reads each verb's input schema off the capability catalog and sends exactly the declared arguments —
/// the required ones plus the declared defaults the gateway fills. Every agent-facing sequence mutation decodes from that
/// alone (G12 p33-5: `moveStep`/`connectSteps`/`setStepParams` declared nothing, the bridge built empty ids and the retained
/// route refused them as "does not match its exact retained route or payload envelope").
#[test]
fn every_agent_facing_verb_bridges_from_its_declared_arguments_alone() {
    use semio_framework_plugin::{ArgSchema, ArtifactEditor};
    let declared_value = |arg: &semio_framework_plugin::ActionArgDef| -> Option<dsl::DslValue> {
        if let Some(default) = &arg.default {
            return Some(default.clone());
        }
        if !arg.required {
            return None;
        }
        Some(match &arg.schema {
            ArgSchema::String { options, .. } => dsl::DslValue::String(options.first().map_or_else(|| "q1".to_string(), |option| option.value.clone())),
            ArgSchema::Number { .. } => dsl::DslValue::Number(dsl::Number::UInt(0)),
            ArgSchema::Boolean => dsl::DslValue::Bool(false),
            _ => dsl::DslValue::String("q1".into()),
        })
    };
    let definition = super::create_sequence_app();
    let mut checked = std::collections::BTreeSet::new();
    for action in definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())) {
        if semio_framework::resolve_audience(action) != semio_framework_plugin::CapabilityAudience::Agent || !matches!(action.kind, semio_framework_plugin::ActionKind::Mutation) {
            continue;
        }
        let args = dsl::DslValue::Object(action.args.iter().filter_map(|arg| declared_value(arg).map(|value| (arg.id.clone(), value))).collect());
        <super::SequencePlayApp as ArtifactEditor>::command_from_action(&action.id, Some(&args)).unwrap_or_else(|fault| panic!("{} does not decode from its declared arguments alone: {}", action.id, fault.message));
        checked.insert(action.id.clone());
    }
    for verb in ["addStep", "addStepToSlot", "connectSteps", "disconnectSteps", "moveStep", "removeStep", "setStepCollapsed", "setStepParams"] {
        assert!(checked.contains(verb), "{verb} is an agent-facing sequence mutation");
    }
}

/// 🚫️ LAW: a step or edge verb without its step is refused by name (`app.command.invalid-args`) — never bridged into an
/// empty id the retained route then refuses under another name.
#[test]
fn a_step_verb_without_its_step_is_refused_by_name() {
    use semio_framework_plugin::ArtifactEditor;
    for verb in ["removeStep", "moveStep", "connectSteps", "disconnectSteps", "setStepParams", "setStepCollapsed", "addStepToSlot"] {
        let fault = <super::SequencePlayApp as ArtifactEditor>::command_from_action(verb, Some(&dsl::DslValue::Object(Vec::new()))).err().unwrap_or_else(|| panic!("{verb} without arguments must be refused"));
        assert_eq!(fault.code.0, "app.command.invalid-args", "{verb}: {}", fault.message);
    }
}

'''
LAW_ANCHOR = "//#endregion 🔖️CommandSurface\n"


def tests(text):
    if "fn a_step_verb_without_its_step_is_refused_by_name" in text:
        return text
    assert text.count(LAW_ANCHOR) == 1, "CommandSurface region end"
    return text.replace(LAW_ANCHOR, LAW + LAW_ANCHOR)


lib.run("sequence-args", [(EDITOR, editor), (TESTS, tests)], sys.argv[1:])
