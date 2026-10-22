import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../🟦️.ts";

const corpus=(count:number,seed:number):string[]=>{let state=seed;const next=()=>(state=(state*1103515245+12345)&0x7fffffff)/0x7fffffff,rows:string[]=[];for(let index=0;index<count;index++){let text="";for(let step=Math.floor(next()*8);step>0;step--)text+=fixture.atoms[Math.floor(next()*fixture.atoms.length)];rows.push(text);}return rows;};
const compilable=fixture.patterns.filter(pattern=>{try{new RegExp(pattern,"u");return true;}catch{return false;}});

test("the shared relative path pattern gives the declared verdict for every string, in both code unit and unicode readings",()=>{
 const schema={type:"string",pattern:fixture.path.pattern};
 for(const row of fixture.path.verdicts){
  expect(validateJsonSchemaSubset(schema,row.value).length===0,JSON.stringify(row.value)).toBe(row.valid);
  expect(new RegExp(fixture.path.pattern,"u").test(row.value),JSON.stringify(row.value)).toBe(row.valid);
 }
});

test("every distinct schema pattern of the repository decides 300 generated strings exactly like its unicode regular expression",()=>{
 expect(compilable.length).toBeGreaterThan(500);
 const strings=corpus(300,7);
 for(const pattern of compilable){
  const reference=new RegExp(pattern,"u"),schema={type:"string",pattern};
  for(const value of strings)if(reference.test(value)!==(validateJsonSchemaSubset(schema,value).length===0))throw Error(`${pattern} differs on ${JSON.stringify(value)}`);
 }
});

test("a pattern that is not valid in unicode mode is still refused, and the verdicts equal Ajv, an independent validator, on the shared patterns",()=>{
 expect(()=>validateJsonSchemaSubset({type:"string",pattern:"\\-["},"a")).toThrow();
 expect(()=>validateJsonSchemaSubset({type:"string",pattern:"a\\qb"},"a")).toThrow();
 const ajv=new Ajv({strict:false,allErrors:false}),strings=corpus(120,11);
 let compared=0;
 for(const pattern of compilable.slice(0,300)){
  let check;try{check=ajv.compile({type:"string",pattern});}catch{continue;}
  compared++;
  for(const value of strings)if(check(value)!==(validateJsonSchemaSubset({type:"string",pattern},value).length===0))throw Error(`Ajv differs for ${pattern} on ${JSON.stringify(value)}`);
 }
 expect(compared).toBeGreaterThan(100);
});

test("the shared path pattern runs at least twice as fast as its unicode regular expression on repository-sized paths",()=>{
 const paths=Array.from({length:3000},(_,index)=>`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🏎️member-${index}/📦️packages/🦀️rust/📋️project.json`),reference=new RegExp(fixture.path.pattern,"u"),schema={type:"string",pattern:fixture.path.pattern};
 const time=(body:()=>void)=>{const start=performance.now();body();return performance.now()-start;};
 const original=Math.min(time(()=>{for(const path of paths)reference.test(path);}),time(()=>{for(const path of paths)reference.test(path);})),current=Math.min(time(()=>{for(const path of paths)validateJsonSchemaSubset(schema,path);}),time(()=>{for(const path of paths)validateJsonSchemaSubset(schema,path);}));
 expect(current).toBeLessThan(original/2);
});

test("the shared path pattern equals the whole live owner registry and an adversarial corpus, for the original expression, the validator and an independent Python reading of the specification",()=>{
 const registry=resolve(import.meta.dir,"../../../../../../../📋️build-owners.json"),live:string[]=existsSync(registry)?(JSON.parse(readFileSync(registry,"utf8")) as {owners:{root:string;manifest:string;nativeRoots?:string[]}[]}).owners.flatMap(owner=>[owner.root,owner.manifest,...(owner.nativeRoots??[])]):[];
 const edge=["a".repeat(255),"a".repeat(256),"a".repeat(257),`${"a/".repeat(127)}a`,"a/".repeat(300),"👨‍👩‍👧‍👦/x","é/e\u0301","\u202e/x","\u0085","\u000b","\u000c","\u00a0","x\u00a0y","a/\u2028..","a/..\u2028","./a","a/./b",".","...",".../a","a/...","a/../","../","/","//","a/b\\","\\","a\u0000","\u0000","a\rb","\ud800","a\ud800/b","\ude00","🦀️/..","🦀️/..x","x/.. ","x/ ..","a/ /b"],generated=corpus(4000,23),rows=[...live,...edge,...generated];
 const reference=new RegExp(fixture.path.pattern,"u"),schema={type:"string",pattern:fixture.path.pattern},oracle=resolve(import.meta.dir,"../🐍️oracle/🐍️.py");
 const run=spawnSync("python3",["-I",oracle],{input:JSON.stringify(rows),encoding:"utf8",maxBuffer:1<<26});
 expect(run.status,run.stderr).toBe(0);
 const specification=JSON.parse(run.stdout) as boolean[];
 expect(specification.length).toBe(rows.length);
 rows.forEach((value,index)=>{
  const original=reference.test(value),current=validateJsonSchemaSubset(schema,value).length===0;
  if(original!==current||original!==specification[index])throw Error(`verdicts differ on ${JSON.stringify(value)}: regex ${original}, validator ${current}, specification ${specification[index]}`);
 });
 expect(rows.length).toBeGreaterThan(4000);
});
