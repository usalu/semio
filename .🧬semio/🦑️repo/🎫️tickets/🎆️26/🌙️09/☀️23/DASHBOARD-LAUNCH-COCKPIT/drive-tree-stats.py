#!/usr/bin/env python3
"""Reads `semio command-tree --dump-tree` JSON from stdin and prints catalog statistics (leaf kinds, verbs, label lengths, estimated launcher memory)."""
import json, sys, collections, time
t0 = time.time(); data = json.load(sys.stdin); print("parsed in", round(time.time() - t0, 1), "s")
leaves = []; 
def walk(n, path):
    p = path if n["key"] == "root" else path + [n["label"]]
    if "leaf" in n: leaves.append((" / ".join(p), n["leaf"]))
    for c in n.get("children", []): walk(c, p)
walk(data, [])
kinds = collections.Counter(l["kind"] for _, l in leaves)
verbs = collections.Counter(p.split(" / ")[0] for p, _ in leaves)
lens = sorted(len(p) for p, _ in leaves)
env_total = sum(len(pr["env"]) for _, l in leaves for pr in ([l] if l["kind"] == "process" else l.get("processes", [])))
print("leaves", len(leaves), dict(kinds))
print("label length: total", sum(lens), "avg", sum(lens) // len(lens), "p50", lens[len(lens)//2], "p95", lens[int(len(lens)*.95)], "max", lens[-1])
print("env pairs", env_total)
print("top verbs", verbs.most_common(40))
ticket = [p for p, _ in leaves if p.startswith("tickets / ")]
print("ticket leaves", len(ticket), "of which close/reopen", sum(1 for p in ticket if p.endswith("/ close") or p.endswith("/ reopen")))
dups = collections.Counter(p for p, _ in leaves); print("duplicate labels", sum(1 for c in dups.values() if c > 1))
print("launch-config leaves", sum(1 for p, _ in leaves if " / launch / " in p), "scripts", sum(1 for p, _ in leaves if " / workspace scripts / " in p))
