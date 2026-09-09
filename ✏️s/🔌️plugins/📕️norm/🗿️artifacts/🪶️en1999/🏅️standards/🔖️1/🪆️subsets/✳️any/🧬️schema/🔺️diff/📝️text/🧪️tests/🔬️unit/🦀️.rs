use super::*;
use crate::mutations::En1999Mutation;
use protocol::{Mutation as _, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn change_mutation_diff_updates_only_its_field() {
    let base = En1999Snapshot::default();
    let mutation = En1999Mutation::ChangeNEdKn(crate::mutations::change_n_ed_kn::ChangeNEdKn { new_n_ed_kn: 95.0 });
    let outcome = mutation.diff(&base);
    let mut expected = base.clone();
    expected.n_ed_kn = 95.0;
    assert_eq!(outcome.diff().apply(&base).expect("valid mutation diff"), expected);
}
