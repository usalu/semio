/// 🔤️ LAW: txt, tsv and html open over the hub through linked native codecs. For every case of the neutral fixture
/// `📇️registry/🧫️fixtures/📇️native-text-codecs` the artifact-owned receipt's codec compiles the source into a pack whose
/// decoded snapshot is the fixture's snapshot and whose printed mirror compiles back to the identical pack; the same
/// snapshots are what independent readers (Python's `str.splitlines`, `csv` and `html.parser`) derive from each source.
#[semio_framework_async_macros::async_test]
async fn text_document_codecs_compile_their_neutral_fixture_through_the_linked_receipts() {
    fn neutral<T: semio_framework_os_kernel::ToValue>(snapshot: T) -> serde_json::Value {
        serde_json::from_str(&pack::json_to_string(&pack::json_from_dsl_value(&snapshot.to_value()))).unwrap()
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../📇️registry/🧫️fixtures/📇️native-text-codecs/🔣️.json"))).unwrap();
    assert_eq!(fixture["schema"], "semio.stdio.native-text-codecs/v1");
    let receipts = native_codec_factory_receipts().expect("artifact-owned native codec receipts");
    let cases = fixture["cases"].as_array().unwrap();
    for kind in ["s.stdio.txt", "s.stdio.tsv", "s.stdio.html"] {
        assert!(cases.iter().any(|case| case["artifactKind"] == kind), "{kind} has no neutral case");
    }
    for case in cases {
        let id = case["id"].as_str().unwrap();
        let kind = case["artifactKind"].as_str().unwrap();
        let receipt = receipts.iter().find(|receipt| receipt.artifact_kind == kind).unwrap_or_else(|| panic!("{id}: {kind} owns no linked receipt"));
        assert_eq!(receipt.factory_id, case["factoryId"].as_str().unwrap(), "{id}");
        let codec = receipt.instantiate().expect("verified linked codec");
        let (files, mirror) = (codec.compile_dsl)(case["source"].as_str().unwrap(), "").await.unwrap_or_else(|error| panic!("{id}: {error:?}"));
        let printed = (codec.print_mirror)(&files.pack, &files.spr).await.unwrap_or_else(|error| panic!("{id}: {error:?}"));
        assert_eq!(printed.dsl, mirror, "{id}: the pack prints the mirror its compile returned");
        let (again, _) = (codec.compile_dsl)(&printed.dsl, &printed.ops).await.unwrap_or_else(|error| panic!("{id}: {error:?}"));
        assert_eq!(again.pack, files.pack, "{id}: the mirror compiles back to the identical pack");
        let snapshot = match kind {
            "s.stdio.txt" => neutral(<semio_s_artifact_stdio_txt::TxtSnapshot as semio_framework_os_kernel::ArtifactPack>::decode_pack(&files.pack).unwrap()),
            "s.stdio.tsv" => neutral(<semio_s_artifact_stdio_tsv::TsvSnapshot as semio_framework_os_kernel::ArtifactPack>::decode_pack(&files.pack).unwrap()),
            "s.stdio.html" => neutral(<semio_s_artifact_stdio_html::HtmlSnapshot as semio_framework_os_kernel::ArtifactPack>::decode_pack(&files.pack).unwrap()),
            other => panic!("{id}: {other} is outside the text codec fixture"),
        };
        assert_eq!(snapshot, case["snapshot"], "{id}");
    }
}
