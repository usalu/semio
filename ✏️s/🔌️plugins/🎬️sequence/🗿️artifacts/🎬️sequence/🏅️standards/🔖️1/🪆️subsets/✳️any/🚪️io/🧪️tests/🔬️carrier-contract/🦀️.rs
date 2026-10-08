use crate::standards::v1::subsets::any::io::{export::serializers::artifacts as export, import::deserializers::artifacts as import};
use crate::{SequenceHostSnapshot, SequenceSnapshot};
use dsl::os_pack as pack;
use semio_framework_os_kernel::io::io_mechanism::{Deserializer, Serializer};
use semio_framework::io_schema::IoPayload;
use semio_s_artifact_stdio_csv::CsvSnapshot;
use semio_s_artifact_stdio_md::{schema::snapshot::MdBlock, MdSnapshot};

#[semio_framework_async_macros::async_test]
async fn sequence_carrier_contracts_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️carrier-contracts.json")).expect("neutral carrier vectors");
    for row in vectors["cases"].as_array().expect("cases") {
        let fixture = neural_engine::ColdOwner::new(semio_framework_pack_json::from_json_str::<SequenceHostSnapshot>(&row["fixture"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned fixture decoder"));
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&*fixture)).expect("independent fixture oracle"), row["fixture"]);
        let snapshot = neural_engine::ColdOwner::new(SequenceSnapshot::from_host_snapshot(fixture.into_inner()));
        let payload = export::md::v_commonmark::any::SequenceIntoMd::serialize(&snapshot, &semio_framework_os_kernel::io::io_mechanism::ArchiveChildren::empty()).await.expect("markdown export").value;
        let IoPayload::Binary(bytes) = &payload else { panic!("binary markdown snapshot") };
        let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes).expect("markdown snapshot codec");
        let [MdBlock::CodeBlock { info: Some(info), literal }] = md.blocks.as_slice() else { panic!("one JSON code block") };
        assert_eq!(info, "json");
        assert_eq!(serde_json::from_str::<serde_json::Value>(literal).expect("independent markdown oracle"), row["fixture"]);
        let restored = neural_engine::ColdOwner::new(import::md::v_commonmark::any::MdIntoSequence::deserialize(&payload).await.expect("markdown import").value);
        assert_eq!(neural_engine::ColdOwner::new(restored.to_host_snapshot()), neural_engine::ColdOwner::new(snapshot.to_host_snapshot()));
        let csv_payload = export::csv::v_rfc4180::any::SequenceIntoCsv::serialize(&snapshot, &semio_framework_os_kernel::io::io_mechanism::ArchiveChildren::empty()).await.expect("csv export").value;
        let IoPayload::Binary(bytes) = &csv_payload else { panic!("binary csv snapshot") };
        let csv = <CsvSnapshot as store::ArtifactPack>::decode_pack(bytes).expect("csv snapshot codec");
        assert!(csv.has_header, "raw CSV carrier decoding applies its default header metadata");
        assert_eq!(csv.records.len(), row["fixture"]["steps"].as_array().expect("steps").len());
        for (record, step) in csv.records.iter().zip(row["fixture"]["steps"].as_array().expect("steps")) {
            assert_eq!(record.fields.len(), 3);
            assert_eq!(record.fields[0].value, step["id"].as_str().expect("step id"));
            assert_eq!(record.fields[1].value, step["kind"].as_str().expect("step kind"));
            assert!(record.fields[2].quoted);
            assert_eq!(serde_json::from_str::<serde_json::Value>(&record.fields[2].value).expect("independent params oracle"), step["params"]);
        }
        let inferred = <crate::schema::inferences::SequenceInference as protocol::Inference<SequenceSnapshot>>::infer(&snapshot).expect("valid materialized inference fixture");
        let inferred_json: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&inferred)).expect("independent inference oracle");
        assert_eq!(inferred_json["topology"], row["topology"]);
        let decoded: crate::schema::inferences::SequenceInference = semio_framework_pack_json::from_json_str(&inferred_json.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("inference value decoder");
        assert_eq!(decoded, inferred);
    }
}

#[semio_framework_async_macros::async_test]
async fn artifact_io_descriptors_match_the_neutral_fixture() {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    use semio_framework_os_kernel::io::io_mechanism::{io_entries, io_register, io_route};
    use {semio_framework_artifact_reference::ArtifactDialect,semio_framework::io_schema::IoEntryDescriptor,semio_framework::io_schema::IoRoute};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📇️descriptor-parity.json")).expect("neutral descriptors");
    let declaration = super::io();
    let actual = declaration.entries.iter().map(|entry| IoEntryDescriptor { from: ArtifactDialect::from(entry.from), into: ArtifactDialect::from(entry.into), fidelity: entry.fidelity, sniffs: entry.sniff.is_some() }).collect::<Vec<_>>();
    let encoded: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&actual)).expect("independent descriptor JSON oracle");
    assert_eq!(encoded, fixture["entries"]);
    assert_eq!(semio_framework_pack_json::from_json_str::<Vec<IoEntryDescriptor>>(&fixture["entries"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned descriptors"), actual);
    for row in fixture["invalidEntries"].as_array().expect("invalid descriptors") {
        assert!(semio_framework_pack_json::from_json_str::<IoEntryDescriptor>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "{row}");
    }
    for row in fixture["routes"].as_array().expect("routes") {
        let route = semio_framework_pack_json::from_json_str::<IoRoute>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("owned route");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&route)).expect("independent route oracle"), *row);
    }
    for row in fixture["invalidRoutes"].as_array().expect("invalid routes") {
        assert!(semio_framework_pack_json::from_json_str::<IoRoute>(&row.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_err(), "{row}");
    }
    io_register(declaration.entries).expect("sequence registry");
    let mut registered = io_entries().into_iter().filter(|entry| entry.from.artifact_kind == "s.sequence.sequence" || entry.into.artifact_kind == "s.sequence.sequence").collect::<Vec<_>>();
    registered.sort_by_key(|entry| (entry.from.to_coordinate(), entry.into.to_coordinate()));
    let mut expected = actual.clone();
    expected.sort_by_key(|entry| (entry.from.to_coordinate(), entry.into.to_coordinate()));
    assert_eq!(registered, expected);
    let route = io_route(&actual[6].from, &actual[6].into, 1).await.expect("registered exact text route").value;
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&route)).expect("route JSON"), fixture["routes"][0]);
    eprintln!("Artifact IO descriptor parity entries={}, registered={}", actual.len(), registered.len());
}

#[semio_framework_async_macros::async_test]
async fn artifact_io_reset_payload_and_registered_text_route_preserve_the_native_snapshot() {
    use semio_framework_os_kernel::io::io_mechanism::{io_register, io_route, io_run};
    use {semio_framework_artifact_reference::ArtifactDialect,semio_framework::io_schema::CARRIER_TEXT};
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️carrier-contracts.json")).expect("neutral native snapshots");
    io_register(super::io().entries).expect("sequence IO registration");
    let native = ArtifactDialect::from(crate::SEQUENCE_DIALECT);
    let text = ArtifactDialect::from(CARRIER_TEXT);
    for row in vectors["cases"].as_array().expect("cases") {
        let snapshot = neural_engine::ColdOwner::new(SequenceSnapshot::from_host_snapshot(semio_framework_pack_json::from_json_str::<SequenceHostSnapshot>(&row["fixture"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("neutral snapshot")));
        let payload = super::snapshot_pack(&snapshot);
        assert_eq!(payload, <SequenceSnapshot as store::ArtifactPack>::encode_pack(&snapshot));
        let effect = crate::editor::sequence::reset_sequence_document_effect(&snapshot);
        let semio_framework_plugin::Effect::LoadDocument { pack: effect_pack, spr } = effect else { panic!("load effect") };
        assert_eq!(effect_pack, payload);
        assert!(!spr.is_empty());
        let export = io_route(&native, &text, 1).await.expect("text export route").value;
        let exported = io_run(&export, IoPayload::Binary(payload.clone())).await.expect("registered text export").value;
        let IoPayload::Text(body) = &exported else { panic!("raw carrier text") };
        assert_eq!(body, &<SequenceSnapshot as store::ArtifactDsl>::print_dsl(&snapshot));
        let import = io_route(&text, &native, 1).await.expect("text import route").value;
        let restored = io_run(&import, exported).await.expect("registered text import").value;
        assert_eq!(restored, IoPayload::Binary(payload));
        eprintln!("Artifact IO native reset and text route bytes={}, spr={}", effect_pack.len(), spr.len());
    }
}
