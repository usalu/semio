#!/usr/bin/env python3
"""Scan every tracked+untracked TS-like file, resolve import specifiers, write edges JSON {file: [[spec, resolved|null], ...]}."""
import json, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
OUT = sys.argv[1]
EXT = (".ts", ".tsx", ".mts", ".cts", ".mjs", ".cjs", ".js")
SPEC = re.compile(r"""(?:\bfrom|\bimport|\brequire\s*\(|\bimport\s*\()\s*["']([^"'\n]+)["']""")


def files():
    a = subprocess.run(["git", "-c", "core.fsmonitor=false", "ls-files", "-z"], cwd=ROOT, capture_output=True).stdout
    b = subprocess.run(["git", "-c", "core.fsmonitor=false", "ls-files", "-o", "--exclude-standard", "-z"], cwd=ROOT, capture_output=True).stdout
    out = set()
    for blob in (a, b):
        for p in blob.split(b"\0"):
            if not p:
                continue
            s = p.decode("utf8", "surrogateescape")
            if s.startswith(".🧬semio/") or "/node_modules/" in s:
                continue
            if s.endswith(EXT):
                out.add(s)
    return sorted(out)


def resolve(src, spec):
    if not spec.startswith("."):
        return None
    base = os.path.normpath(os.path.join(os.path.dirname(src), spec))
    for cand in (base, base[:-3] + ".ts" if base.endswith(".js") else base):
        if os.path.isfile(os.path.join(ROOT, cand)):
            return cand
    for ext in (".ts", ".tsx", ".mts", ".mjs", ".js", ".cjs"):
        if os.path.isfile(os.path.join(ROOT, base + ext)):
            return base + ext
    return base


edges = {}
for f in files():
    try:
        text = open(os.path.join(ROOT, f), encoding="utf8", errors="surrogateescape").read()
    except OSError:
        continue
    rows = []
    for m in SPEC.finditer(text):
        s = m.group(1)
        rows.append([s, resolve(f, s)])
    if rows:
        edges[f] = rows
json.dump(edges, open(OUT, "w", encoding="utf8", errors="surrogateescape"), ensure_ascii=False)
print(len(edges), "files")
