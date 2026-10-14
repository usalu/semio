#!/usr/bin/env python3
"""🕸️ Engine module dependency graph with comments and literals stripped (lane P1 cut planning).

usage: 🕸️engine-graph.py  -> prints per-module lib/test dependencies on sibling engine modules and the artifact.
"""
import os, re, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
ENGINE = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine"
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "e3b-format-cut", "✂️crate-cut.py"), encoding="utf-8").read().split("census[\"strip\"]")[0].split("CENSUS = ")[0])
def strip(src):
    out, i, n = [], 0, len(src)
    blank = lambda t: re.sub(r"[^\n]", " ", t)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i); j = n if j < 0 else j; out.append(" " * (j - i)); i = j
        elif src.startswith("/*", i):
            d, j = 1, i + 2
            while j < n and d:
                if src.startswith("/*", j): d, j = d + 1, j + 2
                elif src.startswith("*/", j): d, j = d - 1, j + 2
                else: j += 1
            out.append(blank(src[i:j])); i = j
        elif c == "r" and re.match(r'r#*"', src[i:i + 40]) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = re.match(r'r(#*)"', src[i:]); close = '"' + m.group(1); j = src.find(close, i + len(m.group(0))); j = n if j < 0 else j + len(close)
            out.append(blank(src[i:j])); i = j
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"': j += 2 if src[j] == "\\" else 1
            j += 1; out.append(blank(src[i:j])); i = j
        elif c == "'" and re.match(r"'(\\.|[^\\'])'", src[i:i + 4]):
            m = re.match(r"'(\\.|[^\\'])'", src[i:]); out.append(" " * len(m.group(0))); i += len(m.group(0))
        else:
            out.append(c); i += 1
    return "".join(out)
mods = {re.sub(r'^[^a-z_]+', '', d): d for d in sorted(os.listdir(ENGINE)) if os.path.isdir(os.path.join(ENGINE, d))}
graph = {}
for name, d in mods.items():
    lib, test = set(), set()
    for root, _, files in os.walk(os.path.join(ENGINE, d)):
        for f in files:
            if f.endswith(".rs"):
                p = os.path.join(root, f); s = strip(open(p, encoding="utf-8").read())
                refs = set(re.findall(r"\bcrate::([a-zA-Z_0-9]+)", s)) | set(re.findall(r"\buse\s+crate::\{([^}]*)\}", s) and [])
                (test if "🧪️tests" in p else lib).update(refs - {name})
    graph[name] = {"lib": sorted(lib), "test": sorted(test - lib)}
json.dump(graph, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "engine-graph.json"), "w"), indent=1)
for n, g in graph.items(): print(n, "LIB", g["lib"], "TEST", g["test"])
