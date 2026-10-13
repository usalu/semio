#!/usr/bin/env python3
"""🧲️ Lists every `impl` whose self type is defined in a different facet than the impl (inherent impls must move with
their type; trait impls need the trait or the type local after a crate cut). Uses E1's `🧱️rust_items.py` splitter over
every non-test file of the crate and the facet file rules of an `🕸️item-graph.py` facets JSON.
usage: 🧲️impl-homes.py <facets.json>"""
import importlib.util, json, os, re, sys
from collections import defaultdict
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rust_items", os.path.join(HERE, "..", "e1-os-kernel", "🧱️rust_items.py"))
ri = importlib.util.module_from_spec(spec); spec.loader.exec_module(ri)
REPO = "/Users/ueli/Documents/semio"; CFG = json.load(open(sys.argv[1])); BASE = os.path.join(REPO, CFG["base"])
def facet(rel):
    for rx, f in CFG["files"]:
        if re.search(rx, rel): return f
    return "other"
defs = defaultdict(set); impls = []
DEF = re.compile(r"(?<![\w])(?:pub(?:\s*\([^)]*\))?\s+)?(?:struct|enum|trait|union|type)\s+(\w+)")
for root, ds, fs in os.walk(BASE):
    if "🧪️tests" in root or "/target" in root: continue
    for f in fs:
        if not f.endswith(".rs"): continue
        p = os.path.join(root, f); rel = os.path.relpath(p, BASE); fa = facet(rel)
        raw = open(p, encoding="utf-8").read(); cl = ri.clean(raw)
        for m in DEF.finditer(cl): defs[m.group(1)].add(fa)
        for m in re.finditer(r"(?<![\w])impl\b", cl):
            e = cl.find("{", m.start()); s2 = cl.find(";", m.start())
            if e < 0: continue
            ty, tr = ri.impl_target(cl, m.start(), e + 1)
            impls.append((fa, rel, ri.line_of(raw, m.start()), ty, tr))
for fa, rel, line, ty, tr in impls:
    homes = defs.get(ty, set())
    if homes and fa not in homes:
        trh = defs.get(tr, set()) if tr else set()
        kind = "INHERENT" if not tr else ("trait-local" if fa in trh else "ORPHAN?")
        print(f"{kind:12} {fa:14} impl {tr or ''} for {ty} (type in {','.join(sorted(homes))})  {rel}:{line}")
