use crate::document::AnnexChoice;
use crate::mutations::change_annex::ChangeAnnex;
use crate::{En1996Mutation, En1996Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn change_annex_diff_updates_only_annex() {
    let base = En1996Snapshot::compliant_clay_wall();
    let mutation = En1996Mutation::ChangeAnnex(ChangeAnnex { new_annex: AnnexChoice::En });
    let outcome = Mutation::diff(&mutation, &base);
    let mut expected = base.clone();
    expected.annex = AnnexChoice::En;
    assert_eq!(outcome.diff().apply(&base).expect("valid mutation diff"), expected);
}
