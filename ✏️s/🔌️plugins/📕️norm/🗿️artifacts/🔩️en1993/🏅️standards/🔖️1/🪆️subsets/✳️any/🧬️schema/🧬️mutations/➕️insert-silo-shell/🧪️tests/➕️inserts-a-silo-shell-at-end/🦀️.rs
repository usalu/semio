use crate::mutations::insert_silo_shell::InsertSiloShell;
use crate::{SiloShell, En1993Snapshot};
#[test]
fn inserts_at_end() {
    let base = En1993Snapshot::compliant_heb240_frame();
    let item = SiloShell { id: "silo-new".into(), thickness: 0.006, radius: 2.5, depth: 4.0, k: 0.5, gamma: 16_000.0, fy: 355e6 };
    let payload = InsertSiloShell { index: base.silo_shells.len(), silo_shell: item };
    let out = protocol::MutationKind::diff(&payload, &base);
    let next = protocol::MutationDiff::apply(out.diff(), &base).unwrap();
    assert_eq!(next.silo_shells.len(), base.silo_shells.len() + 1);
}
