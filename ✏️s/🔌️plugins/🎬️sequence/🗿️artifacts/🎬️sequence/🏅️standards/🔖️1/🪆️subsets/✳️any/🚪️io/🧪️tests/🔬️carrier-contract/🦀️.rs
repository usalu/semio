use crate::standards::v1::subsets::any::io::{export::serializers::artifacts as export, import::deserializers::artifacts as import};
use crate::{SequenceFixture, SequenceSnapshot};
use dsl::os_pack as pack;
use semio_framework::io::io_mechanism::{Deserializer, Serializer};
use semio_framework::io_schema::IoPayload;
use semio_s_artifact_stdio_csv::CsvSnapshot;
use semio_s_artifact_stdio_md::{schema::snapshot::MdBlock, MdSnapshot};

#[semio_framework_async_macros::async_test]
async fn sequence_carrier_contracts_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️carrier-contracts.json")).expect("neutral carrier vectors");
    for row in vectors["cases"].as_array().expect("cases") {
        let fixture = neural_engine::ColdOwner::new(pack::from_json_str::<SequenceFixture>(&row["fixture"].to_string()).expect("owned fixture decoder"));
        assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&*fixture)).expect("independent fixture oracle"), row["fixture"]);
        let snapshot = neural_engine::ColdOwner::new(SequenceSnapshot::from_fixture(fixture.into_inner()));
        let payload = export::md::v_commonmark::any::SequenceIntoMd::serialize(&snapshot).await.expect("markdown export").value;
        let IoPayload::Binary(bytes) = &payload else { panic!("binary markdown snapshot") };
        let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes).expect("markdown snapshot codec");
        let [MdBlock::CodeBlock { info: Some(info), literal }] = md.blocks.as_slice() else { panic!("one JSON code block") };
        assert_eq!(info, "json");
        assert_eq!(serde_json::from_str::<serde_json::Value>(literal).expect("independent markdown oracle"), row["fixture"]);
        let restored = neural_engine::ColdOwner::new(import::md::v_commonmark::any::MdIntoSequence::deserialize(&payload).await.expect("markdown import").value);
        assert_eq!(neural_engine::ColdOwner::new(restored.to_fixture()), neural_engine::ColdOwner::new(snapshot.to_fixture()));
        let csv_payload = export::csv::v_rfc4180::any::SequenceIntoCsv::serialize(&snapshot).await.expect("csv export").value;
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
        let inferred = <crate::schema::inferences::SequenceInference as protocol::Inference<SequenceSnapshot>>::infer(&snapshot);
        let inferred_json: serde_json::Value = serde_json::from_str(&pack::to_json_string(&inferred)).expect("independent inference oracle");
        assert_eq!(inferred_json["topology"], row["topology"]);
        let decoded: crate::schema::inferences::SequenceInference = pack::from_json_str(&inferred_json.to_string()).expect("inference value decoder");
        assert_eq!(decoded, inferred);
    }
}
