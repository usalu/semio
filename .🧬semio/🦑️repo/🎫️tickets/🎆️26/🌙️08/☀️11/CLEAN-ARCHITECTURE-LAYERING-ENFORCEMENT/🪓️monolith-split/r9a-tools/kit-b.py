#!/usr/bin/env python3
"""Rewrites the bim mutation test kit to runtime-loaded cases and writes the fixture-discovery runner. usage: kit-b.py [--apply]"""
import sys,os
APPLY="--apply" in sys.argv
S="✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
kit=S+"🧪️tests/🧰️kit/🦀️.rs"
t=open(kit).read()
def rep(old,new,count=1):
    global t
    assert t.count(old)==count,(old,t.count(old))
    t=t.replace(old,new)
rep('''//! Set `BIM_BLESS=1` to (re)write''','''//! Cases are read at run time from the fixture folder, so a new case folder is covered without touching Rust.
//! Set `BIM_BLESS=1` to (re)write''')
start=t.index("/// 📦️ One committed case")
end=t.index("fn normalise(")
t=t[:start]+'''const BEFORE: &str = "📸️snapshot/⬅️before/🔣️.json";
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

'''+t[end:]
rep('''fn fixture_path(case: &Case, relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations").join(case.dir).join(relative)
}

''','')
rep('std::fs::write(fixture_path(case, relative), format!','std::fs::write(fixture_root().join(&case.dir).join(relative), format!')
rep('from_json_str(case.before, JsonMemberPolicy::Reject).expect("before snapshot decodes")','from_json_str(&case.before, JsonMemberPolicy::Reject).expect("before snapshot decodes")')
rep('from_json_str(case.mutation, JsonMemberPolicy::Reject).expect("mutation decodes")','from_json_str(&case.mutation, JsonMemberPolicy::Reject).expect("mutation decodes")')
rep('''fn status(case: &Case) -> String {
    json(case.outcome)["status"].as_str().expect("outcome status").to_string()
}
''','''fn status(case: &Case) -> String {
    json(&case.outcome)["status"].as_str().expect("outcome status").to_string()
}

/// ✅️ The declared outcome of the case is `applied`.
pub fn applied(case: &Case) -> bool {
    status(case) == "applied"
}
''')
rep('let declared = json(case.outcome);','let declared = json(&case.outcome);')
rep('bless(case, "📸️snapshot/➡️after/🔣️.json", &to_json_string(&next));','bless(case, AFTER, &to_json_string(&next));')
rep('from_json_str(case.after, JsonMemberPolicy::Reject)','from_json_str(&case.after, JsonMemberPolicy::Reject)',2)
rep('bless(case, "🔺️diff/🔣️.json", &produced);','bless(case, DIFF, &produced);')
rep('assert_eq!(json(&produced), json(case.diff),','assert_eq!(json(&produced), json(&case.diff),')
rep('from_json_str(case.diff, JsonMemberPolicy::Reject)','from_json_str(&case.diff, JsonMemberPolicy::Reject)',2)
rep('for (label, text) in [("before", case.before), ("after", case.after)] {','for (label, text) in [("before", &case.before), ("after", &case.after)] {')
rep('assert_eq!(json(&to_json_string(&decoded)), json(text),','assert_eq!(json(&to_json_string(&decoded)), json(text),')
rep('json(case.mutation)','json(&case.mutation)')
rep('json(&to_json_string(&diff)), json(case.diff),','json(&to_json_string(&diff)), json(&case.diff),')
assert "case.dir," not in t or True
runner='''//! 🗂️ Fixture-driven mutation laws: every committed case folder proves the same laws, discovered at run time, so a leaf is covered by adding its case folders.

use crate::standards::v1::subsets::any::schema::mutations::kit;

#[semio_framework_async_macros::async_test]
async fn the_fixture_root_holds_cases() {
    assert!(!kit::cases().is_empty(), "no committed mutation case was discovered");
}

#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    for case in kit::cases() {
        kit::outcome(&case);
    }
}

#[semio_framework_async_macros::async_test]
async fn applies_to_the_committed_after_snapshot() {
    for case in kit::cases() {
        kit::applies(&case);
    }
}

#[semio_framework_async_macros::async_test]
async fn produces_the_committed_diff() {
    for case in kit::cases() {
        kit::produces_diff(&case);
    }
}

#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_before_snapshot() {
    for case in kit::cases() {
        kit::inverse_restores(&case);
    }
}

#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for case in kit::cases() {
        kit::canonical(&case);
    }
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    for case in kit::cases().iter().filter(|case| kit::applied(case)) {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&kit::mutation(case), &kit::before(case)).await;
    }
}
'''
agg=S+"🦀️.rs"
a=open(agg).read()
old='''#[cfg(test)]
#[path = "🧪️tests/🧰️kit/🦀️.rs"]
pub mod kit;
'''
assert a.count(old)==1
anew=old+'''
#[cfg(test)]
#[path = "🧪️tests/🗂️fixture-cases/🦀️.rs"]
mod fixture_cases;
'''
import re
for m in re.finditer(r"(?<![&\w])case\.(before|after|mutation|diff|outcome)\b",t): print("BARE",t[max(0,m.start()-50):m.end()+20].replace("\n"," "))
print("kit lines",t.count("\n"))
if APPLY:
    open(kit,"w").write(t)
    os.makedirs(S+"🧪️tests/🗂️fixture-cases",exist_ok=True)
    open(S+"🧪️tests/🗂️fixture-cases/🦀️.rs","w").write(runner)
    open(agg,"w").write(a.replace(old,anew))
else:
    open("/Users/ueli/.cache/r9a/kit.preview.rs","w").write(t)
