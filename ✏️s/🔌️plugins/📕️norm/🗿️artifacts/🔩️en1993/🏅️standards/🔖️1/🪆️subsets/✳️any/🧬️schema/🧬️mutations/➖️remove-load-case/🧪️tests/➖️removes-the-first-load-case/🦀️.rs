use crate::mutations::{remove_load_case::RemoveLoadCase, insert_load_case::InsertLoadCase};
use crate::{LoadCase, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = LoadCase { id: "lc-new".into(), name: "New".into(), kind: "imposed".into(), category: "office".into() };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertLoadCase { index: 0, load_case: item  }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.load_cases.len();
    let out = protocol::MutationKind::diff(&RemoveLoadCase { index: 0  }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.load_cases.len(), before_len - 1);
}
