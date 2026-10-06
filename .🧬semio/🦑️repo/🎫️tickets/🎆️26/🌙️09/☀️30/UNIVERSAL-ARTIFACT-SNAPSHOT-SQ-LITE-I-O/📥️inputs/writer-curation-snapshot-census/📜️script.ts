/** 🧾️ Strengthens both actual newer declaration builders while preserving their complete owning public payload laws. */
import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {resolve,join} from "node:path";
const repo=resolve(import.meta.dir,"../../../../../../../../..");
const owners=[
 {root:"✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer",owner:"Writer",snapshot:"WriterSnapshot",mutation:"crate::WriterMutation",app:"WriterSqliteApps",kind:"s.writer.writer",schema:"writer.document",law:"sqlite_snapshot_writer_real_owned_declaration_public_and_erased_io",tables:["writer_document","writer_document_child"]},
 {root:"✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation",owner:"Curation",snapshot:"CurationSnapshot",mutation:"SourcingMutation",app:"SqliteApps",kind:"s.sourcing.curation",schema:"sourcing.curation/v1",law:"sqlite_snapshot_curation_actual_native_io_declaration_routes_queryable_files",tables:["curation_box","curation_catalog","curation_curated","curation_document","curation_frame","curation_geometry","curation_glb","curation_mesh","curation_mesh_index","curation_mesh_normal","curation_mesh_position","curation_slab","curation_stock_extra","curation_typology_segment"]}
];
const changes:{path:string,before:string|null,after:string}[]=[];
const add=(path:string,after:string)=>{try{readFileSync(path);throw Error("new declaration demand exists: "+path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}changes.push({path,before:null,after});};
const edit=(path:string,transform:(text:string)=>string)=>{const before=readFileSync(path,"utf8");changes.push({path,before,after:transform(before)});};
for(const owner of owners){
 const snapshot=join(repo,owner.root,"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot");
 const fixture={schemaVersion:1,assembly:"Runtime",snapshot:owner.snapshot,dialect:{artifactKind:owner.kind,standard:"1",subset:"*"},schema:owner.schema,tables:owner.tables,requirements:{concreteSnapshotType:true,completeDeclaredInstalledHookIdentity:true,nonemptyExactAuthoredSql:true,nativeSqlLookup:true,oneHopExactRoutesBothDirections:true}};
 const fixtureRoot=join(snapshot,"🧫️fixtures/🪶️sqlite/🧩️declaration");
 add(join(fixtureRoot,"🔣️.json"),JSON.stringify(fixture,null,2)+"\n");
 add(join(fixtureRoot,"🧬️schema/🔣️.json"),JSON.stringify({$schema:"http://json-schema.org/draft-07/schema#",$id:"https://json.schemas.assets.semio-tech.com/"+owner.kind.replaceAll(".","/")+"/snapshot-sqlite-declaration-census.json",title:owner.owner+" Snapshot Declaration Census",const:fixture},null,2)+"\n");
 edit(join(snapshot,"🧪️tests/🪶️sqlite/🟦️.ts"),text=>text+`
test("${owner.owner} declared owner corpus pins complete authored SQL tables independently",async()=>{
 const {default:law}=await import("../../🧫️fixtures/🪶️sqlite/🧩️declaration/🔣️.json");
 const {default:schema}=await import("../../🧫️fixtures/🪶️sqlite/🧩️declaration/🧬️schema/🔣️.json");
 const {default:Ajv}=await import("ajv");
 expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const db=new Database(":memory:");
 try{db.exec(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());expect(db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all()).toEqual(law.tables.map(name=>({name})));expect(db.query("PRAGMA integrity_check").all()).toEqual([{integrity_check:"ok"}]);expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);}finally{db.close();}
 console.log("[DEBUG] ${owner.owner} closed declaration owner "+law.dialect.artifactKind+" schema="+law.schema+" complete_authored_SQL_tables="+law.tables.length);
});
`);
 edit(join(snapshot,"🧪️tests/🪶️sqlite/🦀️.rs"),text=>{
  const start=text.indexOf("async fn "+owner.law+"()");const end=text.indexOf("\n}",start)+2;if(start<0||end<2)throw Error("owning law guard absent");
  let region=text.slice(start,end);
  const opener=region.indexOf("{\n")+2;
  if(opener<2)throw Error("owning law body guard absent");
  region=region.slice(0,opener)+` let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🧩️declaration/🔣️.json")).unwrap();
 let declaration=crate::artifact::<${owner.app}>();
 assert_eq!(declaration.kind.as_str(),law["dialect"]["artifactKind"].as_str().unwrap());assert_eq!(declaration.standards.len(),1);assert_eq!(declaration.standards[0].id.0,law["dialect"]["standard"].as_str().unwrap());assert_eq!(declaration.standards[0].subsets.len(),1);
 let subset=&declaration.standards[0].subsets[0];assert_eq!(subset.dialect.artifact_kind,law["dialect"]["artifactKind"].as_str().unwrap());assert_eq!(subset.dialect.standard.0,law["dialect"]["standard"].as_str().unwrap());assert_eq!(subset.dialect.subset.0,law["dialect"]["subset"].as_str().unwrap());
 let codec=subset.io.native.codec.clone();assert_eq!(codec.schema,law["schema"].as_str().unwrap());let declared=codec.snapshot_sqlite.clone().expect("actual declared Snapshot semantic provider");assert_eq!(declared.snapshot_type,Some(std::any::TypeId::of::<${owner.snapshot}>()));assert!(!${owner.snapshot}::SQLITE_SCHEMA.trim().is_empty());assert_eq!(declared.schema.as_ref(),${owner.snapshot}::SQLITE_SCHEMA);let typed=store::ArtifactCodec::bare::<${owner.snapshot},${owner.mutation}>(law["schema"].as_str().unwrap());assert!(declared.identical_to(typed.snapshot_sqlite.as_ref().unwrap()));
`+region.slice(opener);
  const call=owner.owner==="Writer"?".declare_artifact(crate::artifact())":".declare_artifact(crate::artifact::<SqliteApps>())";
  if(region.split(call).length!==2)throw Error("actual declaration handoff guard absent");region=region.replace(call,".declare_artifact(declaration)");
  const builder="semio_framework_plugin::Plugin::<"+owner.app+">::builder(";if(region.split(builder).length!==2)throw Error("actual owning builder absent");region=region.replace(builder,"let plugin="+builder);
  const insert=region.indexOf("try_build().unwrap();")+"try_build().unwrap();".length;if(insert<"try_build().unwrap();".length)throw Error("actual successful builder guard absent");
  region=region.slice(0,insert)+`
 let installed=store::document_codec(&codec.schema).await.unwrap().expect("actual owning builder installs document codec");assert_eq!(installed.schema,codec.schema);assert!(declared.identical_to(installed.snapshot_sqlite.as_ref().unwrap()));let native=store::io_schema::ArtifactDialect{artifact_kind:law["dialect"]["artifactKind"].as_str().unwrap().into(),standard:law["dialect"]["standard"].as_str().unwrap().into(),subset:law["dialect"]["subset"].as_str().unwrap().into()};assert_eq!(store::io::io_mechanism::native_snapshot_sqlite_schema(&native).unwrap(),${owner.snapshot}::SQLITE_SCHEMA);let sqlite=store::io_schema::ArtifactDialect::from(store::io_schema::SQLITE_SNAPSHOT);for(from,into)in[(&native,&sqlite),(&sqlite,&native)]{let route=store::io::io_mechanism::io_route(from,into,1).await.unwrap().value;assert_eq!(route.hops.len(),1);assert_eq!(route.fidelity,store::io_schema::IoFidelity::Exact);}
 eprintln!("[DEBUG] ${owner.owner} actual generic declaration Snapshot={} schema={} TypeId={:?} sql_bytes={} installed_same_hooks=true direct_exact_directions=2",law["snapshot"].as_str().unwrap(),codec.schema,declared.snapshot_type,declared.schema.len());
`+region.slice(insert);
  region=region.slice(0,-1)+" drop(plugin);\n}";
  return text.slice(0,start)+region+text.slice(end);
 });
}
writeFileSync(join(import.meta.dir,"demand-pairs.json"),JSON.stringify(changes,null,2)+"\n");
for(const change of changes)if(change.before!==null&&readFileSync(change.path,"utf8")!==change.before)throw Error("concurrent owning law source change");
for(const change of changes){mkdirSync(join(change.path,".."),{recursive:true});writeFileSync(change.path,change.after);}
console.log("[DEBUG] exact Writer/Curation actual declaration census demands published paths="+changes.length);
