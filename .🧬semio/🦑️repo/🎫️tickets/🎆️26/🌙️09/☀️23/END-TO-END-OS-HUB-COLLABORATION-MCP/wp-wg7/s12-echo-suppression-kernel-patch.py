#!/usr/bin/env python3
"""🔁️ WG7 session 12 — late joiners see no history: the kernel half (native + wasm32 document actors).

Root cause (run s12i, decoded frames): the hub stamps its hello catch-up tail with the RECEIVING socket's actor, and both Rust actors
drop every `Commands` frame whose origin equals their own socket actor, so a joiner never applies the existing document. Coordinator
decision: echo suppression is by operation identity, never by frame origin — a replica drops an envelope only when its operation id is
one it authored or already applied; frames are never discarded whole. Schema `🏪️store/🔄️sync/🧬️schema/document-echo-suppression`,
fixture `🏪️store/🧫️fixtures/document-echo-suppression-v1` (walked by the TS twin now, by this Rust law after landing).

Kernel is guest-linked and frozen (rules 20/21/24): apply only in the landing window. Dry run by default; `--apply` writes. Every
replacement asserts its anchor occurs exactly once.
"""

import difflib
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SYNC = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs"
LAW = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️document-echo-suppression/🦀️.rs"

REGION = '''//#endregion 🔖️DocumentLinkShortage

//#region 🔁️DocumentEchoSuppression
/// @emoji 🔁️ The envelopes of one server `Commands` frame a replica applies, in order: echo suppression is by operation identity,
/// never by frame origin. An envelope is applied unless its id is one this replica authored or already applied, and every admitted id
/// is recorded in `applied`. A frame is never discarded whole — the hub's hello catch-up tail carries anyone's edits, the joiner's own
/// earlier device included (ticket 26/09/23 session 12, run s12i: late joiners saw no history). Schema
/// `🧬️schema/document-echo-suppression`; law `🧫️fixtures/document-echo-suppression-v1`; TS twin `admitRemoteEnvelopes`.
pub fn admit_remote_envelopes(applied: &mut std::collections::HashSet<String>, envelopes: impl IntoIterator<Item = MutationEnvelope>) -> Vec<MutationEnvelope> {
    envelopes.into_iter().filter(|envelope| applied.insert(envelope.mutation_id.0.clone())).collect()
}

/// @emoji 🔁️ Records the operations this replica authored, so their echo is never applied a second time.
pub fn note_authored_envelopes(applied: &mut std::collections::HashSet<String>, envelopes: &[MutationEnvelope]) {
    applied.extend(envelopes.iter().map(|envelope| envelope.mutation_id.0.clone()));
}
//#endregion 🔁️DocumentEchoSuppression'''

NATIVE_COMMANDS_OLD = '''                ServerFrame::Commands { envelopes, origin, frontier } => {
                    if self.artifact_bootstrap.is_some() {
                        self.fail_artifact_bootstrap("tail arrived before artifact bootstrap completion").await;
                        return;
                    }
                    if self.socket_actor.as_deref() != Some(origin.0.as_str()) {
                        let converted: Vec<MutationEnvelope> = envelopes.into_iter().filter(|envelope| !self.known_op_ids.contains(&envelope.mutation_id.0)).collect();
                        if !converted.is_empty() {
                            self.persist_operations(&converted).await;
                            for envelope in &converted {
                                self.known_op_ids.insert(envelope.mutation_id.0.clone());
                            }
                            if !self.deliver_remote_operations(converted).await {
                                self.fail_artifact_bootstrap("artifact tail could not be installed").await;
                                return;
                            }
                        }
                    }
                    self.server_frontier = Some(frontier);'''

NATIVE_COMMANDS_NEW = '''                ServerFrame::Commands { envelopes, frontier, .. } => {
                    if self.artifact_bootstrap.is_some() {
                        self.fail_artifact_bootstrap("tail arrived before artifact bootstrap completion").await;
                        return;
                    }
                    let persisted = &self.known_op_ids;
                    let fresh = admit_remote_envelopes(&mut self.applied_op_ids, envelopes.into_iter().filter(|envelope| !persisted.contains(&envelope.mutation_id.0)));
                    if !fresh.is_empty() {
                        self.persist_operations(&fresh).await;
                        if !self.deliver_remote_operations(fresh).await {
                            self.fail_artifact_bootstrap("artifact tail could not be installed").await;
                            return;
                        }
                    }
                    self.server_frontier = Some(frontier);'''

WASM_COMMANDS_OLD = '''                ServerFrame::Commands { envelopes, origin, frontier } => {
                    if self.artifact_bootstrap.is_some() {
                        self.fail_artifact_bootstrap("tail arrived before artifact bootstrap completion");
                        return;
                    }
                    if self.socket_actor.as_deref() != Some(origin.0.as_str()) && !self.deliver_remote_operations(envelopes).await {
                        self.fail_artifact_bootstrap("artifact tail could not be installed");
                        return;
                    }'''

WASM_COMMANDS_NEW = '''                ServerFrame::Commands { envelopes, frontier, .. } => {
                    if self.artifact_bootstrap.is_some() {
                        self.fail_artifact_bootstrap("tail arrived before artifact bootstrap completion");
                        return;
                    }
                    let fresh = admit_remote_envelopes(&mut self.applied_op_ids, envelopes);
                    if !self.deliver_remote_operations(fresh).await {
                        self.fail_artifact_bootstrap("artifact tail could not be installed");
                        return;
                    }'''

APPLIED_DOC = "        /// @emoji 🔁️ Every operation id this replica authored or applied — the echo filter ([`admit_remote_envelopes`]).\n"

SYNC_EDITS = [
    ("//#endregion 🔖️DocumentLinkShortage", REGION),
    ("        known_op_ids: HashSet<String>,\n", "        known_op_ids: HashSet<String>,\n" + APPLIED_DOC + "        applied_op_ids: HashSet<String>,\n"),
    ("                known_op_ids: HashSet::new(),\n", "                known_op_ids: HashSet::new(),\n                applied_op_ids: HashSet::new(),\n"),
    (
        """        async fn relay_operations_to_hub(&mut self, envelopes: &[MutationEnvelope]) {
            if envelopes.is_empty() {
                return;
            }
""",
        """        async fn relay_operations_to_hub(&mut self, envelopes: &[MutationEnvelope]) {
            if envelopes.is_empty() {
                return;
            }
            note_authored_envelopes(&mut self.applied_op_ids, envelopes);
""",
    ),
    (NATIVE_COMMANDS_OLD, NATIVE_COMMANDS_NEW),
    (
        """        outbox: Vec<MutationEnvelope>,
        document_backbone_retention: DocumentBackboneRetentionV1,
        next_batch_id: u64,
        next_local_rejection_batch_id: u64,
        hlc_seed: Option<u64>,
""",
        """        outbox: Vec<MutationEnvelope>,
""" + APPLIED_DOC + """        applied_op_ids: std::collections::HashSet<String>,
        document_backbone_retention: DocumentBackboneRetentionV1,
        next_batch_id: u64,
        next_local_rejection_batch_id: u64,
        hlc_seed: Option<u64>,
""",
    ),
    ("\n            outbox: Vec::new(),\n", "\n            outbox: Vec::new(),\n            applied_op_ids: std::collections::HashSet::new(),\n"),
    (
        """        async fn relay_operations(&mut self, envelopes: &[MutationEnvelope]) {
            if envelopes.is_empty() {
                return;
            }
""",
        """        async fn relay_operations(&mut self, envelopes: &[MutationEnvelope]) {
            if envelopes.is_empty() {
                return;
            }
            note_authored_envelopes(&mut self.applied_op_ids, envelopes);
""",
    ),
    (WASM_COMMANDS_OLD, WASM_COMMANDS_NEW),
    (
        """#[cfg(test)]
#[path = "🧪️tests/🔬️document-link-shortage/🦀️.rs"]
mod document_link_shortage_tests;""",
        """#[cfg(test)]
#[path = "🧪️tests/🔬️document-link-shortage/🦀️.rs"]
mod document_link_shortage_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️document-echo-suppression/🦀️.rs"]
mod document_echo_suppression_tests;""",
    ),
]

LAW_SOURCE = '''//! 🔁️ Language-agnostic echo-suppression fixture — Rust runner. The same
//! `🏪️store/🧫️fixtures/document-echo-suppression-v1/🔣️.json` drives the React twin (`admitRemoteEnvelopes` in
//! `🛍️products/💻️os/🟦️.ts`), so the native actor, the browser actor and the React worker suppress echoes by operation identity and
//! never discard a frame by its origin (ticket 26/09/23 session 12, run s12i: late joiners saw no history).

use super::{admit_remote_envelopes, note_authored_envelopes};
use crate::os_spr::{ActorId, ArtifactId, MutationEnvelope, MutationId};

const FIXTURE: &str = include_str!("../../../🧫️fixtures/document-echo-suppression-v1/🔣️.json");

fn envelope(value: &serde_json::Value) -> MutationEnvelope {
    MutationEnvelope {
        mutation_id: MutationId(value["mutationId"].as_str().expect("fixture mutationId").into()),
        document_id: ArtifactId("artifact-echo".into()),
        actor: ActorId(value["actor"].as_str().expect("fixture actor").into()),
        dependencies: Vec::new(),
        diff: crate::os_spr::ArtifactDiff { schema: crate::os_spr::SchemaId("demo/v1".into()), payload: vec![1] },
        inverse: crate::os_spr::InverseMutation { schema: crate::os_spr::SchemaId("demo/v1".into()), payload: vec![2] },
        timestamp: crate::os_spr::HybridLogicalTimestamp { actor: 1, physical_ms: 2, logical: 3 },
    }
}

#[test]
fn every_vector_applies_by_operation_identity_and_never_discards_a_frame_by_origin() {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("echo suppression fixture json");
    let mut frames = 0;
    for vector in fixture["vectors"].as_array().expect("vectors") {
        let id = vector["id"].as_str().expect("vector id");
        let mut applied = std::collections::HashSet::new();
        for (index, step) in vector["steps"].as_array().expect("steps").iter().enumerate() {
            if let Some(authored) = step.get("authored") {
                let authored: Vec<MutationEnvelope> = authored.as_array().expect("authored").iter().map(|id| envelope(&serde_json::json!({ "mutationId": id, "actor": vector["socketActor"] }))).collect();
                note_authored_envelopes(&mut applied, &authored);
                continue;
            }
            let envelopes: Vec<MutationEnvelope> = step["frame"]["envelopes"].as_array().expect("frame envelopes").iter().map(envelope).collect();
            let admitted: Vec<String> = admit_remote_envelopes(&mut applied, envelopes).into_iter().map(|envelope| envelope.mutation_id.0).collect();
            let expected: Vec<String> = step["expectApplied"].as_array().expect("expectApplied").iter().map(|id| id.as_str().expect("id").to_string()).collect();
            assert_eq!(admitted, expected, "{id} step {index}");
            frames += 1;
        }
    }
    assert!(frames >= 7, "the fixture walks relay and tail frames ({frames})");
}

#[test]
fn neither_actor_reads_a_frame_origin_to_suppress_an_echo() {
    let source = include_str!("../../🦀️.rs");
    assert!(!source.contains("Some(origin.0.as_str())"), "no Commands frame is discarded by its origin");
    assert_eq!(source.matches("admit_remote_envelopes(&mut self.applied_op_ids").count(), 2, "both actors admit by operation identity");
    assert_eq!(source.matches("note_authored_envelopes(&mut self.applied_op_ids, envelopes);").count(), 2, "both actors record what they author");
}
'''


def replaced(path, source, edits):
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.name}: {old[:90]!r}")
        source = source.replace(old, new)
    return source


def main():
    apply = "--apply" in sys.argv
    before = SYNC.read_text(encoding="utf-8")
    after = replaced(SYNC, before, SYNC_EDITS)
    sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), SYNC.name, SYNC.name + " (patched)", n=1))
    print(f"\n+++ new law {LAW.relative_to(ROOT)} ({len(LAW_SOURCE.splitlines())} lines)")
    if apply:
        SYNC.write_text(after, encoding="utf-8")
        LAW.parent.mkdir(parents=True, exist_ok=True)
        LAW.write_text(LAW_SOURCE, encoding="utf-8")
    print(f"{'APPLIED' if apply else 'DRY RUN'}: 1 file patched, 1 law")


if __name__ == "__main__":
    main()
