import fs from"node:fs";import path from"node:path";const base="🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp",dir=base+"/🎚️config/🧪️tests/📥️first-response";fs.mkdirSync(dir,{recursive:true});
const data={items:[{owner:"coreWire",count:21},{owner:"sourceOwners",count:5},{owner:"fourReceiptClaims",count:24},{owner:"exposure",count:6},{owner:"guard",count:32}],work:[{owner:"coreWire",count:2948},{owner:"sourceOwners",count:5},{owner:"fourReceiptClaims",count:24},{owner:"exposure",count:6},{owner:"guard",count:64}],minimumItems:88,minimumWork:3047,producerItems:32,producerWork:1};
const element={type:"object",required:["owner","count"],additionalProperties:false,properties:{owner:{type:"string",minLength:1},count:{type:"integer",minimum:1}}};const schema={type:"object",required:Object.keys(data),additionalProperties:false,properties:{items:{type:"array",minItems:5,items:element},work:{type:"array",minItems:5,items:element},minimumItems:{type:"integer",minimum:1},minimumWork:{type:"integer",minimum:1},producerItems:{type:"integer",minimum:1},producerWork:{type:"integer",minimum:1}}};
const ts='import{test,expect}from"bun:test";import Ajv from"ajv";import{Database}from"bun:sqlite";import quote from"./🔣️.json";import schema from"./🧬️.json";import policy from"../../🏛️host-driver-policy.json";test("authored MCP defaults fund complete first response plus producer headroom",()=>{expect(new Ajv({strict:true}).validate(schema,quote)).toBe(true);const db=new Database(":memory:");for(const[name,expected]of[["items",quote.minimumItems],["work",quote.minimumWork]]as const){const row=db.query("select sum(json_extract(value, \'$.count\')) as total from json_each(?)").get(JSON.stringify(quote[name]))as{total:number};expect(row.total).toBe(expected);}for(const grant of[policy.admission,policy.drive,policy.wake])expect(grant.maximumItems).toBeGreaterThanOrEqual(quote.minimumItems+quote.producerItems);expect(policy.root.turnFuel).toBeGreaterThanOrEqual(quote.minimumWork+quote.producerWork);db.close();});';
const native=`use semio_plugin_guest_contract::original_actor_context::original_core_first_response_reserve;
#[test]
fn authored_mcp_policies_fund_actual_owner_first_response_without_expanding_runtime_grants(){
 let policy:semio_plugin_host::GuestRelayDriverPolicy=semio_framework_pack_json::from_json_str(include_str!("../../🏛️host-driver-policy.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let model:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for length in[144,152,184]{
  let(items,demand,work)=original_core_first_response_reserve(length).unwrap();
  assert!(items>=model["minimumItems"].as_u64().unwrap()as usize);
  assert!(work>=model["minimumWork"].as_u64().unwrap());
  for grant in[policy.admission,policy.drive,policy.wake]{
   assert!(grant.maximum_items>=items+model["producerItems"].as_u64().unwrap()as usize);
   assert!(grant.maximum_copy_bytes>=demand.copy_bytes);
   assert!(grant.maximum_capacity_bytes>=demand.capacity_bytes);
   assert!(grant.maximum_depth>=demand.depth);
  }
  assert!(policy.root.turn_fuel>=work+model["producerWork"].as_u64().unwrap());
 }
}
`;
for(const[n,s]of[["🔣️.json",JSON.stringify(data)],["🧬️.json",JSON.stringify(schema)],["🟦️.ts",ts],["🦀️.rs",native]]){const p=dir+"/"+n;if(fs.existsSync(p)||path.resolve(p).length>256)throw Error("new test path");fs.writeFileSync(p,s);}
fs.writeFileSync(import.meta.dir+"/default-first-response-policy-2026-11-06.md","# Authored First Response Policy\n\nThe common guest owner now publishes a complete first-response reserve. The prior 32-item admission/drive/wake defaults are below the current 88-item structural bound and cannot reach the producer. A schema-first owner breakdown and independent SQLite sum precede the config change. The native law invokes the actual first-party owner for 144/152/184-byte receipts and verifies copy/capacity/depth and producer headroom; its compilation remains pending. Runtime policies must reject undersized caller grants before effects and never auto-expand them.\n");console.log("Authored first-response policy contract and actual-owner native law.");