import os, sys, json, re
ROOT = "/Users/ueli/Documents/semio"
if "--root" in sys.argv: ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
def resolve(rel):
    cur = ROOT
    for part in rel.split("/"):
        hits = [n for n in os.listdir(cur) if n.replace("️", "") == part.replace("️", "")]
        assert len(hits) == 1, (part, hits); cur = os.path.join(cur, hits[0])
    return cur
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
GEN = B + "🕸️graph/🕰️generations/"
ENGINE = resolve(GEN + "🟨️.mjs"); ENGINE_D = ENGINE.replace(".mjs", ".d.mts")
LOAD = resolve(GEN + "🔌️load/🟨️.mjs"); LOAD_D = LOAD.replace(".mjs", ".d.mts"); LOAD_T = resolve(GEN + "🔌️load/🧪️tests/🟦️.ts")
G = resolve(B + "🕸️graph/🟦️.ts"); SCRIPT = resolve(B + "📜️script.ts")
PJ = resolve(B + "🕸️graph/🧪️tests/🕰️generation-protocol/🔣️.json")
files = {}
def get(p):
    if p not in files: files[p] = [open(p, encoding="utf8", newline="").read(), False]
    return files[p]
def sub(p, marker, old, new, label, count=1):
    row = get(p)
    if marker in row[0]: print("applied ", label); return
    assert row[0].count(old) == count, (label, row[0].count(old), old[:90])
    row[0] = row[0].replace(old, new); row[1] = True; print("pending ", label)

# engine: an explicit closure function replaces roots/neighbors
sub(ENGINE, "closure,identity,implementation,policy=\"fresh\"", "async function refreshClosure({roots,neighbors,identity,implementation,policy=\"fresh\",membership,compute,signal,progress,builder}){",
    "async function refreshClosure({roots,neighbors,closure,identity,implementation,policy=\"fresh\",membership,compute,signal,progress,builder}){\n  const closeOf=current=>closure?closure(current,{signal}):closureOf(current,roots,{neighbors,signal});", "engine closure option")
sub(ENGINE, "try{names=await closeOf(first);}", "try{names=await closureOf(first,roots,{neighbors,signal});}", "try{names=await closeOf(first);}", "engine closeOf first")
sub(ENGINE, "await closeOf(result.current);", "await closureOf(result.current,roots,{neighbors,signal});", "await closeOf(result.current);", "engine closeOf rounds")
sub(ENGINE_D, "closure?:(current", "export interface GenerationRefreshOptionsV1{identity:string;", "export interface GenerationRefreshOptionsV1{closure?:(current:GenerationSealedV1,control:{signal?:AbortSignal})=>Promise<ReadonlySet<string>>;identity:string;", "engine d.mts")

# load module: task closure + lean
row = get(LOAD)
if "export async function sliceClosureV1" not in row[0]:
    m = re.search(r"/\*\*\n \* [^\n]{0,8}Shapes what one partial inference produced", row[0]); assert m
    row[0] = row[0][:m.start()] + '''/**
 * 🍰️ The owners a selected scope needs, as a fixpoint over the generation: every admitted owner admits the owners it imports (its dependency edges, transitively); only the active targets follow their `dependsOn` (`^target` over the dependency edges, `project:target`, `target`, objects with `projects` and `dependencies`, `*` for the active target only). Catalog provider owners join when an active owner lies below one of their consumer roots.
 * `owners` maps every owner name to its root; `fragment(name)` yields `{edges, node}`; an unknown name refuses (`GENERATION_UNKNOWN_OWNER`), because the caller then infers the scope itself.
 */
export async function sliceClosureV1({owners,fragment,selection,roots=[],providers=[]}){
 const cache=new Map(),known=new Set(Object.keys(owners)),admitted=new Set(),active=new Map(),queue=[];
 const get=async name=>{let row=cache.get(name);if(!row){row=await fragment(name);cache.set(name,row);}return row;};
 const unknown=name=>Object.assign(Error("Unknown project in the closure: "+name),{code:"GENERATION_UNKNOWN_OWNER"});
 const admit=async name=>{
  const todo=[name];
  while(todo.length){const at=todo.pop();if(admitted.has(at))continue;if(!known.has(at)){if(at===name)throw unknown(at);continue;}admitted.add(at);for(const edge of(await get(at)).edges)todo.push(edge.target);}
 };
 const task=async(name,target,optional)=>{
  if(!known.has(name))throw unknown(name);await admit(name);
  if(optional&&!(await get(name)).node?.targets?.[target])return;
  const set=active.get(name)??new Set();if(!set.has(target)){set.add(target);active.set(name,set);queue.push([name,target]);}
 };
 if(selection.kind==="tasks")for(const row of selection.tasks)await task(row.project,row.target,false);else for(const name of roots)await admit(name);
 while(queue.length){
  const[name,target]=queue.shift(),row=await get(name),operation=row.node?.targets?.[target];if(!operation)continue;
  const dependencies=[...new Set(row.edges.map(edge=>edge.target))].filter(value=>known.has(value));
  for(const dependency of Array.isArray(operation.dependsOn)?operation.dependsOn:[]){
   if(typeof dependency==="string"){
    if(dependency.startsWith("^"))for(const value of dependencies)await task(value,dependency.slice(1),true);
    else if(dependency.includes(":")){const at=dependency.lastIndexOf(":");await task(dependency.slice(0,at),dependency.slice(at+1),false);}
    else await task(name,dependency,false);
   }else if(dependency&&typeof dependency==="object"&&typeof dependency.target==="string"){
    const projects=dependency.dependencies?dependencies:dependency.projects===undefined?[name]:dependency.projects==="*"?[...known]:typeof dependency.projects==="string"?[dependency.projects]:dependency.projects;
    for(const value of projects)await task(value,dependency.target,dependency.dependencies===true);
   }
  }
 }
 const inside=(root,path)=>root==="."||path===root||path.startsWith(root+"/");
 for(const provider of providers){
  if(![...active.keys()].some(name=>provider.consumerRoots.some(root=>inside(root,owners[name]))))continue;
  let best;for(const[name,root]of Object.entries(owners))if(inside(root,provider.sourcePath)&&(best===undefined||root.length>owners[best].length))best=name;
  if(best!==undefined)await admit(best);
 }
 return admitted;
}

''' + row[0][m.start():]; row[1] = True; print("pending  slice closure")
else: print("applied  slice closure")
sub(LOAD, "{policy=\"fresh\",closure,neighbors,selection,requireGeneration,slice,lean}", "export async function loadGenerationGraphV1(host,{policy=\"fresh\",closure,neighbors,selection,requireGeneration}={}){", "export async function loadGenerationGraphV1(host,{policy=\"fresh\",closure,neighbors,selection,requireGeneration,slice,lean}={}){", "load options 2")
sub(LOAD, "roots=slice?[...new Set(slice.roots)]", "roots=closure?[...new Set(closure)]:undefined,", "roots=slice?[...new Set(slice.roots)]:closure?[...new Set(closure)]:undefined,", "load roots")
sub(LOAD, "roots?await engine.refreshClosure({...base,roots,neighbors,closure:closeFn})", "roots?await engine.refreshClosure({...base,roots,neighbors})", "roots?await engine.refreshClosure({...base,roots,neighbors,closure:closeFn})", "load refresh closure")
sub(LOAD, "const closeFn=slice?", "  if(!builder&&requireGeneration){", "  const closeFn=slice?(current,{signal})=>sliceClosureV1({owners:Object.fromEntries(Object.entries(current.manifest.owners).map(([name,row])=>[name,row.root])),fragment:name=>engine.readFragment(current.manifest.owners[name]).then(value=>{if(!value)throw Object.assign(Error(\"Fragment of \"+name+\" is missing\"),{code:\"GENERATION_INCOMPLETE\"});return value;}),selection:slice.selection,roots:slice.roots,providers:slice.providers??[]}):undefined;\n  if(!builder&&requireGeneration){", "load closeFn")
sub(LOAD, "if(lean){", "  const assembled=await engine.assemble(", "  if(lean){const manifest=result.current.manifest,leanProjects=Object.fromEntries(Object.entries(manifest.owners).filter(([,row])=>row.kind===\"project\").map(([name,row])=>[name,{name,root:row.root,targets:Object.fromEntries(row.targets.map(target=>[target,{}]))}]));host.say(\"verified\",{generation:manifest.generation,checked:result.plan?.checked??0});return{graph:{version:1,projects:leanProjects,dependencies:[],lean:true},result};}\n  const assembled=await engine.assemble(", "load lean")
sub(LOAD_D, "slice?:{selection", "requireGeneration?:boolean}):Promise<unknown>;", "requireGeneration?:boolean;slice?:{selection:unknown;roots:readonly string[];providers?:readonly {consumerRoots:readonly string[];sourcePath:string}[]};lean?:boolean}):Promise<unknown>;\n/** 🍰️ The owners a selected scope needs. */\nexport function sliceClosureV1(input:{owners:Readonly<Record<string,string>>;fragment:(name:string)=>Promise<{edges:readonly {target:string}[];node?:unknown}>;selection:{kind:string;tasks?:readonly {project:string;target:string}[]};roots?:readonly string[];providers?:readonly {consumerRoots:readonly string[];sourcePath:string}[]}):Promise<Set<string>>;", "load d.mts 2")

# graph glue: the slice uses the task closure; lean names
sub(G, "slice:{selection,roots,providers", "try{return await loadGenerationGraphV1(generationHostV1(root,invocation,phaseControl,registry),{policy:options.graph?.policy??\"fresh\",closure:roots,neighbors:sliceNeighborsV1,selection,requireGeneration:true}) as BuildGraphV1|undefined;}",
    "try{return await loadGenerationGraphV1(generationHostV1(root,invocation,phaseControl,registry),{policy:options.graph?.policy??\"fresh\",selection,requireGeneration:true,slice:{selection,roots,providers:registry.catalogProviders.map(({consumerRoots,sourcePath})=>({consumerRoots,sourcePath}))}}) as BuildGraphV1|undefined;}", "G slice options")
row = get(G)
if "lean:options.graph?.lean" not in row[0]:
    old = "{policy:options.graph?.policy??\"fresh\",closure:options.graph?.closure})"
    if row[0].count(old) == 1: row[0] = row[0].replace(old, "{policy:options.graph?.policy??\"fresh\",closure:options.graph?.closure,lean:options.graph?.lean})"); row[1] = True; print("pending  G lean pass")
    else: print("DRIFT G lean pass", row[0].count(old))
else: print("applied  G lean pass")
sub(G, "closure?:readonly string[];lean?:boolean}", "graph?:{policy?:\"fresh\"|\"current\";closure?:readonly string[]}}", "graph?:{policy?:\"fresh\"|\"current\";closure?:readonly string[];lean?:boolean}}", "G options type", count=1)
sub(SCRIPT, "?{lean:true}:{})}", "...(command===\"show\"&&resolved.args[1]===\"project\"&&resolved.args[2]?{closure:[resolved.args[2]]}:{})}", "...(command===\"show\"&&resolved.args[1]===\"project\"&&resolved.args[2]?{closure:[resolved.args[2]]}:{}),...(command===\"show\"&&(resolved.args[1]===\"projects\"||resolved.args[1]===\"targets\")?{lean:true}:{})}", "script lean")


sub(LOAD_T, "sliceClosureV1,", "scopeImplementationRowsV1,sliceNeighborsV1,", "scopeImplementationRowsV1,sliceClosureV1,sliceNeighborsV1,", "load test import 2")
row = get(LOAD_T)
if "selected closure is the independent model" not in row[0]:
    row[0] += r"""
for(const[index,row]of(fixture as any).closures.entries())test("selected closure is the independent model "+index,async()=>{
 const owners=Object.fromEntries(Object.entries(row.owners).map(([name,value]:[string,any])=>[name,value.root])),fragments=row.owners as Record<string,any>;
 const run=()=>sliceClosureV1({owners,fragment:async(name:string)=>({edges:fragments[name].edges.map((target:string)=>({source:name,target})),node:{targets:fragments[name].targets}}),selection:row.selection as never,roots:row.roots,providers:row.providers??[]});
 if(row.expected.error)await expect(run()).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});else expect([...await run()].sort()).toEqual(row.expected.admitted);
});

test("a task slice reads its closure from the generation, a lean load names projects and targets from the manifest alone",async()=>{
 const dir=scratch(),selection={kind:"tasks",tasks:[{project:"a",target:"build"}]};
 try{
  await seed(dir,sliceTree());const mock=createMockHost(dir);
  expect(await loadGenerationGraphV1(mock.host,{selection,requireGeneration:true,slice:{selection,roots:["a"]}})).toBeUndefined();
  await loadGenerationGraphV1(mock.host);
  mock.events.length=0;mock.expanded.length=0;const graph=await loadGenerationGraphV1(mock.host,{policy:"current",selection,requireGeneration:true,slice:{selection,roots:["a"]}}) as any;
  expect(mock.events).toEqual(["shared"]);expect(Object.keys(graph.projects).sort()).toEqual(["a","b","c"]);expect(graph.admission.selection).toEqual(selection);expect(mock.expanded).toEqual([["a","b","c"]]);
  mock.events.length=0;mock.expanded.length=0;const lean=await loadGenerationGraphV1(mock.host,{policy:"current",lean:true}) as any;
  expect(mock.events).toEqual(["shared"]);expect(mock.expanded).toEqual([]);expect(lean.lean).toBe(true);expect(Object.keys(lean.projects).sort()).toEqual(["a","b","c","d","e","f"]);expect(lean.projects.a.targets).toEqual({});
  await expect(loadGenerationGraphV1(mock.host,{selection:{kind:"tasks",tasks:[{project:"zz",target:"build"}]},requireGeneration:true,slice:{selection:{kind:"tasks",tasks:[{project:"zz",target:"build"}]},roots:["zz"]}})).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});
 }finally{clean(dir);}
},60000);
"""
    row[1] = True; print("pending  load tests 2")

if "--write" in sys.argv:
    for p, (t, ch) in files.items():
        if ch: open(p, "w", encoding="utf8", newline="").write(t)
    print("WRITTEN")
