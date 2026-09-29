#!/usr/bin/env python3
"""🧮️ G12 session 15 host fix (os-mcp 🗂️catalog + 🔀️dispatch; S19 relay, forms `addBlock` "missing field `kind`"): an agent's
omitted optional-with-default argument runs with its declared default — the shells' ONE effective-args rule
(`manifest::effective_action_args`) applied in `ActionAdapter::prepare` before validation and the guest's PureCommand; the
agent's own keys stay untouched (an undeclared key is still refused by the schema). Idempotent; --dry-run reports.
usage: python3 g12-declared-defaults.py [--dry-run] [--root <tree>]"""
import pathlib
import sys

DRY = "--dry-run" in sys.argv
ROOT = pathlib.Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else pathlib.Path("/Users/ueli/Documents/semio")
MCP = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp"
CATALOG = MCP / "🗂️catalog/🦀️.rs"
DISPATCH = MCP / "🔀️dispatch/🦀️.rs"
QUICK = MCP / "🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs"

SUMMARY_OLD = "CapabilityArgSummary { id: arg.id.clone(), label: arg.label.resolve(terminology, locale).to_string(), required: arg.required }"
SUMMARY_NEW = "CapabilityArgSummary { id: arg.id.clone(), label: arg.label.resolve(terminology, locale).to_string(), required: arg.required, definition: Some(arg.clone()) }"

EDITS = {
    CATALOG: [
        (
            """/// 📝️ One argument's search/display-facing summary — deliberately not the full `manifest::ActionArgDef`
/// (whose `schema`/`presentation` already live in `input_schema` below; repeating them here would be
/// duplicate state).
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityArgSummary {
    pub id: String,
    pub label: String,
    pub required: bool,
}
""",
            """/// 📝️ One argument's search/display-facing summary — on the wire deliberately not the full `manifest::ActionArgDef`
/// (whose `schema`/`presentation` already live in `input_schema` below; repeating them there would be
/// duplicate state).
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityArgSummary {
    pub id: String,
    pub label: String,
    pub required: bool,
    /// 🧮️ The declared argument itself, never on the wire: `ActionAdapter::prepare` runs an agent's input through the
    /// shells' one effective-args rule (`manifest::effective_action_args`) with it, so an omitted optional argument
    /// runs with its declared `default`. `None` only for a summary decoded from a value.
    #[serde(skip)]
    #[value(skip)]
    pub definition: Option<manifest::ActionArgDef>,
}
""",
        ),
        (SUMMARY_OLD, SUMMARY_NEW),
        (SUMMARY_OLD, SUMMARY_NEW),
    ],
    DISPATCH: [
        (
            """        if let Ok(validator) = crate::schema::compile_validator(&capability.input_schema) {
            if let Err(validation_error) = crate::schema::validate(&validator, &input) {""",
            """        let input = declared_effective_input(capability, input);
        if let Ok(validator) = crate::schema::compile_validator(&capability.input_schema) {
            if let Err(validation_error) = crate::schema::validate(&validator, &input) {""",
        ),
        (
            """/// 📮️ The error a commit answers when its hub document did not acknowledge it:""",
            """/// 🧮️ The input an action runs with: the caller's own values, plus the declared `default` of every argument the caller
/// left out — the ONE effective-args rule the shells apply before dispatch (`manifest::effective_action_args`), because the
/// guest SDK no longer fills defaults (an agent's `{}` for forms `addBlock` failed "missing field `kind`" though `kind`
/// defaults to `text`). A key the caller sent is never replaced or dropped, declared or not, so the capability's schema
/// still refuses an undeclared one.
pub fn declared_effective_input(capability: &crate::catalog::CapabilityDefinition, input: serde_json::Value) -> serde_json::Value {
    let mut given = match input {
        serde_json::Value::Object(given) => given,
        other => return other,
    };
    let definitions = capability.presentation.args.iter().filter_map(|arg| arg.definition.clone()).collect::<Vec<_>>();
    if definitions.is_empty() {
        return serde_json::Value::Object(given);
    }
    let effective = semio_framework::manifest::effective_action_args(&definitions, &DslValue::from(&serde_json::Value::Object(given.clone())), None);
    for (key, value) in effective.as_object().unwrap_or_default() {
        if !given.contains_key(key) {
            given.insert(key.clone(), serde_json::Value::from(value.clone()));
        }
    }
    serde_json::Value::Object(given)
}

/// 📮️ The error a commit answers when its hub document did not acknowledge it:""",
        ),
    ],
    QUICK: [
        (
            """#[test]
fn empty_input_is_valid_for_a_capability_with_no_required_args() {""",
            """/// 🧮️ LAW (S19 relay, forms `addBlock` "missing field `kind`" on 7800/p33): an optional argument the agent leaves out runs
/// with its declared `default` — the shells' one effective-args rule (`manifest::effective_action_args`), the guest SDK no
/// longer fills defaults — the agent's own values run as given, and an undeclared key is still refused, never dropped.
#[test]
fn an_omitted_optional_argument_runs_with_its_declared_default() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let definitions = [
        semio_framework::manifest::ActionArgDef::text("kind", semio_framework_ui::wgpu::LocalizedLabel::native("Kind", "Art")).default_value(&"text".to_string()),
        semio_framework::manifest::ActionArgDef::text("title", semio_framework_ui::wgpu::LocalizedLabel::native("Title", "Titel")),
    ];
    let mut capability = synthetic_capability("forms.addBlock", &[], ApprovalMode::Never, false);
    capability.input_schema = serde_json::json!({ "type": "object", "properties": { "kind": { "type": "string", "default": "text" }, "title": { "type": "string" } }, "additionalProperties": false });
    capability.presentation.args = definitions.into_iter().map(|definition| crate::catalog::CapabilityArgSummary { id: definition.id.clone(), label: definition.id.clone(), required: definition.required, definition: Some(definition) }).collect();
    let catalog = single_capability_catalog(capability);
    let session = SessionHandle::new("sess_1");
    let principal = principal(&[]);
    let sent = || channel.frame_log().into_iter().filter_map(|(_, command)| match command { AppCommand::PureCommand { input, .. } => Some(input), _ => None }).last();
    adapter.prepare(&catalog, &principal, &session, "forms.addBlock", serde_json::json!({}), 0, 0).expect("{} runs with the declared default");
    assert_eq!(sent(), Some(serde_json::json!({ "kind": "text" })));
    adapter.prepare(&catalog, &principal, &session, "forms.addBlock", serde_json::json!({ "kind": "choice", "title": "T" }), 0, 1).expect("given values run as given");
    assert_eq!(sent(), Some(serde_json::json!({ "kind": "choice", "title": "T" })));
    let refused = adapter.prepare(&catalog, &principal, &session, "forms.addBlock", serde_json::json!({ "notAnArg": 1 }), 0, 2).unwrap_err();
    assert_eq!(refused.code, GatewayErrorCode::InputInvalid, "an undeclared key is refused, never dropped");
}

#[test]
fn empty_input_is_valid_for_a_capability_with_no_required_args() {""",
        ),
    ],
}


def main() -> int:
    texts = {path: path.read_text(encoding="utf-8") for path in EDITS}
    applied = 0
    for path, edits in EDITS.items():
        text = texts[path]
        for old, new in edits:
            if old == SUMMARY_OLD:
                if text.count(old) == 0:
                    continue
            elif text.count(new) == 1:
                continue
            if text.count(old) < 1 or (old != SUMMARY_OLD and text.count(old) != 1):
                print(f"PROBLEM {path.parent.name}: anchor count {text.count(old)}: {old[:90]!r}")
                return 1
            text = text.replace(old, new, 1)
            applied += 1
        texts[path] = text
    print(f"{applied} hunk(s) {'to apply' if DRY else 'applied'}" if applied else "nothing to do")
    if DRY or not applied:
        return 0
    for path, text in texts.items():
        path.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    sys.exit(main())
