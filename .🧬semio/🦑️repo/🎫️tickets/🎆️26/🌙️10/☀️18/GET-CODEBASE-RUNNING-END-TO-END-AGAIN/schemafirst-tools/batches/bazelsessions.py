import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import *

Z = BUILD + "🔧️tools/🏗️bazel/🧵️sessions/"
b = Batch("bazelsessions", "the Bazel session slots read the minimum server memory, the storage path ceiling and the spare sampling window from their policy, and every section of the policy is described in English and German (no cache key carries these files)")
b.add_defs(Z + "🧬️schema/🔣️.json", [Def("Policy", "The authored numbers and names of the Bazel session slots: server sizing, waiting, retention, stop, naming, leases, pressure collection, pool age and launcher deduplication.", "Die verfassten Zahlen und Namen der Bazel-Sitzungsplaetze: Server-Dimensionierung, Warten, Aufbewahrung, Stoppen, Benennung, Leases, Druck-Sammlung, Pool-Alter und Launcher-Deduplizierung.", [
    ("version", 1, "The version of the policy.", "Die Version der Richtlinie."),
    ("servers", KEEP, "How many Bazel servers run at once and what each is given: memory share, heap, resident size, cores and growth.", "Wie viele Bazel-Server gleichzeitig laufen und was jeder bekommt: Speicheranteil, Heap, residente Groesse, Kerne und Wachstum."),
    ("waiting", KEEP, "How often a command that waits for a slot polls and reports progress.", "Wie oft ein Befehl, der auf einen Platz wartet, abfragt und Fortschritt meldet."),
    ("retention", KEEP, "How many packages a slot keeps and how old a package or an output base may be.", "Wie viele Pakete ein Platz behaelt und wie alt ein Paket oder eine Ausgabebasis sein darf."),
    ("stop", KEEP, "How a server is stopped: its pid file and the timeout and polling of the stop.", "Wie ein Server gestoppt wird: seine Pid-Datei sowie Zeitlimit und Abfrage des Stoppens."),
    ("names", KEEP, "The patterns and names of slots, keys, stable packages and state files.", "Die Muster und Namen von Plaetzen, Schluesseln, stabilen Paketen und Zustandsdateien."),
    ("leases", KEEP, "The resource names of the slot leases and the most resources one restore holds.", "Die Ressourcennamen der Platz-Leases und die meisten Ressourcen, die eine Wiederherstellung haelt."),
    ("pressure", KEEP, "The thresholds of the collection under disk pressure: protected packages, grace, idle time and measuring concurrency.", "Die Schwellen der Sammlung unter Plattendruck: geschuetzte Pakete, Gnadenfrist, Leerlaufzeit und Messparallelitaet."),
    ("pool", KEEP, "The age after which a pooled package is removed.", "Das Alter, nach dem ein gepooltes Paket entfernt wird."),
    ("dedupe", KEEP, "The smallest launcher that is deduplicated and where the stamp of the deduplication lives.", "Der kleinste Launcher, der dedupliziert wird, und wo der Stempel der Deduplizierung liegt."),
    ("limits", {"minimumMemoryPerServerMegabytes": 256, "pathUnits": 256, "spareSampleMilliseconds": 5000}, "The floor of the memory one server is planned with, the longest storage path and how long a sampled spare memory is reused.", "Die Untergrenze des Speichers, mit dem ein Server geplant wird, der laengste Speicherpfad und wie lange ein gemessener freier Speicher wiederverwendet wird."),
])])
b.subs(Z + "🟦️.ts", [
    ("Math.max(256,Math.floor((spare-budget)/slots))", "Math.max(policy.limits.minimumMemoryPerServerMegabytes,Math.floor((spare-budget)/slots))"),
    ("Math.max(256,Math.floor(plan.memoryPerServerMegabytes*plan.slots/slots))", "Math.max(policy.limits.minimumMemoryPerServerMegabytes,Math.floor(plan.memoryPerServerMegabytes*plan.slots/slots))"),
    ('if(path.length>256)throw Error("Bazel storage path exceeds 256 characters")', 'if(path.length>policy.limits.pathUnits)throw Error("Bazel storage path exceeds "+policy.limits.pathUnits+" characters")'),
])
b.subs(Z + "🔌️port/🟦️.ts", [
    ("sampled.at<5000", "sampled.at<policy.limits.spareSampleMilliseconds"),
    ("sampledFloor.at<5000", "sampledFloor.at<policy.limits.spareSampleMilliseconds"),
])
rel = lambda p: "🏗️build/🔧️tools/🏗️bazel/🧵️sessions/" + p
b.row("bazel-sessions", "library", rel("🧬️schema/🔣️.json"), ["Policy"], [rel("🟦️.ts"), rel("🔌️port/🟦️.ts")], minimum_number=250, allow=[(rel("🟦️.ts"), 10000, "The scale of a basis-point ratio (10000 basis points are one whole), not the stop timeout.")])
b.write(tests=["./" + Z + "🧪️tests", "./" + LAW + "🟦️.ts"])
