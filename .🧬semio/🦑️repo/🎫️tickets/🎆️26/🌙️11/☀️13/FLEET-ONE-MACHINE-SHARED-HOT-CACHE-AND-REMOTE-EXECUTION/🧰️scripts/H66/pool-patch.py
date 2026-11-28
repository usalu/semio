"""H66: pool wiring. usage: pool-patch.py <pool 🟦️.ts> (edits in place, atomic)."""
import os,sys
p=sys.argv[1]
s=open(p).read()
def once(old,new):
    global s
    assert s.count(old)==1,(s.count(old),old[:80])
    s=s.replace(old,new)
once('import{measureNativeImageCommitmentV1}from','import{growNativeImageShardPlannedV1,nativeImageLaneSizingV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🪜️growth/🟦️.ts";\nimport{measureNativeImageCommitmentV1}from')
once('measuredNeedBytes?():number|undefined;measure?','measuredNeedBytes?():number|undefined;laneSizes?():readonly number[];holders?(layout:NativeImageShardLayoutV1):Promise<number>;measure?')
once("the largest lane measured so far (`measuredNeedBytes`, what a new image is sized for and an existing one is stretched to)","the largest lane measured so far (`measuredNeedBytes`, what an existing image is stretched to), the sizes of the lanes measured so far (`laneSizes`: a new image is sized from their high percentile instead of the declared need, so a host with less room than the declared need still births one)")
once(' const live=nativeImageShardFoldV1(readNativeImageShardLedgerV1(ports.directory)),shardId=',' const sizing=ports.laneSizes===undefined?undefined:nativeImageLaneSizingV1(ports.rule,0,ports.laneSizes()),live=nativeImageShardFoldV1(readNativeImageShardLedgerV1(ports.directory)),shardId=')
old_birth='''const made=await ensureNativeImageShardV1({directory:ports.directory,rule:ports.rule,platform:ports.platform,path,hostFreeBytes:ports.hostFreeBytes,...ports.budgetBytes===undefined?{}:{budgetBytes:ports.budgetBytes},...ports.hostCommitment===undefined?{}:{hostRoomBytes:()=>laneImageHostRoomBytesV1(ports)},...ports.measuredNeedBytes===undefined?{}:{measuredNeedBytes:ports.measuredNeedBytes},now:ports.now'''
new_birth='''const born=sizing?.measuredNeedBytes===undefined?ports.measuredNeedBytes:()=>sizing.measuredNeedBytes,made=await ensureNativeImageShardV1({directory:ports.directory,rule:sizing?.rule??ports.rule,platform:ports.platform,path,hostFreeBytes:ports.hostFreeBytes,...ports.budgetBytes===undefined?{}:{budgetBytes:ports.budgetBytes},...ports.hostCommitment===undefined?{}:{hostRoomBytes:()=>laneImageHostRoomBytesV1(ports)},...born===undefined?{}:{measuredNeedBytes:born},now:ports.now'''
once(old_birth,new_birth)
i=s.index('    const grown=await growNativeImageShardV1(');j=s.index('\n',s.index('if(grown!==null)return',i))
s=s[:i]+'''    const grown=await growNativeImageShardPlannedV1({directory:ports.directory,rule:ports.rule,platform:ports.platform,id:made.layout.id,measure:ports.measure,...ports.budgetBytes===undefined?{}:{budgetBytes:ports.budgetBytes},...ports.hostCommitment===undefined?{}:{hostRoomBytes:()=>laneImageHostRoomBytesV1(ports)},...ports.measuredNeedBytes===undefined?{}:{measuredNeedBytes:ports.measuredNeedBytes},...ports.hostFreeFloorBytes===undefined?{}:{hostKeepBytes:ports.hostFreeFloorBytes},...ports.holders===undefined?{}:{holders:ports.holders},now:ports.now,lifecycle:ports.lifecycle,lock:ports.lock});
    if(grown.kind==="grown")return{kind:"grown",id:made.layout.id,detail:String(grown.capacityBytes)};
    if(grown.kind==="wait")return{kind:"existing",id:made.layout.id,detail:"grow: "+grown.reason};'''+s[j:]
once('import{NATIVE_IMAGE_SHARD_V1,createNativeImageShardDarwinLifecycleV1,ensureNativeImageShardV1,growNativeImageShardV1,','import{NATIVE_IMAGE_SHARD_V1,createNativeImageShardDarwinLifecycleV1,ensureNativeImageShardV1,')
once('measuredNeedBytes:()=>laneMeasuredNeedBytesV1(root),measure:','measuredNeedBytes:()=>laneMeasuredNeedBytesV1(root),laneSizes:()=>laneSizesV1(root),holders:async layout=>(await createNativeImageDetachPortV1({cwd:directory,run,sleep:async()=>{}}).holders(layout.mountPoint)).length,measure:')
once('import{growNativeImageShardPlannedV1,','import{createNativeImageDetachPortV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🔁️detach/🟦️.ts";\nimport{growNativeImageShardPlannedV1,')
once('/** 📏️ The most bytes one lane of a cache root has held','/** 📊️ The sizes of the lanes of a cache root as the collector recorded them (a lane without a record counts nothing): what a new lane image is sized from. */\nexport function laneSizesV1(root:string):readonly number[]{return laneIdsV1(root).map(id=>readLaneSizeV1(root,id)?.bytes??0).filter(bytes=>bytes>0);}\n/** 📏️ The most bytes one lane of a cache root has held')
tmp=p+".h66"
open(tmp,"w").write(s);os.replace(tmp,p)
print("patched",p)
