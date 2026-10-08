//! 🧰️ Shared leaf-test kit: every mutation leaf proves its committed fixture quintet with one call per law.
//! Set `BIM_BLESS=1` to (re)write the `after` snapshot and the `diff` of applied cases from the code under test;
//! a blessed fixture is then reviewed by hand and cross-checked by the independent oracles.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{Mutation, MutationDiff};
use semio_framework_diagnostic::Severity;
use semio_framework_pack_json::{from_json_str, to_json_string, JsonMemberPolicy};
use std::path::{Path, PathBuf};

/// 📦️ One committed case: the quintet of its fixture documents and where they live.
pub struct Case {
    pub dir: &'static str,
    pub before: &'static str,
    pub after: &'static str,
    pub mutation: &'static str,
    pub diff: &'static str,
    pub outcome: &'static str,
}

fn normalise(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Number(number) => number.as_f64().and_then(serde_json::Number::from_f64).map_or(serde_json::Value::Number(number), serde_json::Value::Number),
        serde_json::Value::Array(items) => serde_json::Value::Array(items.into_iter().map(normalise).collect()),
        serde_json::Value::Object(entries) => serde_json::Value::Object(entries.into_iter().map(|(key, value)| (key, normalise(value))).collect()),
        other => other,
    }
}

fn json(text: &str) -> serde_json::Value {
    normalise(serde_json::from_str(text).expect("fixture is JSON"))
}

fn blessing() -> bool {
    std::env::var_os("BIM_BLESS").is_some()
}

fn fixture_path(case: &Case, relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations").join(case.dir).join(relative)
}

fn bless(case: &Case, relative: &str, text: &str) {
    let pretty = serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(text).expect("blessed text is JSON")).expect("pretty JSON");
    std::fs::write(fixture_path(case, relative), format!("{pretty}\n")).expect("blessed fixture is written");
}

/// 📸️ The committed before snapshot.
pub fn before(case: &Case) -> ModelSnapshot {
    from_json_str(case.before, JsonMemberPolicy::Reject).expect("before snapshot decodes")
}

/// 🦠️ The committed mutation payload.
pub fn mutation(case: &Case) -> ModelMutation {
    from_json_str(case.mutation, JsonMemberPolicy::Reject).expect("mutation decodes")
}

fn status(case: &Case) -> String {
    json(case.outcome)["status"].as_str().expect("outcome status").to_string()
}

fn rejected(messages: &[protocol::MutationMessage]) -> bool {
    messages.iter().any(|message| matches!(message.level, Severity::Error | Severity::Fatal))
}

/// 🎯️ The declared outcome holds: applied cases raise no Error/Fatal, rejected cases raise exactly the declared code and path and an empty diff.
pub fn outcome(case: &Case) {
    let declared = json(case.outcome);
    let (diff, messages) = mutation(case).diff(&before(case)).into_parts();
    match status(case).as_str() {
        "applied" => assert!(!rejected(&messages), "{}: declared applied but raised {messages:?}", case.dir),
        "rejected" => {
            let code = declared["code"].as_str().expect("rejected outcome declares a code");
            let hit = messages.iter().find(|message| message.code.0 == code && matches!(message.level, Severity::Warning | Severity::Error | Severity::Fatal));
            let hit = hit.unwrap_or_else(|| panic!("{}: declared rejection {code} not raised, got {messages:?}", case.dir));
            let path: Vec<String> = declared["path"].as_array().map(|items| items.iter().map(|item| item.as_str().expect("path segment").to_string()).collect()).unwrap_or_default();
            assert_eq!(hit.target, path, "{}: rejection target", case.dir);
            assert_eq!(diff, ModelDiff::default(), "{}: a rejected mutation produces an empty diff", case.dir);
        }
        other => panic!("{}: unknown declared status {other}", case.dir),
    }
}

/// ✅️ Applying the mutation through the central applier yields the committed after snapshot.
pub fn applies(case: &Case) {
    let base = before(case);
    let (diff, _) = mutation(case).diff(&base).into_parts();
    let next = protocol::apply_diff(&diff, &base).expect("the produced diff applies");
    if status(case) == "applied" && blessing() {
        bless(case, "📸️snapshot/➡️after/🔣️.json", &to_json_string(&next));
        return;
    }
    let expected: ModelSnapshot = from_json_str(case.after, JsonMemberPolicy::Reject).expect("after snapshot decodes");
    assert_eq!(next, expected, "{}: applied state differs from the committed after snapshot", case.dir);
}

/// 🔺️ The sparse diff the mutation produces is exactly the committed diff, and the committed diff carries before to after.
pub fn produces_diff(case: &Case) {
    let base = before(case);
    let (diff, _) = mutation(case).diff(&base).into_parts();
    let produced = to_json_string(&diff);
    if status(case) == "applied" && blessing() {
        bless(case, "🔺️diff/🔣️.json", &produced);
        return;
    }
    assert_eq!(json(&produced), json(case.diff), "{}: produced diff differs from the committed diff", case.dir);
    let committed: ModelDiff = from_json_str(case.diff, JsonMemberPolicy::Reject).expect("committed diff decodes");
    let expected: ModelSnapshot = from_json_str(case.after, JsonMemberPolicy::Reject).expect("after snapshot decodes");
    assert_eq!(protocol::apply_diff(&committed, &base).expect("committed diff applies"), expected, "{}: committed diff does not carry before to after", case.dir);
}

/// ↩️ Replaying the concrete inverse (storage-reversed, as the store does) restores the before snapshot.
pub fn inverse_restores(case: &Case) {
    if status(case) != "applied" {
        return;
    }
    let base = before(case);
    let forward = mutation(case);
    let (diff, _) = forward.diff(&base).into_parts();
    let mut state = protocol::apply_diff(&diff, &base).expect("forward applies");
    for undo in forward.inverse(&base).expect("inverse builds").iter().rev() {
        let (step, messages) = undo.diff(&state).into_parts();
        assert!(!rejected(&messages), "{}: an inverse step was rejected: {messages:?}", case.dir);
        state = protocol::apply_diff(&step, &state).expect("inverse step applies");
    }
    assert_eq!(state, base, "{}: inverse did not restore the before snapshot", case.dir);
}

/// 🔣️ Every committed document is canonical: decoding then encoding is a fixed point.
pub fn canonical(case: &Case) {
    for (label, text) in [("before", case.before), ("after", case.after)] {
        let decoded: ModelSnapshot = from_json_str(text, JsonMemberPolicy::Reject).expect("snapshot decodes");
        assert_eq!(json(&to_json_string(&decoded)), json(text), "{}: committed {label} is not canonical", case.dir);
    }
    assert_eq!(json(&to_json_string(&mutation(case))), json(case.mutation), "{}: committed mutation is not canonical", case.dir);
    let diff: ModelDiff = from_json_str(case.diff, JsonMemberPolicy::Reject).expect("diff decodes");
    assert_eq!(json(&to_json_string(&diff)), json(case.diff), "{}: committed diff is not canonical", case.dir);
}
