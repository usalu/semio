import os,re,sys,json,hashlib
LIVE="/Users/ueli/Documents/semio"
B=LIVE+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
N=B+"/🕸️graph/📥️inference/📦️project-nodes"
P=N+"/🌐️owner-pool"
FILES={
 "nodes":N+"/🟨️.mjs",
 "pool":P+"/🟦️.ts",
 "schema":P+"/🧬️schema/🔣️.json",
 "status":P+"/📊️status/🟦️.ts",
 "runner":P+"/🏃️runner/🟨️.mjs",
 "child":P+"/🧒️child/🟨️.mjs",
 "driver":P+"/🔬️inference/🟨️.mjs",
 "publisher":P+"/📣️publisher/🟦️.ts",
 "selected":N+"/🧪️tests/🤝️selected-fleet/🟦️.ts",
}
def read(p): return open(p,encoding="utf8").read()
def write(p,t):
    os.makedirs(os.path.dirname(p),exist_ok=True); open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
def one(t,old,new):
    assert t.count(old)==1,("anchor count",t.count(old),old[:100])
    return t.replace(old,new)

def nodes(t):
    t=one(t,'/** 🏷️ The job of one owner of a wave that is inferred without an admission selection,',
'''/** 🧷️ What an owner admission selects of one project: the selected targets (`undefined` when the admission names none for it), the sorted selected inputs and the project JSON reduced to the selected targets. The one function the reading machine and a pool child share, so both file the record under the same key.
 * @see ./🧪️tests/🤝️pool-admission/🟦️.ts */
export function ownerAdmissionSelectionV1(admission, json) {
  const selectedTargets = admission?.targets[json.name], selectedInputs = [...new Set(admission?.inputs?.[json.name] ?? [])].sort(), activeJson = admission ? { ...json, targets: Object.fromEntries(Object.entries(json.targets ?? {}).filter(([name]) => selectedTargets?.includes(name))) } : json;
  return { selectedTargets, selectedInputs, activeJson };
}

/** 🧷️ Whether the selected part of a project is compiled (not only read as source): the decision of an admission read for `native`, shared with the pool child.
 * @see ./🧪️tests/🤝️pool-admission/🟦️.ts */
export const ownerAdmissionCompilesV1 = (activeJson, selectedInputs, nativeCandidate, nativeTargets) => buildNativeSourceObservationV1({ selectedSource: Object.values(activeJson.targets ?? {}).some(target => target.metadata?.semio?.execution?.kind === "source"), selectedNative: nativeCandidate && nativeTargets.length > 0, requestedInputs: selectedInputs.length > 0, dependencyInputs: false, inputRole: buildInputSourceRoleV1(Object.keys(activeJson.targets ?? {}).length > 0, false, Object.values(activeJson.targets ?? {}).map(target => target.metadata?.semio?.sourceInputObservation)) }) === "compile";

/** 📦️ What a reader of an admission hands a pool wave about one owner so a machine that pulls it files the same record: the targets and inputs the admission selected for it and the generated task dependencies it applies. */
export const ownerAdmissionOfV1 = (admission, job) => ({ ...admission.targets[job.json.name] === undefined ? {} : { targets: admission.targets[job.json.name] }, inputs: [...new Set(admission.inputs?.[job.json.name] ?? [])].sort(), ...admission.generatedDependencies?.[job.name] ? { generated: admission.generatedDependencies[job.name] } : {} });

/** 🏷️ The job of one owner of a wave that is inferred without an admission selection,''')
    t=one(t,'export function ownerJobV1(environment, globals, { name, json }) {\n  const { workspaceRoot, projectsByRoot, componentRoots, nativeCache, nativeClosures } = environment, root = json.root, nativeRoot = json.metadata?.nativeRoot ?? root, selectedInputs = [];\n  const nativeTargets = Object.entries(json.targets ?? {}).filter(([, target]) => target.metadata?.semio?.execution?.kind !== "source");\n  const native = existsSync(join(workspaceRoot, nativeRoot, "Cargo.toml")) || nativeTargets.some(([, target]) => target.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts")), pipeline = Object.values(json.targets ?? {}).some(target => target.metadata?.semio?.release);',
'export function ownerJobV1(environment, globals, { name, json: raw, admission }) {\n  const { workspaceRoot, projectsByRoot, componentRoots, nativeCache, nativeClosures } = environment, json = admission?.generated ? applyBuildGeneratedTaskDependenciesV1(raw, admission.generated) : raw, root = json.root, nativeRoot = json.metadata?.nativeRoot ?? root;\n  const { selectedTargets, selectedInputs, activeJson } = ownerAdmissionSelectionV1(admission === undefined ? undefined : { targets: { [json.name]: admission.targets }, inputs: { [json.name]: admission.inputs } }, json);\n  const nativeTargets = Object.entries(activeJson.targets ?? {}).filter(([, target]) => target.metadata?.semio?.execution?.kind !== "source");\n  const nativeCandidate = existsSync(join(workspaceRoot, nativeRoot, "Cargo.toml")) || nativeTargets.some(([, target]) => target.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts")), native = admission === undefined ? nativeCandidate : ownerAdmissionCompilesV1(activeJson, selectedInputs, nativeCandidate, nativeTargets), pipeline = Object.values(activeJson.targets ?? {}).some(target => target.metadata?.semio?.release);')
    t=one(t,'return { name, root, json, native, pipeline, nativeRoot, selectedInputs, head, cacheKey: ownerCacheKeyV1(root) };\n}',
          'return { name, root, json, native, pipeline, nativeRoot, selectedTargets, selectedInputs, head, cacheKey: ownerCacheKeyV1(root), ...admission === undefined ? {} : { admission: true } };\n}')
    t=one(t,'observeProjectInferenceV1({ cachePath, key: projectRecordKeyV1(job.head, job.root, job.json, undefined, job.selectedInputs), owner: job.root, stamps: projectStampPathsV1({ native: job.native }), reusable: projectRecordReusableV1 }, () => inferProjectBodyV1(environment, job, undefined))',
          'observeProjectInferenceV1({ cachePath, key: projectRecordKeyV1(job.head, job.root, job.json, job.selectedTargets, job.selectedInputs), owner: job.root, stamps: projectStampPathsV1({ native: job.native }), reusable: projectRecordReusableV1 }, () => inferProjectBodyV1(environment, job, job.admission === true))')
    t=one(t,'const selectedTargets=options?.ownerAdmission?.targets[json.name],selectedInputs=[...new Set(options?.ownerAdmission?.inputs?.[json.name]??[])].sort(),activeJson=options?.ownerAdmission?{...json,targets:Object.fromEntries(Object.entries(json.targets??{}).filter(([name])=>selectedTargets?.includes(name)))}:json;',
          'const{selectedTargets,selectedInputs,activeJson}=ownerAdmissionSelectionV1(options?.ownerAdmission,json);')
    t=one(t,'native=options?.ownerAdmission?buildNativeSourceObservationV1({selectedSource:Object.values(activeJson.targets??{}).some(target=>target.metadata?.semio?.execution?.kind==="source"),selectedNative:nativeCandidate&&nativeTargets.length>0,requestedInputs:selectedInputs.length>0,dependencyInputs:false,inputRole:buildInputSourceRoleV1(Object.keys(activeJson.targets??{}).length>0,false,Object.values(activeJson.targets??{}).map(target=>target.metadata?.semio?.sourceInputObservation))})==="compile":nativeCandidate',
          'native=options?.ownerAdmission?ownerAdmissionCompilesV1(activeJson,selectedInputs,nativeCandidate,nativeTargets):nativeCandidate')
    t=one(t,'let ownerPool;if(!options?.ownerAdmission){try{','let ownerPool;{try{')
    t=one(t,'...enforcement.known?.(job.name)===undefined?{}:{megabytes:enforcement.known(job.name)},json:job.json}));},say:',
          '...enforcement.known?.(job.name)===undefined?{}:{megabytes:enforcement.known(job.name)},...options?.ownerAdmission?{admission:ownerAdmissionOfV1(options.ownerAdmission,job)}:{},json:job.json}));},say:')
    t=one(t,'import{applyBuildGeneratedTaskDependenciesV1,buildInputSourceRoleV1,buildNativeSourceObservationV1}from"../../🚪️admission/🟦️.ts";','import{applyBuildGeneratedTaskDependenciesV1,buildInputSourceRoleV1,buildNativeSourceObservationV1}from"../../🚪️admission/🟦️.ts";\nimport{ownerPoolNoProducerTextV1,ownerPoolProducersV1,ownerPoolReadFleetV1}from"./🌐️owner-pool/🤝️fleet/🟦️.ts";')
    t=one(t,'const refuseHeavy=left=>enforcement.cost.refuseHeavyOwnerV1({named:named(left),','const refuseHeavy=(left,extra)=>enforcement.cost.refuseHeavyOwnerV1({named:named(left)+(extra?" ("+extra+")":""),')
    t=one(t,'    ownerPool.miss(Number.MAX_SAFE_INTEGER);\n    const wantedModule=',"""    ownerPool.miss(Number.MAX_SAFE_INTEGER);
    const fleetStore=process.env.SEMIO_FLEET_STORE_ROOT,askUnits=pending.map(job=>({name:job.name,kind:ownerKindV1(job.json,workspaceRoot),expectedMilliseconds:OWNER_SLOW_SECONDS_V1*2000,megabytes:Math.max(1,enforcement.known?.(job.name)??Math.round(enforcement.soloLimitBytes()/1048576*enforcement.cost.ownerCostPolicyV1.footprintFactor)+1),...options?.ownerAdmission?{admission:ownerAdmissionOfV1(options.ownerAdmission,job)}:{},json:job.json})),need=Math.max(0,...askUnits.map(unit=>unit.megabytes)),seeFleet=()=>ownerPoolReadFleetV1(fleetStore),seeProducers=()=>ownerPoolProducersV1(seeFleet(),need,Date.now());
    if(seeFleet().length>0&&seeProducers().length===0){const why=ownerPoolNoProducerTextV1(seeFleet(),need,Date.now());takeSay(()=>why);throw refuseHeavy(pending.map(job=>job.name),why);}
    await ownerPool.ask(askUnits).catch(()=>false);
    const wantedModule=""")
    t=one(t,'alive:sign,progressSign:sign,renew:async()=>{ownerPool.miss(Number.MAX_SAFE_INTEGER);},','alive:()=>sign()+Math.max(0,...seeProducers().map(row=>row.updatedMs)),progressSign:sign,renew:async()=>{ownerPool.miss(Number.MAX_SAFE_INTEGER);await ownerPool.ask(askUnits,true).catch(()=>false);},')
    return t

def pool(t):
    t=one(t,'megabytes?:number;json:Readonly<Record<string,unknown>>}>;\n/** 🧾️','megabytes?:number;admission?:Readonly<{targets?:readonly string[];inputs:readonly string[];generated?:unknown}>;json:Readonly<Record<string,unknown>>}>;\n/** 🧾️')
    t=one(t,'units.map(unit=>[unit.name,sha(JSON.stringify(unit.json))])','units.map(unit=>[unit.name,sha(JSON.stringify(unit.admission===undefined?unit.json:[unit.json,unit.admission]))])')
    t=one(t,'input:Readonly<{units:()=>readonly U[];fits?(unit:U):boolean;','input:Readonly<{units:()=>readonly U[];fits?(unit:U):boolean;turn?(unit:U):number;')
    t=one(t,'const fits=input.fits??(()=>true),usable=','const seen=new Map<string,number>(),turnOf=input.turn,waiting=(unit:U)=>{const wait=turnOf?.(unit)??0;if(wait<=0)return false;const first=seen.get(unit.name)??(seen.set(unit.name,now()),now());return now()-first<wait;};\n const fits=input.fits??(()=>true),usable=')
    t=one(t,'&&!queuedHeavy.has(unit.name)&&(retryAt.get(unit.name)??0)<=now())heavy.push(','&&!queuedHeavy.has(unit.name)&&(retryAt.get(unit.name)??0)<=now()&&!waiting(unit))heavy.push(')
    t=one(t,'if(finished.has(unit.name)||running.has(unit.name)||queued.has(unit.name)||(retryAt.get(unit.name)??0)>now())continue;','if(finished.has(unit.name)||running.has(unit.name)||queued.has(unit.name)||(retryAt.get(unit.name)??0)>now()||ownerPoolHeavyV1(unit,policy)&&waiting(unit))continue;')
    return t

def schema(t):
    t=one(t,'''        "megabytes": {
          "type": "integer",
          "minimum": 0
        },
        "json": {''','''        "megabytes": {
          "type": "integer",
          "minimum": 0
        },
        "admission": {
          "description": "What the admission of the reading machine selected for this owner (targets, inputs, generated task dependencies), so a machine that pulls the owner files the same record the admission read looks up.",
          "type": "object",
          "additionalProperties": false,
          "required": ["inputs"],
          "properties": {
            "targets": { "type": "array", "items": { "type": "string" } },
            "inputs": { "type": "array", "items": { "type": "string" } },
            "generated": {}
          }
        },
        "json": {''')
    t=one(t,'''        "rssMegabytes": {
          "type": "integer",
          "minimum": 0
        },
        "siblings": {''','''        "rssMegabytes": {
          "type": "integer",
          "minimum": 0
        },
        "totalMegabytes": {
          "description": "The memory of the machine, so the fleet knows which machines can hold which owner.",
          "type": "integer",
          "minimum": 0
        },
        "grantMegabytes": {
          "description": "The most one child of this machine may hold (the hard limit of a child).",
          "type": "integer",
          "minimum": 0
        },
        "siblings": {''')
    return t

def status(t):
    return one(t,'rssMegabytes:number;siblings:number;','rssMegabytes:number;totalMegabytes?:number;grantMegabytes?:number;siblings:number;')

def runner(t):
    t=one(t,'status?.update({ state: "idle", siblings });','status?.update({ state: "idle", siblings, totalMegabytes: total, grantMegabytes: ownerPoolChildLimitsV1(total).hardMegabytes });')
    t=one(t,'writeFileSync(file, JSON.stringify({ dir: wave.dir, root, fleetRoot,','writeFileSync(file, JSON.stringify({ store: environment.SEMIO_FLEET_STORE_ROOT, origin: originOf(fleetRoot), dir: wave.dir, root, fleetRoot,')
    t=one(t,'const sleep = (milliseconds, signal)','const originOf = fleetRoot => { try { return JSON.parse(readFileSync(machinePathV1(fleetRoot, ".cache", "f", "machine.json"), "utf8")).originId; } catch { return undefined; } };\nconst sleep = (milliseconds, signal)')
    return t

def child(t):
    t=one(t,'import { OWNER_POOL_POLICY_V1 as P, ownerPoolFitsV1, ownerPoolRecycleV1, pullOwnerPoolV1 } from "../🟦️.ts";','import { OWNER_POOL_POLICY_V1 as P, ownerPoolFitsV1, ownerPoolHeavyV1, ownerPoolRecycleV1, pullOwnerPoolV1 } from "../🟦️.ts";\nimport { ownerPoolReadFleetV1, ownerPoolTurnMillisecondsV1 } from "../🤝️fleet/🟦️.ts";')
    t=one(t,'fits: unit => ownerPoolFitsV1(unit, args.grantMegabytes ?? Infinity),','fits: unit => ownerPoolFitsV1(unit, args.grantMegabytes ?? Infinity), turn: unit => { if (!ownerPoolHeavyV1(unit) || args.origin === undefined) return 0; if (Date.now() - fleetAt > 5000) { fleet = ownerPoolReadFleetV1(args.store); fleetAt = Date.now(); } return ownerPoolTurnMillisecondsV1(args.origin, fleet, unit.megabytes ?? 0, Date.now()) ?? 0; },')
    t=one(t,'  let stats;\n  try {\n    stats = await pullOwnerPoolV1({','  let stats, fleet = [], fleetAt = -Infinity;\n  try {\n    stats = await pullOwnerPoolV1({')
    return t

def driver(t):
    t=one(t,'keyText: ownerRecordKeyTextV1(projectRecordKeyV1(job.head, job.root, job.json, undefined, job.selectedInputs))','keyText: ownerRecordKeyTextV1(projectRecordKeyV1(job.head, job.root, job.json, job.selectedTargets, job.selectedInputs))')
    return t

def publisher(t):
    t=one(t,'const run=async(forced:boolean):Promise<boolean>=>{\n  tried=now();','const run=async(forced:boolean,only?:readonly OwnerPoolUnitV1[]):Promise<boolean>=>{\n  if(only===undefined)tried=now();')
    t=one(t,'units=input.units(),partial=','units=only??input.units(),partial=')
    t=one(t,'const id=ownerPoolIdV1(partial,units);if(!forced&&last?.id===id)return true;\n   waveId=id;count=units.length;','const id=ownerPoolIdV1(partial,units);if(only!==undefined){if(!forced&&asked?.id===id&&now()-asked.at<P.republishKeepAliveMilliseconds)return true;}else if(!forced&&last?.id===id)return true;\n   if(only===undefined){waveId=id;count=units.length;}')
    t=one(t,'if(stored)last={id,count:units.length,at:now()};\n   input.say?.(','if(stored){if(only===undefined)last={id,count:units.length,at:now()};else asked={id,at:now()};}\n   input.say?.(')
    t=one(t,'status?.update({state:"pulling",wave:id,units:units.length,published:0,lastError:stored?null:"the fleet did not accept the pool"});\n   return stored;','if(only===undefined)status?.update({state:"pulling",wave:id,units:units.length,published:0,lastError:stored?null:"the fleet did not accept the pool"});\n   return stored;')
    t=one(t,'let chain:Promise<boolean>|undefined,waveId:string|null=null,count=0,tried=0,over=false,last:{id:string;count:number;at:number}|undefined;','let chain:Promise<boolean>|undefined,waveId:string|null=null,count=0,tried=0,over=false,last:{id:string;count:number;at:number}|undefined,asked:{id:string;at:number}|undefined,askChain:Promise<boolean>=Promise.resolve(false);')
    t=one(t,'  published:()=>chain??Promise.resolve(false),','  ask(units,forced=false){if(settings===undefined||units.length===0)return Promise.resolve(false);return askChain=askChain.then(()=>run(forced,units));},\n  published:()=>chain??Promise.resolve(false),')
    t=one(t,'miss(count:number):void;published()','miss(count:number):void;ask(units:readonly OwnerPoolUnitV1[],forced?:boolean):Promise<boolean>;published()')
    return t

def selected(t):
    return one(t,'''test("the farm and the owner pool of the loop stay off for an admission, which they cannot serve",()=>{
 expect(text).toContain("const farm=timedFarmV1(options?.ownerAdmission?undefined:openProjectFarmV1(");
 expect(text).toContain("let ownerPool;if(!options?.ownerAdmission){");
});''','''test("the farm stays off for an admission, the owner pool serves it: the wave carries the admission of every owner",()=>{
 expect(text).toContain("const farm=timedFarmV1(options?.ownerAdmission?undefined:openProjectFarmV1(");
 expect(text).toContain("let ownerPool;{try{");
 expect(text).toContain("...options?.ownerAdmission?{admission:ownerAdmissionOfV1(options.ownerAdmission,job)}:{}");
});''')

PATCH={"nodes":nodes,"pool":pool,"schema":schema,"status":status,"runner":runner,"child":child,"driver":driver,"publisher":publisher,"selected":selected}
if __name__=="__main__":
    cmd=sys.argv[1]
    if os.environ.get("R164_MIRROR"): FILES={k:os.environ["R164_MIRROR"]+"/"+k for k in FILES}
    if cmd=="check":
        for k,f in FILES.items():
            if k in PATCH: PATCH[k](read(f)); print("anchors ok",k)
    elif cmd=="apply":
        backup=sys.argv[2];os.makedirs(backup,exist_ok=True);manifest={};new={}
        for k in PATCH: new[k]=PATCH[k](read(FILES[k]))
        for i,k in enumerate(PATCH):
            old=read(FILES[k]);write(backup+"/%d.bak"%i,old);manifest[k]={"i":i,"before":sha(old),"after":sha(new[k])}
        json.dump(manifest,open(backup+"/manifest.json","w"))
        for k in PATCH: write(FILES[k],new[k])
        print("applied")
    elif cmd=="restore":
        backup=sys.argv[2];manifest=json.load(open(backup+"/manifest.json"))
        for k,m in manifest.items():
            now=sha(read(FILES[k]))
            if now==m["after"]: write(FILES[k],read(backup+"/%d.bak"%m["i"])); print("restored",k)
            else: print("CHANGED SINCE, not restored",k)
