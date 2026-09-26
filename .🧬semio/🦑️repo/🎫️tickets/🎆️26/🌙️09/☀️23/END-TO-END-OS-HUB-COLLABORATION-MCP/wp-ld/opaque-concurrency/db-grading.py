#!/usr/bin/env python3
"""⚔️ LD item 2 (db half, re-derived from H9's `db-opaque-concurrency.py`): the database grades a write only
against the other authors' writes committed after the one its author `observed`, and only when their targets
overlap (or either's fields are opaque and undeclared). Durable group decisions join the recent commit window as
markers a later write can name, and a dependency on a durable group's operation is a known operation.

usage: db-grading.py [--dry-run]
"""
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
ARTIFACT = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs"

HELPERS = '''/// @emoji 🏷️ `CommandTouch` kind of a durable group decision's marker in the recent commit window: it names
/// the decision's edit so a later write can say it observed it, and it is never itself graded.
const DURABLE_GROUP_TOUCH_KIND: &str = "db.durable-group";

/// @emoji 🧷️ The recent-window marker of one committed durable group decision (see [`DURABLE_GROUP_TOUCH_KIND`]).
fn durable_group_touch(edit_id: &str) -> db_conflict::CommandTouch {
    db_conflict::CommandTouch::new(protocol::MutationId(edit_id.to_string()), protocol::ActorId(String::new()), db_conflict::CommandKind::from(DURABLE_GROUP_TOUCH_KIND), protocol::HybridLogicalTimestamp::new(0, 0))
}

/// @emoji 🪟️ Appends one committed touch to the bounded recent commit window, oldest out first.
fn remember_recent_touch(window: &mut VecDeque<db_conflict::CommandTouch>, touch: db_conflict::CommandTouch) {
    if window.len() >= MAX_RECENT_TOUCHES {
        window.pop_front();
    }
    window.push_back(touch);
}

/// @emoji 👁️ Whether `observed` names `touch`: its command itself, or — for a durable group decision's marker —
/// one of the operations its edit folded (`<edit>#<position>`), which reach a replica inside that edit.
fn touch_is_named(touch: &db_conflict::CommandTouch, observed: &str) -> bool {
    observed == touch.command_id.0 || (touch.kind.0 == DURABLE_GROUP_TOUCH_KIND && observed.strip_prefix(touch.command_id.0.as_str()).is_some_and(|position| position.starts_with('#')))
}

/// @emoji ✍️ Whether `touch` wrote document fields: history transitions (undo, redo, checkpoints) and durable
/// group markers never did.
fn touch_writes_fields(touch: &db_conflict::CommandTouch) -> bool {
    touch.kind.0 != DURABLE_GROUP_TOUCH_KIND && touch.kind.0 != protocol::HISTORY_TRANSITION_SCHEMA
}

/// @emoji 🕰️ The writes `envelope` was authored without seeing: in commit order (the recent commit window, then the
/// batch's earlier envelopes), every write after the one it names as `observed` — all of them when it observed
/// nothing in the window — by another actor. A replica stamps each operation it authors with the newest foreign
/// operation it had applied, and replicas receive commits in commit order, so everything up to that one was seen;
/// the author's own writes are its own history. A history transition is never graded.
fn unseen_concurrent_writes<'a>(recent: &'a VecDeque<db_conflict::CommandTouch>, batch: &'a [db_conflict::CommandTouch], envelope: &protocol::MutationEnvelope) -> Vec<&'a db_conflict::CommandTouch> {
    if protocol::is_history_transition(envelope) {
        return Vec::new();
    }
    let window: Vec<&db_conflict::CommandTouch> = recent.iter().chain(batch.iter()).collect();
    let seen = envelope.observed.as_ref().and_then(|observed| window.iter().rposition(|touch| touch_is_named(touch, &observed.0))).map_or(0, |index| index + 1);
    window.into_iter().skip(seen).filter(|touch| touch.actor != envelope.actor && touch_writes_fields(touch)).collect()
}

/// @emoji 🎯️ Whether two declared targets (outermost segment first) can address the same part: an empty target is
/// the whole artifact; otherwise they overlap when they share a segment — conservative for path-shaped targets
/// (`[collection, id]`) and exact for id-set targets.
fn targets_overlap(a: &[String], b: &[String]) -> bool {
    a.is_empty() || b.is_empty() || a.iter().any(|segment| b.contains(segment))
}

/// @emoji ⚖️ Grades `written` — authored without seeing the concurrent write `unseen` — into `mutation.clamped`
/// (contract §C9: region intersection = `Warning`): fields both touch when the database can read both diffs, else
/// their declared targets overlapping. Disjoint parts are no conflict. `Normal` accepts a clamped write with its
/// message, `Vigilant` refuses it (ticket 26/09/23 C10: a vigilant hub accepted a same-field write authored
/// during a 12 s cut, because no plugin diff is readable here and no client said what it had seen).
async fn grade_concurrent_write(written: &db_conflict::CommandTouch, written_target: &[String], unseen: &db_conflict::CommandTouch, unseen_target: &[String]) -> Vec<protocol::MutationMessage> {
    if !written.touched.regions.is_empty() && !unseen.touched.regions.is_empty() {
        let mut messages = Vec::new();
        for record in db_conflict::ConflictDetector::new().detect(&[unseen.clone(), written.clone()]) {
            messages.push(grade_conflict_record(&record).await);
        }
        return messages;
    }
    if !targets_overlap(written_target, unseen_target) {
        return Vec::new();
    }
    vec![protocol::MutationMessage::warn("mutation.clamped", format!("command {} was authored without seeing concurrent command {}, which writes the same part of the document", written.command_id.0, unseen.command_id.0)).at(written_target.to_vec())]
}
//#endregion 🔖️Conflict
'''

EDITS = [
    ("""    durable_group_receipts: HashMap<String, store::durable_group::DurableOwnedGroupJournalReceiptV1>,
    actor_seq: HashMap<String, u64>,""", """    durable_group_receipts: HashMap<String, store::durable_group::DurableOwnedGroupJournalReceiptV1>,
    durable_group_edit_ids: HashSet<String>,
    recent_targets: HashMap<String, Vec<String>>,
    actor_seq: HashMap<String, u64>,"""),
    ("""            durable_group_receipts: HashMap::new(),
            actor_seq: HashMap::new(),""", """            durable_group_receipts: HashMap::new(),
            durable_group_edit_ids: HashSet::new(),
            recent_targets: HashMap::new(),
            actor_seq: HashMap::new(),"""),
    ("""                        let duplicate = engine.durable_group_receipts.insert(receipt.decision_sha256.clone(), receipt).is_some();
""", """                        let duplicate = engine.durable_group_receipts.insert(receipt.decision_sha256.clone(), receipt).is_some();
                        engine.durable_group_edit_ids.insert(record.parent_edit_id().to_string());
"""),
    ("""                            replay_head_edit_id = Some(protocol::MutationId(record.parent_edit_id().to_string()));
""", """                            replay_head_edit_id = Some(protocol::MutationId(record.parent_edit_id().to_string()));
                            remember_recent_touch(&mut engine.recent_touches, durable_group_touch(record.parent_edit_id()));
"""),
    ("""                                    let touch = command_touch(&envelope, &touched);
                                    if engine.recent_touches.len() >= MAX_RECENT_TOUCHES {
                                        engine.recent_touches.pop_front();
                                    }
                                    engine.recent_touches.push_back(touch);
""", """                                    let touch = command_touch(&envelope, &touched);
                                    engine.remember_target(&envelope);
                                    remember_recent_touch(&mut engine.recent_touches, touch);
"""),
    ("""        self.head_edit_id = Some(head_edit_id.clone());
        self.commit_log.push(CommitNotification { frontier: next_frontier, operation_ids: vec![head_edit_id], touched: db_state::TouchedSet::new() });""", """        self.head_edit_id = Some(head_edit_id.clone());
        self.durable_group_edit_ids.insert(head_edit_id.0.clone());
        remember_recent_touch(&mut self.recent_touches, durable_group_touch(&head_edit_id.0));
        self.commit_log.push(CommitNotification { frontier: next_frontier, operation_ids: vec![head_edit_id], touched: db_state::TouchedSet::new() });"""),
    ("""            if !self.applied.contains_key(&dependency.0) && !batch_ids.contains(&dependency.0) {""", """            if !self.applied.contains_key(&dependency.0) && !batch_ids.contains(&dependency.0) && !self.names_durable_group_operation(&dependency.0) {"""),
    ("""        let new_ids: HashSet<&str> = newly_applied.iter().map(|(envelope, _, _)| envelope.mutation_id.0.as_str()).collect();
        let batch_touches: Vec<db_conflict::CommandTouch> = newly_applied.iter().map(|(envelope, touched, _)| command_touch(envelope, touched)).collect();
        let probe: Vec<db_conflict::CommandTouch> = self.recent_touches.iter().cloned().chain(batch_touches.iter().cloned()).collect();
        // 🔀️ `grade_conflict_record` genuinely awaits (`protocol::MutationMessage::warn`/`fatal`),
        // so this can't stay an `Iterator::map` chain (R10 residue shape 1: `.await` inside a sync
        // closure) — hoisted into an explicit async loop instead.
        let mut messages: Vec<protocol::MutationMessage> = Vec::new();
        for record in db_conflict::ConflictDetector::new().detect(&probe).iter().filter(|record| new_ids.contains(record.command_id.0.as_str()) || new_ids.contains(record.conflicting_with.0.as_str())) {
            messages.push(grade_conflict_record(record).await);
        }
""", """        let batch_touches: Vec<db_conflict::CommandTouch> = newly_applied.iter().map(|(envelope, touched, _)| command_touch(envelope, touched)).collect();
        let mut messages: Vec<protocol::MutationMessage> = Vec::new();
        for (index, (envelope, _, _)) in newly_applied.iter().enumerate() {
            for unseen in unseen_concurrent_writes(&self.recent_touches, &batch_touches[..index], envelope) {
                let unseen_target = self.recent_targets.get(&unseen.command_id.0).map(Vec::as_slice).or_else(|| newly_applied[..index].iter().find(|(earlier, _, _)| earlier.mutation_id == unseen.command_id).map(|(earlier, _, _)| earlier.target.as_slice())).unwrap_or(&[]);
                messages.extend(grade_concurrent_write(&batch_touches[index], &envelope.target, unseen, unseen_target).await);
            }
        }
"""),
    ("""        for touch in batch_touches {
            if self.recent_touches.len() >= MAX_RECENT_TOUCHES {
                self.recent_touches.pop_front();
            }
            self.recent_touches.push_back(touch);
        }
""", """        for (envelope, _, _) in &newly_applied {
            self.remember_target(envelope);
        }
        for touch in batch_touches {
            remember_recent_touch(&mut self.recent_touches, touch);
        }
"""),
    ("""    /// @emoji 🚦️ The full command pipeline: admit → dedupe → base-resolve/deps → authz → validate →""", """    /// @emoji 👁️ Whether `dependency` names an operation a committed durable group decision folded: the
    /// decision's edit itself, or `<edit>#<position>`.
    fn names_durable_group_operation(&self, dependency: &str) -> bool {
        self.durable_group_edit_ids.contains(dependency) || dependency.rsplit_once('#').is_some_and(|(edit, position)| !position.is_empty() && position.bytes().all(|byte| byte.is_ascii_digit()) && self.durable_group_edit_ids.contains(edit))
    }

    /// @emoji 🎯️ Keeps `envelope`'s declared target while its touch is in the recent commit window, so a later
    /// concurrent write can be graded against it; targets that left the window are dropped with it.
    fn remember_target(&mut self, envelope: &protocol::MutationEnvelope) {
        if self.recent_targets.len() >= MAX_RECENT_TOUCHES {
            let window: HashSet<&str> = self.recent_touches.iter().map(|touch| touch.command_id.0.as_str()).collect();
            self.recent_targets.retain(|command, _| window.contains(command.as_str()));
        }
        self.recent_targets.insert(envelope.mutation_id.0.clone(), envelope.target.clone());
    }

    /// @emoji 🚦️ The full command pipeline: admit → dedupe → base-resolve/deps → authz → validate →"""),
    ("""//#endregion 🔖️Conflict
""", HELPERS),
    ("""//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
""", """//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/⚔️concurrent-write/🦀️.rs"]
mod concurrent_write_tests;
"""),
]


def main() -> int:
    dry = "--dry-run" in sys.argv
    text = ARTIFACT.read_text(encoding="utf-8")
    failures = 0
    for index, (before, after) in enumerate(EDITS):
        count = text.count(before)
        if count != 1:
            print(f"edit {index}: anchor found {count} times: {before[:70]!r}", file=sys.stderr)
            failures += 1
            continue
        text = text.replace(before, after, 1)
    if failures:
        return 1
    if not dry:
        ARTIFACT.write_text(text, encoding="utf-8")
    print(f"{len(EDITS)} edits ok{' (dry run)' if dry else ''}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
