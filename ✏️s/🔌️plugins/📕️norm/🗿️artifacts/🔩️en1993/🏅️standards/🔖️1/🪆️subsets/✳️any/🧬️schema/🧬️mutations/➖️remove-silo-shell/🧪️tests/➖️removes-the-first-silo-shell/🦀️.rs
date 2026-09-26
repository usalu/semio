use crate::mutations::{remove_silo_shell::RemoveSiloShell, insert_silo_shell::InsertSiloShell};
use crate::{SiloShell, En1993Snapshot};
#[test]
fn removes_first() {
    let base0 = En1993Snapshot::compliant_heb240_frame();
    let item = SiloShell { id: "silo-new".into(), thickness: 0.006, radius: 2.5, depth: 4.0, k: 0.5, gamma: 16_000.0, fy: 355e6 };
    let inserted = protocol::MutationDiff::apply(
        protocol::MutationKind::diff(&InsertSiloShell { index: 0, silo_shell: item }, &base0).diff(),
        &base0,
    ).unwrap();
    let before_len = inserted.silo_shells.len();
    let out = protocol::MutationKind::diff(&RemoveSiloShell { index: 0 }, &inserted);
    let next = protocol::MutationDiff::apply(out.diff(), &inserted).unwrap();
    assert_eq!(next.silo_shells.len(), before_len - 1);
}
