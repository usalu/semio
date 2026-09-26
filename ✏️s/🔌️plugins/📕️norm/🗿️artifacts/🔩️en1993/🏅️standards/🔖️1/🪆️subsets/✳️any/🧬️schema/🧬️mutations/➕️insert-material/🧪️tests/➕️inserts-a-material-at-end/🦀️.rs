use crate::mutations::insert_material::InsertMaterial;
use crate::{SteelMaterial, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = SteelMaterial { id: "mat-new".into(), grade: "S275".into(), fy: 275e6, fu: 430e6, e_modulus: 210e9, g_modulus: 81e9, subgrade: "J2".into(), kind: "carbon".into() };
    let payload = InsertMaterial { index: base.materials.len(), material: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.materials.len(), base.materials.len() + 1);
}
