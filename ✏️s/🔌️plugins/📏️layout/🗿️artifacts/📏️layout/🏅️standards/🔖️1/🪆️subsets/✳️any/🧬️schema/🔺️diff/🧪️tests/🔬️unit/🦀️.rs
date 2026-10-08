use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::default_document;
use crate::standards::v1::subsets::any::schema::diff::{PageFramesDelta, PagePatch};
use crate::{FramePatch};
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
    LayoutDiff { pages: Some(LayoutPagesDelta { modified: vec![LayoutPagesModification { id: "page-1".into(), patch }], ..Default::default() }), ..Default::default() }
}

fn move_frame(frame_id: &str, x: f64) -> LayoutDiff {
    page_patch(PagePatch { frames: PageFramesDelta::modification(frame_id, FramePatch { x: Some(x), ..Default::default() }), ..Default::default() })
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
    assert_eq!(delta.modified.len(), 1);
    assert_eq!(delta.modified[0].patch.frames.modified.len(), 1);
    assert_eq!(apply(&sum, &default_document()), apply(&second, &apply(&first, &default_document())));
}

/// 🔁️ A frame removed and inserted again composes into one page patch that applies exactly like its parts.
#[test]
fn absorb_keeps_structural_patches_in_order() {
    let base = default_document();
    let frame = base.pages[0].frames[0].clone();
    let removed = page_patch(PagePatch { frames: PageFramesDelta::removal_by_id(frame.id().to_string(), 0), ..Default::default() });
    let restored = page_patch(PagePatch { frames: PageFramesDelta::insertion(0, frame.clone()), ..Default::default() });
    let sum = absorbed(&removed, &restored);
    assert_eq!(sum.pages.as_ref().expect("pages").modified.len(), 1);
    assert_eq!(apply(&sum, &base), apply(&restored, &apply(&removed, &base)));
}

/// ⚖️ A page added and then removed is no row at all; a story patched after being added keeps its own modified entry.
#[test]
fn absorb_cancels_added_then_removed_and_folds_patches_into_adds() {
    let base = default_document();
    let page = base.pages[0].clone();
    let fresh = Page { id: "page-9".into(), ..page };
    let at = base.pages.len();
    let added = LayoutDiff { pages: Some(LayoutPagesDelta { inserted: vec![LayoutPageInsertion { index: at, row: fresh }], ..Default::default() }), ..Default::default() };
    let removed = LayoutDiff { pages: Some(LayoutPagesDelta { removed: vec![LayoutPageRemoval { id: "page-9".into(), index: at }], ..Default::default() }), ..Default::default() };
    assert_eq!(absorbed(&added, &removed).pages, None);
    let story = crate::TextStory { id: "story-9".into(), content: "a".into(), style_runs: Vec::new() };
    let stories = LayoutDiff { stories: Some(LayoutStoriesDelta { inserted: vec![LayoutStoryInsertion { index: base.stories.len(), row: story }], ..Default::default() }), ..Default::default() };
    let edited = LayoutDiff { stories: Some(LayoutStoriesDelta { modified: vec![LayoutStoriesModification { id: "story-9".into(), patch: TextStoryPatch { content: Some("b".into()), style_runs: None } }], ..Default::default() }), ..Default::default() };
    let sum = absorbed(&stories, &edited);
    let delta = sum.stories.as_ref().expect("stories");
    assert_eq!((delta.inserted.len(), delta.modified.len(), delta.inserted[0].row.content.as_str()), (1, 1, "a"));
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
        page_patch(PagePatch { frames: PageFramesDelta::removal_by_id(frame.id().to_string(), 0), ..Default::default() }),
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

fn override_of(id: &str) -> PageOverride {
    PageOverride { object_id: id.into(), bounds: None, visible: None, locked: None }
}

/// 🔀️ A relocation row places the page at its after index; the inverse reads the base and relocates it back.
#[test]
fn moved_rows_apply_and_invert_by_coordinates() {
    let mut base = default_document();
    let page = base.pages[0].clone();
    base.pages = ["a", "b", "c"].map(|id| Page { id: id.into(), ..page.clone() }).to_vec();
    let diff = LayoutDiff { pages: Some(LayoutPagesDelta { moved: vec![LayoutPageRelocation { id: "a".into(), from: 0, to: 2 }], ..Default::default() }), ..Default::default() };
    let after = apply(&diff, &base);
    assert_eq!(after.pages.iter().map(|page| page.id.as_str()).collect::<Vec<_>>(), ["b", "c", "a"]);
    let inverse = DiffAlgebra::inverse(&diff, &base);
    assert_eq!(inverse.pages.as_ref().expect("pages").moved, vec![LayoutPageRelocation { id: "a".into(), from: 2, to: 0 }]);
    assert_eq!(apply(&inverse, &after), base);
}

/// ➖️ A removal row carries the base index, so the inverse reinserts the base row there.
#[test]
fn removed_rows_invert_to_insertions_at_their_base_index() {
    let mut base = default_document();
    let page = base.pages[0].clone();
    base.pages = ["a", "b", "c"].map(|id| Page { id: id.into(), ..page.clone() }).to_vec();
    let diff = LayoutDiff { pages: Some(LayoutPagesDelta { removed: vec![LayoutPageRemoval { id: "b".into(), index: 1 }], ..Default::default() }), ..Default::default() };
    let after = apply(&diff, &base);
    let inverse = DiffAlgebra::inverse(&diff, &base);
    let rows = &inverse.pages.as_ref().expect("pages").inserted;
    assert_eq!((rows.len(), rows[0].index, rows[0].row.id.as_str()), (1, 1, "b"));
    assert_eq!(apply(&inverse, &after), base);
}

/// 🚫️ A removal naming a row that is not at its base index is refused.
#[test]
fn removal_off_its_base_index_is_refused() {
    let base = default_document();
    let id = base.pages[0].id.clone();
    let diff = LayoutDiff { pages: Some(LayoutPagesDelta { removed: vec![LayoutPageRemoval { id, index: 7 }], ..Default::default() }), ..Default::default() };
    assert!(protocol::apply_diff(&diff, &base).is_err());
}

/// 🎯️ The rows built for a whole-list payload apply back to exactly that payload: removed, inserted and relocated rows together.
#[test]
fn payload_rows_apply_to_the_payload() {
    let held = vec![override_of("o1"), override_of("o2"), override_of("o3"), override_of("o4")];
    let edited = PageOverride { visible: Some(false), ..override_of("o1") };
    let payload = vec![override_of("o4"), override_of("o3"), edited, override_of("o5")];
    let delta = page_override_rows(&held, &payload);
    assert_eq!(delta.removed.iter().map(|row| row.id.as_str()).collect::<Vec<_>>(), ["o1", "o2"]);
    assert_eq!(delta.inserted.len(), 2);
    assert_eq!(delta.moved.len(), 1);
    let mut base = default_document();
    base.pages[0].overrides = held;
    let after = apply(&page_patch(PagePatch { overrides: delta, ..Default::default() }), &base);
    assert_eq!(after.pages[0].overrides, payload);
}

/// ⏱️ Sequences of moves, removals and insertions coalesce to the one delta that applies like the sequence.
#[test]
fn absorbed_row_sequences_equal_their_parts() {
    let mut base = default_document();
    let page = base.pages[0].clone();
    base.pages = ["a", "b", "c", "d"].map(|id| Page { id: id.into(), ..page.clone() }).to_vec();
    let first = LayoutDiff { pages: Some(LayoutPagesDelta { moved: vec![LayoutPageRelocation { id: "a".into(), from: 0, to: 3 }], ..Default::default() }), ..Default::default() };
    let mid = apply(&first, &base);
    let second = LayoutDiff { pages: Some(LayoutPagesDelta { removed: vec![LayoutPageRemoval { id: "c".into(), index: 1 }], inserted: vec![LayoutPageInsertion { index: 0, row: Page { id: "z".into(), ..page.clone() } }], ..Default::default() }), ..Default::default() };
    let sum = absorbed(&first, &second);
    assert_eq!(apply(&sum, &base), apply(&second, &mid));
}
