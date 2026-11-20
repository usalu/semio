import {strict as assert} from "node:assert";
import {mkdtempSync,mkdirSync,lstatSync,rmSync,statfsSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {spawnSync} from "node:child_process";
import {randomBytes} from "node:crypto";

const source=resolve("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage");
const ref=(path:string)=>pathToFileURL(join(source,path)).href;
const directory=await import(ref("📁️directory/🟦️.ts"));
const {observeJournalRevisionV1}=await import(ref("📜️journal/🔖️revision/🟦️.ts"));
const {nativeStorageStateV1}=await import(ref("📜️journal/🟦️.ts"));
const scratch=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!scratch)throw Error("Ticket-generated scratch required");
const base=mkdtempSync(join(resolve(scratch),"n-")),filesystemBytes=33554432,maximumPhysicalBytes=34603008;
const partitions:string[]=[];
const scopes:string[]=[];
const physical=(path:string)=>{const row=lstatSync(path,{bigint:true});return [row.dev,row.ino,row.mode,row.mtimeNs,row.ctimeNs].map(String);};
const probe=(scope:string)=>{
 const program=`import os,stat,sys,json
from collections import defaultdict
counts=defaultdict(int)
seen=set()
stack=[sys.argv[1]]
while stack:
 p=stack.pop()
 r=os.lstat(p)
 counts["entries"]+=1
 if stat.S_ISREG(r.st_mode) and r.st_nlink>1:
  k=r.st_dev,r.st_ino
  if k in seen: continue
  seen.add(k)
 counts["allocatedBytes"]+=r.st_blocks*512
 if stat.S_ISREG(r.st_mode): counts["logicalBytes"]+=r.st_size
 if stat.S_ISDIR(r.st_mode): stack.extend(os.path.join(p,n) for n in os.listdir(p))
print(json.dumps(counts))`;
 const python=Bun.which("python3")??Bun.which("python");
 if(python===null)throw Error("Python filesystem oracle required");
 const row=spawnSync(python,["-I","-c",program,scope],{encoding:"utf8"});
 assert.equal(row.status,0,row.stderr);
 return JSON.parse(row.stdout);
};

try{
 for(const name of ["a","b"]){
  const path=join(base,name),root=join(path,"m");
  mkdirSync(root,{recursive:true});directory.allocateNativeDirectoryV1({directory:path,filesystemBytes,maximumPhysicalBytes},()=>true);
  directory.activateNativeDirectoryV1(path);
  partitions.push(path);scopes.push(root);
 }
 const [root,sibling]=scopes, nested=join(root!,"n"),journal=join(root!,"storage-events.jsonl");
 mkdirSync(nested);
 for(let i=0;i<512;i++)writeFileSync(join(nested,String(i)),randomBytes(1024));
 writeFileSync(journal,JSON.stringify({type:"checkpoint",state:nativeStorageStateV1()})+"\n");
 const mapping=directory.nativeDirectoryMappingV1(partitions[0]),revision=observeJournalRevisionV1(journal),rootIdentity=physical(root!),left=statfsSync(root!,{bigint:true}),right=statfsSync(sibling!,{bigint:true});
 assert.ok(revision);assert.equal(lstatSync(root!,{bigint:true}).dev,lstatSync(sibling!,{bigint:true}).dev);
 assert.equal(left.blocks,right.blocks);assert.equal(left.bsize,right.bsize);
 const firstOracle=probe(root!),siblingOracle=probe(sibling!);
 assert.ok(firstOracle.allocatedBytes>siblingOracle.allocatedBytes);
 const observations=[];
 let clock=4000;
 const usage=directory.createNativeDirectoryUsageV1({root,maximumEntries:1024,now:()=>clock,measure:async(path:string,signal:AbortSignal)=>{
  const start=performance.now(),row=await directory.measureNativeDirectoryUsageAsyncV1(path,{signal,maximumEntries:1024});
  observations.push({row,milliseconds:Math.round(performance.now()-start)});
  return row.allocatedBytes;
 }});
 const meter=directory.createNativeDirectoryMeterV1({root,filesystemBytes,maximumPhysicalBytes,usage,now:()=>clock}),recheck={mode:"recheck",requestBytes:0};
 const interval=directory.nativeDirectoryProtocolV1().meter.intervalMilliseconds;
 await meter.admit(recheck);assert.equal(observations.length,1);assert.deepEqual(observations[0].row,firstOracle);
 clock+=interval-1;await meter.admit(recheck);assert.equal(observations.length,1);
 clock++;await meter.admit(recheck);assert.equal(observations.length,2);assert.deepEqual(observations[1].row,firstOracle);
 writeFileSync(join(nested,"0"),randomBytes(filesystemBytes+1048576));
 assert.equal(directory.nativeDirectoryMappingV1(partitions[0]),mapping);
 assert.equal(observeJournalRevisionV1(journal),revision);
 assert.deepEqual(physical(root!),rootIdentity);
 const largerOracle=probe(root!);
 assert.ok(largerOracle.allocatedBytes>maximumPhysicalBytes);
 clock+=interval;
 await assert.rejects(meter.admit(recheck),/cap|limit|bound|envelope/i);
 assert.equal(observations.length,3);assert.deepEqual(observations[2].row,largerOracle);
 console.log(JSON.stringify({interval,observations,firstOracle,siblingOracle,largerOracle,kernelVolumeBlocks:String(left.blocks),kernelBlockBytes:String(left.bsize),sameKernelDevice:true,mappingUnchanged:true,rootStatUnchanged:true,journalRevisionUnchanged:true,refused:true}));
}finally{
 for(const partition of partitions.reverse())directory.deactivateNativeDirectoryV1(partition);
 rmSync(base,{recursive:true,force:true});
}
