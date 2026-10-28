#!/usr/bin/env python3
"""[derive-g] builds staged/batch.json (anchored edits with old and new text, base and after hashes) and staged/new-files.tgz from the CURRENT tree, which carries the batch. The old side of every edit is the text before derive-g's edit; whole-file edits take git HEAD as the old side (their files had no other edit)."""
import json, hashlib, subprocess, tarfile, os, sys
R = "/Users/ueli/Documents/semio/"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B = L + "🏗️build/"; G = B + "🕸️graph/"; I = G + "📥️inference/"
OUT = os.path.dirname(os.path.abspath(__file__))
def read(p): return open(R + p, encoding="utf8").read()
def head(p): return subprocess.run(["git", "show", "HEAD:" + p], cwd=R, capture_output=True, check=True).stdout.decode("utf8")
def between(text, start, end):
    a = text.index(start); b = text.index(end, a); return text[a:b]
edits = []
def edit(file, old, new, note): edits.append({"file": file, "old": old, "new": new, "note": note})
def whole(file, note):
    edit(file, head(file), read(file), note + " (whole file against git HEAD)")

# E1 closure-facts law
f = I + "🧭️script-inputs/🟨️.mjs"; t = read(f)
edit(f, "let buildPath, readPhysicalSource, requirePhysicalSourceV1, prepareSourceAnalysesV1, LIBRARY_ROOT,", "let buildPath, readPhysicalSource, requirePhysicalSourceV1, prepareSourceAnalysesV1, revisionIdentity, register, PROVIDER_PORTS, LIBRARY_ROOT,", "E1 vars")
w = "  ({ prepareSourceAnalysesV1 } = registry.sourceAnalyses);\n"
edit(f, w, w + between(t, "  ({ revisionIdentity, register } = registry);\n", "  ({ readToml } = registry.nativeManifests);"), "E1 provider ports analysis")
edit(f, '      const text = readPhysicalSource(path).toString("utf8"), ports = scriptProviderPortsV1(text), imports = ports.length ? scriptProviderImportsV1(cache, ports) : [], words = selectors.get(path), kept = words?.length ? routerKeptV1(compiler, path, text, words) : undefined;',
     between(t, "      requirePhysicalSourceV1(path);\n      const ports = observedSourceAnalysisV1(PROVIDER_PORTS", "\n      const generated=") , "E1 ports through the analysis fact")
r = "/** 🔀️ The imports a router script needs for all the commands it runs with,"
edit(f, r, between(t, "/** 🔀️ The router script is the one closure member", r) + r, "E1 router content fact")
# E3 resolveCommandWalkV1
anchor = "/** 🎯️ The fields of a target that decide its command closure; the recipe names nothing else of the target. */"
edit(f, anchor, between(t, "/** 🧭️ The command walk of `entries` for a caller", anchor) + anchor, "E3 resolveCommandWalkV1")
f = I + "🧭️script-inputs/🟨️.d.mts"; t = read(f)
a = "export function closureApiV1(scripts: ScriptInputsValueV1, workspaceRoot: string): ScriptInputsValueV1;"
edit(f, a, a + "\n" + [l for l in t.split("\n") if l.startswith("export function resolveCommandWalkV1")][0], "E3 d.mts")
f = I + "🟨️.mjs"
edit(f, 'createScriptInputCache: scope.modules["script-inputs"].createScriptInputCache };', 'createScriptInputCache: scope.modules["script-inputs"].createScriptInputCache, resolveCommandWalkV1: scope.modules["script-inputs"].resolveCommandWalkV1, createStreamingEdgesV1: scope.modules["dependency-edges"].createStreamingEdgesImplementation };', "E3/E2 loadCurrentImplementationV1")
f = I + "🟨️.d.mts"; t = read(f)
edit(f, "createScriptInputCache(context?:ScriptInputContextV1):ScriptInputCacheV1}", between(t, "createScriptInputCache(context?:ScriptInputContextV1):ScriptInputCacheV1;resolveCommandWalkV1", "\nexport interface ScriptInputCacheV1") , "E3/E2 interface")
whole(I + "🧪️tests/🧷️closure-facts/🔣️.json", "E1 law rows")
# E2/E3 whole files
for p, n in [(I + "🕸️dependency-edges/🟨️.mjs", "E2 edges"), (G + "🚪️admission/🟦️.ts", "E2 ownership resolver"), (G + "📋️model/🟦️.ts", "E2 part model")]:
    whole(p, n)
f = I + "🧪️tests/✂️derived-walks/🟦️.ts"; t = read(f)
anchor = 'test("a derived question that no walk answers is refused'
edit(f, anchor, between(t, 'test("a caller that is no project (the test discovery)', anchor) + anchor, "E3 law: the walk of a caller that is no project")
# E3 test owners (a file of asm-g that git HEAD does not know)
f = G + "🎯️test-owners/🟨️.mjs"; t = read(f)
edit(f, " * Answers the contributions in discovery order and the facts by project name.\n */", between(t, " * The shared state may carry a `walk`", " */\nexport async function discoverTestOwnersV1") + " */", "E3 doc")
edit(f, " const contributions=[],rows=new Map();\n if(!groups.size)return{contributions,rows};\n const shared=await capture(()=>tests.discoverTestSharedV1(context)),ordered=[...groups].sort(([a],[b])=>compareTestOwnersV1(a,b));\n const discover=async features=>{\n  const one=await capture(()=>tests.discoverTestBuildProjectsV1(features,{},{...context,testShared:shared.value})),facts=[...shared.observations,...one.observations];",
     between(t, " const contributions=[],rows=new Map();", "  for(const entry of one.value)"), "E3 held rows")
edit(f, " return{contributions,rows};\n}", " return{contributions,rows,derived};\n}", "E3 result")
f = G + "🎯️test-owners/🟨️.d.mts"; t = read(f)
edit(f, "readonly rows:Map<string,Row[]>}", "readonly rows:Map<string,Row[]>;readonly derived:readonly {readonly row:Row;readonly support:readonly Row[]}[]}", "E3 d.mts type")
edit(f, "/** 🧪️ Discovers the owners of `groups` one by one, each in its own capture, and answers their contributions and the complete facts of every project. */", "/** 🧪️ Discovers the owners of `groups` one by one, each in its own capture, and answers their contributions, the complete facts of every project and the derived facts those facts name. */", "E3 d.mts doc")
# observation round
f = B + "👁️observation/🟨️.mjs"; t = read(f)
a = "export function observationRoundV1(){return inferenceValidationScopes.getStore();}\n"
edit(f, a, a + between(t, "/** 🔂️ Runs `compute` inside one validation round", "/** 🫧️ Runs `compute` with its observations detached"), "A validation round")
f = B + "👁️observation/🟨️.d.mts"; t = read(f)
a = "export function observationRoundV1():object|undefined;\n"
edit(f, a, a + between(t, "/** 🔂️ Runs `compute`", "export function withObservationRoundV1<T>(compute:()=>T):T;\n") + "export function withObservationRoundV1<T>(compute:()=>T):T;\n", "A d.mts")
# texts
f = B + "📖️texts/🔣️.json"; t = read(f)
a = '    "closure.query-unknown": {'
edit(f, a, between(t, '    "closure.late-cache-required": {', a) + a, "E3 text key")
# generation-protocol fixture
f = G + "🧪️tests/🕰️generation-protocol/🔣️.json"
edit(f, '   "before": "deriveEnvironmentV1({",\n   "after": "const edgeNames="', '   "before": "deriveEnvironmentV1({",\n   "after": "edgeNames=refresh.projects"', "fixture repair")
# generations
f = G + "🕰️generations/🟨️.mjs"; t = read(f)
edit(f, "{...fragment,edges:owner.edges,edgesEnv:envDigest}", "{...fragment,edges:owner.edges??await result.reEdge(owner.name,fragment),edgesEnv:envDigest}", "E2 build uses reEdge")
old_key = 'export function rowKeyV1(row){let key=rowKeys.get(row);if(key===undefined){key=canonicalJsonV1([row.kind,row.path,row.fact]);rowKeys.set(row,key);}return key;}'
edit(f, "/** 🪪️ The exact identity of one observation row; a shared row object is keyed once. */\n" + old_key, between(t, "/** 🪪️ The identity of one observation row: its canonical text", "export const ROW_KEY_TEXT_LIMIT_V1=128;") + "export const ROW_KEY_TEXT_LIMIT_V1=128;", "row key digest for long rows")
f = G + "🕰️generations/🟨️.d.mts"; t = read(f)
edit(f, "kind?:\"project\"|\"unavailable\";node?:unknown;edges:unknown[];", "kind?:\"project\"|\"unavailable\";node?:unknown;edges?:unknown[];", "E2 edges optional")
edit(f, "export interface GenerationComputeResultV1{owners:GenerationComputedOwnerV1[];", "export interface GenerationComputeResultV1{owners:GenerationComputedOwnerV1[];reEdge?:(name:string,fragment:{node?:unknown})=>Promise<unknown[]>;", "E2 reEdge type")
edit(f, "export function rowKeyV1(row:GenerationRowV1):string;", "export function rowKeyV1(row:GenerationRowV1):string;\nexport const ROW_KEY_TEXT_LIMIT_V1:128;", "row key limit type")
f = G + "🕰️generations/🔌️load/🟨️.d.mts"
edit(f, "edgeOnly:{name:string}[];derived?:{row:GenerationRowV1;support:GenerationRowV1[]}[];", "edgeOnly:{name:string;streamed?:true}[];reEdge?:(name:string,fragment:{node?:unknown})=>Promise<unknown[]>|unknown[];derived?:{row:GenerationRowV1;support:GenerationRowV1[]}[];", "E2 part type")
f = G + "🕰️generations/🔌️load/🟨️.mjs"; t = read(f)
edit(f, "owners.push({name:owner.name,partial:\"edges\",edges:orderedEdgesV1(edgesOf.get(owner.name)??[]),rows:claim([],[stub.root,stub.nativeRoot])});", "owners.push({name:owner.name,partial:\"edges\",...(owner.streamed?{}:{edges:orderedEdgesV1(edgesOf.get(owner.name)??[])}),rows:claim([],[stub.root,stub.nativeRoot])});", "E2 shape streamed")
edit(f, "return{owners,...(part.derived?.length?{derived:part.derived}:{}),", "return{owners,...(part.reEdge?{reEdge:async(name,fragment)=>orderedEdgesV1(await part.reEdge(name,fragment))}:{}),...(part.derived?.length?{derived:part.derived}:{}),", "E2 shape reEdge")
edit(f, "const claim=(base,roots)=>{\n  const keys=new Set(base.map(rowKeyV1)),list=prefixes(roots);let rows=base;\n  if(list.length)for(const at of[...new Set(list.flatMap(under))].sort((a,b)=>a-b)){const row=part.rows[at],key=rowKeyV1(row);if(!keys.has(key)){keys.add(key);if(rows===base)rows=[...base];rows.push(row);}}\n  for(const key of keys)claimed.add(key);return rows;\n };",
     between(t, "const claimedRows=new WeakSet()", " for(const owner of part.owners)"), "shape claim shares key sets of shared row lists")
# graph
f = G + "🟦️.ts"; t = read(f)
edit(f, 'import{createClaimRegistryV1,createQueuedClaimLeasePortV1,prepareStaleProjectsV1,renderClaimMessageV1,slotCountV1,type ClaimMessageKeyV1}from', 'import{CLAIMS_POLICY_V1,createClaimRegistryV1,createQueuedClaimLeasePortV1,prepareStaleProjectsV1,renderClaimMessageV1,slotCountV1,type ClaimMessageKeyV1}from', "claims policy import")
edit(f, "const discovery=await discoverTestOwnersV1<BuildGraphRowV1>({tests:tests as never,", "const discovery=await observationV1.withObservationRoundV1(()=>discoverTestOwnersV1<BuildGraphRowV1>({tests:tests as never,", "A round")
edit(f, "context,groups}),testContributions=discovery.contributions as [string,any][];", "context,groups})),testContributions=discovery.contributions as [string,any][];for(const fact of discovery.derived)derivedFacts.set(fact.row.path+\"\\0\"+fact.row.fact,fact);", "E3 derived facts")
edit(f, 'const graphContext={workspaceRoot:root,projects:contextProjects,fileMap:{projectFileMap},invocation,phaseControl,selection:{kind:"all"},registry,completeSourceImports:false};', between(t, '    const streamed=new Set<string>(edgeNames)', '    const testEdges:unknown=').rstrip("\n").lstrip(" "), "E2 graph context")
edit(f, "    const edges=[...await owner.discoverBuildDependenciesV1(options,graphContext),...testEdges];", between(t, "    const implementation=streamed.size?", "\n    const metadata=await owner.createMetadata(").rstrip("\n"), "E2 streaming edges")
edit(f, "edgeOnly:[...edgeNames,...refresh.withheld].map(name=>({name})),", "edgeOnly:[...edgeNames.map(name=>streamed.has(name)?{name,streamed:true as const}:{name}),...refresh.withheld.map(name=>({name}))],reEdge:streaming&&((name:string,fragment:{node?:any})=>streaming.edgesOf(name,fragment.node)),", "E2 part edgeOnly")
edit(f, "nodeRows:tags[name]===TEST_TAG_V1?[...(discovery.rows.get(name)??[]),...testRows]:tags[name]?tagRows:nodeRows[name]})),", "nodeRows:tags[name]===TEST_TAG_V1?testNodeRows(name):tags[name]?tagRows:nodeRows[name]})),", "shape shared test rows")
edit(f, "nodeRows:tags[row.project!]===TEST_TAG_V1?[...(discovery.rows.get(row.project!)??[]),...testRows]:tags[row.project!]?tagRows:nodeRows[row.project!]}))],", "nodeRows:tags[row.project!]===TEST_TAG_V1?testNodeRows(row.project!):tags[row.project!]?tagRows:nodeRows[row.project!]}))],", "shape shared test rows 2")
edit(f, "    const rows=[...unavailable.values()];\n    return {", "    const rows=[...unavailable.values()],testRowLists=new Map<object,BuildGraphRowV1[]>(),testNodeRows=(name:string)=>{const own=discovery.rows.get(name)??[];let list=testRowLists.get(own);if(!list)testRowLists.set(own,list=[...own,...testRows]);return list;};\n    return {", "shape shared test rows def")
# test module (separate root)
f = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs"; t = read(f)
edit(f, 'async function commandInputsOf(context, vocabulary, native, javascript, createScriptInputCache) {\n  const { workspaceRoot } = context;\n  return compactSourceInputs([...await native.relativeScriptInputs([join(workspaceRoot, vocabulary.testDomainPath, "📜️script.ts")], workspaceRoot, createScriptInputCache(context)), ...javascript.files.map((path) => `{workspaceRoot}/${path}`), ...javascript.environment.map((env) => ({ env })), ...javascript.commands.map((runtime) => ({ runtime }))], workspaceRoot);\n}',
     between(t, "async function commandInputsOf(", "/** 🧬️ Computes what every test owner shares once").rstrip("\n"), "E3 commandInputsFrom")
edit(f, '  const { cacheInternals: native, libraryBootstrap, createScriptInputCache } = await (await loadInferenceModuleV1()).loadCurrentImplementationV1();\n  await libraryBootstrap;\n  const commandInputs = await commandInputsOf(context, vocabulary, native, policy.toolchains.javascript, createScriptInputCache);\n  const state = { manifests: new Map(), closures: new Map(), names: new Map() }, host',
     between(t, "  const { cacheInternals: native, libraryBootstrap, createScriptInputCache, resolveCommandWalkV1 }", "= `${vocabulary.testDomainPath}/📦️packages/🦀️rust`;") .rstrip(), "E3 testShared walk")
edit(f, "  return Object.freeze({ commandInputs, state: Object.freeze(state) });", "  return Object.freeze({ commandInputs, state: Object.freeze(state), walk: Object.freeze({ rows: walk.rows, derived: walk.derived }) });", "E3 shared carries walk")
# README bullets
f = "🧰️framework/🛍️products/🦑️repo/README.md"; t = read(f)
en = [l for l in t.split("\n") if l.startswith("- **Cheap generations after an environment move (graph generations).**")][0]
de = [l for l in t.split("\n") if l.startswith("- **Günstige Generationen nach einer Umgebungsverschiebung (Graph-Generationen).**")][0]
ka = [l for l in t.split("\n") if l.startswith("- **Kept test and policy owners (graph generations).**")][0]
kd = [l for l in t.split("\n") if l.startswith("- **Beibehaltene Test- und Richtlinien-Eigentümer (Graph-Generationen).**")][0]
edit(f, ka + "\n", ka + "\n" + en + "\n", "README EN"); edit(f, kd + "\n", kd + "\n" + de + "\n", "README DE")

# verification of the new side and write
bad = []
for e in edits:
    cur = read(e["file"]) if e["file"] else ""
    if cur.count(e["new"]) != 1: bad.append((e["note"], cur.count(e["new"])))
if bad: print("NEW SIDE NOT UNIQUE/PRESENT:", bad); sys.exit(1)
sha = lambda s: hashlib.sha256(s.encode("utf8")).hexdigest()
files = {}
for e in edits: files.setdefault(e["file"], []).append(e)
manifest = {"version": 1, "owner": "derive-g", "files": []}
for file, es in files.items():
    cur = read(file); base = cur
    for e in reversed(es):
        if e["new"] in base: base = base.replace(e["new"], e["old"], 1)
    manifest["files"].append({"file": file, "baseSha256": sha(base), "afterSha256": sha(cur), "edits": es})
json.dump(manifest, open(OUT + "/batch.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(edits), "edits in", len(files), "files")
newdirs = [G + "🚪️admission/🧪️tests/🗂️relative-owners", I + "🕸️dependency-edges/🧪️tests/🪣️re-edge-chunks", I + "🕸️dependency-edges/🧪️tests/🌊️streaming-edges", B + "👁️observation/🧪️tests/🔂️validation-round", G + "🕰️generations/🧪️tests/🪪️row-keys"]
names = sorted(sum([subprocess.run(["git", "ls-files", "--others", "--cached", "--", d], cwd=R, capture_output=True, check=True).stdout.decode("utf8").split("\n") for d in newdirs], []))
names = [n for n in names if n and not n.endswith("probe.test.ts")]
with tarfile.open(OUT + "/new-files.tgz", "w:gz") as tar:
    for n in names: tar.add(R + n, arcname=n, recursive=False)
json.dump(names, open(OUT + "/new-files.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(names), "new files")
