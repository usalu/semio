use crate::mutations::{remove_pile::RemovePile, insert_pile::InsertPile};
use crate::{SteelPile, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = SteelPile { id: "pile-new".into(), section_id: "sec-heb240".into(), material_id: "mat-s355".into(), driving_stress: 250e6,
        embedded_length: 8.0,
        shaft_perimeter: 1.2, actions: vec![crate::ForceAction { id: "a".into(), load_case_id: "g-permanent".into(), force: 300_000.0 }] };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertPile { index: 0, pile: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.piles.len();
    let out = protocol::MutationKind::diff(&RemovePile { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.piles.len(), before_len - 1);
}
