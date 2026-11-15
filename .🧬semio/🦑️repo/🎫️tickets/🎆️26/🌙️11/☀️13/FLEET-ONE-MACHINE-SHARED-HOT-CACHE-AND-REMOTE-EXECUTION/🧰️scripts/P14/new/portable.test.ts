import Ajv from"ajv";
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import{aboveRootV1,dependencyGenerationV1,localRowsV1,portableRowsPolicyV1,portableRowsV1}from"../🟦️.ts";

/** 🧳️ Contract: a record is published without the facts that name the producer's machine and is replayed with the taker's own dependency generation. The fixture is language-agnostic; a Python oracle derives the same rows from the schema alone. */
const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/Fixture",definitions:schema.definitions});

test("the fixture and the policy are valid against the schema",()=>{
 expect(validate(fixture)).toBe(true);
 expect(new Ajv({strict:false}).validate({$ref:"#/definitions/Policy",definitions:schema.definitions},portableRowsPolicyV1)).toBe(true);
});

test("every case publishes the portable rows and replays them with the generation of the taker",()=>{
 for(const row of fixture.cases){
  const portable=portableRowsV1(row.rows,row.root);
  expect(portable,row.name).toEqual(row.portable);
  expect(localRowsV1(portable,row.generation??undefined),row.name).toEqual(row.local);
 }
});

test("only directories above the checkout are location rows",()=>{
 expect(aboveRootV1("/Users","/Users/Shared/semio")).toBe(true);
 expect(aboveRootV1("/","/Users/Shared/semio")).toBe(true);
 expect(aboveRootV1("/Users/Shared/semio","/Users/Shared/semio")).toBe(false);
 expect(aboveRootV1("/Users/Shared/semio2","/Users/Shared/semio")).toBe(false);
 expect(aboveRootV1("C:\\work","C:\\work\\semio")).toBe(true);
 expect(aboveRootV1("/Users/Shared/semio/.cache","/Users/Shared/semio")).toBe(false);
});

test("the production shape: a record of one machine holds on a machine of another platform and checkout after the rewrite",()=>{
 const producer=fixture.cases[0]!,rows=portableRowsV1(producer.rows,producer.root),taker=localRowsV1(rows,"8714ffcdb7fdf62c3f57");
 const text=JSON.stringify(taker);
 expect(text).not.toContain("86415d695aa883fffc22");expect(text).not.toContain("\"/Users\"");expect(text).toContain("8714ffcdb7fdf62c3f57/node_modules/typescript/lib/typescript.js");
 expect(taker.filter(row=>row.kind==="file")).toEqual(producer.rows.filter(row=>row.kind==="file"));
});

test("the generation of a machine is the real path of its node_modules; without one nothing is claimed",()=>{
 const base=mkdtempSync(join(tmpdir(),"portable-rows-"));
 try{
  const root=join(base,"root"),generation=join(base,"root",".cache","b","d","m","j","0123456789abcdef0123","node_modules");
  mkdirSync(generation,{recursive:true});symlinkSync(generation,join(root,"node_modules"));
  expect(dependencyGenerationV1(realpathSync(root))).toBe("0123456789abcdef0123");
  expect(dependencyGenerationV1(join(base,"missing"))).toBeUndefined();
  mkdirSync(join(base,"plain","node_modules"),{recursive:true});expect(dependencyGenerationV1(join(base,"plain"))).toBeUndefined();
  expect(dependencyGenerationV1("/x",()=>"/x/.cache/b/d/m/j/aaaaaaaaaaaaaaaaaaaa/node_modules")).toBe("aaaaaaaaaaaaaaaaaaaa");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the Python oracle derives the same rows from the schema",()=>{
 const here=(name:string)=>fileURLToPath(new URL(name,import.meta.url)),run=spawnSync("python3",["-I",here("./🐍️.py"),here("../🧬️schema/🔣️.json"),here("./🔣️.json")],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const oracle=JSON.parse(run.stdout) as {name:string;portable:unknown;local:unknown}[];
 expect(oracle.map(row=>row.portable)).toEqual(fixture.cases.map(row=>portableRowsV1(row.rows,row.root)));
 expect(oracle.map(row=>row.local)).toEqual(fixture.cases.map(row=>localRowsV1(portableRowsV1(row.rows,row.root),row.generation??undefined)));
});
