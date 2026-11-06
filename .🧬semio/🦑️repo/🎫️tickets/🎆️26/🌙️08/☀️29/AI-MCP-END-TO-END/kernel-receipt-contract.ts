import fs from"node:fs";import path from"node:path";const root="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🌀️poll-kernel-turn";
const cases=[{aggregate:8,reserved:6,grant:3,work:7,reservedWork:5,fuel:3,accepted:true,fresh:2,freshWork:2},{aggregate:2,reserved:3,grant:3,work:1,reservedWork:0,fuel:3,accepted:false,fresh:0,freshWork:0},{aggregate:5,reserved:1,grant:3,work:1,reservedWork:0,fuel:3,accepted:false,fresh:0,freshWork:0},{aggregate:2,reserved:1,grant:3,work:2,reservedWork:3,fuel:3,accepted:false,fresh:0,freshWork:0},{aggregate:2,reserved:1,grant:3,work:6,reservedWork:2,fuel:3,accepted:false,fresh:0,freshWork:0}];
const properties=Object.fromEntries(["aggregate","reserved","grant","work","reservedWork","fuel","fresh","freshWork"].map(n=>[n,{type:"integer",minimum:0}]));properties.accepted={type:"boolean"};const schema={type:"object",required:["cases"],additionalProperties:false,properties:{cases:{type:"array",minItems:5,items:{type:"object",required:Object.keys(properties),additionalProperties:false,properties}}}};
for(const[n,v]of[["🔣️.json",JSON.stringify({cases})],["🧬️.json",JSON.stringify(schema)]]){const p=root+"/"+n;if(fs.existsSync(p)||path.resolve(p).length>256)throw Error("path");fs.writeFileSync(p,v);}
fs.writeFileSync(root+"/🟦️.ts",'import{test,expect}from"bun:test";import Ajv from"ajv";import{Database}from"bun:sqlite";import corpus from"./🔣️.json";import schema from"./🧬️.json";test("kernel receipt conserves fresh currencies while output fuel reports reducer work",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const db=new Database(":memory:");for(const row of corpus.cases){const value=db.query("select (?1>=?2 and ?1-?2<=?3 and ?4>=?5 and ?4-?5<=?6) as accepted, ?1-?2 as fresh, ?4-?5 as freshWork").get(row.aggregate,row.reserved,row.grant,row.work,row.reservedWork,row.fuel)as{accepted:number,fresh:number,freshWork:number};expect(Boolean(value.accepted)).toBe(row.accepted);if(row.accepted){expect(value.fresh).toBe(row.fresh);expect(value.freshWork).toBe(row.freshWork);}}db.close();});');
const file=root+"/🦀️.rs",old=fs.readFileSync(file,"utf8");const law=`
#[cfg(test)]
mod receipt_tests{
 use super::*;
 #[test]
 fn kernel_receipt_keeps_fresh_conservation_and_reserved_aggregate_work(){
  let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
  for case in corpus["cases"].as_array().unwrap(){
   let input=serde_json::from_value(serde_json::json!({"operation":1,"generation":1,"epoch":1,"work":{"clockBinding":1,"absoluteDeadlineUs":1,"fuel":case["fuel"]},"grant":{"maximumItems":case["grant"],"maximumCopyBytes":0,"maximumCapacityBytes":0,"maximumReleaseBytes":0,"maximumDepth":1}})).unwrap();
   let aggregate=semio_framework_value::RetainedCloneProgress{copied_items:case["aggregate"].as_u64().unwrap()as usize,..Default::default()};
   let reserved=semio_framework_value::RetainedCloneProgress{copied_items:case["reserved"].as_u64().unwrap()as usize,..Default::default()};
   let result=kernel_work_receipt(input,aggregate,case["work"].as_u64().unwrap(),reserved,case["reservedWork"].as_u64().unwrap());
   assert_eq!(result.is_ok(),case["accepted"].as_bool().unwrap());
   if let Ok(receipt)=result{receipt.validate_for(input).unwrap();assert_eq!(receipt.spent.copied_items,case["fresh"].as_u64().unwrap()as usize);assert_eq!(receipt.work_units_spent,case["freshWork"].as_u64().unwrap());}
  }
 }
}
`;if(fs.readFileSync(file,"utf8")!==old)throw Error("stale");fs.writeFileSync(import.meta.dir+"/🗑️generated/kernel-receipt-law.rs",old+law);fs.renameSync(import.meta.dir+"/🗑️generated/kernel-receipt-law.rs",file);console.log("Authored kernel fresh-receipt law.");
