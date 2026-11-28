import os,sys,shutil
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
if len(sys.argv)>1: R=sys.argv[1]
U=R+"⚡️caching/🦀️cargo/🗃️unit-cache/"
READY=R+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
HERE=os.path.dirname(os.path.abspath(__file__))
def edit(p,pairs):
    s=open(p,encoding="utf8").read()
    for old,new in pairs:
        if s.count(old)!=1: sys.exit("anchor count %d in %s: %s"%(s.count(old),p,old[:90]))
        s=s.replace(old,new)
    t=p+".h47tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,p)
def create(p,text):
    os.makedirs(os.path.dirname(p),exist_ok=True)
    if os.path.exists(p): sys.exit("exists "+p)
    t=p+".h47tmp";open(t,"w",encoding="utf8").write(text);os.replace(t,p)
SAY='process.stderr.write(`[lane] image of lane ${id}: ${image.kind}${image.detail===undefined?"":" ("+image.detail+")"} / Image der Spur ${id}: ${image.kind}${image.detail===undefined?"":" ("+image.detail+")"}\\n`)'
# 1 new module
create(U+"🧵️lane/🏊️pool/🧩️image/🟦️.ts",open(HERE+"/lane-image.ts.txt",encoding="utf8").read())
# 2 lane: mount-safe removal
edit(U+"🧵️lane/🟦️.ts",[
('import {closeSync,mkdirSync,openSync,readFileSync,readdirSync,renameSync,rmSync,statSync,utimesSync,writeFileSync,writeSync} from "node:fs";','import {closeSync,lstatSync,mkdirSync,openSync,readFileSync,readdirSync,renameSync,rmSync,statSync,utimesSync,writeFileSync,writeSync} from "node:fs";'),
('/** 🗑️ Removes a whole lane or only its incremental directories.','''/** 🧩️ Removes the files of a lane base; a base that is the mount point of its own image (a lane shard) keeps the directory, because a mount point cannot be removed while the image is mounted: its files go and the idle image is retired whole afterwards (`retireIdleLaneImagesV1`). */
function removeLaneTreeV1(base:string):void{
 let mounted=false;try{mounted=lstatSync(base).dev!==lstatSync(join(base,"..")).dev;}catch{}
 if(!mounted){rmSync(base,{recursive:true,force:true});return;}
 for(const name of readdirSync(base))rmSync(join(base,name),{recursive:true,force:true});
}

/** 🗑️ Removes a whole lane or only its incremental directories.'''),
('if(beforeRemoval?.(paths)===false)return false;rmSync(paths.base,{recursive:true,force:true});}else','if(beforeRemoval?.(paths)===false)return false;removeLaneTreeV1(paths.base);}else'),
])
# 3 pool
edit(U+"🧵️lane/🏊️pool/🟦️.ts",[
('import {retainRemovedLaneV1} from "../📤️retention/🟦️.ts";','import {retainRemovedLaneV1} from "../📤️retention/🟦️.ts";\nimport {ensureLaneImageV1,laneImagePortsV1,repairLaneImagesV1,retireIdleLaneImagesV1} from "./🧩️image/🟦️.ts";'),
(' mkdirSync(join(root,layout.lanes),{recursive:true});\n let poll=rules.pollMilliseconds,reported=false;\n for(;;){',
 ' mkdirSync(join(root,layout.lanes),{recursive:true});\n const lanePorts=await laneImagePortsV1(root,options.signal);\n if(lanePorts!==undefined)try{await repairLaneImagesV1(lanePorts);await retireIdleLaneImagesV1(root,lanePorts,now());}catch{}\n let poll=rules.pollMilliseconds,reported=false;\n for(;;){'),
('  try{acquired=(()=>{\n','  try{acquired=await(async()=>{\n'),
('  let entered:ReturnType<typeof enterLaneV1>;try{entered=enterLaneV1(root,id,request.owner,request.crates,request.flavour);}catch(error){claim.release();throw error;}',
 '  let entered:ReturnType<typeof enterLaneV1>;try{const image=await ensureLaneImageV1(root,id,lanePorts);if(image.kind==="birth"||image.kind==="failed"||image.kind==="refused")'+SAY+';entered=enterLaneV1(root,id,request.owner,request.crates,request.flavour);}catch(error){claim.release();throw error;}'),
])
# 4 product
edit(U+"🧵️lane/🏊️pool/🎯️product/🟦️.ts",[
('import {boundedPoolV1,laneHoldersV1,poolBoundsV1,readPoolV1,trimPoolV1,type PoolReportV1,type PoolTrimContextV1} from "../🟦️.ts";','import {boundedPoolV1,laneHoldersV1,poolBoundsV1,readPoolV1,trimPoolV1,type PoolReportV1,type PoolTrimContextV1} from "../🟦️.ts";\nimport {ensureLaneImageV1,laneImagePortsV1,repairLaneImagesV1,retireIdleLaneImagesV1} from "../🧩️image/🟦️.ts";'),
(' mkdirSync(join(root,layout.lanes),{recursive:true});\n for(;;){\n  options.signal.throwIfAborted();const catalog=',
 ' mkdirSync(join(root,layout.lanes),{recursive:true});\n const lanePorts=await laneImagePortsV1(root,options.signal);\n if(lanePorts!==undefined)try{await repairLaneImagesV1(lanePorts);await retireIdleLaneImagesV1(root,lanePorts,Date.now());}catch{}\n for(;;){\n  options.signal.throwIfAborted();const catalog='),
('  const acquired=(()=>{try{\n','  const acquired=await(async()=>{try{\n'),
('   options.signal.throwIfAborted();const receipt=join(paths.base,policy.product.receipt);',
 '   options.signal.throwIfAborted();{const image=await ensureLaneImageV1(root,id,lanePorts);if(image.kind==="birth"||image.kind==="failed"||image.kind==="refused")'+SAY+';}\n   const receipt=join(paths.base,policy.product.receipt);'),
])
# 5 ready guard: shard images and the home store of a shard path
edit(READY+"🖼️image-guard/🟦️.ts",[
('import{closeSync,fstatSync,openSync,readSync,readdirSync,statSync}from"node:fs";\nimport{join}from"node:path";',
 'import{closeSync,fstatSync,openSync,readSync,readdirSync,statSync}from"node:fs";\nimport{join}from"node:path";\nimport{nativeImageShardFoldV1,nativeImageShardLayoutsV1,nativeImageShardPartitionOfV1,readNativeImageShardLedgerV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";'),
('  rows.push(Object.freeze({partition:name,path,freeBytes:row.freeBytes,totalBytes:row.totalBytes,hostFreeBytes}));\n }\n return Object.freeze(rows);',
 '  rows.push(Object.freeze({partition:name,path,freeBytes:row.freeBytes,totalBytes:row.totalBytes,hostFreeBytes}));\n  let shards:readonly string[]=[];try{shards=nativeImageShardLayoutsV1(join(base,name),nativeImageShardFoldV1(readNativeImageShardLedgerV1(join(base,name)))).slice(1).map(layout=>layout.mountPoint);}catch{}\n  for(const shard of shards){\n   try{if(!ownVolume(shard,base))continue;}catch{continue;}\n   const shardRow=facts(shard);\n   if(shardRow===null||typeof shardRow.freeBytes!=="number"||typeof shardRow.totalBytes!=="number"||shardRow.totalBytes<policy.minimumTotalMegabytes*MB)continue;\n   rows.push(Object.freeze({partition:name,path:shard,freeBytes:shardRow.freeBytes,totalBytes:shardRow.totalBytes,hostFreeBytes}));\n  }\n }\n return Object.freeze(rows);'),
('/** 🌡️ Whether the free bytes of an image are those of the host volume',
 '/** 🏠️ The home mount of the partition that holds a path: a path inside a lane shard answers the mount of the home image whose collectors manage every image of the partition, any other path answers itself. */\nexport function readyImageStoreV1(path:string):string{const directory=nativeImageShardPartitionOfV1(path);return directory===undefined?path:join(directory,"m");}\n\n/** 🌡️ Whether the free bytes of an image are those of the host volume'),
])
edit(READY+"🟦️.ts",[
('readyImageReliefV1,readyImageRetainBytesV1,readyImageShortV1,readyImageWritersV1,','readyImageReliefV1,readyImageRetainBytesV1,readyImageShortV1,readyImageStoreV1,readyImageWritersV1,'),
('await reclaimUnitStorageV1(request.short.path,Math.max(','await reclaimUnitStorageV1(readyImageStoreV1(request.short.path),Math.max('),
('await reclaimUnitStorageV1(image.path,Math.max(','await reclaimUnitStorageV1(readyImageStoreV1(image.path),Math.max('),
])
print("shards landed")
create(U+"🧵️lane/🏊️pool/🧩️image/🧪️tests/🟦️.ts",open(HERE+"/lane-image-tests.ts.txt",encoding="utf8").read())
print("shard tests landed")
edit(READY+"🖼️image-guard/🧪️tests/🟦️.ts",[('import{readyImageReliefBytesV1,readyImageReliefV1,readyNoSpaceEvidenceV1}from"../🟦️.ts";','import{readyImageReliefBytesV1,readyImageReliefV1,readyImageStoreV1,readyNoSpaceEvidenceV1}from"../🟦️.ts";\nimport{appendNativeImageShardEventV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";')])
with open(READY+"🖼️image-guard/🧪️tests/🟦️.ts","a",encoding="utf8") as handle:
    handle.write('''
test("the shard images of a partition are guarded like its home image and their collection runs on the home mount",()=>{
 const workspace=mkdtempSync(join(tmpdir(),"ready-shards-")),directory=join(workspace,".cache","b","r");
 try{
  mkdirSync(join(directory,"m","u","l","q90be4b"),{recursive:true});
  appendNativeImageShardEventV1(directory,{type:"born",id:"q90be4b",key:"q90be4b",root:0,path:"u/l/q90be4b",capacityBytes:24*GB,atMs:1});
  const free=(path:string)=>({freeBytes:(path.endsWith("q90be4b")?500:9000)*MB,totalBytes:(path.endsWith("q90be4b")?24576:40959)*MB});
  const rows=readyImageFactsV1(workspace,"workspace",free,P,()=>true);
  expect(rows.map(row=>[row.partition,row.path.endsWith("q90be4b")])).toEqual([["r",false],["r",true]]);
  const hit=readyImageGuardV1(rows)!;
  expect(hit.partition).toBe("r");
  expect(hit.path.endsWith("q90be4b")).toBe(true);
  expect(readyImageStoreV1(join(directory,"m","u","l","q90be4b"))).toBe(join(directory,"m"));
  expect(readyImageStoreV1(join(directory,"m"))).toBe(join(directory,"m"));
  expect(readyImageStoreV1(join(workspace,"elsewhere"))).toBe(join(workspace,"elsewhere"));
 }finally{rmSync(workspace,{recursive:true,force:true});}
});
''')
print("guard shard tests landed")
