import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

R = BUILD + "📥️inputs/🔭️read-oracle/"
ROWS = [
    ("memoryLimitMegabytes", 500, 64, 65536, "The memory the read oracle may hold before it aborts.", "Der Speicher, den das Lese-Orakel belegen darf, bevor es abbricht."),
    ("tailBytes", 65536, 1024, 16777216, "The most bytes of the end of a command's output that are kept.", "Die meisten Bytes vom Ende der Ausgabe eines Befehls, die behalten werden."),
    ("logOutputBytes", 268435456, 1048576, 1073741824, "The most bytes of the macOS unified log that are read after a round (256 MiB).", "Die meisten Bytes des macOS-Unified-Logs, die nach einer Runde gelesen werden (256 MiB)."),
    ("reasonCharacters", 200, 20, 2000, "The most characters of the reason a backend probe reports.", "Die meisten Zeichen des Grundes, den die Pruefung eines Backends meldet."),
    ("refusalCharacters", 300, 20, 2000, "The most characters of the refusal a sandbox profile or the Landlock helper reports.", "Die meisten Zeichen der Verweigerung, die ein Sandbox-Profil oder der Landlock-Helfer meldet."),
    ("outputTailCharacters", 1500, 100, 100000, "The most characters of a round's output that its report keeps.", "Die meisten Zeichen der Ausgabe einer Runde, die ihr Bericht behaelt."),
    ("flushWaitMilliseconds", 60000, 1000, 600000, "How long the oracle waits until the unified log shows the marker of the round.", "Wie lange das Orakel wartet, bis das Unified Log die Markierung der Runde zeigt."),
    ("markerCommandMilliseconds", 30000, 1000, 600000, "The timeout of the command that writes the marker into the unified log.", "Das Zeitlimit des Befehls, der die Markierung in das Unified Log schreibt."),
    ("markerPollMilliseconds", 300, 10, 10000, "The pause between two looks into the unified log for the marker.", "Die Pause zwischen zwei Blicken ins Unified Log nach der Markierung."),
    ("logLookbackMilliseconds", 2000, 0, 60000, "How far before the start of a round the unified log is read.", "Wie weit vor dem Start einer Runde das Unified Log gelesen wird."),
    ("seatbeltCapacity", 600, 24, 100000, "The number of rules a macOS sandbox profile may hold before the plan is split into rounds.", "Die Zahl der Regeln, die ein macOS-Sandbox-Profil halten darf, bevor der Plan in Runden geteilt wird."),
    ("smallestCapacity", 24, 1, 1000, "The fewest rules a round is split down to when the kernel refuses a profile.", "Die wenigsten Regeln, auf die eine Runde geteilt wird, wenn der Kernel ein Profil verweigert."),
    ("defaultTimeoutSeconds", 600, 1, 86400, "The default limit of a read oracle run on the command line, in seconds.", "Die Standardgrenze eines Laufs des Lese-Orakels auf der Kommandozeile, in Sekunden."),
]
def schema_change(document):
    policy = document["definitions"]["Policy"]
    policy["required"] = [row[0] for row in ROWS]
    policy["properties"] = {row[0]: {"type": "integer", "minimum": row[2], "maximum": row[3], "description": "EN: %s DE: %s" % (row[4], row[5])} for row in ROWS}
def data_change(document):
    document["policy"] = {row[0]: row[1] for row in ROWS}
b = Batch("readoracle", "the read oracle reads its log and output bounds, sandbox capacity, timeouts and default limit from the policy of its document, whose schema describes every property in English and German (no cache key carries these files)")
b.edit_json(R + "🧬️schema/🔣️.json", schema_change)
b.edit_json(R + "🔣️.json", data_change)
S = R + "🍎️seatbelt/🟦️.ts"
b.subs(S, [
    ('import {join} from "node:path";\n', 'import {join} from "node:path";\nimport document from "../🔣️.json" with {type: "json"};\n'),
    ('maxBuffer: 256 * 1024 * 1024}', 'maxBuffer: document.policy.logOutputBytes}'),
    ('.trim().slice(0, 200)}, " / ")', '.trim().slice(0, document.policy.reasonCharacters)}, " / ")'),
    ("export const seatbeltCapacityV1 = 600;", "export const seatbeltCapacityV1 = document.policy.seatbeltCapacity;"),
    ("const SMALLEST_CAPACITY = 24;", "const SMALLEST_CAPACITY = document.policy.smallestCapacity;"),
    ("limitMilliseconds = 60000)", "limitMilliseconds = document.policy.flushWaitMilliseconds)"),
    ("timeoutMs: 30000,", "timeoutMs: document.policy.markerCommandMilliseconds,"),
    ("await sleep(300);", "await sleep(document.policy.markerPollMilliseconds);"),
    ("Date.now() - 2000)", "Date.now() - document.policy.logLookbackMilliseconds)"),
    ("done.output.trim().slice(0, 300)", "done.output.trim().slice(0, document.policy.refusalCharacters)", 2),
    ("done.output.slice(-1500)", "done.output.slice(-document.policy.outputTailCharacters)"),
])
b.subs(R + "🐧️linux/🟦️.ts", [
    ('import {join} from "node:path";\n', 'import {join} from "node:path";\nimport document from "../🔣️.json" with {type: "json"};\n'),
    ('.trim().slice(0, 200), trace}', '.trim().slice(0, document.policy.reasonCharacters), trace}'),
    ("done.output.trim().slice(-300)", "done.output.trim().slice(-document.policy.refusalCharacters)"),
    ("done.output.slice(-1500)", "done.output.slice(-document.policy.outputTailCharacters)"),
])
b.subs(R + "🚪️cli/🟦️.ts", [
    ('import {readFileSync, writeFileSync} from "node:fs";\n', 'import {readFileSync, writeFileSync} from "node:fs";\nimport document from "../🔣️.json" with {type: "json"};\n'),
    ('?? 600) * 1000', '?? document.policy.defaultTimeoutSeconds) * 1000'),
])
rel = lambda p: "🏗️build/📥️inputs/🔭️read-oracle/" + p
b.row("read-oracle", "library", rel("🧬️schema/🔣️.json"), ["Policy"], [rel("🍎️seatbelt/🟦️.ts"), rel("🐧️linux/🟦️.ts"), rel("🚪️cli/🟦️.ts"), rel("🏃️run/🟦️.ts")], document=rel("🔣️.json"), pointer=["policy"], minimum_number=200)
b.write(tests=["./" + R + "🧪️tests", "./" + LAW + "🟦️.ts"])
