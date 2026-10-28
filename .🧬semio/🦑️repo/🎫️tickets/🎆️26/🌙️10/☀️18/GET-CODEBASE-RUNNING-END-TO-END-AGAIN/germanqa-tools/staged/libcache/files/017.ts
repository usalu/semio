import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {reclaimUnitStorageV1} from "../../🟦️.ts";
import {collectIncrementalV1,incrementalLockPathV1,incrementalRootsV1,measureIncrementalV1,planIncrementalV1,tryLockSessionV1,type IncrementalInputV1,type IncrementalLockPortV1} from "../🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),schemaPath=join(here,"..","🧬️schema","🔣️.json");
const oracle=(states:readonly IncrementalInputV1[]):unknown=>{const run=spawnSync("python3",["-I",join(here,"🐍️.py"),schemaPath],{input:JSON.stringify(states),encoding:"utf8",maxBuffer:1<<28});if(run.status!==0)throw Error("Python incremental oracle failed: "+run.stderr);return JSON.parse(run.stdout);};

test("every vector validates against the schema and matches the planner and the independent Python/SQLite oracle",()=>{
 const states=fixture.vectors.map(row=>row.input as IncrementalInputV1),expected=fixture.vectors.map(row=>row.remove);
 for(const row of fixture.vectors)expect(validateJsonSchemaSubset({definitions:schema.definitions,$ref:"#/definitions/Vector"},row),row.name).toEqual([]);
 fixture.vectors.forEach((row,index)=>expect<unknown>(planIncrementalV1(row.input as IncrementalInputV1),row.name).toEqual(row.remove));
 expect<unknown>(oracle(states)).toEqual(expected);
});

test("400 seeded random states give the same plan from the planner and the independent oracle",()=>{
 let seed=20261023;const next=(limit:number)=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed%limit;},now=1_000_000_000,states:IncrementalInputV1[]=[];
 for(let round=0;round<400;round++){
  const count=next(12),sessions=Array.from({length:count},(_,index)=>({path:`/m/c${next(5)}/s-${index}-${next(9)}`,bytes:next(5000),lastUsedMs:now-next(9_000_000),held:next(5)===0,newest:next(3)===0}));
  states.push({volumeBytes:1+next(60_000),needBytes:next(12_000),nowMs:now,sessions});
 }
 const answers=oracle(states) as unknown[];let removals=0;
 states.forEach((state,index)=>{const plan=planIncrementalV1(state);removals+=plan.length;expect<unknown>(plan).toEqual(answers[index]);expect(plan.every(row=>!state.sessions.find(item=>item.path===row.path)!.held)).toBe(true);});
 expect(removals).toBeGreaterThan(300);
});

function session(root:string,crate:string,name:string,ageMs:number,bytes:number,lock=true):string{
 const directory=join(root,crate,name);mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"dep-graph.bin"),Buffer.alloc(bytes,1));
 const time=new Date(Date.now()-ageMs);utimesSync(directory,time,time);
 if(lock){const path=incrementalLockPathV1(directory);writeFileSync(path,"");utimesSync(path,time,time);}
 return directory;
}

test("discovery finds the incremental directories of the declared bases and nothing else",()=>{
 const store=mkdtempSync(join(tmpdir(),"sinc-"));
 try{
  for(const path of ["b/p/prototype/incremental","b/p/wasm32-unknown-unknown/prototype/incremental","b/r/debug/incremental","t/p/release/incremental","u/l/d1/b/prototype/incremental","components/f1/b/prototype/incremental","tmp/x/incremental","cargo/home/registry/incremental"])mkdirSync(join(store,path),{recursive:true});
  expect(incrementalRootsV1(store).map(path=>path.slice(store.length+1))).toEqual(["b/p/prototype/incremental","b/p/wasm32-unknown-unknown/prototype/incremental","b/r/debug/incremental","components/f1/b/prototype/incremental","t/p/release/incremental","u/l/d1/b/prototype/incremental"]);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test.skipIf(!["darwin","linux"].includes(process.platform))("a session whose lock a running compiler holds is never measured as free nor removed; the rest is collected with progress and a German report",async()=>{
 const store=mkdtempSync(join(tmpdir(),"sinc-")),root=join(store,"b","p","prototype","incremental"),hour=3_600_000;
 try{
  const old=session(root,"a-1","s-1-aaa-hash1",3*hour,40_000),live=session(root,"b-1","s-2-bbb-hash2",3*hour,40_000),fresh=session(root,"c-1","s-3-ccc-hash3",1000,40_000);
  const holder=Bun.spawn(["python3","-I","-c","import fcntl,sys,time;f=open(sys.argv[1],'r+');fcntl.flock(f,fcntl.LOCK_EX);print('locked',flush=True);time.sleep(60)",incrementalLockPathV1(live)],{stdout:"pipe"});
  try{
   const reader=holder.stdout.getReader();expect(new TextDecoder().decode((await reader.read()).value)).toContain("locked");
   const measured=await measureIncrementalV1([root],{nowMs:Date.now()});
   expect(measured.find(row=>row.path===live)?.held).toBe(true);expect(measured.find(row=>row.path===old)?.held).toBe(false);
   const progress:number[]=[],lines:string[]=[];
   const dry=await collectIncrementalV1(store,{dryRun:true,volumeBytes:10_000_000,needBytes:()=>0,report:{language:"de",write:line=>lines.push(line)}});
   expect(dry.removed.map(row=>row.path)).toEqual([]);
   const result=await collectIncrementalV1(store,{dryRun:false,volumeBytes:10_000,needBytes:()=>1,satisfied:()=>false,progress:done=>progress.push(done),report:{language:"de",write:line=>lines.push(line)}});
   expect(existsSync(live)).toBe(true);expect(existsSync(fresh)).toBe(true);expect(existsSync(old)).toBe(false);expect(existsSync(incrementalLockPathV1(old))).toBe(false);expect(existsSync(dirname(old))).toBe(false);
   expect(result.held).toBe(1);expect(result.removed.map(row=>row.path)).toEqual([old]);expect(progress.length).toBeGreaterThan(0);
   expect(lines.some(line=>line.includes("Inkrementsitzungen:"))).toBe(true);
  }finally{holder.kill();}
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("cancellation stops a collection between sessions",async()=>{
 const store=mkdtempSync(join(tmpdir(),"sinc-")),root=join(store,"b","p","prototype","incremental"),hour=3_600_000,controller=new AbortController();
 try{
  for(let index=0;index<6;index++)session(root,`k-${index}`,`s-${index}-x-h`,(10+index)*hour,100);
  const never:IncrementalLockPortV1=()=>"missing";
  await expect(collectIncrementalV1(store,{dryRun:false,volumeBytes:10_000_000,needBytes:()=>0,lock:never,signal:controller.signal,progress:()=>controller.abort()})).rejects.toBeDefined();
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("the compiler lock probe reports a missing lock file",()=>{expect(tryLockSessionV1("/nonexistent/s-1-a.lock")).toBe(process.platform==="win32"?"unsupported":"missing");});

test("reclaiming before a reservation refuses removes superseded sessions, keeps the newest and the recent, and a running collector makes the second caller return at once",async()=>{
 const store=mkdtempSync(join(tmpdir(),"sinc-")),root=join(store,"b","p","prototype","incremental"),hour=3_600_000;
 try{
  const superseded=session(root,"a-1","s-1-aaa-h1",3*hour,40_000);session(root,"a-1","s-2-bbb-h2",2*hour,40_000);const other=session(root,"b-1","s-3-ccc-h3",1000,40_000);
  mkdirSync(join(store,"u"),{recursive:true});
  const lines:string[]=[];
  await reclaimUnitStorageV1(store,1,{report:{language:"de",write:line=>lines.push(line)}});
  expect(existsSync(superseded)).toBe(false);expect(existsSync(other)).toBe(true);
  expect(lines.some(line=>line.includes("Inkrementsitzung"))).toBe(true);
  const { tryClaimV1 } = await import("../../../🔒️lock/🟦️.ts");
  const claim=tryClaimV1(join(store,"u","k"),"gc");
  try{expect(claim).toBeDefined();expect(await reclaimUnitStorageV1(store,1)).toBe(0);}finally{claim?.release();}
 }finally{rmSync(store,{recursive:true,force:true});}
});
