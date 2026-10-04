use super::*;

#[test]
fn inference_current_hub_wire_preserves_required_nullable_hash() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json")).unwrap();
    let receipt: GisMapInferenceJobReceiptV1 = semio_framework_pack_json::from_json_str(&corpus["wire"]["receipt"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let page: GisMapInferenceEventPageV1 = semio_framework_pack_json::from_json_str(&corpus["wire"]["page"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(receipt.schema, "semio.hub.inference-job-receipt/v1");
    assert_eq!(page.schema, "semio.hub.inference-job-events/v1");
    for (name, encoded) in [("receipt", semio_framework_pack_json::to_json_string(&receipt)), ("page", semio_framework_pack_json::to_json_string(&page))] {
        let observed: serde_json::Value = serde_json::from_str(&encoded).unwrap();
        assert_eq!(observed, corpus["wire"][name], "{name}");
        assert!(observed.as_object().unwrap().contains_key("proposalHash"));
        assert!(observed["proposalHash"].is_null());
    }
    for name in ["receipt", "page"] {
        let mut missing = corpus["wire"][name].clone();
        missing.as_object_mut().unwrap().remove("proposalHash");
        let accepted = match name {
            "receipt" => semio_framework_pack_json::from_json_str::<GisMapInferenceJobReceiptV1>(&missing.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_ok(),
            "page" => semio_framework_pack_json::from_json_str::<GisMapInferenceEventPageV1>(&missing.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).is_ok(),
            _ => unreachable!(),
        };
        assert!(!accepted, "{name} must require an explicit nullable proposalHash");
    }
    let mut substituted_receipt = receipt.clone();
    substituted_receipt.schema = "semio.hub.inference-job-events/v1".to_string();
    assert!(!substituted_receipt.validate());
    let mut substituted_page = page;
    substituted_page.schema = "semio.hub.inference-job-receipt/v1".to_string();
    assert!(!substituted_page.validate(&receipt.job_id));
}

#[test]
fn inference_indeterminate_lifecycle_matches_neutral_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json")).unwrap();
    let mut status = GisMapInferencePortStatusV1::default();
    for row in corpus["uncertainLifecycle"].as_array().unwrap() {
        let event = &row["event"];
        let event = match event["kind"].as_str().unwrap() {
            "start" => GisMapInferencePortEventV1::Start,
            "cancel" => GisMapInferencePortEventV1::Cancel,
            "indeterminate" => GisMapInferencePortEventV1::Indeterminate(semio_framework_pack_json::from_json_str(&event["code"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()),
            "receipt" => GisMapInferencePortEventV1::Receipt(semio_framework_pack_json::from_json_str(&event["receipt"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()),
            "page" => GisMapInferencePortEventV1::Page(semio_framework_pack_json::from_json_str(&event["page"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()),
            _ => panic!("unexpected neutral inference event"),
        };
        status = reduce_gis_map_inference_port_v1(&status, &event);
        let observed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&status)).unwrap();
        assert_eq!(observed, row["expected"], "{}", row["name"]);
    }
}

