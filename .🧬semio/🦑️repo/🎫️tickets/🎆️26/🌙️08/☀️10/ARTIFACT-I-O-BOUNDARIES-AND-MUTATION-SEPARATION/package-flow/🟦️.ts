import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import fixtures from "./🔣️.json";
test("package semantic roots and Space exports have no physical package codec",()=>{
 for(const model of fixtures.packages){
  const source=readFileSync(join(model.source,"🦀️.rs"),"utf8");
  expect(source).not.toContain("package_from_schema");expect(source).not.toContain("ARTIFACT_DEFINITION_SCHEMA");expect(source).not.toContain("from_json_str");
 }
 const host=readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs","utf8");
 expect(host).not.toContain("package_from_schema");expect(host).not.toContain("ARTIFACT_DEFINITION_SCHEMA");
});
test("Flow registry cache ingestion receives typed dictionaries",()=>{
 const source=readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs","utf8");
 expect(source).not.toContain("output_json");expect(source).not.toContain("from_json_str");
 expect(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🌱️seed/🦀️.rs","utf8")).toContain("output: Dictionary");
});

import Ajv from "ajv";
const control=()=>({maximumBytes:8192,maximumNodes:64,maximumDepth:8,chunk:16,cancelled:()=>false,progress:()=>{},yield:async()=>{}});
for(const model of fixtures.packages){
 test(model.owner+" typed identity and physical ingress agree with independent JSON Schema",async()=>{
  const cap=model.owner[0]!.toUpperCase()+model.owner.slice(1);
  const semantic=await import(join(process.cwd(),model.source,"🧬️schema/📦️package/🟦️.ts"));
  const io=await import(join(process.cwd(),model.source,"🚪️io/📝️text/📦️package/🟦️.ts"));
  const admit=semantic["admit"+cap+"PackageDeclaration"],decode=io["decode"+cap+"PackageJson"];
  const schema=JSON.parse(readFileSync(join(model.source,"🧬️schema/📦️package/🔣️.json"),"utf8"));
  const validate=new Ajv({strict:true}).compile(schema);
  const source=readFileSync(join(model.source,"🧬️schema/📜️artifact-definition.json"),"utf8");
  expect(validate(JSON.parse(source))).toBe(true);
  expect(await decode(source,control())).toEqual(admit(model.declaration));
  const wrong=[{...model.declaration,id:"wrong"},{...model.declaration,definition_version:2},{...model.declaration,dependencies:["alien"]},{...model.declaration,unknown:true}];
  for(const value of wrong){expect(validate(value)).toBe(false);await expect(decode(JSON.stringify(value),control())).rejects.toThrow();}
  await expect(decode(source.replace('"id":', '"id":"wrong","id":'),control())).rejects.toThrow();
  await expect(decode(source,{...control(),maximumBytes:1})).rejects.toThrow();
  await expect(decode(source,{...control(),cancelled:()=>true})).rejects.toThrow();
  console.log("[DEBUG]",model.owner,"canonical+4refusals+duplicate+budget+cancellation");
 });
}
test("Flow typed seed preserves independent JSON outputs and exact dictionary identity",async()=>{
 const {seedFlowEvalNodeCache}=await import(join(process.cwd(),"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🌱️seed/🟦️.ts"));
 const entries=new Map<bigint,object>();const port={seed:(key:bigint,value:object)=>{entries.set(key,value)}};
 for(const output of fixtures.flow.outputs){seedFlowEvalNodeCache(port,17n,output);expect(entries.get(17n)).toBe(output);expect(JSON.parse(JSON.stringify(entries.get(17n)))).toEqual(output);}
 console.log("[DEBUG] Flow typed seed retained",entries.size,"entry");
});
