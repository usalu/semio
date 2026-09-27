//! 🧪️ Two-actor interleaving law for range-text typing (ticket 26/09/23 C12): two registered writer replicas on one
//! `MemoryBackbone` type runs at the same point and at different points, exchange their events in both orders, and must agree
//! on one text that holds BOTH runs, each contiguous; one author's undo removes only that author's run on both replicas. The
//! fold of the same splices in the same order by the TS twin (`✂️text-splice/🟦️.ts`, fixture `concurrent`) is the oracle for
//! the relocation itself; this law proves the store path (HLC merge, rewind to the fork point, replay = rebase) keeps it.
use crate::editor::writer::commands::{set_text, text_splice};
use crate::editor::writer::unit_tests::context::main_window_view;
use crate::editor::writer::{WriterCommand, WriterPlayApp};
use crate::writer_text;
use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, paired_registered_apps_with_members, settle_framework_reserved_admission, settle_registered_typed_operation};
use semio_framework_plugin::{ActionMeta, EditorApp, PluginApp, VcsArtifactApp};

type Replica = VcsArtifactApp<EditorApp<WriterPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;

/// ⌨️ One author's view of the text: what it typed into and where its caret is (scalars).
struct Author {
    actor: &'static str,
    view: String,
    caret: usize,
    seq: u64,
}

fn author_meta(actor: &str) -> ActionMeta {
    let mut meta = meta(actor);
    meta.view_state = Some(main_window_view());
    meta
}

fn text_of(replica: &Replica) -> String {
    writer_text(&replica.snapshot().expect("projection"))
}

/// ⌨️ Types `run` at the author's caret, one `textSplice` per keystroke against the author's own view.
async fn type_run(replica: &mut Replica, author: &mut Author, run: &str) {
    for char in run.chars() {
        let chars: Vec<char> = author.view.chars().collect();
        let next: String = chars[..author.caret].iter().copied().chain(std::iter::once(char)).chain(chars[author.caret..].iter().copied()).collect();
        let splice = semio_framework_plugin::TextSplice::from_edit(&author.view, &next, semio_framework_plugin::TEXT_SPLICE_CONTEXT_SCALARS).expect("a keystroke changes the text");
        author.seq += 1;
        let caret_bytes = next.chars().take(author.caret + 1).map(char::len_utf8).sum();
        let command = WriterCommand::TextSplice(text_splice::TextSplice { start: splice.start, deleted: splice.deleted, insert: splice.insert, before: splice.before, after: splice.after, seq: author.seq, anchor: caret_bytes, caret: caret_bytes });
        replica.dispatch_typed(command, &author_meta(author.actor)).await.expect("the keystroke applies");
        settle_registered_typed_operation(replica, meta(author.actor).instance_id).await.expect("the keystroke publishes");
        author.view = next;
        author.caret += 1;
    }
}

/// 🔀️ Both replicas fold each other's events and must agree.
async fn exchange(a: &mut Replica, b: &mut Replica) -> String {
    a.tick_backbone().await.expect("a folds b's events");
    b.tick_backbone().await.expect("b folds a's events");
    a.tick_backbone().await.expect("a folds b's rebased events");
    let (left, right) = (text_of(a), text_of(b));
    assert_eq!(left, right, "both replicas must fold the same text");
    left
}

#[semio_framework_async_macros::async_test]
async fn two_authors_typing_at_once_keep_both_runs_and_undo_only_their_own() {
    let (mut a, mut b) = paired_registered_apps_with_members::<EditorApp<WriterPlayApp>, semio_s_artifact_stdio_semio::SemioMembers, _, _>("mem://writer-concurrent-typing", || async { semio_framework_plugin::App { definition: crate::editor::writer::create_writer_app(), examples: Vec::new() } }).await;
    a.dispatch_typed(WriterCommand::SetText(set_text::SetText { text: "Doc: \nend".into() }), &author_meta("actor-a")).await.expect("a seeds the text");
    settle_registered_typed_operation(&mut a, meta("actor-a").instance_id).await.expect("the seed publishes");
    assert_eq!(exchange(&mut a, &mut b).await, "Doc: \nend");
    let mut alice = Author { actor: "actor-a", view: "Doc: \nend".into(), caret: 5, seq: 0 };
    let mut bob = Author { actor: "actor-b", view: "Doc: \nend".into(), caret: 5, seq: 0 };
    type_run(&mut a, &mut alice, "alpha").await;
    type_run(&mut b, &mut bob, "beta").await;
    let same_point = exchange(&mut a, &mut b).await;
    assert!(same_point.contains("alpha") && same_point.contains("beta"), "both runs typed at the same point survive, each contiguous: {same_point:?}");
    assert_eq!(same_point.chars().count(), "Doc: \nend".chars().count() + 9, "no keystroke is lost or doubled: {same_point:?}");
    alice.view = same_point.clone();
    alice.caret = same_point.find("end").map(|at| same_point[..at].chars().count()).expect("the tail survives");
    bob.view = same_point.clone();
    bob.caret = 0;
    type_run(&mut a, &mut alice, "A").await;
    type_run(&mut b, &mut bob, "B").await;
    let different_points = exchange(&mut a, &mut b).await;
    assert!(different_points.starts_with('B') && different_points.contains("Aend"), "runs at different points land where their authors put them: {different_points:?}");
    let admitted = a.handle_action("undo", None, &author_meta("actor-a")).await.expect("a's undo is admitted");
    settle_framework_reserved_admission(&mut a, admitted).await.expect("a's undo commits");
    settle_registered_typed_operation(&mut a, meta("actor-a").instance_id).await.expect("a's undo publishes");
    let undone = exchange(&mut a, &mut b).await;
    assert!(!undone.contains("Aend") && undone.starts_with('B') && undone.contains("beta"), "a's undo removes only a's own run on both replicas: {undone:?}");
    a.detach_backbone().await.expect("a releases its backbone");
    b.detach_backbone().await.expect("b releases its backbone");
    close_registered_fixture_app(&mut a);
    close_registered_fixture_app(&mut b);
}
