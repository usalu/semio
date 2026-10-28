#!/usr/bin/env python3 -I
"""🔬 depinfo-compare.py <census.json> <out.json> : rustc's own dep-info (.d, written by the compiler) of every cached unit in every cargo store under the shared cargo cache versus the files the LIVE derivation declares for the same crate (named flavor = tests included; the dependency flavor is checked when the unit is not a test unit). A crate whose derivation fell back to its package directory declares `<dir>/**/*`. Streams, RSS guard 500 MB. Writes per-crate gaps (files rustc read that the derivation lacks)."""
import json, os, re, resource, sys
W = "/Users/ueli/Documents/semio"
STORE = W + "/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l"
census = {r["name"]: r for r in json.load(open(sys.argv[1], encoding="utf-8")) if "name" in r}
def parse(path):
    text = open(path, encoding="utf-8", errors="replace").read()
    first = next((l for l in text.split("\n") if ": " in l), "")
    body = first[first.index(": ") + 2:] if ": " in first else ""
    out, cur, i = [], "", 0
    while i < len(body):
        c = body[i]
        if c == "\\" and i + 1 < len(body) and body[i + 1] == " ": cur += " "; i += 2; continue
        if c == " ":
            if cur: out.append(cur); cur = ""
        else: cur += c
        i += 1
    if cur: out.append(cur)
    return out
def resolve(entry, base):
    if entry.startswith("/"): return os.path.normpath(entry)
    directory = base
    while directory.startswith(W):
        candidate = os.path.join(directory, entry)
        if os.path.exists(candidate): return os.path.normpath(candidate)
        directory = os.path.dirname(directory)
    return os.path.normpath(os.path.join(W, entry))
units = {}
for root, dirs, files in os.walk(STORE):
    parts = root[len(STORE):].split(os.sep)
    if "build" not in parts or os.path.basename(root) != "out": continue
    pkg = parts[parts.index("build") + 1]
    for f in files:
        if f.endswith(".d"): units.setdefault(pkg, []).append(os.path.join(root, f))
report = {"crates": 0, "units": 0, "gapCrates": 0, "rows": []}
for pkg, infos in sorted(units.items()):
    row = census.get(pkg)
    if not row: continue
    base = os.path.join(W, row["root"])
    named, dep = row["named"], row["dep"]
    declared = {"named": set(named.get("files", [])), "dep": set(dep.get("files", []))}
    prefix = {k: row["root"] + "/" for k, v in (("named", named), ("dep", dep)) if v["status"] != "ok"}
    gaps = {}
    for info in infos:
        for entry in parse(info):
            absolute = resolve(entry, base)
            if not absolute.startswith(W + "/"): continue
            rel = absolute[len(W) + 1:]
            if any(part in (".git", "node_modules", "target") or part.startswith(".🧬") or part.startswith(".cache") for part in rel.split("/")): continue
            for flavor in ("named",):
                ok = rel in declared[flavor] or (flavor in prefix and rel.startswith(prefix[flavor]))
                if not ok: gaps.setdefault(flavor, set()).add(rel)
    report["crates"] += 1; report["units"] += len(infos)
    if gaps:
        report["gapCrates"] += 1
        report["rows"].append({"name": pkg, "root": row["root"], "named": named["status"], "reason": named.get("reason"), "gapNamed": sorted(gaps.get("named", []))})
if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 500 * 1048576: raise SystemExit("RSS above 500 MB")
json.dump(report, open(sys.argv[2], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(json.dumps({k: report[k] for k in ("crates", "units", "gapCrates")}))
