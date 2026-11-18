import sys,re,os,json
root=sys.argv[1]
FW="🧰️framework/"
DESC=FW+"🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/"
PLAT=FW+"🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🌐️platform/"
GEN=FW+"🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/"
def load(p): return open(os.path.join(root,p),encoding="utf8").read()
def save(p,s): open(os.path.join(root,p),"w",encoding="utf8").write(s)
def rep(s,a,b,n=1):
    c=s.count(a)
    if c!=n: raise SystemExit("anchor count %d != %d: %s"%(c,n,a[:90]))
    return s.replace(a,b)

p=DESC+"🧬️schema/🔣️.json"; s=load(p)
s=rep(s,'"maximumWriteBytes": 65536,','"maximumWriteBytes": 8388608,')
i=s.index('"RelativeProtocol": {'); j=s.index('"maximumReadBytes": 65536,',i)
k=s.index("\n",j)
s=s[:k]+'\n        "maximumFileReadBytes": 8388608,'+s[k:]
save(p,s)

p=DESC+"🟦️.ts"; s=load(p)
s=rep(s,'length>policy.maximumReadBytes||!Number.isSafeInteger(position+length)||offset+length>buffer.byteLength)throw Error("Bounded relative file read required")','length>policy.maximumFileReadBytes||!Number.isSafeInteger(position+length)||offset+length>buffer.byteLength)throw Error("Bounded relative file read required")')
s=rep(s,'length>policy.maximumReadBytes||!Number.isSafeInteger(position+length)||offset+length>buffer.byteLength)throw Error("Bounded Windows relative file read required")','length>policy.maximumFileReadBytes||!Number.isSafeInteger(position+length)||offset+length>buffer.byteLength)throw Error("Bounded Windows relative file read required")')
save(p,s)

p=GEN+"💾️storage/🟦️.ts"; s=load(p)
s=rep(s,'const relativeWriteCeilingBytes=65536,','const relativeWriteCeilingBytes=8388608,')
save(p,s)
p=GEN+"🧪️tests/🚪️ingress/🔣️.json"; s=load(p)
s=rep(s,'"sizes": [0, 1, 65535, 65536, 65537, 300000, 5000000], "chunkCeiling": 65536','"sizes": [0, 1, 65535, 65536, 65537, 300000, 5000000, 8388607, 8388608, 8388609], "chunkCeiling": 8388608')
save(p,s)
p=GEN+"🧪️tests/🚪️ingress/🟦️.ts"; s=load(p)
s=rep(s,'bytes:pattern(300000)}]','bytes:pattern(25000000)}]')
save(p,s)
p=GEN+"🧪️tests/🚪️ingress/🧬️schema/🔣️.json"; s=load(p)
s=rep(s,'"chunkCeiling": {"const": 65536}','"chunkCeiling": {"const": 8388608}')
save(p,s)

p=PLAT+"🧮️read-plan/🧬️schema/🔣️.json"; s=load(p)
s=rep(s,'    "WorkAllowance":','    "ChunkPlan": {"const": {"version": 1, "minimumEncodedExponent": 18, "maximumEncodedExponent": 23, "batchDivisor": 4, "memoryDivisor": 128}},\n    "WorkAllowance":')
s=rep(s,'Directories are walked once each, from their parent, not once per file from the root.",','Directories are walked once each, from their parent, not once per file from the root. A file is written and read back in steps of one chunk (`ChunkPlan`): the chunk is a power of two of base64 characters between 2^minimumEncodedExponent (256 KiB) and 2^maximumEncodedExponent (8 MiB) that is at most a `batchDivisor`-th of the bytes of the batch and at most a `memoryDivisor`-th of the free memory, so a batch is a few dozen steps and not one step for every 48 KiB: every step costs a write or a read call of the filesystem layer with its guards and its facts, one progress line and one yield, and the layer of a loaded Windows worker needs seconds for these, which made 165 MB a matter of hours. A step is one call of the layer, one progress line and one check after the yield; a cut-off run is cancelled at the next step.",')
save(p,s)

p=PLAT+"🧮️read-plan/🟦️.ts"; s=load(p)
s+='''
/** 🪣️ The step of staging and reading back the bytes of a batch: the largest power of two of base64 characters between the declared bounds that is at most a quarter of the batch and at most a 128th of the free memory (`decoded` is the byte count of `encoded` characters).
 * @see ./🧬️schema/🔣️.json */
export function protectedArtifactInputChunkV1(totalBytes:number,freeMemoryBytes:number):Readonly<{encoded:number;decoded:number}>{
 const plan=schema.definitions.ChunkPlan.const,want=Math.max(1,Math.min(Math.floor(totalBytes/plan.batchDivisor),Math.floor(freeMemoryBytes/plan.memoryDivisor)));
 let exponent=plan.minimumEncodedExponent;
 while(exponent<plan.maximumEncodedExponent&&2**(exponent+1)/4*3<=want)exponent++;
 return Object.freeze({encoded:2**exponent,decoded:2**exponent/4*3});
}

/** ✍️ Writes the bytes of one base64 file in steps of one chunk: one decode and one call of the file per step, one check before each call and one progress step after it; returns the bytes written. */
export async function copyProtectedArtifactInputV1(file:Readonly<{write(buffer:Uint8Array,offset:number,length:number,position:number):number}>,data:string,chunk:Readonly<{encoded:number}>,ports:Readonly<{check():void;step():Promise<void>}>):Promise<number>{
 let position=0;
 for(let offset=0;offset<data.length;offset+=chunk.encoded){
  const bytes=Buffer.from(data.slice(offset,offset+chunk.encoded),"base64");
  for(let consumed=0;consumed<bytes.length;){ports.check();const count=file.write(bytes,consumed,bytes.length-consumed,position);if(!count)throw Error("Immutable input write made no progress");consumed+=count;position+=count;}
  await ports.step();
 }
 return position;
}

/** 📖️ Feeds the bytes of one staged file to `update` in steps of one chunk: one call of the file per step, one check before it and one progress step after it; returns the bytes read. */
export async function readProtectedArtifactInputV1(file:Readonly<{read(buffer:Uint8Array,offset:number,length:number,position:number):number}>,bytes:number,chunk:Readonly<{decoded:number}>,update:(buffer:Uint8Array)=>void,ports:Readonly<{check():void;step():Promise<void>}>):Promise<number>{
 const buffer=Buffer.alloc(Math.max(1,Math.min(chunk.decoded,bytes)));
 let offset=0;
 while(offset<bytes){ports.check();const count=file.read(buffer,0,Math.min(buffer.length,bytes-offset),offset);if(!count)throw Error("Complete staged input read extent required");offset+=count;update(buffer.subarray(0,count));await ports.step();}
 return offset;
}
'''
save(p,s)

p=PLAT+"🟦️.ts"; s=load(p)
s=rep(s,'import{protectedArtifactInputHashedV1,protectedArtifactInputWorkAllowanceV1,visitProtectedArtifactInputDirectoriesV1,type ProtectedArtifactInputHashModeV1}from"./🧮️read-plan/🟦️.ts";','import{copyProtectedArtifactInputV1,protectedArtifactInputChunkV1,protectedArtifactInputHashedV1,protectedArtifactInputWorkAllowanceV1,readProtectedArtifactInputV1,visitProtectedArtifactInputDirectoriesV1,type ProtectedArtifactInputHashModeV1}from"./🧮️read-plan/🟦️.ts";\nimport{freemem}from"node:os";')
pre='for(const file of batch.files){checkResourceLeaseControl(control);if(++work.count>limits.maximumWork)throw Error("Fresh artifact input verification work exceeded");const hash='
s=rep(s,pre,'const chunk=protectedArtifactInputChunkV1(batch.files.reduce((sum,row)=>sum+row.bytes,0),freemem());'+pre,2)
s=rep(s,'for(let offset=0;offset<file.data.length;offset+=limits.chunkBytes){checkResourceLeaseControl(control);if(++work.count>limits.maximumWork)throw Error("Fresh artifact input verification work exceeded");const encoded=file.data.slice(offset,offset+limits.chunkBytes)','for(let offset=0;offset<file.data.length;offset+=chunk.encoded){checkResourceLeaseControl(control);if(++work.count>limits.maximumWork)throw Error("Fresh artifact input verification work exceeded");const encoded=file.data.slice(offset,offset+chunk.encoded)',2)
al='const allowance=protectedArtifactInputWorkAllowanceV1(batch,limits.chunkBytes),check=()=>{if(++work.count>allowance)throw Error("Immutable input physical work capacity exceeded")'
s=rep(s,al,al.replace('check=()=>','chunk=protectedArtifactInputChunkV1(batch.files.reduce((sum,row)=>sum+row.bytes,0),freemem()),check=()=>',1),2)
loop='let position=0;for(let offset=0;offset<input.data.length;offset+=limits.chunkBytes){const bytes=Buffer.from(input.data.slice(offset,offset+limits.chunkBytes),"base64");let consumed=0;while(consumed<bytes.length){check();const count=file.write(bytes,consumed,bytes.length-consumed,position);if(!count)throw Error("Immutable input write made no progress");consumed+=count;position+=count;}await actual.publish({resource:"native-artifact-input-stage:"+nativeArtifactInputStepV1(),mode:"exclusive",elapsedMs:0});await actual.yieldContinuation();check();}'
s=rep(s,loop,'const position=await copyProtectedArtifactInputV1(file,input.data,chunk,{check,step:async()=>{await actual.publish({resource:"native-artifact-input-stage:"+nativeArtifactInputStepV1(),mode:"exclusive",elapsedMs:0});await actual.yieldContinuation();check();}});',2)
rb='const hash=artifactRetirementHashV1("sha256"),buffer=Buffer.alloc(limits.chunkBytes);let offset=0;while(offset<input.bytes){check();const count=file.read(buffer,0,Math.min(buffer.length,input.bytes-offset),offset);check();if(!count)throw Error("Complete staged input read extent required");offset+=count;hash.update(buffer.subarray(0,count));await control.publish({resource:"native-artifact-input-read:"+nativeArtifactInputStepV1(),mode:"exclusive",elapsedMs:0});await control.yieldContinuation();check();}if(hash.digest("hex")!==input.sha256'
s=rep(s,rb,'const hash=artifactRetirementHashV1("sha256");await readProtectedArtifactInputV1(file,input.bytes,chunk,piece=>hash.update(piece),{check,step:async()=>{await control.publish({resource:"native-artifact-input-read:"+nativeArtifactInputStepV1(),mode:"exclusive",elapsedMs:0});await control.yieldContinuation();check();}});if(hash.digest("hex")!==input.sha256')
vf='const limits=nativeProtectedArtifactInputProtocolV1(),closed=record||fresh,rosters=new Map<string,Set<string>>(),'
s=rep(s,vf,vf+'chunk=protectedArtifactInputChunkV1(batch.files.reduce((sum,row)=>sum+row.bytes,0),freemem()),')
save(p,s)
print("edited",root)
