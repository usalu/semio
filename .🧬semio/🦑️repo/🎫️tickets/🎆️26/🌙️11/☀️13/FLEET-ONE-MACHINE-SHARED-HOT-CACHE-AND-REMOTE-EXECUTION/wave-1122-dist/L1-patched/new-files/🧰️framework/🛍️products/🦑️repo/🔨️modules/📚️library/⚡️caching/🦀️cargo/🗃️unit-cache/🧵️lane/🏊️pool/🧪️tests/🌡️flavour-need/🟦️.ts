/** 🌡️ Contract: the lanes a build flavour needs are never shed for the pool's soft bounds. The byte bound is raised to the need of the largest declared flavour, a lane the starting build requests is never a victim, recent lanes of a flavour go only under free-space pressure (then least recently used first), the use stamp survives a restore that resets file times, and an image that cannot hold the need says so before anything is removed. The rule is written twice, in production and in an independent Python oracle over one corpus, and once more over real lane directories.
 * @see ../../🟦️.ts */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import{requireTestToolV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{laneUsedMsV1,lanePathsV1,readLaneV1,recordLaneSizeV1}from"../../../🟦️.ts";
import{laneMessageV1}from"../../../../🚪️launch/🟦️.ts";
import{acquirePooledLaneV1,planPoolTrimV1,poolBoundsV1,poolNeedBytesV1,readPoolV1,trimPoolV1,type PoolLaneV1}from"../../🟦️.ts";
import corpus from"./🔣️.json";

const base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());mkdirSync(base,{recursive:true});
const scratch=()=>mkdtempSync(join(base,"fn-")),GIB=corpus.gib,bytesOf=(rows:readonly Record<string,unknown>[]):PoolLaneV1[]=>rows.map(row=>({claimed:false,lastUsedMs:0,crateKey:"",...row}) as PoolLaneV1);

test("the production rules and the Python oracle reach the verdict of every corpus case",()=>{
 const run=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const verdicts=JSON.parse(run.stdout);
 for(const[index,row]of corpus.need.entries()){expect(poolNeedBytesV1(bytesOf(row.lanes)),row.id+" production").toBe(row.expect);expect(verdicts.need[index],row.id+" oracle").toBe(row.expect);}
 for(const[index,row]of corpus.trim.entries()){expect(planPoolTrimV1(bytesOf(row.lanes),row.bounds,{requested:row.requested,pressure:row.pressure}),row.id+" production").toEqual(row.expect);expect(verdicts.trim[index],row.id+" oracle").toEqual(row.expect);}
});

const flavoured=async(root:string,names:readonly [string,number][],flavour:string,usedMs:number)=>{
 const ids:string[]=[];
 for(const[crate,bytes]of names){
  const lane=await acquirePooledLaneV1(root,{crates:[crate],quotaBytes:Number.MAX_SAFE_INTEGER,flavour});lane.release();ids.push(lane.paths.id);
  const paths=lanePathsV1(root,lane.paths.id),record=readLaneV1(root,lane.paths.id)!;writeFileSync(paths.metadata,JSON.stringify({...record,usedMs}));utimesSync(paths.metadata,new Date(0),new Date(0));recordLaneSizeV1(root,lane.paths.id,bytes,0,usedMs);
 }
 return ids;
};

test("restored release lanes on an 80 GiB image are not trimmed while a release build starts (only the stale lane goes), although their file times were reset and their sum exceeds the lane share",async()=>{
 const root=scratch(),now=Date.now();
 try{
  const ids=await flavoured(root,[["component",17*GIB],["host",8*GIB],["tool",6*GIB]],"r",now-3600_000);
  expect(laneUsedMsV1(root,ids[0]!)).toBe(now-3600_000);
  const stale=await flavoured(root,[["old",5*GIB]],"p",0);
  const share=Math.floor(76*GIB*0.234375),volume=()=>({volumeBytes:80*GIB,freeBytes:60*GIB}),said:string[]=[];
  expect(share).toBeLessThan(31*GIB);
  const result=trimPoolV1(root,poolBoundsV1(share),now,kind=>{said.push(kind);},{flavour:"r",volume});
  expect(result.removed).toBe(1);expect(said).toEqual([]);
  expect(readPoolV1(root,now).map(lane=>lane.id).sort()).toEqual([...ids].sort());
  expect(readPoolV1(root,now).find(lane=>lane.id===ids[0])).toMatchObject({recent:true,flavour:"r"});
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("under free-space pressure the least recently used lane that the build does not request is removed, and an unrequested stale lane goes before recent ones",async()=>{
 const root=scratch(),now=Date.now();
 try{
  const release=await flavoured(root,[["component",20*GIB],["host",10*GIB]],"r",now-3600_000),other=await flavoured(root,[["other",10*GIB]],"p",now-7200_000);
  const pressured=()=>({volumeBytes:80*GIB,freeBytes:5*GIB});
  const result=trimPoolV1(root,poolBoundsV1(1*GIB),now,undefined,{flavour:"r",volume:pressured});
  expect(result.removed).toBe(1);expect(readPoolV1(root,now).map(lane=>lane.id).sort()).toEqual([...release].sort());
  expect(existsSync(lanePathsV1(root,other[0]!).base)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("an image that cannot hold the need says so in English and German and removes nothing",async()=>{
 const root=scratch(),now=Date.now();
 try{
  const release=await flavoured(root,[["component",40*GIB],["host",20*GIB]],"r",now-3600_000),stale=await flavoured(root,[["old",5*GIB]],"p",0),said:Record<string,unknown>[]=[];
  const result=trimPoolV1(root,poolBoundsV1(1*GIB),now,(kind,parameters)=>{if(kind==="poolCannotHold")said.push(parameters);},{flavour:"r",volume:()=>({volumeBytes:50*GIB,freeBytes:1*GIB})});
  expect(result.removed).toBe(0);expect(said).toHaveLength(1);expect(said[0]).toMatchObject({flavour:"r",needMib:60*1024});
  expect(readPoolV1(root,now).map(lane=>lane.id).sort()).toEqual([...release,...stale].sort());
  for(const language of["en","de"]as const)expect(laneMessageV1("poolCannotHold",said[0] as Record<string,string|number>,language)).toContain("61440");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the lane share is what the declared reserves leave of the image",()=>{
 const text=readFileSync(fileURLToPath(new URL("../../../../🧬️schema/🔣️.json",import.meta.url)),"utf8"),gc=JSON.parse(text).definitions.Policy.const.gc;
 expect(gc.laneShare+gc.unitShare+gc.admissionShare+gc.incrementalShare+(1-gc.highWaterShare)).toBeLessThanOrEqual(1);
});
