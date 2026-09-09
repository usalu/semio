use super::*;
use crate::mutations::En1997Mutation;
use protocol::{Mutation as _, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn change_mutation_diff_updates_only_its_field() {
    let base = En1997Snapshot::default();
    let mutation = En1997Mutation::ChangeVEdKn(crate::mutations::change_v_ed_kn::ChangeVEdKn { new_v_ed_kn: 620.0 });
    let outcome = mutation.diff(&base);
    let mut expected = base.clone();
    expected.v_ed_kn = 620.0;
    assert_eq!(outcome.diff().apply(&base).expect("valid mutation diff"), expected);
}
