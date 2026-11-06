import fs from"node:fs";
const root="🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress";
const dir=root+"/🧪️tests/🧫️fixtures/📥️borrow";
const corpus={cases:[{declared:1,occupied:0,incoming:true,bytes:1,accepted:true,remaining:false},{declared:1,occupied:1,incoming:true,bytes:4096,accepted:false,remaining:true},{declared:2,occupied:1,incoming:true,bytes:19,accepted:true,remaining:false},{declared:1,occupied:0,incoming:false,bytes:0,accepted:false,remaining:false},{declared:1,occupied:0,incoming:true,bytes:0,accepted:true,remaining:false}]};
const schema={type:"object",required:["cases"],additionalProperties:false,properties:{cases:{type:"array",minItems:5,items:{type:"object",required:["declared","occupied","incoming","bytes","accepted","remaining"],additionalProperties:false,properties:{declared:{type:"integer",minimum:1,maximum:2},occupied:{type:"integer",minimum:0,maximum:1},incoming:{type:"boolean"},bytes:{type:"integer",minimum:0,maximum:4096},accepted:{type:"boolean"},remaining:{type:"boolean"}}}}}};
fs.mkdirSync(dir,{recursive:true});for(const[n,v]of[["🔣️.json",JSON.stringify(corpus)],["🧬️.json",JSON.stringify(schema)]]){if(fs.existsSync(dir+"/"+n))throw Error("exists");fs.writeFileSync(dir+"/"+n,v);}
const test='import{test,expect}from"bun:test";import Ajv from"ajv";import{Database}from"bun:sqlite";import corpus from"./🔣️.json";import schema from"./🧬️.json";test("command page remains with its input until admission succeeds",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const db=new Database(":memory:");for(const row of corpus.cases){const result=db.query("select (?1 and ?2 < ?3) as accepted, (?1 and ?2 >= ?3) as remaining").get(Number(row.incoming),row.occupied,row.declared)as{accepted:number,remaining:number};expect(Boolean(result.accepted)).toBe(row.accepted);expect(Boolean(result.remaining)).toBe(row.remaining);}db.close();});';fs.writeFileSync(dir+"/🟦️.ts",test);
const path=root+"/🧪️tests/🔬️unit/🦀️.rs",old=fs.readFileSync(path,"utf8"),law=`
#[test]
fn command_page_input_moves_only_after_actual_page_admission(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📥️borrow/🔣️.json")).unwrap();
 for case in corpus["cases"].as_array().unwrap(){
  let mut pages=CommandPageSet::try_new(case["declared"].as_u64().unwrap()as usize).unwrap();
  for _ in 0..case["occupied"].as_u64().unwrap(){pages.try_push(FixedCommandPage::try_copy_from(&[1]).unwrap()).unwrap();}
  let cursor=CommandPageCursor{owner:7,generation:11,command_index:0,command_count:1,instance:13,seq:17,kind:19,page_index:0,page_count:1,item_count:0,metadata:0};
  let bytes=vec![19;case["bytes"].as_u64().unwrap()as usize];
  let mut input=case["incoming"].as_bool().unwrap().then(||(cursor,FixedCommandPage::try_copy_from(&bytes).unwrap()));
  let original=input.as_ref().map(|(_,page)|page.as_slice().as_ptr());
  let accepted=pages.try_push_input(&mut input).is_ok_and(|cursor|cursor.is_some());
  assert_eq!(accepted,case["accepted"].as_bool().unwrap());
  assert_eq!(input.is_some(),case["remaining"].as_bool().unwrap());
  if accepted{assert_eq!(pages.pages.back().unwrap().as_slice().as_ptr(),original.unwrap());assert_eq!(pages.pages.back().unwrap().as_slice(),bytes)}
  else if let Some((_,page))=input.as_ref(){assert_eq!(Some(page.as_slice().as_ptr()),original);assert_eq!(page.as_slice(),bytes)}
 }
}
`;if(fs.readFileSync(path,"utf8")!==old)throw Error("stale");fs.writeFileSync(import.meta.dir+"/🗑️generated/command-input-law.rs",old+law);fs.renameSync(import.meta.dir+"/🗑️generated/command-input-law.rs",path);
fs.writeFileSync(import.meta.dir+"/command-input-custody-2026-11-05.md","# Command Input Custody\n\nAuthored schema-first page ownership cases and a native physical-pointer law before the borrowed admission implementation. A pending or rejected page must remain at its actual caller input; accepted admission must preserve the original 4096-byte backing rather than cloning it. SQLite provides an independent decision oracle.\n\nA separate critical gap remains: a decoded page cannot outlive its OriginalActorInputReceiving decoder ledger merely by a Rust move. The final command path needs a checked same-Source decoder ownership handoff, or a retained stable input owner whose actual page remains borrowed through command execution. Current raw page-set readers also contain unqualified drop/release paths. Borrowed admission alone does not qualify that transfer.\n");console.log("Authored command page custody contract and native pointer law.");
