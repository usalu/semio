/** 💰️ Tests for build cache-gc budget. */
import {afterAll,beforeAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {spawn,spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {CACHE_SIZE_FILE_V1,readCacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {readMachinePolicyV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts";
import {cacheAtimeReliableV1,cacheRunnerPortsV1,collectCacheV1,measureCacheTreeV1} from "../../🏃️runner/🟦️.ts";
import {cacheArgumentsV1,runCacheLedgerCommandV1} from "../../⌨️command/🟦️.ts";

const DAY=86400000,HOUR=3600000,UNIT=20480,reason={en:"x",de:"y"},purpose={en:"p",de:"q"};
const partitions={alpha:{directory:".cache/b/a",capacityBytes:1<<30,maximumLogicalBytes:1<<30}};
const ledger=readCacheLedgerV1({
 version:1,
 roots:{repository:{directory:"r/cache",maximumBytes:1<<30,undeclaredUnusedAgeMs:14*DAY,undeclaredQuietMs:HOUR,ignored:[CACHE_SIZE_FILE_V1]},native:{directory:".cache",maximumBytes:1<<20,undeclaredUnusedAgeMs:7*DAY,undeclaredQuietMs:HOUR,ignored:[]}},
 areas:[
  {id:"alpha",root:"native",kind:"partition",directory:"b/a",partition:"alpha",managed:{owner:"image"},writers:["w"],purpose},
  {id:"loose",root:"repository",kind:"tree",directory:"loose",budget:{bytes:1<<20},collect:{unusedAgeMs:100*DAY,quietMs:HOUR,custody:"quiet"},writers:["w"],purpose},
  {id:"other",root:"repository",kind:"tree",directory:"other",budget:{bytes:1<<20},collect:{unusedAgeMs:100*DAY,quietMs:HOUR,custody:"quiet"},writers:["w"],purpose}
 ],
 retirements:[{root:"repository",directory:"old",quietMs:HOUR,reason}]
},partitions);
const policy=readMachinePolicyV1({
 version:1,defaultTotalBytes:1<<20,environment:"TEST_BUDGET",file:"m.json",hostFreeFloorBytes:0,alignBytes:1048576,toleranceBytes:0,dedupe:[],
 stores:[{id:"alpha",partition:"alpha",weightBasisPoints:1000,floorBytes:0,headroomBasisPoints:0,purpose},{id:"loose",loose:true,weightBasisPoints:9000,floorBytes:0,headroomBasisPoints:0,purpose}]
},partitions);

let workspace="";const now=Date.now();
function unit(path:string,ageMs:number,size=UNIT):string{const full=join(workspace,path);mkdirSync(join(full,".."),{recursive:true});writeFileSync(full,"x".repeat(size));const time=(now-ageMs)/1000;utimesSync(full,time,time);return full;}
const ports=(atimeReliable:boolean=true)=>cacheRunnerPortsV1({signal:new AbortController().signal,now:()=>now,progress:async()=>{},exclusive:async()=>({release:()=>{}}),atimeReliable:async()=>atimeReliable});
const quiet=()=>({write:()=>{},language:"en" as const});
beforeAll(()=>{workspace=mkdtempSync(join(tmpdir(),"mgc-"));});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

function oracle(units:Array<{name:string;bytes:number;recency:number;held:boolean}>,budget:number,quietMs:number,nowMs:number):string[]{
 const code=`
import json,sys
spec=json.load(sys.stdin)
units=sorted(spec["units"],key=lambda u:(u["recency"],u["name"]))
total=sum(u["bytes"] for u in units)
out=[]
for u in units:
    if total<=spec["budget"]: break
    if u["held"] or spec["now"]-u["recency"]<spec["quiet"]: continue
    out.append(u["name"]); total-=u["bytes"]
print(json.dumps(sorted(out)))`;
 const run=spawnSync("python3",["-I","-c",code],{input:JSON.stringify({units,budget,quiet:quietMs,now:nowMs}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 return JSON.parse(run.stdout) as string[];
}

test("the machine budget evicts least recently used loose units down to the quota, never a unit a live process holds, and converges",async()=>{
 const names=["u0","u1","u2","u3","u4"],ages=[9,8,7,6,5];
 names.forEach((name,index)=>unit("r/cache/loose/"+name+"/blob.bin",ages[index]!*DAY));
 const held=join(workspace,"r/cache/loose/u1/lease.sqlite"),script='const {Database}=require("bun:sqlite");const d=new Database(process.argv[1]);d.exec("PRAGMA journal_mode=DELETE;CREATE TABLE t(x);BEGIN EXCLUSIVE");process.stdout.write("ready\\n");process.stdin.resume();process.stdin.on("end",()=>{d.exec("ROLLBACK");d.close();process.exit(0)});';
 const holder=spawn(process.execPath,["-e",script,held],{stdio:["pipe","pipe","inherit"]});
 try{
  await new Promise<void>((resolveReady,reject)=>{holder.stdout.once("data",()=>resolveReady());holder.once("error",reject);holder.once("exit",code=>reject(Error("holder exited "+code)));});
  const old=(Date.now()-9*DAY)/1000;utimesSync(held,old,old);
  const measured=[];for(const name of names){const measure=await measureCacheTreeV1(join(workspace,"r/cache/loose",name),ports(),1000);measured.push({name,bytes:measure.bytes,recency:measure.recencyMs,held:name==="u1"});}
  const total=measured.reduce((sum,row)=>sum+row.bytes,0),quota=Math.floor(total*0.45),expected=oracle(measured,quota,HOUR,now);
  expect(expected).not.toContain("u1");expect(expected.length).toBeGreaterThan(0);
  const dry=await collectCacheV1(workspace,ledger,{dryRun:true,only:new Set(["loose"]),machine:{totalBytes:quota,source:"argument",policy}},ports(),quiet(),partitions);
  expect(dry.removed.map(row=>row.name).sort()).toEqual(expected);expect(dry.machine).not.toBeNull();
  expect(existsSync(join(workspace,"r/cache/loose/u0"))).toBe(true);
  const real=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["loose"]),machine:{totalBytes:quota,source:"argument",policy}},ports(),quiet(),partitions);
  expect(real.failed).toEqual([]);expect(real.removed.map(row=>row.name).sort()).toEqual(expected);
  expect(readdirSync(join(workspace,"r/cache/loose")).sort()).toEqual(names.filter(name=>!expected.includes(name)));
  expect(existsSync(held)).toBe(true);
  const again=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["loose"]),machine:{totalBytes:quota,source:"argument",policy}},ports(),quiet(),partitions);
  expect(again.removed).toEqual([]);
  const after=await measureCacheTreeV1(join(workspace,"r/cache/loose"),ports(),1000);
  expect(after.bytes).toBeLessThanOrEqual(quota+UNIT*(1+Number(!expected.includes("u1"))));
 }finally{holder.stdin.end();await new Promise(resolveExit=>{holder.once("exit",resolveExit);setTimeout(resolveExit,3000);});}
});

test("a budget that already holds everything removes nothing, and a missing machine budget keeps the declared area budgets only",async()=>{
 for(const name of ["a","b","c"])unit("r/cache/other/"+name+"/blob.bin",3*DAY);
 const plain=await collectCacheV1(workspace,ledger,{dryRun:true,only:new Set(["other"])},ports(),quiet(),partitions);
 expect(plain.removed).toEqual([]);expect(plain.machine).toBeNull();
 const roomy=await collectCacheV1(workspace,ledger,{dryRun:true,only:new Set(["other"]),machine:{totalBytes:1<<30,source:"default",policy}},ports(),quiet(),partitions);
 expect(roomy.removed).toEqual([]);expect(roomy.machine!.converged).toBe(true);
 const tight=await collectCacheV1(workspace,ledger,{dryRun:true,only:new Set(["other"]),machine:{totalBytes:UNIT,source:"default",policy}},ports(),quiet(),partitions);
 expect(tight.removed.length).toBe(2);
});

test("status and gc print the machine budget in both languages, take it from --budget, the environment or the machine file, and refuse nonsense in both languages",async()=>{
 expect(cacheArgumentsV1(["status","--budget=40GiB"],ledger).budget).toBe("40GiB");
 expect(cacheArgumentsV1(["gc","--dry-run"],ledger).budget).toBeUndefined();
 expect(()=>cacheArgumentsV1(["verify","--deep","--budget=1G"],ledger)).toThrow();
 expect(()=>cacheArgumentsV1(["status","--budget"],ledger)).toThrow();
 const run=async(args:string[],environment:Record<string,string|undefined>={})=>{const out:string[]=[];const code=await runCacheLedgerCommandV1(args,{workspace,environment,write:line=>out.push(line),ledger,partitions},ports());return{code,out};};
 const english=await run(["status","--lang=en","--budget=12GiB"]),german=await run(["status","--lang=de","--budget=12GiB"]);
 expect(english.code).toBe(0);
 expect(english.out.join("\n")).toContain("Machine budget: 12.00 GiB (from --budget)");expect(german.out.join("\n")).toContain("Maschinen-Budget: 12.00 GiB (aus --budget)");
 expect((await run(["status","--lang=en"],{SEMIO_CACHE_BUDGET_BYTES:"25GiB"})).out.join("\n")).toContain("25.00 GiB (from the environment)");
 mkdirSync(join(workspace,"r/cache"),{recursive:true});writeFileSync(join(workspace,"r/cache/🎚️machine-budget.json"),JSON.stringify({version:1,totalBytes:"18GiB"}));
 expect((await run(["status","--lang=en"])).out.join("\n")).toContain("18.00 GiB (from the machine file)");
 const json=JSON.parse((await run(["status","--json","--budget=9GiB"])).out.join("\n"));
 expect(json.machine.totalBytes).toBe(9*1024**3);expect(json.machine.source).toBe("argument");expect(json.machine.stores.length).toBe(9);
 await expect(run(["status","--budget=lots"])).rejects.toThrow("Keine Bytezahl");
 await expect(run(["status","--budget=1MiB"])).rejects.toThrow("below the floors");
 const gc=await run(["gc","--dry-run","--lang=de","--budget=12GiB"]);
 expect(gc.out.join("\n")).toContain("Maschinen-Budget: 12.00 GiB");expect(gc.out.join("\n")).toContain("Lose Caches dürfen");
});

test("planning against the machine budget reports progress and stops at once when cancelled, leaving every unit",async()=>{
 for(const name of ["p","q"])unit("r/cache/loose2/"+name+"/blob.bin",5*DAY);
 const controller=new AbortController(),stages:string[]=[],active=cacheRunnerPortsV1({signal:controller.signal,now:()=>now,progress:async stage=>{stages.push(stage);if(stage.startsWith("status:"))controller.abort(Error("stop"));},exclusive:async()=>({release:()=>{}})});
 await expect(collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["other"]),machine:{totalBytes:1,source:"argument",policy}},active,quiet(),partitions)).rejects.toThrow("stop");
 expect(stages.some(stage=>stage.startsWith("status:"))).toBe(true);
 expect(readdirSync(join(workspace,"r/cache/loose2")).sort()).toEqual(["p","q"]);
});

test("a long list of removals collapses to a bounded number of lines per target and a count, while the result keeps every removal",async()=>{
 for(let index=0;index<40;index++)unit("r/cache/other/many"+String(index).padStart(2,"0")+"/blob.bin",(30+index)*DAY);
 const out:string[]=[],result=await collectCacheV1(workspace,ledger,{dryRun:true,only:new Set(["other"]),machine:{totalBytes:UNIT,source:"default",policy}},ports(),{write:line=>out.push(line),language:"en"},partitions);
 expect(result.removed.length).toBeGreaterThan(40);
 expect(out.filter(line=>line.startsWith("Would remove other/")).length).toBe(25);
 expect(out.some(line=>/^\.\.\. and \d+ more entries of other/u.test(line))).toBe(true);
});

test("Law: quota eviction of a quiet-custody cache needs a liveness signal it can trust: a volume whose access times do not advance is never tightened to its quota share, and where they do a unit used within the quota quiet time stays however far over the quota the machine is",async()=>{
 rmSync(join(workspace,"r/cache/other"),{recursive:true,force:true});
 for(const name of ["l0","l1","l2"])unit("r/cache/other/"+name+"/blob.bin",(10+Number(name.slice(1)))*DAY);
 unit("r/cache/other/young/blob.bin",DAY);unit("r/cache/other/recent/blob.bin",2*DAY);
 const request={dryRun:true,only:new Set(["other"]),machine:{totalBytes:UNIT,source:"default" as const,policy}};
 const blind=await collectCacheV1(workspace,ledger,request,ports(false),quiet(),partitions);
 expect(blind.removed,"declared budget 1 MiB holds all units, and the volume cannot tell used from unused: nothing is evicted for the quota").toEqual([]);
 const german:string[]=[],english:string[]=[];
 await collectCacheV1(workspace,ledger,request,ports(false),{write:line=>german.push(line),language:"de"},partitions);
 await collectCacheV1(workspace,ledger,request,ports(false),{write:line=>english.push(line),language:"en"},partitions);
 expect(english.some(line=>line.startsWith("Not tightening other to its quota share"))).toBe(true);expect(german.some(line=>line.startsWith("other wird nicht auf seinen Quota-Anteil begrenzt"))).toBe(true);
 const sighted=await collectCacheV1(workspace,ledger,request,ports(true),quiet(),partitions);
 const names=sighted.removed.map(row=>row.name).sort();
 expect(names,"the three units unused for 10 days or more go, the two used within the quota quiet time stay").toEqual(["l0","l1","l2"]);
 expect(names).not.toContain("young");expect(names).not.toContain("recent");
 const independent=spawnSync("python3",["-I","-c","import json,sys;rows=json.load(sys.stdin);print(json.dumps(sorted(r['name'] for r in rows if r['age']>=3*86400)))"],{input:JSON.stringify([["l0",10],["l1",11],["l2",12],["young",1],["recent",2]].map(([name,days])=>({name,age:Number(days)*86400}))),encoding:"utf8"});
 expect(JSON.parse(independent.stdout),"the independent Python model keeps everything younger than three days").toEqual(names);
});

test("the access-time probe equals an independent observation: a file dated two days back and read once either moves its access time or it does not, and a directory that cannot be written is never trusted",async()=>{
 const directory=mkdtempSync(join(workspace,"atime-"));
 const observed=spawnSync("python3",["-I","-c","import os,sys,time;p=os.path.join(sys.argv[1],'.o');open(p,'w').write('x');past=time.time()-172800;os.utime(p,(past,past));open(p).read();print(os.stat(p).st_atime>past+1);os.unlink(p)",directory],{encoding:"utf8"});
 expect(observed.status,observed.stderr).toBe(0);
 expect(await cacheAtimeReliableV1(directory)).toBe(observed.stdout.trim()==="True");
 expect(readdirSync(directory),"the probe leaves nothing behind").toEqual([]);
 expect(await cacheAtimeReliableV1(join(directory,"missing"))).toBe(false);
});
