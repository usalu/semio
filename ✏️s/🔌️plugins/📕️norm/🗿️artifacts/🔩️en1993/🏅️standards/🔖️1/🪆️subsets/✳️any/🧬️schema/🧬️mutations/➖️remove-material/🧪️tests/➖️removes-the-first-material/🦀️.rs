use crate::mutations::{remove_material::RemoveMaterial, insert_material::InsertMaterial};
use crate::{SteelMaterial, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = SteelMaterial { id: "mat-new".into(), grade: "S275".into(), fy: 275e6, fu: 430e6, e_modulus: 210e9, g_modulus: 81e9, subgrade: "J2".into(), kind: "carbon".into() };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertMaterial { index: 0, material: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.materials.len();
    let out = protocol::MutationKind::diff(&RemoveMaterial { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.materials.len(), before_len - 1);
}
