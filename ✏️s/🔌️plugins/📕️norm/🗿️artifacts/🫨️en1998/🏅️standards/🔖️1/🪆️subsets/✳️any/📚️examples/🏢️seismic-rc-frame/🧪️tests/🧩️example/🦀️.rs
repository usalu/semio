#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use crate::artifact_schema::inferences::En1998Inference;
    use crate::En1998Snapshot;
    use protocol::Inference;
    let snapshot = En1998Snapshot::default();
    assert_eq!(En1998Inference::infer(&snapshot), En1998Inference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use crate::artifact_schema::inferences::En1998Inference;
    use crate::En1998Snapshot;
    use protocol::Inference;
    assert_eq!(En1998Inference::infer(&En1998Snapshot::default()), En1998Inference::default());
}

/// 🖼️ Every en1998 example asset (what the picker, the descriptor and `setActiveExample` load) decodes to exactly its
/// code-built snapshot (what the compliance gate evaluates) — the August flat-format assets stopped parsing when the
/// snapshot moved to `layout = "lines"`, and the old hand-kept `setActiveExample` arms hid it by loading the constructors.
#[semio_framework_async_macros::async_test]
async fn every_example_asset_decodes_to_its_code_built_snapshot() {
    for (id, text, snapshot) in [
        (crate::seismic_rc_frame::ID, crate::seismic_rc_frame::PRIMARY_TEXT, crate::seismic_rc_frame::snapshot()),
        (crate::seismic_rc_frame_fail::ID, crate::seismic_rc_frame_fail::PRIMARY_TEXT, crate::seismic_rc_frame_fail::snapshot()),
        (crate::seismic_multipart::ID, crate::seismic_multipart::PRIMARY_TEXT, crate::seismic_multipart::snapshot()),
        (crate::seismic_multipart_fail::ID, crate::seismic_multipart_fail::PRIMARY_TEXT, crate::seismic_multipart_fail::snapshot()),
    ] {
        let decoded = crate::standards::v1::subsets::any::schema::snapshot::decode_en1998_dsl(text).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert!(decoded == snapshot, "{id}: the committed asset must be the code-built snapshot");
    }
}
