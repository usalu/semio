use crate::mutations::{remove_section::RemoveSection, insert_section::InsertSection};
use crate::{SteelSection, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = crate::catalogue_heb260();
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertSection { index: 0, section: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.sections.len();
    let out = protocol::MutationKind::diff(&RemoveSection { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.sections.len(), before_len - 1);
}
