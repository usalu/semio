import {resolve} from "node:path";
const ticket=resolve(import.meta.dir,"../.."),root=resolve(ticket,"../../../../../../..");
const specs=[
 {id:"generation2d",artifact:"🌀️procedural/🗿️artifacts/🌀️generation2d",kind:"s.procedural.generation2d",type:"Generation2dSnapshot",guard:"Generation2dSnapshotRead",sql:"UPDATE generation_note_widget SET text=?",assignment:'if let semio_framework_artifact_flow_flow::Widget::InputNote{text,..}=&mut literal.host_snapshot.widgets[2]{*text=value.into()}else{panic!("literal note owner")}'},
 {id:"generation3d",artifact:"🌀️procedural/🗿️artifacts/🧊️generation3d",kind:"s.procedural.generation3d",type:"Generation3dSnapshot",guard:"Generation3dSnapshotRead",sql:"UPDATE generation_note_widget SET text=?",assignment:'if let semio_framework_artifact_flow_flow::Widget::InputNote{text,..}=&mut literal.host_snapshot.widgets[2]{*text=value.into()}else{panic!("literal note owner")}'},
 {id:"presentation",artifact:"🎞️animate/🗿️artifacts/🎬️presentation",kind:"s.animate.presentation",type:"PresentationSnapshot",guard:"public_owner::Owned",sql:"UPDATE presentation_tile SET name=? WHERE ordinal=0",assignment:"literal.tiles[0].name=value.into();"},
 {id:"sequence",artifact:"🎬️sequence/🗿️artifacts/🎬️sequence",kind:"s.sequence.sequence",type:"SequenceSnapshot",guard:"public_owner::Owned",sql:"UPDATE sequence_document SET schema=?",assignment:"literal.schema=value.into();"},
 {id:"curation",artifact:"🪵️sourcing/🗿️artifacts/🗂️curation",kind:"s.sourcing.curation",type:"CurationSnapshot",guard:"public_owner::Owned",sql:"UPDATE curation_stock_extra SET name=? WHERE ordinal=0",assignment:"literal.stock_extra[0].name=value.into();"},
];
for(const s of specs.filter(s=>!process.argv[2]||s.id===process.argv[2])){
 const gen=s.id.startsWith("generation");
 const base=resolve(root,"✏️s/🔌️plugins",s.artifact,"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
 const sql=await Bun.file(resolve(base,"🪶️sqlite/🗄️.sql")).text();
 const tables=[...sql.matchAll(/CREATE TABLE\s+([a-zA-Z0-9_]+)/g)].map(m=>m[1]);
 const corpus={metadataColumns:["id","artifact_kind","standard","subset","schema_version","native_encoding"],metadata:{id:1,artifact_kind:s.kind,standard:"1",subset:"*",schema_version:1},nativeEncodings:["binary","text"],domainTables:tables,entityIdentity:"positive-rowid",independentEdit:{sql:s.sql,value:"Public SQLite 日本\u0000literal €"},retirement:"explicit"};
 const fixturePath=resolve(base,"🧫️fixtures/🪶️sqlite/🚦️public/🔣️.json"),schemaPath=resolve(base,"🧫️fixtures/🪶️sqlite/🚦️public/🧬️schema/🔣️.json");
 const schema={$schema:"http://json-schema.org/draft-07/schema#",title:s.id+" Actual Public SQLite Witness",const:corpus};
 const oracle="import{Database}from'bun:sqlite';import Ajv from'ajv';const c=JSON.parse(process.argv[1]),encoding=process.argv[2],schema=JSON.parse(process.argv[3]);if(!new Ajv({strict:true}).validate(schema,c))throw Error('closed neutral public schema');const d=Database.deserialize(await Bun.stdin.bytes(),{safeIntegers:true});const meta=d.query('SELECT * FROM semio_snapshot').all();if(meta.length!==1||JSON.stringify(Object.keys(meta[0]))!==JSON.stringify(c.metadataColumns))throw Error('complete metadata shape');for(const[k,v]of Object.entries({...c.metadata,native_encoding:encoding}))if(meta[0][k]!==((k==='id'||k==='schema_version')?BigInt(v):v))throw Error('metadata '+k);const names=d.query(\"SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name\").all().map(r=>r.name);if(JSON.stringify(names)!==JSON.stringify([...c.domainTables,'semio_snapshot'].sort()))throw Error('complete domain tables');for(const name of c.domainTables){const q='\"'+name.replaceAll('\"','\"\"')+'\"';const rows=d.query('SELECT rowid AS owner_identity,* FROM '+q).all();for(const row of rows)if(row.id<=0n||row.id!==row.owner_identity)throw Error('entity ownership '+name);}if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('public integrity');const changes=d.query(c.independentEdit.sql).run(c.independentEdit.value).changes;if(changes!==1)throw Error('exact authored edit '+changes);if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('edited integrity');await Bun.write(Bun.stdout,d.serialize());d.close();";
 let helper='use super::*;\nuse std::{io::Write,process::{Command,Stdio}};\npub(super) fn corpus()->serde_json::Value{serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/🚦️public/🔣️.json")).unwrap()}\npub(super) fn verify_and_edit(bytes:&[u8],encoding:SnapshotEncoding)->Vec<u8>{\n let fixture=corpus();assert_eq!(fixture["retirement"],"explicit");\n let script=r#"'+oracle+'"#;\n let mut child=Command::new("bun").args(["-e",script,&fixture.to_string(),encoding.as_str(),include_str!("../../../🧫️fixtures/🪶️sqlite/🚦️public/🧬️schema/🔣️.json")]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(bytes).unwrap();let result=child.wait_with_output().unwrap();assert!(result.status.success(),"{}",String::from_utf8_lossy(&result.stderr));result.stdout\n}\n';
 if(!gen)helper+='pub(super) struct Owned(Option<'+s.type+'>);\nimpl Owned{pub(super) fn new(value:'+s.type+')->Self{Self(Some(value))}}\nimpl std::ops::Deref for Owned{type Target='+s.type+';fn deref(&self)->&Self::Target{self.0.as_ref().unwrap()}}\nimpl std::ops::DerefMut for Owned{fn deref_mut(&mut self)->&mut Self::Target{self.0.as_mut().unwrap()}}\nimpl Drop for Owned{fn drop(&mut self){if let Some(value)=self.0.take(){store::ArtifactSqliteSnapshot::retire_sqlite_snapshot(value);}}}\n';
 const helperPath=resolve(base,"🧪️tests/🪶️sqlite/🚦️public/🦀️.rs"),testPath=resolve(base,"🧪️tests/🪶️sqlite/🦀️.rs");
 const before=await Bun.file(testPath).text();if(before.includes("mod public_owner;"))throw Error("public witness already mounted "+s.id);let after=before;
 const call="io_import_sqlite_snapshot::<"+s.type+">";
 const start=after.indexOf("async fn sqlite_snapshot_"+(gen?"procedural_"+s.id:s.id)+"_actual_");
 if(start<0)throw Error("actual public law absent "+s.id);
 const end=after.indexOf("\n}",start)+2;let law=after.slice(start,end);
 if(!gen){
 law=law.replace("let expected=fixture();","let expected=public_owner::Owned::new(fixture());").replace("(&dialect(),&expected,","(&dialect(),&*expected,");
 const old="assert_eq!("+call+"(&dialect(),&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value,expected);";
 if(!law.includes(old))throw Error("public equality guard "+s.id);
 law=law.replace(old,"let restored=public_owner::Owned::new("+call+"(&dialect(),&bytes,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);assert_eq!(*restored,*expected);");
 }
 const anchor=gen?"assert_eq!(project(&restored),project(&source));":"assert_eq!(*restored,*expected);";
 const literal=gen?'{let(host_snapshot,generation)=neutral::fixture(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json"));'+s.type+'{host_snapshot,generation}}':"fixture()";
 const equality=gen?"assert_eq!(project(&restored),project(&literal));":"assert_eq!(*restored,*literal);";
 const addition="let edited=public_owner::verify_and_edit(&bytes,encoding);let restored="+s.guard+"::new("+call+"("+(gen?"&dialect":"&dialect()")+",&edited,SqliteDatabaseLimits::default(),&mut |_|true).await.unwrap().value);let mut literal="+s.guard+"::new("+literal+');let public=public_owner::corpus();let value=public["independentEdit"]["value"].as_str().unwrap();'+s.assignment+";"+equality+'eprintln!("[DEBUG] '+s.id+' actual public six-field metadata / entity ownership / independent SQL edit / explicit retirement encoding={encoding:?}");';
 if(!law.includes(anchor))throw Error("public insertion "+s.id);
 law=law.replace(anchor,anchor+addition);
 after=after.slice(0,start)+law+after.slice(end)+'\n#[path="🚦️public/🦀️.rs"]\nmod public_owner;\n';
 const pairs=[{path:schemaPath,before:null,after:JSON.stringify(schema,null,2)+"\n"},{path:fixturePath,before:null,after:JSON.stringify(corpus,null,2)+"\n"},{path:helperPath,before:await Bun.file(helperPath).exists()?await Bun.file(helperPath).text():null,after:helper},{path:testPath,before,after}];
 await Bun.write(resolve(ticket,"📥️inputs/physical-current-public-witness/"+s.id+"-pair.json"),JSON.stringify(pairs,null,2)+"\n");
 if(process.argv[3]!=="hold")for(const p of pairs)await Bun.write(p.path,p.after);
 console.log("[DEBUG] "+(process.argv[3]==="hold"?"Held":"Mounted")+" existing "+s.id+" public witness, "+tables.length+" exact domain tables");
}

