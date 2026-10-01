//! Legacy flat-field fixture retired — smoke that ChangeDesignSituation SEMANTICS stay wired.
#[semio_framework_async_macros::async_test]
async fn change_design_situation_semantics() {
    use crate::mutations::change_design_situation::ChangeDesignSituation;
    use crate::document::DesignSituation;
    use crate::En1996Snapshot;
    use protocol::{Mutation, MutationKind};
    let base = En1996Snapshot::compliant_clay_wall();
    let m = ChangeDesignSituation { new_design_situation: DesignSituation::Seismic };
    assert_eq!(<ChangeDesignSituation as MutationKind<En1996Snapshot, crate::En1996Mutation>>::SEMANTICS.kind, "change-design-situation");
    let outcome = <crate::En1996Mutation as Mutation<En1996Snapshot>>::diff(&crate::En1996Mutation::ChangeDesignSituation(m), &base);
    let after = protocol::MutationDiff::apply(outcome.diff(), &base).expect("apply");
    assert_eq!(after.design_situation, DesignSituation::Seismic);
}
