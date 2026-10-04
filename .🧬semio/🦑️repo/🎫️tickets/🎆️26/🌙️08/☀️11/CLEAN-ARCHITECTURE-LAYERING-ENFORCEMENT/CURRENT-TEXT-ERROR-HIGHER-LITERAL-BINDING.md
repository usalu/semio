# Actual Higher TextError Literal Causes

Two actual higher struct expressions require mandatory kind: RetainedSnapshot's ParseIntError is InvalidValue at actual integer syntax admission; DecodedBuffers rejects its unsupported text owner with UnsupportedOwner. Message, span, expected fields and all original test bodies remain. No controller is classified from a message. The OS physical Text encoder observed independently already preserves direct ValueError kind at an authored span; its complete current bytes are captured without author attribution.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-literals-authored-1.json. Native admission is pending.

## 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-retirement/🧪️tests/🦀️.rs

```rust
use super::*;
use crate::sqlite_snapshot::{SqliteDatabase, SqliteDatabaseLimits, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase, SnapshotEncoding, artifact::NativeEncodingBound};

struct RetainedSnapshot { value: i64, retired: bool }
std::thread_local! { static RETIREMENTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

impl Drop for RetainedSnapshot {
    fn drop(&mut self) { assert!(self.retired, "retained snapshot requires its declared owner retirement"); }
}

impl ArtifactDsl for RetainedSnapshot {
    const EXTENSION: &'static str = "retained";
    fn parse_dsl(text: &str) -> Result<Self, TextError> { Ok(Self { value: text.parse().map_err(|error: std::num::ParseIntError| TextError { message: error.to_string(), span: Default::default(), expected: None })?, retired: false }) }
    fn print_dsl(&self) -> String { self.value.to_string() }
}

impl ArtifactPack for RetainedSnapshot {
    fn encode_pack_with(&self, _: &PackEncodeOptions) -> Result<Vec<u8>, PackError> { if self.value == -2 { return Err(PackError::Schema("owner encoder refusal".into())); } Ok(self.value.to_le_bytes().to_vec()) }
    fn decode_pack_with(bytes: &[u8], _: &PackDecodeOptions) -> Result<Self, PackError> { Ok(Self { value: i64::from_le_bytes(bytes.try_into().map_err(|_| PackError::Schema("expected integer word".into()))?), retired: false }) }
}

fn value_database(value: i64) -> SqliteDatabase {
    let mut database = SqliteDatabase::from_schema(RetainedSnapshot::SQLITE_SCHEMA).unwrap();
    database.table_mut("retained_value").unwrap().rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Integer(value)] });
    database
}

impl ArtifactSqliteSnapshot for RetainedSnapshot {
    const SQLITE_SCHEMA: &'static str = include_str!("../🧬️schema/🗄️.sql");
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<crate::io_schema::IoPayload,String>{
        use semio_framework_value::native_encoding::{NativeEncodeControl,NativeEncodeProgress};
        let limits=control.limits();let mut callback=|event:NativeEncodeProgress|control.checkpoint(SqliteSnapshotPhase::EncodeNative,event.completed,event.total).is_ok();let mut native=NativeEncodeControl::new(limits.max_value_bytes,&mut callback);native.begin_stage(1)?;let output=match encoding{SnapshotEncoding::Binary=>{if self.value == -2{return Err("owner encoder refusal".into())}crate::io_schema::IoPayload::Binary(native.copy_bytes(&self.value.to_le_bytes())?)},SnapshotEncoding::Text=>{native.charge(20)?;crate::io_schema::IoPayload::Text(self.value.to_string())}};native.step()?;Ok(output)
    }
    fn decode_sqlite_snapshot_native(payload:&crate::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
        control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;
        control.check_value_bytes(std::mem::size_of::<Self>())?;
        let length=match payload{crate::io_schema::IoPayload::Binary(bytes)=>bytes.len(),crate::io_schema::IoPayload::Text(text)=>text.len()};
        if length>control.limits().max_file_bytes{return Err("retained scalar exceeds file byte limit".into());}
        let value=match payload{
            crate::io_schema::IoPayload::Binary(bytes)=>i64::from_le_bytes(bytes.as_slice().try_into().map_err(|_|"expected exact integer word")?),
            crate::io_schema::IoPayload::Text(text)if text.len()<=20=>text.parse::<i64>().map_err(|_|"expected bounded integer text")?,
            _=>return Err("expected bounded integer text".into()),
        };
        Ok(Self{value,retired:false})
    }
    fn retire_sqlite_snapshot(mut self) { self.retired = true; RETIREMENTS.with(|count| count.set(count.get() + 1)); }
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?; Ok(value_database(self.value)) }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?; Ok(Self { value: database.table("retained_value")?.single_row()?.integer(1)?, retired: false }) }
    fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), String> { let mut bound = NativeEncodingBound::new(control)?; bound.add(32)?; bound.finish() }
    fn validate_sqlite_snapshot_subset(&self, dialect: &crate::io_schema::ArtifactDialect, _: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> crate::io_schema::IoResult<()> { control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?; if dialect.subset == "reject" { return Err("declared semantic owner refusal".to_string().into()); } Ok(crate::io_schema::IoOutcome::clean(())) }
}

#[test]
fn sqlite_snapshot_native_retirement_covers_success_cancellation_and_refusal() {
    use std::{io::Write, process::{Command, Stdio}};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let schema: serde_json::Value = serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).unwrap();
    let script = "import {Database} from 'bun:sqlite';import Ajv from 'ajv/dist/2020.js';const x=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('fixture');const db=Database.deserialize(Buffer.from(x.sqlite,'base64'));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');if(db.query('SELECT value FROM retained_value WHERE id=1').get().value!==7)throw Error('owner value');await Bun.write(Bun.stdout,'ok');db.close();";
    let sqlite = crate::sqlite_snapshot::export_sqlite_database(&value_database(7), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture,"sqlite":protocol::bytes::encode_base64(&sqlite)}).to_string().as_bytes()).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    let codec = RetainedSnapshot::sqlite_codec();
    for case in fixture["cases"].as_array().unwrap() {
        RETIREMENTS.with(|count| count.set(0));
        let value = case["value"].as_i64().unwrap();
        let operation = case["operation"].as_str().unwrap();
        let dialect = crate::io_schema::ArtifactDialect { artifact_kind: "fixture.retained".into(), standard: "1".into(), subset: case["subset"].as_str().unwrap().into() };
        let mut callback = |event: crate::sqlite_snapshot::SqliteSnapshotProgress| case["cancelPhase"].as_str().is_none_or(|phase| format!("{:?}", event.phase) != phase);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default());
            if operation.starts_with("export") {
                let payload = if operation.ends_with("binary") { crate::io_schema::IoPayload::Binary(value.to_le_bytes().to_vec()) } else { crate::io_schema::IoPayload::Text(value.to_string()) };
                (codec.export)("fixture.retained/v1", &dialect, &payload, &mut control).map(|_| ())
            } else {
                let encoding = if operation.ends_with("binary") { SnapshotEncoding::Binary } else { SnapshotEncoding::Text };
                (codec.import)("fixture.retained/v1", &dialect, value_database(value), encoding, &mut control).map(|_| ())
            }
        }));
        assert!(result.is_ok(), "{} skipped owner retirement", case["id"]);
        assert_eq!(result.unwrap().is_ok(), case["succeeds"].as_bool().unwrap(), "{}", case["id"]);
        RETIREMENTS.with(|count| assert_eq!(count.get() as u64, case["retirements"].as_u64().unwrap(), "{}", case["id"]));
    }
}

```

## 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🦀️.rs

```rust
use super::*;
use crate::sqlite_snapshot::{SqliteDatabase, SqliteDatabaseLimits, SqliteRow, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase};
use crate::os_dsl::schema::{FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};

struct DecodedBuffers { buffers: Vec<Vec<u8>> }
std::thread_local! { static COMPLETED_DECODERS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
std::thread_local! { static ORDINARY_DECODER_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; static CONTROLLED_DECODER_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

fn buffer_spec() -> RecordSpec {
    RecordSpec::new(Some("buffers"), RecordLayout::Lines, vec![FieldSpec::new(1, "buffers", Shape::List(Box::new(Shape::Bytes64)))])
}
fn buffer_spec_controlled<C:crate::os_dsl::NativeSchemaControl>(control:&mut C)->Result<RecordSpec,String>{control.scoped_stage(|control|{control.begin_stage(1)?;let mut fields=control.allocate_vec(1)?;let shape=Shape::List(crate::os_dsl::schema::producer::boxed(Shape::Bytes64,control)?);fields.push(crate::os_dsl::schema::producer::field(1,"buffers",shape,control)?);control.step()?;crate::os_dsl::schema::producer::record(Some("buffers"),RecordLayout::Lines,fields,control)})}
fn buffer_spec_producer()->crate::os_dsl::RecordSpecProducer{crate::os_dsl::RecordSpecProducer{ordinary:buffer_spec,decoding:|control|buffer_spec_controlled(control),encoding:|control|buffer_spec_controlled(control)}}
#[test]
fn sqlite_snapshot_erased_export_dispatches_only_the_declared_controlled_native_owner(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let case=&fixture["erasedDispatch"];
 let snapshot=DecodedBuffers{buffers:vec![vec![1;case["bufferBytes"].as_u64().unwrap()as usize]]};let bytes=snapshot.encode_pack();
 let dialect=crate::io_schema::ArtifactDialect{artifact_kind:"fixture.buffers".into(),standard:"1".into(),subset:"*".into()};
 ORDINARY_DECODER_CALLS.with(|count|count.set(0));CONTROLLED_DECODER_CALLS.with(|count|count.set(0));
 let outcome=(DecodedBuffers::sqlite_codec().export)("fixture.buffers/v1",&dialect,&crate::io_schema::IoPayload::Binary(bytes.clone()),&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 assert_eq!(outcome.value.table("decoded_buffer").unwrap().single_row().unwrap().blob(1).unwrap(),snapshot.buffers[0]);
 ORDINARY_DECODER_CALLS.with(|count|assert_eq!(count.get()as u64,case["ordinaryCalls"].as_u64().unwrap()));CONTROLLED_DECODER_CALLS.with(|count|assert_eq!(count.get()as u64,case["controlledCalls"].as_u64().unwrap()));
 let mut reached=false;let result=(DecodedBuffers::sqlite_codec().export)("fixture.buffers/v1",&dialect,&crate::io_schema::IoPayload::Binary(bytes),&mut SqliteSnapshotControl::new(&mut |event|{if event.phase==SqliteSnapshotPhase::DecodeNative&&event.completed>=case["cancelAt"].as_u64().unwrap()as usize{reached=true;false}else{true}},SqliteDatabaseLimits::default()));
 assert!(result.is_err());assert!(reached);
}

impl ArtifactDsl for DecodedBuffers {
    const EXTENSION: &'static str = "buffers";
    fn envelope_id() -> &'static str { "fixture.buffers" }
    fn parse_dsl(_: &str) -> Result<Self, TextError> { Err(TextError { message: "binary admission fixture".into(), span: Default::default(), expected: None }) }
    fn print_dsl(&self) -> String { String::new() }
}

impl ArtifactPack for DecodedBuffers {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let mut record = RecordValue::default();
        record.fields.insert(1, FieldValue::List(self.buffers.iter().cloned().map(FieldValue::Bytes64).collect()));
        let body=pack_rt::encode_document(&buffer_spec(), &record, options)?;
        let envelope=semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),semio_format::Component::Pack,1).unwrap();
        Ok(semio_format::wrap_binary(&envelope,&body))
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        ORDINARY_DECODER_CALLS.with(|count|count.set(count.get()+1));
        let(envelope,body)=semio_format::unwrap_binary(bytes).map_err(|error|PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(Self::envelope_id(),semio_format::Component::Pack,1){return Err(PackError::Schema("fixture envelope identity".into()));}
        let (mut record, _) = pack_rt::decode_document(&body, &buffer_spec(), options)?;
        let Some(FieldValue::List(fields)) = record.fields.remove(&1) else { return Err(PackError::Schema("expected buffers".into())); };
        let buffers = fields.into_iter().map(|field| match field { FieldValue::Bytes64(bytes) => Ok(bytes), _ => Err(PackError::Schema("expected intrinsic octets".into())) }).collect::<Result<Vec<_>, _>>()?;
        COMPLETED_DECODERS.with(|count| count.set(count.get() + 1));
        Ok(Self { buffers })
    }
}

impl ArtifactSqliteSnapshot for DecodedBuffers {
    const SQLITE_SCHEMA: &'static str = include_str!("../🧬️schema/🗄️.sql");
    fn decode_sqlite_snapshot_native(payload:&crate::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
        CONTROLLED_DECODER_CALLS.with(|count|count.set(count.get()+1));
        decode_sqlite_snapshot_record_native(payload,Self::envelope_id(),buffer_spec_producer(),|record,native|{
            let Some(FieldValue::List(fields))=record.get(1)else{return Err(crate::os_dsl::__rt::field_error("expected buffers"));};
            let mut buffers=native.allocate_vec(fields.len()).map_err(crate::os_dsl::__rt::field_error)?;
            for field in fields{native.step().map_err(crate::os_dsl::__rt::field_error)?;let FieldValue::Bytes64(bytes)=field else{return Err(crate::os_dsl::__rt::field_error("expected intrinsic octets"));};buffers.push(native.copy_bytes(bytes).map_err(crate::os_dsl::__rt::field_error)?);}
            COMPLETED_DECODERS.with(|count|count.set(count.get()+1));Ok(Self{buffers})
        },control)
    }
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        let count = self.buffers.iter().try_fold(0usize, |count, bytes| count.checked_add(bytes.len()).ok_or("octet count overflow"))?;
        control.check_value_bytes(count)?;
        control.check_rows(self.buffers.len())?;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, self.buffers.len())?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA).map_err(|error| error.to_string())?;
        for (index, bytes) in self.buffers.iter().enumerate() {
            let id = i64::try_from(index + 1).map_err(|error| error.to_string())?;
            database.table_mut("decoded_buffer")?.rows.push(SqliteRow { rowid: id, values: vec![SqliteValue::Integer(id), SqliteValue::Blob(bytes.clone())] });
        }
        Ok(database)
    }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        Ok(Self { buffers: database.table("decoded_buffer")?.rows.iter().map(|row| row.blob(1).map(<[u8]>::to_vec)).collect::<Result<_, _>>()? })
    }
}

#[test]
fn sqlite_snapshot_native_decoding_admits_compressed_aggregate_before_projection() {
    use std::{io::Write, process::{Command, Stdio}};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let schema: serde_json::Value = serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).unwrap();
    let dialect = crate::io_schema::ArtifactDialect { artifact_kind: "fixture.buffers".into(), standard: "1".into(), subset: "*".into() };
    for case in fixture["cases"].as_array().unwrap() {
        let snapshot = DecodedBuffers { buffers: case["bufferBytes"].as_array().unwrap().iter().map(|value| vec![0; value.as_u64().unwrap() as usize]).chain(std::iter::repeat_with(Vec::new).take(case["emptyBuffers"].as_u64().unwrap() as usize)).collect() };
        let mut options = PackEncodeOptions::default();
        options.codec = crate::os_pack::CodecId(1);
        options.chunk_threshold = u64::MAX;
        options.frame_size = 32768;
        let bytes = snapshot.encode_pack_with(&options).unwrap();
        let maximum = case["maximumBytes"].as_u64().unwrap() as usize;
        assert!(bytes.len() < maximum, "fixture must exercise decoded expansion");
        let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let sqlite = crate::sqlite_snapshot::export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
        let script = "import {Database} from 'bun:sqlite';import Ajv from 'ajv/dist/2020.js';const input=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(input.schema,input.fixture))throw Error('fixture');const db=Database.deserialize(Buffer.from(input.sqlite,'base64'));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');await Bun.write(Bun.stdout,String(db.query('SELECT SUM(length(octets)) AS n FROM decoded_buffer').get().n));db.close();";
        let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        let input = serde_json::json!({"schema": schema, "fixture": fixture, "sqlite": protocol::bytes::encode_base64(&sqlite)});
        child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();
        let result = child.wait_with_output().unwrap();
        assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
        assert_eq!(String::from_utf8(result.stdout).unwrap(), snapshot.buffers.iter().map(Vec::len).sum::<usize>().to_string());
        let mut limits = SqliteDatabaseLimits::default();
        limits.max_value_bytes = maximum;
        COMPLETED_DECODERS.with(|count| count.set(0));
        let result = (DecodedBuffers::sqlite_codec().export)("fixture.buffers/v1", &dialect, &crate::io_schema::IoPayload::Binary(bytes.clone()), &mut SqliteSnapshotControl::new(&mut |_| true, limits));
        let decoded = case["decoded"].as_bool().unwrap();
        assert_eq!(result.is_ok(), decoded, "{}: {result:?}", case["id"]);
        COMPLETED_DECODERS.with(|count| assert_eq!(count.get(), usize::from(decoded), "{}", case["id"]));
        COMPLETED_DECODERS.with(|count| count.set(0));
        let result = (DecodedBuffers::sqlite_codec().export)("fixture.buffers/v1", &dialect, &crate::io_schema::IoPayload::Binary(bytes), &mut SqliteSnapshotControl::new(&mut |event| event.phase != SqliteSnapshotPhase::DecodeNative, limits));
        assert!(result.is_err());
        COMPLETED_DECODERS.with(|count| assert_eq!(count.get(), 0));
    }
}

#[test]
fn sqlite_snapshot_native_decoding_admits_inline_empty_collection_storage() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["emptyBuffers"].as_u64().unwrap() > 0).unwrap();
    let mut record = RecordValue::default();
    record.fields.insert(1, FieldValue::List(std::iter::repeat_with(|| FieldValue::Bytes64(Vec::new())).take(case["emptyBuffers"].as_u64().unwrap() as usize).collect()));
    let bytes = pack_rt::encode_record_body(&buffer_spec(), &record, &PackEncodeOptions::default()).unwrap();
    let mut options = PackDecodeOptions::default();
    options.limits.max_total_alloc = case["maximumBytes"].as_u64().unwrap();
    assert!((bytes.len() as u64) < options.limits.max_total_alloc);
    assert!(pack_rt::decode_record_body(&bytes, &buffer_spec(), &options).is_err(), "empty intrinsic buffers still require owned list slots");
}

#[test]
fn sqlite_snapshot_metadata_admits_final_file_before_mutating_domain_database() {
    use std::{io::Write, process::{Command, Stdio}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).unwrap();
    let rows=fixture["metadataAdmission"]["domainRows"].as_u64().unwrap() as usize;
    let file_rows=fixture["metadataAdmission"]["fileRows"].as_u64().unwrap() as usize;
    let snapshot=DecodedBuffers{buffers:std::iter::repeat_with(Vec::new).take(rows).collect()};
    let database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_rows:rows,..Default::default()})).unwrap();
    let dialect=crate::io_schema::ArtifactDialect{artifact_kind:"fixture.buffers".into(),standard:"1".into(),subset:"*".into()};
    for limits in [SqliteDatabaseLimits{max_rows:rows,..Default::default()},SqliteDatabaseLimits{max_value_bytes:1,..Default::default()},SqliteDatabaseLimits{max_tables:1,..Default::default()},SqliteDatabaseLimits{max_columns:5,..Default::default()},SqliteDatabaseLimits{max_schema_bytes:1,..Default::default()}] {
        let mut bounded=database.clone();
        assert!(semio_framework_os_kernel::io::io_mechanism::attach_sqlite_snapshot_metadata(&mut bounded,&dialect,crate::sqlite_snapshot::SnapshotEncoding::Binary,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());
        assert_eq!(bounded,database);
    }
    let mut canceled=database.clone();
    assert!(semio_framework_os_kernel::io::io_mechanism::attach_sqlite_snapshot_metadata(&mut canceled,&dialect,crate::sqlite_snapshot::SnapshotEncoding::Binary,&mut SqliteSnapshotControl::new(&mut |_|false,Default::default())).is_err());
    assert_eq!(canceled,database);
    let mut admitted=database;
    let limits=SqliteDatabaseLimits{max_rows:file_rows,..Default::default()};
    semio_framework_os_kernel::io::io_mechanism::attach_sqlite_snapshot_metadata(&mut admitted,&dialect,crate::sqlite_snapshot::SnapshotEncoding::Binary,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap();
    let bytes=crate::sqlite_snapshot::export_sqlite_database(&admitted,limits,&mut |_|true).unwrap();
    let script="import {Database} from 'bun:sqlite';import Ajv from 'ajv/dist/2020.js';const x=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).validate(x.schema,x.fixture))throw Error('fixture');const db=Database.deserialize(Buffer.from(x.bytes,'base64'));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');await Bun.write(Bun.stdout,String(db.query('SELECT (SELECT COUNT(*) FROM decoded_buffer)+(SELECT COUNT(*) FROM semio_snapshot) AS n').get().n));db.close();";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture,"schema":schema,"bytes":protocol::bytes::encode_base64(&bytes)}).to_string().as_bytes()).unwrap();
    let result=child.wait_with_output().unwrap();assert!(result.status.success(),"{}",String::from_utf8_lossy(&result.stderr));assert_eq!(String::from_utf8(result.stdout).unwrap(),file_rows.to_string());
}

```

