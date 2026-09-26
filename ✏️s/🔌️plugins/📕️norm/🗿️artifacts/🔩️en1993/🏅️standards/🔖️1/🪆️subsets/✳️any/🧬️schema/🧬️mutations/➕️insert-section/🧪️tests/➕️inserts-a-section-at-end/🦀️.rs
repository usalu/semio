use crate::mutations::insert_section::InsertSection;
use crate::{SteelSection, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = crate::snapshot::catalogue_heb260();
    let payload = InsertSection { index: base.sections.len(), section: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.sections.len(), base.sections.len() + 1);
}
