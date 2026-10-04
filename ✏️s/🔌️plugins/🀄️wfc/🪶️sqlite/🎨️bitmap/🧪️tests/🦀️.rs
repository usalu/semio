//! 🧪️ Independent byte-index edits and full raw-word owner equality share the closed corpus.
use semio_framework_os_kernel::{ArtifactPack,ArtifactSqliteSnapshot,sqlite_snapshot::*};
use std::{io::Write,process::{Command,Stdio}};
pub fn verify<S:ArtifactPack+ArtifactSqliteSnapshot>(source:&S,edited:Option<&S>,prefix:&str,case:&serde_json::Value){
 let expected=source.encode_pack();
 let database=source.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 let restored=S::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 assert_eq!(restored.encode_pack(),expected);
 let bytes=export_sqlite_database(&database,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 let script=r#"import{Database}from'bun:sqlite';const[prefix,json]=Bun.argv.slice(-2),c=JSON.parse(json),db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));try{
 if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('independent integrity');
 const mode=db.query('SELECT storage_kind FROM '+prefix+'_bitmap').get();if(mode.storage_kind!==c.mode)throw Error('storage branch');
 const rows=db.query('SELECT ordinal,palette_index FROM '+prefix+'_bitmap_pixel ORDER BY ordinal').all();if(JSON.stringify(rows)!==JSON.stringify(c.indices.map((palette_index,ordinal)=>({ordinal,palette_index}))))throw Error('complete decoded bytes');
 const literals=db.query('SELECT ordinal,scalar_text FROM '+prefix+'_bitmap_literal_scalar ORDER BY ordinal').all(),expected=c.mode==='literal'?Array.from(c.text).map((scalar_text,ordinal)=>({ordinal,scalar_text})):[];if(JSON.stringify(literals)!==JSON.stringify(expected))throw Error('exact complete native literal');
 if(c.mode==='indices'&&Buffer.from(c.indices).toString('base64')!==c.text)throw Error('independent canonical bytes');
 if(c.id==='three-unknown-index')db.query('UPDATE '+prefix+'_bitmap_pixel SET palette_index=255 WHERE ordinal=1').run();
 await Bun.write(Bun.stdout,db.serialize());
 }finally{db.close();}"#;
 let case_json=case.to_string();let mut process=Command::new("bun").args(["-e",script,prefix,&case_json]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
 process.stdin.take().unwrap().write_all(&bytes).unwrap();let output=process.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
 let physical=import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();
 let actual=S::from_sqlite_database(&physical,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();
 assert_eq!(actual.encode_pack(),edited.unwrap_or(source).encode_pack(),"independent per-pixel edit must preserve every unrelated raw-word field");
 assert_eq!(source.encode_pack(),expected,"complete borrowed input is unchanged");
 for violation in["orphan","duplicate","branch"]{if case["id"]!="three-unknown-index"{break}
  let mut malformed=database.clone();let bitmap=malformed.table(&format!("{prefix}_bitmap")).unwrap().rows[0].rowid;
  match violation{
   "orphan"=>malformed.table_mut(&format!("{prefix}_bitmap_pixel")).unwrap().rows[0].values[1]=SqliteValue::Integer(999),
   "duplicate"=>malformed.table_mut(&format!("{prefix}_bitmap_pixel")).unwrap().rows[1].values[2]=SqliteValue::Integer(0),
   _=>malformed.table_mut(&format!("{prefix}_bitmap_literal_scalar")).unwrap().rows.push(SqliteRow{rowid:1,values:vec![SqliteValue::Integer(1),SqliteValue::Integer(bitmap),SqliteValue::Integer(0),SqliteValue::Text("文".into())]}),
  }
  assert!(S::from_sqlite_database(&malformed,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err(),"bitmap relationship refusal: {violation}");
 }
}
