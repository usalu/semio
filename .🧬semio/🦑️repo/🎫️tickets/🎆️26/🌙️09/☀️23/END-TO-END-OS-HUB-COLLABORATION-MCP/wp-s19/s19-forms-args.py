#!/usr/bin/env python3
"""🤖️ S19 set `forms-args` (guest, T6 round 4+): the forms editor published NO arguments for 17 of its 19 agent-facing
mutation verbs while each command payload requires fields (`title`, `step_id`, `question_id`, `question_ids`, `json`,
…) — measured: G12 p33-3 (`updateForm`/`patchStep`/`moveStep`/`addQuestionOption`/`moveBlock`/`addVectorField` "arguments do
not decode: missing field …") and the native declared-verb census (`s14-s19-logs/census-t7-1.txt`: 17 `Unbridged`
findings, every agent-lane probe `Refused app.command.invalid-args`). Each verb now declares exactly the arguments its
bridge decodes, spelled as the block-list host sends them (`blockId` → `question_id`, `toStepId`, the Try/inspection
`value` → `value_json`); `addBlock.kind` is required (default `text`), `addBlock.stepId` optional; `setSpecJson.json` is
required JSON text; `submit` submits the Try window's live answers (its payload is `windowId`/`windowKindId`) → Chrome
audience, like `resetTry`/`previousStep`/`nextStep`. MCP catalog audits (os-mcp `search::long`, G12 relay): `updateForm`'s description was 23 characters (< 24) and
`exportFixture`/`exportResponses` write a user path without being destructive → longer en + de description, both
exports destructive. Law `every_agent_facing_verb_bridges_from_its_declared_arguments_alone`.
usage: s19-forms-args.py [--dry-run|--write|--revert]"""
import importlib.util
import os
import sys

spec = importlib.util.spec_from_file_location("s19_setlib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "s19_setlib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)

EDITOR = "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
TESTS = "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"

ADD_BLOCK_OLD = '''            .action_args("addBlock", vec![
                ActionArgDef::select(
                    "kind",
                    LocalizedLabel::native("Kind", "Art"),
                    FORM_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, LocalizedLabel::data(*kind))).collect(),
                )
                .default_value(&"text"),
            ])
'''
L = "LocalizedLabel::native"
POSITIONS = f'vec![ActionArgOption::new("before", {L}("Before", "Davor")), ActionArgOption::new("after", {L}("After", "Danach")), ActionArgOption::new("inside", {L}("Inside", "Innerhalb"))]'
KINDS = "FORM_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, LocalizedLabel::data(*kind))).collect()"


def text(id_, en, de):
    return f'ActionArgDef::text("{id_}", {L}("{en}", "{de}")).required()'


QUESTION = text("questionId", "Question", "Frage")
QUESTIONS = f'ActionArgDef::text_list("questionIds", {L}("Questions", "Fragen")).required()'
STEP = text("stepId", "Step", "Schritt")
FIELD = text("field", "Property", "Eigenschaft")
VALUE = f'ActionArgDef::any("value", {L}("Value", "Wert")).required()'
FIELD_KEY = text("fieldKey", "Field", "Feld")
OPTION = text("optionValue", "Option", "Option")
ARGS = {
    "removeBlock": [text("blockId", "Question", "Frage")],
    "patchQuestions": [QUESTIONS, FIELD, VALUE, f'ActionArgDef::text("paramKey", {L}("Parameter", "Parameter"))'],
    "patchQuestionOptions": [QUESTIONS, OPTION, FIELD, VALUE],
    "addQuestionOption": [QUESTION, text("label", "Label", "Beschriftung")],
    "removeQuestionOption": [QUESTION, OPTION],
    "patchVectorField": [QUESTION, FIELD_KEY, f'ActionArgDef::select("field", {L}("Property", "Eigenschaft"), vec![ActionArgOption::new("label", {L}("Label", "Beschriftung")), ActionArgOption::new("value", {L}("Value", "Wert"))]).required()', VALUE],
    "addVectorField": [QUESTION, FIELD_KEY],
    "removeVectorField": [QUESTION, FIELD_KEY],
    "moveBlock": [text("blockId", "Question", "Frage"), text("toStepId", "Target Step", "Zielschritt"), f'ActionArgDef::text("targetId", {L}("Target Question", "Zielfrage"))', f'ActionArgDef::select("position", {L}("Position", "Position"), {POSITIONS}).default_value(&"after")', f'ActionArgDef::index("index", {L}("Index", "Index"))'],
    "moveStep": [STEP, f'ActionArgDef::index("index", {L}("Index", "Index")).required()'],
    "removeStep": [STEP],
    "patchStep": [STEP, f'ActionArgDef::select("field", {L}("Property", "Eigenschaft"), vec![ActionArgOption::new("title", {L}("Title", "Titel")), ActionArgOption::new("description", {L}("Description", "Beschreibung"))]).required()', f'ActionArgDef::text("value", {L}("Value", "Wert")).required().min_length(0)'],
    "updateForm": [text("title", "Title", "Titel")],
    "dropQuestionKind": [f'ActionArgDef::select("kind", {L}("Kind", "Art"), {KINDS}).required()', text("targetId", "Target", "Ziel"), f'ActionArgDef::select("dropPosition", {L}("Position", "Position"), {POSITIONS}).required()'],
    "discardResponse": [text("id", "Response", "Antwort")],
}


def render(verb, args):
    rows = "".join(f"                {arg},\n" for arg in args)
    return f'            .action_args("{verb}", vec![\n{rows}            ])\n'


ADD_BLOCK_NEW = '''            .action_args("addBlock", vec![
                ActionArgDef::select(
                    "kind",
                    LocalizedLabel::native("Kind", "Art"),
                    FORM_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, LocalizedLabel::data(*kind))).collect(),
                )
                .default_value(&"text")
                .required(),
                ActionArgDef::text("stepId", LocalizedLabel::native("Step", "Schritt")),
            ])
''' + "".join(render(verb, args) for verb, args in ARGS.items())
SPEC_OLD = '            .action_args("setSpecJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Spec JSON", "Spezifikations-JSON"))])\n'
SPEC_NEW = '            .action_args("setSpecJson", vec![ActionArgDef::json_text("json", LocalizedLabel::native("Spec JSON", "Spezifikations-JSON")).required()])\n'
SUBMIT_OLD = '            .action_audience("nextStep", semio_framework_plugin::CapabilityAudience::Chrome)\n'
SUBMIT_NEW = SUBMIT_OLD + '            .action_audience("submit", semio_framework_plugin::CapabilityAudience::Chrome)\n'


UPDATE_FORM_OLD = '            .action_describe("updateForm", LocalizedLabel::native("Changes the form title.", "Ändert den Formulartitel."))\n'
UPDATE_FORM_NEW = '            .action_describe("updateForm", LocalizedLabel::native("Changes the title of the form that respondents see above its questions.", "Ändert den Titel des Formulars, den Antwortende über den Fragen sehen."))\n'
EXPORTS_OLD = '            .action_destructive("setSpecJson")\n'
EXPORTS_NEW = EXPORTS_OLD + '            .action_destructive("exportFixture")\n            .action_destructive("exportResponses")\n'


def editor(text_):
    return lib.chain(lib.replace_once(ADD_BLOCK_OLD, ADD_BLOCK_NEW), lib.replace_once(SPEC_OLD, SPEC_NEW), lib.replace_once(SUBMIT_OLD, SUBMIT_NEW), lib.replace_once(UPDATE_FORM_OLD, UPDATE_FORM_NEW), lib.replace_once(EXPORTS_OLD, EXPORTS_NEW))(text_)


LAW = '''/// 🤖️ LAW: an agent reads each verb's input schema off the capability catalog and sends exactly the declared arguments —
/// the required ones plus the declared defaults the gateway fills. Every agent-facing forms mutation must decode from that
/// alone (MCP p33-3: `updateForm`/`patchStep`/`moveStep`/`addQuestionOption`/`moveBlock`/`addVectorField` answered
/// "arguments do not decode: missing field …" because they declared nothing), and `submit` — whose payload is the Try
/// window's own address — is no agent verb.
#[test]
fn every_agent_facing_verb_bridges_from_its_declared_arguments_alone() {
    use semio_framework_plugin::ArgSchema;
    let declared_value = |arg: &ActionArgDef| -> Option<dsl::DslValue> {
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
            ArgSchema::Array { .. } => dsl::DslValue::Array(vec![dsl::DslValue::String("q1".into())]),
            _ => dsl::DslValue::String("q1".into()),
        })
    };
    let definition = create_forms_app();
    let mut checked = std::collections::BTreeSet::new();
    for action in definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())) {
        if semio_framework::resolve_audience(action) != semio_framework_plugin::CapabilityAudience::Agent || !matches!(action.kind, ActionKind::Mutation) {
            continue;
        }
        let args = dsl::DslValue::Object(action.args.iter().filter_map(|arg| declared_value(arg).map(|value| (arg.id.clone(), value))).collect());
        FormsPlayApp::command_from_action(&action.id, Some(&args)).unwrap_or_else(|fault| panic!("{} does not decode from its declared arguments alone: {}", action.id, fault.message));
        checked.insert(action.id.clone());
    }
    let expected = ["addBlock", "addQuestionOption", "addStep", "addVectorField", "discardResponse", "dropQuestionKind", "moveBlock", "moveStep", "patchQuestionOptions", "patchQuestions", "patchStep", "patchVectorField", "removeBlock", "removeQuestionOption", "removeStep", "removeVectorField", "setActiveExample", "setSpecJson", "updateForm"];
    assert_eq!(checked.iter().map(String::as_str).collect::<Vec<_>>(), expected, "the agent-facing forms mutations");
}

'''
LAW_ANCHOR = "/// 🐫️ The shell's spelling of the payload keys.\n"


def tests(text_):
    if "fn every_agent_facing_verb_bridges_from_its_declared_arguments_alone" in text_:
        return text_
    assert text_.count(LAW_ANCHOR) == 1, "law anchor"
    return text_.replace(LAW_ANCHOR, LAW + LAW_ANCHOR)


lib.run("forms-args", [(EDITOR, editor), (TESTS, tests)], sys.argv[1:])
