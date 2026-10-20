import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {linkSync,mkdirSync,mkdtempSync,rmSync,symlinkSync,truncateSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {tmpdir} from "node:os";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../../🧬️schema/🔣️.json";
import {createNativeDirectoryMeterV1,nativeDirectoryProtocolV1} from "../../🟦️.ts";
import {createNativeDirectoryUsageV1,measureNativeDirectoryUsageAsyncV1,measureNativeDirectoryUsageV1,nativeDirectoryEntryBoundV1,type NativeDirectoryWalkPortV1} from "../🟦️.ts";

const protocol=nativeDirectoryProtocolV1(),oracle=new Ajv({strict:true});
const scratch=()=>{const base=process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir();mkdirSync(base,{recursive:true});return mkdtempSync(join(base,"use-"));};
const sh=(...args:string[])=>Bun.spawnSync(args,{stdout:"pipe",stderr:"pipe"});
const python=`import os,stat,sys,json
seen=set();total={"allocatedBytes":0,"logicalBytes":0,"entries":0}
def walk(path):
    row=os.lstat(path);total["entries"]+=1
    if stat.S_ISREG(row.st_mode) and row.st_nlink>1:
        key=(row.st_dev,row.st_ino)
        if key in seen: return
        seen.add(key)
    total["allocatedBytes"]+=row.st_blocks*512
    if stat.S_ISREG(row.st_mode): total["logicalBytes"]+=row.st_size
    if stat.S_ISDIR(row.st_mode):
        for name in os.listdir(path): walk(os.path.join(path,name))
walk(sys.argv[1]);print(json.dumps(total))`;
const hasPython=testPythonV1()!==null;

test("the entry bound follows the envelope and never drops below the floor, as an independent python formula says",()=>{
 const check=oracle.compile({type:"object",additionalProperties:false,required:["physicalBytes","entries"],properties:{physicalBytes:{type:"integer",minimum:1},entries:{type:"integer",minimum:1}}});
 for(const row of fixture.bounds){
  expect(check(row),JSON.stringify(check.errors)).toBe(true);expect(nativeDirectoryEntryBoundV1(row.physicalBytes),String(row.physicalBytes)).toBe(row.entries);
  if(hasPython){const answer=sh(testPythonV1()??"python3","-c",`import math,sys;b=int(sys.argv[1]);print(max(${protocol.maximumEntries},math.ceil(b/${protocol.usage.blockBytes})*${protocol.usage.entryFactor}))`,String(row.physicalBytes));expect(Number(answer.stdout.toString())).toBe(row.entries);}
 }
 expect(()=>nativeDirectoryEntryBoundV1(0)).toThrow("Positive physical envelope");
 expect(schema.definitions.Protocol.const.usage.concurrency).toBeGreaterThan(0);
});

test("the asynchronous walk equals the synchronous walk, an independent python walk and du on trees with hard links, links and holes",async()=>{
 const base=scratch();try{
  for(const row of fixture.trees){
   const root=join(base,row.id);mkdirSync(root);
   for(const name of row.directories)mkdirSync(join(root,name),{recursive:true});
   for(const[path,size]of row.files)writeFileSync(join(root,String(path)),Buffer.alloc(size as number,1));
   const generated=(row as {generated?:{directory:string;count:number;bytes:number}}).generated;
   if(generated)for(let index=0;index<generated.count;index++)writeFileSync(join(root,generated.directory,"f"+index),Buffer.alloc(generated.bytes,2));
   for(const[from,to]of row.hardLinks)linkSync(join(root,from!),join(root,to!));
   for(const[target,path]of row.symlinks)symlinkSync(target!.startsWith("/")?target!:join(root,target!),join(root,path!));
   for(const[path,size]of row.sparse){writeFileSync(join(root,String(path)),"");truncateSync(join(root,String(path)),Number(size));}
   const parallel=await measureNativeDirectoryUsageAsyncV1(root),serial=measureNativeDirectoryUsageV1(root);
   expect(parallel,row.id).toEqual(serial);
   if(process.platform!=="win32"){const du=Number(sh("du","-sk",root).stdout.toString().split(/\s/u)[0]);expect(Math.ceil(parallel.allocatedBytes/1024),row.id).toBe(du);}
   if(hasPython)expect(JSON.parse(sh(testPythonV1()??"python3","-c",python,root).stdout.toString()),row.id).toEqual({allocatedBytes:parallel.allocatedBytes,logicalBytes:parallel.logicalBytes,entries:parallel.entries});
  }
  const wide=await measureNativeDirectoryUsageAsyncV1(join(base,"wide"));expect(wide.entries).toBe(602);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the walk refuses at the entry bound, accepts exactly the bound and counts a vanished entry as freed space",async()=>{
 const base=scratch();try{
  const root=join(base,"t");mkdirSync(root);for(let index=0;index<30;index++)writeFileSync(join(root,"f"+index),"x");
  await expect(measureNativeDirectoryUsageAsyncV1(root,{maximumEntries:30})).rejects.toThrow("entry bound");
  expect((await measureNativeDirectoryUsageAsyncV1(root,{maximumEntries:31})).entries).toBe(31);
  const stats=new Map<string,Awaited<ReturnType<NativeDirectoryWalkPortV1["stat"]>>>(),port:NativeDirectoryWalkPortV1={
   list:async path=>path===root?["gone","kept"]:[],
   stat:async path=>{const row=stats.get(path);if(!row){const error=Object.assign(Error("gone"),{code:"ENOENT"});throw error;}return row;}
  };
  const file=(blocks:number)=>({isFile:()=>true,isDirectory:()=>false,nlink:1n,blocks:BigInt(blocks),size:BigInt(blocks*512),dev:1n,ino:BigInt(blocks)});
  stats.set(resolve(root),{...file(8),isFile:()=>false,isDirectory:()=>true});stats.set(join(root,"kept"),file(8));
  const row=await measureNativeDirectoryUsageAsyncV1(root,{port});expect(row).toEqual({allocatedBytes:8192,logicalBytes:4096,entries:2});
  await expect(measureNativeDirectoryUsageAsyncV1(join(base,"absent"))).rejects.toThrow("ENOENT");
  expect(measureNativeDirectoryUsageV1(root,31).entries).toBe(31);expect(()=>measureNativeDirectoryUsageV1(root,30)).toThrow("entry bound");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the walk stops on cancellation and does not block the event loop while it runs",async()=>{
 const base=scratch();try{
  const root=join(base,"big");mkdirSync(root);for(let directory=0;directory<20;directory++){mkdirSync(join(root,"d"+directory));for(let file=0;file<250;file++)writeFileSync(join(root,"d"+directory,"f"+file),"y");}
  let ticks=0,running=true;const spin=()=>{if(!running)return;ticks++;setImmediate(spin);};setImmediate(spin);
  const row=await measureNativeDirectoryUsageAsyncV1(root);running=false;
  expect(row.entries).toBe(1+20+5000);expect(ticks,"the loop must run between filesystem batches").toBeGreaterThan(20);
  const controller=new AbortController();let calls=0;
  const port:NativeDirectoryWalkPortV1={list:async()=>Array.from({length:400},(_,index)=>"n"+index),stat:async()=>{calls++;if(calls===20)controller.abort(new Error("stop now"));return {isFile:()=>false,isDirectory:()=>false,nlink:1n,blocks:0n,size:0n,dev:1n,ino:BigInt(calls)};}};
  const first={isFile:()=>false,isDirectory:()=>true,nlink:1n,blocks:0n,size:0n,dev:1n,ino:0n};
  const guarded:NativeDirectoryWalkPortV1={list:port.list,stat:async path=>path===resolve(root)?first:port.stat(path)};
  await expect(measureNativeDirectoryUsageAsyncV1(root,{signal:controller.signal,port:guarded})).rejects.toThrow("stop now");
  expect(calls,"no new filesystem calls are issued long after the signal").toBeLessThan(20+protocol.usage.concurrency*protocol.usage.batch+protocol.usage.batch);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("simultaneous measurements are coalesced: any number of callers cost at most two walks and no caller sees a walk that began before its own write",async()=>{
 const flightSchema=oracle.compile({type:"object",additionalProperties:false,required:["id","diskAtStart","script","starts","results"],properties:{id:{type:"string"},diskAtStart:{type:"integer"},script:{type:"array",items:{type:"string",pattern:"^(arrive:[A-Z]|abort:[A-Z]|write:[0-9]+|finish|fail)$"}},starts:{type:"integer"},aborts:{type:"integer"},results:{type:"object",additionalProperties:{anyOf:[{type:"integer"},{enum:["error","aborted"]}]}}}});
 for(const row of fixture.flights){
  expect(flightSchema(row),row.id+JSON.stringify(flightSchema.errors)).toBe(true);
  let disk=row.diskAtStart,starts=0,aborts=0;const live:{value:number;settle:(ok:boolean)=>void}[]=[],controllers=new Map<string,AbortController>(),results=new Map<string,Promise<number>>();
  const cell=createNativeDirectoryUsageV1({root:"/unused",maximumEntries:10,measure:(_,signal)=>new Promise<number>((done,fail)=>{starts++;const value=disk,entry={value,settle:(ok:boolean)=>ok?done(value):fail(new Error("walk failed"))};live.push(entry);signal.addEventListener("abort",()=>{aborts++;live.splice(live.indexOf(entry),1);fail(signal.reason);},{once:true});})});
  for(const step of row.script){
   const[kind,arg]=step.split(":");
   if(kind==="arrive"){const controller=new AbortController();controllers.set(arg!,controller);const call=cell.fresh(controller.signal);call.catch(()=>undefined);results.set(arg!,call);}
   else if(kind==="write")disk+=Number(arg);
   else if(kind==="abort")controllers.get(arg!)!.abort(new Error("caller left"));
   else{const entry=live.shift();expect(entry,row.id+" has a walk to settle").toBeDefined();entry!.settle(kind==="finish");}
   await new Promise(done=>setTimeout(done,2));
  }
  const outcome:Record<string,number|string>={};
  for(const[name,call]of results)outcome[name]=await call.then(value=>value,error=>(error as Error).message==="caller left"?"aborted":"error");
  expect(outcome,row.id).toEqual(row.results as Record<string,number|string>);expect(starts,row.id).toBe(row.starts);expect(aborts,row.id).toBe((row as {aborts?:number}).aborts??0);
 }
});

test("a recent measurement is served from the cell and a recorded number never moves backwards in time",async()=>{
 let clock=1000,walks=0;const cell=createNativeDirectoryUsageV1({root:"/unused",maximumEntries:10,now:()=>clock,measure:async()=>{walks++;return 42;}});
 expect(cell.peek()).toBeUndefined();expect(await cell.recent(5000)).toBe(42);expect(walks).toBe(1);
 clock+=4000;expect(await cell.recent(5000)).toBe(42);expect(walks).toBe(1);
 clock+=2000;expect(await cell.recent(5000)).toBe(42);expect(walks).toBe(2);
 cell.record(7,clock-50);expect(cell.peek()!.bytes).toBe(42);cell.record(7,clock+1);expect(cell.peek()!.bytes).toBe(7);
});

test("the capacity meter shares one walk between simultaneous commits and tolerates a cancelled waiter",async()=>{
 let walks=0,disk=0;const cell=createNativeDirectoryUsageV1({root:"/unused",maximumEntries:10,measure:async()=>{walks++;await new Promise(done=>setTimeout(done,20));return disk;}});
 const meter=createNativeDirectoryMeterV1({root:"/unused",filesystemBytes:33554432,maximumPhysicalBytes:34603008,usage:cell});
 disk=1048576;const commits=Array.from({length:6},()=>meter.admit({mode:"commit",requestBytes:0}));await Promise.all(commits);expect(walks).toBeLessThanOrEqual(2);
 disk=67108864;await expect(meter.admit({mode:"commit",requestBytes:0})).rejects.toThrow(protocol.refusals.capacity);
 const controller=new AbortController(),waiting=meter.admit({mode:"commit",requestBytes:0},controller.signal);controller.abort(new Error("caller left"));await expect(waiting).rejects.toThrow("caller left");
 disk=0;await meter.admit({mode:"commit",requestBytes:0});
});
