import Ajv from"ajv";
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{fileURLToPath}from"node:url";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import{hostBoundV1,platformClassPolicyV1,platformClassV1,recordPlatformV1,servesPlatformV1}from"../🟦️.ts";

/** 🌍️ Contract: the POSIX hosts share records and requests, Windows and unknown platforms stay alone, and a host-bound project is keyed by the exact platform. The fixture is language-agnostic; a Python oracle derives the same verdicts from the schema alone. */
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

test("a machine serves the requests of its own class and of no other",()=>{
 for(const row of fixture.serves)expect(servesPlatformV1(row.server,row.requester),`${row.server} ${row.requester}`).toBe(row.serves);
});

test("the Python oracle derives the same verdicts from the schema",()=>{
 const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),run=spawnSync("python3",["-I",here("./🐍️.py"),here("../🧬️schema/🔣️.json"),here("./🔣️.json")],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const oracle=JSON.parse(run.stdout) as {cases:{name:string;record:string;class:string}[];serves:{serves:boolean}[]};
 expect(oracle.cases.map(row=>[row.record,row.class])).toEqual(fixture.cases.map(row=>[recordPlatformV1(row.keyText,row.platform),platformClassV1(row.platform)]));
 expect(oracle.serves.map(row=>row.serves)).toEqual(fixture.serves.map(row=>servesPlatformV1(row.server,row.requester)));
});
