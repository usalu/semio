I = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/"
SI, SD = I + "🧭️script-inputs/🟨️.mjs", I + "🧭️script-inputs/🟨️.d.mts"
OD = I + "🔎️owner-discovery/🟨️.mjs"
CI = I + "🧪️tests/🎓️course-inputs/🟦️.ts"
SDM = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts"
EDITS = [
(SI, '''  const expanded = async value => { const references = sharedReferencesOfV1(value); if (!references.length) return value; const files = []; for (const reference of references) for (const item of await produceClosureRecipeV1(reference.recipe, workspaceRoot, scripts)) if (typeof item === "string" && item.startsWith("{workspaceRoot}/")) files.push(item); return expandClosureValueV1(value, files); };''', '''  const expanded = value => expandedCommandInputsV1(value, workspaceRoot, scripts);'''),
(SI, '''/** 🧭️ The command walk of `entries` for a caller that is no project inference''', '''/** 🧩️ A command input list (or a target-sources value) with every shared reference (a script provider or tool closure of the providers project, `./🔌️provider-closures`) replaced by the files of its group: what a consumer that needs the literal files of a command closure reads (a read oracle, a law, a query) instead of the references a node holds. */
export async function expandedCommandInputsV1(value, workspaceRoot, scripts = createScriptInputCache()) {
  const references = sharedReferencesOfV1(value);
  if (!references.length) return value;
  const files = [];
  for (const reference of references) for (const item of await produceClosureRecipeV1(reference.recipe, workspaceRoot, scripts)) if (typeof item === "string" && item.startsWith("{workspaceRoot}/")) files.push(item);
  return expandClosureValueV1(value, files);
}

/** 🧭️ The command walk of `entries` for a caller that is no project inference'''),
(SD, '''export function resolveCommandWalkV1(''', '''export function expandedCommandInputsV1<T>(value: T, workspaceRoot: string, scripts?: ScriptInputsValueV1): Promise<T>;
export function resolveCommandWalkV1('''),
(OD, '''commandResources.set(name,new Set((sources.closure??[]).filter(path=>!compiled.has(path)).map(absolute)));
  for(const input of sources.closure??[])if(typeof input==="string"&&input.startsWith("{workspaceRoot}/"))addFile(input.slice("{workspaceRoot}/".length));''', '''commandResources.set(name,new Set((sources.closure??[]).filter(path=>typeof path==="string"&&!compiled.has(path)).map(absolute)));
  for(const input of sources.closure??[]){if(typeof input==="string"){if(input.startsWith("{workspaceRoot}/"))addFile(input.slice("{workspaceRoot}/".length));}else if(typeof input?.input==="string"&&Array.isArray(input.projects))for(const project of input.projects)addInput(project,input.input);}'''),
(CI, '''  const target=manifest.targets[name],sources=await cacheInternals.targetScriptSources(target,workspace,createScriptInputCache(),fixture.project);
  const declared=declaredInputs(target.inputs??["default"]),inferred=new Set(strip(sources.closure??[])),reads=await readsOf(spec as any);
  results[name]={closure:sources.closure??[],declared,reads};''', '''  const target=manifest.targets[name],cache=createScriptInputCache(),sources=await cacheInternals.targetScriptSources(target,workspace,cache,fixture.project),closure=await expandedCommandInputsV1(sources.closure??[],workspace,cache) as string[];
  const declared=declaredInputs(target.inputs??["default"]),inferred=new Set(strip(closure)),reads=await readsOf(spec as any);
  results[name]={closure,declared,reads};'''),
(CI, ''' const sources=await cacheInternals.targetScriptSources({...target,metadata:undefined},workspace,createScriptInputCache(),fixture.project);
 const inferred=new Set(strip(sources.closure??[])),''', ''' const cache=createScriptInputCache(),sources=await cacheInternals.targetScriptSources({...target,metadata:undefined},workspace,cache,fixture.project);
 const inferred=new Set(strip(await expandedCommandInputsV1(sources.closure??[],workspace,cache) as string[])),'''),
(CI, '''const inference=await import("../../🟨️.mjs"),{cacheInternals,createScriptInputCache,libraryBootstrap}=inference as any;''', '''const inference=await import("../../🟨️.mjs"),{cacheInternals,createScriptInputCache,libraryBootstrap}=inference as any,{expandedCommandInputsV1}=await import("../../🧭️script-inputs/🟨️.mjs") as any;'''),
(SDM, '''const scripts={declared:new Map(),compiledNames:new WeakMap(),moduleSets:new WeakMap(),generatedImports:new Map()},ports={''', '''const scripts={declared:new Map(),compiledNames:new WeakMap(),walkPorts:new WeakMap(),moduleSets:new WeakMap(),generatedImports:new Map()},ports={providerReferencesV1:()=>[],'''),
(SDM, '''memoizedObservedComputationV1:(cache:Map<string,unknown>,key:string,body:()=>unknown)=>{if(!cache.has(key))cache.set(key,{value:body(),observations:[]});return (cache.get(key) as {value:unknown}).value;},relativeScriptInputs:async(entries:string[],_root:string,cache:any)=>{''', '''memoizedObservedComputationV1:(cache:Map<string,unknown>,key:string,body:()=>unknown)=>{if(!cache.has(key))cache.set(key,{value:body(),observations:[]});return (cache.get(key) as {value:unknown}).value;},boundedScriptInputsV1:async(entries:string[],_root:string,cache:any)=>{'''),
]
