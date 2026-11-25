import sys,re
p=sys.argv[1];s=open(p,encoding='utf-8').read()
def rep(old,new,count=1):
    global s
    n=s.count(old)
    assert n==count,(n,old[:120])
    s=s.replace(old,new)
def cut(start_marker,end_marker,new,include_end=True):
    global s
    a=s.index(start_marker);b=s.index(end_marker,a)+(len(end_marker) if include_end else 0)
    s=s[:a]+new+s[b:]

# imports
rep(", observeDetachedSyncV1, withObservationPassRoundV1 , registerProjectClosureTravelV1,", ", observeDetachedSyncV1, withObservationPassRoundV1 , registerProjectClosureTravelV1, observeGlobalFactV1, registerGlobalFactV1, observationWitnessHoldsSyncV1,")
rep(", retainObservedRowsV1, observeDetachedSyncV1,", ", observeDetachedSyncV1,")
rep('import { creditOwnerSharedV1, throwIfOwnerExceededV1 } from "../../../🧠️memory/🎯️owner-scope/🟨️.mjs";','import { throwIfOwnerExceededV1 } from "../../../🧠️memory/🎯️owner-scope/🟨️.mjs";')
rep('import { WALK_VIEW_POLICY_V1, closureRoleViewV1, walkViewAnswerV1, walkViewOfRecipeV1, walkViewPrefixesV1 } from "./👓️walk-views/🟨️.mjs";','import { closureRoleViewV1, walkViewOfRecipeV1, walkViewPrefixesV1 } from "./👓️walk-views/🟨️.mjs";')
rep('import { createReachSetsV1 } from "./🧠️reach-memo/🟨️.mjs";\n','')
rep('import { createClosureGraphV1, moduleNodeDigestV1 } from "./🌳️closure-graph/🟨️.mjs";','import { CLOSURE_GRAPH_FACT_V1, CLOSURE_GRAPH_POLICY_V1, createClosureGraphV1, loadClosureGraphSyncV1, moduleNodeDigestV1, saveClosureGraphV1 } from "./🌳️closure-graph/🟨️.mjs";')

# boundedScriptInputsV1
a=s.index("/** 🧠️ Tracks literal relative command imports through the existing TypeScript tooling boundary")
b=s.index("/** 🧭️ The edges of one module:",a)
s=s[:a]+'''/** 🧠️ Tracks literal relative command imports through the existing TypeScript tooling boundary, up to the script provider ports: a module that defines ports adds the designated ports to the walk's ports (`cache.walkPorts`) instead of the provider modules, so a walk and its derived fact never contain a provider's files; a router script named in `commands` (absolute path to the registered command words it is run with) contributes only the imports those commands need, and every router this reading does not fully understand keeps all of its imports. What the walk read is ONE `closure-graph` global row: its question and the closure digest the module graph of the tree answers for it ({@link closureGraphAnswerV1}), never the rows of the modules it visited, so a closure entry or a project that used the walk is valid exactly while that digest holds; the files are walked again only when the digest moved. */
async function boundedScriptInputsV1(entries, workspaceRoot, cache = createScriptInputCache(), requiredSources=false, resourceEntries=[], commands=new Map(), stops=[]) {
  const question = { roots: [...new Set(entries.map(entry => resolve(entry)))].sort(), requiredSources: Boolean(requiredSources), resources: [...new Set(resourceEntries.map(path => resolve(path)))].sort(), selectors: [...commands].map(([path, words]) => [resolve(path), [...new Set(words)].sort()]).sort(byNameV1), stops };
  const key = memoDigestV1(resolve(workspaceRoot), question.roots, question.requiredSources, question.resources, question.selectors, ...(stops.length ? [stopDigestV1(stops)] : [])), digest = await closureGraphAnswerV1(cache, workspaceRoot, question);
  let held = cache.closures.get(key);
  if (held?.digest !== digest) {
    let steps = 0;
    const compiler = parserTypescriptV1(), checkpoint = (force = false) => yieldInferenceV1(cache.context, "build:command-source-imports", ++steps, force);
    const walk = await reachWalkV1({ roots: question.roots, workspaceRoot, cache, requiredSources: question.requiredSources, resources: new Set(question.resources), selectors: new Map(question.selectors), checkpoint, compiler, host: observedTypeScriptHost(compiler.sys), stops: new Map(stops) });
    cache.compiledNames.set(walk.result, new Set([...walk.compiled].map((file) => closurePathNameV1(cache, workspaceRoot, file))));
    cache.walkPorts.set(walk.result, [...walk.defined].sort());
    cache.walkTools.set(walk.result, [...walk.tools].sort());
    cache.closures.set(key, held = { digest, value: walk.result });
  }
  observeGlobalFactV1(CLOSURE_GRAPH_FACT_V1, resolve(workspaceRoot), closureGraphSegmentsV1(cache, workspaceRoot, question), digest);
  return held.value;
}

'''+s[b:]

# reachWalkV1: no retention, no members
rep('''/** 🌳️ The command walk over graph nodes: depth first over `(path, source)` pairs, the same visiting rules as the inline walk, every node's rows merged by reference into the closure's scope; a module''','''/** 🌳️ The command walk over graph nodes: depth first over `(path, source)` pairs, the same visiting rules as the inline walk; no row of a node joins the caller's scope (the walk is named by its `closure-graph` row, see {@link boundedScriptInputsV1}); a module''')
rep('''  const env = { cache, workspaceRoot, requiredSources, selectors, compiler, host, checkpoint, pass: cache.pass ?? {} };
  env.pass.valid ??= new WeakSet();
  const files = new Set(), compiled = new Set(), defined = new Set(), seen = new Set(), stack = roots.map((path) => [path, true]), directories = new Set(), members = new Set(), crossed = new Set();''','''  const env = { cache, workspaceRoot, requiredSources, selectors, compiler, host, checkpoint, pass: closureWalkPassV1(cache) };
  env.pass.valid ??= new WeakSet();
  const files = new Set(), compiled = new Set(), defined = new Set(), seen = new Set(), stack = roots.map((path) => [path, true]), directories = new Set(), crossed = new Set();''')
rep('''      const node = await walkNodeV1(env, path, source);
      for (const part of node.parts) { retainObservedRowsV1(part); members.add(part); }
      for (const part of node.shared) { retainObservedRowsV1(part); members.add(part); }
      if (node.kind === "skip") continue;''','''      const node = await walkNodeV1(env, path, source);
      if (node.kind === "skip") continue;''')
rep('''  for (const directory of directories) { const node = await walkConfigNodeV1(env, directory); for (const part of node.parts) { retainObservedRowsV1(part); members.add(part); } for (const config of node.configs) configs.add(config); }''','''  for (const directory of directories) for (const config of (await walkConfigNodeV1(env, directory)).configs) configs.add(config);''')
rep('''  return { result: [...files].map((file) => closurePathNameV1(cache, workspaceRoot, file)).sort(), compiled, defined, members: [...members], tools: crossed };''','''  return { result: [...files].map((file) => closurePathNameV1(cache, workspaceRoot, file)).sort(), compiled, defined, tools: crossed };''')

# remove MEMO_SLOT, reachState/reachPass/chargedToShared/reachClosure
rep('const MEMO_SLOT_V1 = 2 ** 22;\n','')
a=s.index("/** 🧮️ The module graph of one cache with the reach set of every node:")
b=s.index("/** 🌳️ One pass of the module closure graph",a)
mid=s[a:b]
keep_managed='const managedBytesV1 = () => { const usage = process.memoryUsage(); return usage.heapUsed + usage.external; };\n\n'
assert keep_managed in mid
s=s[:a]+keep_managed+s[b:]
open(p,'w',encoding='utf-8').write(s)
print("ok")
