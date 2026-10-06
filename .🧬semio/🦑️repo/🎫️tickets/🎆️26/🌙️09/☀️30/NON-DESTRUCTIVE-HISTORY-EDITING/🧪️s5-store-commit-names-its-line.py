#!/usr/bin/env python3
"""🪴️ S5-STORE wave CL (found by `viewer_head_tests::the_viewer_head_corpus_matches_two_stores` once the renumber fix let it
run further, 2026-10-05 10:21): a checkpoint committed on an alternative names that alternative (`Commit.line_id`), and the
history fold refuses a commit whose line it does not list — but the `Commit` event declared only its operations and its
parent checkpoint as dependencies, never the `Branch` that registered the line. A replica that received the commit's
operations and parent before the registration applied the commit and its ingest was refused:

    ValidationFailed("malformed history fold at offset 0: commit ck-… names unknown alternative alternative-…")

Causality is declared by the author: a commit on a registered line now depends on the line's registration, so the causal
DAG holds it pending until the replica lists the line. One more dependency id on newly authored `Commit` events; no codec,
wire or persisted format changes.

Parts: `rule` (store, two hunks), `law` (one focused viewer-head law; the corpus law is the wide one).
Every hunk is keyed on an anchor that must occur exactly once. Idempotent. `--check` writes nothing.

    python3 🧪️s5-store-commit-names-its-line.py [--check] [--only rule|law] [--revert]
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
        "store:alternative-origin",
        '''            .map(|envelope| envelope.mutation_id.clone())
    }

    /// ✉️ `edit`'s operations as causal wire envelopes — one per forward operation.
''',
        '''            .map(|envelope| envelope.mutation_id.clone())
    }

    /// 🫚️ The `Branch` transition that registered `alternative_id` — so a commit on that line depends on it causally and no
    /// replica folds the commit before it lists the line. The trunk has no registration.
    fn alternative_origin(&self, alternative_id: &str) -> Option<MutationId> {
        self.envelope
            .transitions
            .iter()
            .rev()
            .find(|envelope| matches!(crate::os_spr::history_transition_from_envelope(envelope), Ok(Some(crate::os_spr::HistoryTransition::Branch { alternative_id: registered, .. })) if registered == alternative_id))
            .map(|envelope| envelope.mutation_id.clone())
    }

    /// ✉️ `edit`'s operations as causal wire envelopes — one per forward operation.
''',
    ),
    (
        STORE,
        "store:commit-dependency",
        '''        dependencies.extend(parent_id.as_deref().and_then(|parent_id| self.checkpoint_origin(parent_id)));
        let authors = authors.into_iter()''',
        '''        dependencies.extend(parent_id.as_deref().and_then(|parent_id| self.checkpoint_origin(parent_id)));
        dependencies.extend(self.envelope.active_alternative_id.as_deref().and_then(|line_id| self.alternative_origin(line_id)));
        let authors = authors.into_iter()''',
    ),
]

LAW = [
    (
        VIEWER,
        "law:commit-waits",
        '''//#endregion 🧪️FinalizeLaws
''',
        '''
/// 🪴️ A checkpoint committed on an alternative depends causally on the alternative's registration: its `Commit` names the
/// `Branch` that listed the line among its dependencies. A replica that takes the log in any rotation or in reverse
/// therefore holds the commit back until it lists the line — it never refuses the fold — and shows the line's
/// registration, checkpoints and projection exactly as the author does.
#[semio_framework_async_macros::async_test]
async fn a_commit_on_an_alternative_waits_for_the_alternative() {
    let mut author = fresh("viewer-commit-line", Some(0)).await;
    apply(&mut author, vec![DemoMutation::SetN(SetN { n: 1 })]).await;
    author.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("a trunk checkpoint");
    author.dispatch(ArtifactCommand::CreateAlternative { name: "side".into() }).await.expect("an alternative");
    apply(&mut author, vec![DemoMutation::AddN(AddN { delta: 2 })]).await;
    author.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }).await.expect("a checkpoint on the alternative");
    let side = author.envelope().active_alternative_id.clone().expect("the author stands on the alternative");
    let log = author.event_log().expect("log");
    let transitions: Vec<(MutationId, Vec<MutationId>, crate::os_spr::HistoryTransition)> =
        log.iter().filter_map(|event| crate::os_spr::history_transition_from_envelope(event).ok().flatten().map(|transition| (event.mutation_id.clone(), event.dependencies.clone(), transition))).collect();
    let registration = transitions.iter().find(|(_, _, transition)| matches!(transition, crate::os_spr::HistoryTransition::Branch { alternative_id, .. } if *alternative_id == side)).map(|(id, _, _)| id.clone()).expect("the registration is in the log");
    let (_, dependencies, _) = transitions.iter().find(|(_, _, transition)| matches!(transition, crate::os_spr::HistoryTransition::Commit(checkpoint) if checkpoint.line_id.as_deref() == Some(side.as_str()))).expect("the commit on the alternative is in the log");
    assert!(dependencies.contains(&registration), "the commit names its line's registration among its dependencies");
    let registrations = |store: &DemoStore| store.envelope().vcs.alternatives.iter().map(|alternative| (alternative.id.clone(), alternative.name.clone(), alternative.checkpoint_ids.clone())).collect::<Vec<_>>();
    let mut arrivals = vec![log.iter().rev().cloned().collect::<Vec<_>>()];
    arrivals.extend((1..log.len()).map(|rotation| {
        let mut rotated = log.clone();
        rotated.rotate_left(rotation);
        rotated
    }));
    for (arrival, events) in arrivals.into_iter().enumerate() {
        let mut replica = fresh("viewer-commit-line", Some(0)).await;
        deliver(&mut replica, events).await;
        assert_eq!(registrations(&replica), registrations(&author), "arrival {arrival}: the line and its checkpoints are shared");
        replica.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: side.clone() }).await.expect("a local switch");
        assert_eq!(replica.snapshot_ref().n, Some(3), "arrival {arrival}: the line projects as its author's");
    }
}
//#endregion 🧪️FinalizeLaws
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
        applied = new in text
        if applied != revert:
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
