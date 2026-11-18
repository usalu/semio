import re,sys
def sub(text,old,new,count=1):
    assert text.count(old)==count,(text.count(old),old[:80])
    return text.replace(old,new)
t=open('merged-f1.ts',encoding='utf8').read()
t=sub(t,'statSync,statfsSync,writeFileSync','statSync,writeFileSync')
t=sub(t,'import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";','import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";\nimport{readFreeSpaceV1,type FreeSpaceOptionsV1,type FreeSpaceUnknownReasonV1}from"../../../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts";')
t=sub(t,'|"disk-short"|"disk-collecting"','|"disk-short"|"disk-unmeasured"|"disk-collecting"') if '|"disk-short"|"disk-collecting"' in t else sub(t,'|"disk-short"|','|"disk-short"|"disk-unmeasured"|')
old_type=t[t.index('/** 📦️ What a filesystem says it has free at a path'):t.index('/** 💽️ The first resource a stage lacks before it starts')]
new_type='''/** 📦️ What a filesystem says it has free at a path: bytes available to the user and free inodes, `null` for a fact the platform does not report (Windows reports no inodes, a network share may report nothing). `freeBytes` is `null` exactly when the volume could not be measured, and then `unknown` says why: an unknown measurement is never read as zero bytes free. */
export type ReadyFilesystemFactsV1=Readonly<{freeBytes:number|null;freeInodes:number|null;unknown?:Readonly<{reason:FreeSpaceUnknownReasonV1;detail:string}>}>;
/** 🕳️ A path whose free space could not be measured, with the reason and a short detail. */
export type ReadyDiskUnknownV1=Readonly<{path:string;reason:string;detail:string}>;
'''
t=t.replace(old_type,new_type)
a=t.index('/** 🔎️ The facts of the filesystem that holds `path`')
b=t.index('/** 💽️ Whether the paths a stage writes to hold what it needs')
new_facts='''/** 🔎️ The facts of the filesystem that holds `path` (its nearest existing ancestor when it is not there yet), from the one free-space question of the build system (`readFreeSpaceV1`: `statfs` on POSIX, `GetDiskFreeSpaceExW` on Windows, where the runtime's `statfs` answers a row of zeros). A volume that cannot be measured has `freeBytes` `null` and the reason; a filesystem that reports no inode table (NTFS, btrfs, some network shares) has `freeInodes` `null`, never 0. Disk quotas of a user are not visible to `statfs` (only project quotas of a directory are), so a quota shortage still shows as the write error that the stage classifier names.
 * @see ../../../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts
 * @see https://man7.org/linux/man-pages/man2/statfs.2.html */
export function readyFilesystemFactsV1(path:string,options?:Omit<FreeSpaceOptionsV1,"nearest">):ReadyFilesystemFactsV1{
 const facts=readFreeSpaceV1(path,{nearest:true,...options});
 return facts.known?Object.freeze({freeBytes:facts.availableBytes,freeInodes:facts.freeInodes}):Object.freeze({freeBytes:null,freeInodes:null,unknown:Object.freeze({reason:facts.reason,detail:facts.detail})});
}

'''
t=t[:a]+new_facts+t[b:]
old_loop='''  const facts=input.facts(path);if(facts===null)continue;
  if(facts.freeBytes!==null&&facts.freeBytes<need.megabytes*1048576)'''
new_loop='''  const facts=input.facts(path);if(facts===null)continue;
  const bytes=facts.freeBytes!==null&&Number.isFinite(facts.freeBytes)&&facts.freeBytes>=0?facts.freeBytes:null,inodes=facts.freeInodes!==null&&Number.isFinite(facts.freeInodes)&&facts.freeInodes>=0?facts.freeInodes:null;
  if(bytes!==null&&bytes<need.megabytes*1048576)'''
t=sub(t,old_loop,new_loop)
t=sub(t,'available:Math.floor(facts.freeBytes/1048576)});','available:Math.floor(bytes/1048576)});')
t=sub(t,'if(facts.freeInodes!==null&&facts.freeInodes<need.inodes)return Object.freeze({path,resource:"inodes" as const,needed:need.inodes,available:facts.freeInodes});','if(inodes!==null&&inodes<need.inodes)return Object.freeze({path,resource:"inodes" as const,needed:need.inodes,available:inodes});')
i=t.index('/** 🗣️ The refusal of a stage that lacks disk')
unk='''/** 🕳️ The paths a stage writes to whose free bytes could not be measured (the provider answered nothing, or `freeBytes` is `null`, not finite or negative): the stage starts anyway with a warning, because "unknown" is not "full"; the gate of the stage and the write error itself stay. A pure function of the facts provider. */
export function readyDiskUnknownV1(input:Readonly<{paths:readonly string[];facts(path:string):ReadyFilesystemFactsV1|null}>):readonly ReadyDiskUnknownV1[]{
 const rows:ReadyDiskUnknownV1[]=[];
 for(const path of input.paths){
  const facts=input.facts(path);
  if(facts===null)rows.push(Object.freeze({path,reason:"unreadable",detail:"-"}));
  else if(facts.freeBytes===null||!Number.isFinite(facts.freeBytes)||facts.freeBytes<0)rows.push(Object.freeze({path,reason:facts.unknown?.reason??"incoherent-row",detail:facts.unknown?.detail??String(facts.freeBytes)}));
 }
 return Object.freeze(rows);
}

/** 🗣️ The warning of a stage that starts although a path could not be measured: English and German. */
export function readyDiskUnmeasuredV1(stage:string,row:ReadyDiskUnknownV1):ReadyRefusalV1{
 return readyRefusalV1("disk-unmeasured",{stage,path:row.path,reason:row.reason,detail:row.detail});
}

'''
t=t[:i]+unk+t[i:]
t=sub(t,'disk?(stage:string):ReadyDiskShortV1|null;recover?','disk?(stage:string):ReadyDiskShortV1|null;diskUnknown?(stage:string):readonly ReadyDiskUnknownV1[];recover?')
t=sub(t,' const stalls=new Map<string,number>();',' const stalls=new Map<string,number>(),warnedUnmeasured=new Set<string>();')
old='''    let short=ports.inflight?.()===diskStage?null:ports.disk?.(diskStage)??null,remaining:readonly ReadyDiskConsumerV1[]|undefined;'''
new='''    let short=ports.inflight?.()===diskStage?null:ports.disk?.(diskStage)??null,remaining:readonly ReadyDiskConsumerV1[]|undefined;
    if(ports.inflight?.()!==diskStage)for(const row of ports.diskUnknown?.(diskStage)??[]){const key=`${diskStage}\\0${row.path}\\0${row.reason}`;if(!warnedUnmeasured.has(key)){warnedUnmeasured.add(key);const warning=readyDiskUnmeasuredV1(diskStage,row);ports.say(`${warning.en} / ${warning.de}`);}}'''
t=sub(t,old,new)
t=sub(t,'   disk:stage=>readyDiskShortV1({stage,paths:[machinePathV1(input.root,".cache"),tmpdir()],facts:input.facts??readyFilesystemFactsV1,runs:recentRuns()}),','   disk:stage=>readyDiskShortV1({stage,paths:[machinePathV1(input.root,".cache"),tmpdir()],facts:input.facts??readyFilesystemFactsV1,runs:recentRuns()}),\n   diskUnknown:()=>readyDiskUnknownV1({paths:[machinePathV1(input.root,".cache"),tmpdir()],facts:input.facts??readyFilesystemFactsV1}),')
open('stage/ready.ts','w',encoding='utf8').write(t)

s=open('merged-f2.json',encoding='utf8').read()
en='"disk-unmeasured": "Stage {stage}: the free space in {path} could not be measured ({reason}: {detail}); that is not the same as a full disk, so the stage starts anyway; the space gate of the stage and the write error itself still stop it.",\n          '
de='"disk-unmeasured": "Stufe {stage}: der freie Platz in {path} konnte nicht gemessen werden ({reason}: {detail}); das ist nicht dasselbe wie ein volles Laufwerk, daher startet die Stufe trotzdem; die Platzprüfung der Stufe und der Schreibfehler selbst halten sie weiterhin an.",\n          '
s=sub(s,'"disk-consumers": "Stage {stage}',en+'"disk-consumers": "Stage {stage}')
s=sub(s,'"disk-consumers": "Stufe {stage}',de+'"disk-consumers": "Stufe {stage}')
s=sub(s,'"disk-short", "disk-collecting"','"disk-short", "disk-unmeasured", "disk-collecting"')
open('stage/ready-schema.json','w',encoding='utf8').write(s)
