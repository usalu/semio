use crate::standards::v1::subsets::any::io::export::serializers::artifacts::csv::v_rfc4180::any as csv_out;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::csv::v_rfc4180::any as csv_in;
use crate::schema::snapshot::ProgramSnapshot;

const TABLE: &str = include_str!("../../🧫️fixtures/📊️registers/📊️.csv");

/// 🔮️ The third-party `csv` reader (test-only) parses what the export writes, row for row.
#[test]
fn register_table_round_trips_through_a_third_party_reader() {
    let program = csv_in::deserialize_bytes(TABLE.as_bytes()).expect("csv import");
    let bytes = csv_out::serialize_bytes(&program).expect("csv export");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📊️registers/🔣️.json")).expect("neutral register table");
    let mut reader = csv::Reader::from_reader(bytes.as_slice());
    let header = reader.headers().expect("header").iter().collect::<Vec<_>>();
    assert_eq!(serde_json::json!(header), fixture["header"]);
    let rows: Vec<csv::StringRecord> = reader.records().map(|r| r.expect("row")).collect();
    assert_eq!(serde_json::json!(rows.iter().map(|row| row.iter().collect::<Vec<_>>()).collect::<Vec<_>>()), fixture["rows"]);
    assert_eq!(csv_in::deserialize_bytes(&bytes).expect("re-import").stakeholders, program.stakeholders, "the registers survive a second round trip");
    eprintln!("[DEBUG] Architect register exchange: owned CSV import/export, independent csv reader and neutral rows={}", rows.len());
}

#[test]
fn a_foreign_table_is_refused_not_emptied() {
    assert!(csv_in::deserialize_bytes(b"a,b\n1,2\n").is_err());
    let _ = ProgramSnapshot::default();
}
