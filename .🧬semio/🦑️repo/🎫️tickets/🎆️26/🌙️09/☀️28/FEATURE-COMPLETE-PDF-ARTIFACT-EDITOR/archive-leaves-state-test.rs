#[test]
fn original_archive_ordinary_leaves_apply_and_restore_real_pdf_fixtures() {
    use crate::standards::v1_7::subsets::base::schema::testkit::snapshots;
    use dsl::original_receiving::receive_to_completion;
    let fixtures: [(&[u8], &[u8]); 3] = [
        (include_bytes!("../../../../🧪️tests/🧫️fixtures/🔄️set-page-rotation/⬅️before.pdf"), include_bytes!("../../../../🧪️tests/🧫️fixtures/🔄️set-page-rotation/➡️after.pdf")),
        (include_bytes!("../../../../🧪️tests/🧫️fixtures/ℹ️set-info/⬅️before.pdf"), include_bytes!("../../../../🧪️tests/🧫️fixtures/ℹ️set-info/➡️after.pdf")),
        (include_bytes!("../../../../🧪️tests/🧫️fixtures/🗣️set-language/⬅️before.pdf"), include_bytes!("../../../../🧪️tests/🧫️fixtures/🗣️set-language/➡️after.pdf")),
    ];
    let target = |state: &PdfSnapshot| (state.pages[0].rotate, state.info.clone(), state.language.clone());
    let laws = object_archive_cases();
    for ((before_bytes, after_bytes), pair) in fixtures.into_iter().zip(laws["ordinaryLeaves"]["pairs"].as_array().unwrap()) {
        let (before, after) = snapshots(before_bytes, after_bytes);
        let tag = pair["tag"].as_u64().unwrap() as u8;
        let [forward, inverse] = ["forward", "inverse"].map(|side| {
            let frame = leaf_archive_frame(tag, &pair[side]);
            receive_to_completion::<PdfMutation>(&frame, (0, frame.len()), image_archive_receiving_stock()).unwrap()
        });
        assert_eq!(semio_s_artifact_stdio_contract::mutation_inverse(&forward, &before).unwrap(), vec![inverse.clone()]);
        let mut state = before.clone();
        apply_pdf_mutation(&mut state, &forward);
        assert_eq!(target(&state), target(&after));
        apply_pdf_mutation(&mut state, &inverse);
        assert_eq!(target(&state), target(&before));
    }
}
