"""Adds the product benchmark catalogue (units, probes, budgets) and the new labels to the SLO table of a staged bench copy.
usage: stage-catalogue.py <bench dir> <labels-added.json>
"""
import json, os, sys

root, labels_file = sys.argv[1], sys.argv[2]
path = os.path.join(root, "🎯️slo", "🔣️.json")
text = open(path, encoding="utf8").read()
t = json.loads(text)
GiB, MiB = 1073741824, 1048576

for key, (en, de) in json.load(open(labels_file, encoding="utf8")).items():
    t["labels"][key] = {"en": en, "de": de}

FW = "🧰️framework/"
products = [
    # id, EN, DE, target, language, file, edit budget ms
    ("dashboard", "dashboard", "Dashboard", "@semio-tech/repo-dashboard-rs:build", "rs", FW + "🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🦀️.rs", 120000),
    ("course", "architecture course", "Architekturkurs", "@teaching/architecture-course:build", "ts", "🎓️teaching/🏛️architecture/📚️course/📚️course/🟦️.ts", 60000),
    ("print", "print", "Druck", "@semio-tech/print:build", "ts", FW + "🛍️products/📓️print/📦️artifacts/📖️pdf/🟦️.ts", 60000),
]
peers = [
    ("unrelated", "unrelated plugin script", "fremdes Plugin-Skript", "ts", "comment", "✏️s/🔌️plugins/🌀️procedural/📦️packages/🟦️typescript/📜️script.ts"),
    ("scheduling", "scheduling file of the build library", "Scheduling-Datei der Build-Bibliothek", "ts", "comment", FW + "🔨️modules/🏃️process/🏗️build/🎟️capacity/🪆️scope/🤲️consumption/🟦️.ts"),
]

def L(en, de):
    return {"en": en, "de": de}

for pid, en, de, target, lang, file, _ in products:
    t["units"].append({"id": "product-" + pid, "language": lang, "target": target, "file": file, "op": "private-body", "title": L("One-line edit of the %s source" % en, "Einzeilige Änderung der Quelle: %s" % de)})
for qid, en, de, lang, op, file in peers:
    t["units"].append({"id": "peer-" + qid, "language": lang, "target": "@semio-tech/framework-graph:test", "file": file, "op": op, "title": L("Peer edit of a %s" % en, "Fremdänderung: %s" % de)})

def probe(ident, kind, title, **extra):
    row = {"id": ident, "kind": kind, "title": title}
    row.update(extra)
    t["probes"].append(row)

def slo(ident, title, probe_id, metric, statistic, goal, gate, enforcement, rel, absolute, source="product-budget", direction="lower"):
    t["slos"].append({"id": ident, "title": title, "probe": probe_id, "metric": metric, "statistic": statistic, "direction": direction, "goal": goal, "gate": gate, "tools": ["nx", "bazel"], "enforcement": enforcement, "tolerance": {"relative": rel, "absolute": absolute}, "source": source})

MS = (0.2, 500)
PHASES = [("queueMs", "time in queues", "Zeit in Warteschlangen"), ("graphMs", "graph preparation", "Graph-Vorbereitung"), ("executeMs", "tool execution", "Werkzeugausführung")]

for pid, en, de, target, lang, file, edit_budget in products:
    cold, noop, edit = "product-%s-cold" % pid, "product-%s-noop" % pid, "product-%s-edit" % pid
    probe(cold, "noop", L("First build of the %s on a fresh copy" % en, "Erster Bau: %s auf frischer Kopie" % de), lane="oneshot", target=target, runs=1, warmup=0, cold=True, memory=True)
    probe(noop, "noop", L("Warm no-op build of the %s" % en, "Warmer Leerlauf-Bau: %s" % de), lane="oneshot", target=target, runs=5, warmup=0, memory=True)
    probe(edit, "edit", L("One-line edit and rebuild of the %s" % en, "Einzeilige Änderung und Neubau: %s" % de), lane="oneshot", unit="product-" + pid, runs=3, warmup=0, memory=True)
    slo(cold + "-wall-max", L("First build of the %s, wall time" % en, "Erster Bau: %s, Laufzeit" % de), cold, "wallMs", "max", None, None, "advisory", 0.25, 5000, "provisional")
    slo(cold + "-rss-max", L("First build of the %s, peak memory of the process tree" % en, "Erster Bau: %s, Speicherspitze des Prozessbaums" % de), cold, "peakRssBytes", "max", None, None, "advisory", 0.15, 64 * MiB, "provisional")
    slo(noop + "-wall-p50", L("Warm no-op of the %s, median wall time" % en, "Warmer Leerlauf: %s, Median der Laufzeit" % de), noop, "wallMs", "p50", 10000, 10000, "blocking", *MS)
    slo(noop + "-rss-max", L("Warm no-op of the %s, peak memory of the process tree" % en, "Warmer Leerlauf: %s, Speicherspitze des Prozessbaums" % de), noop, "peakRssBytes", "max", int(1.5 * GiB), int(1.5 * GiB), "blocking", 0.15, 64 * MiB)
    slo(noop + "-executed-max", L("Warm no-op of the %s, re-executed actions" % en, "Warmer Leerlauf: %s, neu ausgeführte Aktionen" % de), noop, "executedActions", "max", 0, 0, "blocking", 0, 0)
    for metric, pen, pde in PHASES:
        slo("%s-%s-p50" % (noop, metric.lower().replace("ms", "")), L("Warm no-op of the %s, %s, median" % (en, pen), "Warmer Leerlauf: %s, %s, Median" % (de, pde)), noop, metric, "p50", None, None, "advisory", *MS, source="provisional")
    slo(edit + "-green-p50", L("One-line edit of the %s, edit to green, median" % en, "Einzeilige Änderung: %s, Änderung bis grün, Median" % de), edit, "greenMs", "p50", edit_budget, edit_budget, "blocking", 0.2, 1000)
    slo(edit + "-executed-max", L("One-line edit of the %s, re-executed actions" % en, "Einzeilige Änderung: %s, neu ausgeführte Aktionen" % de), edit, "executedActions", "max", None, None, "advisory", 0.2, 2, "provisional")
    slo(edit + "-rss-max", L("One-line edit of the %s, peak memory of the process tree" % en, "Einzeilige Änderung: %s, Speicherspitze des Prozessbaums" % de), edit, "peakRssBytes", "max", None, None, "advisory", 0.15, 64 * MiB, "provisional")
    for metric, pen, pde in PHASES:
        slo("%s-%s-p50" % (edit, metric.lower().replace("ms", "")), L("One-line edit of the %s, %s, median" % (en, pen), "Einzeilige Änderung: %s, %s, Median" % (de, pde)), edit, metric, "p50", None, None, "advisory", *MS, source="provisional")
    for qid, qen, qde, qlang, qop, qfile in peers:
        peer = "product-%s-peer-%s" % (pid, qid)
        probe(peer, "peer", L("Build of the %s after a peer edit of a %s" % (en, qen), "Bau: %s nach Fremdänderung: %s" % (de, qde)), lane="oneshot", target=target, unit="peer-" + qid, runs=3, warmup=0, memory=True)
        slo(peer + "-executed-max", L("%s after a peer edit of a %s, re-executed actions" % (en.capitalize(), qen), "%s nach Fremdänderung (%s), neu ausgeführte Aktionen" % (de, qde)), peer, "executedActions", "max", 0, 0, "blocking", 0, 0)
        slo(peer + "-graph-p50", L("%s after a peer edit of a %s, graph preparation, median" % (en.capitalize(), qen), "%s nach Fremdänderung (%s), Graph-Vorbereitung, Median" % (de, qde)), peer, "graphMs", "p50", 120000, 120000, "blocking", 0.2, 2000)
        slo(peer + "-inferred-max", L("%s after a peer edit of a %s, re-inferred projects" % (en.capitalize(), qen), "%s nach Fremdänderung (%s), neu abgeleitete Projekte" % (de, qde)), peer, "inferredProjects", "max", None, None, "advisory", 0.2, 2, "provisional")
        slo(peer + "-rss-max", L("%s after a peer edit of a %s, peak memory of the process tree" % (en.capitalize(), qen), "%s nach Fremdänderung (%s), Speicherspitze des Prozessbaums" % (de, qde)), peer, "peakRssBytes", "max", 2 * GiB, 2 * GiB, "blocking", 0.15, 64 * MiB)

probe("product-agents", "concurrency", L("Three agents build the three products at once", "Drei Agenten bauen die drei Produkte gleichzeitig"), workload="product-agents", sessions=3, runs=1, warmup=0)
slo("product-agents-makespan-max", L("Makespan of three agents on the three products", "Gesamtdauer dreier Agenten auf den drei Produkten"), "product-agents", "makespanMs", "max", None, None, "advisory", 0.25, 1000, "provisional")
slo("product-agents-wait-max", L("Longest wait of an agent for an unrelated product", "Längste Wartezeit eines Agenten auf ein fremdes Produkt"), "product-agents", "maxUnrelatedWaitMs", "max", 1000, 5000, "advisory", 0.25, 250, "product-budget")
slo("product-agents-speedup-min", L("Speed-up of three agents over serial runs", "Beschleunigung dreier Agenten gegenüber seriellen Läufen"), "product-agents", "speedup", "min", None, None, "advisory", 0.25, 0.1, "provisional", "higher")

out = json.dumps(t, indent=1, ensure_ascii=False)
open(path, "w", encoding="utf8").write(out + ("\n" if text.endswith("\n") else ""))
print(len(t["units"]), "units", len(t["probes"]), "probes", len(t["slos"]), "objectives")
