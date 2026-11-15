import sys
root=sys.argv[1]
def edit(path,pairs):
    s=open(root+"/"+path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:70],s.count(a))
        s=s.replace(a,b)
    open(root+"/"+path,"w").write(s)
edit("🔁️replica/🟦️.ts",[
 ('import {createSidecarsV1,','import {packLocalV1,packPartsV1,readPackAuthorisationV1,rescueLocalPacksV1} from "./🧷️packgate/🟦️.ts";\nimport {createSidecarsV1,'),
 ('export const guarded=(ignore:FleetIgnoreV1):FleetIgnoreV1=>({digest:ignore.digest,ignores:(path,isDirectory)=>sealed(path)||ignore.ignores(path,isDirectory),child:(path,isDirectory)=>sealed(path)||(ignore.child??ignore.ignores)(path,isDirectory)});',
  'export const guarded=(ignore:FleetIgnoreV1,local?:(path:string)=>boolean):FleetIgnoreV1=>({digest:ignore.digest,ignores:(path,isDirectory)=>sealed(path)||ignore.ignores(path,isDirectory),child:(path,isDirectory)=>sealed(path)||(ignore.child??ignore.ignores)(path,isDirectory),...local===undefined?{}:{local}});\n\nconst fleetPackMemo=new WeakMap<ReplicaV1,Readonly<{at:number;has:boolean}>>();\n/** 🧷️ Tells whether the fleet register holds a pack of `.git/objects/pack` at all (once true it stays true; a negative answer is kept for five seconds): until the first machine has seeded its packs every pack is shared. */\nfunction fleetHasPacks(replica:ReplicaV1):boolean{\n const held=fleetPackMemo.get(replica),now=Date.now();\n if(held?.has===true)return true;\n if(held!==undefined&&now-held.at<5000)return false;\n let has=false;\n for(const id of replica.manifest.ids())if(replica.manifest.operationAt(id)==="put"){const name=replica.book.name(id);if(name.startsWith(".git/objects/pack/")&&name.endsWith(".pack")){has=true;break;}}\n fleetPackMemo.set(replica,{at:now,has});\n return has;\n}\n\nconst fleetPack=(replica:ReplicaV1,pack:string):boolean=>replica.manifest.idOf(pack)!==undefined;\n\n/** 🧷️ The pack gate of a replica: a pack the fleet does not hold, that the consolidator did not authorise, stays on its machine (never scanned, never published, its deletion never published); see `🧷️packgate`. */\nconst packLocalOf=(replica:ReplicaV1):((path:string)=>boolean)=>path=>packLocalV1(path,pack=>fleetPack(replica,pack),()=>fleetHasPacks(replica),replica.settings.worktree);'),
 (' replica.ignore=guarded(compileIgnoreV1(text));replica.ignoreHash=hash;',' replica.ignore=guarded(compileIgnoreV1(text),packLocalOf(replica));replica.ignoreHash=hash;'),
 ('    replica.ignore=guarded(compileIgnoreV1(text));replica.ignoreHash=digest(text);','    replica.ignore=guarded(compileIgnoreV1(text),packLocalOf(replica));replica.ignoreHash=digest(text);'),
 ('  if(corrupt!==undefined){\n   replica.say({event:"state-rebuilt"','  replica.ignore=guarded(compileIgnoreV1(text),packLocalOf(replica));\n  if(corrupt!==undefined){\n   replica.say({event:"state-rebuilt"'),
 (' deletions.sort();\n',' deletions.sort();\n {\n  const retireAllowed=readPackAuthorisationV1(worktree).retire;\n  for(let index=deletions.length-1;index>=0;index--){\n   const path=deletions[index]!;\n   if(!path.startsWith(objectsPrefix))continue;\n   const parts=packPartsV1(path);\n   if(parts===undefined||!retireAllowed.has(parts.stem))deletions.splice(index,1);\n  }\n }\n'),
 (" const sound={at:0,healAt:settings.now()+(options.gitHealMilliseconds??600_000),broken:null as GitBrokenV1|null};"," const sound={at:0,healAt:settings.now()+(options.gitHealMilliseconds??600_000),broken:null as GitBrokenV1|null};\n let rescueAt=0;"),
 ("  sound.at=now+(options.gitCheckMilliseconds??30_000);\n","  sound.at=now+(options.gitCheckMilliseconds??30_000);\n  if(now>=rescueAt){\n   rescueAt=now+(options.rescueMilliseconds??60_000);\n   const rescued=await rescueLocalPacksV1(settings.worktree,pack=>fleetPack(replica,pack),{now,signal,...options.rescueAgeMilliseconds===undefined?{}:{minAgeMilliseconds:options.rescueAgeMilliseconds}});\n   if(rescued.objects>0||rescued.retired>0){replica.say({event:\"repaired\",stage:\"git\",reason:\"local-packs\",count:rescued.objects});if(rescued.objects>0)full=true;}\n  }\n"),
 ("export type ReplicaOptionsV1=Readonly<{gitCheckMilliseconds?:number;","export type ReplicaOptionsV1=Readonly<{rescueMilliseconds?:number;rescueAgeMilliseconds?:number;gitCheckMilliseconds?:number;"),
])
edit("🔁️replica/🧱️consolidate/🟦️.ts",[
 ('import {foldStoreV1,type ReplicaSinkV1} from "../🟦️.ts";','import {authorisePacksV1} from "../🧷️packgate/🟦️.ts";\nimport {foldStoreV1,type ReplicaSinkV1} from "../🟦️.ts";'),
 ('  for(const extension of ["pack","rev","idx"])await rename(','  await authorisePacksV1(options.worktree,{publish:[stem]},clock());\n  await writeFile(join(directory,`${stem}.keep`),"semio-consolidated\\n");\n  for(const extension of ["pack","rev","idx"])await rename('),
 ('  for(const name of await readdir(directory).catch(()=>[] as string[]))if(name==="multi-pack-index"','  await authorisePacksV1(options.worktree,{retire:[...retire,"multi-pack-index"]},clock());\n  for(const name of await readdir(directory).catch(()=>[] as string[]))if(name==="multi-pack-index"'),
])

edit("🚧️boundary/🟦️.ts",[
 ('export type FleetIgnoreV1=Readonly<{digest:string;ignores(path:string,isDirectory:boolean):boolean;child?(path:string,isDirectory:boolean):boolean}>;','export type FleetIgnoreV1=Readonly<{digest:string;ignores(path:string,isDirectory:boolean):boolean;child?(path:string,isDirectory:boolean):boolean;local?(path:string):boolean}>;'),
 (' if(path===".git"||path.startsWith(".git/"))return gitSharedV1(path)&&refusePathV1(path)===undefined;',' if(path===".git"||path.startsWith(".git/"))return gitSharedV1(path)&&refusePathV1(path)===undefined&&ignore.local?.(path)!==true;'),
 ('   if(rootGit&&gitPrivateV1(relative))continue;','   if(rootGit&&(gitPrivateV1(relative)||ignore.local?.(relative)===true))continue;'),
])

edit("🔁️replica/🟦️.ts",[
 ('  try{await rmdir(current);}catch{return;}\n  verifiedDirectories(replica).clear();','  if(structural.has(current.slice(root.length+1).split(sep).join("/")))return;\n  try{await rmdir(current);}catch{return;}\n  verifiedDirectories(replica).clear();'),
 ('async function prune(replica:ReplicaV1,directory:string):Promise<void>{','/** 🧱️ The directories git needs to recognise a repository; applying a deletion never removes them even when they are empty (a `git gc` that packs the refs empties `refs/heads`). */\nconst structural=new Set([".git",".git/refs",".git/refs/heads",".git/refs/tags",".git/objects",".git/objects/pack",".git/objects/info"]);\n\nasync function prune(replica:ReplicaV1,directory:string):Promise<void>{'),
])

edit("🛡️resilience/🟦️.ts",[
 ('"maintain","compact"] as const;','"maintain","compact","git","drift","hash","journal","persist","settle"] as const;'),
])
edit("🔁️replica/🛟️supervisor/🟦️.ts",[
 ('"maintain","compact"] as const;','"maintain","compact","git","drift","hash","journal","persist","settle"] as const;'),
])
edit("🔁️replica/🟦️.ts",[
 ('  const walk=walked??await walkLocalV1(replica,signal);','  activityV1("walk");\n  const walk=walked??await walkLocalV1(replica,signal);\n  activityV1("publish");'),
 ('  await pool(slice,concurrency,async (job,index)=>{prepared[index]=await prepare(replica,job,signal);},signal);','  activityV1("hash");\n  await pool(slice,concurrency,async (job,index)=>{prepared[index]=await prepare(replica,job,signal);},signal);\n  activityV1("publish");'),
 ('  await persist(replica,false);\n  if(++rounds%8===0)settleHeap();','  activityV1("persist");\n  await persist(replica,false);\n  if(++rounds%8===0){activityV1("settle");settleHeap();}\n  activityV1("publish");'),
 ('   const rescued=await rescueLocalPacksV1(','   activityV1("git");\n   const rescued=await rescueLocalPacksV1('),
 ('  let reason=await gitBrokenV1(settings.worktree,signal);','  activityV1("git");\n  let reason=await gitBrokenV1(settings.worktree,signal);'),
 ('  const now=settings.now();\n  if(now<drift.at)return [];','  const now=settings.now();\n  if(now<drift.at)return [];\n  activityV1("drift");'),
])
