/** 🧷️ Contract: facts about a shared command closure are shaped, not bytes. A closure member, a data leaf or an entry script is admitted by its physical shape, code is consumed through its analysis fact, the partial inference runs in project scope without an all-or-nothing round; and the observation primitives behave accordingly: a shape fact survives an edit of the bytes, a content fact does not, and a project that raced an edit is stale on its own while the pass completes. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import {inferenceHeaderLine,observeInference,observeInferenceSnapshotV1,observeProjectInferenceV1,observedFs,projectInferenceCountsV1,readInferenceHeaderV1} from "../../../../👁️observation/🟨️.mjs";
import {censusInferenceRecordsV1} from "../../../../../⚡️caching/🕸️graph/🔏️seal/🧮️census/🟨️.mjs";

const python=Bun.which("python3"),here=dirname(fileURLToPath(import.meta.url)),sources=Object.fromEntries(Object.entries(fixture.files).map(([name,path])=>[name,readFileSync(join(here,path),"utf8")]));
const body=(name:string):string=>{
 const row=fixture.functions[name as keyof typeof fixture.functions],source=sources[row.file]!,start=source.indexOf(row.start),end=source.indexOf(row.end,start+row.start.length);
 expect(start,name+" start").toBeGreaterThan(-1);expect(source.indexOf(row.start,start+1),name+" start unique").toBe(-1);expect(end,name+" end").toBeGreaterThan(start);
 return source.slice(start,end);
};
let scratch="";
beforeAll(async()=>{scratch=mkdtempSync(join(realpathSync(tmpdir()),"closure-facts-"));await observeInference({cachePath:join(scratch,"init.json"),key:"closure-facts-initialization"},()=>null);});
afterAll(()=>{rmSync(scratch,{recursive:true,force:true});});

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});
for(const row of fixture.order)test("order: "+row.name,()=>{const text=body(row.function),before=text.indexOf(row.before),after=text.indexOf(row.after);expect(before,"before text").toBeGreaterThan(-1);expect(after,"after text").toBeGreaterThan(-1);expect(before).toBeLessThan(after);});
for(const row of fixture.present)test("present: "+row.name,()=>{expect(body(row.function).includes(row.text)).toBe(true);});
for(const row of fixture.absent)test("absent: "+row.name,()=>{expect(body(row.function).includes(row.text)).toBe(false);});
test("the independent text model reaches the same verdict",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});

test("a shape fact survives an edit of the bytes, a content fact does not",async()=>{
 const file=join(scratch,"shared.ts");writeFileSync(file,"export const a=1;\n");
 let shape=0,content=0;
 const viaShape=()=>observeInference({cachePath:join(scratch,"shape.json"),key:"shape"},()=>{shape++;try{return observedFs.lstatSync(file).isFile();}catch{return false;}});
 const viaBytes=()=>observeInference({cachePath:join(scratch,"bytes.json"),key:"bytes"},()=>{content++;return observedFs.readFileSync(file,"utf8").length;});
 await viaShape();await viaBytes();expect([shape,content]).toEqual([1,1]);
 await Bun.sleep(2100);
 writeFileSync(file,"export const a=2;\n// edited\n");
 await viaShape();await viaBytes();
 expect(shape).toBe(1);expect(content).toBe(2);
 rmSync(file);await viaShape();expect(shape).toBe(2);
});

test("a project that raced an edit is stale on its own while the pass completes in one round",async()=>{
 const shared=join(scratch,"closure.json");writeFileSync(shared,"1");
 const names=["a","b"];
 const pass=(editBetween:boolean)=>observeInferenceSnapshotV1(async()=>{
  const out:string[]=[];
  for(const name of names){
   out.push(await observeProjectInferenceV1({cachePath:join(scratch,"project-"+name+".json"),key:["closure-facts",name],owner:"owner-"+name},()=>observedFs.readFileSync(shared,"utf8")+name));
   if(editBetween&&name==="a")writeFileSync(shared,String(Number(readFileSync(shared,"utf8"))+1));
  }
  return out;
 });
 await Bun.sleep(2100);
 const first=await pass(true);
 expect(first).toEqual(["1a","2b"]);
 const counts=()=>Object.fromEntries(Object.entries(projectInferenceCountsV1()).filter(([name])=>name.startsWith("owner-")));
 expect(counts()).toEqual({"owner-a":1,"owner-b":1});
 await Bun.sleep(2100);
 const second=await pass(false);
 expect(second).toEqual(["2a","2b"]);
 expect(counts()).toEqual({"owner-a":2,"owner-b":1});
});

test("records that read the same closure share its fact blocks: small headers, identical rows, a bad block recomputes and heals",async()=>{
 const library=join(scratch,"library"),np=join(scratch,"np");mkdirSync(library,{recursive:true});
 const closure=Array.from({length:400},(_,i)=>{const path=join(library,"m"+i+".ts");writeFileSync(path,"export const v"+i+"="+i+";\n");return path;});
 const privateFile=(name:string)=>{const path=join(scratch,name);writeFileSync(path,name);return path;};
 const first=privateFile("private-first.txt"),second=privateFile("private-second.txt");
 await Bun.sleep(2200);
 const counts={a:0,b:0};
 const record=(name:"a"|"b",own:string)=>observeInference({cachePath:join(np,name+".json"),key:["blocks",name]},()=>{counts[name]++;let total=0;for(const path of closure)total+=observedFs.readFileSync(path,"utf8").length;return{total,own:observedFs.readFileSync(own,"utf8")};});
 const a=await record("a",first),b=await record("b",second);
 expect([counts.a,counts.b]).toEqual([1,1]);expect(a.total).toBe(b.total);
 const headerA=JSON.parse(inferenceHeaderLine(join(np,"a.json"))!),headerB=JSON.parse(inferenceHeaderLine(join(np,"b.json"))!);
 expect(Object.keys(headerA)).toEqual(["digest","resultSha256","authorityObservations","blocks","observations"]);
 expect(headerA.blocks.length).toBeGreaterThanOrEqual(2);expect(headerA.observations).toEqual([]);
 const shared=headerA.blocks.filter((digest:string)=>headerB.blocks.includes(digest));
 expect(shared.length).toBeGreaterThanOrEqual(headerA.blocks.length-2);
 const rowsA=readInferenceHeaderV1(join(np,"a.json")).observations,rowsB=readInferenceHeaderV1(join(np,"b.json")).observations;
 expect(rowsA.filter((row:any)=>row.kind==="file"&&closure.includes(row.path)).length).toBe(closure.length);
 expect(rowsA.some((row:any)=>row.path===first)).toBe(true);expect(rowsB.some((row:any)=>row.path===second)).toBe(true);
 const blockFiles=readdirSync(np).filter(name=>!name.startsWith("a.")&&!name.startsWith("b.")&&name.endsWith(".json"));
 expect(blockFiles.length).toBeLessThan(headerA.blocks.length+headerB.blocks.length-shared.length+1);
 expect(Buffer.byteLength(inferenceHeaderLine(join(np,"a.json"))!)).toBeLessThan(headerA.blocks.length*80+2000);
 const census=censusInferenceRecordsV1([join(np,"a.json"),join(np,"b.json")]);
 expect(census.records.map(row=>row[2])).toEqual([rowsA.length,rowsB.length]);expect(census.rows.length).toBeLessThan(rowsA.length+rowsB.length);
 const again=await record("a",first);expect(again).toEqual(a);expect(counts.a).toBe(1);
 const victim=join(np,headerA.blocks[0].slice(0,24)+".json"),moduleUrl=new URL("../../../../👁️observation/🟨️.mjs",import.meta.url).href;
 const child=()=>{
  const script="const m=await import(process.env.MODULE);let ran=0;const files=JSON.parse(process.env.FILES);const value=await m.observeInference({cachePath:process.env.CACHE,key:['blocks','a']},()=>{ran++;let total=0;for(const p of files)total+=m.observedFs.readFileSync(p,'utf8').length;return{total,own:m.observedFs.readFileSync(process.env.OWN,'utf8')};});console.log(JSON.stringify({ran,total:value.total}));";
  const result=Bun.spawnSync([process.execPath,"-e",script],{env:{...process.env,MODULE:moduleUrl,CACHE:join(np,"a.json"),FILES:JSON.stringify(closure),OWN:first}});
  return JSON.parse(result.stdout.toString().trim().split("\n").at(-1)!) as {ran:number;total:number};
 };
 expect(child()).toEqual({ran:0,total:a.total});
 writeFileSync(victim,Buffer.from("not a block"));
 expect(child()).toEqual({ran:1,total:a.total});
 expect(child()).toEqual({ran:0,total:a.total});
 expect(brotliDecompressSync(readFileSync(victim)).length).toBeGreaterThan(100);
});
