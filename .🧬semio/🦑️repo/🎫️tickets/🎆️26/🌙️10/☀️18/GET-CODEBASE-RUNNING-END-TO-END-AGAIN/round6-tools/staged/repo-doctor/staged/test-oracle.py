#!/usr/bin/env python3
"""Independent re-derivation of the doctor analysis: reads {"policy": ..., "facts": ...} (or "cases": [facts, ...]) from standard input and prints the findings (id, severity, subject), the wait chains, the process phases, the verdict and the counts. It shares no code with the TypeScript implementation; the language-agnostic fixture must come out the same from both."""
import json
import re
import sys

AREAS = ["tools", "storage", "leases", "capacity", "sessions", "graph", "dependencies", "caches", "processes", "host", "doctor"]
RANK = {"error": 0, "warn": 1, "info": 2}
PHASE = {"waiting-lease": 0, "waiting-capacity": 1, "holding": 2, "admitted": 3, "orphan": 4, "running": 5}
WRAPPERS = {"sh", "bash", "zsh", "dash", "fish", "pwsh", "powershell", "cmd", "env", "nohup", "sudo", "time", "caffeinate", "taskpolicy", "nice", "xargs", "tee", "script"}


def area_of(rule_id):
    return rule_id.split(".")[0]


def relevant(command, patterns, excluded):
    words = command.split()
    program = re.sub(r"\.exe$", "", words[0].split("/")[-1], flags=re.I) if words else ""
    if program in WRAPPERS:
        return False
    head = " ".join(words[:8])
    return any(re.search(p, head) for p in patterns) and not any(re.search(p, head) for p in excluded)


def analyze(policy, facts):
    out = []

    def add(rule_id, severity, subject):
        out.append((rule_id, severity, subject[:300]))

    probes = facts.get("probes", [])
    for probe in probes:
        if probe["status"] == "failed":
            add("doctor.probe-failed", "warn", probe["area"])
        elif probe["status"] == "timeout":
            add("doctor.probe-timeout", "warn", probe["area"])
        elif probe["status"] == "unavailable":
            add("doctor.probe-unavailable", "info", probe["area"])

    leases = facts.get("leases")
    capacity = facts.get("capacity")
    host = facts.get("host")
    storage = facts.get("storage")
    table = {row["pid"]: row for row in facts.get("processes", [])}
    lease_waiters = leases["waiters"] if leases else []
    lease_holders = leases["holders"] if leases else []
    cap_waiters = capacity["waiters"] if capacity else []
    cap_grants = capacity["grants"] if capacity else []

    waits = []
    dead_holders = set(leases["deadHolders"]) if leases else set()
    holders_by_pid_hash = {(h["pid"], h["hash"]): h for h in lease_holders}
    waiter_by_pid = {}
    for w in lease_waiters:
        waiter_by_pid.setdefault(w["pid"], w)

    def parties(waiter):
        rows, seen = [], set()
        for pid in waiter["blockers"]:
            if pid in seen or pid == waiter["pid"]:
                continue
            seen.add(pid)
            held = holders_by_pid_hash.get((pid, waiter["hash"]))
            rows.append({"pid": pid, "role": "holder" if held else "ahead", "held": held["heldSeconds"] if held else None})
        return rows

    def pick(rows):
        holders = sorted([r for r in rows if r["role"] == "holder"], key=lambda r: (-(r["held"] or 0), r["pid"]))
        return holders[0] if holders else (rows[0] if rows else None)

    for w in lease_waiters:
        root = pick(parties(w))
        state = "unknown"
        visited = {w["pid"]}
        for _ in range(8):
            if root is None:
                break
            visited.add(root["pid"])
            nxt = waiter_by_pid.get(root["pid"])
            if nxt is None:
                state = "gone" if root["pid"] in dead_holders else ("running" if root["pid"] in table else "unknown")
                break
            candidate = pick(parties(nxt))
            if candidate is None or candidate["pid"] in visited:
                state = "waiting"
                break
            root = candidate
        waits.append(["lease", w["pid"], root["pid"] if root else None, state if root else None, w["waitedSeconds"]])
    grants = sorted(cap_grants, key=lambda g: (-g["reservedMegabytes"], g["pid"]))
    waiting_pids = {w["pid"] for w in lease_waiters} | {w["pid"] for w in cap_waiters}
    for w in cap_waiters:
        top = grants[0] if grants else None
        state = None
        if top:
            state = "waiting" if top["pid"] in waiting_pids else ("running" if top["pid"] in table else "unknown")
        waits.append(["capacity", w["pid"], top["pid"] if top else None, state, w["waitedSeconds"]])
    waits.sort(key=lambda r: (-r[4], r[1]))

    if host:
        h = policy["host"]
        if host["pressure"] == "critical":
            add("host.memory-pressure", "error", "memory")
        elif host["pressure"] == "warn":
            add("host.memory-pressure", "warn", "memory")
        if host["floorMegabytes"] > 0:
            if host["availableMegabytes"] <= host["floorMegabytes"]:
                add("host.low-memory", "error", "memory")
            elif host["availableMegabytes"] * 100 < host["floorMegabytes"] * (100 + h["lowMemoryHeadroomPercent"]):
                add("host.low-memory", "warn", "memory")
        if (host["swapUsedMegabytes"] is not None and host["swapUsedMegabytes"] >= h["swapWarnMegabytes"]) or (host["swapOutMegabytesPerSecond"] is not None and host["swapOutMegabytesPerSecond"] >= h["swapOutWarnMegabytesPerSecond"]):
            add("host.swap-high", "warn", "swap")
        if host["load"] is not None:
            ratio = host["load"] / host["cores"]
            if ratio >= h["loadPerCoreError"]:
                add("host.load-high", "error", "load")
            elif ratio >= h["loadPerCoreWarn"]:
                add("host.load-high", "warn", "load")
        if host["openFiles"] is not None and host["openFilesMaximum"] is not None:
            if (100 * host["openFiles"]) // host["openFilesMaximum"] >= h["openFilesWarnPercent"]:
                add("host.open-files-high", "warn", "files")
        if host["attachedImages"] is not None:
            if host["attachedImages"] >= h["imagesError"]:
                add("host.images-many", "error", "images")
            elif host["attachedImages"] >= h["imagesWarn"]:
                add("host.images-many", "warn", "images")
        if host["governor"] == "off":
            add("capacity.governor-off", "warn", "SEMIO_HOST_CAPACITY")

    if storage:
        s = policy["storage"]
        free, total = storage["hostFreeBytes"], storage["hostTotalBytes"]
        if free is not None and total:
            if free < s["hostFreeErrorBytes"]:
                add("storage.host-free-low", "error", "cache-disk")
            elif free < s["hostFreeWarnBytes"]:
                add("storage.host-free-low", "warn", "cache-disk")
        for p in storage["partitions"]:
            if p["image"] == "damaged":
                add("storage.image-damaged", "error", p["id"])
            if p["image"] == "ok" and p["state"] == "absent" and p["backing"] == "missing":
                add("storage.backing-missing", "error", p["id"])
            if p["state"] == "detached":
                add("storage.image-detached", "info", p["id"])
            if p["publication"] != "none":
                add("storage.publication-pending", "error" if (p["publicationAgeSeconds"] or 0) >= s["publicationErrorSeconds"] else "info", p["id"])
            if p["state"] == "mounted" and p["freeBytes"] is not None and p["capacityBytes"]:
                floor = s["floorBytes"].get(p["id"])
                percent = (100 * p["freeBytes"]) // p["capacityBytes"]
                if floor is not None and p["freeBytes"] < floor:
                    add("storage.image-below-floor", "error", p["id"])
                elif percent < s["freePercentError"]:
                    add("storage.image-low", "error", p["id"])
                elif percent < s["freePercentWarn"]:
                    add("storage.image-low", "warn", p["id"])

    essential = set(policy["tools"]["essential"])
    for row in (facts.get("tools") or {}).get("rows", []):
        if row["state"] == "missing":
            add("tools.missing-essential" if row["id"] in essential else "tools.missing", "error" if row["id"] in essential else "warn", row["id"])
        elif row["state"] == "drift":
            add("tools.pin-drift", "warn", row["id"])

    if leases:
        l = policy["leases"]
        for c in leases["cycles"]:
            add("leases.cycle", "error", " → ".join(c["members"]))
        seen = set()
        for hld in lease_holders:
            key = (hld["pid"], hld["hash"])
            if key in seen or hld["mode"] != "exclusive":
                continue
            seen.add(key)
            behind = len([w for w in lease_waiters if w["hash"] == hld["hash"] and w["pid"] != hld["pid"]])
            if behind > 0 and hld["heldSeconds"] >= l["holdErrorSeconds"]:
                add("leases.hold-long", "error", hld["resource"])
            elif behind > 0 and hld["heldSeconds"] >= l["holdWarnSeconds"]:
                add("leases.hold-long", "warn", hld["resource"])
            elif hld["heldSeconds"] >= l["holdErrorSeconds"]:
                add("leases.hold-long", "warn", hld["resource"])
        for w in lease_waiters:
            if w["waitedSeconds"] >= l["waitErrorSeconds"]:
                add("leases.wait-long", "error", w["resource"])
            elif w["waitedSeconds"] >= l["waitWarnSeconds"]:
                add("leases.wait-long", "warn", w["resource"])
        groups = {}
        for w in lease_waiters:
            groups.setdefault(w["hash"], []).append(w)
        for rows in groups.values():
            if max([len(rows)] + [r["queued"] or 0 for r in rows]) >= l["queueDepthWarn"]:
                add("leases.queue-deep", "warn", rows[0]["resource"])
        kinds = {}
        for v in leases["violations"]:
            kinds[v["kind"]] = kinds.get(v["kind"], 0) + 1
        for kind in kinds:
            add("leases.order-violation", "info" if kind == "out-of-order" else "warn", kind)
        reg = leases["registry"]
        if reg["orphanFolders"] > 0 or reg["deadEntries"] > 0:
            add("leases.registry-stale", "warn" if reg["orphanFolders"] >= l["registryOrphanFoldersWarn"] else "info", "lease-registry")
        if leases["deadTickets"] > 0:
            add("leases.dead-ticket", "info", "queue")
        for pid in sorted(dead_holders):
            behind = [w for w in lease_waiters if pid in w["blockers"]]
            if behind:
                add("leases.dead-holder", "warn" if max(w["waitedSeconds"] for w in behind) >= l["waitWarnSeconds"] else "info", str(pid))

    if capacity:
        c = policy["capacity"]
        for w in cap_waiters:
            if w["waitedSeconds"] >= c["waitErrorSeconds"]:
                add("capacity.wait-long", "error", w["operation"])
            elif w["waitedSeconds"] >= c["waitWarnSeconds"]:
                add("capacity.wait-long", "warn", w["operation"])
        if cap_waiters:
            for g in cap_grants:
                if g["heldSeconds"] >= c["idleGrantSeconds"] and g["observedMegabytes"] is not None and g["reservedMegabytes"] > 0 and g["observedMegabytes"] * 100 < g["reservedMegabytes"] * c["idleObservedPercent"]:
                    add("capacity.grant-idle", "warn", g["operation"])

    sessions = facts.get("sessions")
    if sessions:
        if sessions["gone"] > 0:
            add("sessions.gone", "info", "agent-sessions")
        if sessions["live"] >= policy["sessions"]["crowdedLive"]:
            add("sessions.crowded", "warn", "agent-sessions")
        if sessions["damaged"] > 0:
            add("sessions.damaged", "warn", "agent-sessions")

    graph = facts.get("graph")
    if graph:
        slow = any(w["owner"] == "graph" and w["waitedSeconds"] >= policy["leases"]["waitWarnSeconds"] for w in lease_waiters)
        if graph["pointer"] == "absent":
            add("graph.unpublished", "error" if slow else "warn", "graph")
        elif graph["pointer"] == "damaged":
            add("graph.damaged", "error", "graph")
        elif graph["manifest"] in ("missing", "damaged"):
            add("graph.damaged", "error", "graph")
        elif graph["implementationNewerSeconds"] is not None and graph["implementationNewerSeconds"] > 0:
            add("graph.implementation-newer", "info", "graph")

    deps = facts.get("dependencies")
    if deps:
        d = policy["dependencies"]
        if deps["pointers"] == "absent":
            add("dependencies.unpublished", "error", "dependencies")
        for sc in deps["scopes"]:
            if sc["directory"] != "ready":
                add("dependencies.generation-missing", "error", sc["scope"])
            if sc["inputsNewer"] is not None:
                add("dependencies.inputs-newer", "warn", sc["scope"])
        if deps["orphans"] > 0:
            add("dependencies.orphans", "warn" if deps["orphans"] >= d["orphanCountWarn"] else "info", "dependencies")
        building = any(h["mode"] == "exclusive" and re.search("dependenc", h["resource"], re.I) for h in lease_holders)
        if not building and deps["unready"] > 0 and (deps["unreadyOldestSeconds"] or 0) >= d["unreadyAgeSeconds"]:
            add("dependencies.unready-generation", "warn", "dependencies")

    caches = facts.get("caches")
    if caches:
        if caches["deadPublications"] > 0:
            declared = policy["caches"]["deadPublications"][0]["partition"]
            part = next((p for p in (storage or {"partitions": []})["partitions"] if p["id"] == declared), None)
            floor = policy["storage"]["floorBytes"].get(declared)
            tight = part is not None and part["freeBytes"] is not None and floor is not None and part["freeBytes"] < floor
            severity = "error" if tight else ("warn" if caches["deadPublicationBytes"] >= policy["caches"]["deadPublicationWarnBytes"] else "info")
            add("caches.dead-publication", severity, declared)
        if caches["retired"]:
            add("caches.retired-roots", "info", "cache")
        if caches["undeclared"]:
            add("caches.undeclared", "info", "cache")

    rows = analyze_processes(policy, facts, add)

    out.sort(key=lambda r: (RANK[r[1]], AREAS.index(area_of(r[0])), r[0], r[2]))
    counts = {"error": 0, "warn": 0, "info": 0}
    for r in out:
        counts[r[1]] += 1
    verdict = "blocked" if counts["error"] else ("degraded" if counts["warn"] else "ok")
    return {"findings": [list(r) for r in out], "waits": [w[:4] for w in waits], "processes": rows, "verdict": verdict, "counts": counts}


def analyze_processes(policy, facts, add):
    pr = policy["processes"]
    leases = facts.get("leases")
    capacity = facts.get("capacity")
    table = {row["pid"]: row for row in facts.get("processes", [])}
    waiting_lease = {w["pid"] for w in (leases["waiters"] if leases else [])}
    waiting_capacity = {w["pid"] for w in (capacity["waiters"] if capacity else [])}
    holders = leases["holders"] if leases else []
    holding = {h["pid"] for h in holders}
    exclusive = {}
    for h in holders:
        if h["mode"] == "exclusive":
            exclusive.setdefault(h["pid"], h["resource"])
    admitted = {g["pid"]: g["operation"] for g in (capacity["grants"] if capacity else [])}
    known = waiting_lease | waiting_capacity | holding | set(admitted)
    waiting = waiting_lease | waiting_capacity

    def ancestor(pid):
        seen = set()
        cur = table[pid]["parent"] if pid in table else None
        while cur is not None and cur > 1 and cur not in seen:
            if cur in known:
                return cur
            seen.add(cur)
            cur = table[cur]["parent"] if cur in table else None
        return None

    children = {}
    for row in table.values():
        children.setdefault(row["parent"], []).append(row["pid"])

    def tree_cpu(pid):
        total, stack, seen = 0.0, [pid], set()
        while stack:
            cur = stack.pop()
            if cur in seen:
                continue
            seen.add(cur)
            total += table[cur]["cpuPercent"] if cur in table else 0
            stack.extend(children.get(cur, []))
        return total

    rows = []
    for row in table.values():
        pid = row["pid"]
        via = None if pid in known else ancestor(pid)
        rel = relevant(row["command"], pr["patterns"], pr["exclude"])
        if not rel and pid not in known and via is None:
            continue
        if pid in waiting_lease:
            own = "waiting-lease"
        elif pid in waiting_capacity:
            own = "waiting-capacity"
        elif pid in holding:
            own = "holding"
        elif pid in admitted:
            own = "admitted"
        else:
            own = None
        phase = own if own else (("running" if via in waiting else "admitted") if via is not None else "running")
        gone_parent = row["parent"] <= 1
        if own is None and via is None and gone_parent and row["ageSeconds"] >= pr["orphanAgeSeconds"] and rel:
            phase = "orphan"
        rows.append([pid, phase, None if own else via, row["ageSeconds"]])
        if own and row["parent"] == 1 and row["ageSeconds"] >= pr["orphanAgeSeconds"] and rel:
            holds = len([h for h in holders if h["pid"] == pid]) + (1 if pid in admitted else 0)
            if holds > 0:
                add("processes.orphan", "warn", str(pid))
        if own in ("holding", "admitted") and (pid in exclusive or pid in admitted):
            held = max([0] + [h["heldSeconds"] for h in holders if h["pid"] == pid and h["mode"] == "exclusive"] + [g["heldSeconds"] for g in (capacity["grants"] if capacity else []) if g["pid"] == pid])
            if held >= pr["idleHolderSeconds"] and tree_cpu(pid) < pr["idleCpuPercent"]:
                add("processes.idle-holder", "warn", str(pid))
    rows.sort(key=lambda r: (PHASE[r[1]], -r[3], r[0]))
    return [r[:3] for r in rows]


if __name__ == "__main__":
    document = json.load(sys.stdin)
    if "cases" in document:
        print(json.dumps([analyze(document["policy"], facts) for facts in document["cases"]], ensure_ascii=False))
    else:
        print(json.dumps(analyze(document["policy"], document["facts"]), ensure_ascii=False))
