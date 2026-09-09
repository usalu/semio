use super::*;
use crate::schema::onboarding_example_spec;

#[semio_framework_async_macros::async_test]
async fn export_flattens_one_row_per_question_plus_header() {
    let spec = onboarding_example_spec();
    let question_count: usize = forms_steps(&spec).iter().map(|step| step.blocks.len()).sum();
    let IoOutcome { value: IoPayload::Binary(bytes), .. } = FormsIntoCsv::serialize(&spec).await.expect("serialize") else {
        panic!("expected a binary payload");
    };
    let csv = <CsvSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(csv.records.len(), question_count + 1, "header row + one row per question");
    assert_eq!(csv.records[0].fields[0].value, "id");
}
