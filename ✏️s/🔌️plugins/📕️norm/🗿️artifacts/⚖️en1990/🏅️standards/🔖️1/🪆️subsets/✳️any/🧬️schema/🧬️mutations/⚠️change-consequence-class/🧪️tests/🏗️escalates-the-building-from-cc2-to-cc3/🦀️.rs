use crate::{En1990Mutation, En1990Snapshot};
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn escalates_the_building_from_cc2_to_cc3() {
    let before = En1990Snapshot::default();
    assert_eq!(before.consequence_class, 2);
    let mutation = En1990Mutation::ChangeConsequenceClass(
        crate::standards::v1::subsets::any::schema::mutations::change_consequence_class::ChangeConsequenceClass { new_consequence_class: 3 },
    );
    let (after, _) = vcs::apply_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.consequence_class, 3);
}
