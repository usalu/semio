#!/usr/bin/env python3
"""🎏️ S5-STORE wave LO (found by `viewer_head_tests::the_viewer_head_corpus_matches_two_stores`, 2026-10-05 16:21, once wave CL
let the corpus law reach its fourth case): a live store listed its changes, checkpoints and alternatives in ARRIVAL order.

`adopt_history_facts` materializes the history fold into the three ledgers by keeping every known fact where it stands and
appending the new ones. The fold lists its facts in log order — events sorted by `(hybrid clock, id)`, a pure function of the
log — and that is the order a reload shows (`project_envelope_history`). A replica that received a later event before an
earlier one therefore listed, say, `["", "blue", "red"]` where its own reload and every peer list `["", "red", "blue"]`;
positions in the checkpoint ledger drifted the same way.

The ledgers now follow the fold's order: after the facts are adopted, entries are exchanged in place (slots, keys and
reservations stay where they are). One comparison pass when the order already holds, which is every local authoring.

Parts: `rule` (store, two hunks), `law` (one focused viewer-head law; the corpus law is the wide one).
Every hunk is keyed on an anchor that must occur exactly once. Idempotent. `--check` writes nothing.

    python3 🧪️s5-store-ledgers-follow-log-order.py [--check] [--only rule|law] [--revert]
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
STORE_DIR = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
STORE = f"{STORE_DIR}/🦀️.rs"
VIEWER = f"{STORE_DIR}/🧪️tests/🧪️viewer-head/🦀️.rs"

RULE = [
    (
        STORE,
        "store:order-after-adoption",
        '''            self.insert_reserved_alternative_history(reservation, Alternative { id: alternative.id.clone(), name: alternative.name.clone(), checkpoint_ids: alternative.checkpoint_ids.clone() })?;
        }
        Ok(())
    }
''',
        '''            self.insert_reserved_alternative_history(reservation, Alternative { id: alternative.id.clone(), name: alternative.name.clone(), checkpoint_ids: alternative.checkpoint_ids.clone() })?;
        }
        order_ledger_as(&mut self.envelope.vcs.changes, |change: &Change| change.id.as_str(), fold.changes.iter().map(|change| change.id.as_str()));
        order_ledger_as(&mut self.envelope.vcs.checkpoints, |checkpoint: &Checkpoint| checkpoint.id.as_str(), fold.checkpoints.iter().map(|checkpoint| checkpoint.id.as_str()));
        order_ledger_as(&mut self.envelope.vcs.alternatives, |alternative: &Alternative| alternative.id.as_str(), fold.alternatives.iter().map(|alternative| alternative.id.as_str()));
        Ok(())
    }
''',
    ),
    (
        STORE,
        "store:order-ledger-as",
        '''/// 🤝️ How many leading ids `live` and `next` share.
''',
        '''/// 🎏️ Lists `ledger`'s facts in the order `order` names them — the fold's order, a pure function of the log, whatever order
/// their events reached this replica in — by exchanging entries in place: slots, keys and reservations stay where they are.
/// One comparison pass when the order already holds.
fn order_ledger_as<'order, T>(ledger: &mut ArtifactHistoryLedger<T>, id: impl Fn(&T) -> &str, order: impl Iterator<Item = &'order str> + Clone) {
    if ledger.iter().map(&id).eq(order.clone()) {
        return;
    }
    let mut entries: Vec<&mut T> = ledger.iter_mut().collect();
    for (position, expected) in order.enumerate() {
        if let Some(found) = (position..entries.len()).find(|candidate| id(&*entries[*candidate]) == expected).filter(|found| *found != position) {
            let (head, tail) = entries.split_at_mut(found);
            std::mem::swap(&mut *head[position], &mut *tail[0]);
        }
    }
}

/// 🤝️ How many leading ids `live` and `next` share.
''',
    ),
]

LAW = [
    (
        VIEWER,
        "law:log-order",
        '''//#region 🧪️FinalizeLaws
''',
        '''//#region 🧪️FinalizeLaws
/// 🎏️ The history ledgers list their facts in log order, never in arrival order: two replicas each commit a checkpoint and
/// register an alternative, the second after it took the first's events. A replica that takes the whole log reversed or in
/// any rotation lists the changes, checkpoints and alternatives exactly as its own persisted pair restores them — the
/// trunk, then `red`, then `blue` — and so do both authors.
#[semio_framework_async_macros::async_test]
async fn the_ledgers_list_their_facts_in_log_order_whatever_order_the_events_arrived_in() {
    let mut a = fresh("viewer-ledger-order", Some(0)).await;
    apply(&mut a, vec![DemoMutation::SetN(SetN { n: 1 })]).await;
    a.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("a's checkpoint");
    a.dispatch(ArtifactCommand::CreateAlternative { name: "red".into() }).await.expect("a's alternative");
    let mut b = fresh("viewer-ledger-order", Some(0)).await;
    deliver(&mut b, a.event_log().expect("log")).await;
    apply(&mut b, vec![DemoMutation::AddN(AddN { delta: 2 })]).await;
    b.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("b's checkpoint");
    b.dispatch(ArtifactCommand::CreateAlternative { name: "blue".into() }).await.expect("b's alternative");
    deliver(&mut a, b.event_log().expect("log")).await;
    let facts = |store: &DemoStore| {
        let vcs = &store.envelope().vcs;
        (vcs.changes.iter().map(|change| change.id.clone()).collect::<Vec<_>>(), vcs.checkpoints.iter().map(|checkpoint| checkpoint.id.clone()).collect::<Vec<_>>(), vcs.alternatives.iter().map(|alternative| (alternative.id.clone(), alternative.name.clone())).collect::<Vec<_>>())
    };
    let expected = facts(&reloaded(&a, "spr").await);
    assert_eq!(expected.2.iter().map(|(_, name)| name.as_str()).collect::<Vec<_>>(), ["", "red", "blue"], "the log lists the trunk, then the alternatives as they were registered");
    assert_eq!((expected.0.len(), expected.1.len()), (2, 2));
    assert_eq!((facts(&a), facts(&b)), (expected.clone(), facts(&reloaded(&b, "spr").await)), "each author lists what its own pair restores");
    let log = a.event_log().expect("the whole log");
    let mut arrivals = vec![log.iter().rev().cloned().collect::<Vec<_>>()];
    arrivals.extend((1..log.len()).map(|rotation| {
        let mut rotated = log.clone();
        rotated.rotate_left(rotation);
        rotated
    }));
    for (arrival, events) in arrivals.into_iter().enumerate() {
        let mut replica = fresh("viewer-ledger-order", Some(0)).await;
        deliver(&mut replica, events).await;
        assert_eq!(facts(&replica), expected, "arrival {arrival}: the ledgers follow the log");
        assert_eq!(facts(&reloaded(&replica, "spr").await), expected, "arrival {arrival}: and so does the replica's own pair");
    }
}

''',
    ),
]


def main():
    arguments = sys.argv[1:]
    only = arguments[arguments.index("--only") + 1] if "--only" in arguments else None
    revert = "--revert" in arguments
    hunks = [hunk for part, part_hunks in (("rule", RULE), ("law", LAW)) if only in (None, part) for hunk in part_hunks]
    texts, pending = {}, []
    for path, name, old, new in hunks:
        text = texts.setdefault(path, (ROOT / path).read_text(encoding="utf-8"))
        before, after = (new, old) if revert else (old, new)
        if (new in text) != revert:
            continue
        if text.count(before) != 1:
            raise SystemExit(f"{name}: anchor occurs {text.count(before)} times in {path} (expected 1): re-derive the wave")
        texts[path] = text.replace(before, after)
        pending.append(name)
    print("pending: " + (", ".join(pending) if pending else "none"))
    if "--check" in arguments or not pending:
        return
    for path, text in texts.items():
        (ROOT / path).write_text(text, encoding="utf-8")
    print(f"applied {len(pending)} hunks in {len(texts)} files")


main()
