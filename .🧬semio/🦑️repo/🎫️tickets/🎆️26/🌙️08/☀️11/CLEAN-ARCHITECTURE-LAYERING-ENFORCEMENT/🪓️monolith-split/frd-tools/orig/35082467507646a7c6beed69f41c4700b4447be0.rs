//! 🔬️ Unit tests of the artifact snapshot reader: paid row index, projection and cancellation.
use super::*;
#[test]
fn paid_row_index_neutral_explicit_signed_identity_preserves_sqlite_zero_and_negative_surrogates(){
    let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../🧩️artifact/🧪️tests/🧫️fixtures/🗂️paid-row-index/🔣️.json")).unwrap();
    let mut db=neutral_paid_row_index_database();db.tables[0].rows=fixture["signedIdentity"]["rows"].as_array().unwrap().iter().map(|row|SqliteRow{rowid:row["id"].as_i64().unwrap(),values:vec![SqliteValue::Integer(row["id"].as_i64().unwrap()),SqliteValue::Integer(row["owner"].as_i64().unwrap()),SqliteValue::Integer(row["ordinal"].as_i64().unwrap()),SqliteValue::Text(row["literal"].as_str().unwrap().into()),row["parent"].as_i64().map(SqliteValue::Integer).unwrap_or(SqliteValue::Null)]}).collect();
    let mut callback=|_|true;let limits=super::super::SqliteDatabaseLimits::default();let mut control=SqliteSnapshotControl::new(&mut callback,limits);
    assert!(RowIndex::new(&mut RowIndexStorage::empty(),&db,"neutral_row",5,&[],&mut control,"positive row required").is_err());assert!(RowIndex::new_with_identity(&mut RowIndexStorage::empty(),&db,"neutral_row",5,&[],RowIdentity::Positive,&mut control,"positive row required").is_err());
    let mut storage=RowIndexStorage::empty();let mut rows=RowIndex::new_with_identity(&mut storage,&db,"neutral_row",5,&[],RowIdentity::Signed,&mut control,"invalid signed row").unwrap();
    assert_eq!(rows.indices().iter().map(|index|rows.row(*index).unwrap().rowid).collect::<Vec<_>>(),vec![-7,-3,0,4]);let mut order=Vec::new();rows.grouped_by_into(&mut order,2,&mut control,"invalid signed ordinal",|row|Ok((0,Some(row.integer(1)?)))).unwrap();assert_eq!(order.iter().map(|index|rows.row(*index).unwrap().rowid).collect::<Vec<_>>(),vec![0,-3,-7,4]);
    assert_eq!(rows.get(0,&mut control).unwrap().unwrap().text(3).unwrap(),"alpha");assert!(!rows.row(0).unwrap().is_null(4).unwrap());let mut parents=Vec::new();rows.parent_positions_into(&mut parents,4,&mut control,"dangling signed parent").unwrap();assert_eq!(parents,vec![(Some(1),0),(None,0),(Some(3),0),(None,0)]);RowIndex::cycles(&mut parents,&mut control,"cyclic signed parent").unwrap();
    assert_eq!(rows.take(0,&mut control).unwrap().unwrap().rowid,0);assert!(rows.take(0,&mut control).unwrap().is_none());assert_eq!(rows.remaining(),3);drop(rows);
    db.tables[0].rows[0].rowid=0;db.tables[0].rows[0].values[0]=SqliteValue::Integer(0);assert!(RowIndex::new_with_identity(&mut RowIndexStorage::empty(),&db,"neutral_row",5,&[],RowIdentity::Signed,&mut control,"duplicate signed row").is_err());
}

fn neutral_paid_row_index_database()->SqliteDatabase{
    let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../🧩️artifact/🧪️tests/🧫️fixtures/🗂️paid-row-index/🔣️.json")).unwrap();
    let rows=fixture["rows"].as_array().unwrap().iter().map(|row|SqliteRow{rowid:row["id"].as_i64().unwrap(),values:vec![SqliteValue::Integer(row["id"].as_i64().unwrap()),SqliteValue::Integer(row["owner"].as_i64().unwrap()),SqliteValue::Integer(row["ordinal"].as_i64().unwrap()),SqliteValue::Text(row["literal"].as_str().unwrap().into()),row["parent"].as_i64().map(SqliteValue::Integer).unwrap_or(SqliteValue::Null)]}).collect();
    SqliteDatabase{tables:vec![super::super::SqliteTable{name:fixture["table"].as_str().unwrap().into(),sql:fixture["schema"].as_str().unwrap().into(),rows}]}
}
fn complete_neutral_paid_row_index(control:&mut SqliteSnapshotControl<'_>)->usize{
    let db=neutral_paid_row_index_database();let mut storage=RowIndexStorage::empty();let mut rows=RowIndex::new(&mut storage,&db,"neutral_row",5,&[],control,"invalid neutral row").unwrap();
    let mut grouped=Vec::new();rows.grouped_by_into(&mut grouped,2,control,"invalid neutral ordinal",|row|Ok((0,Some(row.integer(1)?)))).unwrap();
    assert_eq!(grouped.iter().map(|index|rows.row(*index).unwrap().rowid).collect::<Vec<_>>(),vec![1,3,2,4]);
    let range=rows.range_by(&grouped,(0,Some(20)),control,|row|Ok((0,Some(row.integer(1)?)))).unwrap();assert_eq!(range,0..2);
    rows.unique_text(&mut Vec::new(),rows.indices(),3,control,"duplicate neutral literal").unwrap();let mut parents=Vec::new();rows.parent_positions_into(&mut parents,4,control,"dangling neutral parent").unwrap();assert_eq!(parents,vec![(Some(1),0),(None,0),(Some(3),0),(None,0)]);RowIndex::cycles(&mut parents,control,"cyclic neutral parent").unwrap();
    assert!(std::ptr::eq(rows.get(3,control).unwrap().unwrap().values.as_ptr(),db.tables[0].rows[0].values.as_ptr()));
    assert_eq!(rows.position(999,control).unwrap(),None);assert_eq!(rows.take(3,control).unwrap().unwrap().rowid,3);assert!(rows.take(3,control).unwrap().is_none());assert_eq!(rows.remaining(),3);
    for index in 0..rows.len(){rows.take_index(index,control).unwrap();}assert_eq!(rows.remaining(),0);rows.len()
}
#[test]
fn paid_row_index_neutral_complete_positions_groups_and_single_consumption(){
    let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,super::super::SqliteDatabaseLimits::default());assert_eq!(complete_neutral_paid_row_index(&mut control),4);assert!(control.allocation_remaining_bytes()<control.limits().max_allocation_bytes);
}
#[test]
fn paid_row_index_neutral_identity_ordinal_text_foreign_key_and_cycle_refusals(){
    let mut callback=|_|true;let limits=super::super::SqliteDatabaseLimits::default();
    for role in 0..3{let mut db=neutral_paid_row_index_database();match role{0=>db.tables[0].rows[0].values[0]=SqliteValue::Integer(9),1=>{db.tables[0].rows[0].rowid=1;db.tables[0].rows[0].values[0]=SqliteValue::Integer(1)},_=>{db.tables[0].rows[0].values.pop();}}let mut control=SqliteSnapshotControl::new(&mut callback,limits);assert!(RowIndex::new(&mut RowIndexStorage::empty(),&db,"neutral_row",5,&[],&mut control,"invalid neutral row").is_err());}
    let mut db=neutral_paid_row_index_database();db.tables[0].rows[0].values[2]=SqliteValue::Integer(3);let mut control=SqliteSnapshotControl::new(&mut callback,limits);let mut storage=RowIndexStorage::empty();let rows=RowIndex::new(&mut storage,&db,"neutral_row",5,&[],&mut control,"invalid neutral row").unwrap();assert!(rows.grouped_by_into(&mut Vec::new(),2,&mut control,"invalid neutral ordinal",|row|Ok((0,Some(row.integer(1)?)))).is_err());
    let mut db=neutral_paid_row_index_database();db.tables[0].rows[0].values[3]=SqliteValue::Text("alpha".into());let mut control=SqliteSnapshotControl::new(&mut callback,limits);let mut storage=RowIndexStorage::empty();let rows=RowIndex::new(&mut storage,&db,"neutral_row",5,&[],&mut control,"invalid neutral row").unwrap();assert!(rows.unique_text(&mut Vec::new(),rows.indices(),3,&mut control,"duplicate neutral literal").is_err());
    let mut db=neutral_paid_row_index_database();db.tables[0].rows[0].values[4]=SqliteValue::Integer(999);let mut control=SqliteSnapshotControl::new(&mut callback,limits);let mut storage=RowIndexStorage::empty();let rows=RowIndex::new(&mut storage,&db,"neutral_row",5,&[],&mut control,"invalid neutral row").unwrap();assert!(rows.parent_positions_into(&mut Vec::new(),4,&mut control,"dangling neutral parent").is_err());
    let db=neutral_paid_row_index_database();let mut control=SqliteSnapshotControl::new(&mut callback,limits);let mut storage=RowIndexStorage::empty();let rows=RowIndex::new(&mut storage,&db,"neutral_row",5,&[],&mut control,"invalid neutral row").unwrap();assert!(rows.ordered_into(&mut Vec::new(),2,&mut control,"invalid neutral ordinal").is_err());let mut parents=Vec::new();rows.parent_positions_into(&mut parents,4,&mut control,"dangling neutral parent").unwrap();parents[1].0=Some(0);assert!(RowIndex::cycles(&mut parents,&mut control,"cyclic neutral parent").is_err());
}
#[test]
fn paid_row_index_neutral_exact_short_cumulative_and_cancellation_controls(){
    let limits=super::super::SqliteDatabaseLimits::default();let db=neutral_paid_row_index_database();let mut callback=|_|true;let mut probe=SqliteSnapshotControl::new(&mut callback,limits);complete_neutral_paid_row_index(&mut probe);let paid=limits.max_allocation_bytes-probe.allocation_remaining_bytes();assert!(paid>0);
    let exact=super::super::SqliteDatabaseLimits{max_allocation_bytes:paid,..limits};let mut control=SqliteSnapshotControl::new(&mut callback,exact);complete_neutral_paid_row_index(&mut control);assert_eq!(control.allocation_remaining_bytes(),0);assert!(RowIndex::new(&mut RowIndexStorage::empty(),&db,"neutral_row",5,&[],&mut control,"invalid neutral row").is_err());
    let mut control=SqliteSnapshotControl::new(&mut callback,super::super::SqliteDatabaseLimits{max_allocation_bytes:4*(std::mem::size_of::<usize>()+1)-1,..limits});assert!(RowIndex::new(&mut RowIndexStorage::empty(),&db,"neutral_row",5,&[],&mut control,"invalid neutral row").is_err());
    let mut callback=|_|false;let mut control=SqliteSnapshotControl::new(&mut callback,limits);assert!(RowIndex::new(&mut RowIndexStorage::empty(),&db,"neutral_row",5,&[],&mut control,"invalid neutral row").is_err());assert_eq!(control.allocation_remaining_bytes(),limits.max_allocation_bytes);
    let mut reached=false;let mut callback=|event:super::super::SqliteSnapshotProgress|{if event.phase==SqliteSnapshotPhase::ReconstructSnapshot&&event.completed==1&&event.total==4{reached=true;false}else{true}};let mut control=SqliteSnapshotControl::new(&mut callback,limits);assert!(RowIndex::new(&mut RowIndexStorage::empty(),&db,"neutral_row",5,&[],&mut control,"invalid neutral row").is_err());let admitted=limits.max_allocation_bytes-control.allocation_remaining_bytes();drop(control);assert!(reached);assert_eq!(admitted,4*(std::mem::size_of::<usize>()+1));
}

#[test]
fn native_encoding_bounds_reject_overflow_and_cancel_before_large_admission() {
    let mut limits = super::super::SqliteDatabaseLimits::default(); limits.max_value_bytes = 32;
    let mut callback = |_| true; let mut control = SqliteSnapshotControl::new(&mut callback, limits);
    let mut bound = NativeEncodingBound::new(&mut control).unwrap();
    bound.repeated(4, 8).unwrap(); assert!(bound.add(1).is_err()); assert_eq!(bound.bytes, 32);
    assert!(bound.repeated(usize::MAX, 2).is_err());
    let mut calls = 0; let mut callback = |_| { calls += 1; calls == 1 };
    let mut control = SqliteSnapshotControl::new(&mut callback, super::super::SqliteDatabaseLimits::default());
    let mut bound = NativeEncodingBound::new(&mut control).unwrap();
    assert!(bound.add(65_537).is_err()); assert_eq!(bound.bytes, 0);
    let mut limits = super::super::SqliteDatabaseLimits::default(); limits.max_file_bytes = 16;
    let mut callback = |_| true; let mut control = SqliteSnapshotControl::new(&mut callback, limits);
    assert!(NativeEncodingBound::new(&mut control).unwrap().add(17).is_err());
}
#[test]
fn native_file_forecast_separates_exact_semantic_cells_and_preserves_file_refusals(){
    let fixture:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../🧩️artifact/🧪️tests/🧫️fixtures/📏️file-bound/🔣️.json")).unwrap();
    let limits=super::super::SqliteDatabaseLimits{max_value_bytes:fixture["maxSemanticBytes"].as_u64().unwrap()as usize,max_file_bytes:fixture["maxFileBytes"].as_u64().unwrap()as usize,..super::super::SqliteDatabaseLimits::default()};
    let mut accepted=|_|true;let mut control=SqliteSnapshotControl::new(&mut accepted,limits);let mut bound=NativeEncodingBound::file_only(&mut control).unwrap();
    for chunk in fixture["chunks"].as_array().unwrap(){bound.add(chunk.as_u64().unwrap()as usize).unwrap();}
    assert_eq!(bound.bytes,fixture["expectedBytes"].as_u64().unwrap()as usize);assert_eq!(bound.add(1).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(bound.bytes,limits.max_file_bytes);assert!(bound.repeated(usize::MAX,2).is_err());bound.finish().unwrap();
    let mut control=SqliteSnapshotControl::new(&mut accepted,super::super::SqliteDatabaseLimits{max_file_bytes:limits.max_file_bytes-1,..limits});let mut short=NativeEncodingBound::file_only(&mut control).unwrap();assert!(short.add(limits.max_file_bytes).is_err());assert_eq!(short.bytes,0);
    let mut denied=|_|false;let mut control=SqliteSnapshotControl::new(&mut denied,limits);assert!(NativeEncodingBound::file_only(&mut control).is_err());
    let mut calls=0;let mut cancellation=|_|{calls+=1;calls==1};let mut control=SqliteSnapshotControl::new(&mut cancellation,super::super::SqliteDatabaseLimits::default());let mut canceled=NativeEncodingBound::file_only(&mut control).unwrap();assert!(canceled.add(65_537).is_err());assert_eq!(canceled.bytes,0);
}
#[test]
fn reconstruction_bounds_repeated_foreign_key_copies_and_large_copy_cancellation() {
    let mut limits = super::super::SqliteDatabaseLimits::default(); limits.max_value_bytes = 8;
    let mut callback = |_| true;
    let mut control = SqliteSnapshotControl::new(&mut callback, limits);
    let mut restore = Reconstruction::new(&mut control).unwrap();
    assert_eq!(restore.text("same").unwrap(), "same");
    assert_eq!(restore.text("same").unwrap(), "same");
    assert!(restore.text("same").is_err());
    let bytes = vec![0; 100_000]; let mut first = true;
    let mut callback = |_| { let result = first; first = false; result };
    let mut control = SqliteSnapshotControl::new(&mut callback, super::super::SqliteDatabaseLimits::default());
    let mut restore = Reconstruction::new(&mut control).unwrap();
    assert!(restore.blob(&bytes).is_err()); assert_eq!(restore.bytes, 0);
}
#[test]
fn borrowed_projection_bounds_values_before_copy_and_checks_cancellation() {
    let sql = "CREATE TABLE explicit_entity (id INTEGER PRIMARY KEY, label TEXT NOT NULL)";
    let mut limits = super::super::SqliteDatabaseLimits::default(); limits.max_value_bytes = 9;
    let mut callback = |_| true;
    let mut control = SqliteSnapshotControl::new(&mut callback, limits);
    let mut projection = Projection::new(sql, &mut control).unwrap();
    assert!(projection.insert("explicit_entity", &[Cell::Text("too long")]).is_err());
    let mut control = SqliteSnapshotControl::new(&mut callback, super::super::SqliteDatabaseLimits::default());
    let mut projection = Projection::new(sql, &mut control).unwrap();
    projection.insert("explicit_entity", &[Cell::Text("a")]).unwrap();
    let db = projection.finish().unwrap(); assert_eq!(db.table("explicit_entity").unwrap().single_row().unwrap().text(1).unwrap(), "a");
    let payload = "a".repeat(100_000);
    let armed = std::cell::Cell::new(false);
    let mut progress = |_| !armed.get();
    let mut control = SqliteSnapshotControl::new(&mut progress, super::super::SqliteDatabaseLimits::default());
    let mut projection = Projection::new(sql, &mut control).unwrap();
    armed.set(true);
    assert_eq!(projection.insert("explicit_entity", &[Cell::Text(&payload)]).unwrap_err().kind, ValueRefusalKind::Canceled);
    assert!(projection.database.table("explicit_entity").unwrap().rows.is_empty());
    assert!(Projection::new(sql, &mut SqliteSnapshotControl::new(&mut |_| false, super::super::SqliteDatabaseLimits::default())).is_err());
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
