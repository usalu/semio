"""Patches the bench schema (`🧬️schema/🔣️.json`) in a staged copy: product metrics, peer probes, phase/graph/action patterns, history names, labels.
usage: stage-schema.py <bench dir> <labels-out.json>
"""
import json, sys, os

root = sys.argv[1]
path = os.path.join(root, "🧬️schema", "🔣️.json")
s = json.load(open(path, encoding="utf8"))
d = s["definitions"]
P = d["Policy"]["const"]

EXTRA = ["queueMs", "graphMs", "executeMs", "executedActions", "inferredProjects"]
P["kinds"] = ["cli-help", "noop", "edit", "peer", "graph", "cache", "concurrency"]
P["order"] = list(P["kinds"])
P["metrics"] = {
    "cli-help": {"plain": ["wallMs"], "memory": []},
    "noop": {"plain": ["wallMs"] + EXTRA, "memory": ["peakRssBytes"]},
    "edit": {"plain": ["startMs", "greenMs"] + EXTRA, "memory": ["peakRssBytes"]},
    "peer": {"plain": ["wallMs"] + EXTRA, "memory": ["peakRssBytes"]},
    "graph": {"plain": ["wallMs"], "memory": []},
    "cache": {"plain": ["hitRatio"], "memory": []},
    "concurrency": {"plain": ["makespanMs", "maxUnrelatedWaitMs", "speedup"], "memory": []},
}
P["units"].update({"queueMs": "ms", "graphMs": "ms", "executeMs": "ms", "executedActions": "count", "inferredProjects": "count"})
P["sources"] = P["sources"] + ["product-budget"]
P["maximumObservedLineCharacters"] = 4096
P["phases"] = {
    "initial": "boot",
    "rules": [
        {"id": "queue-capacity", "patterns": ["^\\[capacity\\] waiting"]},
        {"id": "queue-graph", "patterns": ["Waiting for access to", "lease-queue", "^graph: waiting for the graph build"]},
        {"id": "verify", "patterns": ["generation-verify", "Verifying graph facts", "resource-directory-admission", "lease-sweep", "boot-recovery", "repository-cache-charge"]},
        {"id": "admission", "patterns": ["build:source-admission"]},
        {"id": "graph", "patterns": ["build:(graph-plan|discover|projects|graph-phase)", "^graph: ", "Assembling the graph", "Inferring projects"]},
        {"id": "tool-setup", "patterns": ["^\\[bazel\\] ", "^Analyzing", "^Loading", "^Computing main repo", "^INFO: (Invocation ID|Analyzed)"]},
        {"id": "execute", "patterns": ["^\\[\\d+ / \\d+\\]", "^> nx run ", "^INFO: From ", "^\\$ nx "]},
        {"id": "finish", "patterns": ["^INFO: (Elapsed time|Build completed)", "Build Event Protocol", "^Successfully ran", "^NX +Successfully"]},
    ],
    "metrics": {
        "queueMs": ["queue-capacity", "queue-graph"],
        "graphMs": ["verify", "admission", "graph"],
        "executeMs": ["tool-setup", "execute", "finish"],
    },
}
P["observation"] = {
    "graphRefreshing": "^graph: refreshing (\\d+) projects",
    "graphInferring": "Inferring projects: \\d+ of (\\d+) projects",
    "bazelProcesses": "^INFO: (\\d+) processes?: (.*?)\\.?$",
    "bazelIdle": ["^internal$", "cache hit$"],
    "nxTask": "^> nx run ",
    "nxCached": "^> nx run .*\\[(local cache|remote cache|existing outputs match the cache)",
}
P["history"] = {"name": "^([0-9]{8}T[0-9]{6}Z)-(.+)\\.json$", "prefix": "history:"}

kinds = {"enum": P["kinds"]}
d["Kind"] = kinds
metrics = ["wallMs", "startMs", "greenMs", "peakRssBytes", "hitRatio", "makespanMs", "maxUnrelatedWaitMs", "speedup"] + EXTRA
d["Metric"] = {"enum": metrics}
d["Unit"] = {"enum": ["ms", "bytes", "ratio", "count"]}
d["Source"] = {"enum": P["sources"]}
d["Values"]["properties"].update({m: {"type": "number", "minimum": 0} for m in EXTRA})
d["StatsMap"]["properties"].update({m: {"$ref": "#/definitions/Stats"} for m in EXTRA})

labels = {
    "queueMs": ("time in queues", "Zeit in Warteschlangen"),
    "graphMs": ("graph preparation", "Graph-Vorbereitung"),
    "executeMs": ("tool execution", "Werkzeugausführung"),
    "executedActions": ("re-executed actions", "neu ausgeführte Aktionen"),
    "inferredProjects": ("re-inferred projects", "neu abgeleitete Projekte"),
    "count": ("count", "Anzahl"),
    "peer": ("peer edit", "Fremdänderung"),
    "overBudget": ("Over budget", "Über dem Budget"),
    "budget": ("Budget", "Budget"),
    "history": ("History", "Verlauf"),
}
L = d["Labels"]
for key, (en, de) in labels.items():
    if key not in L["required"]:
        L["required"].append(key)
    L["properties"][key] = {"$ref": "#/definitions/LocalizedText"}

F = d["Fixture"]
F["required"] = F["required"] + ["observations", "history"]
F["properties"]["observations"] = {
    "type": "array",
    "items": {
        "type": "object", "additionalProperties": False,
        "required": ["name", "lines", "wallMs", "phases", "executed", "inferred"],
        "properties": {
            "name": {"type": "string", "minLength": 1},
            "lines": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["at", "line"], "properties": {"at": {"type": "number", "minimum": 0}, "line": {"type": "string"}}}},
            "wallMs": {"type": "number", "minimum": 0},
            "phases": {"type": "object", "additionalProperties": {"type": "number", "minimum": 0}},
            "executed": {"oneOf": [{"type": "integer", "minimum": 0}, {"type": "null"}]},
            "inferred": {"type": "integer", "minimum": 0},
        },
    },
}
F["properties"]["history"] = {
    "type": "array",
    "items": {
        "type": "object", "additionalProperties": False,
        "required": ["name", "startedAt", "hostClass", "fileName", "files", "before", "latest"],
        "properties": {
            "name": {"type": "string", "minLength": 1},
            "startedAt": {"type": "string", "minLength": 1},
            "hostClass": {"type": "string", "minLength": 1},
            "fileName": {"type": "string", "minLength": 1},
            "files": {"type": "array", "items": {"type": "string"}},
            "before": {"oneOf": [{"type": "string"}, {"type": "null"}]},
            "latest": {"oneOf": [{"type": "string"}, {"type": "null"}]},
        },
    },
}
original = open(path, encoding="utf8").read()
open(path, "w", encoding="utf8").write(json.dumps(s, indent=2, ensure_ascii=False) + ("\n" if original.endswith("\n") else ""))
json.dump(labels, open(sys.argv[2], "w", encoding="utf8"), indent=1, ensure_ascii=False)
print("ok")
