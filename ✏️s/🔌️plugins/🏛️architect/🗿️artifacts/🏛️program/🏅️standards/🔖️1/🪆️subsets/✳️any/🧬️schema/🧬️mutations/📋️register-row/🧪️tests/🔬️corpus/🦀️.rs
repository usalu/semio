//! 🧫️ Register-row corpus — every committed case under `🧪️tests/🧫️fixtures/🧬️mutations/📋️register-row/<op>/<case>/`,
//! read at run time and held to the laws each per-register leaf suite held before the four generic leaves replaced them.
//!
//! The hand-authored JSON beside each case is the source of truth (contract D1, ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`):
//! `🦠️mutation`, `📸️snapshot/{⬅️before,➡️after}`, `🎯️outcome`, and either `🔺️diff/🔣️.json` (applied) or the empty
//! `🔺️diff/🚫️.absent` marker (rejected). Every law reports every failing case, never only the first.

use crate::standards::v1::subsets::any::schema::{diff::ProgramDiff, mutations::ProgramMutation, snapshot::ProgramSnapshot};
use protocol::Mutation;
use semio_framework_test_fixtures::fixture_path;
use std::path::{Path, PathBuf};

const CORPUS: &str = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/📋️register-row";

/// 🧫️ One committed case: its `<op>/<case>` name and its raw JSON members.
struct Case {
    name: String,
    before: String,
    after: String,
    mutation: String,
    diff: Option<String>,
    absent: Option<String>,
    outcome: serde_json::Value,
}

impl Case {
    fn base(&self) -> ProgramSnapshot {
        snapshot(&self.before, &self.name)
    }

    fn expected(&self) -> ProgramSnapshot {
        snapshot(&self.after, &self.name)
    }

    fn mutation(&self) -> ProgramMutation {
        serde_json::from_str(&self.mutation).unwrap_or_else(|error| panic!("{}: mutation decodes: {error}", self.name))
    }

    fn applied(&self) -> bool {
        self.outcome.get("status").and_then(serde_json::Value::as_str) == Some("applied")
    }
}

fn snapshot(text: &str, name: &str) -> ProgramSnapshot {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{name}: snapshot decodes: {error}"))
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn directories(path: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(path).unwrap_or_else(|error| panic!("{}: {error}", path.display())).map(|entry| entry.expect("corpus entry").path()).filter(|path| path.is_dir()).collect();
    found.sort();
    found
}

/// 📂️ Every committed case, in path order.
fn cases() -> Vec<Case> {
    let root = fixture_path!("../../../../../../../../..").join(CORPUS);
    let cases: Vec<Case> = directories(&root)
        .iter()
        .flat_map(|op| directories(op))
        .map(|case| {
            let optional = |leaf: &str| case.join(leaf).is_file().then(|| read(&case.join(leaf)));
            Case {
                name: case.strip_prefix(&root).expect("case under corpus").display().to_string(),
                before: read(&case.join("📸️snapshot/⬅️before/🔣️.json")),
                after: read(&case.join("📸️snapshot/➡️after/🔣️.json")),
                mutation: read(&case.join("🦠️mutation/🔣️.json")),
                diff: optional("🔺️diff/🔣️.json"),
                absent: optional("🔺️diff/🚫️.absent"),
                outcome: serde_json::from_str(&read(&case.join("🎯️outcome/🔣️.json"))).expect("outcome decodes"),
            }
        })
        .collect();
    assert_eq!(cases.len(), 326, "the register-row corpus holds 326 committed cases");
    cases
}

fn verdict(law: &str, failures: Vec<String>) {
    assert!(failures.is_empty(), "{law} fails for {} case(s):\n{}", failures.len(), failures.join("\n"));
}

fn canonical(text: &str, encoded: String) -> bool {
    serde_json::from_str::<serde_json::Value>(&encoded).ok() == serde_json::from_str::<serde_json::Value>(text).ok()
}

/// 🧩️ Every case declares exactly one of the two diff members, matching its declared status.
#[semio_framework_async_macros::async_test]
async fn every_case_declares_one_diff_member_matching_its_status() {
    verdict("diff member", cases().iter().filter(|case| case.diff.is_some() == case.absent.is_some() || case.diff.is_some() != case.applied()).map(|case| case.name.clone()).collect());
}

/// ▶️ An applied case carries its before-snapshot to exactly its after-snapshot; a rejected one leaves it untouched.
#[semio_framework_async_macros::async_test]
async fn every_case_applies_to_its_committed_after() {
    verdict(
        "apply",
        cases()
            .iter()
            .filter(|case| {
                let base = case.base();
                protocol::apply_diff(case.mutation().diff(&base).diff(), &base).ok() != Some(case.expected())
            })
            .map(|case| case.name.clone())
            .collect(),
    );
}

/// ↩️ Applying a case and then its own inverse restores the before-snapshot; a rejected case records no inverse step.
#[semio_framework_async_macros::async_test]
async fn every_case_inverse_restores_before() {
    let mut failures = Vec::new();
    for case in cases() {
        let (base, forward) = (case.base(), case.mutation());
        let mut undo = forward.inverse(&base).unwrap_or_else(|error| panic!("{}: inverse: {error}", case.name));
        if !case.applied() {
            if !undo.is_empty() {
                failures.push(case.name.clone());
            }
            continue;
        }
        undo.reverse();
        let mut state = protocol::apply_diff(forward.diff(&base).diff(), &base).unwrap_or_else(|error| panic!("{}: forward applies: {error:?}", case.name));
        for step in &undo {
            state = protocol::apply_diff(step.diff(&state).diff(), &state).unwrap_or_else(|error| panic!("{}: inverse step applies: {error:?}", case.name));
        }
        if state != base {
            failures.push(case.name.clone());
        }
    }
    verdict("inverse", failures);
}

/// 🔣️ Both committed snapshots and the committed payload are canonical: decode then encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn every_committed_json_is_canonical() {
    let mut failures = Vec::new();
    for case in cases() {
        for (side, text) in [("before", &case.before), ("after", &case.after)] {
            if !canonical(text, semio_framework_pack_json::to_json_string(&snapshot(text, &case.name))) {
                failures.push(format!("{} ({side})", case.name));
            }
        }
        if !canonical(&case.mutation, serde_json::to_string(&case.mutation()).expect("payload re-encodes")) {
            failures.push(format!("{} (mutation)", case.name));
        }
    }
    verdict("canonical JSON", failures);
}

/// 🎯️ The declared outcome holds: an applied case raises no diagnostic, a rejected one raises the declared code at the declared path.
#[semio_framework_async_macros::async_test]
async fn every_declared_outcome_holds() {
    let mut failures = Vec::new();
    for case in cases() {
        let outcome = case.mutation().diff(&case.base());
        let holds = if case.applied() {
            outcome.messages().is_empty()
        } else {
            let path: Vec<String> = case.outcome.get("path").and_then(serde_json::Value::as_array).map(|path| path.iter().filter_map(|step| step.as_str().map(str::to_string)).collect()).unwrap_or_default();
            outcome.messages().iter().find(|message| message.level >= semio_framework_diagnostic::Severity::Error).is_some_and(|raised| Some(raised.code.0.as_str()) == case.outcome.get("code").and_then(serde_json::Value::as_str) && raised.target == path)
        };
        if !holds {
            failures.push(case.name.clone());
        }
    }
    verdict("declared outcome", failures);
}

/// 🔺️ The sparse delta a case produces is exactly its committed diff — which collection and which fields it may touch — and a rejection builds none.
#[semio_framework_async_macros::async_test]
async fn every_case_produces_its_committed_diff() {
    let mut failures = Vec::new();
    for case in cases() {
        let produced = case.mutation().diff(&case.base()).diff().clone();
        let holds = match &case.diff {
            Some(committed) => canonical(committed, semio_framework_pack_json::to_json_string(&produced)),
            None => produced == ProgramDiff::default(),
        };
        if !holds {
            failures.push(case.name.clone());
        }
    }
    verdict("produced diff", failures);
}

/// 🩹 A committed diff is canonical and carries the before-snapshot to the after-snapshot on its own; a rejection's marker is empty and its after repeats its before.
#[semio_framework_async_macros::async_test]
async fn every_committed_diff_is_canonical_and_complete() {
    let mut failures = Vec::new();
    for case in cases() {
        let holds = match &case.diff {
            Some(committed) => {
                let decoded: ProgramDiff = semio_framework_pack_json::from_json_str(committed, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{}: committed diff decodes: {error}", case.name));
                canonical(committed, semio_framework_pack_json::to_json_string(&decoded)) && protocol::apply_diff(&decoded, &case.base()).ok() == Some(case.expected())
            }
            None => case.absent.as_deref() == Some("") && case.before == case.after,
        };
        if !holds {
            failures.push(case.name.clone());
        }
    }
    verdict("committed diff", failures);
}

/// 🧮️ Law L3: the diffs of every applied case's inverse mutations, summed with `absorb`, equal the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn every_applied_case_obeys_the_inverse_sum_law() {
    for case in cases().iter().filter(|case| case.applied()) {
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&case.mutation(), &case.base()).await;
    }
}
