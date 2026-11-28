#!/usr/bin/env python3
import json,os,sys
ROOT="/Users/ueli/Documents/semio"
READY=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready"
TARGET=sys.argv[1] if len(sys.argv)>1 else READY
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for old,new in pairs:
        n=s.count(old)
        if n!=1:
            raise SystemExit(f"anchor count {n} != 1: {old[:90]!r}")
        s=s.replace(old,new)
    tmp=path+".h34.tmp"
    open(tmp,"w",encoding="utf8").write(s)
    os.replace(tmp,path)
TS=TARGET+"/🟦️.ts"
HELPERS='''/** 🖼️ The purpose and the folder a failed write names, read from the tail of the failed stage: what the refusal of a full image says about the image. */
export function readyImageFullFieldsV1(tail:readonly string[]):Readonly<{purpose:Readonly<{en:string;de:string}>;path:string}>{
 const partition=readyImagePartitionOfLinesV1(tail,P.noSpacePatterns)??"?";
 return Object.freeze({purpose:readyImagePurposeV1(partition),path:[...READY_IMAGE_POLICY_V1.directory,partition,READY_IMAGE_POLICY_V1.mount].join("/")});
}
/** 🖼️ The tail line that names the full image of a failed stage in both languages, in the words the classifier reads (`No space left on device` below `.cache/b/<partition>/m`), so that the failure stays an image failure when it is read again from the stored tail. */
export function readyImageFullLineV1(partition:string):string{
 const purpose=readyImagePurposeV1(partition),path=["…",...READY_IMAGE_POLICY_V1.directory,partition,READY_IMAGE_POLICY_V1.mount].join("/");
 return `${path}: No space left on device (the ${purpose.en} cache image is full) / Kein Platz mehr auf dem Gerät (das Cache-Image ${purpose.de} ist voll)`;
}
/** 🖼️ The refusal of a stage whose cache image is short after the collection inside it: the purpose, the image, what is free of its size, what is needed, the largest consumers with the command that frees each and how soon it is looked at again; English and German. */
export function readyImageShortRefusalV1(stage:string,short:ReadyDiskShortV1,consumers:readonly ReadyDiskConsumerV1[]=[]):ReadyRefusalV1{
 return readyRefusalV1("image-short",{stage,purpose:readyImagePurposeV1(short.partition??"?"),path:short.path,needed:short.needed,available:short.available,total:short.total??0,host:short.host??"?",hint:short.limitedByHost===true?READY_IMAGE_POLICY_V1.hostLimited:{en:"",de:""},minutes:Math.round(READY_IMAGE_POLICY_V1.recheckMilliseconds/60000),growth:READY_IMAGE_POLICY_V1.recheckGrowthMegabytes,consumers:consumers.map(row=>`${row.root} (${Math.round(row.bytes/1048576)} MB): \\`${row.command}\\``).join("; ")||"-"});
}
'''
pairs=[
('import{createPartitionCensusMirrorV1}from"./🧱️partition-census/🟦️.ts";',
 'import{createPartitionCensusMirrorV1}from"./🧱️partition-census/🟦️.ts";\nimport{READY_IMAGE_POLICY_V1,readyImageFactsV1,readyImageGuardV1,readyImagePartitionOfLinesV1,readyImagePurposeV1,readyImageRecheckDueV1,readyImageRetainBytesV1,readyImageShortV1,readyImageWritersV1}from"./🖼️image-guard/🟦️.ts";'),
('|"disk-guard"|"server";','|"disk-guard"|"image-full"|"image-short"|"server";'),
('export type ReadyDiskShortV1=Readonly<{path:string;resource:"space"|"inodes";needed:number;available:number}>;','export type ReadyDiskShortV1=Readonly<{path:string;resource:"space"|"inodes";needed:number;available:number;partition?:string;total?:number;host?:number|null;limitedByHost?:boolean}>;'),
('/** 🔁️ Whether an unfit worker is looked at again:',HELPERS+'/** 🔁️ Whether an unfit worker is looked at again:'),
('seconds:Math.round(P.sourceTransientDebounceMilliseconds/1000)}):null;','seconds:Math.round(P.sourceTransientDebounceMilliseconds/1000),...state.failure.code==="image-full"?readyImageFullFieldsV1(state.failure.tail):{}}):null;'),
('else if(unfit.has(diskStage)&&readyUnfitRecheckDueV1(unfit.get(diskStage)!,short.available,ports.now())){','else if(unfit.has(diskStage)&&(short.partition===undefined?readyUnfitRecheckDueV1(unfit.get(diskStage)!,short.available,ports.now()):readyImageRecheckDueV1(unfit.get(diskStage)!,short.available,ports.now()))){'),
('const marked=unfit.get(diskStage),refusal=marked!==undefined?readyDiskUnfitRefusalV1(diskStage,short,marked.consumers):readyDiskRefusalV1(diskStage,short,remaining);','const marked=unfit.get(diskStage),refusal=short.partition!==undefined?readyImageShortRefusalV1(diskStage,short,marked?.consumers??remaining??[]):marked!==undefined?readyDiskUnfitRefusalV1(diskStage,short,marked.consumers):readyDiskRefusalV1(diskStage,short,remaining);'),
('const failedTail=wrong!==null?[wrong.en,wrong.de]:readyTailWithDiskCauseV1(','const imagePartition=wrong!==null?null:readyImagePartitionOfLinesV1(result.tail,P.noSpacePatterns),baseTail=wrong!==null?[wrong.en,wrong.de]:readyTailWithDiskCauseV1('),
('readyNoSpaceV1(result.tail)?ports.native?.()??null:null);\n    const attempts=','readyNoSpaceV1(result.tail)?ports.native?.()??null:null),failedTail=imagePartition===null?baseTail:[...baseTail,readyImageFullLineV1(imagePartition)];\n    const attempts='),
(':readyStoreFullV1(failedTail)!==null?"store-full":',':imagePartition!==null?"image-full":readyStoreFullV1(failedTail)!==null?"store-full":'),
('code==="memory-cancelled"||code==="disk-guard"?1:attempts','code==="memory-cancelled"||code==="disk-guard"||code==="image-full"?1:attempts'),
('disk:stage=>readyDiskShortV1({stage,paths:[machinePathV1(input.root,".cache"),tmpdir()],facts:input.facts??readyFilesystemFactsV1,runs:recentRuns(),derivedInodes:readyDerivedInodesV1(stage,input.root,readyScopesOfStageV1(plan,stage))}),','disk:stage=>readyDiskShortV1({stage,paths:[machinePathV1(input.root,".cache"),tmpdir()],facts:input.facts??readyFilesystemFactsV1,runs:recentRuns(),derivedInodes:readyDerivedInodesV1(stage,input.root,readyScopesOfStageV1(plan,stage))})??readyImageShortV1(readyImageFactsV1(input.root,stage,input.facts??readyFilesystemFactsV1)),'),
('hit=readyDiskGuardV1(path,(input.facts??readyFilesystemFactsV1)(path));if(hit!==null&&kill("cancel"))','hit=readyDiskGuardV1(path,(input.facts??readyFilesystemFactsV1)(path))??readyImageGuardV1(readyImageFactsV1(input.root,inflight.stage,input.facts??readyFilesystemFactsV1));if(hit!==null&&kill("cancel"))'),
('if(process.platform!=="darwin"||request.stage!=="dependencies")return{grown:0,unfit:null};','if(process.platform!=="darwin"||!READY_IMAGE_POLICY_V1.healStages.includes(request.stage))return{grown:0,unfit:null};'),
('unfit:unfit?.state==="skipped"?{path:','unfit:request.stage==="dependencies"&&unfit?.state==="skipped"?{path:'),
('    const writers=realDiskWritersV1({workspace:input.root,home:input.home??homedir(),tmp:tmpdir(),uid:process.getuid?.()??0,environment:input.environment,language:"both",report:request.report,free:()=>reading()?.freeBytes,...(request.short.resource==="space"?{pressure:{path:request.short.path,requiredFreeBytes:request.short.needed*1048576}}:{})});\n    const outcome=await recoverNeedV1(',
 '    const image=request.short.partition!==undefined,all=realDiskWritersV1({workspace:input.root,home:input.home??homedir(),tmp:tmpdir(),uid:process.getuid?.()??0,environment:input.environment,language:"both",report:request.report,free:()=>reading()?.freeBytes,...(!image&&request.short.resource==="space"?{pressure:{path:request.short.path,requiredFreeBytes:request.short.needed*1048576}}:{})}),writers=image&&request.short.limitedByHost!==true?readyImageWritersV1(all,request.short.path,sep):all;\n    if(image&&request.short.partition==="r")try{const{reclaimUnitStorageV1}=await import("../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🟦️.ts");await reclaimUnitStorageV1(request.short.path,readyImageRetainBytesV1({freeBytes:request.short.available*1048576,totalBytes:(request.short.total??0)*1048576}),{signal:request.signal});}catch(error){if(request.signal.aborted)throw error;request.report(`collecting inside the image failed (${String((error as Error)?.message??error).slice(0,200)}) / Sammeln im Image fehlgeschlagen`);}\n    const outcome=await recoverNeedV1('),
]
edit(TS,pairs)
SCHEMA=TARGET+"/🧬️schema/🔣️.json"
EN_FULL="Stage {stage} stopped because the {purpose} cache image ({path}) ran out of space (No space left on device). Before the stage starts again the worker collects inside that image in place (compiler temporaries, incremental sessions and stale build units that no running build holds, oldest first; no exclusive lease, no detaching) and checks how much is free; retrying with a short back-off."
DE_FULL="Stufe {stage} wurde angehalten, weil das Cache-Image {purpose} ({path}) keinen Platz mehr hatte (No space left on device). Bevor die Stufe neu startet, sammelt der Worker im Image an Ort und Stelle (Compiler-Temporärdateien, inkrementelle Sitzungen und veraltete Build-Einheiten, die kein laufender Build hält, älteste zuerst; ohne exklusive Sperre, ohne Aushängen) und prüft den freien Platz; neuer Versuch nach kurzer Wartezeit."
EN_SHORT="Stage {stage} needs {needed} MB free in the {purpose} cache image ({path}) but only {available} of {total} MB are free after collecting inside the image in place (the host volume has {host} MB free); nothing that a running build holds was touched. {hint} The stage starts as soon as the image has room; it is looked at again every {minutes} min or when {growth} MB more are free. Largest consumers and their commands: {consumers}."
DE_SHORT="Stufe {stage} braucht {needed} MB frei im Cache-Image {purpose} ({path}), aber nach dem Sammeln im Image an Ort und Stelle sind nur {available} von {total} MB frei (das Wirtsvolumen hat {host} MB frei); nichts, was ein laufender Build hält, wurde angetastet. {hint} Die Stufe startet, sobald das Image Platz hat; es wird alle {minutes} Min. oder bei {growth} MB mehr Platz erneut geprüft. Größte Verbraucher und ihre Befehle: {consumers}."
def j(x): return json.dumps(x,ensure_ascii=False)
edit(SCHEMA,[
 ('"disk-guard", "spawn-failed", "loop-failed"],','"disk-guard", "image-full", "spawn-failed", "loop-failed"],'),
 ('"memory-cancelled", "disk-guard", "server"]','"memory-cancelled", "disk-guard", "image-full", "image-short", "server"]'),
])
s=open(SCHEMA,encoding="utf8").read()
import re
def insert_after_key(s,lang_first_key,addition):
    return s
lines=s.split("\n")
out=[];seen=0
for line in lines:
    out.append(line)
    m=re.match(r'^(\s*)"disk-guard": "',line)
    if m:
        seen+=1
        ind=m.group(1)
        en=seen==1
        txt=(EN_FULL,EN_SHORT) if en else (DE_FULL,DE_SHORT)
        out.append(f'{ind}"image-full": {j(txt[0])},')
        out.append(f'{ind}"image-short": {j(txt[1])},')
if seen!=2: raise SystemExit("disk-guard text rows %d"%seen)
tmp=SCHEMA+".h34.tmp"
open(tmp,"w",encoding="utf8").write("\n".join(out))
os.replace(tmp,SCHEMA)
json.load(open(SCHEMA,encoding="utf8"))
print("ok")
