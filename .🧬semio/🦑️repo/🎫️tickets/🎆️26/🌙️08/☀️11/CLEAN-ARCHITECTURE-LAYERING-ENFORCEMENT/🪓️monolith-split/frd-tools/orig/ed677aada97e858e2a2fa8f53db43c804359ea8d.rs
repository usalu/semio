use crate::standards::isobmff::subsets::any::schema::snapshot::*;

async fn sample_snapshot() -> Mp4Snapshot {
    Mp4Snapshot {
        schema: STDIO_MP4_DOCUMENT_SCHEMA.into(),
        ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 512, compatible_brands: vec!["isom".into(), "iso2".into(), "avc1".into(), "mp41".into()] },
        movie: Mp4Movie::default(),
        tracks: vec![Mp4Track {
            track_id: 1,
            timescale: 1000,
            codec: Mp4Codec::avc(vec![vec![0x67, 0x42, 0x00, 0x1E, 0x8C, 0x8D, 0x40]], vec![vec![0x68, 0xCE, 0x3C, 0x80]], 4, None),
            width: 64,
            height: 64,
            metadata: Mp4TrackMetadata::default(),
            chunk_sample_counts: vec![2],
            samples: vec![Mp4Sample { data: vec![0, 0, 0, 4, 0x65, 1, 2, 3], duration: 33, cts_offset: 0, sync: true }, Mp4Sample { data: vec![0, 0, 0, 3, 0x61, 4, 5], duration: 33, cts_offset: 33, sync: false }],
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips_via_real_mp4_bytes() {
    let snap = sample_snapshot().await;
    let bytes = <Mp4Snapshot as pack::snapshot::ArtifactPack>::encode_pack(&snap);
    let back = <Mp4Snapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips_via_real_mp4_bytes() {
    let snap = sample_snapshot().await;
    let text = <Mp4Snapshot as pack::snapshot::ArtifactDsl>::print_dsl(&snap);
    let back = <Mp4Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn default_snapshot_round_trips_through_real_codec() {
    let snap = Mp4Snapshot::default();
    let bytes = <Mp4Snapshot as pack::snapshot::ArtifactPack>::encode_pack(&snap);
    let back = <Mp4Snapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn logical_snapshot_and_facets_have_no_shadow_state() {
    let model = format!("{:?}", sample_snapshot().await);
    for forbidden in ["unknownBoxes", "physical", "sourceBytes", "nativeArchive", "\"raw\""] {
        assert!(!model.contains(forbidden), "snapshot contains forbidden shadow field {forbidden}");
    }
    for facet in [
        runtime_fixture_text("../../../../🟦️.ts"),
        runtime_fixture_text("../../../../🔗️.graphql"),
        runtime_fixture_text("../../../../🔣️.json"),
        runtime_fixture_text("../../../../🛰️.proto"),
        runtime_fixture_text("../../../../../🔺️diff/🦀️.rs"),
        runtime_fixture_text("../../../../../🔺️diff/🟦️.ts"),
        runtime_fixture_text("../../../../../🔺️diff/🔗️.graphql"),
        runtime_fixture_text("../../../../../🔺️diff/🔣️.json"),
        runtime_fixture_text("../../../../../🔺️diff/🛰️.proto"),
        runtime_fixture_text("../../../../../🧬️mutations/🦀️.rs"),
        runtime_fixture_text("../../../../../🧬️mutations/🟦️.ts"),
        runtime_fixture_text("../../../../../🧬️mutations/🔗️.graphql"),
        runtime_fixture_text("../../../../../🧬️mutations/🔣️.json"),
        runtime_fixture_text("../../../../../🧬️mutations/🛰️.proto"),
        runtime_fixture_text("../../../../../../🚪️io/💾️binary/📸️snapshot/🔠️.abnf"),
        runtime_fixture_text("../../../../../../🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio"),
        runtime_fixture_text("../../../../../../🚪️io/📝️text/🧬️mutations/📖️.grammar.semio"),
        runtime_fixture_text("../../../../../../🚪️io/📝️text/🧬️mutations/🔤️.ebnf"),
        runtime_fixture_text("../../../../../../🚪️io/📝️text/🧬️mutations/🅰️.g4"),
        runtime_fixture_text("../../../../../../🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"),
        runtime_fixture_text("../../../../../../🚪️io/💾️binary/🧬️mutations/🌶️.spicy"),
    ] {
        for forbidden in ["unknownBoxes", "unknown_boxes", "Mp4CodecOther", "ADD_UNKNOWN_BOX", "addUnknownBox", "nativeArchive", "sourceBytes", "serde_json::", "json_line", "jsonLine", "json_utf8", "JSON bytes", "iso-bmff-box-stream"] {
            assert!(!facet.contains(forbidden), "facet contains forbidden shadow concept {forbidden}");
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn exact_fixture_survives_pack_and_dsl_codecs() {
    let bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../🧪️tests/🧫️fixtures/🎬️.mp4")).expect("the committed fixture is readable");
    let snapshot = crate::standards::isobmff::subsets::any::io::decode_mp4(&bytes).expect("decode exact MP4 fixture");

    let pack = <Mp4Snapshot as pack::snapshot::ArtifactPack>::encode_pack(&snapshot);
    let from_pack = <Mp4Snapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack).expect("decode pack");
    assert_eq!(crate::standards::isobmff::subsets::any::io::encode_mp4(&from_pack), bytes);

    let dsl = <Mp4Snapshot as pack::snapshot::ArtifactDsl>::print_dsl(&snapshot);
    let from_dsl = <Mp4Snapshot as pack::snapshot::ArtifactDsl>::parse_dsl(&dsl).expect("parse dsl");
    assert_eq!(crate::standards::isobmff::subsets::any::io::encode_mp4(&from_dsl), bytes);
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
