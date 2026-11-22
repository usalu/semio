B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
def sub(path,old,new,count=1):
    s=open(B+path,encoding="utf8").read()
    assert s.count(old)==count,(path,old[:70],s.count(old))
    open(B+path,"w",encoding="utf8").write(s.replace(old,new))
P="🟦️.ts"
sub(P,'''import{createHash}from"node:crypto";''','''import{nativeShardConsumerV1,nativeShardTextV1,planNativeShardReliefV1,readNativeShardsV1,relieveNativeShardsV1,type NativeShardConsumerV1}from"../../../🌡️host-capacity/🧹️disk-recovery/🧱️native-shards/🟦️.ts";
import{nativeShardHoldV1}from"../../../🌡️host-capacity/🧹️disk-recovery/🧱️native-shards/🔒️hold/🟦️.ts";
import{createHash}from"node:crypto";''')
sub(P,'''/** 🩹️ The tail of a failed stage with the cause named: a database that "unable to open database file" while a path is out of space or inodes is a full disk, so the tail gets the line the classifier knows (`storeFullPattern`) for that path. */
export function readyTailWithDiskCauseV1(tail:readonly string[],short:ReadyDiskShortV1|null):readonly string[]{
 return short!==null&&tail.some(line=>line.includes(P.sqliteOpenPattern))?[...tail,`No space left (ENOSPC) in ${short.path}: the disk`]:tail;
}''','''/** 🩹️ The tail of a failed stage with the cause named: a database that "unable to open database file" while a path is out of space or inodes is a full disk, so the tail gets the line the classifier knows (`storeFullPattern`) for that path; a bare "no space left" from a tool that does not say where (Nx creating its lock file) gets the line for the native shard that cannot rewrite its file map, followed by the sentence that names it in both languages. */
export function readyTailWithDiskCauseV1(tail:readonly string[],short:ReadyDiskShortV1|null,native:NativeShardConsumerV1|null=null):readonly string[]{
 const named=short!==null&&tail.some(line=>line.includes(P.sqliteOpenPattern))?[...tail,`No space left (ENOSPC) in ${short.path}: the disk`]:tail;
 if(native===null||readyStoreFullV1(named)!==null||!readyNoSpaceV1(named))return named;
 const values={id:native.id.slice(0,12),path:native.path,used:Math.round(native.usedBytes/1048576),total:Math.round(native.totalBytes/1048576),file:native.file,map:Math.round(native.fileMapBytes/1048576),free:Math.round(native.freeBytes/1048576)};
 return[...named,`No space left (ENOSPC) in ${native.path}: the disk`,nativeShardTextV1("consumer","both",values)];
}''')
sub(P,'''sweep?(request:Readonly<{stage:string;neededMegabytes:number;signal:AbortSignal;report(line:string):void}>):Promise<ReadyDiskRecoveryV1>;''','''sweep?(request:Readonly<{stage:string;neededMegabytes:number;signal:AbortSignal;report(line:string):void}>):Promise<ReadyDiskRecoveryV1>;native?():NativeShardConsumerV1|null;relieve?(request:Readonly<{signal:AbortSignal;report(line:string):void}>):Promise<number>;''')
sub(P,'''    let result:ReadyRunResultV1,watching=true;''','''    if(inflightStage===undefined&&ports.relieve!==undefined)try{await ports.relieve({signal,report:line=>{if(!signal.aborted)ports.say(line);}});}catch{}
    let result:ReadyRunResultV1,watching=true;''')
sub(P,'''refusal:readyDiskCollectingV1(row.name,{path:readyStoreFullV1(result.tail)?.path??"-",''','''refusal:readyDiskCollectingV1(row.name,{path:readyStoreFullV1(result.tail)?.path??ports.native?.()?.path??"-",''')
sub(P,'''const failedTail=wrong!==null?[wrong.en,wrong.de]:readyTailWithDiskCauseV1(result.tail,result.tail.some(line=>line.includes(P.sqliteOpenPattern))?ports.disk?.(row.name)??null:null);''','''const failedTail=wrong!==null?[wrong.en,wrong.de]:readyTailWithDiskCauseV1(result.tail,result.tail.some(line=>line.includes(P.sqliteOpenPattern))?ports.disk?.(row.name)??null:null,readyNoSpaceV1(result.tail)?ports.native?.()??null:null);''')
sub(P,'''   compiler,
 memory:''','''   native:()=>nativeShardConsumerV1(readNativeShardsV1(input.root))??null,
   relieve:async request=>{const outcome=await relieveNativeShardsV1({facts:readNativeShardsV1(input.root),hold:nativeShardHoldV1(input.root),language:"both",signal:request.signal});for(const line of outcome.lines)request.report(line);return outcome.freedBytes;},
   compiler,
 memory:''')
