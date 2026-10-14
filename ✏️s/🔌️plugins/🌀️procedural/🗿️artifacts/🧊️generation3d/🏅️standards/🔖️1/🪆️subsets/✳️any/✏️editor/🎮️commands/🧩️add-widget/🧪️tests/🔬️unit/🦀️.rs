use super::*;


fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).expect("creation fixture")
}

#[test]
fn automatic_positions_match_the_language_neutral_fixture() {
    for row in fixture()["placements"].as_array().unwrap() {
        let occupied: Vec<[f64; 4]> = row["occupied"].as_array().unwrap().iter().map(|rect| std::array::from_fn(|i| rect[i].as_f64().unwrap())).collect();
        let actual = automatic_position(row["x"].as_f64(), row["y"].as_f64(), [row["size"][0].as_f64().unwrap(), row["size"][1].as_f64().unwrap()], &occupied);
        assert_eq!(actual, [row["expected"][0].as_f64().unwrap(), row["expected"][1].as_f64().unwrap()], "{}", row["name"]);
    }
}

fn payload(value: &serde_json::Value) -> AddWidget {
    AddWidget {
        kind: value["kind"].as_str().unwrap().into(),
        neuron_kind: value["neuronKind"].as_str().map(String::from),
        format: value["format"].as_str().map(String::from),
        action: value["action"].as_str().map(String::from),
        x: value["x"].as_f64(),
        y: value["y"].as_f64(),
        source_widget:value["sourceWidget"].as_str().map(String::from),
        source_port:value["sourcePort"].as_str().map(String::from),
        target_port:value["targetPort"].as_str().map(String::from),
        source_revision:value["sourceRevision"].as_str().map(String::from),
    }
}

#[test]
fn descriptor_contract_matches_the_language_neutral_fixture() {
    for value in fixture()["valid"].as_array().unwrap() {
        let descriptor: serde_json::Value = serde_json::from_str(&payload(value).descriptor_json().expect("valid descriptor")).unwrap();
        assert_eq!(&descriptor, value);
    }
    for value in fixture()["invalid"].as_array().unwrap() {
        let fault = payload(value).descriptor_json().expect_err("invalid descriptor");
        assert_eq!(fault.code.0, fixture()["faultCode"].as_str().unwrap());
    }
    for row in fixture()["defaults"].as_array().unwrap() {
        let descriptor: serde_json::Value = serde_json::from_str(&payload(&row["payload"]).descriptor_json().unwrap()).unwrap();
        assert_eq!(descriptor, row["descriptor"]);
    }
    for row in fixture()["connected"].as_array().unwrap(){let command=payload(&row["payload"]);let descriptor:serde_json::Value=serde_json::from_str(&command.descriptor_json().unwrap()).unwrap();assert_eq!(descriptor,row["descriptor"]);let request=command.source_request().unwrap().unwrap();assert_eq!(request.source_widget,"shape");assert_eq!(request.source_port,row["sourcePort"].as_str().unwrap());assert_eq!(request.target_port(),row["targetPort"].as_str());assert_eq!(command.source_revision_bytes().unwrap(),[1;32]);}
}
#[test]
fn non_finite_coordinates_are_rejected_before_host_mutation() {
    let mut command = payload(&serde_json::json!({"kind":"inputNote"}));
    for number in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        command.x = Some(number);
        assert!(command.descriptor_json().is_err());
    }
}

#[test]
fn connected_source_refusal_uses_the_actual_bilingual_fault_notice(){
 let law=fixture();let notices=semio_framework::fault_notice_definitions(source_fault_notices());assert!(semio_framework::validate_fault_notices(&notices).is_empty());
 for reason in [semio_framework_os_flow::FlowSourceReadFailure::Revision,semio_framework_os_flow::FlowSourceReadFailure::ChangedSource,semio_framework_os_flow::FlowSourceReadFailure::MissingOutput,semio_framework_os_flow::FlowSourceReadFailure::FailedOutput]{let fault=source_fault(reason);assert_eq!(fault.code.0,law["sourceNotice"]["code"]);for(locale,name)in[(semio_framework_ui_locale::Locale::En,"en"),(semio_framework_ui_locale::Locale::De,"de")]{assert_eq!(semio_framework::fault_notice_text(&notices,&fault.code.0,fault.params.as_deref(),semio_framework_ui_locale::Terminology::Native,locale).as_deref(),law["sourceNotice"][name].as_str());}}
}
