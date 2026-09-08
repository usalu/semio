
use super::*;
use crate::mutations::En1998Mutation;
use protocol::{Mutation as _, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn change_mutation_diff_updates_only_its_field() {
    let base = En1998Snapshot::default();
    let mutation = En1998Mutation::ChangeSeismicZone(crate::mutations::change_seismic_zone::ChangeSeismicZone { new_seismic_zone: 3 });
    let outcome = mutation.diff(&base);
    let mut expected = base.clone();
    expected.seismic_zone = 3;
    assert_eq!(outcome.diff().apply(&base).expect("valid mutation diff"), expected);
}
