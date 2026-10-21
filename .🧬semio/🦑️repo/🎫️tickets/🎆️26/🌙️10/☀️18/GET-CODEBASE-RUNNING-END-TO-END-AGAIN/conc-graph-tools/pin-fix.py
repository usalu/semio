#!/usr/bin/env python3
"""Implementation pinning: a running build completes with the implementation it LOADED and publishes a generation that names it.

 1. `withImplementationPinV1` / `implementationPinnedV1` (revision module): while pinned, every loaded-Source self-check lets the invocation finish (12 sites + G + the inference router).
 2. The generation identity no longer contains the implementation: `identity` = registry only. The manifest carries `implementation` (digest of the loaded implementation facts);
    every record carries the facts (workspace relative `digest` rows) of the implementation modules it consumes: inert modules (ledger) none, node shaping modules the valid projects, all others every owner.
 3. `current` serves a generation of another implementation (message `stale`), `fresh` verifies the rows and refreshes exactly the consumers of an edited module.
 4. The CLI door and the generation branch of `loadBuildGraphV1` run pinned.

usage: pin-fix.py [--root REPO_ROOT] [--write]    (dry run unless --write; idempotent; stdlib only)"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
if "--root" in sys.argv:
    ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
WRITE = "--write" in sys.argv
BARE = lambda text: text.replace("️", "")


def resolve(rel, base=ROOT):
    current = base
    for part in rel.split("/"):
        if not part:
            continue
        hits = [name for name in os.listdir(current) if BARE(name) == BARE(part)]
        if len(hits) != 1:
            raise SystemExit("cannot resolve %r under %s (%d hits)" % (part, current, len(hits)))
        current = os.path.join(current, hits[0])
    return current


LIB = resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library")
lib = lambda rel: resolve(rel, LIB)
rel = lambda path: os.path.relpath(path, ROOT)
BUILD, GRAPH = lib("🏗️build"), lib("🏗️build/🕸️graph")
GEN = lib("🏗️build/🕸️graph/🕰️generations")
REVISION = lib("🏗️build/📥️inputs/🔏️revision/🟨️.mjs")
TS, MJS, DMTS = [os.path.basename(lib("🏗️build/🕸️graph/🟦️.ts")), os.path.basename(REVISION), None]
INFER = lib("🏗️build/🕸️graph/📥️inference")
dir_of = lambda path: os.path.dirname(path)


def import_path(from_file, to_file):
    out = os.path.relpath(to_file, os.path.dirname(from_file)).replace(os.sep, "/")
    return out if out.startswith(".") else "./" + out


def lib_prefix(rel_dir):
    return os.path.relpath(lib(rel_dir), LIB).replace(os.sep, "/") + "/"


NONE_PREFIXES = [lib_prefix("🏗️build/📥️inputs/💾️storage/📒️ledger")]
PROJECT_PREFIXES = [lib_prefix("🏗️build/🕸️graph/📥️inference/" + name) for name in ["📦️project-nodes", "🎯️target-policy", "🎮️playground-targets", "🧩️component-targets", "🏭️generator-outputs"]]
J = lambda items: "[" + ",".join(json.dumps(item, ensure_ascii=False) for item in items) + "]"

changes = {}


def text_of(path):
    if path not in changes:
        with open(path, encoding="utf8", newline="") as handle:
            changes[path] = [handle.read(), None]
    return changes[path]


def edit(path, marker, pattern, replacement, flags=0, literal=False):
    row = text_of(path)
    state = row[0]
    done = (replacement in state) if isinstance(replacement, str) else (marker in state)
    if done:
        print("applied  ", rel(path)[-76:], "|", marker[:40])
        return
    matches = list(re.finditer(re.escape(pattern) if literal else pattern, state, flags))
    if len(matches) != 1:
        raise SystemExit("DRIFT %s: %r occurs %d times" % (rel(path)[-70:], pattern[:70], len(matches)))
    row[0] = state[: matches[0].start()] + (replacement(matches[0]) if callable(replacement) else replacement) + state[matches[0].end():]
    row[1] = "pending"
    print("pending  ", rel(path)[-76:], "|", marker[:40])


def append(path, marker, text):
    row = text_of(path)
    if marker in row[0]:
        print("applied  ", rel(path)[-76:], "|", marker[:40])
        return
    row[0] = row[0] + ("" if row[0].endswith("\n") else "\n") + text
    row[1] = "pending"
    print("pending  ", rel(path)[-76:], "| append", marker[:30])


def json_edit(path, marker, change):
    row = text_of(path)
    data = json.loads(row[0])
    canon = json.dumps(data, ensure_ascii=False, indent=1)
    if row[0] not in (canon, canon + "\n"):
        raise SystemExit("json round trip differs: " + path)
    if marker in row[0]:
        print("applied  ", rel(path)[-76:], "|", marker[:40])
        return
    change(data)
    row[0] = json.dumps(data, ensure_ascii=False, indent=1) + ("\n" if row[0].endswith("\n") else "")
    row[1] = "pending"
    print("pending  ", rel(path)[-76:], "|", marker[:40])


# 1 revision module -------------------------------------------------------------------------------------------------------------------------------------------------------------
edit(REVISION, "withImplementationPinV1", r"function current\(\)\{if\(capture\(ownPath,loaded\.byteLength,false\)", lambda m: (
    'const pinKeyV1=Symbol.for("semio.implementationPin.v1");\n'
    "/** 📍️ True while an invocation is committed to the implementation it loaded: every self-check of a loaded Source then lets the invocation finish, and what it publishes names the loaded bytes, so a Source edited meanwhile is left to the next invocation. */\n"
    "export function implementationPinnedV1(){return(globalThis[pinKeyV1]??0)>0;}\n"
    "/** 📌️ Runs `body` committed to the loaded implementation; pins nest and a failing body releases its pin. */\n"
    "export async function withImplementationPinV1(body){globalThis[pinKeyV1]=(globalThis[pinKeyV1]??0)+1;try{return await body();}finally{globalThis[pinKeyV1]--;}}\n"
    "function current(){if(implementationPinnedV1())return;if(capture(ownPath,loaded.byteLength,false)"))
REVISION_D = os.path.join(dir_of(REVISION), [n for n in os.listdir(dir_of(REVISION)) if n.endswith(".d.mts")][0])
append(REVISION_D, "withImplementationPinV1",
       "/** 📍️ True while an invocation is committed to the implementation it loaded, so no self-check refuses a Source edited since. */\n"
       "export function implementationPinnedV1():boolean;\n"
       "/** 📌️ Runs `body` committed to the implementation this process loaded; pins nest and a failing body releases its pin. */\n"
       "export function withImplementationPinV1<T>(body:()=>Promise<T>|T):Promise<T>;\n")

# 2 every loaded-Source self-check ------------------------------------------------------------------------------------------------------------------------------------------------
SITES = [
    (lib("🏗️build/📥️inputs/📋️data/📥️load/🟨️.mjs"), "function requireCurrentSourceInventoryV1(){", "if(!loadedExact"),
    (lib("🏗️build/📥️inputs/🧮️digest/📥️read/🟨️.mjs"), "export function requireCurrentExactSourceReadV1(){"),
    (lib("🏗️build/📥️inputs/💾️storage/🟨️.mjs"), "export function requireCurrentRepositoryBuildGraphConsumerV1(){"),
    (lib("🏗️build/👁️observation/📥️reuse/🟨️.mjs"), "export function requireCurrentInferenceInventoryReuseV1(){"),
    (lib("🏗️build/👁️observation/🟨️.mjs"), "export function requireCurrentObservationV1(){"),
    (lib("🏗️build/🕸️graph/🦀️rust/🧩️assembly/🟦️.ts"), "function current():void{"),
    (lib("⚡️caching/🦀️cargo/📦️units/📥️descriptor/📖️read/🟦️.ts"), "function requireCurrent():void{"),
    (lib("⚡️caching/🦀️cargo/📦️units/📥️descriptor/💾️cache/📤️publication/🟦️.ts"), "function requireCurrent():void{"),
    (lib("⚡️caching/🦀️cargo/📦️units/📥️descriptor/🎯️selection/📥️configuration/🟦️.ts"), "function requireCurrent():void{"),
    (lib("⚡️caching/🦀️cargo/📦️units/📤️receipt/🟦️.ts"), "function requireCurrent():void{"),
    (lib("⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts"), "export function requireCurrentRepositoryCacheStoreV1():void{"),
]
SITES = [row if len(row) == 3 else row + ("",) for row in SITES]
for path, head, tail in SITES:
    edit(path, head + "if(implementationPinnedV1())return;", head + tail, head + "if(implementationPinnedV1())return;" + tail, literal=True)
    edit(path, "import{captureBoundedSourceRevisionV1,implementationPinnedV1,", "import{captureBoundedSourceRevisionV1,", "import{captureBoundedSourceRevisionV1,implementationPinnedV1,", literal=True)

# 3 inference: the router keeps the loaded revision while pinned; the loaded implementation is listed as facts -----------------------------------------------------------------------
REVCOMP = lib("🏗️build/🕸️graph/📥️inference/♻️revision/🟨️.mjs")
edit(REVCOMP, "function implementationFiles()", r"(/\*\* [^\n]*\*/\n)(export function implementationRevision\(\) \{\n)", lambda m: (
    "/** 🗂️ Names the authored files beside the component sources whose bytes the implementation revision covers. */\n"
    "function implementationFiles() {\n  return [" + re.search(r"  for \(const path of \[(.*?)\]\) hash\.update\(path === ", text_of(REVCOMP)[0]).group(1) + "];\n}\n\n" + m.group(1) + m.group(2)), re.S)
edit(REVCOMP, "of implementationFiles()) hash.update", r"  for \(const path of \[(?!\))[^\n]*?\]\) hash\.update\(path === ", "  for (const path of implementationFiles()) hash.update(path === ")
edit(REVCOMP, "export function implementationRows()", r'  hash\.update\(JSON\.stringify\(buildModesV1\(\)\)\);\n  return hash\.digest\("hex"\);\n\}\n', lambda m: (
    '  hash.update(JSON.stringify(buildModesV1()));\n  const digest = hash.digest("hex");\n  loadedRevision ??= digest;\n  return digest;\n}\n'
    "\n/** 📋️ Lists the authored files of the implementation revision as workspace facts of the bytes they have right now; the taxonomy is a data input of the environment and is not listed. */\n"
    "export function implementationRows() {\n"
    "  const rows = new Map();\n"
    '  for (const path of [...services.sourceFiles, ...implementationFiles().filter((entry) => !entry.endsWith("taxonomy.json")).map((entry) => resolve(LIBRARY_ROOT, entry))]) {\n'
    "    if (rows.has(path)) continue;\n"
    "    const status = lstatSync(path), bytes = readPhysicalSource(path);\n"
    '    rows.set(path, { kind: "digest", path, fact: { sha256: createHash("sha256").update(bytes).digest("hex"), size: status.size, mtimeMs: status.mtimeMs } });\n'
    "  }\n"
    "  return [...rows.values()];\n}\n"))
edit(REVCOMP, "if (implementationPinnedV1() && loadedRevision !== undefined) return loadedRevision;", r"export function implementationRevision\(\) \{\n", "export function implementationRevision() {\n  if (implementationPinnedV1() && loadedRevision !== undefined) return loadedRevision;\n")
edit(REVCOMP, "let loadedRevision;", 'export const component = "revision";\n', 'export const component = "revision";\n/** 🧷️ The revision this component instance computed first: a pinned invocation keeps it while the Source on disk moves on. */\nlet loadedRevision;\n', literal=True)
edit(REVCOMP, 'import { implementationPinnedV1 } from', 'import { createHash } from "node:crypto";', lambda m: m.group(0) + '\nimport { implementationPinnedV1 } from "' + import_path(REVCOMP, REVISION) + '";')
edit(REVCOMP, 'import { lstatSync } from "node:fs";', 'import { createHash } from "node:crypto";', lambda m: m.group(0) + '\nimport { lstatSync } from "node:fs";')
INDEX = os.path.join(INFER, [n for n in os.listdir(INFER) if n.endswith(".mjs")][0])
edit(INDEX, "loadedImplementationRowsV1", r"loader\.adopt\(revisionIdentity\.revision, current\);\n", lambda m: m.group(0) + (
    "\n/** 📋️ The authored files of the implementation this module loaded with the bytes they had at load: the facts a generation names for every project it built. */\n"
    "export const loadedImplementationRowsV1 = Object.freeze(current.modules.revision.implementationRows());\n"))
INDEX_D = os.path.join(INFER, [n for n in os.listdir(INFER) if n.endswith(".d.mts")][0])
append(INDEX_D, "loadedImplementationRowsV1",
       "/** 📋️ The authored files of the implementation this module loaded with the bytes they had at load. */\n"
       "export const loadedImplementationRowsV1:readonly {readonly kind:\"digest\";readonly path:string;readonly fact:{readonly sha256:string;readonly size:number;readonly mtimeMs:number}}[];\n")

# 4 the generation store ---------------------------------------------------------------------------------------------------------------------------------------------------------------
STORE = os.path.join(GEN, MJS)
edit(STORE, "implementation:Object.freeze({none:", "brotliQuality:5,brotliWindow:22,\n", "brotliQuality:5,brotliWindow:22,\n implementation:Object.freeze({none:Object.freeze(" + J(NONE_PREFIXES) + "),projects:Object.freeze(" + J(PROJECT_PREFIXES) + ")}),\n", literal=True)
edit(STORE, '"identity","implementation","env"', 'requireKeys(value,["version","generation","parent","identity","env"', 'requireKeys(value,["version","generation","parent","identity","implementation","env"', literal=True)
edit(STORE, 'for(const key of["identity","implementation","env"', 'for(const key of["identity","env"', 'for(const key of["identity","implementation","env"', literal=True)
edit(STORE, "let manifest;try{manifest=readGenerationManifestV1(bytes);}", 'const manifest=readGenerationManifestV1(bytes);if(manifest.generation!==pointer.generation)', 'let manifest;try{manifest=readGenerationManifestV1(bytes);}catch{return{pointer,manifest:undefined};}if(manifest.generation!==pointer.generation)', literal=True)
edit(STORE, "async function build({identity,implementation,", "async function build({identity,membership,", "async function build({identity,implementation,membership,", literal=True)
edit(STORE, "parent:current?.pointer.manifest??null,identity,implementation,env:envDigest", "parent:current?.pointer.manifest??null,identity,env:envDigest", "parent:current?.pointer.manifest??null,identity,implementation,env:envDigest", literal=True)
edit(STORE, "async function refresh({identity,implementation,", 'async function refresh({identity,policy="fresh",only,', 'async function refresh({identity,implementation,policy="fresh",only,', literal=True)
edit(STORE, "const body=()=>build({identity,implementation,", "const body=()=>build({identity,membership,compute,signal,progress,hint});", "const body=()=>build({identity,implementation,membership,compute,signal,progress,hint});", literal=True)
edit(STORE, "async function refreshClosure({roots,neighbors,identity,implementation,", 'async function refreshClosure({roots,neighbors,identity,policy="fresh"', 'async function refreshClosure({roots,neighbors,identity,implementation,policy="fresh"', literal=True)
edit(STORE, "refresh({identity,implementation,policy,only:names", "refresh({identity,policy,only:names", "refresh({identity,implementation,policy,only:names", literal=True)
STORE_D = os.path.join(GEN, [n for n in os.listdir(GEN) if n.endswith(".d.mts")][0])
edit(STORE_D, "readonly implementation:string;readonly env", "readonly identity:string;readonly env:string", "readonly identity:string;readonly implementation:string;readonly env:string", literal=True)
edit(STORE_D, "interface GenerationRefreshOptionsV1{identity:string;implementation:string;", "interface GenerationRefreshOptionsV1{identity:string;", "interface GenerationRefreshOptionsV1{identity:string;implementation:string;", literal=True)

edit(STORE_D, "readonly implementation:{readonly none:", "export interface GenerationPolicyV1{readonly version:1;", "export interface GenerationPolicyV1{readonly version:1;readonly implementation:{readonly none:readonly string[];readonly projects:readonly string[]};", literal=True)
SCHEMA = os.path.join(lib("🏗️build/🕸️graph/🕰️generations/🧬️schema"), os.listdir(lib("🏗️build/🕸️graph/🕰️generations/🧬️schema"))[0])


def schema_change(data):
    definitions = data["definitions"]
    definitions["Policy"]["const"]["implementation"] = {"none": NONE_PREFIXES, "projects": PROJECT_PREFIXES}
    definitions["Messages"]["const"]["en"]["stale"] = "graph: serving generation {generation} built by another implementation (graph policy current)"
    definitions["Messages"]["const"]["de"]["stale"] = "Graph: Generation {generation} stammt von einer anderen Implementierung und wird geliefert (Graph-Richtlinie current)"
    manifest = definitions["Manifest"]
    manifest["required"].insert(manifest["required"].index("identity") + 1, "implementation")
    properties = {}
    for key, value in manifest["properties"].items():
        properties[key] = value
        if key == "identity":
            properties["implementation"] = dict(value)
    manifest["properties"] = properties


json_edit(SCHEMA, '"implementation": {', schema_change)

# 5 the load module --------------------------------------------------------------------------------------------------------------------------------------------------------------------
LOAD = os.path.join(lib("🏗️build/🕸️graph/🕰️generations/🔌️load"), MJS)
LOAD_D = os.path.join(lib("🏗️build/🕸️graph/🕰️generations/🔌️load"), [n for n in os.listdir(lib("🏗️build/🕸️graph/🕰️generations/🔌️load")) if n.endswith(".d.mts")][0])
edit(LOAD, "canonicalJsonV1,createGenerationStoreV1", "import{GENERATION_POLICY_V1,createGenerationStoreV1,rowKeyV1}from", 'import{createHash}from"node:crypto";\nimport{GENERATION_POLICY_V1,canonicalJsonV1,compareCodePointsV1,createGenerationStoreV1,rowKeyV1}from', literal=True)
edit(LOAD, "stale:\"graph: serving", '  cold:"graph: no usable generation ({reason}), building the first one",\n', '  cold:"graph: no usable generation ({reason}), building the first one",\n  stale:"graph: serving generation {generation} built by another implementation (graph policy current)",\n', literal=True)
edit(LOAD, "stale:\"Graph: Generation", '  cold:"Graph: keine brauchbare Generation ({reason}), die erste wird aufgebaut",\n', '  cold:"Graph: keine brauchbare Generation ({reason}), die erste wird aufgebaut",\n  stale:"Graph: Generation {generation} stammt von einer anderen Implementierung und wird geliefert (Graph-Richtlinie current)",\n', literal=True)
edit(LOAD, "export function scopeImplementationRowsV1", r"/\*\*\n \* [^\n]{0,8}Shapes what one partial inference", lambda m: (
    "/** 🧱️ Splits the facts of the loaded implementation by their consumers: a module no record can depend on is dropped, a module that only shapes the nodes of valid projects belongs to those projects, every other module to every project, every unavailable project and the environment; identical facts are kept once. */\n"
    "export function scopeImplementationRowsV1(rows){\n"
    " const scopes=GENERATION_POLICY_V1.implementation,all=[],projects=[],seen=new Set();\n"
    " for(const row of rows){\n"
    "  const key=rowKeyV1(row);if(seen.has(key)||scopes.none.some(prefix=>row.path.includes(prefix)))continue;seen.add(key);\n"
    "  (scopes.projects.some(prefix=>row.path.includes(prefix))?projects:all).push(row);\n"
    " }\n"
    " return{all,projects};\n"
    "}\n\n"
    "/** 🪪️ The digest that stamps a generation: the loaded bytes of every module of the implementation, whoever consumes it. */\n"
    "export function implementationDigestV1(rows){\n"
    " return createHash(\"sha256\").update(canonicalJsonV1(rows.map(row=>[row.path,row.fact.sha256]).sort((a,b)=>compareCodePointsV1(a[0],b[0])||compareCodePointsV1(a[1],b[1])))).digest(\"hex\");\n"
    "}\n\n" + m.group(0)), literal=False)
edit(LOAD, "implementation={all:[],projects:[]}", 'export function shapeComputeResultV1({workspaceRoot,part,stubs,separator="/"}){', 'export function shapeComputeResultV1({workspaceRoot,part,stubs,separator="/",implementation={all:[],projects:[]}}){', literal=True)
edit(LOAD, "...implementation.all,...(owner.unavailable", "rows:claim(owner.nodeRows??[],[owner.root,owner.nativeRoot])})", "rows:[...claim(owner.nodeRows??[],[owner.root,owner.nativeRoot]),...implementation.all,...(owner.unavailable?[]:implementation.projects)]})", literal=True)
edit(LOAD, "...part.rows.filter(row=>!claimed.has(rowKeyV1(row))),...implementation.all]", "...part.rows.filter(row=>!claimed.has(rowKeyV1(row)))];", "...part.rows.filter(row=>!claimed.has(rowKeyV1(row))),...implementation.all];", literal=True)
edit(LOAD, "const rows=await host.implementationRows()", "const identity=await host.identity(),roots=closure?[...new Set(closure)]:undefined;", "const identity=await host.identity(),roots=closure?[...new Set(closure)]:undefined,rows=await host.implementationRows(),implementation=implementationDigestV1(rows),scoped=scopeImplementationRowsV1(rows);", literal=True)
edit(LOAD, "stubs:spec.stubs,implementation:scoped})", "stubs:spec.stubs})", "stubs:spec.stubs,implementation:scoped})", literal=True)
edit(LOAD, "const base={identity,implementation,policy", "const base={identity,policy", "const base={identity,implementation,policy", literal=True)
edit(LOAD, 'host.say(result.current.manifest.implementation===implementation?"current":"stale"', 'if(result.served==="current")host.say("current",{generation});', 'if(result.served==="current")host.say(result.current.manifest.implementation===implementation?"current":"stale",{generation});', literal=True)
row = text_of(LOAD_D)
if '"cold"|"stale"' not in row[0]:
    row[0] = row[0].replace('|"closure"|"cold",string', '|"closure"|"cold"|"stale",string')
    row[1] = "pending"
edit(LOAD_D, "implementation?:{readonly all:", "separator?:string}):GenerationComputeResultV1;", "separator?:string;implementation?:ScopedImplementationRowsV1}):GenerationComputeResultV1;", literal=True)
edit(LOAD_D, "ScopedImplementationRowsV1{", r"/\*\* [^\n]{0,8}What one partial inference produced", lambda m: "/** 🧱️ The facts of the loaded implementation by consumer. */\nexport interface ScopedImplementationRowsV1{readonly all:readonly GenerationRowV1[];readonly projects:readonly GenerationRowV1[]}\n/** 🧱️ Splits implementation facts by consumer. */\nexport function scopeImplementationRowsV1(rows:readonly GenerationRowV1[]):ScopedImplementationRowsV1;\n/** 🪪️ Digest that stamps a generation. */\nexport function implementationDigestV1(rows:readonly GenerationRowV1[]):string;\n" + m.group(0))
edit(LOAD_D, "implementationRows():", " identity():Promise<string>;\n", " identity():Promise<string>;\n implementationRows():Promise<readonly GenerationRowV1[]>;\n", literal=True)

# 6 graph glue -------------------------------------------------------------------------------------------------------------------------------------------------------------------------
G = os.path.join(GRAPH, TS)
edit(G, "implementationPinnedV1,withImplementationPinV1", "import{captureBoundedSourceRevisionV1,capturePhysicalSourceTextV1,physicalSourceOriginV1,", "import{captureBoundedSourceRevisionV1,capturePhysicalSourceTextV1,implementationPinnedV1,physicalSourceOriginV1,withImplementationPinV1,", literal=True)
edit(G, 'import {constants,lstatSync} from "node:fs";', 'import {constants} from "node:fs";', 'import {constants,lstatSync} from "node:fs";', literal=True)
edit(G, "function requireCurrentGraphReadPoliciesV1():void{if(implementationPinnedV1())return;", "function requireCurrentGraphReadPoliciesV1():void{", "function requireCurrentGraphReadPoliciesV1():void{if(implementationPinnedV1())return;", literal=True)


def authority_split(m):
    line = m.group(0)
    start = line.index("for(const path of[") + len("for(const path of[")
    end = line.index("])implementation.update(")
    items = line[start:end]
    return ("/** 🗂️ The graph implementation files whose bytes the module authority covers, and whose loaded bytes every generation names. */\n"
            "const graphImplementationUrlsV1=():URL[]=>[" + items + "];\n" + line[:start - len("for(const path of[")] + "for(const path of graphImplementationUrlsV1())implementation.update(" + line[end + len("])implementation.update("):])


edit(G, "const graphImplementationUrlsV1", r"^const graphModuleAuthorityV1=\(\):BuildGraphModuleAuthorityV1=>\{.*$", authority_split, re.M)
edit(G, "const loadedGraphImplementationRowsV1", r"const loadedGraphModuleAuthorityV1=graphModuleAuthorityV1\(\);\n", lambda m: m.group(0) + (
    "/** 📋️ The loaded graph implementation as workspace facts of the bytes it had when this module loaded; a pinned build names these and no later edit. */\n"
    "const loadedGraphImplementationRowsV1:readonly {kind:\"digest\";path:string;fact:{sha256:string;size:number;mtimeMs:number}}[]=graphImplementationUrlsV1().map(url=>{const path=fileURLToPath(url),status=lstatSync(path),captured=captureBoundedSourceRevisionV1(path,8388608);return {kind:\"digest\" as const,path,fact:{sha256:captured.sha256,size:status.size,mtimeMs:status.mtimeMs}};});\n"))
edit(G, "const requireCurrentGraphModuleV1=():void=>{if(implementationPinnedV1())return;", "const requireCurrentGraphModuleV1=():void=>requireBuildGraphModuleAuthorityV1(loadedGraphModuleAuthorityV1,graphModuleAuthorityV1());", "const requireCurrentGraphModuleV1=():void=>{if(implementationPinnedV1())return;requireBuildGraphModuleAuthorityV1(loadedGraphModuleAuthorityV1,graphModuleAuthorityV1());};", literal=True)
edit(G, "async implementationRows(){", r"    async identity\(\)\{const producer=await import\([^\n]*?\.digest\(\"hex\"\);\},\n", lambda m: (
    '    async identity(){return createHash("sha256").update(JSON.stringify(["graph-generations-v2",registryDigest])).digest("hex");},\n'
    '    async implementationRows(){const producer=await import("./' + os.path.basename(INFER) + '/' + os.path.basename(INDEX) + '");await producer.libraryBootstrap;const rows:{kind:"digest";path:string;fact:{sha256:string;size:number;mtimeMs:number}}[]=[];for(const row of [...loadedGraphImplementationRowsV1,...producer.loadedImplementationRowsV1]){const inside=relative(root,row.path);if(inside===""||inside.startsWith("..")||isAbsolute(inside))continue;rows.push({...row,path:inside.split(sep).join("/")});}return rows;},\n'))
LOADFN_TYPE = ("type BuildGraphLoadOptionsV1={selection:BuildGraphSelectionV1;reuse?:boolean;baseline?:boolean;graph?:{policy?:\"fresh\"|\"current\";closure?:readonly string[]}};\n")
edit(G, "loadBuildGraphUnpinnedV1", r"export async function loadBuildGraphV1\(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:\{selection:BuildGraphSelectionV1;reuse\?:boolean;baseline\?:boolean;graph\?:\{policy\?:\"fresh\"\|\"current\";closure\?:readonly string\[\]\}\}\):Promise<BuildGraphV1> \{\n", lambda m: (
    "export async function loadBuildGraphV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {\n"
    "  if(options.selection?.kind===\"all\"&&!options.baseline&&!options.reuse&&invocation)return withImplementationPinV1(()=>loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options));\n"
    "  return loadBuildGraphUnpinnedV1(root,invocation,phaseControl,options);\n}\n\n" + LOADFN_TYPE +
    "\n/** 🔒️ Loads the graph with the strict self-checks of the loaded implementation; the generation path runs pinned and completes with what it loaded. */\n"
    "async function loadBuildGraphUnpinnedV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,options:BuildGraphLoadOptionsV1):Promise<BuildGraphV1> {\n"))

# 7 the CLI door ----------------------------------------------------------------------------------------------------------------------------------------------------------------------
SCRIPT = lib("🏗️build/📜️script.ts")
edit(SCRIPT, "withImplementationPinV1(()=>runAdmittedBuildCliV1(", r"await withBuildCapacityV1\(args\.slice\(prefix\),environment,granted=>runAdmittedBuildCliV1\(root,\[\.\.\.args\.slice\(0,prefix\),\.\.\.granted\.args\],environment\)\);",
     lambda m: 'const{withImplementationPinV1}=await import("' + import_path(SCRIPT, REVISION) + '");\n  await withBuildCapacityV1(args.slice(prefix),environment,granted=>withImplementationPinV1(()=>runAdmittedBuildCliV1(root,[...args.slice(0,prefix),...granted.args],environment)));')
edit(SCRIPT, "withImplementationPinV1(()=>runAdmittedBuildInvocationV1(", r"await withBuildCapacityV1\(args,process\.env,granted=>runAdmittedBuildInvocationV1\(root,granted\.args,parent,phaseControl\),\{control:phaseControl\}\);",
     lambda m: 'const{withImplementationPinV1}=await import("' + import_path(SCRIPT, REVISION) + '");\n await withBuildCapacityV1(args,process.env,granted=>withImplementationPinV1(()=>runAdmittedBuildInvocationV1(root,granted.args,parent,phaseControl)),{control:phaseControl});')


# 8 tests -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------
WORLD = os.path.join(lib("🏗️build/🕸️graph/🕰️generations/🧪️tests/🏃️world"), TS)
edit(WORLD, "export const IMPLEMENTATION=", 'export const IDENTITY=createHash("sha256").update("synthetic-identity-v1").digest("hex");\n', 'export const IDENTITY=createHash("sha256").update("synthetic-identity-v1").digest("hex");\nexport const IMPLEMENTATION=createHash("sha256").update("synthetic-implementation-v1").digest("hex");\n', literal=True)
edit(WORLD, 'import {readFileSync} from "node:fs";', 'import {createHash} from "node:crypto";\n', 'import {createHash} from "node:crypto";\nimport {readFileSync} from "node:fs";\n', literal=True)
edit(WORLD, "package?:string;uses:string[];broken?:boolean}", "package?:string;uses:string[]}", "package?:string;uses:string[];broken?:boolean}", literal=True)
edit(WORLD, "...(json.broken?{broken:true}:{}),uses:json.uses??[]", "uses:json.uses??[]});", "...(json.broken?{broken:true}:{}),uses:json.uses??[]});", literal=True)
edit(WORLD, 'kind:"project"|"unavailable";node:unknown', 'kind:"project";node:unknown', 'kind:"project"|"unavailable";node:unknown', literal=True)
edit(WORLD, "if(owner.broken)return{", "rows.push(fileRow(owner.config,config));\n", "rows.push(fileRow(owner.config,config));\n if(owner.broken)return{name:owner.name,root:owner.root,kind:\"unavailable\",node:{root:owner.root,project:owner.name,source:owner.config,sha256:sha(config),reason:\"broken\"},edges:[],targets:[],config:owner.config,rows};\n", literal=True)
edit(WORLD, "store.refresh({identity,implementation:IMPLEMENTATION,policy", "store.refresh({identity,policy", "store.refresh({identity,implementation:IMPLEMENTATION,policy", literal=True)
edit(WORLD, "store.refreshClosure({roots,identity,implementation:IMPLEMENTATION,policy", "store.refreshClosure({roots,identity,policy", "store.refreshClosure({roots,identity,implementation:IMPLEMENTATION,policy", literal=True)
edit(WORLD, "log:{full:string[];edges:string[];cold:boolean}[]=[],ownerDelayMs=0)", "log:{full:string[];edges:string[];cold:boolean}[]=[])", "log:{full:string[];edges:string[];cold:boolean}[]=[],ownerDelayMs=0)", literal=True)
edit(WORLD, "if(ownerDelayMs)await new Promise(resolve=>setTimeout(resolve,ownerDelayMs));inferred.push", "inferred.push(await inferOwner(dir,owner,owners));}", "if(ownerDelayMs)await new Promise(resolve=>setTimeout(resolve,ownerDelayMs));inferred.push(await inferOwner(dir,owner,owners));}", literal=True)
edit(WORLD, "...(row.kind===\"unavailable\"?{unavailable:row.node}:{node:row.node})", "config:row.config,node:row.node,nodeRows:row.rows}))", "config:row.config,...(row.kind===\"unavailable\"?{unavailable:row.node}:{node:row.node}),nodeRows:row.rows}))", literal=True)
edit(WORLD, "implementationFiles?:string[];ownerDelayMs?:number}", "gate?:()=>Promise<void>;identity?:string}={})", "gate?:()=>Promise<void>;identity?:string;implementationFiles?:string[];ownerDelayMs?:number}={})", literal=True)
edit(WORLD, "createPartDiscover(dir,log,options.ownerDelayMs),holds", "createPartDiscover(dir,log),holds", "createPartDiscover(dir,log,options.ownerDelayMs),holds", literal=True)
edit(WORLD, "const implementationRows:GenerationRowV1[]=", "let building=Promise.resolve();\n", "let building=Promise.resolve();\n const implementationRows:GenerationRowV1[]=(options.implementationFiles??[]).map(path=>{const bytes=readFileSync(join(dir,path));return{kind:\"file\",path,fact:{sha256:sha(bytes),size:bytes.length}};});\n", literal=True)
edit(WORLD, "async implementationRows(){return implementationRows;},", "async identity(){return options.identity??IDENTITY;},\n", "async identity(){return options.identity??IDENTITY;},\n  async implementationRows(){return implementationRows;},\n", literal=True)

ENGINE_TEST = os.path.join(GEN, lib("🏗️build/🕸️graph/🕰️generations/🧪️tests") and "", "") if False else os.path.join(lib("🏗️build/🕸️graph/🕰️generations/🧪️tests"), TS)
edit(ENGINE_TEST, "import {IDENTITY,IMPLEMENTATION,applyOps", r"import \{IDENTITY,applyOps", "import {IDENTITY,IMPLEMENTATION,applyOps")
row = text_of(ENGINE_TEST)
if "identity:IDENTITY,implementation:IMPLEMENTATION" not in row[0]:
    row[0] = row[0].replace("{identity:IDENTITY,", "{identity:IDENTITY,implementation:IMPLEMENTATION,")
    row[1] = "pending"
append(ENGINE_TEST, "the manifest stamps the implementation of its builder", r"""
test("the manifest stamps the implementation of its builder, a generation of another implementation is served as it is, and a manifest of another format is no generation",async()=>{
 const memory=createMemoryFilePort(),state={log:[] as string[]};
 const membership=async()=>({digest:sha("m"),frontier:createFrontierV1({directories:[],members:[],capturedAt:1}),changes:[] as never[]});
 const compute=async(spec:{cold:boolean;owners:string[]})=>{state.log.push(spec.cold?"cold":"delta:"+spec.owners.join("+"));return{owners:[{name:"a",root:"a",kind:"project" as const,node:{name:"a",root:"a"},edges:[],targets:["build"],rows:[{kind:"file",path:"a",fact:{}}]}],env:{digest:sha("env"),rows:[]},global:{},replaceTags:[]};};
 const store=createGenerationStoreV1({root:"/memory/gen",fs:memory.port,resources:{facts:"f"},holds:async()=>true}),options=(implementation:string,policy?:"fresh"|"current")=>({identity:IDENTITY,implementation,...(policy?{policy}:{}),membership,compute:compute as never});
 const first=await store.refresh(options(sha("impl-a")));
 expect(first.current.manifest!.implementation).toBe(sha("impl-a"));
 const verified=await store.refresh(options(sha("impl-b")));
 expect(verified.built).toBe(false);expect(verified.current.manifest!.implementation).toBe(sha("impl-a"));expect(state.log).toEqual(["cold"]);
 expect((await store.refresh(options(sha("impl-b"),"current"))).served).toBe("current");
 const manifestPath=[...memory.files.keys()].find(path=>path.startsWith("/memory/gen/m/"))!,old=JSON.parse(Buffer.from(memory.files.get(manifestPath)!.bytes).toString("utf8"));delete old.implementation;
 const bytes=Buffer.from(canonicalJsonV1(old)),digest=createHash("sha256").update(bytes).digest("hex");
 memory.files.set("/memory/gen/m/"+digest+".json",{bytes,modifiedMs:Date.now()});memory.files.set("/memory/gen/current.json",{bytes:Buffer.from(canonicalJsonV1({version:1,generation:0,manifest:digest,parent:null})),modifiedMs:Date.now()});
 const reborn=await store.refresh(options(sha("impl-b")));
 expect(reborn.built).toBe(true);expect(state.log).toEqual(["cold","cold"]);expect(reborn.current.manifest!.implementation).toBe(sha("impl-b"));
 await expect(createGenerationStoreV1({root:"/memory/other",fs:createMemoryFilePort().port,resources:{facts:"f"},holds:async()=>true}).refresh(options("not-a-digest"))).rejects.toThrow();
});
""")

LOAD_TEST = os.path.join(lib("🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests"), TS)
edit(LOAD_TEST, "import {mkdtempSync,readFileSync,realpathSync,rmSync}", 'import {mkdtempSync,realpathSync,rmSync} from "node:fs";', 'import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";\nimport {writeFile} from "node:fs/promises";', literal=True)
edit(LOAD_TEST, "implementationDigestV1,loadGenerationGraphV1,renderGenerationMessageV1,scopeImplementationRowsV1,", "loadGenerationGraphV1,renderGenerationMessageV1,", "implementationDigestV1,loadGenerationGraphV1,renderGenerationMessageV1,scopeImplementationRowsV1,", literal=True)
edit(LOAD_TEST, "import {GENERATION_POLICY_V1,canonicalJsonV1}", 'import {canonicalJsonV1} from "../../' + MJS + '";', 'import {GENERATION_POLICY_V1,canonicalJsonV1} from "../../' + MJS + '";', literal=True)
edit(LOAD_TEST, "const sleep=", "const clean=(dir:string)=>rmSync(dir,{recursive:true,force:true});\n", "const clean=(dir:string)=>rmSync(dir,{recursive:true,force:true});\nconst sleep=(ms:number)=>new Promise<void>(resolve=>setTimeout(resolve,ms));\n", literal=True)
append(LOAD_TEST, "implementation facts are scoped and stamped like the independent model", r"""
for(const[index,row]of fixture.implementation.entries())test("implementation facts are scoped and stamped like the independent model "+index,()=>{
 const scoped=scopeImplementationRowsV1(row.rows as never),expected=row.expected as {all:unknown[];projects:unknown[];digest:string};
 expect({all:scoped.all,projects:scoped.projects}).toEqual({all:expected.all,projects:expected.projects} as never);
 expect(implementationDigestV1(row.rows as never)).toBe(expected.digest);
});

test("shaping adds the implementation facts of the consumers: valid projects the shared and their own, unavailable ones and the environment only the shared",()=>{
 const{workspaceRoot,part,stubs,implementation,expected}=fixture.shapeImplementation;
 expect(shapeComputeResultV1({workspaceRoot,part:part as never,stubs:stubs as never,implementation:implementation as never})).toEqual(expected as never);
});

test("the stale generation text is declared in both languages",()=>{
 expect(renderGenerationMessageV1("stale",{generation:2},{})).toBe("graph: serving generation 2 built by another implementation (graph policy current)");
 expect(renderGenerationMessageV1("stale",{generation:2},{LANG:"de_DE.UTF-8"})).toBe("Graph: Generation 2 stammt von einer anderen Implementierung und wird geliefert (Graph-Richtlinie current)");
});

const implementationPathsV1=()=>{const scopes=GENERATION_POLICY_V1.implementation;return{inert:"impl/"+scopes.none[0]+"ledger.mjs",nodes:"impl/"+scopes.projects[0]+"nodes.mjs",core:"impl/core/graph.mjs"};};
const stampOf=(dir:string):string=>{const pointer=JSON.parse(readFileSync(join(dir,".gen","current.json"),"utf8"));return JSON.parse(readFileSync(join(dir,".gen","m",pointer.manifest+".json"),"utf8")).implementation;};
const brokenRing=(count:number,broken:number)=>{const files=ringFiles(count);for(let i=count-broken;i<count;i++)files[`pkg/p${i}/project.json`]=JSON.stringify({name:`p${i}`,broken:true});return files;};
const names=(count:number)=>Array.from({length:count},(_,i)=>"p"+i).sort();

test("a build names the implementation modules it loaded: an inert module refreshes nothing, a node module the valid projects, a shared module every project, and a reader of another implementation is served or verified without a rebuild",async()=>{
 const dir=scratch(),paths=implementationPathsV1(),files={...brokenRing(12,2),[paths.inert]:"1",[paths.nodes]:"1",[paths.core]:"1"},valid=names(10),every=names(12);
 try{
  await seed(dir,files);const options={implementationFiles:Object.values(paths)};
  const loader=createMockHost(dir,options);await loadGenerationGraphV1(loader.host);
  const stampA=implementationDigestV1(await loader.host.implementationRows());expect(stampOf(dir)).toBe(stampA);expect(loader.log).toEqual([{full:every,edges:[],cold:true}]);
  await applyOps(dir,[{op:"write",path:paths.inert,content:"2"}]);
  const inert=createMockHost(dir,options);await loadGenerationGraphV1(inert.host);
  expect(inert.events).toEqual(["shared"]);expect(inert.log).toEqual([]);expect(stampOf(dir)).toBe(stampA);
  await applyOps(dir,[{op:"write",path:paths.nodes,content:"2"}]);
  const nodes=createMockHost(dir,options),graph=await loadGenerationGraphV1(nodes.host) as any;
  expect(nodes.log).toEqual([{full:valid,edges:[],cold:false}]);expect(Object.keys(graph.projects).sort()).toEqual(valid);expect(graph.unavailable.map((row:any)=>row.project).sort()).toEqual(["p10","p11"]);
  const stampB=implementationDigestV1(await nodes.host.implementationRows());expect(stampB).not.toBe(stampA);expect(stampOf(dir)).toBe(stampB);
  await applyOps(dir,[{op:"write",path:paths.core,content:"2"}]);
  const core=createMockHost(dir,options);await loadGenerationGraphV1(core.host);
  expect(core.log).toEqual([{full:every,edges:[],cold:false}]);const stampC=implementationDigestV1(await core.host.implementationRows());expect(stampOf(dir)).toBe(stampC);
  loader.events.length=0;loader.said.length=0;const stale=await loadGenerationGraphV1(loader.host,{policy:"current"}) as any;
  expect(loader.events).toEqual(["shared"]);expect(loader.said.map(row=>row.text)).toEqual(["graph: serving generation 2 built by another implementation (graph policy current)"]);expect(Object.keys(stale.projects).length).toBe(10);
  loader.events.length=0;await loadGenerationGraphV1(loader.host);expect(loader.events).toEqual(["shared"]);expect(loader.log.length).toBe(1);expect(stampOf(dir)).toBe(stampC);
 }finally{clean(dir);}
},120000);

test("an implementation module edited every few hundred milliseconds never restarts a 300 project build: it publishes the generation of the implementation it loaded and the next fresh load refreshes exactly the consumers of the edited module",async()=>{
 const count=300,broken=30,paths=implementationPathsV1(),files={...brokenRing(count,broken),[paths.inert]:"0",[paths.nodes]:"0",[paths.core]:"0"},valid=names(count-broken);
 const quiet=scratch(),busy=scratch(),options={implementationFiles:Object.values(paths),ownerDelayMs:8};
 try{
  await seed(quiet,files);const timed=createMockHost(quiet,options),started=Date.now();await loadGenerationGraphV1(timed.host);const quietMs=Date.now()-started;
  await seed(busy,files);const loaded=createMockHost(busy,options),loadedStamp=implementationDigestV1(await loaded.host.implementationRows());
  let editing=true,edits=0;const editor=(async()=>{while(editing){await sleep(250);edits++;await writeFile(join(busy,paths.nodes),String(edits));}})();
  const begin=Date.now(),graph=await loadGenerationGraphV1(loaded.host) as any,busyMs=Date.now()-begin;editing=false;await editor;
  expect(edits).toBeGreaterThanOrEqual(3);expect(loaded.log.length).toBe(1);expect(loaded.said.at(-1)!.text).toContain("published");expect(busyMs).toBeLessThanOrEqual(1.2*quietMs+500);
  expect(stampOf(busy)).toBe(loadedStamp);expect(Object.keys(graph.projects).length).toBe(count-broken);
  const next=createMockHost(busy,options);await loadGenerationGraphV1(next.host);
  expect(next.log).toEqual([{full:valid,edges:[],cold:false}]);expect(stampOf(busy)).toBe(implementationDigestV1(await next.host.implementationRows()));
  const again=createMockHost(busy,options);await loadGenerationGraphV1(again.host);expect(again.log).toEqual([]);expect(again.events).toEqual(["shared"]);
 }finally{clean(quiet);clean(busy);}
},240000);
""")
edit(LOAD_TEST, "shapeImplementation", "import fixture from", "import fixture from", literal=True) if False else None

PROTOCOL = os.path.join(lib("🏗️build/🕸️graph/🧪️tests/🕰️generation-protocol"), TS)
PROTOCOL_J = os.path.join(lib("🏗️build/🕸️graph/🧪️tests/🕰️generation-protocol"), "🔣️.json")


def protocol_change(data):
    if any(row["name"] == "the generation load runs pinned to the implementation it loaded" for row in data["present"]):
        data["absent"].append({"name": "the host identity never contains the module authority", "function": "generationHostV1", "text": "loadedGraphModuleAuthorityV1"})
        return
    data["present"] += [
        {"name": "the generation load runs pinned to the implementation it loaded", "function": "loadBuildGraphV1", "text": "withImplementationPinV1(()=>loadBuildGraphUnpinnedV1("},
        {"name": "the host names the loaded graph implementation as facts", "function": "generationHostV1", "text": "...loadedGraphImplementationRowsV1,"},
        {"name": "the host names the loaded inference implementation as facts", "function": "generationHostV1", "text": "producer.loadedImplementationRowsV1"},
    ]
    data["absent"] += [
        {"name": "the host identity never refuses an edited implementation", "function": "generationHostV1", "text": "requireCurrentGraphModuleV1()"},
        {"name": "the host identity never hashes the implementation on disk", "function": "generationHostV1", "text": "currentImplementationRevisionV1"},
    ]
    data["absent"].append({"name": "the host identity never contains the module authority", "function": "generationHostV1", "text": "loadedGraphModuleAuthorityV1"})


json_edit(PROTOCOL_J, "the host identity never contains the module authority", protocol_change)
append(PROTOCOL, "every self-check of a loaded implementation", r"""
test("every self-check of a loaded implementation lets a pinned invocation finish",()=>{
 const library=fileURLToPath(new URL("../../../../",import.meta.url)),offenders:string[]=[],bare=(name:string)=>name.replace(/️/gu,"");
 const walk=(directory:string)=>{
  for(const entry of readdirSync(directory,{withFileTypes:true})){
   if(entry.name.startsWith(".")||entry.name==="node_modules"||entry.name==="target"||bare(entry.name).endsWith("tests"))continue;
   const path=join(directory,entry.name);
   if(entry.isDirectory())walk(path);
   else if(/\.(?:ts|mjs)$/u.test(entry.name)&&!entry.name.endsWith(".d.mts"))for(const[index,line]of readFileSync(path,"utf8").split("\n").entries())if((line.includes("Stale loaded")||line.includes("Stale inference inventory reuse"))&&!line.includes("implementationPinnedV1()"))offenders.push(path.slice(library.length)+":"+(index+1));
  }
 };
 for(const entry of readdirSync(library,{withFileTypes:true}))if(entry.isDirectory()&&[bare("🏗️build"),bare("⚡️caching")].includes(bare(entry.name)))walk(join(library,entry.name));
 expect(offenders).toEqual([]);
 expect(source.includes("const requireCurrentGraphModuleV1=():void=>{if(implementationPinnedV1())return;")).toBe(true);
});
""")
edit(PROTOCOL, 'import {readFileSync,readdirSync} from "node:fs";', 'import {readFileSync} from "node:fs";', 'import {readFileSync,readdirSync} from "node:fs";', literal=True)

REVISION_TEST = os.path.join(lib("🏗️build/📥️inputs/🔏️revision/🧪️tests"), TS)
append(REVISION_TEST, "a pinned invocation completes with the Source it loaded", r"""
test("a pinned invocation completes with the Source it loaded: an edited reader refuses outside the pin and lets the invocation finish inside it; pins nest and a failing body releases its pin",async()=>{
 const fs=await import("node:fs"),os=await import("node:os"),home=fileURLToPath(new URL("..",import.meta.url)),listing=fs.readdirSync(home),moduleName=listing.find(name=>name.endsWith(".mjs"))!,schemaName=listing.find(name=>name.endsWith("schema"))!;
 const scratch=fs.mkdtempSync(join(fs.realpathSync(os.tmpdir()),"revision-pin-"));
 try{
  fs.mkdirSync(join(scratch,schemaName));const schemaFile=fs.readdirSync(join(home,schemaName))[0]!;fs.copyFileSync(join(home,schemaName,schemaFile),join(scratch,schemaName,schemaFile));fs.copyFileSync(join(home,moduleName),join(scratch,moduleName));
  const reader=await import(join(scratch,moduleName)),data=(name:string)=>{const path=join(scratch,name);fs.writeFileSync(path,name);return path;};
  expect(reader.implementationPinnedV1()).toBe(false);expect(reader.captureBoundedSourceRevisionV1(data("one.txt"),16).byteLength).toBe(7);
  const copy=join(scratch,moduleName),text=fs.readFileSync(copy,"utf8");expect(text.includes("maximumSettledRevisions=4096")).toBe(true);fs.writeFileSync(copy,text.replace("maximumSettledRevisions=4096","maximumSettledRevisions=4097"));
  expect(()=>reader.captureBoundedSourceRevisionV1(data("two.txt"),16)).toThrow("Stale loaded Source revision reader; restart required");
  await reader.withImplementationPinV1(async()=>{
   expect(reader.implementationPinnedV1()).toBe(true);
   await reader.withImplementationPinV1(async()=>{expect(reader.captureBoundedSourceRevisionV1(data("three.txt"),16).byteLength).toBe(9);});
   expect(reader.implementationPinnedV1()).toBe(true);expect(reader.captureBoundedSourceRevisionV1(data("four.txt"),16).byteLength).toBe(8);
  });
  expect(reader.implementationPinnedV1()).toBe(false);expect(()=>reader.captureBoundedSourceRevisionV1(data("five.txt"),16)).toThrow("Stale loaded Source revision reader");
  await expect(reader.withImplementationPinV1(async()=>{throw Error("body failed");})).rejects.toThrow("body failed");expect(reader.implementationPinnedV1()).toBe(false);
 }finally{fs.rmSync(scratch,{recursive:true,force:true});}
});
""")

# fixtures generated by the independent model --------------------------------------------------------------------------------------------------------------------------------------
STAGED = os.path.join(HERE, "staged", "new", "load")
for source, target in [("fixture.json", os.path.join(lib("🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests"), "🔣️.json")), ("fixture-schema.json", os.path.join(lib("🏗️build/🕸️graph/🕰️generations/🔌️load/🧪️tests/🧬️schema"), "🔣️.json"))]:
    wanted = open(os.path.join(STAGED, source), encoding="utf8").read()
    row = text_of(target)
    if row[0] == wanted:
        print("applied  ", rel(target)[-76:], "| fixture")
    else:
        row[0], row[1] = wanted, "pending"
        print("pending  ", rel(target)[-76:], "| fixture")


pending = [path for path, (text, state) in changes.items() if state == "pending"]
print("files to write:", len(pending))
if WRITE:
    for path in pending:
        with open(path, "w", encoding="utf8", newline="") as handle:
            handle.write(changes[path][0])
    print("WRITTEN")
