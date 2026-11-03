#[semio_framework_async_macros::async_test]
async fn primary_asset_is_nonempty() {
    let text = include_str!("../../../../🖼️assets/🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio");
    assert!(text.len() > 8);
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    use semio_s_artifact_norm_en1998_inferences::En1998Inference;
    use semio_s_artifact_norm_en1998_model::En1998Snapshot;
    use protocol::Inference;
    let snapshot = En1998Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    assert_eq!(En1998Inference::infer(&snapshot).expect("valid materialized inference fixture"), En1998Inference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    use semio_s_artifact_norm_en1998_inferences::En1998Inference;
    use semio_s_artifact_norm_en1998_model::En1998Snapshot;
    use protocol::Inference;
    assert_eq!(En1998Inference::infer(&En1998Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)).expect("valid materialized inference fixture"), En1998Inference::default());
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
        let decoded = semio_s_artifact_norm_en1998_io::text::snapshot::decode_en1998_dsl(text).unwrap_or_else(|error| panic!("{id}: {error}"));
        assert!(decoded == snapshot, "{id}: the committed asset must be the code-built snapshot");
    }
}

/// 🎒️ The committed binary twin decodes to the code-built snapshot; with `SEMIO_UPDATE_GOLDEN=1` it is rewritten from the committed DSL text first.
#[semio_framework_async_macros::async_test]
async fn committed_pack_twin_matches_the_code_built_snapshot() {
    use semio_s_artifact_norm_en1998_io::binary::snapshot::{decode_en1998_pack, encode_en1998_pack};
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🖼️assets/🏢️seismic-rc-frame/🎒️.pack.semio");
    if std::env::var_os("SEMIO_UPDATE_GOLDEN").is_some() {
        let decoded = semio_s_artifact_norm_en1998_io::text::snapshot::decode_en1998_dsl(crate::seismic_rc_frame::PRIMARY_TEXT).expect("committed DSL text");
        std::fs::write(path, encode_en1998_pack(&decoded)).expect("write committed pack twin");
    }
    let bytes = std::fs::read(path).expect("read committed pack twin");
    assert!(decode_en1998_pack(&bytes).expect("decode committed pack twin") == crate::seismic_rc_frame::snapshot());
}
