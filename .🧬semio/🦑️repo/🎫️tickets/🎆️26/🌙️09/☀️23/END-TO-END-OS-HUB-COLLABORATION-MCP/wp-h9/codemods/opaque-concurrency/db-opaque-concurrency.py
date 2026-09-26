#!/usr/bin/env python3
"""⚔️ Hub/db half of the opaque-concurrency patch (ticket 26/09/23 H9 session 12, coordinator item (a)).

An envelope whose diff the database cannot interpret (no touched region: every non-pathmap schema) and whose
dependencies do not include the document's last committed mutation (within a batch: its predecessor) was authored
without seeing that mutation; the overlap cannot be ruled out, so it is graded `Warning` (`mutation.clamped`):
`Normal` accepts it with the message, `Vigilant` refuses it. Lands TOGETHER with the guest/store half (a mutation
without declared dependencies is stamped with the store's causal head) — alone it would make a vigilant hub refuse
every write of clients whose dependencies are always empty.

usage: db-opaque-concurrency.py [--reverse] [--dry-run]
"""
import pathlib
import sys

ROOT = next(parent for parent in pathlib.Path(__file__).resolve().parents if (parent / ".git").exists())
ARTIFACT = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs"

EDITS = [
    (ARTIFACT,
     """    durable_group_receipts: HashMap<String, store::durable_group::DurableOwnedGroupJournalReceiptV1>,
""",
     """    durable_group_receipts: HashMap<String, store::durable_group::DurableOwnedGroupJournalReceiptV1>,
    durable_group_edit_ids: HashSet<String>,
"""),
    (ARTIFACT,
     """            durable_group_receipts: HashMap::new(),
""",
     """            durable_group_receipts: HashMap::new(),
            durable_group_edit_ids: HashSet::new(),
"""),
    (ARTIFACT,
     """                        let duplicate = engine.durable_group_receipts.insert(receipt.decision_sha256.clone(), receipt).is_some();
""",
     """                        let duplicate = engine.durable_group_receipts.insert(receipt.decision_sha256.clone(), receipt).is_some();
                        engine.durable_group_edit_ids.insert(record.parent_edit_id().to_string());
"""),
    (ARTIFACT,
     """        self.durable_group_receipts.insert(receipt.decision_sha256.clone(), receipt.clone());
""",
     """        self.durable_group_receipts.insert(receipt.decision_sha256.clone(), receipt.clone());
        self.durable_group_edit_ids.insert(head_edit_id_text);
"""),
    (ARTIFACT,
     """        let head_edit_id = protocol::MutationId(record.parent_edit_id().to_string());
        let next_frontier = Frontier {""",
     """        let head_edit_id = protocol::MutationId(record.parent_edit_id().to_string());
        let head_edit_id_text = head_edit_id.0.clone();
        let next_frontier = Frontier {"""),
    (ARTIFACT,
     """            if !self.applied.contains_key(&dependency.0) && !batch_ids.contains(&dependency.0) {""",
     """            if !self.applied.contains_key(&dependency.0) && !batch_ids.contains(&dependency.0) && !self.durable_group_edit_ids.contains(&dependency.0) {"""),
    (ARTIFACT,
     """//#endregion 🔖️Conflict
""",
     """/// @emoji 👁️‍🗨️ Grades one envelope the database cannot see into: its diff touches no region the database can
/// interpret (every plugin schema is opaque below `db_artifact`) and its dependencies do not include `unseen`, the
/// document's head when it arrived (the frontier head every client is told: a mutation id, or a durable group's edit
/// id), so it was authored without seeing that head and an overlap cannot be ruled out — `Warning`
/// (`mutation.clamped`, which the Shell localizes as a concurrent edit of the same part): `Normal` accepts it with the
/// message, `Vigilant` refuses it (ticket 26/09/23 C10: a vigilant hub accepted a same-field write authored during a
/// 12 s cut, because no client stamped dependencies and no plugin diff is readable here).
fn grade_opaque_concurrency(envelope: &protocol::MutationEnvelope, unseen: &protocol::MutationId) -> protocol::MutationMessage {
    protocol::MutationMessage::warn("mutation.clamped", format!("command {} was authored without seeing concurrent command {}; its fields are opaque to the database, so an overlap cannot be ruled out", envelope.mutation_id.0, unseen.0))
}
//#endregion 🔖️Conflict
"""),
    (ARTIFACT,
     """        for record in db_conflict::ConflictDetector::new().detect(&probe).iter().filter(|record| new_ids.contains(record.command_id.0.as_str()) || new_ids.contains(record.conflicting_with.0.as_str())) {
            messages.push(grade_conflict_record(record).await);
        }
""",
     """        for record in db_conflict::ConflictDetector::new().detect(&probe).iter().filter(|record| new_ids.contains(record.command_id.0.as_str()) || new_ids.contains(record.conflicting_with.0.as_str())) {
            messages.push(grade_conflict_record(record).await);
        }
        let mut seen_head = self.head_edit_id.clone();
        for (envelope, touched, _) in &newly_applied {
            if let Some(unseen) = seen_head.as_ref().filter(|unseen| touched.regions.is_empty() && !envelope.dependencies.contains(unseen)) {
                messages.push(grade_opaque_concurrency(envelope, unseen));
            }
            seen_head = Some(envelope.mutation_id.clone());
        }
"""),
]


def main() -> int:
    reverse = "--reverse" in sys.argv
    dry = "--dry-run" in sys.argv
    texts = {path: path.read_text(encoding="utf-8") for path in {edit[0] for edit in EDITS}}
    failures = 0
    for index, (path, before, after) in enumerate(EDITS):
        find, put = (after, before) if reverse else (before, after)
        count = texts[path].count(find)
        if count != 1:
            print(f"edit {index} {path.name}: anchor found {count} times", file=sys.stderr)
            failures += 1
            continue
        texts[path] = texts[path].replace(find, put, 1)
        print(f"edit {index} {path.parent.name}/{path.name}: ok")
    if failures:
        return 1
    if not dry:
        for path, text in texts.items():
            path.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    sys.exit(main())
