import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()

def sub(old, new, count=1):
    global s
    n = s.count(old)
    if n != count:
        sys.exit("expected %d got %d for: %s" % (count, n, old[:90]))
    s = s.replace(old, new)

sub("use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;",
    "use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::{Fem3dSnapshot,Fem3dSnapshotValue};\nuse store::os_store::serialization::ArtifactSqliteSnapshotHost as _;")
sub("fn state()->Fem3dSnapshot{", "fn state()->Fem3dSnapshotValue{")
sub(" Fem3dSnapshot{\n  nodes:", " Fem3dSnapshotValue{\n  nodes:")
sub("fn every_word(s:&mut Fem3dSnapshot,", "fn every_word(s:&mut Fem3dSnapshotValue,")

old_pr = s[s.index("fn project(s:&Fem3dSnapshot)"):s.index("fn every_word")]
new_pr = (
"struct Own(Option<Fem3dSnapshot>);\n"
"impl Own{fn new(value:Fem3dSnapshot)->Self{Self(Some(value))}fn get(&self)->&Fem3dSnapshot{self.0.as_ref().unwrap()}}\n"
"impl std::ops::Deref for Own{type Target=Fem3dSnapshot;fn deref(&self)->&Fem3dSnapshot{self.get()}}\n"
"impl Drop for Own{fn drop(&mut self){if let Some(value)=self.0.take(){<Fem3dSnapshot as store::os_store::serialization::ArtifactSqliteSnapshotHost>::retire_sqlite_snapshot(value);}}}\n"
"fn own(v:&Fem3dSnapshotValue)->Own{Own::new(Fem3dSnapshot::from_reference(v.clone()).unwrap())}\n"
"fn project_native(s:&Fem3dSnapshot)->SqliteDatabase{original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|s.to_sqlite_database(sqlite_control)).unwrap()}\n"
"fn project(s:&Fem3dSnapshotValue)->SqliteDatabase{project_native(&own(s))}\n"
"fn restore(d:&SqliteDatabase)->Result<Own,semio_framework_value::ValueError>{original_owners::with_sqlite_recipient(SqliteDatabaseLimits::default(),&mut |_|true,|sqlite_control|Fem3dSnapshot::from_sqlite_database(d, sqlite_control)).map(Own::new)}\n")
s = s.replace(old_pr, new_pr)

s = s.replace("project(&restore(", "project_native(&restore(")
sub("assert_eq!(restore(&project(&s)).unwrap(),s);", "assert_eq!(project_native(&restore(&project(&s)).unwrap()),project(&s));")

sub("let provider=codec.snapshot_sqlite.unwrap();", "let provider=codec.snapshot_sqlite.as_ref().unwrap();")
sub("let d=original_owners::with_decoding(|native_owner|(provider.export)(&schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;",
    "let d=original_owners::export_sqlite(provider,&schema,&dialect,&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;")
sub("let actual=original_owners::with_encoding(|native_owner|(provider.import)(&schema,&dialect, &mut Some(d),encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default()), native_owner)).unwrap().value;",
    "let actual=original_owners::import_sqlite(provider,&schema,&dialect,d,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap().value;")

sub("let d=project(&s);for phase in[SqliteSnapshotPhase::ProjectSnapshot", "let o=own(&s);let d=project_native(&o);for phase in[SqliteSnapshotPhase::ProjectSnapshot")
sub("s.to_sqlite_database(&mut c).map(|_|())", "o.to_sqlite_database(&mut c).map(|_|())")
sub("_=>original_owners::with_encoding(|native_owner|s.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut c, native_owner)).map(|_|())};",
    "_=>original_owners::encode_native(&*o,SnapshotEncoding::Text,&mut c).map(|_|())};")
sub("|sqlite_control|s.to_sqlite_database(sqlite_control)).is_err());assert!(original_owners::with_encoding(|native_owner|s.encode_sqlite_snapshot_native(SnapshotEncoding::Binary,&mut SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_rows:2,..SqliteDatabaseLimits::default()}), native_owner)).is_err());assert!(original_owners::with_encoding(|native_owner|s.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1024,..SqliteDatabaseLimits::default()}), native_owner)).is_err());}",
    "|sqlite_control|o.to_sqlite_database(sqlite_control)).is_err());assert!(original_owners::encode_native(&*o,SnapshotEncoding::Binary,&mut SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_rows:2,..SqliteDatabaseLimits::default()})).is_err());assert!(original_owners::encode_native(&*o,SnapshotEncoding::Text,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1024,..SqliteDatabaseLimits::default()})).is_err());}")

sub("assert!(original_owners::with_decoding(|native_owner|Fem3dSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut cb,SqliteDatabaseLimits::default()), native_owner)).is_err());",
    "assert!(original_owners::decode_native::<Fem3dSnapshot>(&payload,&mut SqliteSnapshotControl::new(&mut cb,SqliteDatabaseLimits::default())).is_err());")
sub("assert!(original_owners::with_decoding(|native_owner|Fem3dSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1024,..SqliteDatabaseLimits::default()}), native_owner)).is_err());",
    "assert!(original_owners::decode_native::<Fem3dSnapshot>(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1024,..SqliteDatabaseLimits::default()})).is_err());")

a = s.index("    struct Owned(Option<Fem3dSnapshot>);")
b = s.index("    fn exact(")
s = s[:a] + "    use super::{Own as Owned,own};\n" + s[b:]
sub("let expected=Owned::new(value);", "let expected=own(&value);")
sub("let expected=Owned::new(state());", "let expected=own(&state());")

c = s.index('\n#[path="../../../../../../../../../../../../../🧰️framework')
s = s[:c].rstrip("\n") + "\n"
open(p, "w", encoding="utf-8", newline="\n").write(s)
