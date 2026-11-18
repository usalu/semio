#!/usr/bin/env python3
"""R100 stage builder: base = the live script-inputs module right now, new = base with anchored replacements (every anchor must occur exactly once). usage: mk.py STAGE_DIR"""
import os,sys,hashlib
R="/Users/ueli/Documents/semio"
SI=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
S=sys.argv[1];os.makedirs(S,exist_ok=True)
base=open(SI,encoding="utf8").read();new=base
def edit(old,rep):
    global new
    assert new.count(old)==1,(old[:90],new.count(old))
    new=new.replace(old,rep)
edit("/** 🌳️ The command walk over graph nodes: depth first over `(path, source)` pairs, the same visiting rules as the inline walk, every node's rows merged by reference into the closure's scope. */","/** 🌳️ The command walk over graph nodes: depth first over `(path, source)` pairs, the same visiting rules as the inline walk, every node's rows merged by reference into the closure's scope; a module that is the entry of a declared tool in `stops` (path to tool name) and no root of the walk is not visited, the walk names the tool in `tools` instead and none of the tool's rows is a row of the walk. */")
edit("walkPorts:new WeakMap(),","walkPorts:new WeakMap(),walkTools:new WeakMap(),")
edit("resourceEntries=[], commands=new Map()) {\n  const resources=new Set(","resourceEntries=[], commands=new Map(), stops=[]) {\n  const resources=new Set(")
edit("roots = [...new Set(entries.map(entry => resolve(entry)))].sort(), key = memoDigestV1(resolve(workspaceRoot), roots, requiredSources,resourceKey,selectorKey);","roots = [...new Set(entries.map(entry => resolve(entry)))].sort(), stopMap = new Map(stops), key = memoDigestV1(resolve(workspaceRoot), roots, requiredSources,resourceKey,selectorKey,...(stops.length?[stopDigestV1(stops)]:[]));")
edit("const walk = await reachWalkV1({ roots, workspaceRoot, cache, requiredSources, resources, selectors, checkpoint, compiler, host });","const walk = await reachWalkV1({ roots, workspaceRoot, cache, requiredSources, resources, selectors, checkpoint, compiler, host, stops: stopMap });")
edit("  cache.walkPorts.set(walk.result, [...walk.defined].sort());\n  memberParts = walk.members;","  cache.walkPorts.set(walk.result, [...walk.defined].sort());\n  cache.walkTools.set(walk.result, [...walk.tools].sort());\n  memberParts = walk.members;")
edit("const question=walkQuestionV1(cache,workspaceRoot,roots,requiredSources,resources,selectors);","const question=walkQuestionV1(cache,workspaceRoot,roots,requiredSources,resources,selectors,undefined,stops);")
edit("walkQuestionV1(cache,workspaceRoot,roots,requiredSources,resources,selectors,view),fact:","walkQuestionV1(cache,workspaceRoot,roots,requiredSources,resources,selectors,view,stops),fact:")
edit("async function reachWalkV1({ roots, workspaceRoot, cache, requiredSources, resources, selectors, checkpoint, compiler, host }) {","async function reachWalkV1({ roots, workspaceRoot, cache, requiredSources, resources, selectors, checkpoint, compiler, host, stops }) {")
edit("seen = new Set(), stack = roots.map((path) => [path, true]), directories = new Set(), members = new Set();","seen = new Set(), stack = roots.map((path) => [path, true]), directories = new Set(), members = new Set(), crossed = new Set();")
edit("      if (++steps % 64 === 0) await checkpoint();\n      const node = await walkNodeV1(env, path, source);\n      for (const part of node.parts) { retainObservedRowsV1(part); members.add(part); }","      if (++steps % 64 === 0) await checkpoint();\n      const tool = source && stops?.get(path);\n      if (tool && !roots.includes(path) && !selectors.has(path)) { crossed.add(tool); continue; }\n      const node = await walkNodeV1(env, path, source);\n      for (const part of node.parts) { retainObservedRowsV1(part); members.add(part); }")
edit("compiled, defined, members: [...members] };","compiled, defined, members: [...members], tools: crossed };")
edit("function walkQuestionV1(cache, workspaceRoot, roots, requiredSources, resources, selectors, view) {","function walkQuestionV1(cache, workspaceRoot, roots, requiredSources, resources, selectors, view, stops) {")
edit("provider: cache.providerDesignation ?? null, ...(view === undefined ? {} : { view }) });","provider: cache.providerDesignation ?? null, ...(stops?.length ? { stops } : {}), ...(view === undefined ? {} : { view }) });")
edit("!Array.isArray(question.selectors) || question.view !== undefined &&","!Array.isArray(question.selectors) || question.stops !== undefined && !toolStopsAdmittedV1(question.stops) || question.view !== undefined &&")
edit("  if (cache.pass?.transient === true) {","  if (cache.pass?.transient === true && !question.stops?.length) {")
edit("question.resources, new Map(question.selectors)));","question.resources, new Map(question.selectors), question.stops ?? []));")
edit("const toolTablesV1 = new WeakMap();","""const toolTablesV1 = new WeakMap();

/** 📌️ The digest of the stops a walk is cut at, once per stops list. */
const stopDigests = new WeakMap();
const stopDigestV1 = stops => stopDigests.get(stops) ?? stopDigests.set(stops, memoDigestV1(stops)).get(stops);

/** 🪧️ Whether a target keeps the whole walk of its own scripts and declared modules, so no declared tool is cut out of it: it names module globs, a TypeScript project, runtime source entries or a module literal.
 * @see ./🔌️provider-closures/🔣️.json */
function keepsWholeWalkV1(target) {
  const semio = target.metadata?.semio;
  return semio?.closure?.modules !== undefined || semio?.closure?.typescript !== undefined || semio?.runtimeSourceEntries !== undefined || Array.isArray(target.inputs) && target.inputs.some(declaredModuleLiteralV1);
}

/** 🛑️ The declared tool entry modules (absolute path and tool name, sorted) at which the walk of a target stops: a module the walk imports that is the entry of a declared tool is a tool boundary (for a generic command only a tool whose view carries the external packages, like the cover of an entry script), the tool's closure belongs to the tool's group of the providers project, and the rows of the tool's modules are no row of any command closure that imports it, so an edit inside a tool moves one group and no owner.
 * @see ./🔌️provider-closures/🔣️.json */
function toolStopsV1(scripts, packages) {
  const table = toolTableV1(scripts), kept = table.stops ??= new Map();
  if (!kept.has(Boolean(packages))) kept.set(Boolean(packages), Object.freeze([...table.entries].filter(([, name]) => !packages || PROVIDER_CLOSURE_POLICY_V1.views[PROVIDER_CLOSURE_POLICY_V1.tools.closures[name].view].packages).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0)));
  return kept.get(Boolean(packages));
}

/** 🚏️ Whether the stops a walk question names are pairs of an absolute path and a tool name. */
function toolStopsAdmittedV1(stops) {
  return Array.isArray(stops) && stops.every(row => Array.isArray(row) && row.length === 2 && row.every(item => typeof item === "string"));
}""")
edit("""  const none = { covered: [], references: [] }, semio = target.metadata?.semio;
  if (semio?.closure?.modules !== undefined || semio?.closure?.typescript !== undefined || semio?.runtimeSourceEntries !== undefined) return none;
  if (Array.isArray(target.inputs) && target.inputs.some(declaredModuleLiteralV1)) return none;
""","""  const none = { covered: [], references: [] };
  if (keepsWholeWalkV1(target)) return none;
""")
edit("async function toolFinishV1(scripts, workspaceRoot, own, entryCover, mode) {","async function toolFinishV1(scripts, workspaceRoot, own, entryCover, mode, crossed = []) {")
edit("      if (!tool.entries.some(entry => present.has(closurePathNameV1(scripts, workspaceRoot, resolve(base, entry))))) continue;\n      for (const file of await toolGroupFilesV1(workspaceRoot, scripts, name)) covered.add(file);","      const reached = crossed.includes(name);\n      if (!reached && !tool.entries.some(entry => present.has(closurePathNameV1(scripts, workspaceRoot, resolve(base, entry))))) continue;\n      if (!reached) for (const file of await toolGroupFilesV1(workspaceRoot, scripts, name)) covered.add(file);")
edit("tools=cover===undefined?{covered:[],references:[]}:toolCoverV1(scripts,target,entries,words,resources,cover===\"generic\");","tools=cover===undefined?{covered:[],references:[]}:toolCoverV1(scripts,target,entries,words,resources,cover===\"generic\"),stops=cover===undefined||keepsWholeWalkV1(target)?[]:toolStopsV1(scripts,cover===\"generic\");")
edit("base=baseSize?await boundedScriptInputsV1([...entries],workspaceRoot,scripts,true,resources,commands):undefined;","base=baseSize?await boundedScriptInputsV1([...entries],workspaceRoot,scripts,true,resources,commands,stops):undefined;")
edit("let closure=entries.size===baseSize?base:entries.size?await boundedScriptInputsV1([...entries],workspaceRoot,scripts,true,resources,commands):undefined,","let closure=entries.size===baseSize?base:entries.size?await boundedScriptInputsV1([...entries],workspaceRoot,scripts,true,resources,commands,stops):undefined,")
edit("finished=await toolFinishV1(scripts,workspaceRoot,own,tools,cover)","finished=await toolFinishV1(scripts,workspaceRoot,own,tools,cover,walked?scripts.walkTools.get(walked)??[]:[])")
open(S+"/new.mjs","w",encoding="utf8").write(new)
open(S+"/base.sha","w").write(hashlib.sha256(base.encode("utf8")).hexdigest())
print("staged",len(base),"->",len(new),hashlib.sha256(base.encode()).hexdigest()[:12])
