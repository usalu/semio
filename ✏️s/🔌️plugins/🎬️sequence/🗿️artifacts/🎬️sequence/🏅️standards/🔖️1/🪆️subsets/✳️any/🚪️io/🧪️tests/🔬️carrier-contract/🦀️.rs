use crate::standards::v1::subsets::any::io::{export::serializers::artifacts as export, import::deserializers::artifacts as import};
use crate::{SequenceHostSnapshot, SequenceSnapshot};
use dsl::os_pack as pack;
use semio_framework::io::io_mechanism::{Deserializer, Serializer};
use semio_framework::io_schema::IoPayload;
use semio_s_artifact_stdio_csv::CsvSnapshot;
use semio_s_artifact_stdio_md::{schema::snapshot::MdBlock, MdSnapshot};

#[semio_framework_async_macros::async_test]
async fn sequence_carrier_contracts_match_the_json_oracle() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️carrier-contracts.json")).expect("neutral carrier vectors");
    for row in vectors["cases"].as_array().expect("cases") {
        let fixture = neural_engine::ColdOwner::new(pack::from_json_str::<SequenceHostSnapshot>(&row["fixture"].to_string()).expect("owned fixture decoder"));
        assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&*fixture)).expect("independent fixture oracle"), row["fixture"]);
        let snapshot = neural_engine::ColdOwner::new(SequenceSnapshot::from_host_snapshot(fixture.into_inner()));
        let payload = export::md::v_commonmark::any::SequenceIntoMd::serialize(&snapshot).await.expect("markdown export").value;
        let IoPayload::Binary(bytes) = &payload else { panic!("binary markdown snapshot") };
        let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes).expect("markdown snapshot codec");
        let [MdBlock::CodeBlock { info: Some(info), literal }] = md.blocks.as_slice() else { panic!("one JSON code block") };
        assert_eq!(info, "json");
        assert_eq!(serde_json::from_str::<serde_json::Value>(literal).expect("independent markdown oracle"), row["fixture"]);
        let restored = neural_engine::ColdOwner::new(import::md::v_commonmark::any::MdIntoSequence::deserialize(&payload).await.expect("markdown import").value);
        assert_eq!(neural_engine::ColdOwner::new(restored.to_host_snapshot()), neural_engine::ColdOwner::new(snapshot.to_host_snapshot()));
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

/// 📤️ The subset's real JSON export of `snapshot`, read back with serde_json.
async fn exported_carrier(snapshot: &SequenceSnapshot) -> serde_json::Value {
    let IoPayload::Binary(bytes) = export::json::v_rfc8259::any::SequenceIntoJson::serialize(snapshot).await.expect("json export").value else { panic!("binary json export") };
    serde_json::from_slice(&bytes).expect("the export parses in serde_json")
}

/// 🧩️ The committed serde_json carrier pairs (`🪜️step`/`🔗️dependency` `🧫️fixtures/<kind>/`), written by the
/// standalone `🏭️generator/🧩️json` engine, are held against this subset's REAL `{schema, steps, edges}`
/// export: each kind the CSV carrier cannot see is derived from what the pair changed, applied, exported
/// through `SequenceIntoJson` and read back with serde_json, and must equal the committed after-carrier;
/// its own inverse must export back to the before-carrier. The leaf vectors pin only the guard branches
/// (a content-changing diff mints a hash-addressed child handle no fixture can author), so these pairs are
/// where the applied branch of all four kinds is observed.
#[semio_framework_async_macros::async_test]
async fn the_json_carrier_pairs_hold_the_kinds_the_csv_carrier_cannot_see() {
    use crate::mutations::{apply_sequence_mutation, inverse_sequence_mutation, SequenceMutation};
    let pairs = [
        ("move-step", include_str!("../../../../🪜️step/🧫️fixtures/📍️move-step/⬅️before.json"), include_str!("../../../../🪜️step/🧫️fixtures/📍️move-step/➡️after.json")),
        ("change-step-collapsed", include_str!("../../../../🪜️step/🧫️fixtures/🗂️change-step-collapsed/⬅️before.json"), include_str!("../../../../🪜️step/🧫️fixtures/🗂️change-step-collapsed/➡️after.json")),
        ("connect-steps", include_str!("../../../../🔗️dependency/🧫️fixtures/🔗️connect-steps/⬅️before.json"), include_str!("../../../../🔗️dependency/🧫️fixtures/🔗️connect-steps/➡️after.json")),
        ("disconnect-steps", include_str!("../../../../🔗️dependency/🧫️fixtures/✂️disconnect-steps/⬅️before.json"), include_str!("../../../../🔗️dependency/🧫️fixtures/✂️disconnect-steps/➡️after.json")),
    ];
    let carrier = |text: &str| -> serde_json::Value { serde_json::from_str(text).expect("committed carrier parses in serde_json") };
    for (kind, before_text, after_text) in pairs {
        let (before, after) = (carrier(before_text), carrier(after_text));
        let steps = |value: &serde_json::Value| value["steps"].as_array().expect("steps").clone();
        let edges = |value: &serde_json::Value| value["edges"].as_array().expect("edges").clone();
        let payload = match kind {
            "move-step" | "change-step-collapsed" => {
                let moved = steps(&after).into_iter().zip(steps(&before)).find(|(now, was)| now != was).expect("the pair changes one step").0;
                match kind {
                    "move-step" => serde_json::json!({"mutation": "moveStep", "id": moved["id"], "x": moved["x"], "y": moved["y"]}),
                    _ => serde_json::json!({"mutation": "changeStepCollapsed", "id": moved["id"], "collapsed": moved["collapsed"]}),
                }
            }
            "connect-steps" => {
                let edge = edges(&after).into_iter().find(|edge| !edges(&before).contains(edge)).expect("the pair adds one edge");
                serde_json::json!({"mutation": "connectSteps", "id": edge["id"], "from": edge["from"], "to": edge["to"]})
            }
            _ => {
                let edge = edges(&before).into_iter().find(|edge| !edges(&after).contains(edge)).expect("the pair removes one edge");
                serde_json::json!({"mutation": "disconnectSteps", "id": edge["id"]})
            }
        };
        let mutation: SequenceMutation = pack::from_json_str(&payload.to_string()).unwrap_or_else(|error| panic!("{kind}: derived payload decodes: {error:?}"));
        let base = SequenceSnapshot::from_host_snapshot(pack::from_json_str::<SequenceHostSnapshot>(before_text).expect("before-carrier decodes"));
        let applied = apply_sequence_mutation(&base, &mutation).unwrap_or_else(|error| panic!("{kind}: applies: {error:?}"));
        assert_eq!(exported_carrier(&applied).await, after, "{kind}: the exported carrier after the mutation differs from the committed after-carrier");
        let undone = inverse_sequence_mutation(&base, &mutation).iter().fold(applied, |current, step| apply_sequence_mutation(&current, step).unwrap_or_else(|error| panic!("{kind}: inverse step applies: {error:?}")));
        assert_eq!(exported_carrier(&undone).await, before, "{kind}: the exported carrier after the inverse differs from the committed before-carrier");
    }
}
