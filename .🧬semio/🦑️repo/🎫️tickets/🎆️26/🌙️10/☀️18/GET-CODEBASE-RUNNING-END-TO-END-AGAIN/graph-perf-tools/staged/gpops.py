"""Anchored edit set of the graph-perf wave (module imported by apply.py and the benchmark harness).

Order matters: the show-perf set (C/show-perf-tools/staged) is applied first, then this wave's edits on top of the resulting text."""
import os, sys

HERE = os.path.dirname(os.path.abspath(__file__))


def E(code):
    return chr(code) + "️"


def find_root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")


ROOT = os.environ.get("GP_ROOT") or find_root()
LIBRARY = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
BUILD = os.path.join(LIBRARY, E(0x1F3D7) + "build")
SCRIPT_INPUTS = os.path.join(BUILD, E(0x1F578) + "graph", E(0x1F4E5) + "inference", E(0x1F9ED) + "script-inputs", E(0x1F7E8) + ".mjs")
INPUTS = os.path.join(BUILD, E(0x1F4E5) + "inputs", E(0x1F7E8) + ".mjs")
OBSERVATION = os.path.join(BUILD, E(0x1F441) + "observation", E(0x1F7E8) + ".mjs")
BOOTSTRAP = os.path.join(BUILD, E(0x1F680) + "bootstrap", E(0x1F7E6) + ".ts")
BOOTSTRAP_TEST = os.path.join(BUILD, E(0x1F680) + "bootstrap", E(0x1F4E6) + "parsers", E(0x1F9EA) + "tests", E(0x1F7E6) + ".ts")


def load_show_perf():
    path = os.path.join(HERE, "..", "..", "show-perf-tools", "staged", "apply.py")
    source = open(path, encoding="utf8").read()
    marker = "\nmain()"
    if marker not in source:
        raise SystemExit("show-perf apply.py no longer ends with main()")
    namespace = {"__file__": os.path.abspath(path), "__name__": "show_perf_ops"}
    exec(compile(source[: source.rindex(marker)], path, "exec"), namespace)
    return namespace["OPS"]


COVERED = '''/** \U0001F9EE️ True when a target input names a module the entry closure already compiled: adding it as a closure entry changes nothing, and its existence is already implied by the file fact the closure read. */
function coveredModuleInputV1(input, projectRoot, covered) {
  if (typeof input !== "string") return false;
  if (input.startsWith("{workspaceRoot}/")) return covered.has(input);
  if (input.startsWith("{projectRoot}/")) return covered.has(projectRoot === "." ? `{workspaceRoot}/${input.slice(14)}` : `{workspaceRoot}/${projectRoot}/${input.slice(14)}`);
  return false;
}

'''

GENERATED = '''/** \U0001F3ED️ The generated-output targets that the modules of one closure import: the absolute module set is derived once per distinct closure instead of once per target. */
function generatedTargetsV1(cache, closure, workspaceRoot) {
  let modules = cache.moduleSets.get(closure);
  if (!modules) cache.moduleSets.set(closure, modules = new Set(closure.map(path => resolve(workspaceRoot, path.slice("{workspaceRoot}/".length)))));
  return [...new Set([...cache.generatedImports].filter(([path]) => modules.has(path)).flatMap(([, rows]) => rows.map(row => row.target)))].sort();
}

'''

from bootstrap_ops import BOOTSTRAP_OPS

MINE = {
    SCRIPT_INPUTS: [
        ("declared:new Map(),paths:new Map(),",
         "declared:new Map(),paths:new Map(),compiledNames:new WeakMap(),moduleSets:new WeakMap(),"),
        ("const result = [...files].map((file) => closurePathNameV1(cache, workspaceRoot, file)).sort();",
         "const result = [...files].map((file) => closurePathNameV1(cache, workspaceRoot, file)).sort();\n  cache.compiledNames.set(result, new Set([...compiled].map((file) => closurePathNameV1(cache, workspaceRoot, file))));"),
        ('''function declaredModuleInputsV1(inputs, workspaceRoot, projectRoot, cache) {
  if (!Array.isArray(inputs) || !inputs.length) return [];
  return memoizedObservedComputationV1(cache.declared, memoDigestV1(workspaceRoot, projectRoot, inputs), () => inputs.flatMap(input => { const declared = declaredModuleInputV1(input, workspaceRoot, projectRoot); return declared ? [declared] : []; }));
}''',
         COVERED + '''function declaredModuleInputsV1(inputs, workspaceRoot, projectRoot, cache, covered) {
  if (!Array.isArray(inputs) || !inputs.length) return [];
  const pending = covered ? inputs.filter(input => !coveredModuleInputV1(input, projectRoot, covered)) : inputs;
  if (!pending.length) return [];
  return memoizedObservedComputationV1(cache.declared, memoDigestV1(workspaceRoot, projectRoot, pending), () => pending.flatMap(input => { const declared = declaredModuleInputV1(input, workspaceRoot, projectRoot); return declared ? [declared] : []; }));
}'''),
        ("for(const declared of declaredModuleInputsV1(target.inputs,workspaceRoot,projectRoot,scripts))entries.add(declared);",
         "const baseSize=entries.size,base=baseSize?await relativeScriptInputs([...entries],workspaceRoot,scripts,true):undefined;\n for(const declared of declaredModuleInputsV1(target.inputs,workspaceRoot,projectRoot,scripts,base&&scripts.compiledNames.get(base)))entries.add(declared);"),
        ("const closure=entries.size?await relativeScriptInputs([...entries],workspaceRoot,scripts,true):undefined,modules=new Set((closure??[]).map(path=>resolve(workspaceRoot,path.slice(\"{workspaceRoot}/\".length)))),generatedDependencies=[...new Set([...scripts.generatedImports].filter(([path])=>modules.has(path)).flatMap(([,rows])=>rows.map(row=>row.target)))].sort();",
         "const closure=entries.size===baseSize?base:entries.size?await relativeScriptInputs([...entries],workspaceRoot,scripts,true):undefined,generatedDependencies=scripts.generatedImports.size&&closure?generatedTargetsV1(scripts,closure,workspaceRoot):[];"),
        ("/** \U0001F9ED️ Tracks literal relative command imports through the existing TypeScript tooling boundary. */",
         GENERATED + "/** \U0001F9ED️ Tracks literal relative command imports through the existing TypeScript tooling boundary. */"),
    ],
    INPUTS: [
        ("function publishInputClosureV1(path,contract){",
         "const VERIFIED_CLOSURES_V1=new Map();\n\n/** \U0001F9FE️ Publishes identical content-addressed closure bytes atomically across independent graph writers; a closure whose stamp equals the one last verified equal is not read again. */\nfunction publishInputClosureV1(path,contract){"),
        ('if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1||row.size!==Buffer.byteLength(contract)||!readFileSync(path).equals(Buffer.from(contract)))throw Error("Original immutable input closure bytes required");',
         'if(!row.isFile()||row.isSymbolicLink()||row.nlink!==1||row.size!==Buffer.byteLength(contract))throw Error("Original immutable input closure bytes required");const known=VERIFIED_CLOSURES_V1.get(path);if(known&&known.ino===row.ino&&known.size===row.size&&known.mtimeMs===row.mtimeMs&&known.ctimeMs===row.ctimeMs)return;if(!readFileSync(path).equals(Buffer.from(contract)))throw Error("Original immutable input closure bytes required");VERIFIED_CLOSURES_V1.set(path,{ino:row.ino,size:row.size,mtimeMs:row.mtimeMs,ctimeMs:row.ctimeMs});'),
    ],
    BOOTSTRAP: BOOTSTRAP_OPS,
    BOOTSTRAP_TEST: [("install:async()=>{installs++;}}", "install:async()=>{installs++;},recheck:async()=>false}")],
    OBSERVATION: [
        ('import { brotliCompressSync, brotliDecompressSync, constants as zlibConstants } from "node:zlib";',
         'import { brotliCompress, brotliCompressSync, brotliDecompressSync, constants as zlibConstants } from "node:zlib";'),
        ("const packInferenceRecordV1=bytes=>brotliCompressSync(bytes,{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:5,[zlibConstants.BROTLI_PARAM_LGWIN]:22,[zlibConstants.BROTLI_PARAM_SIZE_HINT]:bytes.length}});",
         "const packInferenceRecordV1=bytes=>new Promise((accept,reject)=>brotliCompress(bytes,{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:5,[zlibConstants.BROTLI_PARAM_LGWIN]:22,[zlibConstants.BROTLI_PARAM_SIZE_HINT]:bytes.length}},(error,packed)=>error?reject(error):accept(packed)));"),
        ("const packed=packInferenceRecordV1(Buffer.from(`${header}\\n${image}`)),bytes=packed.byteLength;",
         "const packed=await packInferenceRecordV1(Buffer.from(`${header}\\n${image}`)),bytes=packed.byteLength;"),
        ("const internedObservations=new Map(),INTERNED_OBSERVATIONS=4194304;",
         "const internedObservations=new Map(),INTERNED_OBSERVATIONS=4194304;\n\n/** \U0001F9FE️ The JSON text of one observation row, serialised once per row object: a header costs concatenation, not the repeated serialisation of rows that thousands of projects share. */\nconst observationJsons=new WeakMap();\nfunction observationRowsJsonV1(rows){return \"[\"+rows.map(row=>{let text=observationJsons.get(row);if(text===undefined){text=JSON.stringify(row);observationJsons.set(row,text);}return text;}).join(\",\")+\"]\";}"),
        ('header = JSON.stringify({ digest, resultSha256:createHash("sha256").update(image).digest("hex"), authorityObservations: authority, observations: rows });',
         'header = `{"digest":${JSON.stringify(digest)},"resultSha256":"${createHash("sha256").update(image).digest("hex")}","authorityObservations":${observationRowsJsonV1(authority)},"observations":${observationRowsJsonV1(rows)}}`;'),
        ("projectImages.set(cachePath,{identity,image:JSON.stringify(value),",
         "projectImages.set(cachePath,{identity,image:Buffer.from(JSON.stringify(value)),"),
        ("const value=JSON.parse(previous.image);",
         'const value=JSON.parse(previous.image.toString("utf8"));'),
    ],
}


BOOTSTRAP_TESTS = os.path.join(BUILD, E(0x1F680) + "bootstrap", E(0x1F9EA) + "tests")
WORKERS = os.path.join(BUILD, E(0x1F9F5) + "workers")
WORKER_TESTS = os.path.join(WORKERS, E(0x1F9EA) + "tests")
COVERED_TESTS = os.path.join(BUILD, E(0x1F578) + "graph", E(0x1F4E5) + "inference", E(0x1F9EA) + "tests", E(0x1F9EE) + "covered-inputs")
HEADER_TESTS = os.path.join(BUILD, E(0x1F441) + "observation", E(0x1F9EA) + "tests", E(0x1F9FE) + "record-header")
NEW = {
    "header/test.ts": os.path.join(HEADER_TESTS, E(0x1F7E6) + ".ts"),
    "header/fixture.json": os.path.join(HEADER_TESTS, E(0x1F523) + ".json"),
    "header/schema.json": os.path.join(HEADER_TESTS, E(0x1F9EC) + "schema", E(0x1F523) + ".json"),
    "covered/test.ts": os.path.join(COVERED_TESTS, E(0x1F7E6) + ".ts"),
    "covered/fixture.json": os.path.join(COVERED_TESTS, E(0x1F523) + ".json"),
    "covered/schema.json": os.path.join(COVERED_TESTS, E(0x1F9EC) + "schema", E(0x1F523) + ".json"),
    "workers/host.mjs": os.path.join(WORKERS, E(0x1F7E8) + ".mjs"),
    "workers/host.d.mts": os.path.join(WORKERS, E(0x1F7E8) + ".d.mts"),
    "workers/worker.mjs": os.path.join(WORKERS, E(0x1F477) + "worker", E(0x1F7E8) + ".mjs"),
    "workers/schema.json": os.path.join(WORKERS, E(0x1F9EC) + "schema", E(0x1F523) + ".json"),
    "workers/test.ts": os.path.join(WORKER_TESTS, E(0x1F7E6) + ".ts"),
    "workers/fixture.json": os.path.join(WORKER_TESTS, E(0x1F523) + ".json"),
    "workers/fixture-schema.json": os.path.join(WORKER_TESTS, E(0x1F9EC) + "schema", E(0x1F523) + ".json"),
    "workers/tasks.mjs": os.path.join(WORKER_TESTS, E(0x1F9F0) + "tasks", E(0x1F7E8) + ".mjs"),
    "bootstrap/test.ts": os.path.join(BOOTSTRAP_TESTS, E(0x1F7E6) + ".ts"),
    "bootstrap/fixture.json": os.path.join(BOOTSTRAP_TESTS, E(0x1F523) + ".json"),
    "bootstrap/schema.json": os.path.join(BOOTSTRAP_TESTS, E(0x1F9EC) + "schema", E(0x1F523) + ".json"),
}

SEAL = os.path.join(LIBRARY, E(0x26A1) + "caching", E(0x1F578) + "graph", E(0x1F50F) + "seal")
MEMBERSHIP = os.path.join(SEAL, E(0x1F4E5) + "membership")
NEW.update({
    "seal/census.mjs": os.path.join(SEAL, E(0x1F9EE) + "census", E(0x1F7E8) + ".mjs"),
    "seal/census.d.mts": os.path.join(SEAL, E(0x1F9EE) + "census", E(0x1F7E8) + ".d.mts"),
    "seal/facts.ts": os.path.join(SEAL, E(0x1F4E6) + "facts", E(0x1F7E6) + ".ts"),
    "seal/facts-test.ts": os.path.join(SEAL, E(0x1F4E6) + "facts", E(0x1F9EA) + "tests", E(0x1F7E6) + ".ts"),
    "seal/facts-fixture.json": os.path.join(SEAL, E(0x1F4E6) + "facts", E(0x1F9EA) + "tests", E(0x1F523) + ".json"),
    "seal/facts-fixture-schema.json": os.path.join(SEAL, E(0x1F4E6) + "facts", E(0x1F9EA) + "tests", E(0x1F9EC) + "schema", E(0x1F523) + ".json"),
    "seal/membership-schema.json": os.path.join(MEMBERSHIP, E(0x1F9EC) + "schema", E(0x1F523) + ".json"),
})
REPLACE = {
    os.path.join(SEAL, E(0x1F9EA) + "tests", E(0x1F7E6) + ".ts"): ("seal/seal-test.ts", "seal/seal-test.base.sha256"),
    os.path.join(SEAL, E(0x1F9EA) + "tests", E(0x1F9EB) + "fixtures", E(0x1F523) + ".json"): ("seal/seal-fixture.json", "seal/seal-fixture.base.sha256"),
    os.path.join(SEAL, E(0x1F7E6) + ".ts"): ("seal/seal.ts", "seal/seal.base.sha256"),
    os.path.join(SEAL, E(0x1F9EC) + "schema", E(0x1F523) + ".json"): ("seal/schema.json", "seal/schema.base.sha256"),
}
MEMBERSHIP_OPS = [
    ('import ownership from "../../../../\U0001F5C2️workspaces/\U0001F523️policy.json" with {type:"json"};',
     'import ownership from "../../../../\U0001F5C2️workspaces/\U0001F523️policy.json" with {type:"json"};\nimport protocol from "./\U0001F9EC️schema/\U0001F523️.json" with {type:"json"};\nconst inventory=protocol.definitions.Protocol.const;'),
    ('finish(Error("Graph membership inventory exceeded ten seconds"));},10_000);',
     'finish(Error(`Graph membership inventory exceeded ${Math.round(inventory.readDeadlineMilliseconds/1000)} seconds`));},inventory.readDeadlineMilliseconds);'),
    ('if(bytes>64*1024*1024){child.kill();finish(Error("Graph membership inventory exceeds64MiB"));}',
     'if(bytes>inventory.maximumInventoryBytes){child.kill();finish(Error("Graph membership inventory exceeds "+inventory.maximumInventoryBytes+" bytes"));}'),
]


def combined():
    base = load_show_perf()
    merged = {}
    for path, ops in base.items():
        merged.setdefault(path, []).extend(ops)
    for path, ops in MINE.items():
        merged.setdefault(path, []).extend(ops)
    merged[os.path.join(MEMBERSHIP, E(0x1F7E6) + ".ts")] = MEMBERSHIP_OPS
    return merged


OPS = combined()
