import Ajv from"ajv";
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{fileURLToPath}from"node:url";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import{readFileSync}from"node:fs";
import{hostBoundV1,platformClassPolicyV1,platformClassV1,recordPlatformV1,servesPlatformV1,splitRequestV1}from"../🟦️.ts";

/** 🌍️ Contract: every platform shares the neutral records, a host-bound project is keyed by the exact platform, a machine serves the requests of its own class whole and the requests of another class except the bound owners. The fixture is language-agnostic; a Python oracle derives the same verdicts from the schema alone. */
const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/Fixture",definitions:schema.definitions});

test("the fixture and the policy are valid against the schema",()=>{
 expect(validate(fixture)).toBe(true);
 expect(new Ajv({strict:false}).validate({$ref:"#/definitions/Policy",definitions:schema.definitions},platformClassPolicyV1)).toBe(true);
});

test("every case gives the record platform and class of the fixture",()=>{
 for(const row of fixture.cases){
  expect(recordPlatformV1(row.keyText,row.platform),row.name).toBe(row.record);
  expect(platformClassV1(row.platform),row.name).toBe(row.class);
 }
 expect(hostBoundV1(fixture.cases[3]!.keyText)).toBe(true);expect(hostBoundV1(fixture.cases[0]!.keyText)).toBe(false);
});

test("a machine serves the requests of its own class whole",()=>{
 for(const row of fixture.serves)expect(servesPlatformV1(row.server,row.requester),`${row.server} ${row.requester}`).toBe(row.serves);
});

test("across classes a machine serves every project but the bound owners, which it names and leaves",()=>{
 for(const row of fixture.splits){
  const split=splitRequestV1(row.projects,row.server,row.requester);
  expect(split.served,row.name).toEqual(row.served);expect(split.left,row.name).toEqual(row.left);
 }
});

test("the bound owners of the policy are the projects of the repository that declare a host platform output",()=>{
 const declaring=new URL("../../../../../../../💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📋️project.json",import.meta.url);
 const project=JSON.parse(readFileSync(declaring,"utf8")) as {name:string};
 expect(readFileSync(declaring,"utf8")).toContain(platformClassPolicyV1.hostBound[0]);
 expect(platformClassPolicyV1.boundOwners).toEqual([project.name]);
});

test("the Python oracle derives the same verdicts from the schema",()=>{
 const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),run=spawnSync("python3",["-I",here("./🐍️.py"),here("../🧬️schema/🔣️.json"),here("./🔣️.json")],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const oracle=JSON.parse(run.stdout) as {cases:{name:string;record:string;class:string}[];serves:{serves:boolean}[]};
 expect(oracle.cases.map(row=>[row.record,row.class])).toEqual(fixture.cases.map(row=>[recordPlatformV1(row.keyText,row.platform),platformClassV1(row.platform)]));
 expect(oracle.serves.map(row=>row.serves)).toEqual(fixture.serves.map(row=>servesPlatformV1(row.server,row.requester)));
 const split=oracle as unknown as {splits:{served:string[];left:string[]}[]};
 expect(split.splits.map(row=>[row.served,row.left])).toEqual(fixture.splits.map(row=>{const found=splitRequestV1(row.projects,row.server,row.requester);return[found.served,found.left];}));
});
