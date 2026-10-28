import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

H = PROC + "🏗️build/🌡️host-capacity/"
b = Batch("hostcap", "the host capacity sampling, gate, upgrade, estimates and decision read their probe limits, heartbeat ceiling, settled-outcome cache and median from the host capacity schema (no cache key carries these files)")
S = H + "🧬️schema/🔣️.json"
b.add_defs(S, [Def("Limits", "The probe, heartbeat and cache limits of the host capacity engine.", "Die Probe-, Herzschlag- und Cache-Grenzen der Host-Kapazitaets-Engine.", [
    ("optionalProbeMilliseconds", 3000, "How long an optional counter probe (disk images, memory pressure) may run before it is abandoned.", "Wie lange eine optionale Zaehlerabfrage (Disk-Images, Speicherdruck) laufen darf, bevor sie verworfen wird."),
    ("commandTimeoutMilliseconds", 10000, "The hard timeout of one platform command that reads a counter.", "Das harte Zeitlimit eines Plattformbefehls, der einen Zaehler liest."),
    ("commandOutputBytes", 4194304, "The most output bytes of one platform command that is read (4 MiB).", "Die meisten Ausgabebytes eines Plattformbefehls, die gelesen werden (4 MiB)."),
    ("defaultPageBytes", 4096, "The memory page size assumed when the platform does not report one.", "Die angenommene Speicherseitengroesse, wenn die Plattform keine meldet."),
    ("reasonCharacters", 200, "The most characters of the reason a failed probe reports.", "Die meisten Zeichen des Grundes, den eine fehlgeschlagene Abfrage meldet."),
    ("heartbeatFirstMilliseconds", 3000, "The longest wait before the first heartbeat of a waiting request, whatever the policy interval says.", "Die laengste Wartezeit bis zum ersten Herzschlag einer wartenden Anfrage, egal was das Richtlinien-Intervall sagt."),
    ("settledOutcomeLimit", 256, "The most upgrade outcomes remembered by key; the oldest goes first.", "Die meisten nach Schluessel gemerkten Hochstufungs-Ergebnisse; das aelteste geht zuerst."),
    ("quickQuantileBasisPoints", 5000, "The quantile of the active time, in basis points, that decides whether a class of runs counts as quick (the median).", "Das Quantil der aktiven Zeit in Basispunkten, das entscheidet, ob eine Laufklasse als schnell gilt (der Median)."),
])])
b.subs(H + "📡️sampling/🟦️.ts", [
    ("const OBSERVERS=schema.definitions.Observers.const,OPTIONAL_PROBE_MILLISECONDS=3000,MEGABYTE=1048576,", "const OBSERVERS=schema.definitions.Observers.const,LIMITS=Object.freeze({...schema.definitions.Limits.const}),OPTIONAL_PROBE_MILLISECONDS=LIMITS.optionalProbeMilliseconds,MEGABYTE=1048576,"),
    ("?.[1]??4096)", "?.[1]??LIMITS.defaultPageBytes)"),
    ("Number(out[1])*4096", "Number(out[1])*LIMITS.defaultPageBytes"),
    ("pages*4096", "pages*LIMITS.defaultPageBytes"),
    ("{timeout:10000,maxBuffer:4194304,windowsHide:true,", "{timeout:LIMITS.commandTimeoutMilliseconds,maxBuffer:LIMITS.commandOutputBytes,windowsHide:true,"),
    ('.split("\\n")[0]!.slice(0,200)', '.split("\\n")[0]!.slice(0,LIMITS.reasonCharacters)', 2),
])
b.subs(H + "🎫️gate/🟦️.ts", [
    ("const protocol=schema.definitions.Protocol.const;\n", "const protocol=schema.definitions.Protocol.const,limits=schema.definitions.Limits.const;\n"),
    ("Math.min(3000,policy.wait.heartbeatMilliseconds)", "Math.min(limits.heartbeatFirstMilliseconds,policy.wait.heartbeatMilliseconds)"),
])
b.subs(H + "🔼️upgrade/🟦️.ts", [
    ("const protocol=schema.definitions.Protocol.const;\n", "const protocol=schema.definitions.Protocol.const,limits=schema.definitions.Limits.const;\n"),
    ("settled.size>256", "settled.size>limits.settledOutcomeLimit"),
])
b.sub(H + "📏️decision/🟦️.ts", "Math.min(input.preferredJobs??1,1024)", "Math.min(input.preferredJobs??1,protocol.maximumJobs)")
b.subs(H + "📐️estimates/🟦️.ts", [
    ('"activeMilliseconds",5000)<=rules.quickMilliseconds', '"activeMilliseconds",schema.definitions.Limits.const.quickQuantileBasisPoints)<=rules.quickMilliseconds'),
])
rel = lambda p: "🏗️build/🌡️host-capacity/" + p
b.row("host-capacity", "process", rel("🧬️schema/🔣️.json"), ["Limits"], [rel("📡️sampling/🟦️.ts"), rel("🎫️gate/🟦️.ts"), rel("🔼️upgrade/🟦️.ts"), rel("📐️estimates/🟦️.ts"), rel("📏️decision/🟦️.ts")], allow=[
    (rel("📡️sampling/🟦️.ts"), 10000, "The scale of a basis-point ratio (10000 basis points are one whole), not the timeout of a platform command."),
    (rel("📐️estimates/🟦️.ts"), 10000, "The scale of a basis-point ratio (10000 basis points are one whole)."),
    (rel("📏️decision/🟦️.ts"), 10000, "The scale of a basis-point ratio (10000 basis points are one whole)."),
    (rel("🔼️upgrade/🟦️.ts"), 10000, "The scale of a basis-point ratio (10000 basis points are one whole)."),
    (rel("🎫️gate/🟦️.ts"), 10000, "The scale of a basis-point ratio (10000 basis points are one whole)."),
])
b.write(tests=["./" + H + "🧪️tests/🟦️.ts", "./" + LAW + "🟦️.ts"])
