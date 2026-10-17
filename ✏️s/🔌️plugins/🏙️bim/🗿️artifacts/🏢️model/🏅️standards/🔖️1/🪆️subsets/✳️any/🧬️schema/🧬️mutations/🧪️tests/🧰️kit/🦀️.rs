//! 🧰️ Shared leaf-test kit: every mutation leaf proves its committed fixture quintet with one call per law.
//! Cases are read at run time from the fixture folder, so a new case folder is covered without touching Rust.
//! Set `BIM_BLESS=1` to (re)write the `after` snapshot and the `diff` of applied cases from the code under test;
//! a blessed fixture is then reviewed by hand and cross-checked by the independent oracles.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, mutations::ModelMutation, snapshot::ModelSnapshot};
use protocol::{Mutation, MutationDiff};
use semio_framework_diagnostic::Severity;
use semio_framework_pack_json::{from_json_str, to_json_string, JsonMemberPolicy};
use std::path::{Path, PathBuf};

const BEFORE: &str = "📸️snapshot/⬅️before/🔣️.json";
const AFTER: &str = "📸️snapshot/➡️after/🔣️.json";
const MUTATION: &str = "🦠️mutation/🔣️.json";
const DIFF: &str = "🔺️diff/🔣️.json";
const OUTCOME: &str = "🎯️outcome/🔣️.json";

/// 📦️ One committed case: the quintet of its fixture documents, read from `dir` below the fixture root.
pub struct Case {
    pub dir: String,
    pub before: String,
    pub after: String,
    pub mutation: String,
    pub diff: String,
    pub outcome: String,
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations")
}

fn entry_name(path: &Path) -> String {
    path.file_name().expect("a fixture entry has a name").to_string_lossy().into_owned()
}

impl Case {
    /// 📥️ Reads the five documents of the case folder `dir` (`<leaf>/<case>`).
    pub fn load(dir: &str) -> Self {
        let read = |relative: &str| std::fs::read_to_string(fixture_root().join(dir).join(relative)).unwrap_or_else(|error| panic!("{dir}: fixture {relative} is readable: {error}"));
        Self { dir: dir.to_string(), before: read(BEFORE), after: read(AFTER), mutation: read(MUTATION), diff: read(DIFF), outcome: read(OUTCOME) }
    }
}

/// 🗂️ Every committed case folder, in path order: a folder `<leaf>/<case>` holding the declared outcome document.
pub fn cases() -> Vec<Case> {
    let mut dirs = Vec::new();
    for leaf in std::fs::read_dir(fixture_root()).expect("the fixture root is readable") {
        let leaf = leaf.expect("a fixture leaf entry").path();
        if !leaf.is_dir() {
            continue;
        }
        for case in std::fs::read_dir(&leaf).expect("a fixture leaf is readable") {
            let case = case.expect("a fixture case entry").path();
            if case.join(OUTCOME).is_file() {
                dirs.push(format!("{}/{}", entry_name(&leaf), entry_name(&case)));
            }
        }
    }
    dirs.sort();
    dirs.iter().map(|dir| Case::load(dir)).collect()
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

fn bless(case: &Case, relative: &str, text: &str) {
    let pretty = serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(text).expect("blessed text is JSON")).expect("pretty JSON");
    std::fs::write(fixture_root().join(&case.dir).join(relative), format!("{pretty}\n")).expect("blessed fixture is written");
}

/// 📸️ The committed before snapshot.
pub fn before(case: &Case) -> ModelSnapshot {
    from_json_str(&case.before, JsonMemberPolicy::Reject).expect("before snapshot decodes")
}

/// 🦠️ The committed mutation payload.
pub fn mutation(case: &Case) -> ModelMutation {
    from_json_str(&case.mutation, JsonMemberPolicy::Reject).expect("mutation decodes")
}

fn status(case: &Case) -> String {
    json(&case.outcome)["status"].as_str().expect("outcome status").to_string()
}

/// ✅️ The declared outcome of the case is `applied`.
pub fn applied(case: &Case) -> bool {
    status(case) == "applied"
}

fn rejected(messages: &[protocol::MutationMessage]) -> bool {
    messages.iter().any(|message| matches!(message.level, Severity::Error | Severity::Fatal))
}

/// 🎯️ The declared outcome holds: applied cases raise no Error/Fatal, rejected cases raise exactly the declared code and path and an empty diff.
pub fn outcome(case: &Case) {
    let declared = json(&case.outcome);
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
        bless(case, AFTER, &to_json_string(&next));
        return;
    }
    let expected: ModelSnapshot = from_json_str(&case.after, JsonMemberPolicy::Reject).expect("after snapshot decodes");
    assert_eq!(next, expected, "{}: applied state differs from the committed after snapshot", case.dir);
}

/// 🔺️ The sparse diff the mutation produces is exactly the committed diff, and the committed diff carries before to after.
pub fn produces_diff(case: &Case) {
    let base = before(case);
    let (diff, _) = mutation(case).diff(&base).into_parts();
    let produced = to_json_string(&diff);
    if status(case) == "applied" && blessing() {
        bless(case, DIFF, &produced);
        return;
    }
    assert_eq!(json(&produced), json(&case.diff), "{}: produced diff differs from the committed diff", case.dir);
    let committed: ModelDiff = from_json_str(&case.diff, JsonMemberPolicy::Reject).expect("committed diff decodes");
    let expected: ModelSnapshot = from_json_str(&case.after, JsonMemberPolicy::Reject).expect("after snapshot decodes");
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
    for (label, text) in [("before", &case.before), ("after", &case.after)] {
        let decoded: ModelSnapshot = from_json_str(text, JsonMemberPolicy::Reject).expect("snapshot decodes");
        assert_eq!(json(&to_json_string(&decoded)), json(text), "{}: committed {label} is not canonical", case.dir);
    }
    assert_eq!(json(&to_json_string(&mutation(case))), json(&case.mutation), "{}: committed mutation is not canonical", case.dir);
    let diff: ModelDiff = from_json_str(&case.diff, JsonMemberPolicy::Reject).expect("diff decodes");
    assert_eq!(json(&to_json_string(&diff)), json(&case.diff), "{}: committed diff is not canonical", case.dir);
}
