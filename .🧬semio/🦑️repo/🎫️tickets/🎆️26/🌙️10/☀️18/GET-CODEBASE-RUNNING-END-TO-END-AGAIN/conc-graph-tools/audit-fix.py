import os, sys, json, re, shutil
ROOT = "/Users/ueli/Documents/semio"
if "--root" in sys.argv: ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
HERE = os.path.dirname(os.path.abspath(__file__))
TOOLS = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/conc-graph-tools"
def resolve(rel, must=True):
    cur = ROOT
    for part in rel.split("/"):
        hits = [n for n in os.listdir(cur) if n.replace("️", "") == part.replace("️", "")]
        if not hits and not must: return os.path.join(cur, part)
        assert len(hits) == 1, (part, hits); cur = os.path.join(cur, hits[0])
    return cur
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
GEN = B + "🕸️graph/🕰️generations/"
ENGINE = resolve(GEN + "🟨️.mjs"); ENGINE_D = ENGINE.replace(".mjs", ".d.mts")
LOAD = resolve(GEN + "🔌️load/🟨️.mjs"); LOAD_D = LOAD.replace(".mjs", ".d.mts"); LOAD_T = resolve(GEN + "🔌️load/🧪️tests/🟦️.ts"); LOAD_DIR = os.path.dirname(LOAD_T)
WORLD = resolve(GEN + "🧪️tests/🏃️world/🟦️.ts")
G = resolve(B + "🕸️graph/🟦️.ts"); GSCHEMA = os.path.dirname(resolve(B + "🧬️schema/🚪️selection.json"))
PJ = resolve(B + "🕸️graph/🧪️tests/🕰️generation-protocol/🔣️.json")
AUTH = resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧬️schema/🔣️.json")
files = {}
def get(p):
    if p not in files: files[p] = [open(p, encoding="utf8", newline="").read() if os.path.exists(p) else None, False]
    return files[p]
def sub(p, marker, old, new, label, count=1, regex=False):
    row = get(p)
    if marker in row[0]: print("applied ", label); return
    if regex:
        found = re.findall(old, row[0], re.S); assert len(found) == count, (label, len(found))
        row[0] = re.sub(old, lambda m: new, row[0], count=1, flags=re.S)
    else:
        assert row[0].count(old) == count, (label, row[0].count(old), old[:90]); row[0] = row[0].replace(old, new)
    row[1] = True; print("pending ", label)

# ---- engine: global and catalog readers
sub(ENGINE, " async function readGlobal(manifest){", " const store=async(directory,value,extension,limit)=>",
 ''' /** 🌐️ Reads the global fragment of one manifest: the external nodes, the cross-owner edges and the parse failures. */
 async function readGlobal(manifest){const bytes=await fs.read(fragmentPath(manifest.global));if(!bytes||sha256(bytes)!==manifest.global)throw fail("Global fragment is missing","GENERATION_INCOMPLETE");return unpack(bytes,P.maximumFragmentBytes).value??{};}
 /** 📇️ Lists the projects (name, root, target names) and the unavailable projects of one sealed generation from the manifest and the few unavailable fragments, without reading any project node. */
 async function catalog(current,{signal}={}){
  const manifest=current?.manifest;if(!manifest)throw fail("A sealed generation is required","GENERATION_EMPTY");
  const rows=Object.entries(manifest.owners),global=await readGlobal(manifest);
  const loaded=await mapLimit(rows.filter(([,row])=>row.kind==="unavailable"),P.readConcurrency,async([name,row])=>{const fragment=await readFragment(row);if(!fragment)throw fail("Fragment of "+name+" is missing","GENERATION_INCOMPLETE");return fragment.node;},signal);
  return{projects:rows.filter(([,row])=>row.kind==="project").map(([name,row])=>({name,root:row.root,targets:row.targets})),unavailable:[...(global.unavailable??[]),...loaded].sort((a,b)=>compareCodePointsV1(a.root,b.root))};
 }
 const store=async(directory,value,extension,limit)=>''', "engine readers")
sub(ENGINE, "closureOf,assemble,snapshot,retire,invalidate,publish,stats,readGlobal,catalog", "closureOf,assemble,snapshot,retire,invalidate,publish,stats});", "closureOf,assemble,snapshot,retire,invalidate,publish,stats,readGlobal,catalog});", "engine export")
sub(ENGINE_D, "readGlobal(", " plan(options:{current:", " readGlobal(manifest:GenerationManifestV1):Promise<{readonly externalNodes?:Record<string,unknown>;readonly unavailable?:readonly {readonly root:string}[]}>;\n catalog(current:GenerationSealedV1,options?:{signal?:AbortSignal}):Promise<{projects:{name:string;root:string;targets:readonly string[]}[];unavailable:{root:string}[]}>;\n plan(options:{current:", "engine d.mts")

# ---- load module
sub(LOAD, "NO_NEIGHBORS", r"/\*\* 🧭️ Names the projects one project reaches in a task closure.*?\n\}\n\n", "", "load drop neighbors", regex=True) if "export function sliceNeighborsV1" in get(LOAD)[0] else print("applied  load drop neighbors")
sub(LOAD, "export async function sliceClosureV1({owners,fragment,selection,roots=[],providers=[],external=[]})", "export async function sliceClosureV1({owners,fragment,selection,roots=[],providers=[]}){\n const cache=new Map(),known=new Set(Object.keys(owners)),admitted=new Set(),active=new Map(),queue=[];",
 "export async function sliceClosureV1({owners,fragment,selection,roots=[],providers=[],external=[]}){\n const cache=new Map(),known=new Set(Object.keys(owners)),outside=new Set(external),admitted=new Set(),active=new Map(),queue=[];", "load closure signature")
sub(LOAD, "at===name||!outside.has(at)", "if(!known.has(at)){if(at===name)throw unknown(at);continue;}", "if(!known.has(at)){if(at===name||!outside.has(at))throw unknown(at);continue;}", "load unknown edges")
sub(LOAD, "an edge to a name that is neither an owner nor an external node refuses", " * `owners` maps every owner name to its root; `fragment(name)` yields `{edges, node}`; an unknown name refuses (`GENERATION_UNKNOWN_OWNER`), because the caller then infers the scope itself.",
 " * `owners` maps every owner name to its root, `external` names the external nodes; `fragment(name)` yields `{edges, node}`. A task, a root or an edge to a name that is neither an owner nor an external node refuses with `GENERATION_UNKNOWN_OWNER`: a slice is never silently incomplete.", "load closure doc")
NEWFN = '''/** 📦️ Expands compact inputs in groups that stay within the capacity of one capture (projects and text bytes per call); a group that still exceeds a capacity is halved, a single project that does is refused. */
export async function expandInChunksV1(projects,expand,{maximumProjects,maximumBytes}){
 const names=Object.keys(projects),result={};
 const pass=async group=>{
  const chunk={};for(const name of group)chunk[name]=projects[name];
  try{Object.assign(result,await expand(chunk));}
  catch(error){if(group.length>1&&/capacity exceeded/u.test(error?.message??"")){const half=group.length>>1;await pass(group.slice(0,half));await pass(group.slice(half));return;}throw error;}
 };
 let group=[],bytes=0;
 for(const name of names){const size=Buffer.byteLength(JSON.stringify(projects[name]));if(group.length&&(group.length>=maximumProjects||bytes+size>maximumBytes)){await pass(group);group=[];bytes=0;}group.push(name);bytes+=size;}
 if(group.length)await pass(group);
 return result;
}

/** 🕰️ Loads the all-workspace graph, the closure of some projects, or the slice of a selected scope (tasks or literal owners) from the newest sealed generation; `lean` answers only project and target names. A shared reader verifies and serves it without ever taking a writer; only a stale or missing generation enters the builder custody, where a holder that finds the work done by a peer serves that generation.
 * `host` binds the repository: `identity()`, `implementationRows()`, `withShared(body)`, `withBuilder(body)`, `storeOptions(store, control)`, `membership(previous, signal)`, `discoverPart(spec, signal)`, `expand(projects, control)`, `expansionLimits`, `readGraph(value)`, `readLean(value)`, `clock(control)`, `say(key, fields)` and the constants `workspaceRoot`, `namedInputs`, `registryDigest`, `signal`.
 */
export async function loadGenerationGraphV1(host,{policy="fresh",closure,selection,slice,lean}={}){
 const identity=await host.identity(),roots=slice?[...new Set(slice.roots)]:closure?[...new Set(closure)]:undefined,rows=await host.implementationRows(),implementation=implementationDigestV1(rows),scoped=scopeImplementationRowsV1(rows),limits=host.expansionLimits??{maximumProjects:32,maximumBytes:16777216};
 const attempt=async(builder,store,control)=>{
  const engine=createGenerationStoreV1(host.storeOptions(store,control)),progress=createGenerationProgressV1(host.clock(control));
  const compute=builder?async(spec,signal)=>shapeComputeResultV1({workspaceRoot:host.workspaceRoot,separator:host.separator,part:await host.discoverPart(spec,signal,control),stubs:spec.stubs,implementation:scoped}):async()=>{throw Error("A shared graph reader never computes");};
  const base={identity,implementation,policy:builder?"fresh":policy,membership:host.membership,compute,signal:host.signal,progress,builder:builder?body=>body():needsBuild};
  const closeFn=slice?async(current)=>{const global=await engine.readGlobal(current.manifest);return sliceClosureV1({owners:Object.fromEntries(Object.entries(current.manifest.owners).map(([name,row])=>[name,row.root])),external:Object.keys(global.externalNodes??{}),fragment:name=>engine.readFragment(current.manifest.owners[name]).then(value=>{if(!value)throw Object.assign(Error("Fragment of "+name+" is missing"),{code:"GENERATION_INCOMPLETE"});return value;}),selection:slice.selection,roots:slice.roots,providers:slice.providers??[]});}:undefined;
  const started=Date.now(),result=roots?await engine.refreshClosure({...base,roots,closure:closeFn}):await engine.refresh(base);
  if(result.needsBuild)return{needsBuild:true};
  const generation=result.current.manifest.generation;
  if(result.served==="current")host.say(result.current.manifest.implementation===implementation?"current":"stale",{generation});
  else if(result.served==="built"&&result.published)host.say("published",{generation,computed:result.computed,seconds:Math.round((Date.now()-started)/1000)});
  else if(host.verbose||lean)host.say(roots?"closure":"verified",roots?{count:result.names.size}:{generation,checked:result.plan?.checked??0});
  if(lean){const listing=await engine.catalog(result.current,{signal:host.signal});return{graph:host.readLean({version:1,lean:true,projects:Object.fromEntries(listing.projects.map(row=>[row.name,{name:row.name,root:row.root,targets:Object.fromEntries(row.targets.map(target=>[target,{}]))}])),dependencies:[],...(listing.unavailable.length?{unavailable:listing.unavailable}:{})}),result};}
  const assembled=await engine.assemble(result.current,{names:result.names&&[...result.names],compare:(a,b)=>a.localeCompare(b),signal:host.signal,progress});
  const projects=await expandInChunksV1(assembled.projects,chunk=>host.expand(chunk,control),limits),closureDigest=selection?createHash("sha256").update(canonicalJsonV1([assembled.membership,[...result.names].sort()])).digest("hex"):assembled.membership;
  const graph=host.readGraph({version:1,projects,dependencies:assembled.dependencies,namedInputs:host.namedInputs,externalNodes:assembled.externalNodes,...(assembled.unavailable.length?{unavailable:assembled.unavailable}:{}),admission:{selection:selection??{kind:"all"},registry:host.registryDigest,closure:closureDigest}});
  return{graph,result};
 };
 const first=await host.withShared((store,control)=>attempt(false,store,control));
 if(!first.needsBuild)return first.graph;
 return(await host.withBuilder((store,control)=>attempt(true,store,control))).graph;
}
'''
row = get(LOAD)
if "export async function expandInChunksV1" not in row[0]:
    i = row[0].index("/**\n * 🕰️ Loads the all-workspace graph")
    row[0] = row[0][:i] + NEWFN; row[1] = True; print("pending  load rewrite")
else: print("applied  load rewrite")
# d.mts
row = get(LOAD_D)
if "expandInChunksV1" not in row[0]:
    row[0] = re.sub(r"/\*\* 🧭️ Names the projects one project reaches in a task closure\. \*/\nexport function sliceNeighborsV1[^\n]*\n", "", row[0])
    row[0] = re.sub(r"export function loadGenerationGraphV1\(host:GenerationHostV1,options\?:\{[^\n]*\n", "export function loadGenerationGraphV1(host:GenerationHostV1,options?:{policy?:GenerationGraphPolicyV1;closure?:readonly string[];selection?:unknown;slice?:{selection:unknown;roots:readonly string[];providers?:readonly {consumerRoots:readonly string[];sourcePath:string}[]};lean?:boolean}):Promise<unknown>;\n/** 📦️ Expands compact inputs within the capacity of one capture. */\nexport function expandInChunksV1(projects:Record<string,unknown>,expand:(chunk:Record<string,unknown>)=>Promise<Record<string,unknown>>,limits:{maximumProjects:number;maximumBytes:number}):Promise<Record<string,unknown>>;\n", row[0])
    row[0] = row[0].replace("roots?:readonly string[];providers?:", "roots?:readonly string[];external?:readonly string[];providers?:")
    row[0] = row[0].replace(" expand(projects:Record<string,unknown>,control:unknown):Promise<Record<string,unknown>>;\n", " expand(projects:Record<string,unknown>,control:unknown):Promise<Record<string,unknown>>;\n expansionLimits?:{maximumProjects:number;maximumBytes:number};\n readLean(value:unknown):unknown;\n")
    row[1] = True; print("pending  load d.mts")
else: print("applied  load d.mts")

# ---- lean schema (schema-first) and graph glue
LEAN = os.path.join(GSCHEMA, "🪶️lean.json")
lean = {"$schema": "https://json-schema.org/draft/2020-12/schema", "title": "Lean build graph: project and target names only", "type": "object", "additionalProperties": False, "required": ["version", "lean", "projects", "dependencies"],
 "properties": {"version": {"const": 1}, "lean": {"const": True},
  "projects": {"type": "object", "additionalProperties": {"type": "object", "additionalProperties": False, "required": ["name", "root", "targets"], "properties": {"name": {"type": "string"}, "root": {"type": "string"}, "targets": {"type": "object", "additionalProperties": {"type": "object", "additionalProperties": False}}}}},
  "dependencies": {"type": "array", "items": {"type": "object"}},
  "unavailable": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["root", "source", "sha256", "reason"], "properties": {"root": {"type": "string"}, "project": {"type": "string"}, "source": {"type": "string"}, "sha256": {"type": "string"}, "reason": {"type": "string"}}}}}}
row = get(LEAN); want = json.dumps(lean, ensure_ascii=False, indent=1) + "\n"
if row[0] != want: row[0] = want; row[1] = True; print("pending  lean schema")
else: print("applied  lean schema")
sub(G, "import{loadGenerationGraphV1,renderGenerationMessageV1}from", "import{loadGenerationGraphV1,renderGenerationMessageV1,sliceNeighborsV1}from", "import{loadGenerationGraphV1,renderGenerationMessageV1}from", "G drop neighbors import")
row = get(G)
if 'new URL("../🧬️schema/🪶️lean.json"' in row[0].split("const graphModuleAuthorityV1")[0]: print("applied  G lean url")
else:
    start = row[0].index("const graphImplementationUrlsV1"); anchor = 'new URL("../🧬️schema/🚪️admission.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),'
    at = row[0].index(anchor, start) + len(anchor)
    row[0] = row[0][:at] + 'new URL("../🧬️schema/🪶️lean.json",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)),' + row[0][at:]; row[1] = True; print("pending  G lean url")
sub(G, "expansionLimits:{maximumProjects:32", r"    expand:async\(projects:Record<string,unknown>,control:ResourceLeaseControl\)=>\{const expandChunkV1=32.*?return result;\},",
 '    expand:async(projects:Record<string,unknown>,control:ResourceLeaseControl)=>(await portableInputs(root,projects,invocation,control)) as Record<string,unknown>,\n    expansionLimits:{maximumProjects:32,maximumBytes:Math.floor(graphDeclarationV1("../👁️observation/📥️reuse/🧬️schema/🔣️.json").definitions.Policy.const.maximumTextBytes/4)},\n    readLean:(value:unknown)=>readBuildGraphLeanV1(value),', "G expansion limits", regex=True)
WRAP_RE = r"export async function loadBuildGraphV1\(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1\):Promise<BuildGraphV1> \{.*?(?=type BuildGraphLoadOptionsV1=)"
NEW_WRAP = '''export async function loadBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  if(options.selection?.kind&&!options.baseline&&!options.reuse&&invocation){
    clearPinnedAuthorityV1();
    if(options.selection.kind==="all")return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));
    return withImplementationPinV1(async()=>{
      const sliced=await loadBuildGraphSliceV1(root,invocation,phaseControl,options);
      if(process.env.SEMIO_BUILD_GRAPH_VERIFY==="1")requireSelectedSubgraphV1(sliced,await loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));
      return sliced;
    });
  }
  return loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options);
}

/** 🍰️ Derives the graph of a selected scope (tasks or literal owners) from the newest sealed generation: the closure of the selected projects is read, verified and, under a fresh policy, refreshed incrementally. The generation is the only source of a graph: when none is usable it is built first, and a selected project the generation does not know refuses. */
async function loadBuildGraphSliceV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {
  const producer=await import("./📥️inference/🟨️.mjs"),registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),selection=readBuildGraphSelectionV1(options.selection);
  if(selection.kind==="all")throw Error("Selected build graph scope required");
  const roots=selection.kind==="tasks"?[...new Set(selection.tasks.map(task=>task.project))]:selectLiteralBuildProjectsV1(registry.owners,selection.projects,selection.exclude);
  return await loadGenerationGraphV1(generationHostV1(root,invocation,phaseControl,registry),{policy:options.graph?.policy??"fresh",selection,slice:{selection,roots,providers:registry.catalogProviders.map(({consumerRoots,sourcePath})=>({consumerRoots,sourcePath}))}}) as BuildGraphV1;
}

/** 🪶️ Admits the lean graph of `show projects|targets`: project and target names, unavailable projects, no inputs. */
function readBuildGraphLeanV1(value:unknown):BuildGraphV1 {
  const errors=validateJsonSchemaSubset(graphDeclarationV1("../🧬️schema/🪶️lean.json"),value);
  if(errors.length)throw Error(`Invalid lean build graph: ${JSON.stringify(errors.slice(0,4))}`);
  return value as BuildGraphV1;
}

/** 🔍️ Names the first difference between a generation slice and the selected graph a full selected inference produced, in both directions: every selected project is equal, the dependencies among the selected projects are the same set, and every selected external node is equal; the slice may hold more projects. The oracle of `SEMIO_BUILD_GRAPH_VERIFY=1` for selected scopes. */
function requireSelectedSubgraphV1(slice:BuildGraphV1,selected:BuildGraphV1):void{
  const same=(left:unknown,right:unknown)=>canonicalGenerationJsonV1(left)===canonicalGenerationJsonV1(right),chosen=new Set(Object.keys(selected.projects));
  for(const[name,project]of Object.entries(selected.projects)){if(!slice.projects[name])throw Error(`Graph slice lacks the selected project ${name}`);if(!same(slice.projects[name],project))throw Error(`Graph slice differs from the selected graph at project ${name}`);}
  const inside=(edge:{source:string;target:string})=>chosen.has(edge.source)&&(chosen.has(edge.target)||selected.externalNodes?.[edge.target]!==undefined||slice.externalNodes?.[edge.target]!==undefined);
  const mine=new Set(slice.dependencies.filter(inside).map(edge=>canonicalGenerationJsonV1(edge))),theirs=new Set(selected.dependencies.map(edge=>canonicalGenerationJsonV1(edge)));
  for(const edge of theirs)if(!mine.has(edge))throw Error("Graph slice lacks a selected dependency: "+edge);
  for(const edge of mine)if(!theirs.has(edge))throw Error("Graph slice adds a dependency the selected graph lacks: "+edge);
  for(const[key,node]of Object.entries(selected.externalNodes??{}))if(!same(slice.externalNodes?.[key],node))throw Error(`Graph slice differs from the selected graph at external node ${key}`);
}

'''
sub(G, "async function loadBuildGraphSliceV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {", WRAP_RE, NEW_WRAP, "G wrapper", regex=True)
sub(G, '&&!options.graph?.closure&&!options.graph?.lean)', 'process.env.SEMIO_BUILD_GRAPH_VERIFY==="1"&&!options.graph?.closure)', 'process.env.SEMIO_BUILD_GRAPH_VERIFY==="1"&&!options.graph?.closure&&!options.graph?.lean)', "G verify lean")
# authorizers: the lean schema is a declared read of the graph module
row = get(AUTH); data = json.loads(row[0]); lib = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
entry = next(r for r in data["$defs"]["RuntimeDataCompanions"]["const"] if r["owner"] == lib + "🏗️build/🕸️graph/🟦️.ts")
leanpath = lib + "🏗️build/🧬️schema/🪶️lean.json"
if leanpath in entry["paths"]: print("applied  authorizers")
else:
    entry["paths"].append(leanpath); entry["paths"] = sorted(entry["paths"], key=lambda s: [ord(c) for c in s]); row[0] = json.dumps(data, ensure_ascii=False, indent=2) + "\n"; row[1] = True; print("pending  authorizers")

# ---- world: the mock host validates lean graphs as identity
sub(WORLD, "readLean:(value:unknown)=>value", "  readGraph:(value:unknown)=>value,", "  readGraph:(value:unknown)=>value,\n  readLean:(value:unknown)=>value,", "world readLean")

# ---- protocol laws
def laws(data):
    present = {r["name"] for r in data["present"]}; absent = {r["name"] for r in data["absent"]}
    data["present"] = [r for r in data["present"] if r["name"] != "the host expands the compact inputs in bounded chunks"]
    for r in data["present"]:
        if r["name"] == "the generation load starts a new authority snapshot": r["text"] = "clearPinnedAuthorityV1();"
    for name, fn, text in [("the host bounds the text of one capture to a quarter of the reference capacity", "generationHostV1", "expansionLimits:{maximumProjects:32,maximumBytes:Math.floor("), ("the host validates the lean graph through its own schema", "generationHostV1", "readLean:(value:unknown)=>readBuildGraphLeanV1(value)")]:
        if name not in present: data["present"].append({"name": name, "function": fn, "text": text})
    if "a selected scope never falls back to the selected inference switch" not in absent: data["absent"].append({"name": "a selected scope never falls back to the selected inference switch", "function": "loadBuildGraphV1", "text": "SEMIO_BUILD_GRAPH_SLICE"})
row = get(PJ); data = json.loads(row[0]); before = json.dumps(data, sort_keys=True); laws(data)
if json.dumps(data, sort_keys=True) != before: row[0] = json.dumps(data, ensure_ascii=False, indent=1) + "\n"; row[1] = True; print("pending  protocol laws")
else: print("applied  protocol laws")

# ---- oracle, corpus and tests
row = get(LOAD_T)
if "roots:row.roots,providers:row.providers??[]" in row[0]: row[0] = row[0].replace("roots:row.roots,providers:row.providers??[]", "roots:row.roots,external:row.external??[],providers:row.providers??[]"); row[1] = True; print("pending  closure harness external")
ORACLE = os.path.join(LOAD_DIR, "🐍️oracle.py"); row = get(ORACLE); want = open(os.path.join(TOOLS, "oracle_slice.py"), encoding="utf8").read()
if row[0] != want: row[0] = want; row[1] = True; print("pending  oracle")
else: print("applied  oracle")
for source, target in [("fixture.json", os.path.join(LOAD_DIR, "🔣️.json")), ("fixture-schema.json", os.path.join(LOAD_DIR, "🧬️schema", "🔣️.json"))]:
    wanted = open(os.path.join(TOOLS, "staged", "new", "load", source), encoding="utf8").read(); row = get(target)
    if row[0] != wanted: row[0] = wanted; row[1] = True; print("pending  corpus", source)
    else: print("applied  corpus", source)

row = get(LOAD_T)
if "selected closure is the independent model" in row[0] and "slice scenario corpus agrees with the python oracle" not in row[0]:
    t = row[0]
    t = re.sub(r'for\(const\[index,row\]of\(fixture as any\)\.neighbors\.entries\(\)\)test\("slice neighbors are the independent model "\+index,\(\)=>\{[^\n]*\n', "", t)
    t = t.replace("scopeImplementationRowsV1,sliceClosureV1,sliceNeighborsV1,", "scopeImplementationRowsV1,sliceClosureV1,expandInChunksV1,")
    a1 = t.index('\ntest("a selected scope is the closure of its projects read from the generation'); b1 = t.index("\nfor(const[index,row]of(fixture as any).closures.entries())")
    t = t[:a1] + "\n" + t[b1:]
    c1 = t.index('\ntest("a task slice reads its closure from the generation'); t = t[:c1] + "\n"
    t += r'''
test("the slice scenario corpus agrees with the python oracle",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});

test("a selected scope is the closure of its projects read from the generation: it builds the generation when none is usable, current serves it, fresh refreshes only the dirty closure, the admission names the selection",async()=>{
 const dir=scratch(),selection={kind:"tasks",tasks:[{project:"a",target:"build"}]},options={selection,slice:{selection,roots:["a"]}};
 try{
  await seed(dir,sliceTree());const mock=createMockHost(dir);
  const cold=await loadGenerationGraphV1(mock.host,options) as any;expect(mock.events).toContain("builder:held");expect(mock.log.length).toBe(1);expect(Object.keys(cold.projects).sort()).toEqual(["a","b","c"]);
  mock.events.length=0;mock.expanded.length=0;const graph=await loadGenerationGraphV1(mock.host,{...options,policy:"current"}) as any;
  expect(mock.events).toEqual(["shared"]);expect(Object.keys(graph.projects).sort()).toEqual(["a","b","c"]);expect(graph.admission.selection).toEqual(selection);expect(graph.admission.registry).toBe(mock.host.registryDigest);expect(graph.admission.closure).toMatch(/^[a-f0-9]{64}$/u);expect(mock.expanded).toEqual([["a","b","c"]]);
  await applyOps(dir,[{op:"write",path:"pkg/f/src/index.ts",content:"export const other=1;\n"},{op:"write",path:"pkg/c/src/index.ts",content:"export const v=2;\n"}]);
  mock.events.length=0;mock.log.length=0;const served=await loadGenerationGraphV1(mock.host,{...options,policy:"current"}) as any;
  expect(mock.events).toEqual(["shared"]);expect(served.projects.c.sources.length).toBe(1);
  const fresh=await loadGenerationGraphV1(mock.host,options) as any;
  expect(mock.log).toEqual([{full:["c"],edges:[],cold:false}]);expect(Object.keys(fresh.projects).sort()).toEqual(["a","b","c"]);
  const unknown={kind:"tasks",tasks:[{project:"zz",target:"build"}]};
  await expect(loadGenerationGraphV1(mock.host,{selection:unknown,slice:{selection:unknown,roots:["zz"]}})).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});
 }finally{clean(dir);}
},60000);

test("a lean load names projects, targets and unavailable projects from the manifest alone and expands nothing",async()=>{
 const dir=scratch();
 try{
  const files=ringFiles(12);for(const index of[10,11])files[`pkg/p${index}/project.json`]=JSON.stringify({name:`p${index}`,broken:true});
  await seed(dir,files);const mock=createMockHost(dir);await loadGenerationGraphV1(mock.host);
  mock.events.length=0;mock.expanded.length=0;const lean=await loadGenerationGraphV1(mock.host,{policy:"current",lean:true}) as any;
  expect(mock.events).toEqual(["shared"]);expect(mock.expanded).toEqual([]);expect(lean.lean).toBe(true);expect(Object.keys(lean.projects).sort()).toEqual(Array.from({length:10},(_,i)=>"p"+i).sort());
  expect(lean.unavailable.map((row:any)=>row.project).sort()).toEqual(["p10","p11"]);
 }finally{clean(dir);}
},60000);

test("compact inputs are expanded within the capacity of one capture: text bytes, not UTF-16 units, bound every call and an oversized project is refused",async()=>{
 const reuse=await import("../../../../👁️observation/📥️reuse/🟨️.mjs"),policy=(await import("../../../../👁️observation/📥️reuse/🧬️schema/🔣️.json")).default.definitions.Policy.const;
 const limit=policy.maximumTextBytes,text=(bytes:number)=>"é".repeat(Math.floor(bytes/2));
 const wide=text(limit+2);expect(wide.length).toBeLessThan(limit);expect(Buffer.byteLength(wide)).toBeGreaterThan(limit);
 expect(()=>reuse.captureInferenceInventoryReferencesV1({p:wide})).toThrow("text capacity exceeded");
 const projects:Record<string,unknown>={};for(let index=0;index<5;index++)projects["p"+index]={sources:text(Math.floor(limit/3))};
 const calls:number[]=[],expand=async(chunk:Record<string,unknown>)=>{reuse.captureInferenceInventoryReferencesV1(chunk);calls.push(Object.keys(chunk).length);return chunk;};
 await expect(expand(projects)).rejects.toThrow("text capacity exceeded");calls.length=0;
 const bounded=await expandInChunksV1(projects,expand,{maximumProjects:32,maximumBytes:Math.floor(limit/4)});
 expect(Object.keys(bounded).sort()).toEqual(Object.keys(projects).sort());expect(calls.every(count=>count===1)).toBe(true);
 calls.length=0;const generous=await expandInChunksV1(projects,expand,{maximumProjects:32,maximumBytes:limit*4});expect(Object.keys(generous).length).toBe(5);expect(calls.reduce((sum,count)=>sum+count,0)).toBe(5);expect(Math.max(...calls)).toBeLessThan(5);
 await expect(expandInChunksV1({big:{sources:wide}},expand,{maximumProjects:32,maximumBytes:limit})).rejects.toThrow("text capacity exceeded");
},120000);
'''
    row[0] = t; row[1] = True; print("pending  load tests")
else: print("applied  load tests")

if "--write" in sys.argv:
    for p, (t, ch) in files.items():
        if ch: open(p, "w", encoding="utf8", newline="").write(t)
    print("WRITTEN")
