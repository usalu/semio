import os
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
T=B+"🏗️build/🧹️cache-gc/🧪️tests/🟦️.ts"
s=open(T,encoding="utf8").read()
def rep(old,new):
    global s
    assert s.count(old)==1,old[:60];s=s.replace(old,new)
rep('import {cacheRunnerPortsV1,cacheSqliteFreeV1,collectCacheV1,','import {cacheRunnerPortsV1,cacheSqliteFreeV1,collectCacheV1,createCacheImagesV1,type CacheImagesV1,')
rep('import {cacheArgumentsV1,runCacheLedgerCommandV1} from "../⌨️command/🟦️.ts";\n','''import {cacheArgumentsV1,runCacheLedgerCommandV1} from "../⌨️command/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import {acquireResourceLease} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {nativeImageSharedReadResourceV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts";
import {CACHE_RUNTIME_V1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
''')
s=s.rstrip("\n")+'''

const imageLedger:CacheLedgerV1=readCacheLedgerV1({
 version:1,
 roots:{repository:{directory:"r/cache",maximumBytes:1<<30,undeclaredUnusedAgeMs:14*DAY,undeclaredQuietMs:HOUR,ignored:[".DS_Store",CACHE_SIZE_FILE_V1]},native:{directory:".cache",maximumBytes:1<<20,undeclaredUnusedAgeMs:7*DAY,undeclaredQuietMs:HOUR,ignored:[]},rust:{directory:".r",maximumBytes:0,undeclaredUnusedAgeMs:1000,undeclaredQuietMs:0,ignored:[]}},
 areas:[
  {id:"alpha",root:"native",kind:"partition",directory:"b/a",partition:"alpha",managed:{owner:"image"},writers:["w"],purpose},
  {id:"units",root:"native",kind:"nested",directory:"units",partition:"alpha",budget:{bytes:1<<20},collect:{unusedAgeMs:DAY,quietMs:0,custody:"quiet"},writers:["w"],purpose}
 ],
 retirements:[]
},partitions);
const none=new AbortController().signal;
async function withLeases<T>(body:(leaseDirectory:string,image:string,busy:()=>ReturnType<typeof acquireResourceLease>)=>Promise<T>):Promise<T>{
 const leaseDirectory=mkdtempSync(join(testScratchRootV1("cgc-lease"),"l-")),image=join(workspace,".cache/b/a");
 try{return await body(leaseDirectory,image,()=>acquireResourceLease({directory:leaseDirectory,resource:nativeImageSharedReadResourceV1(image),mode:"exclusive",signal:none,timeoutMs:0}));}
 finally{rmSync(leaseDirectory,{recursive:true,force:true});}
}

test("every walk, probe and removal below an image runs in short steps under its shared lease, a path outside every image takes none, and settling gives the lease back",async()=>{
 reset();
 await withLeases(async(leaseDirectory,image,busy)=>{
  for(let index=0;index<70;index++)file(".cache/b/a/m/units/u/f"+index,10,DAY);
  file(".cache/b/a/m/units/u/lease.sqlite",10,DAY);file("r/cache/loose/x",10,DAY);
  const real=createCacheImagesV1({workspace,partitions,leaseDirectory,signal:none,timeoutMs:2000});let steps=0,refused=0;
  const images:CacheImagesV1={reader:path=>{const reader=real.reader(path);return{run:body=>{if(path.startsWith(image))steps++;return reader.run(body);}};},settle:()=>real.settle()};
  const active=ports({images,sqliteFree:async()=>{await expect(busy()).rejects.toThrow("timed out");refused++;return true;}}),tree=join(image,"m/units/u"),count=71;
  const measure=await measureCacheTreeV1(tree,active,1000);
  expect(measure.entries).toBe(count);expect(measure.sqlite.length).toBe(1);expect(steps,"one stat, one listing, one batch of entries").toBe(3);
  expect(await probeCacheHeldV1(measure,active)).toBeUndefined();expect(refused,"the lock probe ran inside a step").toBe(1);
  steps=0;await removeCacheTreeV1(tree,active,"t");
  expect(existsSync(tree)).toBe(false);
  expect(steps,"one stat, one listing, the removal batches, one directory").toBe(3+Math.ceil(count/CACHE_RUNTIME_V1.removeBatch));
  steps=0;await measureCacheTreeV1(join(workspace,"r/cache/loose"),active,1000);await removeCacheTreeV1(join(workspace,"r/cache/loose"),active,"t");
  expect(steps,"a loose tree is no image").toBe(0);
  await images.settle();(await busy()).release();
 });
});

test("a collection below an image keeps the area when a writer holds the image and removes what is old once it is free; status takes no lease and deep status waits for it",async()=>{
 reset();
 await withLeases(async(leaseDirectory,image,busy)=>{
  file(".cache/b/a/m/units/old/x.bin",100,5*DAY);file(".cache/b/a/m/units/new/x.bin",100,60000);
  const holder=await busy(),slow=createCacheImagesV1({workspace,partitions,leaseDirectory,signal:none,timeoutMs:150}),out=lines();
  const refused=await collectCacheV1(workspace,imageLedger,{dryRun:false,only:new Set(["units"])},ports({images:slow}),out,partitions);
  expect(refused.removed).toEqual([]);expect(refused.failed).toEqual([]);
  expect(out.out.some(line=>line.includes("units")&&line.includes("image:alpha")),out.out.join("\\n")).toBe(true);
  expect(existsSync(join(image,"m/units/old/x.bin"))).toBe(true);
  const quick=await reportCacheLedgerV1(workspace,imageLedger,ports({images:slow}),partitions,new Set(["alpha","units"]));
  expect(quick.partitions.length).toBe(1);
  await expect(reportCacheLedgerV1(workspace,imageLedger,ports({images:slow}),partitions,new Set(["alpha","units"]),{deep:true})).rejects.toThrow("timed out");
  await slow.settle();holder.release();
  const free=createCacheImagesV1({workspace,partitions,leaseDirectory,signal:none,timeoutMs:2000}),second=await collectCacheV1(workspace,imageLedger,{dryRun:false,only:new Set(["units"])},ports({images:free}),lines(),partitions);
  expect(second.failed).toEqual([]);expect(second.removed.map(row=>row.name)).toEqual(["old"]);
  expect(existsSync(join(image,"m/units/old"))).toBe(false);expect(existsSync(join(image,"m/units/new/x.bin"))).toBe(true);
  await free.settle();(await busy()).release();
 });
});

test("an owner that delegates takes its image only after the collection gave every lease back",async()=>{
 reset();
 await withLeases(async(leaseDirectory,image,busy)=>{
  file(".cache/b/a/m/units/old/x.bin",100,5*DAY);
  const real=createCacheImagesV1({workspace,partitions,leaseDirectory,signal:none,timeoutMs:2000});let free=false;
  const answered=ports({images:real,delegate:async()=>{free=await busy().then(lease=>{lease.release();return true;},()=>false);return undefined;}});
  const withOwner:CacheLedgerV1=readCacheLedgerV1({...JSON.parse(JSON.stringify(imageLedger)),areas:[...JSON.parse(JSON.stringify(imageLedger.areas)),{id:"kept",root:"repository",kind:"tree",directory:"kept",budget:{bytes:100000},managed:{owner:"test-evidence"},writers:["w"],purpose}]},partitions);
  await collectCacheV1(workspace,withOwner,{dryRun:false},answered,lines(),partitions);
  expect(free,"the image was free when the owner was asked").toBe(true);
  await real.settle();
 });
});
'''
tmp=T+".f127tmp";open(tmp,"w",encoding="utf8").write(s);os.replace(tmp,T)
print("gc tests ok")
