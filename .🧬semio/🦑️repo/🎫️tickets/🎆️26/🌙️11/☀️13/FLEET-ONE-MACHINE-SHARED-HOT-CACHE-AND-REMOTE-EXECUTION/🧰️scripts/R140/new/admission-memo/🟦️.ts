/** 🧠️ Contract: the closed owner admission of a selected read is remembered on the machine, keyed by the scope and the loaded producer, and served again only while every fact it was derived from still holds; a corrupt, foreign, stale or oversized memo is a miss, never an error. */
import{expect,test}from"bun:test";
import{mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{readAdmissionMemoV1,writeAdmissionMemoV1,type AdmissionMemoV1}from"../🟦️.ts";

const module={implementation:"a".repeat(64),publication:"b".repeat(64)};
const record=(scope="s".repeat(8)):AdmissionMemoV1=>({version:1,module,scope,admission:{owners:["o"],files:["o/📋️project.json"],targets:{},inputs:{},catalogProviders:[]},observations:[{kind:"file",path:"/w/o/📋️project.json",fact:{sha256:"c".repeat(64)}}]});
const accept=(value:AdmissionMemoV1)=>value.module.implementation===module.implementation&&value.module.publication===module.publication;
const directory=()=>mkdtempSync(join(tmpdir(),"admission-memo-"));

test("a written memo is served while its facts hold and missed when they do not",async()=>{
 const root=directory();try{
  const path=join(root,"s.admission.json");await writeAdmissionMemoV1(path,record(),1<<20);
  const seen:unknown[][]=[];
  const hit=await readAdmissionMemoV1(path,"s".repeat(8),accept,async rows=>{seen.push([...rows]);return true;});
  expect(hit?.admission).toEqual(record().admission);expect(hit?.observations).toEqual(record().observations);expect(seen.length).toBe(1);
  expect(await readAdmissionMemoV1(path,"s".repeat(8),accept,async()=>false)).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a memo of another scope, another producer, a corrupt file or a missing file is a miss and never reaches the facts",async()=>{
 const root=directory();try{
  const path=join(root,"s.admission.json");await writeAdmissionMemoV1(path,record(),1<<20);
  let asked=0;const holds=async()=>{asked++;return true;};
  expect(await readAdmissionMemoV1(path,"t".repeat(8),accept,holds)).toBeUndefined();
  expect(await readAdmissionMemoV1(path,"s".repeat(8),()=>false,holds)).toBeUndefined();
  writeFileSync(path,readFileSync(path,"utf8").slice(0,40));
  expect(await readAdmissionMemoV1(path,"s".repeat(8),accept,holds)).toBeUndefined();
  writeFileSync(path,JSON.stringify({...record(),admission:{owners:"not a list"}}));
  expect(await readAdmissionMemoV1(path,"s".repeat(8),accept,holds)).toBeUndefined();
  expect(await readAdmissionMemoV1(join(root,"absent.json"),"s".repeat(8),accept,holds)).toBeUndefined();
  expect(asked).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a memo above the limit is not written and a write leaves no temporary file behind",async()=>{
 const root=directory();try{
  const path=join(root,"s.admission.json");
  expect(await writeAdmissionMemoV1(path,record(),50)).toBe(false);expect(readdirSync(root)).toEqual([]);
  expect(await writeAdmissionMemoV1(path,record(),1<<20)).toBe(true);expect(readdirSync(root)).toEqual(["s.admission.json"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the selected inventory checks the registry scope, then asks the memo, derives only on a miss and remembers what it derived",()=>{
 const source=readFileSync(process.env.R140_LOAD_SOURCE??join(import.meta.dir,"..","..","🟦️.ts"),"utf8"),inventory=source.indexOf("const inventory=async()=>");
 const scope=source.indexOf("buildGraphScopeKeyV1(options.selection,readBuildOwnerRegistryV1(",inventory),read=source.indexOf("readAdmissionMemoV1(memoPath,scope",inventory),derive=source.indexOf("captureObservedSourceInputsV1(",read),write=source.indexOf("writeAdmissionMemoV1(memoPath,",derive);
 expect(inventory).toBeGreaterThan(0);expect(scope).toBeGreaterThan(inventory);expect(read).toBeGreaterThan(scope);expect(derive).toBeGreaterThan(read);expect(write).toBeGreaterThan(derive);
 expect(source).toContain("remembered===undefined&&selected&&scope&&captured!==undefined");
});
