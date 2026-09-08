
/// 🧵️ Framework-neutral summary exposed by the owned oracle boundary.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NormRetainedDispositionSummary {
    pub(crate) app_count: usize,
    pub(crate) route_count: usize,
    pub(crate) retained_count: u64,
    pub(crate) batch_only_count: u64,
    pub(crate) publication_contract_count: usize,
}

/// 🧪️ Owned boundary hiding the test-only JSON implementation.
pub(crate) trait NormRetainedDispositionOracle {
    fn summarize(&self, source: &str) -> Result<NormRetainedDispositionSummary, String>;
}

/// 🧪️ Third-party `serde_json` implementation kept behind the owned boundary.
pub(crate) struct SerdeJsonNormRetainedDispositionOracle;

impl NormRetainedDispositionOracle for SerdeJsonNormRetainedDispositionOracle {
    fn summarize(&self, source: &str) -> Result<NormRetainedDispositionSummary, String> {
        let value: serde_json::Value = serde_json::from_str(source).map_err(|error| error.to_string())?;
        let routes = value["routes"].as_array().ok_or("routes must be an array")?;
        let expected_ids = super::NORM_RETAINED_TOOL_IDS;
        if routes.len() != expected_ids.len() {
            return Err("route count must be exactly three".into());
        }
        for (route, expected_id) in routes.iter().zip(expected_ids) {
            if route["id"] != *expected_id || route["admission"] != "migrated" {
                return Err(format!("invalid route disposition for {expected_id}"));
            }
        }
        if routes[0]["emittedLanes"] != serde_json::json!(["artifact"]) || routes[1]["emittedLanes"] != serde_json::json!([]) || routes[2]["emittedLanes"] != serde_json::json!(["config"]) {
            return Err("route lane audit does not match the command bodies".into());
        }
        let apps = value["apps"].as_array().ok_or("apps must be an array")?;
        let publication_contracts = value["publicationContracts"].as_array().ok_or("publicationContracts must be an array")?;
        let declared = super::NORM_PUBLICATION_CONTRACTS.iter().map(|contract| serde_json::json!({ "toolId": contract.tool_id, "lanes": contract.lanes.iter().map(|lane| format!("{lane:?}")).collect::<Vec<_>>() })).collect::<Vec<_>>();
        if publication_contracts != &declared {
            return Err("publication contracts do not match the live factory declaration".into());
        }
        for (route, contract) in routes.iter().zip(publication_contracts) {
            if route["publicationLanes"] != contract["lanes"] {
                return Err("route publication lanes diverge from the factory publication contract".into());
            }
        }
        if value["factory"]["payloadSchema"] != super::NORM_RETAINED_PAYLOAD_SCHEMA || value["factory"]["maximumRawBytes"] != serde_json::json!(super::NORM_RETAINED_RAW_BYTES) {
            return Err("factory identity does not match the live shared factory".into());
        }
        let summary = NormRetainedDispositionSummary {
            app_count: apps.len(),
            route_count: routes.len(),
            retained_count: value["expected"]["retained"].as_u64().ok_or("retained must be an integer")?,
            batch_only_count: value["expected"]["batchOnlyPendingRewrite"].as_u64().ok_or("batchOnlyPendingRewrite must be an integer")?,
            publication_contract_count: publication_contracts.len(),
        };
        if summary.app_count != 15 || summary.retained_count != 45 || summary.batch_only_count != 0 || summary.publication_contract_count != 3 {
            return Err("cohort totals or publication contracts are not fully migrated".into());
        }
        Ok(summary)
    }
}

/// 🧪️ Pins the canonical fixture and rejects forged admission, lane, and publication claims.
pub fn assert_fixture(variant: &str) {
    let source = include_str!("../../../🧪️fixtures/🧫️retained-command-dispositions/🔣️.json");
    let oracle = SerdeJsonNormRetainedDispositionOracle;
    let summary = oracle.summarize(source).expect("canonical Norm retained disposition fixture");
    assert_eq!(summary, NormRetainedDispositionSummary { app_count: 15, route_count: 3, retained_count: 45, batch_only_count: 0, publication_contract_count: 3 });
    let canonical: serde_json::Value = serde_json::from_str(source).expect("canonical fixture JSON");
    assert!(canonical["apps"].as_array().expect("apps").iter().any(|app| app["variant"] == variant), "missing Norm app fixture row for {variant}");

    let mut forged_admission = canonical.clone();
    forged_admission["routes"][0]["admission"] = serde_json::json!("batchOnlyPendingRewrite");
    assert!(oracle.summarize(&forged_admission.to_string()).is_err());
    let mut forged_lane = canonical.clone();
    forged_lane["routes"][2]["emittedLanes"] = serde_json::json!(["host"]);
    assert!(oracle.summarize(&forged_lane.to_string()).is_err());
    let mut forged_publication = canonical.clone();
    forged_publication["publicationContracts"][1]["lanes"] = serde_json::json!(["Artifact"]);
    assert!(oracle.summarize(&forged_publication.to_string()).is_err());
    let mut forged_factory = canonical;
    forged_factory["factory"]["payloadSchema"] = serde_json::json!("norm.tool-command.v2");
    assert!(oracle.summarize(&forged_factory.to_string()).is_err());
}
