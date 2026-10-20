#!/usr/bin/env python3
"""mkfixture.py: writes src/sessions-fixture.json. Every expected value is computed here by an implementation independent of the TypeScript one (plain integer arithmetic, sorted() with tuple keys, set algebra over path prefixes, str.format for messages)."""
import json, itertools, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
G2 = len(sys.argv) > 1 and sys.argv[1] == "g2"
STAGED = os.path.join(HERE, "staged", "g2") if G2 else os.path.join(HERE, "staged")
schema = json.load(open(os.path.join(STAGED, "sessions-schema.json")))
P = schema["definitions"]["Policy"]["const"]
M = schema["definitions"]["Messages"]["const"]
DAY = 86400000


def plan(host, mem):
    cores, megabytes = host["cores"], host["memoryMegabytes"]
    floor = min(mem["floorMaximumMegabytes"], max(mem["floorMinimumMegabytes"], megabytes * mem["floorBasisPoints"] // 10000))
    spare = max(0, megabytes - floor)
    budget = spare * P["servers"]["memoryShareBasisPoints"] // 10000
    by_memory = budget // P["servers"]["residentMegabytes"]
    by_cores = cores // P["servers"]["coresPerServer"]
    bounded = min(by_memory, by_cores, P["servers"]["maximum"])
    slots = max(P["servers"]["minimum"], bounded)
    if bounded < P["servers"]["minimum"]:
        limit = "floor"
    elif bounded == by_memory:
        limit = "memory"
    elif bounded == by_cores:
        limit = "cores"
    else:
        limit = "policy"
    overhead = P["servers"]["residentMegabytes"] - P["servers"]["heapMegabytes"]
    heap = P["servers"]["heapMegabytes"] if by_memory >= 1 else max(P["servers"]["minimumHeapMegabytes"], min(P["servers"]["heapMegabytes"], budget - overhead))
    base = {"slots": slots}
    if G2:
        base["services"] = P["servers"]["serviceMaximum"]
    return {**base, "heapMegabytes": heap, "residentMegabytes": heap + overhead, "budgetMegabytes": budget,
            "cpuPerServer": max(1, cores // slots), "memoryPerServerMegabytes": max(256, (spare - budget) // slots), "limit": limit}


FLOOR = {"floorBasisPoints": 2000, "floorMinimumMegabytes": 4096, "floorMaximumMegabytes": 8192}
hosts = [
    ("this machine: 32 GB, 10 cores", 10, 32768), ("laptop: 16 GB, 8 cores", 8, 16384), ("small: 8 GB, 4 cores", 4, 8192),
    ("tiny: 4 GB, 2 cores", 2, 4096), ("big: 128 GB, 32 cores", 32, 131072), ("cores bound: 64 GB, 4 cores", 4, 65536),
    ("single core: 64 GB, 1 core", 1, 65536), ("workstation: 64 GB, 16 cores", 16, 65536),
]
plans = [{"name": n, "host": {"cores": c, "memoryMegabytes": m}, "memory": FLOOR, "expected": plan({"cores": c, "memoryMegabytes": m}, FLOOR)} for n, c, m in hosts]


def k(i):
    return f"{i:016x}"


def rank(slots, states, key):
    def tier(slot):
        s = states.get(slot)
        return 2 if s is None else 0 if s["key"] == key else 1 if key in s["keys"] else 3
    def order(slot):
        s = states.get(slot)
        t = tier(slot)
        recency = -s["usedAt"] if t in (0, 1) else s["usedAt"] if t == 3 else 0
        return (t, recency, int(slot[1:]))
    return sorted(slots, key=order)


def state(slot, key, keys, used):
    return {"version": 1, "slot": slot, "key": key, "keys": keys, "usedAt": used}


rank_cases = []
slots4 = ["s0", "s1", "s2", "s3"]
cases = [
    ("nothing used yet keeps slot order", {}, k(1)),
    ("the slot of the last selection comes first", {"s0": state("s0", k(2), [k(2)], 100), "s1": state("s1", k(1), [k(1)], 50)}, k(1)),
    ("a slot that still keeps the package beats a never used slot", {"s0": state("s0", k(2), [k(2), k(1)], 100), "s2": state("s2", k(3), [k(3)], 10)}, k(1)),
    ("never used slots come before other used slots", {"s0": state("s0", k(2), [k(2)], 100), "s1": state("s1", k(3), [k(3)], 200)}, k(1)),
    ("used slots are least recently used first", {"s0": state("s0", k(2), [k(2)], 300), "s1": state("s1", k(3), [k(3)], 100), "s2": state("s2", k(4), [k(4)], 200), "s3": state("s3", k(5), [k(5)], 400)}, k(1)),
    ("two warm slots prefer the more recent one", {"s0": state("s0", k(1), [k(1)], 100), "s1": state("s1", k(1), [k(1)], 900), "s2": state("s2", k(2), [k(2), k(1)], 500)}, k(1)),
    ("recency ties keep the slot number order", {"s0": state("s0", k(2), [k(2)], 100), "s1": state("s1", k(3), [k(3)], 100)}, k(1)),
]
for name, states, key in cases:
    rank_cases.append({"name": name, "slots": slots4, "states": states, "key": key, "expected": rank(slots4, states, key)})

NOW = 100 * DAY


def garbage(inv, slots, now, held):
    valid = {f"s{i}" for i in range(slots)}
    if G2:
        valid |= {f"d{i}" for i in range(P["servers"]["serviceMaximum"])}
    packages, bases = [], []
    for row in inv["slots"]:
        if row["slot"] in held:
            continue
        if row["slot"] not in valid:
            if row["outputBase"]:
                bases.append({"slot": row["slot"], "reason": "capacity"})
            for p in row["packages"]:
                packages.append({"slot": row["slot"], "key": p["key"], "reason": "capacity"})
            continue
        st = row.get("state")
        if row["outputBase"] and st is not None and now - st["usedAt"] > P["retention"]["outputBaseMaximumIdleDays"] * DAY:
            bases.append({"slot": row["slot"], "reason": "idle"})
        ordered = sorted(row["packages"], key=lambda p: (-p["modifiedAt"], p["key"]))
        for index, p in enumerate(ordered):
            if index == 0 or (st is not None and p["key"] == st["key"]):
                continue
            if index >= P["retention"]["packagesPerSlot"]:
                packages.append({"slot": row["slot"], "key": p["key"], "reason": "count"})
            elif now - p["modifiedAt"] > P["retention"]["packageMaximumAgeDays"] * DAY:
                packages.append({"slot": row["slot"], "key": p["key"], "reason": "age"})
    return {"packages": sorted(packages, key=lambda p: p["slot"] + p["key"]), "outputBases": sorted(bases, key=lambda b: b["slot"]), "foreign": sorted(inv["foreign"])}


def pkgs(*pairs):
    return [{"key": k(i), "modifiedAt": NOW - days * DAY} for i, days in pairs]


gc_cases = []
inventories = [
    ("empty inventory", 3, {"slots": [], "foreign": []}, []),
    ("old packages beyond the retained age go, the newest always stays", 3, {"slots": [{"slot": "s0", "outputBase": True, "state": state("s0", k(1), [k(1)], NOW - DAY), "packages": pkgs((1, 20), (2, 10), (3, 1))}], "foreign": []}, []),
    ("only the newest package of a slot stays when everything is old", 3, {"slots": [{"slot": "s1", "outputBase": True, "state": state("s1", k(9), [k(9)], NOW - 30 * DAY), "packages": pkgs((4, 30), (5, 31), (6, 32))}], "foreign": []}, []),
    ("packages beyond the retained count go even when young", 3, {"slots": [{"slot": "s0", "outputBase": True, "state": state("s0", k(1), [k(1)], NOW - 1), "packages": pkgs(*[(i, 1) for i in range(1, 10)])}], "foreign": []}, []),
    ("slots beyond the capacity lose output base and packages", 2, {"slots": [{"slot": "s0", "outputBase": True, "packages": pkgs((1, 1))}, {"slot": "s2", "outputBase": True, "state": state("s2", k(2), [k(2)], NOW - DAY), "packages": pkgs((2, 1), (3, 2))}], "foreign": []}, []),
    ("an output base idle for two weeks goes", 3, {"slots": [{"slot": "s1", "outputBase": True, "state": state("s1", k(2), [k(2)], NOW - 15 * DAY), "packages": []}, {"slot": "s0", "outputBase": True, "state": state("s0", k(1), [k(1)], NOW - 13 * DAY), "packages": []}], "foreign": []}, []),
    ("a held slot is never touched", 2, {"slots": [{"slot": "s0", "outputBase": True, "state": state("s0", k(1), [k(1)], NOW - 40 * DAY), "packages": pkgs((1, 40), (2, 41))}, {"slot": "s5", "outputBase": True, "packages": pkgs((3, 1))}], "foreign": []}, ["s0", "s5"]),
    ("foreign entries are listed sorted", 3, {"slots": [], "foreign": ["workspaces/0123456789abcdef", "server/fedcba9876543210", "server/0123456789abcdef"]}, []),
]
if G2:
    inventories += [
        ("a service slot within the service capacity keeps its output base", 3, {"slots": [{"slot": "d1", "outputBase": True, "state": state("d1", k(7), [k(7)], NOW - DAY), "packages": pkgs((7, 1))}], "foreign": []}, []),
        ("a service slot beyond the service capacity is retired", 3, {"slots": [{"slot": "d4", "outputBase": True, "state": state("d4", k(8), [k(8)], NOW - DAY), "packages": pkgs((8, 1), (9, 2))}], "foreign": []}, []),
    ]
for name, slots, inv, held in inventories:
    row = {"name": name, "slots": slots, "now": NOW, "held": held, "inventory": inv, "expected": garbage(inv, slots, NOW, set(held))}
    if G2:
        row["services"] = P["servers"]["serviceMaximum"]
    gc_cases.append(row)


def restore(roots):
    prefix = P["leases"]["restorePrefix"]
    modes = {prefix: "shared"}
    for root in roots:
        parts = root.split("/")
        for i in range(1, len(parts)):
            modes.setdefault(prefix + "/".join(parts[:i]), "shared")
        modes[prefix + root] = "exclusive"
    if len(modes) > P["leases"]["restoreMaximumResources"]:
        return [{"resource": prefix, "mode": "exclusive"}]
    return [{"resource": r, "mode": modes[r]} for r in sorted(modes)]


restore_cases = [
    {"name": "one root", "roots": ["a/b/dist"], "expected": restore(["a/b/dist"])},
    {"name": "sibling roots share their ancestors", "roots": ["a/b/dist", "a/b/types"], "expected": restore(["a/b/dist", "a/b/types"])},
    {"name": "a root inside another root stays exclusive", "roots": ["a/b", "a/b/dist"], "expected": restore(["a/b", "a/b/dist"])},
    {"name": "top level root", "roots": ["dist"], "expected": restore(["dist"])},
    {"name": "no roots hold only the shared root", "roots": [], "expected": restore([])},
    {"name": "emoji paths stay as they are", "roots": ["🧰️framework/🛍️x/dist"], "expected": restore(["🧰️framework/🛍️x/dist"])},
    {"name": "too many resources fall back to the exclusive root", "roots": [f"p{i}/q{i}/r{i}/dist" for i in range(30)], "expected": restore([f"p{i}/q{i}/r{i}/dist" for i in range(30)])},
]
restore_refusals = [{"name": "absolute", "roots": ["/etc/x"]}, {"name": "dot dot", "roots": ["a/../b"]}, {"name": "empty segment", "roots": ["a//b"]}, {"name": "empty root", "roots": [""]}]

messages = []
for mid, values in [("waiting", {"slots": 3, "seconds": 12})] + ([("waiting-service", {"slots": 4, "seconds": 7})] if G2 else []) + [ ("claimed", {"slot": "s1", "mode": "warm"}), ("stopped", {"slot": "s2"}), ("status-header", {}), ("status-held", {}), ("status-free", {}), ("warm", {}), ("switch", {}), ("fresh", {})]:
    messages.append({"id": mid, "values": values, "en": M["en"][mid].format(**values), "de": M["de"][mid].format(**values)})

fixture = {
    "version": 1, "plans": plans, "ranks": rank_cases, "garbage": gc_cases, "restore": restore_cases, "restoreRefused": restore_refusals, "messages": messages,
    "workspaces": {"root": "/bounded/products/bazel/workspaces", "accepted": [{"slot": "s0", "key": k(1), "path": "/bounded/products/bazel/workspaces/s0/" + k(1)}], "refused": [{"slot": "x0", "key": k(1)}, {"slot": "s0", "key": "XYZ"}, {"slot": "../s0", "key": k(1)}, {"slot": "s0", "key": "0123456789abcde"}]},
    "languages": [{"environment": {"SEMIO_LANGUAGE": "de"}, "expected": "de"}, {"environment": {"SEMIO_LANGUAGE": "en", "LANG": "de_DE.UTF-8"}, "expected": "en"}, {"environment": {"LANG": "de_DE.UTF-8"}, "expected": "de"}, {"environment": {"LC_ALL": "C", "LANG": "de_DE.UTF-8"}, "expected": "en"}, {"environment": {}, "expected": "en"}],
    "overrides": [{"environment": {}, "slots": None}, {"environment": {"SEMIO_BAZEL_SERVERS": ""}, "slots": None}, {"environment": {"SEMIO_BAZEL_SERVERS": "2"}, "slots": 2}, {"environment": {"SEMIO_BAZEL_SERVERS": "12"}, "slots": 12}, {"environment": {"SEMIO_BAZEL_SERVERS": "0"}, "slots": "refused"}, {"environment": {"SEMIO_BAZEL_SERVERS": "100"}, "slots": "refused"}, {"environment": {"SEMIO_BAZEL_SERVERS": "two"}, "slots": "refused"}],
}
json.dump(fixture, open(os.path.join(STAGED, "sessions-fixture.json"), "w"), ensure_ascii=False, indent=1)
print("fixture rows:", {k_: len(v) for k_, v in fixture.items() if isinstance(v, list)})
