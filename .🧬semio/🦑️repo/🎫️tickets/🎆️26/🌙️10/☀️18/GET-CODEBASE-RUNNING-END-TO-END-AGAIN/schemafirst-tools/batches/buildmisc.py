import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

b = Batch("buildmisc", "the derived-walk report interval, the input closure memo limits and chunk cut, and the build command runtime limits come from schemas (the precision schema is in the graph implementation inventory: re-keys the implementation digest)")
P0 = BUILD + "📥️inputs/🎯️precision/"
b.add_defs(P0 + "🧬️schema/🔣️.json", [Def("Limits", "The progress cadence of the derived walk.", "Der Fortschrittstakt des abgeleiteten Durchlaufs.", [
    ("programReportMilliseconds", 5000, "The least time between two progress reports of a program walk.", "Die kleinste Zeit zwischen zwei Fortschrittsmeldungen eines Programmdurchlaufs."),
])])
b.subs(P0 + "🟦️.ts", [
    ("const PROGRAM_REPORT_MS = 5000;", "const PROGRAM_REPORT_MS = schema.definitions.Limits.const.programReportMilliseconds;"),
])
I = BUILD + "📥️inputs/"
b.new_schema(I + "🧬️schema/🔣️.json", "Build input limits / Grenzen der Build-Eingaben", [Def("Limits", "The memo limits and the chunk cut of the input closure.", "Die Speichergrenzen und der Blockschnitt der Eingabe-Huelle.", [
    ("inventoryMemoLimit", 4096, "The most captured inventories the process remembers at once.", "Die meisten erfassten Inventare, die der Prozess gleichzeitig merkt."),
    ("chunkCutsLimit", 131072, "The most verified closure stamps the process remembers at once.", "Die meisten verifizierten Huellen-Stempel, die der Prozess gleichzeitig merkt."),
    ("closureChunkMinimum", 16, "The fewest paths a closure chunk holds before the content-defined cut may end it.", "Die wenigsten Pfade eines Huellen-Blocks, bevor der inhaltsbestimmte Schnitt ihn beenden darf."),
    ("closureChunkModulus", 128, "A closure chunk ends after a path whose hash is zero modulo this number.", "Ein Huellen-Block endet nach einem Pfad, dessen Hash modulo dieser Zahl null ist."),
])])
b.subs(I + "🟨️.mjs", [
    ('import{physicalSourceOriginV1}from"./🔏️revision/🟨️.mjs";\n', 'import{REVISION_POLICY_V1,physicalSourceOriginV1}from"./🔏️revision/🟨️.mjs";\nimport limitsSchema from"./🧬️schema/🔣️.json"with{type:"json"};\n'),
    ("INVENTORY_MEMO_LIMIT_V1 = 4096,", "INVENTORY_MEMO_LIMIT_V1 = limitsSchema.definitions.Limits.const.inventoryMemoLimit,"),
    ("CHUNK_CUTS_LIMIT_V1 = 131072;", "CHUNK_CUTS_LIMIT_V1 = limitsSchema.definitions.Limits.const.chunkCutsLimit;"),
    ("INPUT_CLOSURE_CHUNK_MINIMUM_V1 = 16, INPUT_CLOSURE_CHUNK_MODULUS_V1 = 128;", "INPUT_CLOSURE_CHUNK_MINIMUM_V1 = limitsSchema.definitions.Limits.const.closureChunkMinimum, INPUT_CLOSURE_CHUNK_MODULUS_V1 = limitsSchema.definitions.Limits.const.closureChunkModulus;"),
    ('temporary.length>256||[...temporary].length>256)throw Error("Input closure staging path exceeds256 characters")', 'temporary.length>REVISION_POLICY_V1.maximumPathUnits||[...temporary].length>REVISION_POLICY_V1.maximumPathUnits)throw Error("Input closure staging path exceeds "+REVISION_POLICY_V1.maximumPathUnits+" characters")'),
])
Q = BUILD + "⌨️command/⚙️runtime/"
b.new_schema(Q + "🧬️schema/🔣️.json", "Build command runtime limits / Grenzen der Laufzeit des Build-Befehls", [Def("Limits", "The limits of the custody session of the build command runtime and the grace of a stopped remote execution server.", "Die Grenzen der Verwahr-Sitzung der Laufzeit des Build-Befehls und die Gnadenfrist eines gestoppten Remote-Ausfuehrungs-Servers.", [
    ("sessionMaximumRequests", 200, "The most requests one custody session serves before it ends.", "Die meisten Anfragen, die eine Verwahr-Sitzung bedient, bevor sie endet."),
    ("sessionMaximumAgeMilliseconds", 600000, "The longest life of one custody session (10 minutes).", "Die laengste Lebensdauer einer Verwahr-Sitzung (10 Minuten)."),
    ("sessionIdleMilliseconds", 20000, "How long a custody session may stay idle before it ends.", "Wie lange eine Verwahr-Sitzung untaetig bleiben darf, bevor sie endet."),
    ("serverStopGraceMilliseconds", 15000, "How long a stopped remote execution server may take to exit before it is killed.", "Wie lange ein gestoppter Remote-Ausfuehrungs-Server zum Beenden brauchen darf, bevor er beendet wird."),
])])
b.put(Q + "🧬️schema/🟦️.ts", '''/** ⏲️ The limits of the build command runtime, read from the sibling schema document (`🔣️.json`, definition `Limits`). */
import schema from "./🔣️.json" with {type: "json"};

/** 🧱️ The custody session limits and the stop grace of the build command runtime; the schema document is their single source. */
export const COMMAND_RUNTIME_LIMITS_V1 = Object.freeze({...schema.definitions.Limits.const});
''')
b.subs(Q + "🟦️.ts", [
    ('import { join, resolve, dirname } from "node:path";\n', 'import { join, resolve, dirname } from "node:path";\nimport { COMMAND_RUNTIME_LIMITS_V1 } from "./🧬️schema/🟦️.ts";\n'),
    ("limits:{maximumRequests:200,maximumAgeMilliseconds:600000,idleMilliseconds:20000}", "limits:{maximumRequests:COMMAND_RUNTIME_LIMITS_V1.sessionMaximumRequests,maximumAgeMilliseconds:COMMAND_RUNTIME_LIMITS_V1.sessionMaximumAgeMilliseconds,idleMilliseconds:COMMAND_RUNTIME_LIMITS_V1.sessionIdleMilliseconds}"),
    ("runtimeSleepV1(15000)", "runtimeSleepV1(COMMAND_RUNTIME_LIMITS_V1.serverStopGraceMilliseconds)"),
])
rel = lambda p: "🏗️build/" + p
b.row("precision", "library", rel("📥️inputs/🎯️precision/🧬️schema/🔣️.json"), ["Limits"], [rel("📥️inputs/🎯️precision/🟦️.ts")], minimum_number=1000)
b.row("inputs", "library", rel("📥️inputs/🧬️schema/🔣️.json"), ["Limits"], [rel("📥️inputs/🟨️.mjs")])
b.row("command-runtime", "library", rel("⌨️command/⚙️runtime/🧬️schema/🔣️.json"), ["Limits"], [rel("⌨️command/⚙️runtime/🟦️.ts")], exports=[(rel("⌨️command/⚙️runtime/🧬️schema/🟦️.ts"), "COMMAND_RUNTIME_LIMITS_V1", "Limits")])
b.write(tests=["./" + P0 + "🧪️tests", "./" + LAW + "🟦️.ts"])
