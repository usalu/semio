"""The staged change of nxret-g as anchored patches. Every anchor must exist exactly once (or the patch is already applied). Usage: python3 stage.py check <root> | build <root> (writes `<name>.nxret.ts` siblings next to the TypeScript targets, same relative imports, for tsc; delete them right after; nothing replaced) | apply <root> (backs every touched file up into `backup/`, then replaces each by atomic rename) | revert <root> (restores the backups)."""
import hashlib,json,os,shutil,sys

HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0,HERE)
from stage_command import COMMAND_PATCHES
BACKUP=os.path.join(HERE,"backup")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
R=f"{L}/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention"
PHYSICAL=f"{R}/📥️physical/🟦️.ts"
PHYSICAL_SCHEMA=f"{R}/📥️physical/🧬️schema/🔣️.json"

TYPE_ANCHOR="/** 🧹️ Retires only released managed Nx generations; current, unknown and witnessed readers remain charged. */"
TYPE_NEW="""/** 🎯️ Names the exact generations a count-bound retirement may remove, oldest first, and calls back for each one it reaches; the same lease, authentication and witness checks as a headroom retirement still decide every single one. */
export type ToolingRetirementSelectionV1=Readonly<{identities:readonly string[];progress?:(identity:string,index:number,total:number)=>void|Promise<void>}>;
"""

PHYSICAL_PATCHES=[
 (TYPE_ANCHOR,TYPE_NEW+TYPE_ANCHOR),
 ("registry:ResourceLease,control:ResourceLeaseControl,quiescent=false):Promise<Readonly<{retired:readonly string[];availableBytes:number}>>{",
  "registry:ResourceLease,control:ResourceLeaseControl,quiescent=false,selection?:ToolingRetirementSelectionV1):Promise<Readonly<{retired:readonly string[];availableBytes:number;held:readonly string[];kept:readonly string[]}>>{"),
 ("const retired:string[]=[],available=async()=>{","const retired:string[]=[],held:string[]=[],kept:string[]=[],available=async()=>{"),
 ("if(await available()>=request.minimumAvailableBytes)return{retired,availableBytes:await available()};",
  "if(!selection&&await available()>=request.minimumAvailableBytes)return{retired,held,kept,availableBytes:await available()};"),
 ('const candidates:string[]=[];for await(const entry of await opendir(base)){if(!/^[a-f0-9]{64}$/u.test(entry.name))continue;if(candidates.length>=protocol.maximumCandidates)throw Error("Bounded Original tooling generation census required");candidates.push(entry.name);}candidates.sort();for(const identity of candidates){checkResourceLeaseControl(control);if(await available()>=request.minimumAvailableBytes)break;if(identity===basename(current))continue;',
  'const candidates:string[]=[];if(selection){if(selection.identities.length>protocol.maximumCandidates||new Set(selection.identities).size!==selection.identities.length||selection.identities.some(row=>!/^[a-f0-9]{64}$/u.test(row)))throw Error("Bounded selected tooling generations required");candidates.push(...selection.identities);}else{for await(const entry of await opendir(base)){if(!/^[a-f0-9]{64}$/u.test(entry.name))continue;if(candidates.length>=protocol.maximumCandidates)throw Error("Bounded Original tooling generation census required");candidates.push(entry.name);}const born=new Map<string,number>();for(const name of candidates){checkResourceLeaseControl(control);const row=await lstat(join(base,name)).catch(()=>undefined);born.set(name,row?Math.max(0,row.birthtimeMs>0?row.birthtimeMs:row.mtimeMs):0);}candidates.sort((left,right)=>born.get(left)!-born.get(right)!||(left<right?-1:left>right?1:0));}let reached=0;for(const identity of candidates){checkResourceLeaseControl(control);await selection?.progress?.(identity,reached++,candidates.length);if(!selection&&await available()>=request.minimumAvailableBytes)break;if(identity===basename(current)){kept.push(identity);continue;}'),
 ("const facts=await lstat(directory,{bigint:true});if(!facts.isDirectory()||facts.isSymbolicLink())return;",
  'const facts=await lstat(directory,{bigint:true}).catch(error=>{if(selection&&(error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;});if(!facts||!facts.isDirectory()||facts.isSymbolicLink())return;'),
 ('retired.push(identity);});});}catch(error){checkResourceLeaseControl(control);if((error as Error).message==="Resource lease timed out: "+protocol.owner+identity)continue;throw error;}}await retireReleasedNxSdkBinaryObjectsV1(store,control);return{retired:Object.freeze(retired),availableBytes:await available()};',
  'retired.push(identity);});});if(!retired.includes(identity))kept.push(identity);}catch(error){checkResourceLeaseControl(control);if((error as Error).message==="Resource lease timed out: "+protocol.owner+identity){held.push(identity);continue;}throw error;}}await retireReleasedNxSdkBinaryObjectsV1(store,control);return{retired:Object.freeze(retired),held:Object.freeze(held),kept:Object.freeze(kept),availableBytes:await available()};'),
 ('request:Readonly<{version:1;mode:"quiescent-generations";minimumAvailableBytes:number}>):Promise<Readonly<{retired:readonly string[];availableBytes:number}>>{requireIssuedResourceControlRefinementV1',
  'request:Readonly<{version:1;mode:"quiescent-generations";minimumAvailableBytes:number}>|Readonly<{version:1;mode:"count-bound";identities:readonly string[]}>,progress?:ToolingRetirementSelectionV1["progress"]):Promise<Readonly<{retired:readonly string[];availableBytes:number;held:readonly string[];kept:readonly string[]}>>{requireIssuedResourceControlRefinementV1'),
 ('if(validateJsonSchemaSubset(schema.definitions.QuiescentRequest,request,schema).length)throw Error("Closed quiescent tooling retirement request required")',
  'if(validateJsonSchemaSubset(request.mode==="count-bound"?schema.definitions.CountBoundRequest:schema.definitions.QuiescentRequest,request,schema).length)throw Error("Closed quiescent tooling retirement request required")'),
 ("retireManagedToolingGenerationsOwnedV1(workspace,store,{version:1,minimumAvailableBytes:request.minimumAvailableBytes},registry,control,true)",
  'retireManagedToolingGenerationsOwnedV1(workspace,store,{version:1,minimumAvailableBytes:request.mode==="count-bound"?1:request.minimumAvailableBytes},registry,control,true,request.mode==="count-bound"?{identities:request.identities,...progress?{progress}:{}}:undefined)'),
]


LEDGER=f"{L}/⚡️caching/💾️storage/🧾️ledger"
RUNNER=f"{L}/🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts"
COMMAND=f"{L}/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts"
LAUNCH=f"{L}/🏗️build/🧹️cache-gc/🚀️launch/🟦️.ts"
TOOLS_MODULE="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts"
NX_WRITER="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/🟦️.ts"

LEDGER_TS_PATCHES=[
 ('export type CacheOwnerV1="image"|"control"|"cargo"|"nx"|"bazel"|"test-evidence"|"dashboard"|"graph"|"remote-cache"|"remote-execution";',
  'export type CacheOwnerV1="image"|"control"|"cargo"|"nx"|"nx-tooling"|"bazel"|"test-evidence"|"dashboard"|"graph"|"remote-cache"|"remote-execution";'),
 ('/** 🧹️ How an area is collected: how long it may stay unused, how long a unit must be quiet, what proves a unit is in use, and whether a unit is a child (default) or a grandchild of the area directory. */\nexport type CacheCollectV1=Readonly<{unusedAgeMs:number|null;quietMs:number;custody:"quiet"|"sqlite"|"lease";lease?:string;depth?:1|2}>;',
  '/** 🧹️ How an area is collected: how long it may stay unused, how long a unit must be quiet, what proves a unit is in use (`pid`: the unit name carries the process id, `(?<pid>…)` in `units`, and a live process holds it), whether a unit is a child (default) or a grandchild of the area directory, and which children of the directory are units at all (`units`, a nested area may then share its directory with the homes of other areas). */\nexport type CacheCollectV1=Readonly<{unusedAgeMs:number|null;quietMs:number;custody:"quiet"|"sqlite"|"lease"|"pid";lease?:string;depth?:1|2;units?:string}>;'),
 ("const bytesOf=(budget:CacheBudgetV1|undefined):number=>budget&&\"bytes\" in budget?budget.bytes:0;",
  "const bytesOf=(budget:CacheBudgetV1|undefined):number=>budget&&\"bytes\" in budget?budget.bytes:0;\ntype NestedHome={path:string[];units:RegExp|undefined};\nconst carved=(parent:NestedHome,child:NestedHome):boolean=>parent.units!==undefined&&parent.path.length<child.path.length&&inside(parent.path,child.path)&&!parent.units.test(child.path[parent.path.length]!);"),
 ('nested=new Map<string,{paths:string[][];explicit:number;remainders:number}>()',
  'nested=new Map<string,{paths:NestedHome[];explicit:number;remainders:number}>()'),
 ('  if(area.pattern!==undefined)try{new RegExp(area.pattern,"u");}catch{refuse("valid pattern required for "+area.id);}',
  '  if(area.pattern!==undefined)try{new RegExp(area.pattern,"u");}catch{refuse("valid pattern required for "+area.id);}\n  if(area.collect&&((area.collect.custody==="pid")!==(area.collect.units?.includes("(?<pid>")===true)||area.collect.units!==undefined&&(area.kind!=="nested"||area.collect.depth===2)))refuse("pid custody needs a unit pattern with a (?<pid>) group, and unit patterns need a depth-1 nested area: "+area.id);'),
 ('if(remainder)row.remainders++;else{const own=segments(area.directory!);if(row.paths.some(other=>inside(other,own)||inside(own,other)))refuse("overlapping nested areas in "+area.partition);row.paths.push(own);}',
  'if(remainder)row.remainders++;else{let units:RegExp|undefined;try{units=area.collect?.units===undefined?undefined:new RegExp(area.collect.units,"u");}catch{refuse("valid unit pattern required for "+area.id);}const own:NestedHome={path:segments(area.directory!),units};if(row.paths.some(other=>(inside(other.path,own.path)||inside(own.path,other.path))&&!carved(other,own)&&!carved(own,other)))refuse("overlapping nested areas in "+area.partition);row.paths.push(own);}'),
]

def ledger_schema(text):
    document=json.loads(text)
    defs=document["$defs"]
    if "units" in defs["collect"]["properties"]:return text
    defs["managed"]["properties"]["owner"]["enum"].insert(defs["managed"]["properties"]["owner"]["enum"].index("nx")+1,"nx-tooling")
    defs["collect"]["properties"]["custody"]["enum"].append("pid")
    defs["collect"]["properties"]["units"]={"type":"string","minLength":1,"maxLength":200}
    return dump_like(text,document)

def dump_like(original,document):
    first=original.split("\n",2)[1]
    size=len(first)-len(first.lstrip())
    return json.dumps(document,indent=size,ensure_ascii=False)+"\n"

NX_AREAS="""    {
      "id": "nx-tooling",
      "root": "native",
      "kind": "nested",
      "directory": "nx",
      "partition": "tools",
      "budget": {
        "bytes": 2147483648
      },
      "managed": {
        "owner": "nx-tooling"
      },
      "reclaim": {
        "softAvailableBytes": 2684354560,
        "targetAvailableBytes": 4294967296
      },
      "writers": [
        "%s"
      ],
      "purpose": {
        "en": "Nx toolkit generations: one per patch, lock or package edit, bounded to the newest few plus the current activation and every generation a live reader holds.",
        "de": "Nx-Werkzeuggenerationen: eine je Patch-, Lock- oder Paketänderung, begrenzt auf die neuesten paar sowie die aktuelle Aktivierung und jede von einem laufenden Leser gehaltene Generation."
      }
    },
    {
      "id": "nx-tmp",
      "root": "repository",
      "kind": "nested",
      "directory": "nx/tmp",
      "partition": "rust",
      "budget": {
        "bytes": 268435456
      },
      "collect": {
        "unusedAgeMs": 3600000,
        "quietMs": 600000,
        "custody": "pid",
        "units": "^tmp-(?<pid>[0-9]+)-[A-Za-z0-9]+$"
      },
      "writers": [
        "%s"
      ],
      "purpose": {
        "en": "Temporary install trees of Nx processes (tmp-<pid>-<random>); a tree whose process is gone is removed, one a live process may still use never is.",
        "de": "Temporäre Installationsbäume von Nx-Prozessen (tmp-<pid>-<zufall>); ein Baum, dessen Prozess beendet ist, wird entfernt, einer, den ein lebender Prozess noch nutzen kann, nie."
      }
    },
"""%(TOOLS_MODULE,NX_WRITER)
LEDGER_POLICY_PATCHES=[('    {\n      "id": "compile-cache-products",',NX_AREAS+'    {\n      "id": "compile-cache-products",'),
 ('      "directory": "nx/tmp/node-compile-cache",\n      "partition": "rust",\n      "budget": {\n        "bytes": 1073741824\n      },','      "directory": "nx/tmp/node-compile-cache",\n      "partition": "rust",\n      "budget": {\n        "bytes": 805306368\n      },')]

RUNNER_PATCHES=[
 ('type Resolved=Readonly<{target:CacheTargetV1;base:string;prefix:string[];unusedAgeMs:number|null;loose?:boolean;custody?:"quiet"|"sqlite"|"lease";release?:()=>void}>;',
  'type Resolved=Readonly<{target:CacheTargetV1;base:string;prefix:string[];unusedAgeMs:number|null;loose?:boolean;custody?:"quiet"|"sqlite"|"lease"|"pid";units?:RegExp;release?:()=>void}>;'),
 ('async function observe(base:string,names:readonly string[],id:string,custody:"quiet"|"sqlite"|"lease",ports:CacheRunnerPortsV1):Promise<CacheUnitV1[]>{',
  '/** 🧟️ Names the live process a unit directory belongs to when its name carries a process id (`(?<pid>…)` in the unit pattern); a process that cannot be signalled still counts as alive. */\nexport function cachePidHolderV1(name:string,units:RegExp|undefined,ports:Pick<CacheRunnerPortsV1,"alive">):string|undefined{\n const pid=Number(units?.exec(name)?.groups?.pid);\n return Number.isSafeInteger(pid)&&pid>0&&ports.alive(pid)?"pid:"+pid:undefined;\n}\n\nasync function observe(base:string,names:readonly string[],id:string,custody:"quiet"|"sqlite"|"lease"|"pid",ports:CacheRunnerPortsV1,pattern?:RegExp):Promise<CacheUnitV1[]>{'),
 ('const held=custody==="lease"?undefined:await probeCacheHeldV1(measure,ports);',
  'const held=custody==="lease"?undefined:custody==="pid"?cachePidHolderV1(name,pattern,ports)??await probeCacheHeldV1(measure,ports):await probeCacheHeldV1(measure,ports);'),
 ('if(directory!==null){base=directory;names=await unitNamesV1(directory,area.collect.depth??1);}',
  'const matcher=area.collect.units===undefined?undefined:new RegExp(area.collect.units,"u");\n   if(directory!==null){base=directory;names=(await unitNamesV1(directory,area.collect.depth??1)).filter(name=>matcher===undefined||matcher.test(name));}'),
 ('const units=heldBy?[]:await observe(base,names,area.id,area.collect.custody,ports);',
  'const units=heldBy?[]:await observe(base,names,area.id,area.collect.custody,ports,matcher);'),
 ('custody:area.collect.custody,...lease?{release:()=>lease.release()}:{}});',
  'custody:area.collect.custody,...matcher?{units:matcher}:{},...lease?{release:()=>lease.release()}:{}});'),
 ('const held=await probeCacheHeldV1(again,ports);',
  'const held=await probeCacheHeldV1(again,ports)??(row.custody==="pid"?cachePidHolderV1(removal.name,row.units,ports):undefined);'),
 ('delegate?(owner:string,areaId:string,dryRun:boolean,language:CacheLanguageV1):Promise<string|undefined>;',
  'delegate?(owner:string,areaId:string,dryRun:boolean,language:CacheLanguageV1,context?:Readonly<{machine:MachinePlanV1|null}>):Promise<string|undefined>;'),
 ('const size=await sized(area.id,[directory],area.id);used.set(area.id,size);',
  'const size=await sized(area.id,area.collect?.units===undefined?[directory]:await matching(directory,area.collect.units),area.id);used.set(area.id,size);'),
 ('resolved:Resolved[]=[],delegated:{owner:string;text:string}[]=[],notes:string[]=[],now=ports.now();',
  'resolved:Resolved[]=[],delegated:{owner:string;text:string}[]=[],deferred:{slot:number;owner:string;id:string}[]=[],notes:string[]=[],now=ports.now();'),
 ('    const answer=ports.delegate?await ports.delegate(area.managed.owner,area.id,request.dryRun,output.language):undefined;\n    if(answer!==undefined){delegated.push({owner:area.managed.owner,text:answer});notes.push(text("gc.owner",{owner:area.managed.owner,text:answer}));}else notes.push(text("gc.managed",{id:area.id,owner:area.managed.owner}));\n',
  '    deferred.push({slot:notes.length,owner:area.managed.owner,id:area.id});notes.push("");\n'),
 ('  const plans=planCacheCollectionV1(targets,now),removed:CacheRemovalV1[]=[]',
  '  for(const row of deferred){\n   const answer=ports.delegate?await ports.delegate(row.owner,row.id,request.dryRun,output.language,{machine}):undefined;\n   if(answer!==undefined){delegated.push({owner:row.owner,text:answer});notes[row.slot]=text("gc.owner",{owner:row.owner,text:answer});}else notes[row.slot]=text("gc.managed",{id:row.id,owner:row.owner});\n  }\n  const plans=planCacheCollectionV1(targets,now),removed:CacheRemovalV1[]=[]'),
]

def read(root,rel):
    with open(os.path.join(root,rel),encoding="utf8") as handle:return handle.read()

def patch_text(text,patches):
    for old,new in patches:
        if new!=old and text.count(new)==1:continue
        elif text.count(old)==1:text=text.replace(old,new)
        else:raise SystemExit(f"anchor missing or ambiguous ({text.count(old)}x): {old[:90]}")
    return text

def indentation(text):
    first=text.split("\n",2)[1]
    return len(first)-len(first.lstrip())

def schema_patch(text):
    document=json.loads(text)
    if document["definitions"]["Protocol"]["const"]["maximumCandidates"]==1024 and "CountBoundRequest" in document["definitions"]:return text
    size=indentation(text) or 2
    return patch_schema_with(text,size)

def patch_schema_with(text,size):
    document=json.loads(text)
    definitions=document["definitions"]
    definitions["Protocol"]["const"]["maximumCandidates"]=1024
    definitions["CountBoundRequest"]={"type":"object","additionalProperties":False,"required":["version","mode","identities"],"properties":{"version":{"const":1},"mode":{"const":"count-bound"},"identities":{"type":"array","minItems":1,"maxItems":1024,"uniqueItems":True,"items":{"type":"string","pattern":"^[a-f0-9]{64}$"}}}}
    keys=list(definitions);keys.insert(keys.index("QuiescentRequest")+1,keys.pop(keys.index("CountBoundRequest")))
    document["definitions"]={key:definitions[key] for key in keys}
    return json.dumps(document,indent=size,ensure_ascii=False)+"\n"

TARGETS=[(PHYSICAL,lambda text:patch_text(text,PHYSICAL_PATCHES)),(PHYSICAL_SCHEMA,schema_patch),(f"{LEDGER}/🟦️.ts",lambda text:patch_text(text,LEDGER_TS_PATCHES)),(f"{LEDGER}/🧬️schema/🔣️.json",ledger_schema),(RUNNER,lambda text:patch_text(text,RUNNER_PATCHES)),(COMMAND,lambda text:patch_text(text,COMMAND_PATCHES)),(f"{LEDGER}/🔣️policy.json",lambda text:patch_text(text,LEDGER_POLICY_PATCHES))]

def produce(root):
    return [(rel,transform(read(root,rel))) for rel,transform in TARGETS]

def remember(root,rel):
    os.makedirs(BACKUP,exist_ok=True)
    name=hashlib.sha1(rel.encode()).hexdigest()[:16]
    manifest=os.path.join(BACKUP,"manifest.tsv")
    known=set(line.split("\t")[0] for line in open(manifest,encoding="utf8")) if os.path.exists(manifest) else set()
    if name in known:return
    shutil.copy2(os.path.join(root,rel),os.path.join(BACKUP,name+".bak"))
    with open(manifest,"a",encoding="utf8") as handle:handle.write(f"{name}\t{rel}\n")

def main():
    mode,root=sys.argv[1],os.path.abspath(sys.argv[2])
    if mode=="revert":
        manifest=os.path.join(BACKUP,"manifest.tsv")
        for line in open(manifest,encoding="utf8"):
            name,rel=line.rstrip("\n").split("\t")
            temporary=os.path.join(root,rel)+".nxret-revert"
            shutil.copy2(os.path.join(BACKUP,name+".bak"),temporary);os.replace(temporary,os.path.join(root,rel));print("restored",rel)
        return
    produced=produce(root)
    if mode=="check":
        for rel,_ in produced:print("ok",rel)
        return
    if mode=="syntax":
        import subprocess,tempfile
        for rel,text in produced:
            if rel.endswith(".json"):json.loads(text);print("json ok",rel[-40:]);continue
            with tempfile.TemporaryDirectory() as scratch:
                path=os.path.join(scratch,"probe.ts")
                with open(path,"w",encoding="utf8") as handle:handle.write(text)
                run=subprocess.run(["bun","build","--no-bundle",path],capture_output=True,text=True)
                print("ts ok" if run.returncode==0 else "ts FAIL",rel[-40:],run.stderr[:400])
        return
    if mode=="build":
        for rel,text in produced:
            if not rel.endswith(".ts"):continue
            target=rel[:-3]+".nxret.ts"
            with open(os.path.join(root,target),"w",encoding="utf8") as handle:handle.write(text)
            print("built",target)
        return
    if mode=="apply":
        for rel,text in produced:
            remember(root,rel)
            temporary=os.path.join(root,rel)+".nxret-apply"
            with open(temporary,"w",encoding="utf8") as handle:handle.write(text)
            os.replace(temporary,os.path.join(root,rel));print("applied",rel)
        return
    raise SystemExit("usage: stage.py check|build|apply|revert <root>")

main()
