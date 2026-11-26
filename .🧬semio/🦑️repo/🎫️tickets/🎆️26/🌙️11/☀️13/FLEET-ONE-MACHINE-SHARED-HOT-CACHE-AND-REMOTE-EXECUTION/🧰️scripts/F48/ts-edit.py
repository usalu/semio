import os,sys
d=sys.argv[1]
def edit(path,pairs):
    s=open(path,encoding="utf8").read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:60],s.count(a))
        s=s.replace(a,b)
    tmp=path+".f48.tmp"
    open(tmp,"w",encoding="utf8").write(s)
    os.replace(tmp,path)
main=d+"/🟦️.ts"
edit(main,[
('export type FleetWorkerFactV1=Readonly<{origin:string;','/** 🏷️ The state of the operating system host name of one machine as its published address set says (see `readFleetHostNamesV1`): never an identity, the way out of `duplicate` and `named-as` is `repo fleet hostname apply` on that machine. */\nexport type FleetWorkerHostNameV1=Readonly<{state:"ok"|"duplicate"|"named-as"|"unknown";label:string|null;others:readonly string[]}>;\nexport type FleetWorkerFactV1=Readonly<{origin:string;'),
('mapping?:FleetWorkerMappingV1|null}>;\n/** 📜️ How the runs','mapping?:FleetWorkerMappingV1|null;hostName?:FleetWorkerHostNameV1|null}>;\n/** 📜️ How the runs'),
('say("worker-absent","info",worker.origin,{origin:worker.origin});\n','say("worker-absent","info",worker.origin,{origin:worker.origin});\n  const host=worker.hostName;\n  if(host!==undefined&&host!==null&&(host.state==="duplicate"||host.state==="named-as"))say(`worker-host-name-${host.state}`,"warn",worker.origin,{origin:worker.origin,label:host.label??"-",others:host.others.join(", ")});\n'),
('  const treeText=(row:Row):string=>','  const hostText=(row:Row):string=>{const host=row.hostName as FleetWorkerHostNameV1|null|undefined;return host===undefined||host===null?"":say(`workers.host.${host.state}`,{label:host.label??"-",others:host.others.join(", ")});};\n  const treeText=(row:Row):string=>'),
('recent:recentText(row),refusal:row.refusal===null','recent:recentText(row),host:hostText(row),refusal:row.refusal===null'),
])
edit(d+"/📥️readers/🟦️.ts",[
('/** 🩺️ Workers: every origin of the fleet store','''/** 🏷️ The host name state of every given origin from the address sets the machines published in the fleet store: `duplicate` when another origin published the same label (or the label with macOS's conflict suffix), `named-as` when the label is the semantic name of another origin, `unknown` without a set or a usable label. Read-only, one directory read; the same gate the agents run before they advertise.
 * @see ../../../../🤝️shared-worktree/🗺️topology/🧭️addresses/🟦️.ts */
export async function readFleetHostNamesV1(store:string,origins:readonly string[],now:number):Promise<ReadonlyMap<string,FleetWorkerHostNameV1>>{
 const {readFleetAddressSetsV1,fleetAdvertiseGateV1}=await import("../../../../🤝️shared-worktree/🗺️topology/🧭️addresses/🟦️.ts"),sets=await readFleetAddressSetsV1(store,now),labels=sets.map(set=>({origin:set.origin,hostLabel:set.hostLabel}));
 return new Map(origins.map(origin=>{
  const label=sets.find(set=>set.origin===origin)?.hostLabel??null;
  if(label===null)return [origin,{state:"unknown",label:null,others:[]}];
  const gate=fleetAdvertiseGateV1({origin,hostLabel:label,others:labels});
  return [origin,gate.namedAs!==null?{state:"named-as",label,others:[gate.namedAs]}:gate.duplicates.length>0?{state:"duplicate",label,others:gate.duplicates}:{state:"ok",label,others:[]}];
 }) as [string,FleetWorkerHostNameV1][]);
}

/** 🩺️ Workers: every origin of the fleet store'''),
(' const trees=await readWorkerTreesV1(store,context),at=context.now();',' const trees=await readWorkerTreesV1(store,context),at=context.now(),hosts=await readFleetHostNamesV1(store,rows.map(row=>row.origin),at);'),
('...trees?.has(row.origin)?{tree:trees.get(row.origin)!}:{},','...trees?.has(row.origin)?{tree:trees.get(row.origin)!}:{},hostName:hosts.get(row.origin)!,'),
('FleetWorkerMappingV1,FleetWorkerMemoryV1,','FleetWorkerHostNameV1,FleetWorkerMappingV1,FleetWorkerMemoryV1,'),
])
