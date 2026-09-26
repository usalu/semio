//! `-cold-formed` named scenario.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn specifies_cold_formed_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-cold-formed applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap());
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    let mut changed = base.cold_formed.clone();
    changed.push(crate::snapshot::ColdFormedSheet {
            id: "sheet-1".into(),
            material_id: "mat-1".into(),
            thickness: 0.002,
            width: 0.25,
            span: 1.0,
            m_ed: 800.0,
            n_ed: 0.0,
            welded: false,
        });
    En1999Mutation::ChangeColdFormed(
        crate::mutations::change_cold_formed::ChangeColdFormed { cold_formed: changed }
    )
}
