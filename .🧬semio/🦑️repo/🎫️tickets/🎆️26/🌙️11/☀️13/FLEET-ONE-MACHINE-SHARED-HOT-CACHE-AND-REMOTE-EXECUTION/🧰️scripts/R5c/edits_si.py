LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
SI=LIB+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
COVER='''
const toolTablesV1 = new WeakMap();
/** 🛞️ The declared tool commands of the providers project by the absolute path of each entry module and by name: where a command's script entry is one of them, the command closure names the tool's group instead of walking the script again.
 * @see ./🔌️provider-closures/🔣️.json */
function toolTableV1(scripts) {
  let table = toolTablesV1.get(scripts);
  if (table !== undefined) return table;
  const base = dirname(providerDesignationV1(scripts).path), entries = new Map(), resources = new Map();
  for (const [name, tool] of Object.entries(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0)) {
    resources.set(name, new Set(tool.resources.map(path => resolve(base, path))));
    for (const entry of tool.entries) { const path = resolve(base, entry); if (!entries.has(path)) entries.set(path, name); }
  }
  toolTablesV1.set(scripts, table = { entries, resources });
  return table;
}

/** 🪝️ The script entries of one target that a declared tool command already covers, and the references to name instead of walking them. A tool covers an entry when its entry module is the entry and the command selects no router words (or the tool states that its closure is the whole router); a target that declares module globs, module inputs, runtime entries or a TypeScript project, or a resource no covering tool carries, keeps its whole walk. `packages` asks for tools whose view carries the external packages (a generic command keys on them).
 * @see ./🔌️provider-closures/🟨️.mjs */
function toolCoverV1(scripts, target, entries, words, resources, packages) {
  const none = { covered: [], references: [] }, semio = target.metadata?.semio;
  if (semio?.closure?.modules !== undefined || semio?.closure?.typescript !== undefined || semio?.runtimeSourceEntries !== undefined) return none;
  if (Array.isArray(target.inputs) && target.inputs.some(declaredModuleLiteralV1)) return none;
  const table = toolTableV1(scripts), covered = [], names = new Set();
  for (const entry of entries) {
    const name = table.entries.get(entry);
    if (name === undefined) continue;
    const tool = PROVIDER_CLOSURE_POLICY_V1.tools.closures[name];
    if (packages && !PROVIDER_CLOSURE_POLICY_V1.views[tool.view].packages) continue;
    if (words.get(entry) && tool.selection !== "included") continue;
    covered.push(entry); names.add(name);
  }
  if (!covered.length) return none;
  if (resources.some(path => ![...names].some(name => table.resources.get(name).has(path)))) return none;
  const project = providerProjectV1(scripts);
  return { covered, references: [...names].sort().map(name => toolReferenceV1(project, name)) };
}

const byNameV1 = ([left], [right]) => left < right ? -1 : left > right ? 1 : 0;
/** 🫙️ The files of the group of one declared tool command, once per script input cache.
 * @see ./🔌️provider-closures/🟨️.mjs */
async function toolGroupFilesV1(workspaceRoot, scripts, name) {
  const groups = scripts.toolGroups ??= new Map();
  let files = groups.get(name);
  if (files === undefined) groups.set(name, files = new Set((await toolClosureGroupV1(workspaceRoot, scripts, name)).filter(item => typeof item === "string")));
  return files;
}

/** 🪚️ The command closure of a target with the declared tools named: the references of the entries a tool covers, and for every declared tool whose entry module the walk reached the references to that tool and the closure without the files of the tool's group (the group is keyed through the reference, so the closure never keys on fewer files than the walk). `mode` is the consumer ("native", "generic" or "queries"); undefined keeps the closure as it is.
 * @see ./🔌️provider-closures/🟨️.mjs */
async function toolFinishV1(scripts, workspaceRoot, own, entryCover, mode) {
  if (mode === undefined) return { list: own, covered: [] };
  const references = new Map(entryCover.references.map(row => [JSON.stringify(row), row]));
  let list = own, left = [];
  if (own !== undefined) {
    const present = new Set(own.filter(item => typeof item === "string")), base = dirname(providerDesignationV1(scripts).path), covered = new Set();
    for (const [name, tool] of Object.entries(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort(byNameV1)) {
      if (!tool.entries.some(entry => present.has(closurePathNameV1(scripts, workspaceRoot, resolve(base, entry))))) continue;
      for (const file of await toolGroupFilesV1(workspaceRoot, scripts, name)) covered.add(file);
      const row = toolReferenceV1(providerProjectV1(scripts), name);
      references.set(JSON.stringify(row), row);
    }
    if (covered.size) list = own.filter(item => typeof item !== "string" || !covered.has(item));
    if (covered.size) left = own.filter(item => typeof item === "string" && covered.has(item));
  }
  return { list: references.size ? [...(list ?? []), ...references.values()] : list, covered: left };
}
'''
EDITS=[
(SI,'export async function targetScriptSources(target, workspaceRoot, scripts, projectRoot = ".",nativeManifest) {\n',
 'export async function targetScriptEntriesV1(target, workspaceRoot, projectRoot = ".", nativeManifest) {\n',1),
(SI,' const commands=new Map([...words].filter(([,known])=>known).map(([script,known])=>[script,[...known]])),baseSize=entries.size,',
 ' return {resources,entries,words,fallback};\n}\n'+COVER+'\n/** 📽️ Captures outer scripts and explicitly declared child scripts in their own working directories; with `cover` ("native", "generic" or "queries") the entries a declared tool command covers are named by reference to the tool group and not walked. */\nexport async function targetScriptSources(target, workspaceRoot, scripts, projectRoot = ".",nativeManifest, cover) {\n const {resources,entries,words,fallback}=await targetScriptEntriesV1(target,workspaceRoot,projectRoot,nativeManifest),tools=cover===undefined?{covered:[],references:[]}:toolCoverV1(scripts,target,entries,words,resources,cover==="generic");\n for(const entry of tools.covered)entries.delete(entry);\n const commands=new Map([...words].filter(([,known])=>known).map(([script,known])=>[script,[...known]])),baseSize=entries.size,',1),
(SI,' return {closure:kept,fallback,...(generatedDependencies.length?{generatedDependencies}:{}),',
 ' return {closure:kept,fallback,...(finished.covered.length?{covered:finished.covered}:{}),...(generatedDependencies.length?{generatedDependencies}:{}),',1),
(SI,'  const key=memoDigestV1(closure,sources.fallback?fallback:null,sources.packages??[]);','  const key=memoDigestV1(closure,sources.fallback?fallback:null,sources.packages??[],sources.covered??[]);',1),
(SI,'packages = await closurePackagesV1(closure, workspaceRoot, graph, checkpoint);','packages = await closurePackagesV1([...closure, ...(sources.covered ?? [])], workspaceRoot, graph, checkpoint);',1),
(SI,'''  if (ports.length) for (const path of [designation.path, designation.schema]) files.add(closurePathNameV1(scripts, workspaceRoot, path));
  return [...files].sort();
}''',
 '''  if (ports.length) for (const path of [designation.path, designation.schema]) files.add(closurePathNameV1(scripts, workspaceRoot, path));
  const roster = [...files].sort();
  if (!PROVIDER_CLOSURE_POLICY_V1.views[tool.view].packages) return roster;
  const graph = existsSync(join(workspaceRoot, javascriptManagerV1.primaryLockName)) ? readBunLockGraph(workspaceRoot) : undefined;
  let index = 0;
  return [...roster, { externalDependencies: [...await closurePackagesV1(roster, workspaceRoot, graph, (force = false) => yieldInferenceV1(scripts.context, "build:command-source-packages", ++index, force))].sort() }];
}''',1),
(SI,'pruned=closure&&withoutSchedulingV1(closure),kept=pruned&&references.length?[...pruned,...references]:pruned;',
 'pruned=closure&&withoutSchedulingV1(closure),own=pruned&&references.length?[...pruned,...references]:pruned,finished=await toolFinishV1(scripts,workspaceRoot,own,tools,cover),kept=finished.list;',1),
(SI,'  const designation = providerDesignationV1(scripts), base = dirname(designation.path), walk = await boundedScriptInputsV1(tool.entries.map(entry => resolve(base, entry)), workspaceRoot, scripts), ports',
 '  const designation = providerDesignationV1(scripts), base = dirname(designation.path), inside = path => !closurePathNameV1(scripts, workspaceRoot, path).startsWith("{workspaceRoot}/../"), walk = await boundedScriptInputsV1(tool.entries.map(entry => resolve(base, entry)).filter(inside), workspaceRoot, scripts), ports',1),
(SI,'  for (const resource of tool.resources) { const path = resolve(base, resource); requirePhysicalSourceV1(path);','  for (const resource of tool.resources) { const path = resolve(base, resource); if (!inside(path)) continue; requirePhysicalSourceV1(path);',1),
(SI,'scripts.paths.clear(); scripts.walkNodes?.clear();','scripts.paths.clear(); scripts.walkNodes?.clear(); scripts.toolGroups?.clear();',1),
(SI,'export async function targetScriptClosure(target, workspaceRoot, scripts, projectRoot = ".") {\n  return (await targetScriptSources(target,workspaceRoot,scripts,projectRoot)).closure;',
 'export async function targetScriptClosure(target, workspaceRoot, scripts, projectRoot = ".", cover) {\n  return (await targetScriptSources(target,workspaceRoot,scripts,projectRoot,undefined,cover)).closure;',1),
(SI,'  const closure = await targetScriptClosure(target, workspaceRoot, scripts, projectRoot);\n  return closure ? joinWithProviderViewV1',
 '  const closure = await targetScriptClosure(target, workspaceRoot, scripts, projectRoot, "native");\n  return closure ? joinWithProviderViewV1',1),
(SI,'  const sources = await targetScriptSources(target, workspaceRoot, scripts, projectRoot),closure=sources.closure;',
 '  const sources = await targetScriptSources(target, workspaceRoot, scripts, projectRoot, undefined, "generic"),closure=sources.closure;',1),
(SI,'    case "target-sources": return targetScriptSources(target, workspaceRoot, scripts, recipe.projectRoot, recipe.nativeManifest);',
 '    case "target-sources": return targetScriptSources(target, workspaceRoot, scripts, recipe.projectRoot, recipe.nativeManifest, "queries");',1),
]
DT=LIB+"🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.d.mts"
EDITS+=[
(DT,'export function targetScriptSources(target: ScriptInputsValueV1, workspaceRoot: string, scripts: ScriptInputsValueV1, projectRoot?: string, nativeManifest?: ScriptInputsValueV1): Promise<ScriptInputsValueV1>;',
 'export function targetScriptEntriesV1(target: ScriptInputsValueV1, workspaceRoot: string, projectRoot?: string, nativeManifest?: ScriptInputsValueV1): Promise<{resources: string[]; entries: Set<string>; words: Map<string, Set<string> | null>; fallback: boolean}>;\nexport function targetScriptSources(target: ScriptInputsValueV1, workspaceRoot: string, scripts: ScriptInputsValueV1, projectRoot?: string, nativeManifest?: ScriptInputsValueV1, cover?: "native" | "generic" | "queries"): Promise<ScriptInputsValueV1>;',1),
(DT,'export function targetScriptClosure(target: ScriptInputsValueV1, workspaceRoot: string, scripts: ScriptInputsValueV1, projectRoot?: string): Promise<ScriptInputsValueV1>;',
 'export function targetScriptClosure(target: ScriptInputsValueV1, workspaceRoot: string, scripts: ScriptInputsValueV1, projectRoot?: string, cover?: "native" | "generic" | "queries"): Promise<ScriptInputsValueV1>;',1),
]
