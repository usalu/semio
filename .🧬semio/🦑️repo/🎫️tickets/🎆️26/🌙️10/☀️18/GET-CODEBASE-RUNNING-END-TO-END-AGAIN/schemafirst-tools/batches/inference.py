import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

N = BUILD + "🕸️graph/📥️inference/"
b = Batch("inference", "the graph inferrers read their running tick, cache bounds, display lengths, progress stride and throttle from one inference schema, and the racy window and path ceiling from the observation and revision policies (re-keys: inferrer sources are stamped by declared protocol versions, but the stamps law reads these files)")
b.new_schema(N + "🧬️schema/🔣️.json", "Graph inference limits / Grenzen der Graph-Inferenz", [Def("Limits", "The ticks, cache bounds, display lengths, strides and throttles of the graph inferrers.", "Die Takte, Cache-Grenzen, Anzeigelaengen, Schrittweiten und Drosselungen der Graph-Inferrer.", [
    ("runningTickMilliseconds", 5000, "How often the projects still being inferred are reported while a computation runs.", "Wie oft die noch zu inferierenden Projekte gemeldet werden, waehrend eine Berechnung laeuft."),
    ("sourceLineCharacters", 512, "The most characters of the source line a refused TOML parse reports.", "Die meisten Zeichen der Quellzeile, die ein abgelehnter TOML-Parse meldet."),
    ("entryOverheadBytes", 192, "The bytes one cached reference list is charged on top of its JSON image.", "Die Bytes, die eine gecachte Referenzliste zusaetzlich zu ihrem JSON-Abbild kostet."),
    ("rustSourceCacheBytes", 16777216, "The default byte budget of the Rust source analysis cache (16 MiB).", "Das Standard-Bytebudget des Caches der Rust-Quellanalyse (16 MiB)."),
    ("advanceThrottleMilliseconds", 200, "The least time between two progress advances of a script input walk.", "Die kleinste Zeit zwischen zwei Fortschrittsschritten eines Skripteingabe-Durchlaufs."),
])])
b.put(N + "🧬️schema/🟨️.mjs", '''import schema from"./🔣️.json"with{type:"json"};

/** 🧱️ The ticks, cache bounds, display lengths, strides and throttles of the graph inferrers; the schema document beside this module is their single source. */
export const INFERENCE_LIMITS_V1=Object.freeze({...schema.definitions.Limits.const});
''')
b.put(N + "🧬️schema/🟨️.d.mts", '''/** 🧱️ The ticks, cache bounds, display lengths, strides and throttles of the graph inferrers: the schema document beside this module is their single source. */
export const INFERENCE_LIMITS_V1:Readonly<{runningTickMilliseconds:number;sourceLineCharacters:number;entryOverheadBytes:number;rustSourceCacheBytes:number;advanceThrottleMilliseconds:number}>;
''')
imp = 'import { INFERENCE_LIMITS_V1 } from "../🧬️schema/🟨️.mjs";\n'
b.subs(N + "📦️project-nodes/🟨️.mjs", [
    ("import { detachedFailureV1, memoizedObservedAsyncComputationV1,", "import { OBSERVATION_LIMITS_V1, detachedFailureV1, memoizedObservedAsyncComputationV1,"),
    ('import { statSync as rawStatSync } from "node:fs";\n', 'import { statSync as rawStatSync } from "node:fs";\n' + imp),
    ("const UNSETTLED_SOURCE_MS = 2000;", "const UNSETTLED_SOURCE_MS = OBSERVATION_LIMITS_V1.racyMilliseconds;"),
    ("const RUNNING_TICK_MS = 5000;", "const RUNNING_TICK_MS = INFERENCE_LIMITS_V1.runningTickMilliseconds;"),
])
b.subs(N + "🦀️native-manifests/🟨️.mjs", [
    ('observedSourceAnalysisV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";\n', 'observedSourceAnalysisV1, observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";\n' + imp),
    ("JSON.stringify(references))+192;", "JSON.stringify(references))+INFERENCE_LIMITS_V1.entryOverheadBytes;"),
    ('??"").slice(0,512)}', '??"").slice(0,INFERENCE_LIMITS_V1.sourceLineCharacters)}'),
])
b.subs(N + "🧬️source-analyses/🟨️.mjs", [
    ('observedRequireResolve } from "../../../../🏗️build/👁️observation/🟨️.mjs";\n', 'observedRequireResolve } from "../../../../🏗️build/👁️observation/🟨️.mjs";\n' + imp),
    ("createRustSourceCache(limit = 16 * 1024 * 1024)", "createRustSourceCache(limit = INFERENCE_LIMITS_V1.rustSourceCacheBytes)"),
    ("JSON.stringify(references)) + 192", "JSON.stringify(references)) + INFERENCE_LIMITS_V1.entryOverheadBytes"),
])
b.subs(N + "🧭️script-inputs/🟨️.mjs", [
    ('import { validateJsonSchemaSubset } from', 'import { REVISION_POLICY_V1 } from "../../../📥️inputs/🔏️revision/🟨️.mjs";\n' + imp + 'import { validateJsonSchemaSubset } from'),
    ("path.length>256||local===", "path.length>REVISION_POLICY_V1.maximumPathUnits||local==="),
    ('if(path.length>256)throw Error("Bounded physical runtime Source entry required")', 'if(path.length>REVISION_POLICY_V1.maximumPathUnits)throw Error("Bounded physical runtime Source entry required")'),
    ("<200){context.invocation", "<INFERENCE_LIMITS_V1.advanceThrottleMilliseconds){context.invocation"),
])
SD = BUILD + "🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts"
b.subs(SD, [
    ('import {buildInputPathMatchesV1} from "../../../📥️inputs/🟨️.mjs";\n', 'import {buildInputPathMatchesV1} from "../../../📥️inputs/🟨️.mjs";\nimport {REVISION_POLICY_V1} from "../../../📥️inputs/🔏️revision/🟨️.mjs";\nimport {INFERENCE_LIMITS_V1} from "../../../🕸️graph/📥️inference/🧬️schema/🟨️.mjs";\n'),
    ("const ports={createRequire,createHash,resolve,relative,join,dirname,", "const ports={REVISION_POLICY_V1,INFERENCE_LIMITS_V1,createRequire,createHash,resolve,relative,join,dirname,"),
    ("const ports={providerReferencesV1:()=>[],resolve,relative,join,isAbsolute", "const ports={REVISION_POLICY_V1,INFERENCE_LIMITS_V1,providerReferencesV1:()=>[],resolve,relative,join,isAbsolute"),
    ("const ports={routerCommandWordV1:()=>undefined,buildSchedulingPrefixesV1", "const ports={REVISION_POLICY_V1,INFERENCE_LIMITS_V1,routerCommandWordV1:()=>undefined,buildSchedulingPrefixesV1"),
])
rel = lambda p: "🏗️build/🕸️graph/📥️inference/" + p
b.row("inference", "library", rel("🧬️schema/🔣️.json"), ["Limits"], [rel("📦️project-nodes/🟨️.mjs"), rel("🦀️native-manifests/🟨️.mjs"), rel("🧬️source-analyses/🟨️.mjs"), rel("🧭️script-inputs/🟨️.mjs")], exports=[(rel("🧬️schema/🟨️.mjs"), "INFERENCE_LIMITS_V1", "Limits")])
b.write(tests=["./" + SD, "./" + N + "📦️project-nodes/🧪️tests", "./" + N + "🔖️stamps/🧪️tests/🟦️.ts", "./" + LAW + "🟦️.ts"])
