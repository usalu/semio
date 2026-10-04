#!/usr/bin/env python3
"""🧫️ W1G-6 staged Rust law (rules 43/44: test code cannot be verified while cargo is frozen, so it is staged, not saved).

The script inserts the Rust corpus law of the language-agnostic deferred-reprojection corpus
(`🏪️store/🧫️fixtures/🧫️deferred-reprojection/🔣️.json`, whose TS twin is `🏪️store/🧪️tests/🧪️deferred-reprojection/🟦️.ts`) into
the region `🧪️DeferredLocalStepLaws` of `🏪️store/🧪️tests/🧪️deferred-reprojection/🦀️.rs`. It also normalizes the codemod-mangled
`CountedOp::inverse` body. It is idempotent. `--check` only reports what is still pending.

Run it from the repo root, then (gated, rule 42):
`cargo test -p semio-framework-os --lib every_deferred_reprojection_corpus_case --message-format=short`.
"""
import pathlib
import sys

LAW_FILE = pathlib.Path("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️deferred-reprojection/🦀️.rs")
ANCHOR = "//#endregion 🧪️DeferredLocalStepLaws"
NAME = "async fn every_deferred_reprojection_corpus_case_waits_turns_and_adopts_as_the_corpus_says()"

INVERSE_OLD = (
    "    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {\n"
    "    Ok({\n"
    "        self.0.inverse(base)?.into_iter().map(|operation| Self(operation, self.1)).collect()\n"
    "    \n"
    "    })\n"
    "}\n"
)
INVERSE_NEW = """    fn inverse(&self, base: &DemoSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(self.0.inverse(base)?.into_iter().map(|operation| Self(operation, self.1)).collect())
    }
"""

LAW = r'''
/// 🧫️ LAW (N17, W1G-6): every case of the language-agnostic deferred-reprojection corpus (`🧫️deferred-reprojection`; its
/// independent fast-json-patch twin is `🟦️.ts` beside this file) holds on a store that authors the case's history and defers
/// its local and remote replays by the case's budget. The store waits exactly when the corpus says. While it waits it shows
/// and records the history from before the step, and its replay's total is the corpus' R. It takes exactly the corpus' turns
/// after the dispatch (or after the interrupt) and adopts or refuses what the corpus expects.
#[semio_framework_async_macros::async_test]
async fn every_deferred_reprojection_corpus_case_waits_turns_and_adopts_as_the_corpus_says() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️deferred-reprojection/🔣️.json")).expect("corpus parses");
    let operation = |value: &serde_json::Value| DemoMutation::from_value(value.clone().into()).expect("corpus operation");
    let snapshot = |value: &serde_json::Value| serde_json::from_value::<Option<i32>>(value["n"].clone()).expect("corpus snapshot");
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let budget = case["budget"].as_u64().map(|budget| ReplayTurnBudget::operations(budget as usize));
        let mut store = ArtifactStore::new(create_document_envelope::<DemoSnapshot, CountedOp>("demo/v1", name, DemoSnapshot { n: snapshot(&case["initial"]) }, None)).await;
        for run in case["history"].as_array().expect("history") {
            let author = if run["author"] == "local" { "local" } else { "other" };
            let edit: Vec<CountedOp> = run["edit"].as_array().expect("edit").iter().map(|value| CountedOp(operation(value), Some(author))).collect();
            for _ in 0..run["times"].as_u64().expect("times") {
                store.dispatch(ArtifactCommand::Apply { mutations: edit.clone(), description: None, transaction: None }).await.unwrap_or_else(|error| panic!("{name}: history {error}"));
                settle(&mut store);
            }
        }
        store.set_local_actor_id(Some("local".into())).expect("actor");
        store.defer_local_replays(budget);
        store.defer_remote_replays(budget);
        let mut logical = 0;
        for (index, step) in case["steps"].as_array().expect("steps").iter().enumerate() {
            let label = format!("{name} step {index}");
            let (command, expected) = (&step["command"], &step["expected"]);
            let target = |store: &ArtifactStore<DemoSnapshot, CountedOp>| {
                let (edit, op) = (command["target"]["edit"].as_u64().expect("edit"), command["target"]["op"].as_u64().expect("op"));
                store.mutation_ops().expect("operations").into_iter().find(|operation| operation.position as u64 == edit && u64::from(operation.op_index) == op).expect("a recorded target").mutation_id
            };
            let replacement = |value: &serde_json::Value| (value != "withdrawn").then(|| CountedOp(operation(value), None));
            let before = counted_view(&store);
            match command["kind"].as_str().expect("kind") {
                "remoteSupersede" => {
                    let input = match replacement(&command["replacement"]) {
                        Some(CountedOp(operation, _)) => replaced(&target(&store), operation),
                        None => protocol::SupersededInput { target: target(&store), replacement: protocol::InputReplacement::Withdrawn },
                    };
                    logical += 1;
                    store.ingest_remote(remote_supersede(name, vec![input], logical)).await.unwrap_or_else(|error| panic!("{label}: ingest {error}"));
                }
                kind => {
                    let next = match kind {
                        "undo" => ArtifactCommand::Undo,
                        "redo" => ArtifactCommand::Redo,
                        "trunk" => ArtifactCommand::SwitchAlternative { alternative_id: store.trunk_alternative_id() },
                        "supersede" => ArtifactCommand::Supersede { scope: None, inputs: vec![SupersedeInput { target: target(&store), replacement: replacement(&command["replacement"]) }] },
                        "alternative" => ArtifactCommand::CreateAlternativeWithSupersede {
                            name: command["name"].as_str().expect("alternative name").into(),
                            inputs: vec![SupersedeInput { target: target(&store), replacement: replacement(&command["replacement"]) }],
                        },
                        other => panic!("{label}: unknown command {other}"),
                    };
                    store.dispatch(next).await.unwrap_or_else(|error| panic!("{label}: dispatch {error}"));
                }
            }
            settle(&mut store);
            let waits = store.reprojection_progress().is_some();
            assert_eq!(waits, expected["waits"].as_bool().expect("waits"), "{label}: waits");
            if waits {
                assert_eq!(counted_view(&store), before, "{label}: nothing of the step shows or is recorded while it waits");
                assert_eq!(store.reprojection_progress().map(|progress| u64::from(progress.total)), expected["replayed"].as_u64(), "{label}: the replay's total is R");
            }
            if let Some(interrupt) = step.get("interrupt") {
                for turn in 0..interrupt["after"].as_u64().expect("after") {
                    assert!(store.step_reprojection(None).await.expect("a turn").is_some(), "{label}: still waiting after {turn} turns");
                    settle(&mut store);
                }
                if interrupt.get("discard").is_some() {
                    assert!(store.discard_local_step(), "{label}: the waiting step discards");
                } else if interrupt.get("cancel").is_some() {
                    assert!(store.cancel_reprojection(), "{label}: the running replay cancels");
                    assert_eq!(counted_view(&store), before, "{label}: a cancel leaves the store untouched");
                } else {
                    let edit = interrupt["edit"].as_array().expect("edit").iter().map(|value| CountedOp(operation(value), Some("local"))).collect();
                    store.dispatch(ArtifactCommand::Apply { mutations: edit, description: None, transaction: None }).await.unwrap_or_else(|error| panic!("{label}: interrupting edit {error}"));
                    settle(&mut store);
                    assert_eq!(store.snapshot_ref().n, snapshot(&expected["interruptedState"]), "{label}: the edit lands on the history before the step");
                }
            }
            let mut turns = 0u64;
            let verdict = loop {
                if store.reprojection_progress().is_none() {
                    break Ok(());
                }
                let stepped = store.step_reprojection(None).await;
                settle(&mut store);
                turns += 1;
                assert!(turns < 10_000, "{label}: the step ends");
                match stepped {
                    Ok(Some(_)) => {}
                    Ok(None) => break Ok(()),
                    Err(error) => break Err(error),
                }
            };
            assert_eq!(turns, expected["turns"].as_u64().expect("turns"), "{label}: turns");
            match expected["refused"].as_str() {
                Some(_) => assert!(matches!(&verdict, Err(VcsError::Rejected { policy: crate::os_spr::MergePolicy::Normal, messages }) if !messages.is_empty()), "{label}: {verdict:?}"),
                None => assert!(verdict.is_ok(), "{label}: {verdict:?}"),
            }
            assert!(!store.local_step_pending() && store.reprojection_progress().is_none(), "{label}: nothing waits afterwards");
            assert_eq!(store.snapshot_ref().n, snapshot(&expected["state"]), "{label}: state");
            assert_eq!(store.applied_edit_ids().len() as u64, expected["applied"].as_u64().expect("applied"), "{label}: applied edits");
            assert_eq!(store.supersessions().len() as u64, expected["supersessions"].as_u64().expect("supersessions"), "{label}: supersessions");
            test_support::assert_live_equals_replay(&store).await;
        }
    }
}
'''


def main() -> int:
    check = "--check" in sys.argv[1:]
    text = LAW_FILE.read_text(encoding="utf-8")
    pending = []
    if INVERSE_OLD in text:
        pending.append("inverse")
        text = text.replace(INVERSE_OLD, INVERSE_NEW, 1)
    if NAME not in text:
        if text.count(ANCHOR) != 1:
            raise SystemExit(f"anchor {ANCHOR!r} is not unique")
        pending.append("law")
        text = text.replace(ANCHOR, LAW + ANCHOR, 1)
    print("pending: " + (", ".join(pending) if pending else "none"))
    if not check and pending:
        LAW_FILE.write_text(text, encoding="utf-8")
        print("written")
    return 0


if __name__ == "__main__":
    sys.exit(main())
