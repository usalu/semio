#!/usr/bin/env python3
"""H73 patch: anchored edits of existing files. `patch.py check|apply UNIT_CACHE_DIR LAUNCHER_FILE`. Every anchor must match exactly once;
`apply` re-reads each file, writes once (temp + rename). Schema edits are `schema_edits.py` (same roots)."""
import os,sys,tempfile
mode,UC,LAUNCH=sys.argv[1],sys.argv[2],sys.argv[3]
edits=[]
def edit(path,old,new):edits.append((path,old,new))
FLEET=UC+"/🛰️fleet/🟦️.ts"
edit(FLEET,'import {installFleetUnitV1} from "./📥️adopt/🟦️.ts";','import {installFleetUnitV1} from "./📥️adopt/🟦️.ts";\nimport {recordSeedSlotV1,seededClientV1,seedDirectoryV1} from "./🌱️seed/🟦️.ts";')
edit(FLEET,'import {closeSync,mkdirSync,openSync,readFileSync,readSync,realpathSync,writeFileSync} from "node:fs";','import {closeSync,existsSync,mkdirSync,openSync,readFileSync,readSync,realpathSync,writeFileSync} from "node:fs";')
edit(FLEET,'''  const known=clients.get(settings.identity+"|"+root);if(known)return known;
  const workspace=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||undefined,identityRoot=workspace??fleetCacheRootV1(environment),client=createFleetClientV1(settings,identityRoot===undefined?{}:{fetch:remoteCacheFetchV1(environment,{root:identityRoot})});
  clients.set(settings.identity+"|"+root,client);return client;''','''  const laneRoot=environment.SEMIO_UNIT_CACHE_LANE_ROOT||undefined,name=settings.identity+"|"+root+"|"+(laneRoot??""),known=clients.get(name);if(known)return known;
  const workspace=environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT||undefined,identityRoot=workspace??fleetCacheRootV1(environment),plain=createFleetClientV1(settings,identityRoot===undefined?{}:{fetch:remoteCacheFetchV1(environment,{root:identityRoot})});
  const client=laneRoot!==undefined&&existsSync(join(seedDirectoryV1(laneRoot),fleet.seed.acDirectory))?seededClientV1(plain,laneRoot):plain;
  clients.set(name,client);return client;''')
edit(FLEET,''' try{
  const settings=readFleetSettingsV1(input.root,input.environment);if(!settings)return undefined;
  const workspace=input.environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT''',''' try{
  recordSeedSlotV1(input.laneRoot,input.basics.slot);
  const settings=readFleetSettingsV1(input.root,input.environment);if(!settings)return undefined;
  const workspace=input.environment.SEMIO_UNIT_CACHE_WORKSPACE_ROOT''')
WRAP=UC+"/📜️script.ts"
edit(WRAP,'import {fleetClientV1} from "./🛰️fleet/🟦️.ts";','import {fleetClientV1} from "./🛰️fleet/🟦️.ts";\nimport {recordSeedSlotV1} from "./🛰️fleet/🌱️seed/🟦️.ts";')
edit(WRAP,'manifestClient=manifestId===undefined?undefined:fleetClientV1(settings.root,environment);','manifestClient=manifestId===undefined?undefined:fleetClientV1(settings.root,environment);\n if(manifestId!==undefined)recordSeedSlotV1(settings.laneRoot,slotEarly!);')
edit(LAUNCH,'import {acquirePooledLaneV1,','import {withSeededLaneV1} from "../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🌱️seed/🛫️lane/🟦️.ts";\nimport {acquirePooledLaneV1,')
edit(LAUNCH,'''     try{return await (await import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts")).retryAfterHostWardenCancelV1((_attempt,startedMs)=>{launchedMs=startedMs;return admitted(compilerArgs,compilerEnvironment,selected,control,readmit,generated&&!generatedPolicy?()=>copyFileSync(generated,env.SEMIO_TYPEGEN_OUT!):undefined,generated);},{report:spaceReport,signal:control.signal});}finally{if(generated)rmSync(generated,{force:true});}''','''     const attempt=async()=>(await import("../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🔁️loop/🟦️.ts")).retryAfterHostWardenCancelV1((_attempt,startedMs)=>{launchedMs=startedMs;return admitted(compilerArgs,compilerEnvironment,selected,control,readmit,generated&&!generatedPolicy?()=>copyFileSync(generated,env.SEMIO_TYPEGEN_OUT!):undefined,generated);},{report:spaceReport,signal:control.signal});
     try{return await ("lane" in selected&&selected.lane!==undefined?withSeededLaneV1({cacheRoot:unitCacheRootV1(store.root),laneRoot:selected.lane.root,lane:selected.lane.id,graph,environment,language:spaceLanguage,report:spaceReport,signal:control.signal,cold:selected.cold===true},attempt):attempt());}finally{if(generated)rmSync(generated,{force:true});}''')
edit(LAUNCH,'export type RepositoryCargoAgentLaneV1=Readonly<{target:string;build:string;lane:Readonly<{id:string;root:string}>}>;','export type RepositoryCargoAgentLaneV1=Readonly<{target:string;build:string;lane:Readonly<{id:string;root:string}>;cold?:boolean}>;')
edit(LAUNCH,'const result=await body(Object.freeze({target:lane.paths.target,build:lane.paths.build,lane:Object.freeze({id:lane.paths.id,root:lane.paths.base})}));','const result=await body(Object.freeze({target:lane.paths.target,build:lane.paths.build,lane:Object.freeze({id:lane.paths.id,root:lane.paths.base}),cold:lane.choice!=="warm"}));')
edit(LAUNCH,'body:(store:Readonly<{target:string;build:string;temporary:string;lane:Readonly<{id:string;root:string}>}>)=>Promise<T>):Promise<T>{','body:(store:Readonly<{target:string;build:string;temporary:string;lane:Readonly<{id:string;root:string}>;cold?:boolean}>)=>Promise<T>):Promise<T>{')
edit(LAUNCH,'selected:RepositoryCargoIsolatedStoreV1|Readonly<{target:string;build:string;temporary:string;lane?:Readonly<{id:string;root:string}>}>)','selected:RepositoryCargoIsolatedStoreV1|Readonly<{target:string;build:string;temporary:string;lane?:Readonly<{id:string;root:string}>;cold?:boolean}>)')
bad=0;staged={}
for path,old,new in edits:
    text=staged.get(path)
    if text is None:text=open(path,encoding="utf8").read()
    n=text.count(old)
    if n!=1:print("ANCHOR",n,path.split("/")[-1],old[:70].replace("\n"," "));bad+=1;continue
    staged[path]=text.replace(old,new)
if bad:raise SystemExit(1)
if mode=="check":print("anchors ok",len(edits));raise SystemExit(0)
for path,text in staged.items():
    fd,tmp=tempfile.mkstemp(dir=os.path.dirname(path));os.write(fd,text.encode("utf8"));os.close(fd);os.rename(tmp,path)
print("applied",len(staged),"files")
