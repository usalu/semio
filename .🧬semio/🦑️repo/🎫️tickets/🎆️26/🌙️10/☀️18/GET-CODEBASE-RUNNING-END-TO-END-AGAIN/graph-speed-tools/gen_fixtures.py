#!/usr/bin/env python3
"""Generates every language-agnostic fixture of the graph-speed set from independent Python references (hashlib, a reference throttle model, a reference label renderer).

usage: gen_fixtures.py            (rewrites staged/new/*/fixture.json and the closed schemas)"""
import hashlib, json, os

HERE = os.path.dirname(os.path.abspath(__file__))
NEW = os.path.join(HERE, "staged", "new")


def write(relative, value):
    path = os.path.join(NEW, relative)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf8") as handle:
        json.dump(value, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def sha(text):
    return hashlib.sha256(text.encode("utf8")).hexdigest()


# --- phase clock: reference model of the throttle ---------------------------------------------------
def simulate(steps, minimum=250):
    phases, published, rows = {}, [], []
    for step in steps:
        now, op, phase = step["at"], step["op"], step["phase"]
        if op == "begin":
            phases[phase] = {"total": step["total"], "done": 0, "started": now, "last_at": None, "last_done": -1}
            state = phases[phase]
            published.append("build:graph-phase:%s:%d:%d" % (phase, 0, state["total"]))
            state["last_at"], state["last_done"] = now, 0
        elif op == "advance":
            state = phases[phase]
            state["done"] = min(step["done"], state["total"])
            if state["done"] == state["last_done"]:
                continue
            if state["done"] != state["total"] and state["last_at"] is not None and now - state["last_at"] < minimum:
                continue
            published.append("build:graph-phase:%s:%d:%d" % (phase, state["done"], state["total"]))
            state["last_at"], state["last_done"] = now, state["done"]
        else:
            state = phases.pop(phase)
            if state["last_done"] != state["done"]:
                published.append("build:graph-phase:%s:%d:%d" % (phase, state["done"], state["total"]))
            rows.append({"phase": phase, "milliseconds": int(now - state["started"] + 0.5), "done": state["done"], "total": state["total"]})
    return published, rows


def clock_case(name, steps, minimum=250):
    published, rows = simulate(steps, minimum)
    return {"name": name, "minimumMilliseconds": minimum, "steps": steps, "published": published, "rows": rows}


inference = [{"op": "begin", "phase": "inference", "total": 6, "at": 0}]
at = 0
for done in range(1, 7):
    at += 100
    inference.append({"op": "advance", "phase": "inference", "done": done, "at": at})
inference.append({"op": "end", "phase": "inference", "at": at + 5})
clock = {
    "version": 1,
    "cases": [
        clock_case("throttled-counter-publishes-first-last-and-every-250-ms", inference),
        clock_case("zero-total-publishes-once-and-times-the-phase", [
            {"op": "begin", "phase": "expansion", "total": 0, "at": 10},
            {"op": "end", "phase": "expansion", "at": 1010}]),
        clock_case("two-interleaved-phases-keep-independent-throttles", [
            {"op": "begin", "phase": "verification", "total": 3, "at": 0},
            {"op": "begin", "phase": "expansion", "total": 2, "at": 50},
            {"op": "advance", "phase": "verification", "done": 1, "at": 100},
            {"op": "advance", "phase": "expansion", "done": 1, "at": 120},
            {"op": "advance", "phase": "verification", "done": 2, "at": 400},
            {"op": "advance", "phase": "expansion", "done": 2, "at": 410},
            {"op": "advance", "phase": "verification", "done": 3, "at": 420},
            {"op": "end", "phase": "expansion", "at": 430},
            {"op": "end", "phase": "verification", "at": 440}]),
        clock_case("end-publishes-an-unpublished-final-state", [
            {"op": "begin", "phase": "inference", "total": 10, "at": 0},
            {"op": "advance", "phase": "inference", "done": 4, "at": 10},
            {"op": "end", "phase": "inference", "at": 20}]),
        clock_case("wide-window-publishes-every-advance", [
            {"op": "begin", "phase": "inference", "total": 3, "at": 0},
            {"op": "advance", "phase": "inference", "done": 1, "at": 1},
            {"op": "advance", "phase": "inference", "done": 2, "at": 2},
            {"op": "advance", "phase": "inference", "done": 3, "at": 3},
            {"op": "end", "phase": "inference", "at": 4}], minimum=0),
    ],
    "refusals": [
        {"name": "advance-before-begin", "steps": [{"op": "advance", "phase": "inference", "done": 1, "at": 0}]},
        {"name": "regressing-counter", "steps": [{"op": "begin", "phase": "inference", "total": 5, "at": 0}, {"op": "advance", "phase": "inference", "done": 3, "at": 1}, {"op": "advance", "phase": "inference", "done": 2, "at": 2}]},
        {"name": "begun-twice", "steps": [{"op": "begin", "phase": "inference", "total": 5, "at": 0}, {"op": "begin", "phase": "inference", "total": 5, "at": 1}]},
        {"name": "phase-name-with-separator", "steps": [{"op": "begin", "phase": "in:ference", "total": 5, "at": 0}]},
        {"name": "negative-total", "steps": [{"op": "begin", "phase": "inference", "total": -1, "at": 0}]},
    ],
}
write("phases/fixture.json", clock)

step = {"type": "object", "additionalProperties": False, "required": ["op", "phase", "at"], "properties": {
    "op": {"enum": ["begin", "advance", "end"]}, "phase": {"type": "string"}, "at": {"type": "number", "minimum": 0},
    "total": {"type": "integer"}, "done": {"type": "integer"}}}
write("phases/fixture-schema.json", {
    "type": "object", "additionalProperties": False, "required": ["version", "cases", "refusals"],
    "properties": {"version": {"const": 1}, "cases": {"type": "array", "minItems": 1, "items": {
        "type": "object", "additionalProperties": False, "required": ["name", "minimumMilliseconds", "steps", "published", "rows"],
        "properties": {"name": {"type": "string", "minLength": 1}, "minimumMilliseconds": {"type": "integer", "minimum": 0},
                       "steps": {"type": "array", "minItems": 1, "items": step}, "published": {"type": "array", "items": {"type": "string", "pattern": "^build:graph-phase:[a-z]+:(?:0|[1-9][0-9]*):(?:0|[1-9][0-9]*)$"}},
                       "rows": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["phase", "milliseconds", "done", "total"],
                                                           "properties": {"phase": {"type": "string"}, "milliseconds": {"type": "integer", "minimum": 0}, "done": {"type": "integer", "minimum": 0}, "total": {"type": "integer", "minimum": 0}}}}}}},
        "refusals": {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["name", "steps"], "properties": {"name": {"type": "string"}, "steps": {"type": "array", "items": step}}}}}})

# --- lean inventory leaves ------------------------------------------------------------------------------
good = json.dumps({"version": 1, "files": ["bunfig.toml", "tsconfig.json", "🧰️framework/📜️script.ts"]}, ensure_ascii=False, separators=(",", ":"))
other = json.dumps({"version": 1, "files": ["another.ts"]}, separators=(",", ":"))
lean = {
    "version": 1,
    "directories": ["build-input-closures", "build-lock-closures"],
    "maximumBytes": 4096,
    "records": {
        "good": {"text": good, "sha256": sha(good)},
        "other": {"text": other, "sha256": sha(other)},
        "notJson": {"text": "not json at all", "sha256": sha("not json at all")},
    },
    "cases": [
        {"name": "valid-leaf-reads-and-hashes", "record": "good", "fileKey": "good", "expect": "value"},
        {"name": "leaf-named-after-other-content-is-refused", "record": "other", "fileKey": "good", "expect": "content differs from its name"},
        {"name": "empty-leaf-is-refused", "record": "empty", "fileKey": "good", "expect": "Single-link bounded regular"},
        {"name": "leaf-over-the-bound-is-refused", "record": "good", "fileKey": "good", "maximumBytes": 8, "expect": "Single-link bounded regular"},
        {"name": "non-json-leaf-with-matching-name-fails-parsing", "record": "notJson", "fileKey": "notJson", "expect": "JSON"},
        {"name": "missing-leaf-is-refused", "record": None, "fileKey": "good", "expect": "ENOENT"},
        {"name": "symbolic-link-is-refused", "record": "good", "fileKey": "good", "link": "symbolic", "expect": "Single-link bounded regular"},
        {"name": "second-hard-link-is-refused", "record": "good", "fileKey": "good", "link": "hard", "expect": "Single-link bounded regular"},
    ],
    "refusals": [
        {"name": "unknown-directory", "directory": "somewhere-else", "key": sha(good), "expect": "Closed inventory leaf identity required"},
        {"name": "short-key", "directory": "build-input-closures", "key": "abc", "expect": "Closed inventory leaf identity required"},
        {"name": "uppercase-key", "directory": "build-input-closures", "key": sha(good).upper(), "expect": "Closed inventory leaf identity required"},
        {"name": "zero-bound", "directory": "build-input-closures", "key": sha(good), "maximumBytes": 0, "expect": "Closed inventory leaf identity required"},
    ],
}
write("lean/fixture.json", lean)
write("lean/fixture-schema.json", {
    "type": "object", "additionalProperties": False,
    "required": ["version", "directories", "maximumBytes", "records", "cases", "refusals"],
    "properties": {"version": {"const": 1}, "directories": {"type": "array", "items": {"type": "string"}}, "maximumBytes": {"type": "integer", "minimum": 1},
                   "records": {"type": "object", "additionalProperties": {"type": "object", "additionalProperties": False, "required": ["text", "sha256"], "properties": {"text": {"type": "string"}, "sha256": {"type": "string", "pattern": "^[a-f0-9]{64}$"}}}},
                   "cases": {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["name", "record", "fileKey", "expect"], "properties": {
                       "name": {"type": "string"}, "record": {"type": ["string", "null"]}, "fileKey": {"type": "string"}, "expect": {"type": "string"}, "maximumBytes": {"type": "integer"}, "link": {"enum": ["symbolic", "hard"]}}}},
                   "refusals": {"type": "array", "minItems": 1, "items": {"type": "object", "additionalProperties": False, "required": ["name", "directory", "key", "expect"], "properties": {
                       "name": {"type": "string"}, "directory": {"type": "string"}, "key": {"type": "string"}, "expect": {"type": "string"}, "maximumBytes": {"type": "integer"}}}}}})

# --- record store descriptors -------------------------------------------------------------------------------
def record_key(namespace, *inputs):
    h = hashlib.sha256()
    for part in [namespace, *inputs]:
        data = part.encode("utf8")
        h.update(("%d:" % len(data)).encode("ascii"))
        h.update(data)
    return h.hexdigest()


records = []
for index in range(96):
    text = "semio-record-%04d-" % index + "x" * (index % 23) + "🧬️" * (index % 3)
    records.append({"key": record_key("descriptor-limit", str(index)), "text": text, "digest": hashlib.sha256(text.encode("utf8")).hexdigest()[:32]})
write("records/fixture.json", {"version": 1, "namespace": "descriptor-limit", "limit": 32, "packs": 96, "records": records})
write("records/fixture-schema.json", {
    "type": "object", "additionalProperties": False, "required": ["version", "namespace", "limit", "packs", "records"],
    "properties": {"version": {"const": 1}, "namespace": {"type": "string"}, "limit": {"const": 32}, "packs": {"type": "integer", "minimum": 33},
                   "records": {"type": "array", "minItems": 33, "items": {"type": "object", "additionalProperties": False, "required": ["key", "text", "digest"], "properties": {
                       "key": {"type": "string", "pattern": "^[a-f0-9]{64}$"}, "text": {"type": "string"}, "digest": {"type": "string", "pattern": "^[a-f0-9]{32}$"}}}}}})

# --- progress labels ------------------------------------------------------------------------------------------------
phases = {
    "en": {"template": "{command} · {phase}: {done} of {total} {units}", "phases": {
        "inference": {"label": "Inferring projects", "units": "projects"},
        "verification": {"label": "Verifying input closures", "units": "closures"},
        "expansion": {"label": "Expanding input closures", "units": "closures"}}},
    "de": {"template": "{command} · {phase}: {done} von {total} {units}", "phases": {
        "inference": {"label": "Projekte ableiten", "units": "Projekte"},
        "verification": {"label": "Eingabehüllen prüfen", "units": "Hüllen"},
        "expansion": {"label": "Eingabehüllen auflösen", "units": "Hüllen"}}},
}


def render(language, command, phase, done, total):
    labels = phases[language]
    row = labels["phases"][phase]
    return labels["template"].replace("{command}", command).replace("{phase}", row["label"]).replace("{done}", str(done)).replace("{total}", str(total)).replace("{units}", row["units"])


cases = []
for language, lang in (("en", "en_US.UTF-8"), ("de", "de_DE.UTF-8")):
    for phase, done, total in (("inference", 120, 3656), ("verification", 0, 17713), ("expansion", 14232, 14232)):
        cases.append({"name": "%s-%s-%d-of-%d" % (language, phase, done, total), "environment": {"LANG": lang}, "command": "build:projects",
                      "resource": "build:graph-phase:%s:%d:%d" % (phase, done, total), "mode": "shared", "elapsedMs": 0, "expected": render(language, "build:projects", phase, done, total)})
rejected = [
    ("done-above-total", "build:graph-phase:inference:5:4", 0, "shared"),
    ("unknown-phase", "build:graph-phase:publishing:1:2", 0, "shared"),
    ("missing-total", "build:graph-phase:inference:1", 0, "shared"),
    ("extra-part", "build:graph-phase:inference:1:2:3", 0, "shared"),
    ("leading-zero", "build:graph-phase:inference:01:2", 0, "shared"),
    ("non-zero-elapsed", "build:graph-phase:inference:1:2", 5, "shared"),
    ("exclusive-mode", "build:graph-phase:inference:1:2", 0, "exclusive"),
    ("overflowing-count", "build:graph-phase:inference:1:99999999999999999999", 0, "shared"),
]
write("progress/fixture.json", {
    "version": 1, "labels": phases, "cases": cases,
    "rejected": [{"name": name, "environment": {"LANG": "en_US.UTF-8"}, "command": "build:projects", "resource": resource, "mode": mode, "elapsedMs": elapsed} for name, resource, elapsed, mode in rejected]})
base_case = {"type": "object", "additionalProperties": False, "required": ["name", "environment", "command", "resource", "mode", "elapsedMs"], "properties": {
    "name": {"type": "string"}, "environment": {"type": "object", "additionalProperties": {"type": "string"}}, "command": {"type": "string"}, "resource": {"type": "string"},
    "mode": {"enum": ["shared", "exclusive"]}, "elapsedMs": {"type": "integer", "minimum": 0}, "expected": {"type": "string"}}}
write("progress/fixture-schema.json", {
    "type": "object", "additionalProperties": False, "required": ["version", "labels", "cases", "rejected"],
    "properties": {"version": {"const": 1}, "labels": {"type": "object"}, "cases": {"type": "array", "minItems": 1, "items": {**base_case, "required": base_case["required"] + ["expected"]}}, "rejected": {"type": "array", "minItems": 1, "items": base_case}}})
write("progress/labels.json", phases)

# --- sealed reuse structure -------------------------------------------------------------------------------------------------------
write("sealed/fixture.json", {
    "version": 1,
    "functions": {
        "loadBuildGraphV1": {"start": "export async function loadBuildGraphV1(", "end": "/** 🎚️ Resolves one configuration"},
        "readSealedBuildGraphV1": {"start": "async function readSealedBuildGraphV1(", "end": "/** 💾️ Publishes only a current graph"},
        "loadBuildGraphAttemptV1": {"start": "async function loadBuildGraphAttemptV1(", "end": "async function readSealedBuildGraphV1("},
    },
    "order": [
        {"name": "a shared reader runs before the exclusive writer lease is requested", "function": "loadBuildGraphV1", "before": "readSealedBuildGraphV1(root,invocation,phaseControl,options,registry)", "after": "withBuildGraphStateV1(root,invocation,phaseControl,async(store,actualControl)=>{"},
        {"name": "the sealed-only attempt leaves before any inference starts", "function": "loadBuildGraphAttemptV1", "before": "if(options.sealedOnly)return undefined;\n  let captured", "after": "discoverBuildGraphV1(root,options.selection"},
        {"name": "the early receipt probe leaves before the workspace inventory is read", "function": "loadBuildGraphAttemptV1", "before": "if(options.sealedOnly){", "after": "const inventory=async()=>"},
        {"name": "the graph is published as its compact form", "function": "loadBuildGraphAttemptV1", "before": "source=JSON.stringify(compact)", "after": "await rename(temporary,path)"},
        {"name": "the caller receives the expanded graph that the compact publication was written from", "function": "loadBuildGraphAttemptV1", "before": "readBuildGraphV1(await portableInputs(root,compact,invocation,phaseControl))", "after": "await rename(temporary,path)"},
    ],
    "present": [
        {"name": "the reader uses the shared cache store, not the graph writer", "function": "readSealedBuildGraphV1", "text": "withRepositoryCacheStoreV1(root,\"graph\",invocation,phaseControl"},
        {"name": "the reader is sealed-only", "function": "readSealedBuildGraphV1", "text": "sealedOnly:true"},
        {"name": "only the all-graph skips the writer", "function": "loadBuildGraphV1", "text": "if(scope===undefined){"},
        {"name": "the shared attempt is cancellable", "function": "loadBuildGraphV1", "text": "checkScriptInvocation(invocation);return sealed;"},
    ],
    "absent": [
        {"name": "the reader never takes the graph writer", "function": "readSealedBuildGraphV1", "text": "withBuildGraphStateV1("},
        {"name": "the reader never takes the facts writer", "function": "readSealedBuildGraphV1", "text": "withRepositoryBuildGraphFactStoreV1("},
        {"name": "the graph is not published expanded", "function": "loadBuildGraphAttemptV1", "text": "source=JSON.stringify(graph)"},
    ],
})
item = {"type": "object", "additionalProperties": False, "required": ["name", "function"], "properties": {"name": {"type": "string"}, "function": {"type": "string"}, "before": {"type": "string"}, "after": {"type": "string"}, "text": {"type": "string"}}}
write("sealed/fixture-schema.json", {
    "type": "object", "additionalProperties": False, "required": ["version", "functions", "order", "present", "absent"],
    "properties": {"version": {"const": 1}, "functions": {"type": "object", "additionalProperties": {"type": "object", "additionalProperties": False, "required": ["start", "end"], "properties": {"start": {"type": "string"}, "end": {"type": "string"}}}},
                   "order": {"type": "array", "minItems": 1, "items": {**item, "required": ["name", "function", "before", "after"]}},
                   "present": {"type": "array", "items": {**item, "required": ["name", "function", "text"]}}, "absent": {"type": "array", "items": {**item, "required": ["name", "function", "text"]}}}})

# --- project file owner index -----------------------------------------------------------------------------------------------------------
import random


def owner_scan(projects, file):
    ordered = sorted(projects, key=lambda project: -len(project["root"]))
    for project in ordered:
        if project["root"] == "." or file.startswith(project["root"] + "/"):
            return project["name"]
    return None


def world(name, seed, with_root, count, depth):
    rng = random.Random(seed)
    names = ["alpha", "beta", "gamma", "delta", "🧰️framework", "📦️pkg", "x", "y"]
    roots = set()
    while len(roots) < count:
        roots.add("/".join(rng.choice(names) for _ in range(rng.randint(1, depth))))
    projects = [{"name": "p%d" % index, "root": root} for index, root in enumerate(sorted(roots, key=lambda r: rng.random()))]
    if with_root:
        projects.insert(rng.randint(0, len(projects)), {"name": "root", "root": "."})
    files = []
    for _ in range(400):
        files.append("/".join(rng.choice(names + ["src", "lib"]) for _ in range(rng.randint(0, depth + 2))) + ("/" if False else "") + ("/" if rng.random() < 0.0 else "") + "f%d.ts" % rng.randint(0, 9))
    files = [file.lstrip("/") for file in files]
    return {"name": name, "projects": projects, "files": files, "owners": [owner_scan(projects, file) for file in files]}


write("owner/fixture.json", {"version": 1, "worlds": [world("nested-without-root", 1, False, 30, 4), world("nested-with-root", 2, True, 30, 4), world("flat-one-letter-roots", 3, True, 6, 1), world("deep", 4, False, 60, 6)]})
write("owner/fixture-schema.json", {"type": "object", "additionalProperties": False, "required": ["version", "worlds"], "properties": {"version": {"const": 1}, "worlds": {"type": "array", "minItems": 1, "items": {
    "type": "object", "additionalProperties": False, "required": ["name", "projects", "files", "owners"], "properties": {
        "name": {"type": "string"}, "projects": {"type": "array", "items": {"type": "object", "additionalProperties": False, "required": ["name", "root"], "properties": {"name": {"type": "string"}, "root": {"type": "string", "minLength": 1}}}},
        "files": {"type": "array", "items": {"type": "string", "minLength": 1}}, "owners": {"type": "array", "items": {"type": ["string", "null"]}}}}}}})

print("fixtures written")
