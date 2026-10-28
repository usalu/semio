"""mkfixture.py <out.json> [<violations.json>]: writes the Nx-path acquisition sequences (declared from reading the code, validated against the real orchestrator order by the law) with the expected violations taken from <violations.json> (a previous model run, reviewed by hand) or empty."""
import json, sys

ID = "b" * 64
H = "c" * 64
NATIVE_TOOLS = "native-storage:[16777229,449367157,1609564160,16777272]"
NATIVE_NX = "native-storage:[16777229,449367157,1609564160,16777273]"
TOOLING, SDK = "nx-tooling:" + ID, "nx-sdk-snapshot:" + ID
FIXED = "nx:fixed-cache:/w/.cache/b/n/m"
POOL, SPACE = "nx:graph-pool:" + H, "nx:graph-namespace:" + H
SERVICE, START = "nx:daemon-service:" + H, "nx:daemon-startup:" + H
SLOT = "nx:daemon-slot:f331ddb9bfed1724:finite:0"
PIN = "build-graph:/w/.cache/b/g/m/snapshots/" + ID + ".json"
ARTIFACT = "artifact:/w/.cache/b/p/m/print/prototype/documents/print/paper"
GATE, PUT = "nx:put-gate:f331ddb9bfed1724", "nx:put-slot:f331ddb9bfed1724:3"
WAIT = 20000


def a(resource, mode, via, waited=None):
    row = {"op": "acquire", "resource": resource, "mode": mode, "via": via}
    if waited is not None:
        row["waitedMilliseconds"] = waited
    return row


def r(resource):
    return {"op": "release", "resource": resource}


def transient(resource, via, mode="shared", waited=WAIT):
    return [a(resource, mode, via, waited), r(resource)]


def sequence(id, note, steps, after):
    return {"id": id, "note": note, "steps": steps, "after": after}


storage_open = lambda via: transient(NATIVE_TOOLS, via)
bind = lambda via: transient(NATIVE_NX, via) + [a(POOL, "exclusive", via), r(POOL), a(SPACE, "shared", via)]
tool_use = lambda via: [a(TOOLING, "shared", via), a(SDK, "shared", via)]
ambient = [a(TOOLING, "shared", "ambient"), a(SDK, "shared", "ambient"), a(FIXED, "shared", "ambient")]

sequences = [
    sequence("client-run", "NxScript.runIssued: tools store, graph load, pin, provision, activation, tool use, then the namespace is bound inside the tool use", [
        *transient(NATIVE_TOOLS, "run"), a(PIN, "shared", "run"),
        a("nx-tooling-provision", "exclusive", "run"), r("nx-tooling-provision"), a("nx-tooling-activation", "exclusive", "run"),
        a("nx-authorizer-source:/w/x", "shared", "run"), a("nx-authorizer-compiler:/w/x", "shared", "run"), r("nx-authorizer-compiler:/w/x"), r("nx-authorizer-source:/w/x"), r("nx-tooling-activation"),
        *tool_use("run"), *bind("run")], True),
    sequence("client-start", "withNxDaemonStartupV1 from a cache hook (tool use and fixed cache already held): store, birth (images, tool use, namespace), then the start lease", [
        *ambient, *storage_open("store"), *transient(NATIVE_NX, "birth"), *tool_use("birth"), *bind("birth"), a(START, "exclusive", "lease")], True),
    sequence("client-start-lease-first", "the order before nxlock-g: the start lease is taken first and the birth waits for storage under it", [
        *ambient, *storage_open("store"), a(START, "exclusive", "lease"), *transient(NATIVE_NX, "birth"), *tool_use("birth"), *bind("birth")], False),
    sequence("daemon-service", "withNxDaemonServiceV1: slot, store, tool use, the namespace is bound inside it, then the exclusive service lease", [
        a(SLOT, "exclusive", "service"), *storage_open("service"), *transient(NATIVE_NX, "service"), *tool_use("service"), *bind("service"), a(SERVICE, "exclusive", "service")], True),
    sequence("daemon-service-bind-first", "the order before nxlock-g: the namespace is bound before the toolkit is used", [
        a(SLOT, "exclusive", "service"), *storage_open("service"), *transient(NATIVE_NX, "service"), *bind("service"), *tool_use("service"), a(SERVICE, "exclusive", "service")], False),
    sequence("journal-reader", "withNxDaemonJournalReaderV1: store, tool use, namespace bound inside it", [
        *storage_open("journal"), *transient(NATIVE_NX, "journal"), *tool_use("journal"), *bind("journal")], True),
    sequence("journal-reader-bind-first", "the order before nxlock-g", [
        *storage_open("journal"), *transient(NATIVE_NX, "journal"), *bind("journal"), *tool_use("journal")], False),
    sequence("cache-put", "a task publishes its outputs into the shared Nx image: toolkit use, fixed cache, artifact path, put gate, put slot", [
        *ambient, a(ARTIFACT, "exclusive", "put"), a(GATE, "exclusive", "put"), a(PUT, "exclusive", "put"), r(GATE)], True),
]

violations = json.load(open(sys.argv[2])) if len(sys.argv) > 2 else {}
for row in sequences:
    found = violations.get(row["id"], {"outOfOrder": [], "holdAndWait": []})
    row["outOfOrder"], row["holdAndWait"] = found["outOfOrder"], found["holdAndWait"]
line = lambda value: json.dumps(value, ensure_ascii=False)
with open(sys.argv[1], "w", encoding="utf8") as out:
    out.write('{\n  "directory": %s,\n  "sequences": [\n' % line("/w/.cache/b/l/m/data"))
    for index, row in enumerate(sequences):
        out.write('    {\n      "id": %s,\n      "note": %s,\n      "after": %s,\n      "steps": [\n' % (line(row["id"]), line(row["note"]), line(row["after"])))
        out.write(",\n".join("        " + line(step) for step in row["steps"]))
        out.write('\n      ],\n      "outOfOrder": %s,\n      "holdAndWait": %s\n    }%s\n' % (line(row["outOfOrder"]), line(row["holdAndWait"]), "," if index < len(sequences) - 1 else ""))
    out.write("  ]\n}\n")
