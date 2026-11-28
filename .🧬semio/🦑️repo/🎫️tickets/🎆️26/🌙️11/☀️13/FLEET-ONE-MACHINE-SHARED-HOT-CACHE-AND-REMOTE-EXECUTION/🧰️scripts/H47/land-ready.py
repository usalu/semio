import os,sys,tempfile
D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/"
def edit(rel,pairs):
    p=D+rel;s=open(p,encoding="utf8").read()
    for old,new in pairs:
        if s.count(old)!=1: sys.exit("anchor count %d in %s: %s"%(s.count(old),rel,old[:80]))
        s=s.replace(old,new)
    t=p+".h47tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,p)
if len(sys.argv)>1 and sys.argv[1]=="check":
    for rel,pairs in []:pass
# schema
edit("🖼️image-guard/🧬️schema/🔣️.json",[(
'"hostLimitedBytes": 16777216,',
'"reliefBasisPoints": 2000,\n        "reliefMinimumMegabytes": 1024,\n        "reliefMaximumMegabytes": 8192,\n        "reliefRetryMilliseconds": 120000,\n        "reliefPartitions": ["r"],\n        "checkMilliseconds": 1000,\n        "noSpaceScanBytes": 268435456,\n        "noSpaceLineCharacters": 600,\n        "hostLimitedBytes": 16777216,')])
# image-guard module
edit("🖼️image-guard/🟦️.ts",[
('import{readdirSync,statSync}from"node:fs";','import{closeSync,fstatSync,openSync,readSync,readdirSync,statSync}from"node:fs";'),
('/** 🚫️ The image that lacks the most of its need before a stage starts',
'''/** 🌊️ The free bytes below which an image of a partition with a collector is collected in place while stages run and before they start: `reliefBasisPoints` of the image clamped to the declared minimum and maximum, never below the need. Collecting at this low-water mark keeps the guard floor a rare last resort. Pure and language-agnostic. */
export function readyImageReliefBytesV1(totalBytes:number,policy=READY_IMAGE_POLICY_V1):number{
 return Math.max(readyImageNeedBytesV1(totalBytes,policy),clamp(Math.floor(totalBytes*policy.reliefBasisPoints/10000),policy.reliefMinimumMegabytes*MB,policy.reliefMaximumMegabytes*MB));
}
/** 🌊️ The image of a partition with a collector that fell below its low-water mark (the one with the smallest share of it free), `null` when none did. The row has the shape of a short image, so `needed` is the low-water mark in megabytes. Pure and language-agnostic. */
export function readyImageReliefV1(images:readonly ReadyImageFactV1[],policy=READY_IMAGE_POLICY_V1):ReadyImageShortV1|null{
 let worst:{image:ReadyImageFactV1;relief:number}|null=null;
 for(const image of images){
  if(!(policy.reliefPartitions as readonly string[]).includes(image.partition))continue;
  const relief=readyImageReliefBytesV1(image.totalBytes,policy);
  if(image.freeBytes>=relief)continue;
  if(worst===null||image.freeBytes/relief<worst.image.freeBytes/worst.relief)worst={image,relief};
 }
 return worst===null?null:Object.freeze({partition:worst.image.partition,path:worst.image.path,resource:"space" as const,needed:Math.ceil(worst.relief/MB),available:Math.floor(worst.image.freeBytes/MB),total:Math.floor(worst.image.totalBytes/MB),host:typeof worst.image.hostFreeBytes==="number"?Math.floor(worst.image.hostFreeBytes/MB):null,limitedByHost:readyImageHostLimitedV1(worst.image,policy)});
}
/** 🔦️ The last line of a stage log that says a write found no space left, however many lines followed it (a stage that goes on for minutes prints thousands of progress lines, so a tail window loses the one line that names the full image); at most `noSpaceScanBytes` of the end of the log are read, `[]` when there is none. */
export function readyNoSpaceEvidenceV1(log:string,patterns:readonly string[],policy=READY_IMAGE_POLICY_V1):string[]{
 try{
  const handle=openSync(log,"r");
  try{
   const size=fstatSync(handle).size,block=4*MB;let end=size,floor=Math.max(0,size-policy.noSpaceScanBytes),carry=Buffer.alloc(0);
   while(end>floor){
    const start=Math.max(floor,end-block),buffer=Buffer.alloc(end-start);readSync(handle,buffer,0,buffer.length,start);
    const joined=Buffer.concat([buffer,carry]),lines=joined.toString("utf8").split("\\n");
    carry=Buffer.from(lines[0]!,"utf8");
    for(let at=lines.length-1;at>=1;at--){const plain=lines[at]!.replace(/\\u001b\\[[0-9;]*m/gu,"").trim();if(!/^\\d+\\s\\|/u.test(plain)&&patterns.some(pattern=>plain.includes(pattern)))return[plain.slice(0,policy.noSpaceLineCharacters)];}
    end=start;
   }
   if(carry.length>0){const plain=carry.toString("utf8").replace(/\\u001b\\[[0-9;]*m/gu,"").trim();if(!/^\\d+\\s\\|/u.test(plain)&&patterns.some(pattern=>plain.includes(pattern)))return[plain.slice(0,policy.noSpaceLineCharacters)];}
   return[];
  }finally{closeSync(handle);}
 }catch{return[];}
}

/** 🚫️ The image that lacks the most of its need before a stage starts''')])

edit("🟦️.ts",[
('import{READY_IMAGE_POLICY_V1,readyImageFactsV1,readyImageGuardV1,readyImagePartitionOfLinesV1,readyImagePurposeV1,readyImageRecheckDueV1,readyImageRetainBytesV1,readyImageShortV1,readyImageWritersV1}from"./🖼️image-guard/🟦️.ts";',
 'import{READY_IMAGE_POLICY_V1,readyImageFactsV1,readyImageGuardV1,readyImagePartitionOfLinesV1,readyImagePurposeV1,readyImageRecheckDueV1,readyImageReliefV1,readyImageRetainBytesV1,readyImageShortV1,readyImageWritersV1,readyNoSpaceEvidenceV1,type ReadyImageShortV1}from"./🖼️image-guard/🟦️.ts";'),
('recover?(request:Readonly<{stage:string;short:ReadyDiskShortV1;signal:AbortSignal;report(line:string):void}>)',
 'low?(stage:string):ReadyDiskShortV1|null;recover?(request:Readonly<{stage:string;short:ReadyDiskShortV1;relief?:boolean;signal:AbortSignal;report(line:string):void}>)'),
('containment?:typeof READY_CONTAINMENT_POLICY_V1;speed?:number}>;',
 'containment?:typeof READY_CONTAINMENT_POLICY_V1;speed?:number;collectImage?:(image:ReadyImageShortV1,signal:AbortSignal)=>Promise<void>}>;'),
(' const unfit=new Map<string,ReadyDiskUnfitV1>(),',' const reliefAt=new Map<string,number>(),unfit=new Map<string,ReadyDiskUnfitV1>(),'),
('    if(short!==null&&short.resource==="space"&&ports.recover!==undefined&&!unfit.has(diskStage)){\n',
 '''    if(short===null&&ports.low!==undefined&&ports.recover!==undefined&&ports.inflight?.()!==diskStage&&ports.now()-(reliefAt.get(diskStage)??-Infinity)>=READY_IMAGE_POLICY_V1.reliefRetryMilliseconds){
     const low=ports.low(diskStage);
     if(low!==null){
      reliefAt.set(diskStage,ports.now());leave();
      ports.say(`image ${low.path} has ${low.available} MB free of ${low.total??0} MB, below its low-water mark of ${low.needed} MB: collecting inside it before stage ${diskStage} starts / Image ${low.path} hat ${low.available} MB frei von ${low.total??0} MB, unter der Untergrenze von ${low.needed} MB: vor Stufe ${diskStage} wird darin gesammelt`);
      status.update({...base,state:"waiting",ready:false,stage,stageIndex:decision.stageIndex+1,refusal:readyDiskCollectingV1(diskStage,low),tail:[],nextAttemptMs:null});
      let relieving=true;
      const reliefBeat=setInterval(()=>{if(relieving&&!signal.aborted)try{ports.status.heartbeat?.();}catch{}},P.statusHeartbeatMilliseconds);reliefBeat.unref?.();
      try{await ports.recover({stage:diskStage,short:low,relief:true,signal,report:line=>{if(!relieving||signal.aborted)return;ports.say(line);const progress=formatLogLineV1(line,P.tailCharacters);if(progress!=="")try{status.update({tail:[progress]});}catch{}}});}
      catch(error){if(signal.aborted)break;ports.say(`collecting inside the image of stage ${diskStage} failed (${String((error as Error)?.message??error).slice(0,200)}) / Sammeln im Image der Stufe ${diskStage} fehlgeschlagen`);}
      finally{relieving=false;clearInterval(reliefBeat);}
      if(signal.aborted)break;
      short=ports.disk?.(diskStage)??null;
     }
    }
    if(short!==null&&short.resource==="space"&&ports.recover!==undefined&&!unfit.has(diskStage)){
'''),
('diskEvery=Math.max(1,Math.round(P.diskGuard.checkMilliseconds/P.pollMilliseconds));',
 'diskEvery=Math.max(1,Math.round(P.diskGuard.checkMilliseconds/P.pollMilliseconds)),imageEvery=Math.max(1,Math.round(READY_IMAGE_POLICY_V1.checkMilliseconds/P.pollMilliseconds));'),
('let settling:Promise<void>|undefined,diskHit:ReadyDiskGuardHitV1|undefined;',
 'let settling:Promise<void>|undefined,diskHit:ReadyDiskGuardHitV1|undefined,collecting:Promise<void>|undefined,collectedAt=0;'),
('hit=readyDiskGuardV1(path,(input.facts??readyFilesystemFactsV1)(path))??readyImageGuardV1(readyImageFactsV1(input.root,inflight.stage,input.facts??readyFilesystemFactsV1));if(hit!==null&&kill("cancel")){diskHit=hit;say(readyDiskGuardLineV1(hit));}}',
 '''hit=readyDiskGuardV1(path,(input.facts??readyFilesystemFactsV1)(path));if(hit!==null&&kill("cancel")){diskHit=hit;say(readyDiskGuardLineV1(hit));}}
   if(!killed&&tick%imageEvery===0){
    const images=readyImageFactsV1(input.root,inflight.stage,input.facts??readyFilesystemFactsV1),hit=readyImageGuardV1(images);
    if(hit!==null){if(kill("cancel")){diskHit=hit;say(readyDiskGuardLineV1(hit));}}
    else if(input.collectImage!==undefined&&collecting===undefined&&Date.now()-collectedAt>=READY_IMAGE_POLICY_V1.reliefRetryMilliseconds){
     const low=readyImageReliefV1(images);
     if(low!==null){collectedAt=Date.now();say(`image ${low.path} has ${low.available} MB free of ${low.total} MB, below its low-water mark of ${low.needed} MB: collecting inside it while the stage runs / Image ${low.path} hat ${low.available} MB frei von ${low.total} MB, unter der Untergrenze von ${low.needed} MB: während die Stufe läuft wird darin gesammelt`);collecting=input.collectImage(low,input.signal).catch(()=>{}).finally(()=>{collecting=undefined;});}
    }
   }'''),
('const note=signal!==null?[exitSignalLineV1(signal)]:why===undefined?[]:[why],evidence=toolFailureEvidenceV1(inflight.stage,inflight.log);return{tail:[...failureLinesV1(inflight.log,P.tailLines-note.length,P.tailCharacters,evidence),...failedTaskTailLinesV1(inflight.log,P.failedTaskSummaryLines),...note],failure:[provenance(),...failureLinesV1(inflight.log,P.mirrorLines-1-note.length,P.mirrorLineCharacters,evidence,true),...failedTaskTailLinesV1(inflight.log),...note]};',
 'const note=signal!==null?[exitSignalLineV1(signal)]:why===undefined?[]:[why],evidence=toolFailureEvidenceV1(inflight.stage,inflight.log),space=readyNoSpaceEvidenceV1(inflight.log,P.noSpacePatterns);return{tail:[...failureLinesV1(inflight.log,P.tailLines-note.length,P.tailCharacters,evidence),...failedTaskTailLinesV1(inflight.log,P.failedTaskSummaryLines),...space,...note],failure:[provenance(),...failureLinesV1(inflight.log,P.mirrorLines-1-note.length,P.mirrorLineCharacters,evidence,true),...failedTaskTailLinesV1(inflight.log),...space,...note]};'),
('   freeInodes:()=>{const inodes=',
 '   low:stage=>readyImageReliefV1(readyImageFactsV1(input.root,stage,input.facts??readyFilesystemFactsV1)),\n   freeInodes:()=>{const inodes='),
('await reclaimUnitStorageV1(request.short.path,readyImageRetainBytesV1({freeBytes:request.short.available*1048576,totalBytes:(request.short.total??0)*1048576}),{signal:request.signal});',
 'await reclaimUnitStorageV1(request.short.path,Math.max(readyImageRetainBytesV1({freeBytes:request.short.available*1048576,totalBytes:(request.short.total??0)*1048576}),(request.short.needed-request.short.available)*1048576),{signal:request.signal});'),
('    const outcome=await recoverNeedV1({facts:reading,needBytes:request.short.needed*1048576,writers,',
 '    if(request.relief===true)return{fits:true,consumers:[]};\n    const outcome=await recoverNeedV1({facts:reading,needBytes:request.short.needed*1048576,writers,'),
('  run:(stage,options)=>runReadyStageV1({completion,root:input.root,environment:input.environment,stage,',
 '  run:(stage,options)=>runReadyStageV1({completion,root:input.root,environment:input.environment,stage,collectImage:async(image,signal)=>{try{const{reclaimUnitStorageV1}=await import("../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts");await reclaimUnitStorageV1(image.path,Math.max(readyImageRetainBytesV1({freeBytes:image.available*1048576,totalBytes:image.total*1048576}),(image.needed-image.available)*1048576),{signal});}catch{}},'),
])
print("landed")
