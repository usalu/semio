import os, sys, json, re
ROOT = "/Users/ueli/Documents/semio"
if "--root" in sys.argv: ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
def resolve(rel):
    cur = ROOT
    for part in rel.split("/"):
        hits = [n for n in os.listdir(cur) if n.replace("️", "") == part.replace("️", "")]
        assert len(hits) == 1, (part, hits); cur = os.path.join(cur, hits[0])
    return cur
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B = LIB + "🏗️build/"
GEN = B + "🕸️graph/🕰️generations/"
ENGINE = resolve(GEN + "🟨️.mjs"); ENGINE_T = resolve(GEN + "🧪️tests/🟦️.ts")
LOAD = resolve(GEN + "🔌️load/🟨️.mjs"); LOAD_D = LOAD.replace(".mjs", ".d.mts"); LOAD_T = resolve(GEN + "🔌️load/🧪️tests/🟦️.ts")
WORLD = resolve(GEN + "🧪️tests/🏃️world/🟦️.ts")
G = resolve(B + "🕸️graph/🟦️.ts")
INV = resolve(B + "🧭️invocation/🟦️.ts"); SCRIPT = resolve(B + "📜️script.ts")
PROT_T = resolve(B + "🕸️graph/🧪️tests/🕰️generation-protocol/🟦️.ts"); PROT_J = resolve(B + "🕸️graph/🧪️tests/🕰️generation-protocol/🔣️.json")
files = {}
def get(p):
    if p not in files: files[p] = [open(p, encoding="utf8", newline="").read(), False]
    return files[p]
def sub(p, marker, old, new, label, count=1):
    row = get(p)
    if marker in row[0]: print("applied ", label); return
    assert row[0].count(old) == count, (label, row[0].count(old), old[:80])
    row[0] = row[0].replace(old, new); row[1] = True; print("pending ", label)
def append(p, marker, text, label):
    row = get(p)
    if marker in row[0]: print("applied ", label); return
    row[0] += ("" if row[0].endswith("\n") else "\n") + text; row[1] = True; print("pending ", label)

# 1 engine: a "*" neighbor admits every owner
sub(ENGINE, 'if(target==="*"){', "for(const fragment of loaded)for(const target of(neighbors?neighbors(fragment):fragment.edges.map(edge=>edge.target)))if(manifest.owners[target]&&!seen.has(target))frontier.push(target);",
    'for(const fragment of loaded)for(const target of(neighbors?neighbors(fragment):fragment.edges.map(edge=>edge.target))){if(target==="*"){for(const name of Object.keys(manifest.owners))if(!seen.has(name))frontier.push(name);}else if(manifest.owners[target]&&!seen.has(target))frontier.push(target);}', "engine wildcard")


# 1b engine: a closure build refreshes only the dirty owners of the closure
sub(ENGINE, "found=reuse?hint.plan:await plan({current,identity,changes:previous?.frontier?live.changes:undefined,signal,progress,only})", "found=reuse?hint.plan:await plan({current,identity,changes:previous?.frontier?live.changes:undefined,signal,progress});", "found=reuse?hint.plan:await plan({current,identity,changes:previous?.frontier?live.changes:undefined,signal,progress,only});", "engine scoped plan")
sub(ENGINE, "async function build({identity,implementation,membership,compute,signal,progress,hint,only}){", "async function build({identity,implementation,membership,compute,signal,progress,hint}){", "async function build({identity,implementation,membership,compute,signal,progress,hint,only}){", "engine build only")
sub(ENGINE, "const body=()=>build({identity,implementation,membership,compute,signal,progress,hint,only});", "const body=()=>build({identity,implementation,membership,compute,signal,progress,hint});", "const body=()=>build({identity,implementation,membership,compute,signal,progress,hint,only});", "engine body only")
sub(LOAD_T, 'expect(mock.log[0]!.full).toEqual(["c"]);', 'expect(mock.log[0]!.full).toEqual(["c","e"]);', 'expect(mock.log[0]!.full).toEqual(["c"]);', "load test closure expectation")


# 1c engine: members that enter the closure through a build are verified too
OLD_TAIL = """  const result=await refresh({identity,implementation,policy,only:names,membership,compute,signal,progress,builder});
  if(result.needsBuild)return result;
  const same=names&&result.current.pointer.generation===first.pointer.generation;
  return{...result,names:same?names:await closureOf(result.current,roots,{neighbors,signal})};
"""
NEW_TAIL = """  let verified=names,result=await refresh({identity,implementation,policy,only:names,membership,compute,signal,progress,builder});
  if(result.needsBuild)return result;
  let built=result.built,published=result.published,computed=result.computed;
  for(let round=0;;round++){
   const closure=verified&&result.current.pointer.generation===first.pointer.generation?verified:await closureOf(result.current,roots,{neighbors,signal});
   if(policy==="current"||!verified||round>=8||[...closure].every(name=>verified.has(name)))return{...result,built,published,computed,...(built?{served:"built"}:{}),names:closure};
   verified=new Set([...verified,...closure]);result=await refresh({identity,implementation,policy,only:verified,membership,compute,signal,progress,builder});
   if(result.needsBuild)return result;built=built||result.built;published=published||result.published;computed+=result.computed;
  }
"""
sub(ENGINE, "let verified=names,result=await refresh(", OLD_TAIL, NEW_TAIL, "engine closure rounds")
sub(ENGINE_T, 'expect(world.log.at(-1)!.full).toEqual(["b"]);', 'expect(world.log.at(-1)!.full).toEqual(["b","e"]);', 'expect(world.log.at(-1)!.full).toEqual(["b"]);', "engine test scoped build")

# 2 load module
sub(LOAD, "export function sliceNeighborsV1", "/** 🧭️ Reads the graph policy", "",  "noop") if False else None
row = get(LOAD)
if "export function sliceNeighborsV1" not in row[0]:
    m = re.search(r"/\*\*\n \* [^\n]{0,8}Shapes what one partial inference produced", row[0]); assert m
    row[0] = row[0][:m.start()] + '''/** 🧭️ Names the projects one project reaches in a task closure: its dependency edges, every project named by a `project:target` or object dependency of its targets, and every project (`*`) when a target asks for all of them. */
export function sliceNeighborsV1(fragment){
 const names=new Set(fragment.edges.map(edge=>edge.target)),targets=fragment.kind==="project"?fragment.node?.targets:undefined;
 if(targets&&typeof targets==="object")for(const target of Object.values(targets)){
  for(const dependency of Array.isArray(target?.dependsOn)?target.dependsOn:[]){
   if(typeof dependency==="string"){const at=dependency.lastIndexOf(":");if(!dependency.startsWith("^")&&at>0)names.add(dependency.slice(0,at));}
   else if(dependency&&typeof dependency==="object"&&typeof dependency.target==="string"){const projects=dependency.projects;if(projects==="*")names.add("*");else if(typeof projects==="string")names.add(projects);else if(Array.isArray(projects))for(const project of projects)if(typeof project==="string")names.add(project);}
  }
 }
 return[...names];
}

''' + row[0][m.start():]; row[1] = True; print("pending  slice neighbors")
else: print("applied  slice neighbors")
sub(LOAD, "{policy=\"fresh\",closure,neighbors,selection,requireGeneration}", "export async function loadGenerationGraphV1(host,{policy=\"fresh\",closure}={}){", "export async function loadGenerationGraphV1(host,{policy=\"fresh\",closure,neighbors,selection,requireGeneration}={}){", "load options")
sub(LOAD, "if(!builder&&requireGeneration){", "  const started=Date.now(),result=roots?await engine.refreshClosure({...base,roots}):await engine.refresh(base);",
    "  if(!builder&&requireGeneration){const sealed=await engine.read(host.signal);if(!sealed?.manifest||sealed.manifest.identity!==identity)return{absent:true};}\n  const started=Date.now(),result=roots?await engine.refreshClosure({...base,roots,neighbors}):await engine.refresh(base);", "load require")
sub(LOAD, ",closureDigest=selection?", "  const projects=await host.expand(assembled.projects,control);\n",
    "  const projects=await host.expand(assembled.projects,control),closureDigest=selection?createHash(\"sha256\").update(canonicalJsonV1([assembled.membership,[...result.names].sort()])).digest(\"hex\"):assembled.membership;\n", "load closure digest")
sub(LOAD, "admission:{selection:selection??{kind:\"all\"},registry:host.registryDigest,closure:closureDigest}", "admission:{selection:{kind:\"all\"},registry:host.registryDigest,closure:assembled.membership}", "admission:{selection:selection??{kind:\"all\"},registry:host.registryDigest,closure:closureDigest}", "load admission")
sub(LOAD, " if(first.absent)return undefined;", " if(!first.needsBuild)return first.graph;", " if(first.absent)return undefined;\n if(!first.needsBuild)return first.graph;", "load absent")
sub(LOAD_D, "requireGeneration?:boolean", "export function loadGenerationGraphV1(host:GenerationHostV1,options?:{policy?:GenerationGraphPolicyV1;closure?:readonly string[]}):Promise<unknown>;",
    "export function loadGenerationGraphV1(host:GenerationHostV1,options?:{policy?:GenerationGraphPolicyV1;closure?:readonly string[];neighbors?:(fragment:{kind:string;edges:readonly {target:string}[];node?:unknown})=>readonly string[];selection?:unknown;requireGeneration?:boolean}):Promise<unknown>;\n/** 🧭️ Names the projects one project reaches in a task closure. */\nexport function sliceNeighborsV1(fragment:{kind:string;edges:readonly {target:string}[];node?:unknown}):string[];", "load d.mts")

# 3 graph glue
sub(G, "const SLICE_CHUNK=", "/** 🧩️ Assembles all owned inference contributions", "/** 🧩️ Assembles all owned inference contributions", "noop") if False else None
sub(G, "const expandChunkV1=64,", "    expand:async(projects:Record<string,unknown>,control:ResourceLeaseControl)=>(await portableInputs(root,projects,invocation,control)) as Record<string,unknown>,",
    "    expand:async(projects:Record<string,unknown>,control:ResourceLeaseControl)=>{const expandChunkV1=64,names=Object.keys(projects),result:Record<string,unknown>={};for(let at=0;at<names.length;at+=expandChunkV1){const chunk:Record<string,unknown>={};for(const name of names.slice(at,at+expandChunkV1))chunk[name]=projects[name];Object.assign(result,await portableInputs(root,chunk,invocation,control) as Record<string,unknown>);}return result;},", "expand chunks")
sub(G, "import{selectLiteralBuildProjectsV1}", 'import {readBuildOwnerRegistryV1,readBuildGraphSelectionV1,closeBuildOwnerAdmissionV1,buildGraphScopeKeyV1,type BuildGraphSelectionV1,type BuildOwnerRegistryV1,type BuildOwnerAdmissionV1} from "./🚪️admission/🟦️.ts";',
    'import {readBuildOwnerRegistryV1,readBuildGraphSelectionV1,closeBuildOwnerAdmissionV1,buildGraphScopeKeyV1,type BuildGraphSelectionV1,type BuildOwnerRegistryV1,type BuildOwnerAdmissionV1} from "./🚪️admission/🟦️.ts";\nimport{selectLiteralBuildProjectsV1}from"../🧭️selection/📥️options/🟦️.ts";', "G import")
sub(G, "loadGenerationGraphV1,renderGenerationMessageV1,sliceNeighborsV1", "import{loadGenerationGraphV1,renderGenerationMessageV1}from", "import{loadGenerationGraphV1,renderGenerationMessageV1,sliceNeighborsV1}from", "G load import")
OLD_WRAP = '''  if(options.selection?.kind==="all"&&!options.baseline&&!options.reuse&&invocation){clearPinnedAuthorityV1();return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));}
  return loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options);
}
'''
NEW_WRAP = '''  if(options.selection?.kind==="all"&&!options.baseline&&!options.reuse&&invocation){clearPinnedAuthorityV1();return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));}
  if(options.selection?.kind&&options.selection.kind!=="all"&&!options.baseline&&!options.reuse&&invocation&&process.env.SEMIO_BUILD_GRAPH_SLICE!=="off"){
    clearPinnedAuthorityV1();
    return withImplementationPinV1(async()=>{
      const sliced=await loadBuildGraphSliceV1(root,invocation,phaseControl,options);
      if(!sliced)return loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options);
      if(process.env.SEMIO_BUILD_GRAPH_VERIFY==="1")requireSelectedSubgraphV1(sliced,await loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));
      return sliced;
    });
  }
  return loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options);
}

/** 🍰️ Derives the graph of a selected scope (tasks or literal owners) from the newest sealed generation: the closure of the selected projects is read, verified and, under a fresh policy, refreshed incrementally; without a usable generation the caller infers the scope itself. */
async function loadBuildGraphSliceV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1|undefined> {
  const producer=await import("./📥️inference/🟨️.mjs"),registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(root))),selection=readBuildGraphSelectionV1(options.selection);
  if(selection.kind==="all")return undefined;
  const roots=selection.kind==="tasks"?[...new Set(selection.tasks.map(task=>task.project))]:selectLiteralBuildProjectsV1(registry.owners,selection.projects,selection.exclude);
  try{return await loadGenerationGraphV1(generationHostV1(root,invocation,phaseControl,registry),{policy:options.graph?.policy??"fresh",closure:roots,neighbors:sliceNeighborsV1,selection,requireGeneration:true}) as BuildGraphV1|undefined;}
  catch(error){if(["GENERATION_UNKNOWN_OWNER","GENERATION_EMPTY"].includes((error as {code?:string}).code??""))return undefined;throw error;}
}

/** 🔍️ Names the first difference between a generation slice and the selected graph a full selected inference produced: every selected project is equal and every selected dependency is present; the oracle of `SEMIO_BUILD_GRAPH_VERIFY=1` for selected scopes. */
function requireSelectedSubgraphV1(slice:BuildGraphV1,selected:BuildGraphV1):void{
  const same=(left:unknown,right:unknown)=>canonicalGenerationJsonV1(left)===canonicalGenerationJsonV1(right);
  for(const[name,project]of Object.entries(selected.projects)){if(!slice.projects[name])throw Error(`Graph slice lacks the selected project ${name}`);if(!same(slice.projects[name],project))throw Error(`Graph slice differs from the selected graph at project ${name}`);}
  const have=new Set(slice.dependencies.map(edge=>canonicalGenerationJsonV1(edge)));
  for(const edge of selected.dependencies)if(!have.has(canonicalGenerationJsonV1(edge)))throw Error("Graph slice lacks a selected dependency: "+canonicalGenerationJsonV1(edge));
  for(const[key,node]of Object.entries(selected.externalNodes??{}))if(!same(slice.externalNodes?.[key],node))throw Error(`Graph slice differs from the selected graph at external node ${key}`);
}
'''
sub(G, "async function loadBuildGraphSliceV1", OLD_WRAP, NEW_WRAP, "G wrapper")

# 4 invocation policy for run
sub(INV, "graphPolicy?:\"fresh\"|\"current\"):Promise<BuildGraphV1>{", "read?:(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse:boolean})=>Promise<BuildGraphV1>):Promise<BuildGraphV1>{",
    "read?:(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse:boolean;graph?:{policy?:\"fresh\"|\"current\"}})=>Promise<BuildGraphV1>,graphPolicy?:\"fresh\"|\"current\"):Promise<BuildGraphV1>{", "invocation signature")
sub(INV, "reuse:false,...(graphPolicy?", "const graph=await load(root,current,phaseControl,{selection,reuse:false});", "const graph=await load(root,current,phaseControl,{selection,reuse:false,...(graphPolicy?{graph:{policy:graphPolicy}}:{})});", "invocation nested")
sub(INV, "selection,reuse,...(graphPolicy?", " const graph=await load(root,current,phaseControl,{selection,reuse});", " const graph=await load(root,current,phaseControl,{selection,reuse,...(graphPolicy?{graph:{policy:graphPolicy}}:{})});", "invocation top")
sub(SCRIPT, 'process.env.SEMIO_BUILD_REUSE_GRAPH==="true",undefined,selected.graphPolicy)', 'const baseline=await loadBuildInvocationGraphV1(root,resolved.args,parent,invocation,phaseControl,process.env.SEMIO_BUILD_REUSE_GRAPH==="true")',
    'const baseline=await loadBuildInvocationGraphV1(root,resolved.args,parent,invocation,phaseControl,process.env.SEMIO_BUILD_REUSE_GRAPH==="true",undefined,selected.graphPolicy)', "script policy")

# 5 protocol laws
def pj(data):
    names = {r["name"] for r in data["present"]}
    add = [("a selected scope is derived from the generation before it is inferred", "loadBuildGraphV1", "loadBuildGraphSliceV1(root,invocation,phaseControl,options)"),
           ("the host expands the compact inputs in bounded chunks", "generationHostV1", "const expandChunkV1=64")]
    for name, fn, text in add:
        if name not in names: data["present"].append({"name": name, "function": fn, "text": text})
row = get(PROT_J); data = json.loads(row[0]); n0 = len(data["present"]); pj(data)
if len(data["present"]) != n0: row[0] = json.dumps(data, ensure_ascii=False, indent=1) + "\n"; row[1] = True; print("pending  protocol json")

# 6 tests: engine wildcard
append(ENGINE_T, "a star neighbor admits every owner", r'''
test("a star neighbor admits every owner of the generation and an unknown root is refused",async()=>{
 const memory=createMemoryFilePort(),membership=async()=>({digest:sha("m"),frontier:createFrontierV1({directories:[],members:[],capturedAt:1}),changes:[] as never[]});
 const compute=async()=>({owners:["a","b","c","d"].map(name=>({name,root:name,kind:"project" as const,node:{name,root:name,targets:{}},edges:name==="d"?[{source:"d",target:"a"}]:[],targets:[],rows:[]})),env:{digest:sha("env"),rows:[]},global:{},replaceTags:[]});
 const store=createGenerationStoreV1({root:"/memory/gen",fs:memory.port,resources:{facts:"f"},holds:async()=>true}),built=await store.refresh({identity:IDENTITY,implementation:IMPLEMENTATION,membership,compute:compute as never});
 const neighbors=(fragment:{name?:string})=>fragment.name==="b"?["*"]:[];
 expect([...await store.closureOf(built.current,["b"],{neighbors:(fragment:any)=>neighbors({name:fragment.name})})].sort()).toEqual(["a","b","c","d"]);
 expect([...await store.closureOf(built.current,["a"],{neighbors:(fragment:any)=>neighbors({name:fragment.name})})].sort()).toEqual(["a"]);
 expect([...await store.closureOf(built.current,["d"])].sort()).toEqual(["a","d"]);
 await expect(store.closureOf(built.current,["zz"])).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});
});
''', "engine test")
if True:
    row = get(ENGINE_T)
    if "implementation:IMPLEMENTATION,membership,compute:compute as never});\n const neighbors" not in row[0]: pass


append(ENGINE_T, "enters the closure through a build is verified", r"""
test("a project that enters the closure through a build is verified in the same call",async()=>{
 const dir=scratch();
 try{
  const files:Record<string,string>={};
  for(const[name,imports]of[["a",["@syn/b"]],["b",[]],["f",[]],["g",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";
  await seed(dir,files);const world=createWorld(dir);await world.refresh();
  await applyOps(dir,[{op:"write",path:"pkg/b/src/index.ts",content:'import "@syn/f";\nexport const v=2;\n'},{op:"write",path:"pkg/f/src/index.ts",content:"export const v=9;\n"},{op:"write",path:"pkg/g/src/index.ts",content:"export const v=7;\n"}]);
  const result=await world.refreshClosure(["a"]);
  expect(result.built).toBe(true);expect([...result.names].sort()).toEqual(["a","b","f"]);expect(world.log.map(row=>row.full)).toEqual([["a","b","f","g"],["b"],["f"]]);
  const graph:any=await world.graph(result.current,[...result.names]);expect(graph.projects.f.sources[0].sha256).toBe(sha("export const v=9;\n"));
 }finally{clean(dir);}
},30000);
""", "engine closure entering test")

# 7 tests: load module
sub(LOAD_T, "sliceNeighborsV1,", "implementationDigestV1,loadGenerationGraphV1,renderGenerationMessageV1,scopeImplementationRowsV1,", "implementationDigestV1,loadGenerationGraphV1,renderGenerationMessageV1,scopeImplementationRowsV1,sliceNeighborsV1,", "load test import")
append(LOAD_T, "slice neighbors are the independent model", r'''
for(const[index,row]of(fixture as any).neighbors.entries())test("slice neighbors are the independent model "+index,()=>{expect([...sliceNeighborsV1(row.fragment as never)].sort()).toEqual(row.expected);});

const sliceTree=()=>{
 const files:Record<string,string>={};
 for(const[name,imports]of[["a",["@syn/b"]],["b",["@syn/c"]],["c",[]],["d",["@syn/e"]],["e",[]],["f",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";
 return files;
};

test("a selected scope is the closure of its projects read from the generation: absent without one, current serves it, fresh refreshes only the dirty closure, the admission names the selection",async()=>{
 const dir=scratch(),selection={kind:"tasks",tasks:[{project:"a",target:"build"}]};
 try{
  await seed(dir,sliceTree());const mock=createMockHost(dir);
  expect(await loadGenerationGraphV1(mock.host,{closure:["a"],selection,requireGeneration:true,neighbors:sliceNeighborsV1})).toBeUndefined();expect(mock.events).toEqual(["shared"]);expect(mock.log).toEqual([]);
  await loadGenerationGraphV1(mock.host);
  mock.events.length=0;const graph=await loadGenerationGraphV1(mock.host,{closure:["a"],selection,requireGeneration:true,neighbors:sliceNeighborsV1,policy:"current"}) as any;
  expect(mock.events).toEqual(["shared"]);expect(Object.keys(graph.projects).sort()).toEqual(["a","b","c"]);expect(graph.admission.selection).toEqual(selection);expect(graph.admission.registry).toBe(mock.host.registryDigest);expect(graph.admission.closure).toMatch(/^[a-f0-9]{64}$/u);
  await applyOps(dir,[{op:"write",path:"pkg/f/src/index.ts",content:"export const other=1;\n"},{op:"write",path:"pkg/c/src/index.ts",content:"export const v=2;\n"}]);
  mock.events.length=0;mock.log.length=0;const served=await loadGenerationGraphV1(mock.host,{closure:["a"],selection,requireGeneration:true,neighbors:sliceNeighborsV1,policy:"current"}) as any;
  expect(mock.events).toEqual(["shared"]);expect(served.projects.c.sources.length).toBe(1);
  const fresh=await loadGenerationGraphV1(mock.host,{closure:["a"],selection,requireGeneration:true,neighbors:sliceNeighborsV1}) as any;
  expect(mock.log).toEqual([{full:["c"],edges:[],cold:false}]);expect(Object.keys(fresh.projects).sort()).toEqual(["a","b","c"]);
  await expect(loadGenerationGraphV1(mock.host,{closure:["zz"],selection,requireGeneration:true,neighbors:sliceNeighborsV1})).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});
  const other=createMockHost(dir,{identity:"f".repeat(64)});expect(await loadGenerationGraphV1(other.host,{closure:["a"],selection,requireGeneration:true,neighbors:sliceNeighborsV1,policy:"current"})).toBeUndefined();expect(other.events).toEqual(["shared"]);
 }finally{clean(dir);}
},60000);
''', "load tests")

if "--write" in sys.argv:
    for p, (t, ch) in files.items():
        if ch: open(p, "w", encoding="utf8", newline="").write(t)
    print("WRITTEN")
