#!/usr/bin/env python3
"""Writes the language-agnostic doctor fixture: facts documents with the findings, wait chains and process phases a person worked out by hand from the policy thresholds (nothing here is computed by the implementation). The TypeScript analysis and the independent Python oracle must both reproduce every case.

usage: gen-fixtures.py [OUTPUT]"""
import hashlib
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
NOW = 1792545600000
GIB = 1073741824
CASES = []


def digest(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def facts(**sections):
    return {"version": 1, "atMs": NOW, "platform": "darwin", "probes": [], **sections}


def host(**o):
    base = {"cores": 10, "totalMegabytes": 32768, "availableMegabytes": 20000, "floorMegabytes": 6553, "pressure": "normal", "swapUsedMegabytes": 0, "swapOutMegabytesPerSecond": 0, "load": 2.0, "openFiles": 8000, "openFilesMaximum": 245760, "attachedImages": 10, "governor": "on"}
    base.update(o)
    return base


def part(identifier, **o):
    base = {"id": identifier, "state": "mounted", "capacityBytes": 8589934592, "freeBytes": 4294967296, "imageBytes": 8589934592, "backing": "present", "publication": "none", "publicationAgeSeconds": None, "image": "ok"}
    base.update(o)
    return base


def storage(partitions=(), free=100 * GIB, total=500 * GIB):
    return {"hostFreeBytes": free, "hostTotalBytes": total, "partitions": list(partitions)}


def holder(pid, resource, held, mode="exclusive", cls="cargo-coordination", owner="cargo", command="bun /w/script.ts repo run a"):
    return {"resource": resource, "hash": digest(resource), "mode": mode, "pid": pid, "heldSeconds": held, "class": cls, "owner": owner, "command": command}


def waiter(pid, resource, waited, blockers, mode="exclusive", position=None, queued=None, cls="cargo-coordination", owner="cargo", command="bun /w/script.ts repo run b"):
    return {"resource": resource, "hash": digest(resource), "mode": mode, "pid": pid, "waitedSeconds": waited, "class": cls, "owner": owner, "position": position, "queued": queued, "blockers": blockers, "command": command}


def leases(holders=(), waiters=(), cycles=(), violations=(), folders=0, orphans=0, dead=0, live=0, tickets=0, dead_holders=()):
    return {"holders": list(holders), "waiters": list(waiters), "cycles": list(cycles), "violations": list(violations), "registry": {"folders": folders, "orphanFolders": orphans, "deadEntries": dead, "liveEntries": live}, "deadTickets": tickets, "deadHolders": list(dead_holders)}


def grant(operation, pid, reserved, observed, held, jobs=1):
    return {"operation": operation, "pid": pid, "jobs": jobs, "reservedMegabytes": reserved, "observedMegabytes": observed, "heldSeconds": held}


def cwait(operation, pid, waited, reason="memory", need=4000):
    return {"operation": operation, "pid": pid, "needMegabytes": need, "waitedSeconds": waited, "reason": reason}


def capacity(grants=(), waiters=(), headroom=3000):
    return {"headroomMegabytes": headroom, "grants": list(grants), "waiters": list(waiters)}


def proc(pid, parent, age, command, cpu=0.0, rss=10):
    return {"pid": pid, "parent": parent, "ageSeconds": age, "rssMegabytes": rss, "cpuPercent": cpu, "command": command}


def graph(pointer="present", generation=3, manifest="present", owners=3700, age=100, newer=0):
    return {"pointer": pointer, "generation": generation, "manifest": manifest, "owners": owners, "publishedAgeSeconds": age, "implementationNewerSeconds": newer}


def scope(name, directory="ready", newer=None):
    return {"scope": name, "generation": digest(name), "directory": directory, "inputsNewer": newer}


def dependencies(scopes=(), pointers="present", orphans=0, unready=0, oldest=None):
    return {"pointers": pointers, "scopes": list(scopes), "orphans": orphans, "orphanEstimatedBytes": orphans * 80000000, "unready": unready, "unreadyOldestSeconds": oldest}


def caches(dead=0, bytes_=0, retired=(), undeclared=()):
    return {"deadPublications": dead, "deadPublicationBytes": bytes_, "deadPublicationPartial": False, "retired": list(retired), "undeclared": list(undeclared)}


def case(name, facts_, findings, waits=(), processes=(), ordered=False):
    counts = {"error": 0, "warn": 0, "info": 0}
    for row in findings:
        counts[row[1]] += 1
    verdict = "blocked" if counts["error"] else "degraded" if counts["warn"] else "ok"
    CASES.append({"name": name, "ordered": ordered, "facts": facts_, "expect": {"findings": [list(row) for row in findings], "waits": [list(row) for row in waits], "processes": [list(row) for row in processes], "verdict": verdict, "counts": counts}})


tools_ok = {"rows": [{"id": "repo", "state": "ok", "pin": "1.0.0", "found": "1.0.0"}, {"id": "git", "state": "ok", "pin": None, "found": "2.50.0"}]}
healthy_sections = dict(host=host(), storage=storage([part("graph", freeBytes=2 * GIB, capacityBytes=4 * GIB), part("dependencies")]), tools=tools_ok, leases=leases(), capacity=capacity(), sessions={"live": 1, "idle": 0, "gone": 0, "damaged": 0}, graph=graph(), dependencies=dependencies([scope(".")]), caches=caches(), processes=[])

case("healthy machine reports nothing", facts(**healthy_sections), [])

case("memory pressure warn and available just under the headroom", facts(host=host(pressure="warn", availableMegabytes=8191)), [("host.memory-pressure", "warn", "memory"), ("host.low-memory", "warn", "memory")])
case("critical pressure and available at the floor", facts(host=host(pressure="critical", availableMegabytes=6553)), [("host.memory-pressure", "error", "memory"), ("host.low-memory", "error", "memory")])
case("host values just below every threshold stay quiet", facts(host=host(availableMegabytes=8192, swapUsedMegabytes=4095, swapOutMegabytesPerSecond=63.9, load=19.99, openFiles=196607, attachedImages=27)), [])
case("host warnings at their thresholds", facts(host=host(swapUsedMegabytes=4096, load=20.0, openFiles=196608, attachedImages=28, governor="off")), [("host.swap-high", "warn", "swap"), ("host.load-high", "warn", "load"), ("host.open-files-high", "warn", "files"), ("host.images-many", "warn", "images"), ("capacity.governor-off", "warn", "SEMIO_HOST_CAPACITY")])
case("host errors and the swap rate", facts(host=host(swapOutMegabytesPerSecond=64, load=40.0, attachedImages=32)), [("host.swap-high", "warn", "swap"), ("host.load-high", "error", "load"), ("host.images-many", "error", "images")])
case("unknown host counters never report", facts(host=host(swapUsedMegabytes=None, swapOutMegabytesPerSecond=None, load=None, openFiles=None, openFilesMaximum=None, attachedImages=None)), [])

case("cache disk below the error threshold", facts(storage=storage(free=5368709119)), [("storage.host-free-low", "error", "cache-disk")])
case("cache disk exactly at the error threshold is a warning", facts(storage=storage(free=5368709120)), [("storage.host-free-low", "warn", "cache-disk")])
case("cache disk at the warning threshold is fine", facts(storage=storage(free=21474836480)), [])
case(
    "image states, floors, percentages and publications",
    facts(storage=storage([
        part("graph", capacityBytes=4294967296, freeBytes=1073741823),
        part("dependencies", freeBytes=429496729),
        part("products", capacityBytes=25769803776, freeBytes=2576980377),
        part("tools", freeBytes=1288490189),
        part("nx", state="detached", capacityBytes=None, freeBytes=None),
        part("leases", state="absent", capacityBytes=None, freeBytes=None, backing="none", image="damaged"),
        part("rust", state="absent", capacityBytes=None, freeBytes=None, backing="missing"),
        part("diagnostics", publication="intent", publicationAgeSeconds=599),
        part("scratch", publication="receipt", publicationAgeSeconds=600),
    ])),
    [("storage.backing-missing", "error", "rust"), ("storage.image-below-floor", "error", "graph"), ("storage.image-damaged", "error", "leases"), ("storage.image-low", "error", "dependencies"), ("storage.publication-pending", "error", "scratch"), ("storage.image-low", "warn", "products"), ("storage.image-detached", "info", "nx"), ("storage.publication-pending", "info", "diagnostics")],
    ordered=True,
)
case("graph image exactly at its floor is fine", facts(storage=storage([part("graph", capacityBytes=4294967296, freeBytes=1073741824)])), [])
case(
    "tools: essential missing, optional missing, drift, unprobed",
    facts(tools={"rows": [
        {"id": "repo", "state": "missing", "pin": None, "found": None},
        {"id": "git", "state": "missing", "pin": None, "found": None},
        {"id": "bazel", "state": "missing", "pin": "9.1.0", "found": None},
        {"id": "bun", "state": "drift", "pin": "1.4.2", "found": "1.3.14"},
        {"id": "node", "state": "unprobed", "pin": None, "found": None},
        {"id": "go", "state": "ok", "pin": "1.26.9", "found": "1.26.9"},
    ]}),
    [("tools.missing-essential", "error", "repo"), ("tools.missing-essential", "error", "git"), ("tools.missing", "warn", "bazel"), ("tools.pin-drift", "warn", "bun")],
)

case(
    "deadlock cycle, order violations, registry noise and dead tickets",
    facts(leases=leases(cycles=[{"members": ["100/0", "200/0"], "victim": "200/0"}], violations=[{"kind": "out-of-order", "held": "a", "requested": "b"}, {"kind": "out-of-order", "held": "c", "requested": "d"}, {"kind": "hold-and-wait", "held": "e", "requested": "f"}], folders=60, orphans=49, dead=3, tickets=2)),
    [("leases.cycle", "error", "100/0 → 200/0"), ("leases.order-violation", "warn", "hold-and-wait"), ("leases.order-violation", "info", "out-of-order"), ("leases.registry-stale", "info", "lease-registry"), ("leases.dead-ticket", "info", "queue")],
)
case("registry noise at the warning count", facts(leases=leases(folders=60, orphans=50, dead=0)), [("leases.registry-stale", "warn", "lease-registry")])
R = "cargo:registry:/w/cargo/home"
case(
    "lease waits at their thresholds with one holder",
    facts(leases=leases([holder(10, R, 100)], [waiter(20, R, 59, [10]), waiter(21, R, 60, [10]), waiter(22, R, 300, [10])]), processes=[proc(10, 1000, 100, "bun /w/script.ts repo cargo test")]),
    [("leases.wait-long", "error", R), ("leases.wait-long", "warn", R)],
    waits=[("lease", 22, 10, "running"), ("lease", 21, 10, "running"), ("lease", 20, 10, "running")],
    processes=[(10, "holding", None)],
)
case(
    "long exclusive holds",
    facts(leases=leases(
        [holder(31, "res-a", 900), holder(31, "res-a", 900), holder(32, "res-b", 1800), holder(33, "res-c", 1800), holder(34, "res-d", 1799), holder(35, "res-e", 5000, mode="shared")],
        [waiter(41, "res-a", 10, [31]), waiter(42, "res-b", 10, [32])],
    )),
    [("leases.hold-long", "error", "res-b"), ("leases.hold-long", "warn", "res-a"), ("leases.hold-long", "warn", "res-c")],
    waits=[("lease", 41, 31, "unknown"), ("lease", 42, 32, "unknown")],
)
Q = "nx:graph-pool:/w"
case("a deep queue on one lease", facts(leases=leases([holder(50, Q, 10, cls="graph", owner="graph")], [waiter(60 + n, Q, 10, [50], queued=5, cls="graph", owner="graph") for n in range(5)])), [("leases.queue-deep", "warn", Q)], waits=[("lease", 60 + n, 50, "unknown") for n in range(5)])
case("a queue depth reported by one waiter counts", facts(leases=leases([holder(50, Q, 10)], [waiter(60, Q, 10, [50], queued=5), waiter(61, Q, 10, [50], queued=2)])), [("leases.queue-deep", "warn", Q)], waits=[("lease", 60, 50, "unknown"), ("lease", 61, 50, "unknown")])
X, Y = "res-x", "res-y"
case(
    "a wait chain reaches the process that works",
    facts(leases=leases([holder(61, X, 500), holder(62, Y, 400)], [waiter(60, X, 100, [61]), waiter(61, Y, 90, [62])]), processes=[proc(62, 1000, 100, "cargo test -p x")]),
    [("leases.wait-long", "warn", X), ("leases.wait-long", "warn", Y)],
    waits=[("lease", 60, 62, "running"), ("lease", 61, 62, "running")],
    processes=[(62, "holding", None)],
)
case(
    "a wait cycle is named and both chains end in a waiting process",
    facts(leases=leases([holder(71, X, 500), holder(70, Y, 500)], [waiter(70, X, 100, [71]), waiter(71, Y, 100, [70])], cycles=[{"members": ["70/0", "71/0"], "victim": "71/0"}])),
    [("leases.cycle", "error", "70/0 → 71/0"), ("leases.wait-long", "warn", X), ("leases.wait-long", "warn", Y)],
    waits=[("lease", 70, 71, "waiting"), ("lease", 71, 70, "waiting")],
)
case("a queued waiter behind another waiter names the holder", facts(leases=leases([holder(81, R, 700)], [waiter(82, R, 80, [81]), waiter(83, R, 70, [82, 81], position=2, queued=2)])), [("leases.wait-long", "warn", R), ("leases.wait-long", "warn", R)], waits=[("lease", 82, 81, "unknown"), ("lease", 83, 81, "unknown")])

case(
    "a waiter queued behind the ticket of a dead process",
    facts(leases=leases([holder(31, "res-d", 400)], [waiter(41, "res-d", 90, [99, 31], queued=2), waiter(42, "res-d", 30, [99])], dead_holders=[99], tickets=1)),
    [("leases.wait-long", "warn", "res-d"), ("leases.dead-holder", "warn", "99"), ("leases.dead-ticket", "info", "queue")],
    waits=[("lease", 41, 31, "unknown"), ("lease", 42, 99, "gone")],
)
case("a wait of exactly the warning age behind a dead ticket is a warning", facts(leases=leases([], [waiter(42, "res-d", 60, [99])], dead_holders=[99])), [("leases.wait-long", "warn", "res-d"), ("leases.dead-holder", "warn", "99")], waits=[("lease", 42, 99, "gone")])
case(
    "a young wait behind a dead ticket is only a note",
    facts(leases=leases([], [waiter(42, "res-d", 59, [99])], dead_holders=[99, 98])),
    [("leases.dead-holder", "info", "99")],
    waits=[("lease", 42, 99, "gone")],
)
case(
    "capacity waits behind the biggest grant, one grant is mostly idle",
    facts(host=host(), capacity=capacity([grant("run", 80, 5632, 114, 30004), grant("show", 81, 5120, 3000, 30), grant("cargo", 82, 3584, 3000, 474)], [cwait("cargo", 90, 29), cwait("test", 91, 30), cwait("show", 92, 300, "load")]), processes=[proc(80, 1000, 30004, "bun /w/script.ts repo run a", cpu=94.6, rss=113)]),
    [("capacity.wait-long", "error", "show"), ("capacity.wait-long", "warn", "test"), ("capacity.grant-idle", "warn", "run")],
    waits=[("capacity", 92, 80, "running"), ("capacity", 91, 80, "running"), ("capacity", 90, 80, "running")],
    processes=[(80, "admitted", None)],
)
case("idle grants at their boundaries", facts(capacity=capacity([grant("alpha", 11, 1000, 249, 600), grant("beta", 12, 1000, 250, 600), grant("gamma", 13, 1000, 0, 599), grant("delta", 14, 1000, None, 600)], [cwait("w", 15, 5)])), [("capacity.grant-idle", "warn", "alpha")], waits=[("capacity", 15, 11, "unknown")])
case("idle grants are not reported while nobody waits", facts(capacity=capacity([grant("alpha", 11, 1000, 0, 6000)], [])), [])

case("sessions: gone, crowded, damaged", facts(sessions={"live": 8, "idle": 2, "gone": 2, "damaged": 1}), [("sessions.crowded", "warn", "agent-sessions"), ("sessions.damaged", "warn", "agent-sessions"), ("sessions.gone", "info", "agent-sessions")])
case("one gone session is a note", facts(sessions={"live": 0, "idle": 0, "gone": 1, "damaged": 0}), [("sessions.gone", "info", "agent-sessions")])
case("sessions just below crowded", facts(sessions={"live": 7, "idle": 0, "gone": 0, "damaged": 0}), [])

case("graph without a generation", facts(graph=graph("absent", None, "none", None, None, None)), [("graph.unpublished", "warn", "graph")])
case("graph without a generation while a graph lease is awaited is an error", facts(graph=graph("absent", None, "none", None, None, None), leases=leases([holder(91, "repository-graph-facts:/w", 400, cls="graph", owner="graph")], [waiter(92, "repository-graph-facts:/w", 60, [91], cls="graph", owner="graph")])), [("graph.unpublished", "error", "graph"), ("leases.wait-long", "warn", "repository-graph-facts:/w")], waits=[("lease", 92, 91, "unknown")])
case("graph pointer damaged", facts(graph=graph("damaged", None, "none", None, 5, None)), [("graph.damaged", "error", "graph")])
case("graph manifest missing", facts(graph=graph("present", 4, "missing", None, 5, 0)), [("graph.damaged", "error", "graph")])
case("graph implementation newer by one second", facts(graph=graph(newer=1)), [("graph.implementation-newer", "info", "graph")])
case("graph implementation not newer", facts(graph=graph(newer=0)), [])

case("dependencies without a pointer", facts(dependencies=dependencies([], "absent")), [("dependencies.unpublished", "error", "dependencies")])
case(
    "dependency scopes: missing, unready, inputs newer, orphans, unready dirs",
    facts(dependencies=dependencies([scope("."), scope("✏️s", "unready"), scope("🎓️teaching", "missing"), scope("🌎️hub", newer="🌎️hub/bun.lock")], orphans=11, unready=1, oldest=3600)),
    [("dependencies.generation-missing", "error", "✏️s"), ("dependencies.generation-missing", "error", "🎓️teaching"), ("dependencies.inputs-newer", "warn", "🌎️hub"), ("dependencies.unready-generation", "warn", "dependencies"), ("dependencies.orphans", "info", "dependencies")],
)
case("many superseded generations warn; an unready directory under a running build is not reported", facts(dependencies=dependencies([scope(".")], orphans=12, unready=2, oldest=7200), leases=leases([holder(95, "javascript-dependency-build:/w", 100, cls="image", owner="storage")])), [("dependencies.orphans", "warn", "dependencies")])
case("an unready directory younger than the age is fine", facts(dependencies=dependencies([scope(".")], unready=1, oldest=3599)), [])

case("dead publications below the byte threshold", facts(caches=caches(2, 104857599), storage=storage([part("graph", capacityBytes=4 * GIB, freeBytes=2 * GIB)])), [("caches.dead-publication", "info", "graph")])
case("dead publications at the byte threshold", facts(caches=caches(2, 104857600), storage=storage([part("graph", capacityBytes=4 * GIB, freeBytes=2 * GIB)])), [("caches.dead-publication", "warn", "graph")])
case("dead publications while the graph image is below its floor", facts(caches=caches(31, 581000000), storage=storage([part("graph", capacityBytes=4 * GIB, freeBytes=990449664)])), [("caches.dead-publication", "error", "graph"), ("storage.image-below-floor", "error", "graph")])
case("retired and undeclared cache entries", facts(caches=caches(0, 0, ["repository:a", "native:b"], ["repository:c"])), [("caches.retired-roots", "info", "cache"), ("caches.undeclared", "info", "cache")])

case(
    "process phases, orphans, idle holders and the relevance filter",
    facts(
        leases=leases([holder(100, "res-1", 700), holder(101, "res-2", 700)]),
        processes=[
            proc(100, 1, 700, "bun /w/script.ts repo cargo test", cpu=0.5),
            proc(102, 100, 650, "cargo test -p x", cpu=0.4),
            proc(101, 5000, 700, "bun /w/script.ts repo run a", cpu=1.0),
            proc(103, 1, 700, "rustc --crate-name x"),
            proc(104, 1, 100, "cargo build"),
            proc(105, 1, 900, "/bin/zsh -c repo cargo test"),
            proc(106, 1, 900, "repo mcp stdio"),
            proc(107, 1, 900, "vim notes.txt"),
        ],
    ),
    [("processes.idle-holder", "warn", "100"), ("processes.orphan", "warn", "100")],
    processes=[(100, "holding", None), (101, "holding", None), (102, "admitted", 100), (103, "orphan", None), (104, "running", None)],
)
case(
    "a process younger than the orphan age is no orphan",
    facts(leases=leases([holder(100, "res-1", 100)]), processes=[proc(100, 1, 599, "bun /w/script.ts repo cargo test", cpu=50), proc(103, 1, 599, "rustc --crate-name x")]),
    [],
    processes=[(100, "holding", None), (103, "running", None)],
)
case(
    "waiting processes keep their phase and their children inherit it",
    facts(leases=leases([holder(110, "res-1", 20)], [waiter(111, "res-1", 20, [110])]), capacity=capacity([], [cwait("cargo", 112, 5)]), processes=[proc(110, 900, 20, "cargo check"), proc(111, 900, 20, "bun /w/script.ts repo run b"), proc(112, 900, 5, "bun /w/script.ts repo cargo test"), proc(113, 111, 19, "tsc -b")]),
    [],
    waits=[("lease", 111, 110, "running"), ("capacity", 112, None, None)],
    processes=[(111, "waiting-lease", None), (112, "waiting-capacity", None), (110, "holding", None), (113, "running", 111)],
)

case(
    "probe results become findings",
    {**facts(), "probes": [{"area": "tools", "status": "unavailable", "durationMs": 3, "reason": "The tool registry is not installed in this checkout"}, {"area": "storage", "status": "failed", "durationMs": 9, "reason": "boom"}, {"area": "leases", "status": "timeout", "durationMs": 8000}, {"area": "graph", "status": "ok", "durationMs": 40}, {"area": "caches", "status": "skipped", "durationMs": 0}]},
    [("doctor.probe-failed", "warn", "storage"), ("doctor.probe-timeout", "warn", "leases"), ("doctor.probe-unavailable", "info", "tools")],
)


def main():
    out = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "staged", "test-fixtures.json")
    for item in CASES:
        item["expect"]["findings"] = [list(row) for row in item["expect"]["findings"]]
    with open(out, "w", encoding="utf-8") as handle:
        json.dump({"version": 1, "cases": CASES}, handle, indent=1, ensure_ascii=False)
        handle.write("\n")
    print("wrote %d cases to %s" % (len(CASES), out))


if __name__ == "__main__":
    main()
