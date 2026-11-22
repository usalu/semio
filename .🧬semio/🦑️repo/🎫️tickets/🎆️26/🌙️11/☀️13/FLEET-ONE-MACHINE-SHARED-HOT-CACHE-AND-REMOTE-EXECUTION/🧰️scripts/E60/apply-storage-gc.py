import json,sys
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def edit(path,pairs):
    s=open(L+path).read()
    for a,b in pairs:
        if b in s and a not in s: continue
        assert s.count(a)==1,(path,a[:60],s.count(a))
        s=s.replace(a,b)
    open(L+path,'w').write(s)

# texts (ledger)
edit("⚡️caching/💾️storage/🧾️ledger/📖️texts/🔣️.json",[(
'    "gc.graph.quota.within":',
'    "gc.graph.publications.plan": { "en": "Would remove {count} producer publications of gone processes ({bytes})", "de": "Würde {count} Erzeuger-Veröffentlichungen beendeter Prozesse entfernen ({bytes})" },\n    "gc.graph.publications.done": { "en": "Removed {count} producer publications of gone processes ({bytes})", "de": "{count} Erzeuger-Veröffentlichungen beendeter Prozesse entfernt ({bytes})" },\n    "gc.graph.quota.within":')])

# ledger classification
edit("⚡️caching/💾️storage/🧾️ledger/🟦️.ts",[
('const parts=segments(path);if(parts.length===1&&ledger.roots[root].ignored.includes(parts[0]!))return{kind:"ignored"};',
 'const parts=segments(path),ignored=ledger.roots[root].ignored;if(ignored.includes(path))return{kind:"ignored"};'),
('return homes.some(home=>inside(parts,home))?{kind:"container"}:{kind:"undeclared"};',
 'return homes.some(home=>inside(parts,home))?{kind:"container"}:ignored.includes(parts[parts.length-1]!)?{kind:"ignored"}:{kind:"undeclared"};'),
('/** 🧮️ Classifies one root-relative path against the ledger. */',
 '/** 🧮️ Classifies one root-relative path against the ledger: an ignored entry is a listed root-relative path, or a listed file name anywhere inside a container of declared homes. */'),
])

# ledger policy (text edit, anchored)
p=L+"⚡️caching/💾️storage/🧾️ledger/🔣️policy.json"
s=open(p).read()
a='''      "directory": ".cache",
      "maximumBytes": 805306368,
      "undeclaredUnusedAgeMs": 604800000,
      "undeclaredQuietMs": 3600000,
      "ignored": [
        ".DS_Store"
      ]'''
b='''      "directory": ".cache",
      "maximumBytes": 805306368,
      "undeclaredUnusedAgeMs": 604800000,
      "undeclaredQuietMs": 3600000,
      "ignored": [
        ".DS_Store",
        "b/image-relocation.json"
      ]'''
if b not in s:
    assert s.count(a)==1; s=s.replace(a,b)
a='''  "retirements": [
'''
b='''  "retirements": [
    {
      "root": "native",
      "directory": "test-admission",
      "quietMs": 900000,
      "reason": {
        "en": "The admitted-worker test scratch lives in the test sandbox area (test-sandbox/admission).",
        "de": "Die Arbeitsfläche des Tests der zugelassenen Worker liegt im Bereich test-sandbox (test-sandbox/admission)."
      }
    },
'''
if '"directory": "test-admission"' not in s:
    assert s.count(a)==1; s=s.replace(a,b)
open(p,'w').write(s)

# test-admission scratch moves into the declared sandbox area
edit("⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧪️tests/🔏️admission/🟦️.ts",[
('join(workspace,".cache","test-admission")','join(workspace,".cache","test-sandbox","admission")')])

# graph reclaim stage
edit("🏗️build/🧹️cache-gc/🚰️reclaim/🟦️.ts",[
('import {cacheTextV1,formatCacheBytesV1,type CacheLanguageV1,type CacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";',
 'import {join} from "node:path";\nimport {cacheTextV1,formatCacheBytesV1,type CacheLanguageV1,type CacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";\nimport {sweepDeadProducerPublicationsV1} from "../../👁️observation/🪦️producer-publications/🟨️.mjs";'),
(' closureStores?(image:object,dryRun:boolean)',
 ' publications?(image:object,dryRun:boolean):Promise<Readonly<{candidates:number;removed:number;bytes:number;cancelled:boolean}>>;\n closureStores?(image:object,dryRun:boolean)'),
('/** 🚰️ Reclaims optional graph caches',
 '/** 🪦️ Sweeps the per-process producer publications (`p-<pid>`) of processes that are gone below the selections of the graph image: nothing reads them once their process ended, a live process keeps its own. */\nexport async function graphProducerPublicationsV1(imageRoot:string,dryRun:boolean,signal:AbortSignal):Promise<Readonly<{candidates:number;removed:number;bytes:number;cancelled:boolean}>>{\n const report=await sweepDeadProducerPublicationsV1(join(imageRoot,"inference","selections"),{dryRun,signal});\n return{candidates:report.candidates.length,removed:report.removed,bytes:report.bytes,cancelled:report.cancelled};\n}\n\n/** 🚰️ Reclaims optional graph caches'),
('  const before=dryRun?undefined:await ports.free(image),pressed=before!==undefined&&before<targetAvailableBytes;\n',
 '  const before=dryRun?undefined:await ports.free(image),pressed=before!==undefined&&before<targetAvailableBytes;\n  if(ports.publications){const report=await ports.publications(image,dryRun);if(report.candidates)lines.push(text(dryRun?"gc.graph.publications.plan":"gc.graph.publications.done",{count:report.candidates,bytes:formatCacheBytesV1(report.bytes)}));if(report.cancelled)return finish(text("error.cancelled"));}\n'),
])

# command binding
edit("🏗️build/🧹️cache-gc/⌨️command/🟦️.ts",[
('const{reclaimGraphOptionalV1}=await import("../🚰️reclaim/🟦️.ts")','const{reclaimGraphOptionalV1,graphProducerPublicationsV1}=await import("../🚰️reclaim/🟦️.ts")'),
('  lru:async(image,dryRun)=>{const scan=','  publications:async(image,dryRun)=>graphProducerPublicationsV1((image as Image).root,dryRun,phaseControl.signal),\n  lru:async(image,dryRun)=>{const scan='),
])
print("ok")
