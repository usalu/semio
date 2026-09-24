use crate::io::export::serializers::artifacts::csv::v_rfc4180::any as csv_out;
use crate::io::import::deserializers::artifacts::csv::v_rfc4180::any as csv_in;
use crate::schema::snapshot::ProgramSnapshot;

const TABLE: &str = "register,id,name,status,priority,tags,source\nstakeholders,00000000-0000-0000-0000-000000000001,\"Owner, Client\",draft,high,,brief\n";

/// 🔮️ The third-party `csv` reader (test-only) parses what the export writes, row for row.
#[test]
fn register_table_round_trips_through_a_third_party_reader() {
    let program = csv_in::deserialize_bytes(TABLE.as_bytes()).expect("csv import");
    let bytes = csv_out::serialize_bytes(&program).expect("csv export");
    let mut reader = csv::Reader::from_reader(bytes.as_slice());
    assert_eq!(reader.headers().expect("header").iter().collect::<Vec<_>>(), vec!["register", "id", "name", "status", "priority", "tags", "source"]);
    let rows: Vec<csv::StringRecord> = reader.records().map(|r| r.expect("row")).collect();
    assert_eq!(rows.len(), 1);
    assert_eq!((&rows[0][0], &rows[0][2]), ("stakeholders", "Owner, Client"));
    assert_eq!(csv_in::deserialize_bytes(&bytes).expect("re-import").stakeholders, program.stakeholders, "the registers survive a second round trip");
}

#[test]
fn a_foreign_table_is_refused_not_emptied() {
    assert!(csv_in::deserialize_bytes(b"a,b\n1,2\n").is_err());
    let _ = ProgramSnapshot::default();
}
