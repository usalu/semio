/** 🧮️ Contract: the seal's fact table holds exactly the distinct facts of every inference record and receipt (a naive serial set and Python agree on the count), its check passes only while the table is intact, every record keeps its stamp and every fact still holds, and the capture refuses records whose header digest the receipt does not name. The scenarios live in `./🔣️.json` as language-agnostic data. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {brotliCompressSync,brotliDecompressSync} from "node:zlib";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import {captureSealFactsV1,retireSealFactsV1,sealFactsFileNameV1,sealFactsHoldV1} from "../🟦️.ts";
import {censusInferenceRecordsV1} from "../../🧮️census/🟨️.mjs";

const python=Bun.which("python3"),sha=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");
type Row={kind:string;file:string;fact?:boolean};
const factOf=(root:string,row:Row)=>{const path=join(root,row.file);if(row.kind==="exists")return {kind:"exists",path,fact:row.fact??false};const bytes=readFileSync(path,{throwIfNoEntry:false} as never);return {kind:"file",path,fact:{sha256:sha(bytes)}};};
const holdRows=async(facts:unknown[])=>facts.every((row:any)=>row.kind==="exists"?(statSync(row.path,{throwIfNoEntry:false})!==undefined)===row.fact:(()=>{try{return sha(readFileSync(row.path))===row.fact.sha256;}catch{return false;}})());
const build=()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"seal-facts-")),data=join(root,"data");mkdirSync(join(data,"inference"),{recursive:true});
 for(const[name,text]of Object.entries(fixture.files))writeFileSync(join(root,name),text);
 const paths=new Map<string,string>(),digests:[string,string][]=[];
 for(const record of fixture.records){const header=JSON.stringify({digest:"0".repeat(64),observations:record.rows.map(row=>factOf(root,row))}),path=join(data,"inference",record.name+".json");writeFileSync(path,brotliCompressSync(Buffer.from(header+"\n{}")));paths.set(record.name,path);digests.push([path,sha(header)]);}
 const receipts=["one.receipt.json","two.receipt.json"],observations=fixture.receiptRows.map(row=>factOf(root,row));
 receipts.forEach((name,index)=>writeFileSync(join(data,"inference",name),JSON.stringify({format:1,pid:1,completedAt:0,inference:index===0?digests.slice(0,2):digests.slice(1),observations})));
 return {root,data,paths,digests,receipts};
};
const body=(value:{seal:unknown;bytes:Buffer})=>({seal:value.seal as any,rows:JSON.parse(brotliDecompressSync(value.bytes).toString("utf8")) as unknown[]});

test("seal fact fixture is closed",()=>{expect(new Ajv({strict:false}).validate(schema,fixture)).toBe(true);});

test("the fact table holds each distinct fact once and agrees with a serial set and with Python",async()=>{
 const{root,data,receipts,paths}=build();
 try{
  const captured=await captureSealFactsV1(data,receipts);expect(captured).not.toBeNull();
  const{seal,rows}=body(captured!);
  const serial=new Set<string>();for(const path of paths.values())for(const row of JSON.parse(brotliDecompressSync(readFileSync(path)).toString("utf8").split("\n")[0]!).observations)serial.add(JSON.stringify(row));
  for(const name of receipts)for(const row of JSON.parse(readFileSync(join(data,"inference",name),"utf8")).observations)serial.add(JSON.stringify(row));
  expect(rows.length).toBe(fixture.expectedDistinct);expect(serial.size).toBe(fixture.expectedDistinct);
  expect(new Set(rows.map(row=>JSON.stringify(row)))).toEqual(serial);
  expect(seal.facts.count).toBe(fixture.expectedDistinct);expect(seal.facts.sha256).toBe(sha(captured!.bytes));
  expect(seal.records.map(([path]:[string])=>path)).toEqual([...paths.values()].sort());
  const inline=censusInferenceRecordsV1([...paths.values()].sort());expect(inline.records.map(([path,digest]:[string,string])=>[path,digest])).toEqual(seal.records.map(([path]:[string])=>[path,sha(brotliDecompressSync(readFileSync(path)).toString("utf8").split("\n")[0]!)]));
  if(python){const headers=[...paths.values()].map(path=>brotliDecompressSync(readFileSync(path)).toString("utf8").split("\n")[0]),recordsOnly=new Set<string>();for(const header of headers)for(const row of JSON.parse(header!).observations)recordsOnly.add(JSON.stringify(row));const result=Bun.spawnSync([python,"-I","-c","import json,sys\nrows=set()\nfor header in json.loads(sys.stdin.read()):\n    for row in json.loads(header)['observations']:\n        rows.add(json.dumps(row,ensure_ascii=False,separators=(',',':')))\nprint(len(rows))"],{stdin:Buffer.from(JSON.stringify(headers))});expect(Number(result.stdout.toString().trim())).toBe(recordsOnly.size);}
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("each scenario holds exactly while the table, the records and the observed facts are unchanged",async()=>{
 for(const scenario of fixture.scenarios){
  const{root,data,receipts,paths}=build();
  try{
   const captured=(await captureSealFactsV1(data,receipts))!,{seal}=body(captured),file=join(data,sealFactsFileNameV1(seal.facts.sha256));writeFileSync(file,captured.bytes);
   expect(await sealFactsHoldV1(data,seal,holdRows),scenario.id+" before").toBe(true);
   const mutation=scenario.mutation as {kind:string;file?:string;content?:string;record?:string};
   if(mutation.kind==="write-file"){mkdirSync(dirname(join(root,mutation.file!)),{recursive:true});writeFileSync(join(root,mutation.file!),mutation.content!);}
   if(mutation.kind==="replace-record"){const path=paths.get(mutation.record!)!,bytes=readFileSync(path);rmSync(path);renameSync((()=>{const temporary=path+".new";writeFileSync(temporary,bytes);return temporary;})(),path);}
   if(mutation.kind==="flip-facts"){const bytes=Buffer.from(captured.bytes);bytes[bytes.length>>1]=bytes[bytes.length>>1]!^255;writeFileSync(file,bytes);}
   if(mutation.kind==="remove-facts")rmSync(file);
   expect(await sealFactsHoldV1(data,seal,holdRows),scenario.id).toBe(scenario.holds);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
});

test("the capture refuses records the receipts do not describe",async()=>{
 for(const refusal of fixture.refusals){
  const{root,data,receipts,paths}=build();
  try{
   const row=refusal as {digestOf?:string;recordCorrupt?:string;removeRecord?:string};
   if(row.recordCorrupt)writeFileSync(paths.get(row.recordCorrupt)!,brotliCompressSync(Buffer.from(JSON.stringify({digest:"0",observations:[]})+"\n{}")));
   if(row.removeRecord)rmSync(paths.get(row.removeRecord)!);
   expect(await captureSealFactsV1(data,receipts),refusal.id).toBeNull();
  }finally{rmSync(root,{recursive:true,force:true});}
 }
});

test("a cancelled capture rejects with the caller's reason and retirement keeps only the named tables",async()=>{
 const{root,data,receipts}=build();
 try{
  const controller=new AbortController(),reason=new Error("stop");controller.abort(reason);
  await expect(captureSealFactsV1(data,receipts,controller.signal)).rejects.toBe(reason);
  for(const text of ["a","b","c"])writeFileSync(join(data,sealFactsFileNameV1(sha(text))),text);
  await retireSealFactsV1(data,new Set([sealFactsFileNameV1(sha("b"))]));
  expect(readdirSync(data).filter(name=>name.startsWith("semio-graph-facts."))).toEqual([sealFactsFileNameV1(sha("b"))]);
 }finally{rmSync(root,{recursive:true,force:true});}
});
