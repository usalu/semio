use crate::mutations::insert_load_case::InsertLoadCase;
use crate::{LoadCase, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = LoadCase { id: "lc-new".into(), name: "New".into(), kind: "imposed".into(), category: "office".into() };
    let payload = InsertLoadCase { index: base.load_cases.len(), load_case: item  };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.load_cases.len(), base.load_cases.len() + 1);
}
