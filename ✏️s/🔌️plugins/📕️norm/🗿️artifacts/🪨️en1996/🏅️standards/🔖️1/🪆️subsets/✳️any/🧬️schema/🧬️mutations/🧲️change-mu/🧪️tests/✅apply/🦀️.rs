//! Legacy flat-field fixture retired — smoke that ChangeMu updates walls[].mu.
#[semio_framework_async_macros::async_test]
async fn change_mu_updates_wall_friction() {
    use crate::mutations::change_mu::ChangeMu;
    use crate::En1996Snapshot;
    use protocol::{Mutation, MutationKind};
    let base = En1996Snapshot::compliant_clay_wall();
    let m = ChangeMu { index: 0, new_mu: 0.625 };
    assert_eq!(<ChangeMu as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-mu");
    let outcome = <crate::En1996Mutation as Mutation<En1996Snapshot>>::diff(&crate::En1996Mutation::ChangeMu(m), &base);
    let after = protocol::MutationDiff::apply(outcome.diff(), &base).expect("apply");
    assert!((after.walls[0].mu - 0.625).abs() < 1e-12);
}
