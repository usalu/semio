//! 🦀️ Ticket-local parity probe: writes the Rust implementation's own artefacts for the 📡️events
//! module so they can be diffed byte for byte against the Go twin and the Node oracle.

use semio_framework_repo_events as events;
use std::fs;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
struct StoreVectors {
    inputs: Vec<events::Input>,
}

#[derive(serde::Deserialize)]
struct PayloadCase {
    id: String,
    #[serde(rename = "type")]
    kind: String,
    input: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct PayloadVectors {
    cases: Vec<PayloadCase>,
}

#[derive(serde::Deserialize)]
struct ExportEntityVector {
    kind: String,
    id: String,
    value: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct ExportVectors {
    entities: Vec<ExportEntityVector>,
}

fn read<T: serde::de::DeserializeOwned>(fixtures: &PathBuf, name: &str) -> T {
    let data = fs::read(fixtures.join(name)).expect("fixture");
    serde_json::from_slice(&data).expect("fixture json")
}

fn round_trip(kind: &str, input: &serde_json::Value) -> String {
    macro_rules! decode {
        ($($name:literal => $type:ty),* $(,)?) => {
            match kind {
                $($name => {
                    let value: $type = serde_json::from_value(input.clone()).expect($name);
                    serde_json::to_string(&value).expect($name)
                })*
                other => panic!("unknown payload type {other}"),
            }
        };
    }
    decode! {
        "TicketPayload" => events::TicketPayload,
        "TicketOpenPayload" => events::TicketOpenPayload,
        "TicketClosePayload" => events::TicketClosePayload,
        "TicketReopenPayload" => events::TicketReopenPayload,
        "TicketChangePayload" => events::TicketChangePayload,
        "GoalPayload" => events::GoalPayload,
        "GoalOpenPayload" => events::GoalOpenPayload,
        "GoalClosePayload" => events::GoalClosePayload,
        "GoalReopenPayload" => events::GoalReopenPayload,
        "GoalChangePayload" => events::GoalChangePayload,
        "ContributorPayload" => events::ContributorPayload,
        "CheckpointPayload" => events::CheckpointPayload,
        "TodoPayload" => events::TodoPayload,
        "TodoCreatePayload" => events::TodoCreatePayload,
        "TodoChangePayload" => events::TodoChangePayload,
        "TodoDeletePayload" => events::TodoDeletePayload,
        "WorkItem" => events::WorkItem,
        "ContributorWork" => events::ContributorWork,
        "DraftPayload" => events::DraftPayload,
        "FilePayload" => events::FilePayload,
        "FolderPayload" => events::FolderPayload,
        "SectionPayload" => events::SectionPayload,
        "IntegratePayload" => events::IntegratePayload,
        "ExtractPayload" => events::ExtractPayload,
    }
}

fn main() {
    let mut argv = std::env::args().skip(1);
    let fixtures = PathBuf::from(argv.next().expect("fixtures"));
    let out = PathBuf::from(argv.next().expect("out"));
    fs::create_dir_all(&out).expect("out dir");

    let store: StoreVectors = read(&fixtures, "🗄️store-vectors.json");
    let log_path = out.join("🦀️store.jsonl");
    let _ = fs::remove_file(&log_path);
    let mut stage = log_path.clone().into_os_string();
    stage.push(".stage");
    let _ = fs::remove_file(PathBuf::from(stage));
    let log = events::Store::new(&log_path);
    log.append(&store.inputs, &events::Uninterrupted).expect("append");

    let payloads: PayloadVectors = read(&fixtures, "✉️payload-vectors.json");
    let encodings: serde_json::Map<String, serde_json::Value> = payloads
        .cases
        .iter()
        .map(|case| {
            (case.id.clone(), serde_json::Value::String(round_trip(&case.kind, &case.input)))
        })
        .collect();

    let export: ExportVectors = read(&fixtures, "📤️export-vectors.json");
    let entities: Vec<events::ExportEntity> = export
        .entities
        .into_iter()
        .map(|entity| events::ExportEntity {
            kind: entity.kind,
            id: entity.id,
            value: entity.value,
        })
        .collect();
    let snapshot =
        events::build_export_snapshot(&entities, &events::Uninterrupted).expect("snapshot");

    let envelope = serde_json::to_string(&events::Event {
        kind: events::TICKET_OPEN_STARTING.to_string(),
        source: "repo-cli".to_string(),
        payload: serde_json::json!({"id": "a"}),
    })
    .expect("envelope");

    let report = serde_json::json!({
        "implementation": "rust",
        "kinds": events::ALL_EVENT_KINDS,
        "encodings": encodings,
        "envelope": envelope,
        "snapshot": snapshot.snapshot,
        "inputIds": snapshot.inputs.iter().map(|input| input.id.clone()).collect::<Vec<_>>(),
        "storeDigest": events::digest(&fs::read(&log_path).expect("log")),
    });
    fs::write(
        out.join("🦀️report.json"),
        format!("{}\n", serde_json::to_string_pretty(&report).expect("report")),
    )
    .expect("write report");
    println!("rust report written");
}
