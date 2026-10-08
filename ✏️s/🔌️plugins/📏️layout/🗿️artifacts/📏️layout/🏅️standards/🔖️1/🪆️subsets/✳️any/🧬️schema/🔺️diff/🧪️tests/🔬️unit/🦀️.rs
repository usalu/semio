use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::default_document;
use crate::{FramePatch, PageFramePatched};
use protocol::{DiffAlgebra, Mutation};

fn apply(diff: &LayoutDiff, base: &LayoutSnapshot) -> LayoutSnapshot {
    protocol::apply_diff(diff, base).expect("the diff applies")
}

fn absorbed(first: &LayoutDiff, second: &LayoutDiff) -> LayoutDiff {
    let mut sum = first.clone();
    MutationDiff::absorb(&mut sum, second.clone());
    sum
}

fn page_patch(patch: PagePatch) -> LayoutDiff {
    LayoutDiff { pages: Some(LayoutPagesDelta { patched: vec![LayoutPagePatchEntry { id: "page-1".into(), patch }], ..Default::default() }), ..Default::default() }
}

fn move_frame(frame_id: &str, x: f64) -> LayoutDiff {
    page_patch(PagePatch { frames_patched: vec![PageFramePatched { frame_id: frame_id.into(), patch: FramePatch { x: Some(x), ..Default::default() } }], ..Default::default() })
}

#[semio_framework_async_macros::async_test]
async fn set_data_fields_diff_applies_onto_the_base_snapshot() {
    let base = default_document();
    let operation = crate::mutations::LayoutMutation::ChangeDataFields(crate::mutations::change_data_fields::ChangeDataFields { new_fields: Some(crate::FormDictionary::default()) });
    let diff: LayoutDiff = operation.diff(&base).into_parts().0;
    assert_eq!(apply(&diff, &base).data_fields, Some(crate::FormDictionary::default()));
}

/// 🩹 Two frame patches of one frame fold into one page patch whose application equals the sequence.
#[test]
fn absorb_folds_frame_patches_of_one_frame_into_one_patch() {
    let (first, second) = (move_frame("frame-text-1", 10.0), move_frame("frame-text-1", 20.0));
    let sum = absorbed(&first, &second);
    let delta = sum.pages.as_ref().expect("pages");
    assert_eq!(delta.patched.len(), 1);
    assert_eq!(delta.patched[0].patch.frames_patched.len(), 1);
    assert_eq!(apply(&sum, &default_document()), apply(&second, &apply(&first, &default_document())));
}

/// 🔁️ Structural page patches stay an ordered sequence, so the absorbed diff applies exactly like its parts.
#[test]
fn absorb_keeps_structural_patches_in_order() {
    let base = default_document();
    let frame = base.pages[0].frames[0].clone();
    let removed = page_patch(PagePatch { frame_removed: Some(frame.id().to_string()), ..Default::default() });
    let restored = page_patch(PagePatch { frame_added: Some(crate::PageFrameAdded { frame: frame.clone(), index: Some(0), layer_id: Some("layer-1".into()) }), ..Default::default() });
    let sum = absorbed(&removed, &restored);
    assert_eq!(sum.pages.as_ref().expect("pages").patched.len(), 2);
    assert_eq!(apply(&sum, &base), apply(&restored, &apply(&removed, &base)));
}

/// ⚖️ A page added and then removed is no row at all; a story patched after being added folds into the added story.
#[test]
fn absorb_cancels_added_then_removed_and_folds_patches_into_adds() {
    let base = default_document();
    let page = base.pages[0].clone();
    let fresh = Page { id: "page-9".into(), ..page };
    let added = LayoutDiff { pages: Some(LayoutPagesDelta { added: vec![fresh], ..Default::default() }), ..Default::default() };
    let removed = LayoutDiff { pages: Some(LayoutPagesDelta { removed: vec!["page-9".into()], ..Default::default() }), ..Default::default() };
    assert_eq!(absorbed(&added, &removed).pages, None);
    let story = crate::TextStory { id: "story-9".into(), content: "a".into(), style_runs: Vec::new() };
    let stories = LayoutDiff { stories: Some(LayoutStoriesDelta { added: vec![story], ..Default::default() }), ..Default::default() };
    let edited = LayoutDiff { stories: Some(LayoutStoriesDelta { patched: vec![LayoutStoryPatchEntry { id: "story-9".into(), patch: TextStoryPatch { content: Some("b".into()), style_runs: None } }], ..Default::default() }), ..Default::default() };
    let sum = absorbed(&stories, &edited);
    let delta = sum.stories.as_ref().expect("stories");
    assert_eq!((delta.added.len(), delta.patched.len(), delta.added[0].content.as_str()), (1, 0, "b"));
}

/// 📍️ Positional rows compose by final length: a shrink followed by a growth keeps exactly the rows the growth carries.
#[test]
fn absorb_composes_positional_rows_by_final_length() {
    let run = |start: u64| TextStyleRun { start, end: start + 1, paragraph_style_id: None, character_style_id: None };
    let shrink = TextStyleRunsDelta { len: Some(1), rows: vec![] };
    let grow = TextStyleRunsDelta { len: Some(3), rows: vec![TextStyleRunRow { index: 1, run: run(1) }, TextStyleRunRow { index: 2, run: run(2) }] };
    let sum = absorb_positional(shrink, grow);
    assert_eq!(sum.len, Some(3));
    assert_eq!(sum.rows.len(), 2);
    let base = vec![run(0), run(5), run(6), run(7)];
    assert_eq!(apply_positional(&base, &sum).expect("applies"), vec![run(0), run(1), run(2)]);
}

/// 🧾️ A data-field dictionary created and then deleted leaves no trace.
#[test]
fn absorb_cancels_a_created_then_deleted_dictionary() {
    let created = LayoutDiff { data_fields: Some(LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Created), entries: None }), ..Default::default() };
    let deleted = LayoutDiff { data_fields: Some(LayoutDataFieldsDelta { presence: Some(DataFieldsPresence::Deleted), entries: None }), ..Default::default() };
    assert_eq!(absorbed(&created, &deleted).data_fields, None);
}

/// ↩️ The inverse applied after the diff restores the base, for frame patches, structural page edits, grid, name and print target alike.
#[test]
fn inverse_restores_the_base() {
    let base = default_document();
    let frame = base.pages[0].frames[0].clone();
    let cases = [
        move_frame("frame-text-1", 99.0),
        page_patch(PagePatch { frame_removed: Some(frame.id().to_string()), ..Default::default() }),
        page_patch(PagePatch { name: Some("Renamed".into()), width: Some(10.0), ..Default::default() }),
        LayoutDiff { grid: Some(GridPatch { baseline_grid: Some(9.0), ..Default::default() }), name: Some("Other".into()), ..Default::default() },
        LayoutDiff { print_target: Some(PrintTargetChange { target: Some("cmyk".into()) }), ..Default::default() },
    ];
    for diff in cases {
        let after = apply(&diff, &base);
        assert_ne!(after, base);
        let inverse = DiffAlgebra::inverse(&diff, &base);
        assert_eq!(apply(&inverse, &after), base, "inverse of {diff:?}");
    }
}
