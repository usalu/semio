#!/usr/bin/env python3
"""Builds staged/ops.json: every anchored edit of the conc-graph set, extracted from the LIVE tree (each anchor must occur exactly once), the base sha256 of every touched file and the map of new files.

usage: ops_gen.py [--root REPO_ROOT]   (re-run after any live drift; apply.py refuses drifted anchors)"""
import hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
if "--root" in sys.argv:
    ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
STAGED = os.path.join(HERE, "staged")


def E(code):
    return chr(code) + "️"


def child(directory, ascii_name):
    """The live child of `directory` whose name is an emoji followed by `ascii_name`."""
    for name in sorted(os.listdir(os.path.join(ROOT, directory))):
        if name.endswith(ascii_name) and len(name) > len(ascii_name) and not name[: -len(ascii_name)].isascii():
            return os.path.join(directory, name)
    raise SystemExit("no child %r under %s" % (ascii_name, directory))


FRAMEWORK = E(0x1F9F0) + "framework"
LIB = os.path.join(FRAMEWORK, E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
BUILD = os.path.join(LIB, E(0x1F3D7) + "build")
GRAPH = os.path.join(BUILD, E(0x1F578) + "graph")
GRAPH_TS = os.path.join(GRAPH, E(0x1F7E6) + ".ts")
PROJECT_NODES = os.path.join(GRAPH, E(0x1F4E5) + "inference", E(0x1F4E6) + "project-nodes", E(0x1F7E8) + ".mjs")
OBSERVATION = os.path.join(BUILD, E(0x1F441) + "observation")
INVOCATION = os.path.join(BUILD, E(0x1F9ED) + "invocation", E(0x1F7E6) + ".ts")
SCRIPT = os.path.join(BUILD, E(0x1F4DC) + "script.ts")
OPERATIONS = os.path.join(BUILD, E(0x2699) + "operations")
PROCESS = os.path.join(FRAMEWORK, E(0x1F528) + "modules", E(0x1F3C3) + "process", E(0x1F9ED) + "routing", E(0x1F4E5) + "invocation", E(0x1F3C3) + "process")
PROGRESS = os.path.join(PROCESS, E(0x1F9EC) + "schema", E(0x1F4C8) + "progress.json")
PROGRESS_FIXTURE = os.path.join(PROCESS, E(0x1F9EA) + "tests", E(0x1F4C8) + "graph-phase", E(0x1F523) + ".json")
HELP_FIXTURE = os.path.join(OPERATIONS, E(0x1F9EA) + "tests", E(0x1F9EB) + "fixtures", E(0x1F4D6) + "help", E(0x1F523) + ".json")
CATALOG = os.path.join(OPERATIONS, E(0x1F523) + "catalog.json")
TS, MJS, DMTS, JSON_, TESTS, SCHEMA = E(0x1F7E6) + ".ts", E(0x1F7E8) + ".mjs", E(0x1F7E8) + ".d.mts", E(0x1F523) + ".json", E(0x1F9EA) + "tests", E(0x1F9EC) + "schema"
GEN = os.path.join(GRAPH, E(0x1F570) + "generations")
ENV = os.path.join(GEN, E(0x1F30D) + "environment")
LOAD = os.path.join(GEN, E(0x1F50C) + "load")
PROTOCOL = os.path.join(GRAPH, TESTS, E(0x1F570) + "generation-protocol")
FACTS = os.path.join(GRAPH, E(0x1F4E5) + "inference", TESTS, E(0x1F9F7) + "closure-facts")

NEW = {
    "gen/module.mjs": os.path.join(GEN, MJS), "gen/module.d.mts": os.path.join(GEN, DMTS), "gen/schema.json": os.path.join(GEN, SCHEMA, JSON_),
    "gen/test.ts": os.path.join(GEN, TESTS, TS), "gen/fixture.json": os.path.join(GEN, TESTS, JSON_), "gen/fixture-schema.json": os.path.join(GEN, TESTS, SCHEMA, JSON_),
    "gen/oracle.py": os.path.join(GEN, TESTS, E(0x1F40D) + "oracle.py"), "gen/world.ts": os.path.join(GEN, TESTS, E(0x1F3C3) + "world", TS),
    "env/module.mjs": os.path.join(ENV, MJS), "env/module.d.mts": os.path.join(ENV, DMTS),
    "env/test.ts": os.path.join(ENV, TESTS, TS), "env/fixture.json": os.path.join(ENV, TESTS, JSON_), "env/fixture-schema.json": os.path.join(ENV, TESTS, SCHEMA, JSON_),
    "load/module.mjs": os.path.join(LOAD, MJS), "load/module.d.mts": os.path.join(LOAD, DMTS),
    "load/test.ts": os.path.join(LOAD, TESTS, TS), "load/fixture.json": os.path.join(LOAD, TESTS, JSON_), "load/fixture-schema.json": os.path.join(LOAD, TESTS, SCHEMA, JSON_),
    "facts/test.ts": os.path.join(FACTS, TS), "facts/fixture.json": os.path.join(FACTS, JSON_), "facts/fixture-schema.json": os.path.join(FACTS, SCHEMA, JSON_), "facts/oracle.py": os.path.join(FACTS, E(0x1F40D) + "oracle.py"),
    "protocol/test.ts": os.path.join(PROTOCOL, TS), "protocol/fixture.json": os.path.join(PROTOCOL, JSON_), "protocol/fixture-schema.json": os.path.join(PROTOCOL, SCHEMA, JSON_), "protocol/oracle.py": os.path.join(PROTOCOL, E(0x1F40D) + "oracle.py"),
}


def read(path):
    with open(os.path.join(ROOT, path), encoding="utf8") as handle:
        return handle.read()


def staged(name):
    with open(os.path.join(STAGED, name), encoding="utf8") as handle:
        return handle.read()


ops, bases = [], {}


def op(path, anchor, replacement, marker=None):
    text = read(path)
    if text.count(anchor) != 1:
        raise SystemExit("anchor occurs %d times in %s: %r" % (text.count(anchor), path, anchor[:80]))
    bases.setdefault(path, hashlib.sha256(text.encode("utf8")).hexdigest())
    ops.append({"path": path, "anchor": anchor, "replacement": replacement, **({"marker": marker} if marker else {})})


def line(path, prefix):
    for candidate in read(path).split("\n"):
        if candidate.startswith(prefix):
            return candidate
    raise SystemExit("no line starting with %r in %s" % (prefix, path))


def origin(relative):
    return 'new URL("%s",physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL))' % relative


# ---- the graph module -------------------------------------------------------------------------------------------------------------------------------
requirement = line(GRAPH_TS, "import {requireNativeStoragePathV1}")
op(GRAPH_TS, requirement, requirement + "\n" + "\n".join([
    'import{createGenerationStoreV1,createNodeFilePortV1,createFrontierV1,diffFrontiersV1,frontierHoldsV1,canonicalJsonV1 as canonicalGenerationJsonV1}from"./%s/%s";' % (E(0x1F570) + "generations", MJS),
    'import{loadGenerationGraphV1,renderGenerationMessageV1}from"./%s/%s/%s";' % (E(0x1F570) + "generations", E(0x1F50C) + "load", MJS),
    'import{deriveEnvironmentV1,environmentRowHoldsV1}from"./%s/%s/%s";' % (E(0x1F570) + "generations", E(0x1F30D) + "environment", MJS),
]))
op(GRAPH_TS, 'import { join, dirname, relative, isAbsolute } from "node:path";', 'import { join, dirname, relative, isAbsolute, resolve, sep } from "node:path";')
op(GRAPH_TS, ",graphSnapshotPolicyURLV1])implementation.update(", "," + ",".join(origin(p) for p in [
    "./%s/%s" % (E(0x1F570) + "generations", MJS), "./%s/%s/%s" % (E(0x1F570) + "generations", SCHEMA, JSON_), "./%s/%s/%s" % (E(0x1F570) + "generations", E(0x1F50C) + "load", MJS),
    "./%s/%s/%s" % (E(0x1F570) + "generations", E(0x1F30D) + "environment", MJS), "./%s/%s" % (E(0x1F9ED) + "file-owner", MJS)]) + ",graphSnapshotPolicyURLV1])implementation.update(")
members_doc = "/** " + E(0x1F9FA) + " Keeps exactly the discovered files whose presence the portable graph consumes"
op(GRAPH_TS, members_doc, staged("glue/graph.host.ts.txt") + staged("glue/graph.part.ts.txt").lstrip("\n") + "\n" + members_doc)
op(GRAPH_TS, "export async function loadBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean}):Promise<BuildGraphV1> {",
   'export async function loadBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:{selection:BuildGraphSelectionV1;reuse?:boolean;baseline?:boolean;graph?:{policy?:"fresh"|"current";closure?:readonly string[]}}):Promise<BuildGraphV1> {')
guard = '  if(!invocation)throw Error("Original build graph producer invocation required");\n'
op(GRAPH_TS, guard, guard + staged("glue/graph.load.ts.txt"))
op(GRAPH_TS, 'await rm(path);await advanceScriptInvocation(invocation,"build:reset-publication","running");\n    }\n  });',
   'await rm(path);await advanceScriptInvocation(invocation,"build:reset-publication","running");\n    }\n    await createGenerationStoreV1({root:join(store.root,"generations"),fs:generationFilesV1(store),holds:async()=>true,resources:{facts:"repository-graph-facts:"+resolve(root)}}).invalidate({reason:"reset",signal:invocation.control.signal});\n  });')

# ---- inference: owner filter and per-owner facts ------------------------------------------------------------------------------------------------------
op(PROJECT_NODES, 'import { observeProjectInferenceV1, observedFs } from "../../../../' + E(0x1F3D7) + 'build/' + E(0x1F441) + 'observation/' + E(0x1F7E8) + '.mjs";',
   'import { observeProjectInferenceV1, observedFs, projectInferenceObservationsV1 } from "../../../../' + E(0x1F3D7) + 'build/' + E(0x1F441) + 'observation/' + E(0x1F7E8) + '.mjs";')
op(PROJECT_NODES, "  const { workspaceRoot } = context, unavailable = new Map();\n", "  const { workspaceRoot } = context, unavailable = new Map(), wantedRoots = options?.ownerRoots, wanted = root => !wantedRoots || wantedRoots.has(root);\n")
op(PROJECT_NODES, "    rootsByName.set(name, root);\n    const targets = await cargoTargets(root, workspaceRoot, commandInputs,scripts);",
   '    rootsByName.set(name, root);\n    if (!wanted(root)) { results.push([configFile, { projects: { [name]: { name, root, projectType: "library", targets: {} } } }]); continue; }\n    const targets = await cargoTargets(root, workspaceRoot, commandInputs,scripts);')
op(PROJECT_NODES, "const jobs=results.flatMap(([,result])=>Object.entries(result.projects).map(([name,json])=>({result,name,json})));let projectIndex=0,cursor=0,stopped=false;",
   "const jobs=results.flatMap(([,result])=>Object.entries(result.projects).map(([name,json])=>({result,name,json}))).filter(job=>wanted(job.json.root));let projectIndex=0,cursor=0,stopped=false;")
op(PROJECT_NODES, "    if(result.projects[name].unavailable){unavailable.set(root,result.projects[name].unavailable);delete result.projects[name];return;}",
   "    options?.onOwnerObservations?.(name,projectInferenceObservationsV1(cachePath)??[]);\n    if(result.projects[name].unavailable){unavailable.set(root,result.projects[name].unavailable);delete result.projects[name];return;}")
op(PROJECT_NODES, "  for(const row of [...unavailable.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0))results.push([row.source,{projects:{},unavailable:[row]}]);",
   "  if(wantedRoots)for(const entry of results)entry[1].projects=Object.fromEntries(Object.entries(entry[1].projects??{}).filter(([,project])=>wanted(project.root)));\n  for(const row of [...unavailable.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0))results.push([row.source,{projects:{},unavailable:[row]}]);")
counts = "export function projectInferenceCountsV1(){return Object.fromEntries(projectComputations);}"
op(os.path.join(OBSERVATION, MJS), counts, counts + "\n\n/** " + E(0x1F4E6) + " The facts one project record consumed, whether it was computed or reused in this process. */\nexport function projectInferenceObservationsV1(cachePath){return originalInferencePublications.get(cachePath)?.observations;}\n\n/** " + E(0x1FAA8) + " Runs a partial inference in project scope without an all-or-nothing round: every project is consistent with the facts of its own record, contradictory facts between projects are legitimate and a project that raced an edit is stale in the next verification instead of restarting the pass. */\nexport async function observeInferenceSnapshotV1(compute){\n requireCurrentObservationV1();loadSealed=true;\n const nested=new Map();\n return inferenceValidationScopes.run(nested,()=>scopes.run(nested,compute));\n}")
declared = "export function projectInferenceCountsV1():Record<string,number>;"
op(os.path.join(OBSERVATION, DMTS), declared, declared + "\n/** " + E(0x1F4E6) + " The facts one project record consumed, whether it was computed or reused in this process. */\nexport function projectInferenceObservationsV1(cachePath:string):readonly SourceObservationV1[]|undefined;\n/** " + E(0x1FAA8) + " Runs a partial inference in project scope without an all-or-nothing round. */\nexport function observeInferenceSnapshotV1<T>(compute:()=>T|PromiseLike<T>):Promise<Awaited<T>>;")

# ---- settle-free partial inference and content-free closure facts ---------------------------------------------------------------------------------------
INFERENCE = os.path.join(GRAPH, E(0x1F4E5) + "inference")
ENTRY = os.path.join(INFERENCE, E(0x1F6A6) + "invocation", MJS)
SCRIPT_INPUTS = os.path.join(INFERENCE, E(0x1F9ED) + "script-inputs", MJS)
op(ENTRY, "import { requireCurrentObservationV1, observeDependencies, observeInference, settleObservedComputationV1 }", "import { requireCurrentObservationV1, observeDependencies, observeInference, observeInferenceSnapshotV1, settleObservedComputationV1 }")
op(ENTRY, 'return kind === "nodes" ? observeInference({ cachePath:', 'return kind === "nodes" ? args[1]?.ownerRoots ? observeInferenceSnapshotV1(() => emojiProjectJsonNodes(...args)) : observeInference({ cachePath:')
op(SCRIPT_INPUTS, "let buildPath, readPhysicalSource, prepareSourceAnalysesV1,", "let buildPath, readPhysicalSource, requirePhysicalSourceV1, prepareSourceAnalysesV1,")
op(SCRIPT_INPUTS, "({ buildPath, readPhysicalSource, LIBRARY_ROOT, POLICY,", "({ buildPath, readPhysicalSource, requirePhysicalSourceV1, LIBRARY_ROOT, POLICY,")
op(SCRIPT_INPUTS, "if(requiredSources){const admitted=cache.physical??=new Set();if(!admitted.has(path)){readPhysicalSource(path);admitted.add(path);}}", "if(requiredSources){const admitted=cache.physical??=new Set();if(!admitted.has(path)){requirePhysicalSourceV1(path);admitted.add(path);}}")
op(SCRIPT_INPUTS, "cache.resources.add(path);if(!seen)readPhysicalSource(path);return;", "cache.resources.add(path);if(!seen)requirePhysicalSourceV1(path);return;")
op(SCRIPT_INPUTS, "if(bound.script){readPhysicalSource(bound.script);entries.add(bound.script);}", "if(bound.script){requirePhysicalSourceV1(bound.script);entries.add(bound.script);}")

# ---- the shared fact table: record headers list content-defined blocks ---------------------------------------------------------------------------------
OBS_MJS = os.path.join(OBSERVATION, MJS)
op(OBS_MJS, "if(Array.isArray(header.observations))header.observations=internObservationsV1(header.observations);", "if(Array.isArray(header.observations))header.observations=internObservationsV1(header.observations);if(Array.isArray(header.blocks))header.observations=[...expandInferenceBlocksV1(dirname(cachePath),header.blocks),...header.observations];")
op(OBS_MJS, 'header = `{"digest":${JSON.stringify(digest)},"resultSha256":"${createHash("sha256").update(image).digest("hex")}","authorityObservations":${observationRowsJsonV1(authority)},"observations":${observationRowsJsonV1(rows)}}`;', 'header = inferenceHeaderTextV1(digest,createHash("sha256").update(image).digest("hex"),authority,rows,dirname(cachePath));')
header_end = "  return newline < 0 ? null : text.slice(0, newline);\n}"
op(OBS_MJS, header_end, header_end + "\n" + staged("glue/observation.blocks.mjs.txt").rstrip("\n"), "function inferenceHeaderTextV1(")
op(os.path.join(OBSERVATION, DMTS), "export function inferenceHeaderLine(path:string):string|null;", "export function inferenceHeaderLine(path:string):string|null;\n/** " + E(0x1F9FE) + " A persisted record header with its fact blocks expanded. */\nexport function readInferenceHeaderV1(path:string):{digest:string;resultSha256:string;authorityObservations:readonly SourceObservationV1[];blocks:readonly string[];observations:readonly SourceObservationV1[]};")
ROSTER = os.path.join(OBSERVATION, TESTS, E(0x1F9EB) + "fixtures", JSON_)
op(ROSTER, '      "observeInference",\n      "observeProjectInferenceV1",', '      "observeInference",\n      "observeInferenceSnapshotV1",\n      "observeProjectInferenceV1",')
op(ROSTER, '      "projectInferenceCountsV1",\n      "receiveSourcePreparationV1",', '      "projectInferenceCountsV1",\n      "projectInferenceObservationsV1",\n      "readInferenceHeaderV1",\n      "receiveSourcePreparationV1",')
op(os.path.join(OBSERVATION, SCHEMA, JSON_), '    },\n    "resultSha256": {\n      "type": "string",\n      "pattern": "^[0-9a-f]{64}$"', '    },\n    "blocks": {\n      "type": "array",\n      "items": {\n        "type": "string",\n        "pattern": "^[0-9a-f]{64}$"\n      }\n    },\n    "resultSha256": {\n      "type": "string",\n      "pattern": "^[0-9a-f]{64}$"')
CENSUS = os.path.join(LIB, E(0x26A1) + "caching", E(0x1F578) + "graph", E(0x1F50F) + "seal", E(0x1F9EE) + "census", MJS)
op(CENSUS, 'import { createHash } from "node:crypto";', 'import { createHash } from "node:crypto";\nimport { dirname, join } from "node:path";')
op(CENSUS, "const header = text.slice(0, split), observations = JSON.parse(header).observations;", "const header = text.slice(0, split), parsed = JSON.parse(header), observations = Array.isArray(parsed.observations) ? [...(parsed.blocks ?? []).flatMap((digest) => blockRows(dirname(paths[index]), digest)), ...parsed.observations] : undefined;")
op(CENSUS, "/** 📊️ Reads the records in order;", "const blocks = new Map();\n\n/** 🧱️ The rows of one fact block, verified against its name and read once per worker. */\nfunction blockRows(directory, digest) {\n  let rows = blocks.get(digest);\n  if (!rows) {\n    const text = brotliDecompressSync(readFileSync(join(directory, digest.slice(0, 24) + \".json\"))).toString(\"utf8\");\n    if (createHash(\"sha256\").update(text).digest(\"hex\") !== digest) throw new Error(\"Inference block content differs from its name\");\n    blocks.set(digest, rows = JSON.parse(text));\n  }\n  return rows;\n}\n\n/** 📊️ Reads the records in order;")
RECORD_HEADER = os.path.join(OBSERVATION, TESTS, E(0x1F9FE) + "record-header")
op(os.path.join(RECORD_HEADER, TS), "expect(parsed.observations.some((row:any)=>", "expect(observation.readInferenceHeaderV1(cachePath).observations.some((row:any)=>")
op(os.path.join(RECORD_HEADER, JSON_), '"headerKeys": ["digest", "resultSha256", "authorityObservations", "observations"]', '"headerKeys": ["digest", "resultSha256", "authorityObservations", "blocks", "observations"]')

# ---- invocation and commands ---------------------------------------------------------------------------------------------------------------------------
parse_head = 'export function parseBuildInvocationV1(args:readonly string[],environment:NodeJS.ProcessEnv=process.env):{tool:"nx"|"bazel";args:string[];workers?:"on"|"off"} {\n  let tool=environment.SEMIO_BUILD_TOOL??policy.defaultTool,workers=environment.SEMIO_BUILD_WORKERS;\n  const selected:string[]=[];\n  for(let index=0;index<args.length;index++){\n    const argument=args[index]!;'
op(INVOCATION, parse_head, 'export function parseBuildInvocationV1(args:readonly string[],environment:NodeJS.ProcessEnv=process.env):{tool:"nx"|"bazel";args:string[];workers?:"on"|"off";graphPolicy?:"fresh"|"current"} {\n  let tool=environment.SEMIO_BUILD_TOOL??policy.defaultTool,workers=environment.SEMIO_BUILD_WORKERS;\n  const graph=extractGraphPolicyV1(args,environment),selected:string[]=[];\n  args=graph.args;\n  for(let index=0;index<args.length;index++){\n    const argument=args[index]!;')
op(INVOCATION, "  return {tool,args:selected,...(workers?{workers:workers as \"on\"|\"off\"}:{})};", "  return {tool,args:selected,...(graph.explicit?{graphPolicy:graph.policy}:{}),...(workers?{workers:workers as \"on\"|\"off\"}:{})};")
first_import = line(INVOCATION, "import ")
op(INVOCATION, first_import, first_import + '\nimport{extractGraphPolicyV1}from"../%s/%s/%s/%s";' % (E(0x1F578) + "graph", E(0x1F570) + "generations", E(0x1F50C) + "load", MJS))
op(SCRIPT, 'graph=await loadBuildGraphV1(root,invocation,phaseControl,{selection:{kind:"all"},reuse:process.env.SEMIO_BUILD_REUSE_GRAPH==="true"});',
   'graph=await loadBuildGraphV1(root,invocation,phaseControl,{selection:{kind:"all"},reuse:process.env.SEMIO_BUILD_REUSE_GRAPH==="true",graph:{policy:selected.graphPolicy??"fresh",...(command==="show"&&resolved.args[1]==="project"&&resolved.args[2]?{closure:[resolved.args[2]]}:{})}});')

# ---- help: the flag and its bilingual text ------------------------------------------------------------------------------------------------------------
help_flag = '    {\n      "name": "--scenario",\n'
flag = '''    {
      "name": "--graph-policy",
      "value": "required",
      "valueName": "fresh|current",
      "operations": ["graph", "show"],
      "description": {
        "en": "fresh (default) verifies the graph and waits for a rebuild when a project changed; current serves the newest sealed graph at once and never waits; otherwise SEMIO_BUILD_GRAPH_POLICY.",
        "de": "fresh (Standard) prüft den Graphen und wartet bei geänderten Projekten auf den Neuaufbau; current liefert sofort den neuesten versiegelten Graphen und wartet nie; sonst SEMIO_BUILD_GRAPH_POLICY."
      }
    },
'''
op(CATALOG, help_flag, flag + help_flag)
op(CATALOG, '      "flags": ["--file", "--json"],\n      "examples": [{ "command": "repo graph --file=graph.json"', '      "flags": ["--file", "--json", "--graph-policy"],\n      "examples": [{ "command": "repo graph --file=graph.json"')
op(CATALOG, '      "flags": ["--file", "--json"],\n      "examples": [\n        { "command": "repo show projects"', '      "flags": ["--file", "--json", "--graph-policy"],\n      "examples": [\n        { "command": "repo show projects"')

# ---- progress labels --------------------------------------------------------------------------------------------------------------------------------
def phases(path, indent, last_label, last_units, facts, assembly):
    unit_indent = " " * indent
    anchor = '%s"label": "%s",\n%s"units": "%s"\n%s}' % (unit_indent, last_label, unit_indent, last_units, " " * (indent - 2))
    block = '%s"label": "%s",\n%s"units": "%s"\n%s},\n%s"facts": {\n%s"label": "%s",\n%s"units": "%s"\n%s},\n%s"assembly": {\n%s"label": "%s",\n%s"units": "%s"\n%s}' % (
        unit_indent, last_label, unit_indent, last_units, " " * (indent - 2), " " * (indent - 2), unit_indent, facts[0], unit_indent, facts[1], " " * (indent - 2), " " * (indent - 2), unit_indent, assembly[0], unit_indent, assembly[1], " " * (indent - 2))
    op(path, anchor, block)


for path, indent in ((PROGRESS, 14), (PROGRESS_FIXTURE, 10)):
    phases(path, indent, "Expanding input closures", "closures", ("Verifying graph facts", "facts"), ("Assembling the graph", "projects"))
    phases(path, indent, "Eingabehüllen auflösen", "Hüllen", ("Graph-Fakten prüfen", "Fakten"), ("Graph zusammensetzen", "Projekte"))
cases = '''    },
    {
      "name": "en-facts-2048-of-182951",
      "environment": {
        "LANG": "en_US.UTF-8"
      },
      "command": "build:projects",
      "resource": "build:graph-phase:facts:2048:182951",
      "mode": "shared",
      "elapsedMs": 0,
      "expected": "build:projects · Verifying graph facts: 2048 of 182951 facts"
    },
    {
      "name": "de-assembly-3656-of-3656",
      "environment": {
        "LANG": "de_DE.UTF-8"
      },
      "command": "build:projects",
      "resource": "build:graph-phase:assembly:3656:3656",
      "mode": "shared",
      "elapsedMs": 0,
      "expected": "build:projects · Graph zusammensetzen: 3656 von 3656 Projekte"
    }
  ],
  "rejected": ['''
op(PROGRESS_FIXTURE, '    }\n  ],\n  "rejected": [', cases)

# ---- whole-file replacements (derived snapshots) ----------------------------------------------------------------------------------------------------------
replacements = []
if os.path.exists(os.path.join(STAGED, "new", "help", "fixture.json")):
    text = read(HELP_FIXTURE)
    replacements.append({"path": HELP_FIXTURE, "base": hashlib.sha256(text.encode("utf8")).hexdigest(), "source": "help/fixture.json"})
    bases[HELP_FIXTURE] = replacements[-1]["base"]

spec = {"version": 1, "ops": ops, "bases": bases, "new": NEW, "replace": replacements}
with open(os.path.join(STAGED, "ops.json"), "w", encoding="utf8") as handle:
    json.dump(spec, handle, ensure_ascii=False, indent=1)
print("ops %d, new %d, replacements %d, files %d" % (len(ops), len(NEW), len(replacements), len(bases)))
