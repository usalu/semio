use super::*;
use semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
use standards::v1::subsets::any::schema::{evaluate_document, mutations::update_climate::UpdateClimate};
use standards::v1::subsets::any::io::{binary::snapshot::{decode_din18599_pack,encode_din18599_pack},text::snapshot::{decode_din18599_dsl,encode_din18599_dsl,decode_din18599_snapshot_json,encode_din18599_snapshot_json}};

/// 🧾️ Language-neutral expectation of the derived climate table (Python `hashlib` oracle, ticket script `🧪️s4-norm-din18599-climate.py`).
fn derivation() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🧫️climate-table-derivation/🔣️.json")).expect("language-neutral DIN 18599 climate-table fixture")
}

/// 🥶️ A document whose climate is not the Potsdam reference, reached through the `update-climate` leaf.
fn cold_document() -> Din18599Snapshot {
    let potsdam = Din18599Snapshot::default();
    let new_climate = MonthlyClimate { theta_e_c: potsdam.climate.theta_e_c.map(|theta| theta - 6.0), g_h_w_m2: potsdam.climate.g_h_w_m2.map(|g| g * 0.8) };
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&Din18599Mutation::UpdateClimate(UpdateClimate { new_climate }), &potsdam);
    <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(raised.diff(), &potsdam).expect("update-climate applies")
}

#[test]
fn the_climate_table_handle_is_the_content_id_of_the_climate() {
    let expected = derivation();
    let climate: MonthlyClimate = serde_json::from_value(expected["climate"].clone()).expect("fixture climate");
    assert_eq!(semio_framework_pack_json::to_json_string(&climate), expected["canonicalJson"].as_str().expect("canonical JSON"));
    let child = din18599_climate_table_child(&climate);
    assert_eq!(child.child_id, expected["childId"].as_str().expect("child id"));
    assert_eq!(child.target.artifact_id, child.child_id);
    let dialect = &child.target.dialect;
    assert_eq!([dialect.artifact_kind.as_str(), dialect.standard.as_str(), dialect.subset.as_str()], [&expected["dialect"]["artifactKind"], &expected["dialect"]["standard"], &expected["dialect"]["subset"]].map(|value| value.as_str().expect("dialect")));
    let table = din18599_climate_table_from_data(&climate);
    assert_eq!(serde_json::json!(table.columns.iter().map(|column| column.name.as_str()).collect::<Vec<_>>()), expected["columns"]);
    let rows: Vec<Vec<&str>> = table.rows.iter().map(|row| row.cells.iter().map(|cell| match cell { SemioValue::Float { lexeme } => lexeme.as_str(), other => panic!("non-float climate cell {other:?}") }).collect()).collect();
    assert_eq!(serde_json::json!(rows), expected["rows"]);
    assert_eq!(din18599_climate_data_from_table(&table), climate);
}

#[test]
fn update_climate_writes_the_parent_climate_and_re_derives_its_table() {
    let edited = cold_document();
    assert_ne!(edited.climate, MonthlyClimate::potsdam_reference());
    assert_eq!(edited.climate_table, din18599_climate_table_child(&edited.climate));
    let inverse = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::inverse(&Din18599Mutation::UpdateClimate(UpdateClimate { new_climate: edited.climate.clone() }), &Din18599Snapshot::default()).expect("inverse");
    assert_eq!(inverse, vec![Din18599Mutation::UpdateClimate(UpdateClimate { new_climate: MonthlyClimate::potsdam_reference() })]);
}

#[test]
fn the_genesis_pack_is_the_snapshot_s_own_climate_table() {
    let edited = cold_document();
    let pack = genesis_din18599_child_pack(&edited, DIN18599_CLIMATE_TABLE_SLOT, &edited.climate_table.child_id).expect("derived climate table");
    let table = <SemioTableSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("table pack");
    assert_eq!(din18599_climate_data_from_table(&table), edited.climate);
    assert!(genesis_din18599_child_pack(&edited, "climate", &edited.climate_table.child_id).is_none());
    assert!(genesis_din18599_child_pack(&edited, DIN18599_CLIMATE_TABLE_SLOT, &din18599_climate_table_child(&MonthlyClimate::potsdam_reference()).child_id).is_none());
}

/// 💾️ LAW (design §20.15): a saved document reloads through every persisted carrier with its own climate and evaluates with
/// it — never with the Potsdam reference — and its derived climate table opens from the reloaded parent alone.
#[test]
fn a_reloaded_document_evaluates_with_its_own_climate() {
    use store::sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl};
    use store::ArtifactSqliteSnapshot;
    let edited = cold_document();
    let report = serde_json::to_value(evaluate_document(&edited)).expect("report");
    assert_ne!(report, serde_json::to_value(evaluate_document(&Din18599Snapshot::default())).expect("potsdam report"), "the cold climate must change the evaluation");
    let limits = SqliteDatabaseLimits::default();
    let sqlite = export_sqlite_database(&edited.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).expect("sqlite projection"), limits, &mut |_| true).expect("sqlite bytes");
    let reloaded = [
        ("dsl", decode_din18599_dsl(&encode_din18599_dsl(&edited)).expect("dsl")),
        ("pack", decode_din18599_pack(&encode_din18599_pack(&edited)).expect("pack")),
        ("json", decode_din18599_snapshot_json(&encode_din18599_snapshot_json(&edited)).expect("json")),
        ("sqlite", Din18599Snapshot::from_sqlite_database(&import_sqlite_database(&sqlite, limits, &mut |_| true).expect("sqlite import"), &mut SqliteSnapshotControl::new(&mut |_| true, limits)).expect("sqlite")),
    ];
    for (carrier, document) in reloaded {
        assert_eq!(document, edited, "{carrier}: reload changed the document");
        assert_eq!(serde_json::to_value(evaluate_document(&document)).expect("report"), report, "{carrier}: reload changed the evaluation");
        assert!(genesis_din18599_child_pack(&document, DIN18599_CLIMATE_TABLE_SLOT, &document.climate_table.child_id).is_some(), "{carrier}: the derived table does not open");
    }
}

#[test]
fn din18599_child_restore_projection_accepts_the_derived_climate_table() {
    let snapshot = Din18599Snapshot::default();
    let projection = din18599_child_restore_projection(&snapshot).expect("canonical DIN 18599 climate table child");
    assert_eq!(projection.len(), 1);
    assert!(projection.admits_member(DIN18599_CLIMATE_TABLE_SLOT, &snapshot.climate_table.target));
    assert_eq!(snapshot.climate_table, din18599_climate_table_child(&snapshot.climate));
}
