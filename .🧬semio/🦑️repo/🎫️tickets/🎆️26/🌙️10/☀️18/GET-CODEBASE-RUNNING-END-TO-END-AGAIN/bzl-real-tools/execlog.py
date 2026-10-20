#!/usr/bin/env python3
"""Summarizes Bazel --execution_log_json_file output: one row per spawn; with two logs, lists inputs whose digest differs per target."""
import hashlib, json, sys

NAMES = {("t_" + hashlib.sha256(i.encode()).hexdigest()[:16]): n for n, i in {"T1 layout:test-media-adoption-source": "@semio-tech/layout-layout-rs:test-media-adoption-source", "T2 writer:graph-generate": "@semio-tech/writer-writer-rs:graph-generate", "T3 framework-graph:generate": "@semio-tech/framework-graph:generate", "T4a ui-styling-tokens:generate": "@semio-tech/ui-styling-tokens:generate", "T4b ui-styling:generate": "@semio-tech/ui-styling:generate", "A akteursnetz-validate": "@semio-tech/mit-bestand-bericht:akteursnetz-validate"}.items()}

def load(path):
    text = open(path, encoding="utf-8", errors="surrogateescape").read()
    dec, i, rows = json.JSONDecoder(), 0, []
    while True:
        while i < len(text) and text[i].isspace(): i += 1
        if i >= len(text): break
        obj, i = dec.raw_decode(text, i)
        rows.append(obj)
    return rows

def fix(name):
    try: return name.encode("latin-1").decode("utf-8")
    except Exception: return name

def spawns(path):
    out = {}
    for r in load(path):
        label = NAMES.get((r.get("targetLabel") or "?").removeprefix("//:"), r.get("targetLabel") or "?")
        out.setdefault(label, []).append(r)
    return out

def summary(path):
    for label, rows in sorted(spawns(path).items()):
        for r in rows:
            print(f'{label}\t{r.get("mnemonic")}\trunner={r.get("runner")}\tcacheHit={r.get("cacheHit", False)}\tinputs={len(r.get("inputs", []))}\texit={r.get("exitCode", 0)}\twallMs={r.get("metrics", {}).get("executionWallTime", "")}')

def digests(row):
    return {fix(i["path"]): i["digest"]["hash"] for i in row.get("inputs", []) if "digest" in i}

def diff(a, b):
    sa, sb = spawns(a), spawns(b)
    for label in sorted(set(sa) & set(sb)):
        da, db = digests(sa[label][0]), digests(sb[label][0])
        changed = sorted(k for k in set(da) & set(db) if da[k] != db[k])
        added = sorted(set(db) - set(da)); removed = sorted(set(da) - set(db))
        print(label, "changed", len(changed), "added", len(added), "removed", len(removed))
        for k in changed[:10]: print("   changed", k)
        for k in added[:5]: print("   added", k)
        for k in removed[:5]: print("   removed", k)

if __name__ == "__main__":
    if len(sys.argv) == 2: summary(sys.argv[1])
    else: diff(sys.argv[1], sys.argv[2])
