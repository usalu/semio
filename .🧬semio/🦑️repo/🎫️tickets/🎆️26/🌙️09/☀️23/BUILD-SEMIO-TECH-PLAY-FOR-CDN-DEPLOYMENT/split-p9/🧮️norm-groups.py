#!/usr/bin/env python3
"""🧮️ Proposes mutation leaf groups per norm artifact: leaves joined by the snapshot fields their diff/inverse touch and by
the leaves their inverse emits (union-find), packed in enum declaration order into crates of at most 10 leaves.

usage: 🧮️norm-groups.py [--json out.json] [artifact-dir-glob]
"""
import glob, json, os, re, sys, collections
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
def leaf_ident(d): return re.sub(r"^[^a-z]*", "", d).replace("-", "_")
out = {}
for art in sorted(os.listdir(ROOT)):
    mroot = glob.glob(f"{ROOT}/{art}/🏅️standards/*/🪆️subsets/✳️any/🧬️schema/🧬️mutations")[0]
    enum_src = open(f"{mroot}/🦀️.rs", encoding="utf-8").read()
    order = re.findall(r"^\s+([A-Z][A-Za-z0-9]*)\(([a-z0-9_]+)::([A-Za-z0-9]+)\),", enum_src, re.M)
    variant_of = {mod: var for var, mod, _ in order}
    root_src = open(f"{ROOT}/{art}/🦀️.rs", encoding="utf-8").read()
    leaves = {m.group(1): m.group(2) for m in re.finditer(r'pub mod (\w+)\s*\{\s*#\[path\s*=\s*"[^"]*🧬️mutations/([^/"]+)/🦀️\.rs"\]', root_src)}
    parent = {l: l for l in leaves}
    def find(x):
        while parent[x] != x: parent[x] = parent[parent[x]]; x = parent[x]
        return x
    def union(a, b): parent[find(a)] = find(b)
    fields = collections.defaultdict(set); emits = collections.defaultdict(set)
    for l, d in leaves.items():
        text = ""
        for f in ("🦀️.rs", "🔺️diff/🦀️.rs", "↩️inverse/🦀️.rs"):
            p = f"{mroot}/{d}/{f}"
            if os.path.exists(p): text += open(p, encoding="utf-8").read()
        for m in re.finditer(r"\bbase\s*\.\s*([a-z_][a-z0-9_]*)", text): fields[l].add(m.group(1))
        for m in re.finditer(r"\b(?:[A-Z][A-Za-z0-9]*Mutation)::([A-Z][A-Za-z0-9]*)\s*\(", text):
            var = m.group(1)
            for mod, v in variant_of.items():
                if v == var and mod != l: emits[l].add(mod)
    by_field = collections.defaultdict(list)
    for l, fs in fields.items():
        for f in fs: by_field[f].append(l)
    for l, es in emits.items():
        for e in es:
            if e in parent: union(l, e)
    groups = collections.defaultdict(list)
    for var, mod, _ in order:
        if mod in parent: groups[find(mod)].append(mod)
    seq = []
    seen = set()
    for var, mod, _ in order:
        g = find(mod)
        if g not in seen: seen.add(g); seq.append(groups[g])
    crates, cur = [], []
    for g in seq:
        if len(cur) + len(g) > 10 and cur: crates.append(cur); cur = []
        cur = cur + g
    if cur: crates.append(cur)
    out[art] = {"leaves": len(order), "groups": [sorted(set().union(*[fields[l] for l in g])) and g for g in seq], "crates": crates, "fields": {l: sorted(fields[l]) for l in leaves}, "emits": {l: sorted(e) for l, e in emits.items()}}
    big = max([len(g) for g in seq] or [0])
    print(f"{art:14} leaves={len(order):3} entity-groups={len(seq):3} biggest={big:2} crates={len(crates)}  sizes={[len(c) for c in crates]}")
if "--json" in sys.argv: json.dump(out, open(sys.argv[sys.argv.index("--json") + 1], "w"), indent=1, ensure_ascii=False)
