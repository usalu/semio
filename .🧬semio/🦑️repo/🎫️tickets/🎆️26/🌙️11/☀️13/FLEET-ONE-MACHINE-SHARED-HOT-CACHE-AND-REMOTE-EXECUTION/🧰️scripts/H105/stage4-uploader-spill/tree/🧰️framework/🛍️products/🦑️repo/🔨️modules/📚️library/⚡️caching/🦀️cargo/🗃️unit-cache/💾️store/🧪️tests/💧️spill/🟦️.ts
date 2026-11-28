/** 💧️ Tests for the spill of a unit whose objects the host store refuses. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {dropSpillObjectsV1,newestEntryV1,pruneEntriesV1,pruneSpillObjectsV1,publishUnitV1,putSpillObjectV1,readSpilledEntryV1,readSpillObjectV1,touchSpilledEntryV1} from "../../🟦️.ts";

test("a spilled unit keeps its objects and its entry out of the host store and its entry index (no collector or prune can take it for dead), reads back by hash, rejects tampering and is dropped after the upload",()=>{
 const root=mkdtempSync(join(tmpdir(),"spill-")),work=mkdtempSync(join(tmpdir(),"spill-work-"));
 try{
  const lib=join(work,"libx.rlib"),dep=join(work,"x.d");writeFileSync(lib,"compiled bytes");writeFileSync(join(work,"x.rs"),"fn main(){}");
  writeFileSync(dep,`${lib}: ${join(work,"x.rs")}\n`);
  const entry=publishUnitV1(root,{key:"ab"+"0".repeat(62),crate:"x",cwd:work,outDir:work,laneRoot:undefined,outputs:[lib,dep],stdout:"",stderr:"warning: x",stampDirectory:undefined,spill:true});
  expect(entry).toBeDefined();
  const sha=createHash("sha256").update("compiled bytes").digest("hex");
  expect(entry!.outputs.find(row=>row.name==="libx.rlib")!.sha256).toBe(sha);
  expect(readSpillObjectV1(root,sha)!.toString()).toBe("compiled bytes");
  expect(readSpillObjectV1(root,"0".repeat(64))).toBeUndefined();
  expect(newestEntryV1(root,entry!.key),"not in the entry index").toBeUndefined();expect(existsSync(join(root,"e"))).toBe(false);
  pruneEntriesV1(root,Date.now()+7_200_000);
  expect(readSpilledEntryV1(root,entry!.key),"a prune keeps it").toEqual(entry);
  const spilled=join(root,"w","spill");writeFileSync(join(spilled,sha),"tampered");
  expect(readSpillObjectV1(root,sha)).toBeUndefined();
  putSpillObjectV1(root,"compiled bytes");expect(readSpillObjectV1(root,sha)).toBeDefined();
  dropSpillObjectsV1(root,entry!);
  expect(existsSync(join(spilled,sha))).toBe(false);expect(readdirSync(spilled).filter(name=>!name.endsWith(".tmp")&&name!=="entries").length).toBe(0);
  expect(readSpilledEntryV1(root,entry!.key)).toBeUndefined();expect(readdirSync(join(spilled,"entries"))).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(work,{recursive:true,force:true});}
});

test("a waiting upload that touches its spilled entry keeps it when the bound of the spill folder forces the least recently used entries out; an untouched older one goes",()=>{
 const root=mkdtempSync(join(tmpdir(),"spill-touch-"));
 try{
  const folder=join(root,"w","spill"),entries=join(folder,"entries");mkdirSync(entries,{recursive:true});
  const spill=(name:string,key:string)=>{const sha=putSpillObjectV1(root,"bytes of "+name),entry={version:1 as const,key,outputs:[{name:name+".rlib",sha256:sha}],stdout:null,stderr:null};writeFileSync(join(entries,key+".json"),JSON.stringify(entry));return entry;};
  const waiting=spill("waiting","ab"+"1".repeat(62)),forgotten=spill("forgotten","cd"+"2".repeat(62)),old=(Date.now()-3_600_000)/1000;
  for(const name of readdirSync(folder))if(name!=="entries")utimesSync(join(folder,name),old,old);
  for(const name of readdirSync(entries))utimesSync(join(entries,name),old,old);
  expect(touchSpilledEntryV1(root,waiting.key)).toBe(true);expect(touchSpilledEntryV1(root,"ef"+"3".repeat(62))).toBe(false);
  pruneSpillObjectsV1(root,{boundBytes:20});
  expect(readSpilledEntryV1(root,waiting.key)?.key,"the touched entry stays").toBe(waiting.key);
  expect(readSpilledEntryV1(root,forgotten.key),"the untouched entry goes").toBeUndefined();
  expect(existsSync(join(folder,waiting.outputs[0]!.sha256)),"its object stays").toBe(true);expect(existsSync(join(folder,forgotten.outputs[0]!.sha256))).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});
