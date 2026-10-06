/** 🪐️ Publishes a closed two-owner census demand at the genuine Hub Space composition seam. */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
const repo=resolve(import.meta.dir,"../../../../../../../../..");
const hub="🌎️hub/🧩️compositions/🪐️space";
const fixtureDir=hub+"/🧫️fixtures/🪶️snapshot-owner-census";
const rows=[
  {assembly:"Runtime",package:"semio-s-artifact-space-home",snapshot:"SHomeSnapshot",kind:"s.space.home",standard:"1",subset:"*",schema:"s.home",sqlPath:"✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🗄️.sql",tables:["home_document"]},
  {assembly:"Runtime",package:"semio-s-artifact-space-space",snapshot:"SSpaceSnapshot",kind:"s.space.space",standard:"1",subset:"*",schema:"s.space",sqlPath:"✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🗄️.sql",tables:["space_artifact","space_artifact_dialect","space_document"]}
];
const fixture={schemaVersion:1,plugin:"space",package:"semio-hub-space",metadataOnlyDefinitions:[],owners:rows,requirements:{concreteSnapshotType:true,nonemptyAuthoredSql:true,identicalDeclaredInstalledProvider:true,exactNativeSqlLookup:true,directExactRoutesBothDirections:true}};
const changes:{path:string,before:string|null,after:string}[]=[];
const edit=(relative:string,transform:(text:string)=>string)=>{const path=join(repo,relative);const before=readFileSync(path,"utf8");changes.push({path,before,after:transform(before)});};
const add=(relative:string,after:string)=>{const path=join(repo,relative);let before:string|null=null;try{before=readFileSync(path,"utf8");}catch{}if(before!==null)throw Error("new demand exists: "+path);changes.push({path,before,after});};
const replace=(text:string,before:string,after:string)=>{if(text.split(before).length!==2)throw Error("missing exact region: "+before);return text.replace(before,after);};
add(fixtureDir+"/🔣️.json",JSON.stringify(fixture,null,2)+"\n");
add(fixtureDir+"/🧬️schema/🔣️.json",JSON.stringify({$schema:"http://json-schema.org/draft-07/schema#",$id:"https://json.schemas.assets.semio-tech.com/hub/space/snapshot-owner-census.json",title:"Hub Space Composed Snapshot Owners",const:fixture},null,2)+"\n");
edit(hub+"/🧪️tests/🔬️interactive-job-catalog/🦀️.rs",text=>text+`
/// 🪶️ Pins the two actual persisted Runtime owners independently of the five app surfaces.
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_composed_owner_census() {
    use semio_framework_os_kernel::{ArtifactCodec, ArtifactSqliteSnapshot};
    use semio_framework::io::io_mechanism::{io_route, native_snapshot_sqlite_schema};
    use semio_framework::io_schema::{ArtifactDialect, IoFidelity, SQLITE_SNAPSHOT};
    use semio_s_artifact_space_home::{SHomeSnapshot,SHomeMutation};
    use semio_s_artifact_space_space::{SSpaceSnapshot,SSpaceMutation};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️snapshot-owner-census/🔣️.json")).expect("closed neutral composed owner census");
    let owners=fixture["owners"].as_array().unwrap();
    assert_eq!(owners.len(),2);
    let metadata=fixture["metadataOnlyDefinitions"].as_array().unwrap();
    assert!(metadata.is_empty(),"this actual composition currently declares two Runtime owners");
    let expected=owners.iter().map(|row|(row["kind"].as_str().unwrap().to_string(),row["standard"].as_str().unwrap().to_string(),row["subset"].as_str().unwrap().to_string(),row["schema"].as_str().unwrap().to_string())).collect::<BTreeSet<_>>();
    let declarations=[semio_s_artifact_space_home::declaration().await.unwrap(),semio_s_artifact_space_space::declaration().unwrap()];
    let declared_kinds=declarations.iter().map(|row|row.definition().identity().as_str().to_string()).collect::<BTreeSet<_>>();
    assert_eq!(declared_kinds,owners.iter().map(|row|row["kind"].as_str().unwrap().to_string()).collect());
    let bindings=declarations.iter().flat_map(|row|row.document_codec_bindings()).collect::<Vec<_>>();
    assert_eq!(bindings.len(),2);
    let actual=bindings.iter().map(|(dialect,codec)|(dialect.artifact_kind.to_string(),dialect.standard.0.to_string(),dialect.subset.0.to_string(),codec.schema.clone())).collect::<BTreeSet<_>>();
    assert_eq!(actual,expected,"expected membership comes from direct declaration authority");
    let plugin=assembled_plugin();
    assert_eq!(plugin.manifest.plugin_id,fixture["plugin"].as_str().unwrap());
    assert_eq!(plugin.artifact_definitions().definitions().map(|row|row.identity().as_str().to_string()).collect::<BTreeSet<_>>(),declared_kinds,"metadata traversal retains the actual complete definition roster");
    assert_eq!(plugin.manifest.hosted_artifact_kinds.iter().map(|row|(row.id.clone(),row.schema.clone())).collect::<BTreeSet<_>>(),owners.iter().map(|row|(row["kind"].as_str().unwrap().to_string(),row["schema"].as_str().unwrap().to_string())).collect());
    let sqlite=ArtifactDialect::from(SQLITE_SNAPSHOT);
    for(dialect,codec)in bindings {
        let native=ArtifactDialect::from(dialect);
        let declared=codec.snapshot_sqlite.as_ref().expect("every declared Snapshot owns its semantic SQLite provider");
        let(type_id,sql,typed)=match native.artifact_kind.as_str(){
            "s.space.home"=>(std::any::TypeId::of::<SHomeSnapshot>(),SHomeSnapshot::SQLITE_SCHEMA,ArtifactCodec::bare::<SHomeSnapshot,SHomeMutation>("s.home")),
            "s.space.space"=>(std::any::TypeId::of::<SSpaceSnapshot>(),SSpaceSnapshot::SQLITE_SCHEMA,ArtifactCodec::bare::<SSpaceSnapshot,SSpaceMutation>("s.space")),
            _=>panic!("undeclared Runtime owner")
        };
        assert_eq!(declared.snapshot_type,Some(type_id));
        assert!(!sql.trim().is_empty());
        assert_eq!(declared.schema.as_ref(),sql);
        assert!(declared.identical_to(typed.snapshot_sqlite.as_ref().unwrap()),"typed owner and declared complete SQL/hooks differ");
        let installed=semio_framework_os_kernel::document_codec(&codec.schema).await.unwrap().expect("the ordinary composition installs each actual document codec");
        assert_eq!(installed.schema,codec.schema);
        assert!(declared.identical_to(installed.snapshot_sqlite.as_ref().expect("installed semantic provider")));
        assert_eq!(native_snapshot_sqlite_schema(&native).unwrap(),sql);
        for(from,into)in[(&native,&sqlite),(&sqlite,&native)]{
            let route=io_route(from,into,1).await.unwrap().value;
            assert_eq!(route.hops.len(),1);
            assert_eq!(route.fidelity,IoFidelity::Exact);
        }
        eprintln!("[DEBUG] Hub Space composed Snapshot owner dialect={} schema={} concrete_type={:?} sql_bytes={} installed_same_hooks=true direct_exact_directions=2",native.to_coordinate(),codec.schema,declared.snapshot_type,sql.len());
    }
    eprintln!("[DEBUG] Hub Space composed Snapshot census Runtime=2 metadataOnly=0 declaredBindings=2 genuineAppSurfaces={}",plugin.manifest.apps.len());
}
`);
edit(hub+"/📦️packages/🦀️rust/📜️script.ts",text=>{
  text=replace(text,"class InteractiveJobCatalogCheckScript extends BundleScript {",`/** 🪶️ Reads the actual two-owner composition census and runs its registered owning Native law. */
class SnapshotOwnerCensusScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if(segments.length!==1 || !["source","native"].includes(segments[0]!)) throw Error("snapshot-owner-census requires source or native");
    const fixtureRoot=join(this.repoRoot,"🌎️hub/🧩️compositions/🪐️space/🧫️fixtures/🪶️snapshot-owner-census");
    const fixture=JSON.parse(readFileSync(join(fixtureRoot,"🔣️.json"),"utf8")) as {package:string,owners:{package:string,kind:string,sqlPath:string,tables:string[]}[]};
    const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(join(fixtureRoot,"🧬️schema/🔣️.json"),"utf8")));
    assert(validate(fixture),JSON.stringify(validate.errors));
    const manifest=parseToml(readFileSync(join(this.root,"Cargo.toml"),"utf8")) as {package:{name:string},dependencies:Record<string,unknown>};
    assert.equal(manifest.package.name,fixture.package);
    const {Database}=await import("bun:sqlite");
    for(const owner of fixture.owners){
      assert(Object.hasOwn(manifest.dependencies,owner.package));
      const db=new Database(":memory:");
      try{db.exec(readFileSync(join(this.repoRoot,owner.sqlPath),"utf8"));assert.deepEqual(db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all(),owner.tables.map(name=>({name})));assert.deepEqual(db.query("PRAGMA integrity_check").all(),[{integrity_check:"ok"}]);assert.deepEqual(db.query("PRAGMA foreign_key_check").all(),[]);}finally{db.close();}
      console.log("[DEBUG] independent Hub Space census owner "+owner.kind+" package="+owner.package+" authored_SQL_tables="+owner.tables.length);
    }
    if(segments[0]==="native"){
      const receipts=await runRepositoryExactCargoLaws({cwd:this.root,env:{...process.env,RUST_MIN_STACK:"268435456"},groups:[{package:"semio-hub-space",target:{kind:"lib"},laws:["interactive_job_catalog_tests::sqlite_snapshot_composed_owner_census"]}],progress(event){console.log("[DEBUG] Hub Space census Native "+event.stage+" "+(event.law??""));}});
      console.log("[DEBUG] Hub Space census Native receipts "+JSON.stringify(receipts));
    }
  }
}

class InteractiveJobCatalogCheckScript extends BundleScript {`);
  text=replace(text,'            "interactive_job_catalog_tests::every_app_instance_constructs_against_its_registered_proof_catalog",','            "interactive_job_catalog_tests::every_app_instance_constructs_against_its_registered_proof_catalog",\n            "interactive_job_catalog_tests::sqlite_snapshot_composed_owner_census",');
  return replace(text,'.register("interactive-job-catalog-check", InteractiveJobCatalogCheckScript)', '.register("interactive-job-catalog-check", InteractiveJobCatalogCheckScript).register("snapshot-owner-census", SnapshotOwnerCensusScript)');
});
edit(hub+"/📦️packages/🦀️rust/📋️project.json",text=>replace(text,'    "interactive-job-catalog-check": {',`    "snapshot-owner-census-source": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {"cwd": "${hub}/📦️packages/🦀️rust", "command": "bun ./📜️script.ts snapshot-owner-census source"},
      "outputs": []
    },
    "snapshot-owner-census-native": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {"cwd": "${hub}/📦️packages/🦀️rust", "command": "bun ./📜️script.ts snapshot-owner-census native"},
      "outputs": []
    },
    "interactive-job-catalog-check": {`));
for(const relative of [".vscode/launch.json",".vscode/🧩️launch.seed.jsonc"]){
  edit(relative,text=>{
    const marker=relative===".vscode/launch.json"?'    {\n      "name": "⚖️interactive-job-catalog-native-check🧩️compositions🪐️space🦀️",':'    {\n      "name": "⚖️test-snapshot-sqlite-census-all🧩️compositions🗄️stdio🦀️",';
    const entries=[{name:"⚖️snapshot-owner-census-native🧩️compositions🪐️space🦀️",type:"node-terminal",request:"launch",command:"bun nx run @semio-tech/space-plugin:snapshot-owner-census-native",cwd:"${workspaceFolder}",presentation:{group:"4_gate",order:900.05974}},{name:"⚖️snapshot-owner-census-source🧩️compositions🪐️space🦀️",type:"node-terminal",request:"launch",command:"bun nx run @semio-tech/space-plugin:snapshot-owner-census-source",cwd:"${workspaceFolder}",presentation:{group:"4_gate",order:900.05973}}].map(row=>JSON.stringify(row,null,2).split("\n").map(line=>"    "+line).join("\n")+",\n").join("");
    return replace(text,marker,entries+marker);
  });
}
writeFileSync(join(import.meta.dir,"demand-pairs.json"),JSON.stringify(changes,null,2)+"\n");
for(const change of changes)if(change.before!==null && readFileSync(change.path,"utf8")!==change.before)throw Error("concurrent change before guarded census publication");
for(const change of changes){mkdirSync(join(change.path,".."),{recursive:true});writeFileSync(change.path,change.after);}
console.log("[DEBUG] guarded Hub Space two-owner census demand published paths="+changes.length);
