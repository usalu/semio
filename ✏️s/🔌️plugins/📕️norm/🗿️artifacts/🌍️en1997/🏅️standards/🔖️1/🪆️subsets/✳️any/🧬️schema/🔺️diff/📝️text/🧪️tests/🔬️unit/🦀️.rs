use super::*;
use crate::document::AnnexChoice;
use crate::mutations::En1997Mutation;
use protocol::{Mutation as _, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn change_mutation_diff_updates_only_its_field() {
    let base = En1997Snapshot::default();
    let mutation = En1997Mutation::ChangeAnnex(crate::mutations::change_annex::ChangeAnnex { new_annex: AnnexChoice::En });
    let outcome = mutation.diff(&base);
    let mut expected = base.clone();
    expected.annex = AnnexChoice::En;
    assert_eq!(outcome.diff().apply(&base).expect("valid mutation diff"), expected);
}
