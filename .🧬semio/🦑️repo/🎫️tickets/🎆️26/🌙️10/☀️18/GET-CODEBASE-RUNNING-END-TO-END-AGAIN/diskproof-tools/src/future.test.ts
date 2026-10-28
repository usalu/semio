import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {CACHE_RUNTIME_V1,cacheRecencyMsV1,readCacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {cacheRunnerPortsV1,collectCacheV1,measureCacheTreeV1,type CacheRunnerPortsV1} from "../../🏃️runner/🟦️.ts";

const DAY=86400000,HOUR=3600000,reason={en:"x",de:"y"},purpose={en:"p",de:"q"},partitions={alpha:{directory:".cache/b/a",capacityBytes:1<<30,maximumLogicalBytes:1<<30}};
const ledger=readCacheLedgerV1({version:1,roots:{repository:{directory:"r/cache",maximumBytes:1<<30,undeclaredUnusedAgeMs:14*DAY,undeclaredQuietMs:HOUR,ignored:[]},native:{directory:".cache",maximumBytes:1<<20,undeclaredUnusedAgeMs:7*DAY,undeclaredQuietMs:HOUR,ignored:[]}},areas:[{id:"alpha",root:"native",kind:"partition",directory:"b/a",partition:"alpha",managed:{owner:"image"},writers:["w"],purpose}],retirements:[{root:"repository",directory:"old",quietMs:HOUR,reason}]},partitions);
const ports=(now:()=>number):CacheRunnerPortsV1=>cacheRunnerPortsV1({signal:new AbortController().signal,now,progress:async()=>undefined,exclusive:async()=>({release:()=>undefined})});
const python=(source:string,args:string[],input?:string):unknown=>{const run=Bun.spawnSync(["python3","-I","-c",source,...args],{stdin:input===undefined?undefined:new TextEncoder().encode(input)});if(run.exitCode!==0)throw Error(run.stderr.toString());return JSON.parse(run.stdout.toString());};
const parent=()=>resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());
const future=new Date("2036-10-11T11:49:59Z");

test("the last use ignores times dated beyond the skew and falls back to the inode change time, and an independent Python model agrees on 400 seeded states",()=>{
 let seed=20261028;const random=()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};
 const cases:number[][]=[];
 for(let at=0;at<400;at++){const now=1.7e12+Math.floor(random()*1e10),skew=random()<0.2?0:CACHE_RUNTIME_V1.futureSkewMs,time=()=>random()<0.35?now+Math.floor(random()*3.2e11):now-Math.floor(random()*1e10);cases.push([time(),time(),now-Math.floor(random()*1e10),now,skew]);}
 const expected=python("import json,sys\nout=[]\nfor m,a,c,now,skew in json.load(sys.stdin):\n limit=now+skew;past=[t for t in (m,a) if t<=limit];out.append(max(past) if past else min(c,now))\nprint(json.dumps(out))",[],JSON.stringify(cases)) as number[];
 expect(cases.map(([mtimeMs,atimeMs,ctimeMs,now,skew])=>cacheRecencyMsV1({mtimeMs,atimeMs,ctimeMs},now!,skew!))).toEqual(expected);
 expect(cacheRecencyMsV1({mtimeMs:5,atimeMs:9,ctimeMs:9},10)).toBe(9);
 expect(cacheRecencyMsV1({mtimeMs:future.getTime(),atimeMs:future.getTime(),ctimeMs:1000},5000)).toBe(1000);
 expect(CACHE_RUNTIME_V1.futureSkewMs).toBeGreaterThan(0);
});

test("a tree whose files are dated ten years ahead is measured by the real clock, equal to an independent walk of the same tree",async()=>{
 const base=mkdtempSync(join(parent(),"fut-"));try{
  mkdirSync(join(base,"install"),{recursive:true});writeFileSync(join(base,"install","server.jar"),"x".repeat(64));utimesSync(join(base,"install","server.jar"),future,future);
  writeFileSync(join(base,"old.bin"),"y");utimesSync(join(base,"old.bin"),new Date(Date.now()-5*DAY),new Date(Date.now()-5*DAY));
  const now=Date.now()+3*DAY,measure=await measureCacheTreeV1(base,ports(()=>now),1000),oracle=python("import json,os,sys\nnow,skew,best=float(sys.argv[2]),float(sys.argv[3]),0.0\nfor b,_,names in os.walk(sys.argv[1]):\n for n in names:\n  s=os.lstat(os.path.join(b,n));m,a,c=s.st_mtime*1000,s.st_atime*1000,s.st_ctime*1000;past=[t for t in (m,a) if t<=now+skew];best=max(best,max(past) if past else min(c,now))\nprint(json.dumps(best))",[base,String(now),String(CACHE_RUNTIME_V1.futureSkewMs)]) as number;
  expect(measure.recencyMs).toBeLessThan(now);expect(Math.abs(measure.recencyMs-oracle)).toBeLessThanOrEqual(1);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a retired root holding future-dated files is collected once its change time is older than the quiet time, and never while it is young",async()=>{
 const workspace=mkdtempSync(join(parent(),"fut-"));try{
  const seed=(name:string)=>{const path=join(workspace,"r/cache/old",name);mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,"x".repeat(100));utimesSync(path,future,future);return path;};
  const young=seed("young.jar"),out:string[]=[],write=(line:string)=>{out.push(line);};
  const first=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["retired"])},ports(()=>Date.now()),{write,language:"en"},partitions);
  expect(first.removed,"a file just written is in use whatever its mtime says").toEqual([]);expect(existsSync(young)).toBe(true);
  const later=await collectCacheV1(workspace,ledger,{dryRun:false,only:new Set(["retired"])},ports(()=>Date.now()+3*DAY),{write,language:"en"},partitions);
  expect(later.removed.map(row=>row.name)).toEqual(["young.jar"]);expect(existsSync(young)).toBe(false);
 }finally{rmSync(workspace,{recursive:true,force:true});}
});
