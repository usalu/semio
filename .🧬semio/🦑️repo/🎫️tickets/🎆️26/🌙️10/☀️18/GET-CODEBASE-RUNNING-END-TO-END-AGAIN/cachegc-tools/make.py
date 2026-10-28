"""[cachegc] Builds the staged batch `quota` from the CURRENT live files by exact anchors (each anchor must occur exactly once, else exit 2, never a guess) plus the new law files. usage: python3 -I make.py [--root DIR] [--batch NAME]"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); SRC = os.path.join(HERE, "src")
def root_of(here):
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
BATCH = sys.argv[sys.argv.index("--batch") + 1] if "--batch" in sys.argv else "quota"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
CLO = LIB + "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/"
SESSION, SCAN = CLO + "📡️session/🟦️.ts", CLO + "🗂️members/🔎️scan/🟦️.ts"
STORAGE = LIB + "🏗️build/📥️inputs/💾️storage/🟨️.mjs"
RECLAIM, COMMAND = LIB + "🏗️build/🧹️cache-gc/🚰️reclaim/🟦️.ts", LIB + "🏗️build/🧹️cache-gc/⌨️command/🟦️.ts"
TEXTS = LIB + "⚡️caching/💾️storage/🧾️ledger/📖️texts/🔣️.json"
LRU_TEST, LRU_ORACLE = CLO + "🗂️members/🧪️tests/♻️lru/🟦️.ts", CLO + "🗂️members/🧪️tests/♻️lru/🐍️.py"
QUOTA_TEST = LIB + "🏗️build/🧹️cache-gc/🧪️tests/♻️quota/🟦️.ts"
OPTIONAL_SUITE = LIB + "🏗️build/🧪️tests/🕸️graph/🧊️optional-cache/🟦️.ts"
ADMISSION_TEST = LIB + "🏗️build/📥️inputs/💾️storage/🧪️tests/🚪️admission/🟦️.ts"
src = lambda name: open(os.path.join(SRC, name), encoding="utf8").read()
def rel(frm, to): return os.path.relpath(os.path.join(ROOT, to), os.path.dirname(os.path.join(ROOT, frm))).replace(os.sep, "/")
def edit(text, path, pairs):
    for old, new in pairs:
        if text.count(old) != 1: sys.exit("[DEBUG] anchor x%d in %s: %s" % (text.count(old), path[-60:], old[:90]))
        text = text.replace(old, new)
    return text
live = lambda path: open(os.path.join(ROOT, path), encoding="utf8").read()
R = "../" * 12 + "🔨️modules/⚙️runtime/"
session = edit(live(SESSION), SESSION, [
 ('import{captureBuildSourceClosureV1,requireAdmittedBuildSourceClosureV1}from"', 'import{captureBuildSourceClosureV1,requireAdmittedBuildSourceClosureV1,type BuildSourceScopeV1}from"'),
 ('import{implementationPinnedV1}from"../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";', 'import{implementationPinnedV1,physicalSourceOriginV1}from"../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";\nimport{fileURLToPath}from"node:url";'),
 ('/** 🧊️ Captures one immutable worker program per actual issued proof lifetime. */',
  'const closureReadRuntimeRootsV1=["%s🟢️node/🟦️.ts","%s🗣️messages/🟦️.ts","%s🟢️node/🧲️native/🏗️addon/🟦️.ts","%s🟢️node/🧲️native/🧮️system/🟦️.ts"].map(path=>fileURLToPath(new URL(path,physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL))));\n'
  '/** 🪶️ The capture scope of the closure read worker: its static imports, the JSON and schemas they name, and the Node runtime adapter, message catalog and native provider its runtime loads lazily; never the dynamic modules of the backend registry the parent kernel resolved (that closure is the whole kernel, beyond the 16 MiB bootstrap channel), so a collector or retirement child stays a sub-megabyte program and anything else it would load is refused at that import. */\n'
  'export function closureReadWorkerScopeV1(root:string):Extract<BuildSourceScopeV1,{kind:"static"}>{return Object.freeze({kind:"static",roots:Object.freeze(closureReadRuntimeRootsV1.map(path=>relative(root,path).split("\\\\").join("/"))),leaves:Object.freeze([])});}\n'
  '/** 🧊️ Captures one immutable worker program per actual issued proof lifetime. */' % (R, R, R, R)),
 ('snapshot=await captureBuildSourceClosureV1(root,relative(root,entry).split("\\\\").join("/"),original)', 'snapshot=await captureBuildSourceClosureV1(root,relative(root,entry).split("\\\\").join("/"),original,closureReadWorkerScopeV1(root))'),
])
scan = edit(live(SCAN), SCAN, [
 ('memberStampV1,parseMemberIndexV1,proveGroupV1,readMemberRowV1,type MemberRowV1}from"../🟨️.mjs";', 'memberGroupProtectedV1,memberStampV1,parseMemberIndexV1,proveGroupV1,readMemberRowV1,type MemberRowV1}from"../🟨️.mjs";\nimport graphStorageSchema from"%s"with{type:"json"};' % rel(SCAN, LIB + "🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json")),
 ('\n/** 🧒️ Binds the closure read child:', src("scan-lru.ts") + '\n/** 🧒️ Binds the closure read child:'),
])
storage = edit(live(STORAGE), STORAGE, [
 ('import { acquireQueuedResourceLease, checkResourceLeaseControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";', 'import { acquireQueuedResourceLease, acquireResourceLease, checkResourceLeaseControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";'),
 ('const status=statSync(original.root),ledger=await openOriginalGraphLedgerV1(workspace,original,actualControl);let failed=false;', 'const status=statSync(original.root);let ledger=await openOriginalGraphLedgerV1(workspace,original,actualControl),failed=false;'),
 ('checkActual();const observed=ledger.observe(),space=nativeStatfsSyncV1(original.root);', 'checkActual();let observed=ledger.observe();if(optional&&!current&&observed.usedBytes>quotaSchema.policy.lowWaterBytes&&await evictOptionalGraphLruV1(workspace,original.root,resource,namespace,actualControl)){ledger.close();ledger=await openOriginalGraphLedgerV1(workspace,original,actualControl);checkActual();observed=ledger.observe();}const space=nativeStatfsSyncV1(original.root);'),
 ('\n/** 🔒️ Retains one physical inference namespace', src("storage-evict.mjs") + '\n/** 🔒️ Retains one physical inference namespace'),
])
reclaim = edit(live(RECLAIM), RECLAIM, [
 (' optional(image:object,request:Readonly<{minimumAvailableBytes:number}>):Promise<unknown>;\n}>;', ' optional(image:object,request:Readonly<{minimumAvailableBytes:number}>):Promise<unknown>;\n' + src("reclaim-lru.txt") + '}>;'),
 ('  const lines:string[]=[],finish=(line:string)=>[...lines,line].join("\\n");\n', '  const lines:string[]=[],finish=(line:string)=>[...lines,line].join("\\n");\n' + src("reclaim-body.txt")),
 ('then it indexes the groups no member index vouches for yet', 'retires the least recently written optional groups while the optional cache holds more than its low-water mark (its own limit, whatever the free space), then it indexes the groups no member index vouches for yet'),
])
command = edit(live(COMMAND), COMMAND, [
 ('  free:async image=>{const space=nativeImageSpaceV1((image as Image).root,(image as Image).filesystemBytes);return space.bavail*space.bsize;},', src("command-lru.txt") + '  free:async image=>{const space=nativeImageSpaceV1((image as Image).root,(image as Image).filesystemBytes);return space.bavail*space.bsize;},'),
])
texts = edit(live(TEXTS), TEXTS, [
 ('    "gc.graph.done": {', '    "gc.graph.quota.within": { "en": "The optional graph cache holds {used} against its limit of {limit}; within the low-water mark of {target}", "de": "Der optionale Graph-Cache belegt {used} bei einer Grenze von {limit}; innerhalb der Niedrigwassermarke von {target}" },\n'
  '    "gc.graph.quota.plan": { "en": "The optional graph cache holds {used} against its limit of {limit}, above the low-water mark of {target}; would retire {groups} least recently used groups ({bytes}), leaving {remaining}; {busy} groups busy, {protected} in use", "de": "Der optionale Graph-Cache belegt {used} bei einer Grenze von {limit}, über der Niedrigwassermarke von {target}; würde {groups} am längsten unbenutzte Gruppen ({bytes}) entfernen, {remaining} blieben; {busy} Gruppen belegt, {protected} in Gebrauch" },\n'
  '    "gc.graph.quota.done": { "en": "Retired {groups} least recently used optional graph groups ({files} files, {bytes}); {remaining} remain against the low-water mark of {target} (limit {limit}); {busy} groups busy, {protected} in use", "de": "{groups} am längsten unbenutzte optionale Graph-Gruppen entfernt ({files} Dateien, {bytes}); {remaining} bleiben bei einer Niedrigwassermarke von {target} (Grenze {limit}); {busy} Gruppen belegt, {protected} in Gebrauch" },\n'
  '    "gc.graph.done": {'),
])
suite = edit(live(OPTIONAL_SUITE), OPTIONAL_SUITE, [
 ('test.skipIf(process.env.SEMIO_GRAPH_WRITER_CONTENDER!=="1")', src("insert-law.ts") + 'test.skipIf(process.env.SEMIO_GRAPH_WRITER_CONTENDER!=="1")'),
])
admission = edit(live(ADMISSION_TEST), ADMISSION_TEST, [
 ("ledgerLive=0,bodyEntered=false;", "ledgerLive=0,bodyEntered=false,evictions=0;"),
 ("    withRepositoryCacheStoreV1:async(_root:string,kind:string", "    evictOptionalGraphLruV1:async(_workspace:string,root:string)=>{expect(root).toBe(workspace);expect(storeShared).toBe(1);expect(ledgerLive).toBe(1);evictions++;events.push(\"lru\");return false;},\n    withRepositoryCacheStoreV1:async(_root:string,kind:string"),
 ("expect(censuses).toBe(row.censuses);", "expect(evictions).toBeLessThanOrEqual(row.physicalBytes>quota.policy.lowWaterBytes?1:0);if(row.interrupt===\"none\")expect(evictions).toBe(row.physicalBytes>quota.policy.lowWaterBytes?1:0);expect(censuses).toBe(row.censuses);"),
])
lru_test = src("lru-test.ts").replace('"../../../../../../../../../../../../../🔨️modules/🏃️process/🧰️tools/🟦️.ts"', '"%s"' % rel(LRU_TEST, "🧰️framework/🔨️modules/🏃️process/🧰️tools/🟦️.ts")).replace('"../../../../../../../../../🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json"', '"%s"' % rel(LRU_TEST, LIB + "🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json"))
out = {SESSION: session, SCAN: scan, STORAGE: storage, RECLAIM: reclaim, COMMAND: command, TEXTS: texts, OPTIONAL_SUITE: suite, ADMISSION_TEST: admission, LRU_TEST: lru_test, LRU_ORACLE: src("lru-oracle.py"), QUOTA_TEST: src("quota-test.ts")}
stage = os.path.join(HERE, "staged", BATCH)
if os.path.isdir(stage): shutil.rmtree(stage)
os.makedirs(os.path.join(stage, "files")); os.makedirs(os.path.join(stage, "base"))
manifest = {}
for i, (path, text) in enumerate(out.items()):
    if len(os.path.join(ROOT, path)) > 256: sys.exit("[DEBUG] path over 256: " + path)
    name = "f%03d" % i; data = text.encode("utf8"); full = os.path.join(ROOT, path); base = open(full, "rb").read() if os.path.exists(full) else None
    open(os.path.join(stage, "files", name), "wb").write(data)
    if base is not None: open(os.path.join(stage, "base", name), "wb").write(base)
    manifest[path] = {"file": name, "base": hashlib.sha256(base).hexdigest() if base is not None else None, "new": hashlib.sha256(data).hexdigest()}
json.dump(manifest, open(os.path.join(stage, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
template = open(os.path.join(HERE, "..", "xplat5-tools", "stage.template.py"), encoding="utf8").read()
doc_end = template.index('"""', 3) + 3
open(os.path.join(stage, "stage.py"), "w", encoding="utf8").write('#!/usr/bin/env python3\n"""♻️ Staged batch `%s` (lane cachegc): the optional graph cache enforces its own limit (least recently used groups retired down to the low-water mark at writer open and by `repo cache gc`), and the closure read worker is captured with a static scope so the collector no longer dies on the 16 MiB channel.\nusage: python3 -I stage.py check|apply|revert [--root DIR]   (default root: walk up from this file to the directory holding 🧰️framework)' % BATCH + template[template.index("\ncheck :"):doc_end] + template[doc_end:])
shutil.copy(os.path.join(SRC, "verify.sh"), os.path.join(stage, "verify.sh")) if os.path.exists(os.path.join(SRC, "verify.sh")) else None
print(json.dumps({"batch": stage, "files": len(manifest), "new": [p[-50:] for p, r in manifest.items() if r["base"] is None]}, ensure_ascii=False))
