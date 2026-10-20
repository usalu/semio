import {expect,test} from "bun:test";
import Ajv from "ajv";
import {mkdirSync,mkdtempSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {tmpdir} from "node:os";
import fixture from "./🧫️fixtures/🔣️.json";
import {hermeticShimsV1,type HermeticShimPortV1} from "../../📦️dependencies/🟦️.ts";

const fail=(code:string)=>Object.assign(Error(code),{code});
const check=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["name"],properties:{name:{type:"string"},list:{type:"array",items:{type:"string"}},listError:{type:"string"},targets:{type:"object",additionalProperties:{type:"string"}},expect:{type:"array"},throws:{type:"string"}}});
const port=(row:{list?:string[];listError?:string;targets?:Record<string,string>}):HermeticShimPortV1=>({
 list:()=>{if(row.listError)throw fail(row.listError);return [...(row.list??[])];},
 target:path=>{const value=row.targets![path.split("/").pop()!]!;if(/^E[A-Z]+$/u.test(value))throw fail(value);return value;}
});

test("only an absent .bin and an entry that is not a link mean no shim; every other failure is thrown",()=>{
 for(const row of fixture.cases){
  expect(check(row),row.name+JSON.stringify(check.errors)).toBe(true);
  const call=()=>hermeticShimsV1("/modules",port(row as never));
  if("throws" in row)expect(call,row.name).toThrow(row.throws!);else expect(call(),row.name).toEqual(row.expect);
 }
});

test("on the real file system a link, a regular file and a dangling link behave as declared and a refused directory is thrown",()=>{
 const base=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"shims-"));try{
  const modules=join(base,"node_modules"),bin=join(modules,".bin");mkdirSync(bin,{recursive:true});
  symlinkSync("../tool/bin/tool",join(bin,"tool"));writeFileSync(join(bin,"plain"),"#!/bin/sh\n");symlinkSync("../missing/bin/x",join(bin,"dangling"));
  expect(hermeticShimsV1(modules)).toEqual([["dangling","../missing/bin/x"],["tool","../tool/bin/tool"]]);
  expect(hermeticShimsV1(join(base,"nowhere"))).toEqual([]);
  writeFileSync(join(base,"file"),"x");expect(hermeticShimsV1(join(base,"file"))).toEqual([]);
 }finally{rmSync(base,{recursive:true,force:true});}
});
