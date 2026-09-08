
use super::*;
use crate::mutations::En1995Mutation;
use protocol::{Mutation as _, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn change_mutation_diff_updates_only_its_field() {
    let base = En1995Snapshot::default();
    let mutation = En1995Mutation::ChangeMEdKnm(crate::mutations::change_m_ed_knm::ChangeMEdKnm { new_m_ed_knm: 25.0 });
    let outcome = mutation.diff(&base);
    let mut expected = base.clone();
    expected.m_ed_knm = 25.0;
    assert_eq!(outcome.diff().apply(&base).expect("valid mutation diff"), expected);
}
