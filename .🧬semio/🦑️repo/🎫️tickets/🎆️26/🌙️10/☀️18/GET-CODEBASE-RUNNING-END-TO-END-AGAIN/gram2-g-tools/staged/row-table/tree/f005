/** 🔢️ Contract: the row table gives equal observation rows (same kind, path and fact text) one row object and one number, and distinct rows distinct numbers, exactly as an independent Python model groups them, also when every row hashes to the same bucket; a row object that is not the canonical one has the number of its canonical row; clearing starts a new numbering. */
import{expect,test}from"bun:test";
import{join}from"node:path";
import Ajv from"ajv";
import{testPythonV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createRowTableV1,factKeyV1,hashTextV1,rowHashV1}from"../🟨️.mjs";
import fixture from"./🔣️.json";
import fixtureSchema from"./🧬️schema/🔣️.json";

type Row={kind:string;path:string;fact:unknown};
const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️.py");
const model=()=>{
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({cases:fixture.cases}))});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return JSON.parse(run.stdout.toString()).cases as {rows:[string,string,unknown][];groups:number[]}[];
};
const numbers=(rows:[string,string,unknown][],options:Parameters<typeof createRowTableV1>[0])=>{
 const table=createRowTableV1<Row>(options),first=new Map<number,Row>(),out:number[]=[];
 for(const [kind,path,fact] of rows){
  const made={kind,path,fact},row=table.canonical(kind,path,fact,()=>made),id=table.id(row);
  out.push(id);if(!first.has(id))first.set(id,row);expect(first.get(id)).toBe(row);
 }
 return{table,out};
};

test("the fixture is closed under its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
});

test("the numbers of the table group the rows exactly as the model does, with the real hash and with every row in one bucket",()=>{
 for(const answer of model())for(const options of[{},{hash:()=>1}]){
  const{out,table}=numbers(answer.rows,options);
  expect(out).toEqual(answer.groups);
  expect(table.size()).toBe(Math.max(...answer.groups));
 }
});

test("a different object of an equal fact has the number of the canonical row and the canonical row is the first one made",()=>{
 const table=createRowTableV1<Row>(),first={kind:"file",path:"/w/f",fact:{size:1,mtimeMs:2,ctimeMs:3,ino:4,sha256:"ab"}},second={kind:"file",path:"/w/f",fact:{size:1,mtimeMs:2,ctimeMs:3,ino:4,sha256:"ab"}};
 expect(table.canonical(first.kind,first.path,first.fact,()=>first)).toBe(first);
 expect(table.canonical(second.kind,second.path,second.fact,()=>second)).toBe(first);
 expect(table.id(second)).toBe(table.id(first));expect(table.size()).toBe(1);
});

test("an unknown row is registered on the first ask for its number, a cleared table numbers again from one",()=>{
 const table=createRowTableV1<Row>(),row={kind:"exists",path:"/w/a",fact:true},other={kind:"exists",path:"/w/b",fact:true};
 expect(table.id(row)).toBe(1);expect(table.id(other)).toBe(2);expect(table.id(row)).toBe(1);
 table.clear();expect(table.size()).toBe(0);expect(table.id(other)).toBe(1);expect(table.id(row)).toBe(2);
});

test("the hash spreads the rows of a workspace-like corpus over many buckets",()=>{
 const seen=new Set<number>();let total=0;
 for(let at=0;at<20000;at++){seen.add(rowHashV1("readdir","/w/"+"d".repeat(120)+at,factKeyV1("x")));total++;}
 expect(seen.size).toBeGreaterThan(total*0.999);
 expect(hashTextV1("",5)).toBe(5);expect(hashTextV1("a",2166136261)).toBe(0xe40c292c);
});
