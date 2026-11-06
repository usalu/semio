import fs from"node:fs";import path from"node:path";
const root="🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget",dir=root+"/🧪️tests";fs.mkdirSync(dir,{recursive:true});
const cases={cases:[{grant:9,fuel:11,spent:3,work:4,accepted:true,remaining:6,remainingFuel:7},{grant:2,fuel:11,spent:3,work:4,accepted:false,remaining:0,remainingFuel:0},{grant:9,fuel:3,spent:3,work:4,accepted:false,remaining:0,remainingFuel:0},{grant:0,fuel:0,spent:0,work:0,accepted:true,remaining:0,remainingFuel:0}]};
const props=Object.fromEntries(["grant","fuel","spent","work","remaining","remainingFuel"].map(k=>[k,{type:"integer",minimum:0}]));props.accepted={type:"boolean"};const schema={type:"object",required:["cases"],additionalProperties:false,properties:{cases:{type:"array",minItems:4,items:{type:"object",required:Object.keys(props),additionalProperties:false,properties:props}}}};
for(const[n,s]of[["🔣️.json",JSON.stringify(cases)],["🧬️.json",JSON.stringify(schema)],["🟦️.ts",'import{test,expect}from"bun:test";import Ajv from"ajv";import{Database}from"bun:sqlite";import corpus from"./🔣️.json";import schema from"./🧬️.json";test("return budget binding conserves its exact admitted stock",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const db=new Database(":memory:");for(const row of corpus.cases){const q=db.query("select (?1<=?2 and ?3<=?4) as accepted, ?2-?1 as remaining, ?4-?3 as fuel").get(row.spent,row.grant,row.work,row.fuel)as{accepted:number,remaining:number,fuel:number};expect(Boolean(q.accepted)).toBe(row.accepted);if(row.accepted){expect(q.remaining).toBe(row.remaining);expect(q.fuel).toBe(row.remainingFuel);}}db.close();});']]){const p=dir+"/"+n;if(fs.existsSync(p)||path.resolve(p).length>256)throw Error("new boundary path exists/limit");fs.writeFileSync(p,s);}
const law=`use super::*;
#[derive(Clone,Copy)]
struct Origin{recipient:usize,grant:RetainedCloneGrant,fuel:u64,spent:P,work:u128}
impl OriginalReturnOrigin for Origin{
 fn same_origin(&self,other:&Self)->bool{self.recipient==other.recipient}
 fn recipient(&self)->usize{self.recipient}
 fn granted(&self)->RetainedCloneGrant{self.grant}
 fn fuel(&self)->u64{self.fuel}
 fn spent(&self)->P{self.spent}
 fn work(&self)->u128{self.work}
}
#[test]
fn typed_budget_binding_keeps_original_destination_and_conserved_stock(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let mut budget=OriginalReturnBudget::<Origin>::empty(1);
  let input=Origin{recipient:&budget as*const _ as usize,grant:RetainedCloneGrant{maximum_items:row["grant"].as_u64().unwrap()as usize,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1},fuel:row["fuel"].as_u64().unwrap(),spent:P{copied_items:row["spent"].as_u64().unwrap()as usize,..Default::default()},work:u128::from(row["work"].as_u64().unwrap())};
  let foreign=Origin{recipient:input.recipient+1,..input};
  assert!(budget.bind(foreign).is_err());assert!(budget.origin.is_none());
  assert_eq!(budget.bind(input).is_ok(),row["accepted"].as_bool().unwrap());
  if row["accepted"]==true{assert_eq!(budget.bind(input).unwrap(),false);let(g,fuel)=budget.remaining().unwrap();assert_eq!(g.maximum_items,row["remaining"].as_u64().unwrap()as usize);assert_eq!(fuel,row["remainingFuel"].as_u64().unwrap());}
 }
}
`;fs.writeFileSync(dir+"/🦀️.rs",law);fs.writeFileSync(import.meta.dir+"/return-budget-boundary-2026-11-06.md","# Return Budget Boundary\n\nSchema-first accounting cases and a native binding law precede extraction. The new first-party owner contains generic typed budget state, pure accounting, guarded storage and physical close. Context retains private Source input capture, identity/clock qualification and actual claim/witness minting. It cannot be replaced with caller numbers. Original tests remain enabled; limits stay unchanged. Native compilation and real Source integration are required.\n");console.log("Authored typed return budget boundary contract.");
