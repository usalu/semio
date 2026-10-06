import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {resolve,join,dirname} from "node:path";
const repo=resolve(import.meta.dir,"../../../../../../../../.."),base="✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any",mode=process.argv[2];
const fixturePath=base+"/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔗️authored-examples/🔣️.json";
const entries=[{id:"jack",uri:"writer://jack",child:"document-1ce1990e85292c68",text:'MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = "core"\nRETURN a.name, b.name',asset:"🖼️assets/🎬️demo/🗣️.dsl.semio"},{id:"dag-jack",uri:"writer://dag-jack",child:"document-21ed2f21032d3ec2",text:'MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = "core"\nRETURN a, b',asset:"📚️examples/🎬️demo/🖼️assets/🧪️dag-example/🗣️.dsl.semio"}];
const fixture={examples:entries.map(entry=>({asset:entry.asset,snapshot:{schema:"writer.document",id:entry.id,languageId:"jack",uri:entry.uri,text:entry.text,document:{childId:entry.child,target:{artifactId:entry.child,dialect:{artifactKind:"s.stdio.semio",standard:"v1",subset:"document"}}}}}))};
const pairs:{path:string,before:string,after:string}[]=[];
function pair(path:string,after:string){let before="";try{before=readFileSync(join(repo,path),"utf8")}catch{}pairs.push({path,before,after});}
if(mode==="demand"){
 pair(fixturePath,JSON.stringify(fixture,null,2)+"\n");
 pair(fixturePath.replace("/🔣️.json","/🧬️schema/🔣️.json"),JSON.stringify({$schema:"http://json-schema.org/draft-07/schema#",const:fixture},null,2)+"\n");
 const source=base+"/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts";
 pair(source,readFileSync(join(repo,source),"utf8")+`\ntest("Writer original authored examples keep every reference component and full owner",async()=>{
 const {default:law}=await import("../../🧫️fixtures/🪶️sqlite/🔗️authored-examples/🔣️.json");const {default:schema}=await import("../../🧫️fixtures/🪶️sqlite/🔗️authored-examples/🧬️schema/🔣️.json");const {default:Ajv}=await import("ajv");expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 for(const entry of law.examples){const snapshot=parseWriterSnapshot(entry.snapshot);const db=Database.deserialize(await exportSqliteDatabase(await writerSnapshotToSqliteDatabase(snapshot)));try{expect(db.query("PRAGMA integrity_check").all()).toEqual([{integrity_check:"ok"}]);expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT child_id,target_artifact_id,target_artifact_kind,target_standard,target_subset FROM writer_document_child").get()).toEqual({child_id:snapshot.document.childId,target_artifact_id:snapshot.document.target.artifactId,target_artifact_kind:snapshot.document.target.dialect.artifactKind,target_standard:snapshot.document.target.dialect.standard,target_subset:snapshot.document.target.dialect.subset});expect(await writerSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(entry.snapshot);}finally{db.close();}}
 console.log("[DEBUG] Writer authored examples complete owners=2 current literal reference fields=4 independently queryable");
});\n`);
 const native=base+"/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs";
 pair(native,readFileSync(join(repo,native),"utf8")+`\n#[test]
fn sqlite_snapshot_writer_authored_examples_are_exact_current_references(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔗️authored-examples/🔣️.json")).unwrap();let examples=law["examples"].as_array().unwrap();assert_eq!(examples.len(),2);
 for entry in examples{let s=&entry["snapshot"];let expected=crate::writer_snapshot_with_text(s["schema"].as_str().unwrap(),s["id"].as_str().unwrap(),s["languageId"].as_str().unwrap(),s["uri"].as_str().unwrap(),s["text"].as_str().unwrap());println!("[DEBUG] Writer actual authored producer id={} canonical={:?}",expected.id,crate::document_dsl::print_writer_dsl(&expected));assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&expected)).unwrap(),*s);}
 for(entry,asset)in examples.iter().zip([crate::document_dsl::JACK_EXAMPLE_TEXT,crate::document_dsl::DAG_JACK_EXAMPLE_TEXT]){let actual=crate::document_dsl::parse_writer_dsl(asset).expect("original handcrafted example must parse through its actual owner");assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&actual)).unwrap(),entry["snapshot"]);assert_eq!(crate::document_dsl::print_writer_dsl(&actual),asset);assert_eq!(actual.document.child_id,actual.document.target.artifact_id);}
 println!("[DEBUG] Writer original two authored DSL assets preserve complete neutral owners and exact current four-field references");
}\n`);
}else if(mode==="provider"){
 for(const entry of entries){const path=base+"/"+entry.asset,before=readFileSync(join(repo,path),"utf8"),old='target="'+entry.child+'!s.stdio.semio@v1/document"';if(before.split(old).length!==2)throw Error("exact authored target guard "+path);pair(path,before.replace(old,"target=artifact-id="+entry.child+" artifact-kind=s.stdio.semio standard=v1 subset=document"));}
 const path=base+"/🚪️io/📸️snapshot/📝️text/🦀️.rs",before=readFileSync(join(repo,path),"utf8");if(before.split("unwrap_or_else(|_| schema::empty_writer_snapshot())").length!==3)throw Error("exact owning getter guard");pair(path,before.replace("parse_dsl(JACK_EXAMPLE_TEXT).unwrap_or_else(|_| schema::empty_writer_snapshot())","parse_dsl(JACK_EXAMPLE_TEXT).expect(\"handcrafted current Writer jack example\")").replace("parse_dsl(DAG_JACK_EXAMPLE_TEXT).unwrap_or_else(|_| schema::empty_writer_snapshot())","parse_dsl(DAG_JACK_EXAMPLE_TEXT).expect(\"handcrafted current Writer DAG example\")"));
}else throw Error("demand or provider required");
writeFileSync(join(import.meta.dir,mode+"-guarded-pairs.json"),JSON.stringify({pairs},null,2)+"\n");
for(const p of pairs){let current="";try{current=readFileSync(join(repo,p.path),"utf8")}catch{}if(current!==p.before)throw Error("concurrent authored fixture guard "+p.path);}
for(const p of pairs){mkdirSync(dirname(join(repo,p.path)),{recursive:true});writeFileSync(join(repo,p.path),p.after);}
console.log("[DEBUG] Writer authored original reference "+mode+" mounted paths="+pairs.length);
