use crate::mutations::insert_pile::InsertPile;
use crate::{SteelPile, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = SteelPile { id: "pile-new".into(), section_id: "sec-heb240".into(), material_id: "mat-s355".into(), driving_stress: 250e6,
        embedded_length: 8.0,
        shaft_perimeter: 1.2, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 300_000.0 }] };
    let payload = InsertPile { index: base.piles.len(), pile: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.piles.len(), base.piles.len() + 1);
}
