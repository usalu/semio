import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {join,dirname} from "node:path";
import assert from "node:assert/strict";
const repo="/Users/ueli/Documents/semio",ticket=join(repo,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O"),root=join(repo,"🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/📸️snapshot");
const observed=JSON.parse(readFileSync(join(ticket,"🗑️generated/workflow-independent-semantic-cell-extents.json"),"utf8"));
assert.equal(observed.length,10);
const expected=[1348,1370,1370,1370,1370,1370,1490,1490,1238,1238];observed.forEach((row:{bytes:number;tables:{rows:number}[]},at:number)=>{assert.equal(row.bytes,expected[at]);assert.equal(row.tables.length,16);assert.equal(row.tables.reduce((n,table)=>n+table.rows,0),30)});
const law={version:1,extent:"Complete domain SQL INTEGER/REAL=8, NULL=0, TEXT=UTF8 bytes; includes surrogate keys, every relationship and IEEE query/word/class companion",tables:16,rows:30,cases:observed,nativeBacking:{maximumBytes:4194304,repeatedOwnerships:2,tinyAllocationBytes:[0,1],requestIncludesReallocFullSize:true}};
const fixture=join(root,"🧫️fixtures/🪶️sqlite/📏️semantic-cells/🔣️.json"),schema=join(root,"🧫️fixtures/🪶️sqlite/📏️semantic-cells/🧬️schema/🔣️.json");
const pairs:{path:string;before:string|null;after:string}[]=[{path:fixture,before:null,after:JSON.stringify(law,null,2)+"\n"},{path:schema,before:null,after:JSON.stringify({$schema:"http://json-schema.org/draft-07/schema#",$id:"https://json.schemas.assets.semio-tech.com/framework/workflow/snapshot-semantic-cells.json",const:law},null,2)+"\n"}];
const source=join(root,"🧪️tests/🪶️sqlite/🟦️.ts"),sourceBefore=readFileSync(source,"utf8");
const sourceLaw=String.raw`
test("Workflow complete independent SQL cell extent admits equality and refuses one byte short",async()=>{
 const cases=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/📏️semantic-cells/🔣️.json",import.meta.url)).json(),schema=await Bun.file(new URL("../../🧫️fixtures/🪶️sqlite/📏️semantic-cells/🧬️schema/🔣️.json",import.meta.url)).json();expect(new Ajv({strict:false}).validate(schema,cases)).toBe(true);
 for(const item of cases.cases){const expected=snapshot();if(item.name!=="base"){const word={bits:BigInt("0x"+item.name)};for(const node of expected.graph.nodes)for(const key of["x","y","width","height"]as const)node[key]=word;for(const parameter of expected.parameters)if(parameter.type==="numeric"){parameter.value=word;parameter.min=word;parameter.max=word;parameter.step=word}}
  const data=await bytes(expected),db=Database.deserialize(data,{safeIntegers:true});try{
   expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
   const names=db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all()as{name:string}[],tables=names.map(({name})=>{const rows=db.query('SELECT * FROM "'+name+'" ORDER BY id').all();let bytes=0;for(const row of rows)for(const value of Object.values(row))bytes+=value===null?0:typeof value==="string"?Buffer.byteLength(value):8;return{table:name,rows:rows.length,bytes}});expect(tables).toEqual(item.tables);expect(tables.length).toBe(cases.tables);expect(tables.reduce((n,table)=>n+table.rows,0)).toBe(cases.rows);expect(tables.reduce((n,table)=>n+table.bytes,0)).toBe(item.bytes);
  }finally{db.close()}
  const database=await ports.workflowSnapshotToSqliteDatabase(expected);await expect(ports.workflowSnapshotToSqliteDatabase(expected,{maxValueBytes:item.bytes})).resolves.toEqual(database);await expect(ports.workflowSnapshotToSqliteDatabase(expected,{maxValueBytes:item.bytes-1})).rejects.toThrow();await expect(ports.workflowSnapshotFromSqliteDatabase(database,{maxValueBytes:item.bytes})).resolves.toEqual(expected);await expect(ports.workflowSnapshotFromSqliteDatabase(database,{maxValueBytes:item.bytes-1})).rejects.toThrow();console.log("[DEBUG] Workflow exact complete SQL semantic boundary case="+item.name+" bytes="+item.bytes+" tables=16 rows=30");
 }
});
`;
pairs.push({path:source,before:sourceBefore,after:sourceBefore+sourceLaw});
const native=join(root,"🧪️tests/🪶️sqlite/🦀️.rs"),nativeBefore=readFileSync(native,"utf8");
const nativeLaw=String.raw`
fn cell_laws()->serde_json::Value{serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/📏️semantic-cells/🔣️.json")).unwrap()}
#[test]
fn sqlite_snapshot_workflow_complete_semantic_cells_are_exact_for_every_ieee_branch(){
 let law=cell_laws();for case in law["cases"].as_array().unwrap(){let mut expected=source();if case["name"]!="base"{let bits=u64::from_str_radix(case["name"].as_str().unwrap(),16).unwrap();let number=f64::from_bits(bits);for node in&mut expected.graph.nodes{node.x=number;node.y=number;node.width=number;node.height=number;}for parameter in&mut expected.parameters{if let crate::WorkflowParameter::Numeric{value,min,max,step,..}=parameter{*value=number;*min=Some(number);*max=Some(number);*step=Some(number)}}}
 let bytes=usize::try_from(case["bytes"].as_u64().unwrap()).unwrap();let defaults=SqliteDatabaseLimits::default();let d=database(&expected);assert_eq!(d.tables.len(),usize::try_from(law["tables"].as_u64().unwrap()).unwrap());assert_eq!(d.tables.iter().map(|table|table.rows.len()).sum::<usize>(),usize::try_from(law["rows"].as_u64().unwrap()).unwrap());
 for allowance in[bytes,bytes-1]{let limits=SqliteDatabaseLimits{max_value_bytes:allowance,..defaults};assert_eq!(expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),allowance==bytes,"project {}",case["name"]);assert_eq!(WorkflowSnapshot::from_sqlite_database(&d,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),allowance==bytes,"reconstruct {}",case["name"]);for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{assert_eq!(expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),allowance==bytes,"encode {}",case["name"]);assert_eq!(WorkflowSnapshot::decode_sqlite_snapshot_native(&payload(&expected,encoding),&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),allowance==bytes,"decode {}",case["name"]);}}
 println!("[DEBUG] Workflow full semantic native frontier case={} complete_bytes={bytes}",case["name"]);
 }
}
#[path="💰️backing/🦀️.rs"]mod allocation;
#[test]
fn sqlite_snapshot_workflow_reconstruction_pays_actual_system_requests_cumulatively(){
 let law=cell_laws();let backing=&law["nativeBacking"];assert_eq!(backing["requestIncludesReallocFullSize"],true);let maximum=usize::try_from(backing["maximumBytes"].as_u64().unwrap()).unwrap();let expected=source();let database=database(&expected);let defaults=SqliteDatabaseLimits{max_allocation_bytes:maximum,..SqliteDatabaseLimits::default()};let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,defaults);
 let(result,requested)=allocation::observe(||WorkflowSnapshot::from_sqlite_database(&database,&mut control));assert_eq!(result.unwrap(),expected);let admitted=maximum-control.allocation_remaining_bytes();println!("[DEBUG] Workflow actual system backing requests={requested} caller_admitted={admitted} semantic_cells_separate=true");assert!(admitted>0,"real Workflow indexes and typed allocations must settle in the caller ledger");assert_eq!(admitted,requested,"every complete source/typed backing request must be admitted");
 for allowance in[admitted,admitted-1,0,1]{let limits=SqliteDatabaseLimits{max_allocation_bytes:allowance,..defaults};let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,limits);let(result,requested)=allocation::observe(||WorkflowSnapshot::from_sqlite_database(&database,&mut control));if allowance==admitted{assert_eq!(result.unwrap(),expected);assert_eq!(requested,admitted);assert_eq!(control.allocation_remaining_bytes(),0);}else{assert_eq!(result.unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert!(control.allocation_remaining_bytes()<=allowance);}}
 let repetitions=usize::try_from(backing["repeatedOwnerships"].as_u64().unwrap()).unwrap();assert_eq!(repetitions,2);let total=admitted.checked_mul(repetitions).unwrap();let mut accept=|_|true;let mut control=SqliteSnapshotControl::new(&mut accept,SqliteDatabaseLimits{max_allocation_bytes:total,..defaults});for _ in 0..repetitions{let(result,requested)=allocation::observe(||WorkflowSnapshot::from_sqlite_database(&database,&mut control));assert_eq!(result.unwrap(),expected);assert_eq!(requested,admitted);}assert_eq!(control.allocation_remaining_bytes(),0);assert_eq!(WorkflowSnapshot::from_sqlite_database(&database,&mut control).unwrap_err().kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);
}
`;
pairs.push({path:native,before:nativeBefore,after:nativeBefore+nativeLaw});
const allocator=readFileSync(join(repo,"✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/💰️backing/🦀️.rs"),"utf8").replaceAll("Curation","Workflow");
pairs.push({path:join(root,"🧪️tests/🪶️sqlite/💰️backing/🦀️.rs"),before:null,after:allocator});
writeFileSync(join(import.meta.dir,"guarded-pairs.json"),JSON.stringify(pairs,null,2)+"\n");
for(const pair of pairs){let current:string|null=null;try{current=readFileSync(pair.path,"utf8")}catch{}assert.equal(current,pair.before,"Concurrent Workflow closed demand change")}
for(const pair of pairs){mkdirSync(dirname(pair.path),{recursive:true});writeFileSync(pair.path,pair.after)}
console.log("[DEBUG] Workflow complete16table10case semantic and actualSystem cumulative backing demands mounted paths="+pairs.length+" production_mutations=0");
