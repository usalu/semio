#!/usr/bin/env python3
"""H83: anchored, atomic patch of the fleet-status schema (policy.summary, Summary, Selection, texts). Usage: schema.py <schema.json>"""
import json, os, sys, copy
path = sys.argv[1]
text = open(path, encoding="utf-8").read()
if '"Summary": {' in text:
    sys.exit("already applied")

def once(old, new):
    global text
    assert text.count(old) == 1, (old, text.count(old))
    text = text.replace(old, new)

I = lambda m=0: {"type": "integer", "minimum": m}
NI = {"anyOf": [I(), {"type": "null"}]}
S = {"type": "string"}
def obj(props, optional=()):
    return {"type": "object", "additionalProperties": False, "required": sorted(k for k in props if k not in optional), "properties": props}
def arr(item): return {"type": "array", "items": item}
def rows(): return {"type": "array", "items": I()}

ANOMALOUS = ["cache.skip.spool-full", "cache.skip.upload-expired", "cache.skip.upload-expired-down", "cache.skip.upload-expired-denied", "cache.skip.upload-entry-gone", "cache.skip.upload-failed", "cache.skip.upload-dropped", "cache.skip.key-incomplete", "cache.skip.path-dependent", "cache.skip.read-only", "cache.skip.diverged", "cache.skip.breaker-denied", "cache.skip.other", "exec.offload.fellback", "exec.refused", "warden.cancelled", "cache.remote.refused", "exec.decline.infrastructure", "exec.decline.pin-failed", "exec.decline.not-portable", "exec.decline.queue-timeout", "exec.decline.incomplete-inputs", "exec.decline.unmapped-path", "exec.decline.other"]
SECTIONS = ["summary", "memory", "runs", "queues", "bazel", "storage", "sealed", "generation", "scheduler", "fleet", "workers", "cas", "retention", "offload", "rates", "findings", "sources"]

policy = {"maximumSubjects": 8, "textCharacters": 200, "maximumOriginMetrics": 4, "maximumFleetMetrics": 16, "maximumReasonsShown": 6, "phaseCharacters": 40, "anomalousMetrics": ANOMALOUS, "sections": SECTIONS}
block = json.dumps(policy, indent=1, ensure_ascii=False).replace("\n", "\n    ")
once('    "digestCharacters": 12\n   }\n  },\n  "PolicySchema"', '    "digestCharacters": 12,\n    "summary": ' + block + '\n   }\n  },\n  "PolicySchema"')
once('    "runs",\n    "version"\n   ],\n   "properties": {\n    "version": {\n     "const": 1\n    },', '    "runs",\n    "summary",\n    "version"\n   ],\n   "properties": {\n    "version": {\n     "const": 1\n    },')
ps = {"type": "object", "additionalProperties": False, "required": sorted(policy), "properties": {
    "maximumSubjects": I(1), "textCharacters": I(2), "maximumOriginMetrics": I(1), "maximumFleetMetrics": I(1), "maximumReasonsShown": I(1), "phaseCharacters": I(2),
    "anomalousMetrics": {"type": "array", "minItems": 1, "items": S}, "sections": {"type": "array", "minItems": 1, "items": {"enum": SECTIONS}}}}
once('    "digestCharacters": {\n     "type": "integer",\n     "minimum": 0\n    }\n   }\n  },\n  "Messages"', '    "digestCharacters": {\n     "type": "integer",\n     "minimum": 0\n    },\n    "summary": ' + json.dumps(ps, indent=1).replace("\n", "\n    ") + '\n   }\n  },\n  "Messages"')

D = json.loads(text)["definitions"]
R = D["Report"]["properties"]
memory = copy.deepcopy(R["memory"]); del memory["properties"]["warden"]
ref = lambda name: {"$ref": "#/definitions/Report/properties/" + name}
builder = copy.deepcopy(R["queues"]["properties"]["graph"]["properties"]["builder"])
run = obj({"pid": I(), "state": {"enum": ["running", "blocked", "waiting"]}, "who": S, "what": S, "phase": S, "done": I(), "total": I(), "etaSeconds": I(), "ageSeconds": I(), "priority": {"enum": ["interactive", "agent", "background"]}, "waiting": obj({"kind": {"enum": ["lease", "capacity"]}, "waitedSeconds": I()})}, optional=("phase", "done", "total", "etaSeconds", "priority", "waiting"))
worker = obj({"origin": S, "liveness": {"enum": ["absent", "offline", "silent", "working", "ready", "idle"]}, "stage": {"type": ["string", "null"]}, "stageIndex": I(), "stageCount": I(), "attempts": I(), "recordAgeSeconds": NI, "refusal": {"type": ["string", "null"]}, "tree": {"enum": ["ok", "off", "retired", "missing", "stale", "lagging", "diverged", None]}, "level": {"enum": ["calm", "guard", "danger", None]}, "availableMegabytes": NI, "swapUsedMegabytes": NI, "failed": NI})
machine = obj({"origin": S, "worker": {"enum": ["absent", "idle", "busy", "refusing"]}, "pressure": {"enum": ["normal", "warn", "critical"]}, "headroom": {"type": "boolean"}, "why": S, "code": {"type": "string", "pattern": "^[0-9a-f]{16}$"}}, optional=("why", "code"))
fleet_props = copy.deepcopy(D["FleetSection"]["properties"])
fleet = obj({"machines": I(), "queued": I(), "executing": I(), "offloadedIn": I(), "offloadedOut": I(), "refused": I(), "declined": I(), "retried": I(), "cacheHit": I(), "cacheMiss": I(), "treeCode": fleet_props["treeCode"], "cache": fleet_props["cache"], "claims": fleet_props["claims"],
    "keys": obj({"origins": I(), "units": I(), "differing": I(), "dependent": I()}), "attention": arr(machine)}, optional=("treeCode", "cache", "claims", "keys"))
offload = obj({"local": I(), "fleet": I(), "out": I(), "refused": I(), "fellBack": I(), "deadline": I(), "lost": I(), "silent": I(), "reasons": {"type": "object", "additionalProperties": I(1)}, "spool": D["OffloadSection"]["properties"]["spool"]}, optional=("spool",))
rates = obj({"windowsSeconds": {"type": "array", "minItems": 1, "items": I(1)}, "fleet": {"type": "object", "additionalProperties": rows()}, "origins": {"type": "object", "additionalProperties": {"type": "object", "additionalProperties": rows()}}, "omitted": I()})
finding = obj({"id": S, "severity": {"enum": ["alert", "warn", "info"]}, "count": I(1), "subjects": arr(S), "more": I(), "text": S})
sources = obj({"count": I(), "milliseconds": I(), "failed": arr(obj({"id": D["SourceFact"]["properties"]["id"], "error": S}, optional=("error",)))})
summary = obj({
    "version": {"const": 1}, "view": {"const": "summary"}, "atMs": I(), "workspace": S, "summary": ref("summary"), "memory": memory,
    "runs": obj({"count": I(), "rows": arr(run)}),
    "queues": obj({"graph": obj({"builder": builder, "computers": I(), "slots": I(), "waiting": I(), "longestWaitSeconds": I()}), "capacity": obj({"grants": I(), "waiters": I(), "floorMegabytes": I(), "spareMegabytes": {"type": "integer"}}), "leases": arr(obj({"resource": S, "class": S, "holders": I(), "waiters": I(), "longestWaitSeconds": I()})), "leasesOmitted": I()}),
    "bazel": obj({"slots": I(), "live": I(), "held": I(), "rssMegabytes": I()}),
    "storage": obj({"hostFreeBytes": NI, "hostFreeFloorBytes": I(), "overBytes": I(), "fullImages": arr(obj({"id": S, "usedBasisPoints": I()}))}),
    "sealed": obj({"mode": {"enum": ["sealed", "live", "promote"]}, "entries": I(), "drifted": I(), "blocked": I()}),
    "generation": {"anyOf": [obj({"number": I(), "ageSeconds": NI, "owners": I(), "dirty": I(), "verifiedBasisPoints": NI}), {"type": "null"}]},
    "scheduler": obj({"running": I(), "queued": I(), "reservedMegabytes": I(), "budgetMegabytes": I(), "sharedTickets": I(), "longestWaitSeconds": I()}),
    "fleet": fleet,
    "workers": obj({"counts": D["WorkersSection"]["properties"]["counts"], "rows": arr(worker), "healthy": arr(S)}),
    "cas": ref("cas"), "retention": ref("retention"), "offload": offload, "rates": rates, "findings": arr(finding), "sources": sources,
}, optional=("scheduler", "fleet", "workers", "cas", "retention", "offload", "rates"))
summary["description"] = "The default view of `repo fleet status`, derived from the detail report alone: totals per window, per-origin counters only where anomalous, findings grouped by id with counts and subjects, workers that need a look as rows and the healthy ones as one list. Everything else is behind `--detail`, `--origin` and `--section`."
selection = obj({"version": {"const": 1}, "view": {"const": "detail"}, "atMs": I(), "workspace": S, "origin": S, "sections": {"type": "object", "additionalProperties": False, "properties": {name: ref(name) for name in SECTIONS}}}, optional=("origin",))
selection["description"] = "The sections of the detail report that `--section` and `--origin` selected, each exactly as in the full report; with `--origin` the workers, machines, counters and findings are those of that origin."

def emit(name, value):
    body = json.dumps({name: value}, indent=1, ensure_ascii=False).split("\n")[1:-1]
    return "\n".join(" " + line for line in body) + ",\n"
once('  "Corpus": {', emit("Summary", summary) + emit("Selection", selection) + '  "Corpus": {')

en = lambda e, d: {"en": e, "de": d}
msgs = {
 "error.syntax": en("Usage: repo fleet status [--json] [--detail] [--origin=<id>] [--section=<name>] [--limit=<n>]", "Verwendung: repo fleet status [--json] [--detail] [--origin=<id>] [--section=<name>] [--limit=<n>]"),
 "error.origin": en("Unknown origin {origin}; known: {list}", "Unbekannter Origin {origin}; bekannt: {list}"),
 "error.section": en("Unknown section {section}; known: {list}", "Unbekannter Abschnitt {section}; bekannt: {list}"),
 "summary.run": en("  {pid} {state} {who} {what}{progress} · {age}{waiting}", "  {pid} {state} {who} {what}{progress} · {age}{waiting}"),
 "summary.run.waiting": en(" · waits for {kind} {waited}", " · wartet auf {kind} {waited}"),
 "summary.queues": en("Queues: graph {graph} waiting (longest {longest}), builder {builder}; capacity {capacity} waiting, {spare} MB spare; {leases} lease queue(s)", "Warteschlangen: Graph {graph} wartend (längste {longest}), Builder {builder}; Kapazität {capacity} wartend, {spare} MB frei; {leases} Lease-Warteschlange(n)"),
 "summary.lease": en("  {resource}: {holders} holder(s), {waiters} waiting, longest {longest}", "  {resource}: {holders} Halter, {waiters} wartend, längste {longest}"),
 "summary.keys": en("  unit keys: {differing} of {units} compared unit(s) differ between {origins} origins, {dependent} on identical source", "  Einheitenschlüssel: {differing} von {units} verglichenen Einheiten weichen zwischen {origins} Origins ab, {dependent} bei gleicher Quelle"),
 "summary.machine": en("  {origin}: worker {worker}, pressure {pressure}{headroom}{why}{code}", "  {origin}: Worker {worker}, Druck {pressure}{headroom}{why}{code}"),
 "summary.worker": en("  {origin}: {state}, stage {stage} ({index}/{count}), attempt {attempts}, record {record}{detail}", "  {origin}: {state}, Stufe {stage} ({index}/{count}), Versuch {attempts}, Eintrag {record}{detail}"),
 "summary.tree": en("tree {verdict}", "Baum {verdict}"),
 "summary.level": en("memory {level}, {available} MB free, swap {swap} MB", "Speicher {level}, {available} MB frei, Auslagerung {swap} MB"),
 "summary.failed": en("{count} failed run(s) recently", "{count} fehlgeschlagene Läufe zuletzt"),
 "summary.healthy": en("  healthy: {list}", "  in Ordnung: {list}"),
 "summary.reasons": en("  declined: {list}", "  abgelehnt: {list}"),
 "summary.rate": en("  {label}: {values}", "  {label}: {values}"),
 "summary.rate.origin": en("  {origin}: {list}", "  {origin}: {list}"),
 "summary.rate.omitted": en("  … {count} more counter row(s); see --detail", "  … {count} weitere Zählerzeilen; siehe --detail"),
 "summary.storage": en("Storage: host free {free} GiB (floor {floor} GiB), {over} GiB over budget{images}", "Speicher auf Datenträger: Rechner frei {free} GiB (Untergrenze {floor} GiB), {over} GiB über dem Budget{images}"),
 "summary.generation": en("Generation {number}: {age} old, {owners} owner(s), {dirty} dirty, {percent} % verified", "Generation {number}: {age} alt, {owners} Eigentümer, {dirty} geändert, {percent} % geprüft"),
 "summary.finding": en("  [{severity}] ×{count} {text} ({subjects})", "  [{severity}] ×{count} {text} ({subjects})"),
 "summary.hint": en("Details: repo fleet status --detail [--origin=<id>] [--section=<name>]", "Einzelheiten: repo fleet status --detail [--origin=<id>] [--section=<name>]"),
}
old = text[text.index('    "error.syntax": {'):]
old = old[:old.index("    },\n") + len("    },\n")]
new = "".join('    ' + json.dumps(k, ensure_ascii=False) + ': ' + json.dumps(v, ensure_ascii=False) + ',\n' for k, v in msgs.items())
once(old, new)
json.loads(text)
tmp = path + ".h83.tmp"
open(tmp, "w", encoding="utf-8").write(text)
os.replace(tmp, path)
print("ok")
