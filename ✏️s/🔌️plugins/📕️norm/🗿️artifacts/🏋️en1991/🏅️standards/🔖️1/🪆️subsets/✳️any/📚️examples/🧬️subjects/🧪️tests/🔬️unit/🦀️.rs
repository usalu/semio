use semio_s_artifact_norm_en1991_io::text::snapshot::decode_en1991_dsl;

#[test]
fn committed_example_assets_decode_to_their_named_subjects() {
    assert_eq!(decode_en1991_dsl(crate::de_office_compliant::PRIMARY_TEXT).expect("de-office-compliant DSL decodes"), crate::example_subjects::de_office_compliant());
    assert_eq!(decode_en1991_dsl(crate::multi_fail_noncompliant::PRIMARY_TEXT).expect("multi-fail-noncompliant DSL decodes"), crate::example_subjects::multi_fail_noncompliant());
}
