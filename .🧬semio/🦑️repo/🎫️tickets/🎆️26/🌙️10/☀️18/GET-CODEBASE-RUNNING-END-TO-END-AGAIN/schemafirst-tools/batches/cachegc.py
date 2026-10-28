import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

C = BUILD + "🧹️cache-gc/"
b = Batch("cachegc", "the cache collector reads its lease wait, phase budget, collector intervals, access-time probe and dedupe progress cadence from a schema beside it (no cache key carries these files)")
b.new_schema(C + "🧬️schema/🔣️.json", "Cache collector limits / Grenzen des Cache-Sammlers", [Def("Limits", "The waits, budgets, intervals and progress cadences of the cache collector.", "Die Wartezeiten, Budgets, Intervalle und Fortschrittstakte des Cache-Sammlers.", [
    ("graphLeaseWaitMilliseconds", 600000, "How long the graph reclaim waits for the exclusive lease of the graph partition before it gives up (10 minutes).", "Wie lange die Graph-Rueckgewinnung auf die exklusive Lease der Graph-Partition wartet, bevor sie aufgibt (10 Minuten)."),
    ("phaseBudgetMilliseconds", 3600000, "The most time one reclaim phase of an owner may run before its resource control expires (1 hour).", "Die meiste Zeit, die eine Rueckgewinnungsphase eines Eigentuemers laufen darf, bevor ihre Ressourcenkontrolle ablaeuft (1 Stunde)."),
    ("graphCollectorIntervalMilliseconds", 300000, "The least time between two launches of the graph collector (5 minutes).", "Die kleinste Zeit zwischen zwei Starts des Graph-Sammlers (5 Minuten)."),
    ("productsCollectorIntervalMilliseconds", 900000, "The least time between two launches of the products collector, which measures trees (15 minutes).", "Die kleinste Zeit zwischen zwei Starts des Produkt-Sammlers, der Baeume misst (15 Minuten)."),
    ("atimeProbeAgeMilliseconds", 172800000, "How far back the probe file of the access-time check is dated (two days).", "Wie weit die Probedatei der Zugriffszeit-Pruefung zurueckdatiert wird (zwei Tage)."),
    ("atimeProbeToleranceMilliseconds", 1000, "How much later than the dated time the access time must be after the read for the volume to count as recording it.", "Um wie viel spaeter als die zurueckdatierte Zeit die Zugriffszeit nach dem Lesen sein muss, damit das Laufwerk sie als erfasst gilt."),
    ("dedupeProgressMilliseconds", 250, "The least time between two progress reports of the deduplication.", "Die kleinste Zeit zwischen zwei Fortschrittsmeldungen der Deduplizierung."),
    ("dedupeCountProgressStride", 512, "After how many counted files the deduplication checks for progress and cancellation.", "Nach wie vielen gezaehlten Dateien die Deduplizierung auf Fortschritt und Abbruch prueft."),
])])
b.put(C + "🧬️schema/🟦️.ts", '''/** ⏲️ The limits of the cache collector, read from the sibling schema document (`🔣️.json`, definition `Limits`). */
import schema from "./🔣️.json" with {type: "json"};

/** 🧹️ The waits, budgets, intervals and progress cadences of the cache collector; the schema document is their single source. */
export const CACHE_GC_LIMITS_V1 = Object.freeze({...schema.definitions.Limits.const});
''')
imp = 'import {CACHE_GC_LIMITS_V1} from "../🧬️schema/🟦️.ts";\n'
b.subs(C + "⌨️command/🟦️.ts", [
    ('import {join} from "node:path";\n', 'import {join} from "node:path";\n' + imp),
    ("timeoutMs:600000", "timeoutMs:CACHE_GC_LIMITS_V1.graphLeaseWaitMilliseconds"),
    ("(phaseControl,3600000,", "(phaseControl,CACHE_GC_LIMITS_V1.phaseBudgetMilliseconds,", 2),
])
b.subs(C + "🚀️launch/🟦️.ts", [
    ('import {dirname,join} from "node:path";\n', 'import {dirname,join} from "node:path";\n' + imp),
    ("GRAPH_COLLECTOR_INTERVAL_MS_V1=300000;", "GRAPH_COLLECTOR_INTERVAL_MS_V1=CACHE_GC_LIMITS_V1.graphCollectorIntervalMilliseconds;"),
    ("PRODUCTS_COLLECTOR_INTERVAL_MS_V1=900000;", "PRODUCTS_COLLECTOR_INTERVAL_MS_V1=CACHE_GC_LIMITS_V1.productsCollectorIntervalMilliseconds;"),
])
b.subs(C + "🏃️runner/🟦️.ts", [
    ('import {isAbsolute,join,relative,resolve,sep} from "node:path";\n', 'import {isAbsolute,join,relative,resolve,sep} from "node:path";\n' + imp),
    ("Date.now()-2*86400000)", "Date.now()-CACHE_GC_LIMITS_V1.atimeProbeAgeMilliseconds)"),
    ("atimeMs>past.getTime()+1000", "atimeMs>past.getTime()+CACHE_GC_LIMITS_V1.atimeProbeToleranceMilliseconds"),
])
b.subs(C + "👯️dedupe/🟦️.ts", [
    ('import {dirname,join} from "node:path";\n', 'import {dirname,join} from "node:path";\n' + imp),
    ("at-last>=250", "at-last>=CACHE_GC_LIMITS_V1.dedupeProgressMilliseconds"),
    ("++seen%512===0", "++seen%CACHE_GC_LIMITS_V1.dedupeCountProgressStride===0"),
])
rel = lambda p: "🏗️build/🧹️cache-gc/" + p
b.row("cache-gc", "library", rel("🧬️schema/🔣️.json"), ["Limits"], [rel("⌨️command/🟦️.ts"), rel("🚀️launch/🟦️.ts"), rel("🏃️runner/🟦️.ts"), rel("👯️dedupe/🟦️.ts")], exports=[(rel("🧬️schema/🟦️.ts"), "CACHE_GC_LIMITS_V1", "Limits")], minimum_number=250, allow=[(rel("🏃️runner/🟦️.ts"), 512, "The size of a disk block in bytes: a unit of the block count, not the stride of the deduplication.")])
b.write(tests=["./" + C + "🧪️tests", "./" + LAW + "🟦️.ts"])
