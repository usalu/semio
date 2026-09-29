#!/usr/bin/env python3
"""🧊️ S19 set `transaction-retire` (guest SDK, T6 round 4+): every agent transaction on a procedural document trapped the guest
(G12 p33-3/p33-5 on hub 7800: generation2d/3d `addWidget`, add/rename/removeGeneration, reorganize, setActiveExample —
prepare OK, EVERY invoke `guest trapped … wasm function 30081`). Root cause (source): the SDK folds the transaction's ops
over a SCRATCH copy of the live document (`self.store.snapshot()`) to collect foreign steps, and plain-drops every displaced
scratch projection, every built diff and the final projection. A procedural document owns a fail-closed root
(`host_snapshot.layout: OrderedMap` — "ordered-map root must be explicitly retired before drop", the same class as the
session-14 hydration/genesis traps), so `transaction_prepare` aborted on its unconditional fold for every procedural invoke;
the two dispatch-side copies of the same fold (local emit + mounted emit, gated by `may_emit_foreign_steps`) carry the same
drop. The three copies become ONE `fold_foreign_steps` that retires every scratch projection
(`MutationDiff::retire_projection`) and every diff (`MutationDiff::retire_cold`, as `os_vcs::apply_mutation` does).
Law (generation3d): `an_agent_transaction_prepares_and_commits_without_trapping`.
usage: s19-transaction-retire.py [--dry-run|--write|--revert]"""
import importlib.util
import os
import sys

spec = importlib.util.spec_from_file_location("s19_setlib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "s19_setlib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)

SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
TESTS = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"

LOCAL_OLD = '''            if artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
                let mut running = self.store.snapshot().map_err(|error| error.into_fault())?;
                let mut foreign = Vec::new();
                for op in &artifact_mutations {
                    foreign.extend(op.foreign_steps(&running));
                    let outcome = op.diff(&running);
                    if !outcome.is_applicable(protocol::MergePolicy::default()) {
                        return Err(Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                    }
                    running = outcome.diff().apply(&running).map_err(|error| Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
                }
                if !foreign.is_empty() {'''
LOCAL_NEW = '''            if artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
                let foreign = self.fold_foreign_steps(&artifact_mutations)?;
                if !foreign.is_empty() {'''
MOUNTED_OLD = '''                        if emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
                            let mut running = self.store.snapshot().map_err(|error| error.into_fault())?;
                            let mut foreign = Vec::new();
                            for op in emit.artifact_mutations.iter() {
                                foreign.extend(op.foreign_steps(&running));
                                let outcome = op.diff(&running);
                                if !outcome.is_applicable(protocol::MergePolicy::default()) {
                                    return Err(Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages())));
                                }
                                running = outcome.diff().apply(&running).map_err(|error| Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()))?;
                            }
                            if !foreign.is_empty() {'''
MOUNTED_NEW = '''                        if emit.artifact_mutations.iter().any(Mutation::may_emit_foreign_steps) {
                            let foreign = self.fold_foreign_steps(&emit.artifact_mutations)?;
                            if !foreign.is_empty() {'''
PREPARE_OLD = '''            let mut running = match self.store.snapshot() {
                Ok(snapshot) => snapshot,
                Err(error) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(Self::transaction_fault(FaultOrigin::Plugin, "transaction.member-rejected", format!("{error:?}"))) },
            };
            let mut foreign = Vec::new();
            for op in &ops {
                foreign.extend(op.foreign_steps(&running));
                let outcome = op.diff(&running);
                if !outcome.is_applicable(protocol::MergePolicy::default()) {
                    return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages()))) };
                }
                running = match outcome.diff().apply(&running) {
                    Ok(next) => next,
                    Err(error) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string())) },
                };
            }
'''
PREPARE_NEW = '''            let foreign = match self.fold_foreign_steps(&ops) {
                Ok(foreign) => foreign,
                Err(fault) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(fault) },
            };
'''
FOLD_ANCHOR = '''        pub fn artifact_generation_now(&self) -> semio_framework_job::Generation {
'''
FOLD = '''        /// 🧮️ Folds `ops` over a scratch copy of the live document to collect their foreign steps and to check that each
        /// applies — the one fold the local emit, the mounted emit and `transaction_prepare` share. Every scratch projection
        /// it displaces and every diff it builds is retired explicitly (`MutationDiff::retire_projection`/`retire_cold`, as
        /// `os_vcs::apply_mutation` does): a projection or delta that owns a fail-closed root (procedural's host layout
        /// `OrderedMap`) aborts the guest on a bare drop — every procedural hub/MCP transaction trapped here.
        fn fold_foreign_steps(&self, ops: &[A::Mutation]) -> Result<Vec<protocol::ForeignStep>, Fault> {
            let retire = <<A::Mutation as ::protocol::Mutation<A::Snapshot>>::Diff as ::protocol::MutationDiff<A::Snapshot>>::retire_projection;
            let mut running = self.store.snapshot().map_err(|error| Self::transaction_fault(FaultOrigin::Plugin, "transaction.member-rejected", format!("{error:?}")))?;
            let mut foreign = Vec::new();
            for op in ops {
                foreign.extend(op.foreign_steps(&running));
                let outcome = op.diff(&running);
                let applicable = outcome.is_applicable(protocol::MergePolicy::default());
                let rejected = (!applicable).then(|| format!("mutation outcome was rejected: {:?}", outcome.messages()));
                let (diff, _messages) = outcome.into_parts();
                let applied = applicable.then(|| ::protocol::MutationDiff::apply(&diff, &running));
                ::protocol::MutationDiff::retire_cold(diff);
                match (rejected, applied) {
                    (_, Some(Ok(next))) => retire(std::mem::replace(&mut running, next)),
                    (_, Some(Err(error))) => {
                        retire(running);
                        return Err(Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string()));
                    }
                    (rejected, None) => {
                        retire(running);
                        return Err(Self::transaction_fault(FaultOrigin::App, "transaction.member-rejected", rejected.unwrap_or_default()));
                    }
                }
            }
            retire(running);
            Ok(foreign)
        }

'''


def sdk(text):
    text = lib.replace_once(LOCAL_OLD, LOCAL_NEW)(text)
    text = lib.replace_once(MOUNTED_OLD, MOUNTED_NEW)(text)
    text = lib.replace_once(PREPARE_OLD, PREPARE_NEW)(text)
    if "fn fold_foreign_steps(&self, ops: &[A::Mutation])" not in text:
        assert text.count(FOLD_ANCHOR) == 1, f"fold anchor x{text.count(FOLD_ANCHOR)}"
        text = text.replace(FOLD_ANCHOR, FOLD + FOLD_ANCHOR)
    return text


LAW = '''
//#region 🧊️AgentTransaction
/// 🧊️ LAW: an agent transaction on a generation3d document prepares and commits without trapping. The SDK folds the
/// transaction's ops over a scratch copy of the document, and this document owns a fail-closed layout root, so a bare
/// drop of that scratch copy aborted the guest on EVERY hub/MCP invoke (G12 p33-5: prepare ok, invoke `wasm function 30081`).
#[semio_framework_async_macros::async_test]
async fn an_agent_transaction_prepares_and_commits_without_trapping() {
    let _serial = crate::publication_authority::lock();
    let mut app = app_with_registry().await;
    let definition = super::create_generation3d_app();
    semio_framework_plugin::artifact_app_laws::agent_preview(&mut *app, &definition, "addWidget", &semio_framework::DslValue::Object(Vec::new())).await.expect("addWidget preview");
    let wire = app.take_last_emit_wire().await.expect("addWidget emits its wire");
    let ops = protocol::os_spr::causal::decode_ops_vec(&wire.document).expect("document ops");
    assert!(!ops.is_empty(), "addWidget writes the document");
    let outcome = app.transaction_prepare("agent-addwidget", "", &[], &ops, &wire.children, "addWidget", None).await;
    assert!(outcome.rejection.is_none(), "prepare: {:?}", outcome.rejection);
    app.transaction_commit("agent-addwidget", &semio_framework_plugin::artifact_app_laws::meta("agent")).await.expect("the agent transaction commits");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}
//#endregion 🧊️AgentTransaction
'''


def tests(text):
    if "fn an_agent_transaction_prepares_and_commits_without_trapping" in text:
        return text
    anchor = "//#endregion 🌱️HubGenesis\n"
    assert text.count(anchor) == 1, "HubGenesis region end"
    return text.replace(anchor, anchor + LAW)


lib.run("transaction-retire", [(SDK, sdk), (TESTS, tests)], sys.argv[1:])
