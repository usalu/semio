use crate::document::AnnexChoice;
use crate::{En1990Mutation, En1990Snapshot};
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn switches_the_national_annex_from_de_to_en() {
    let before = En1990Snapshot::default();
    assert_eq!(before.annex, AnnexChoice::De);
    let mutation = En1990Mutation::ChangeAnnex(crate::standards::v1::subsets::any::schema::mutations::change_annex::ChangeAnnex { new_annex: AnnexChoice::En });
    let (after, _) = vcs::apply_mutation(&before, &mutation).expect("apply");
    assert_eq!(after.annex, AnnexChoice::En);
    assert_eq!(after.project_id, before.project_id);
    assert_eq!(after.permanents, before.permanents);
}
