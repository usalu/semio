//! `-shells` named scenario.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn specifies_shells_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-shells applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap());
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    let mut changed = base.shells.clone();
    changed.push(crate::snapshot::AluminiumShell {
            id: "shell-1".into(),
            material_id: "mat-1".into(),
            radius: 0.6,
            thickness: 0.005,
            length: 2.0,
            actions: vec![crate::snapshot::MemberAction {
                id: "G".into(),
                kind: "permanent".into(),
                category: "self".into(),
                source: "external".into(),
                g_k_line: 0.0,
                q_k_line: 0.0,
                n_k: 50e6,
                v_y_k: 0.0,
                v_z_k: 0.0,
                m_y_k: 30e6,
                m_z_k: 0.0,
            }],
        });
    En1999Mutation::ChangeShells(
        crate::mutations::change_shells::ChangeShells { shells: changed }
    )
}
