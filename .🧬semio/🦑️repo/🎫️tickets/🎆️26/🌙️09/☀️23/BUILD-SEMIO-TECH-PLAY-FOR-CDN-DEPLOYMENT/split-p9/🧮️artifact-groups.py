#!/usr/bin/env python3
"""🧮️ Mutation leaves of any `#[path]`-scaffold artifact: enum order, touched snapshot fields, emitted leaves; union-find
groups packed into ≤ 10-leaf chunks in enum order (the curation input for group names).

usage: 🧮️artifact-groups.py <artifact dir (repo-relative)>... [--json out.json]
"""
import glob, json, os, re, sys, collections
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
out = {}
for art in [a for a in sys.argv[1:] if not a.startswith("--") and not a.endswith(".json")]:
    A = os.path.join(REPO, art)
    root_src = open(os.path.join(A, "🦀️.rs"), encoding="utf-8").read()
    mroots = sorted(glob.glob(f"{A}/🏅️standards/*/🪆️subsets/*/🧬️schema/🧬️mutations/🦀️.rs"))
    leafdir = {m.group(1): m.group(2) for m in re.finditer(r'pub mod (\w+)\s*\{\s*#\[path\s*=\s*"[^"]*🧬️mutations/([^/"]+)/', root_src) if m.group(2) not in ("🧭️edit-rules", "🧪️tests")}
    variant_of, order = {}, []
    for mr in mroots:
        src = open(mr, encoding="utf-8").read()
        for v, mod, ty in re.findall(r"^\s+([A-Z][A-Za-z0-9]*)\((?:([a-z0-9_]+)(?:::mutation)?::)?([A-Za-z0-9]+)\),", src, re.M):
            if not mod:
                mod = next((m for m, d in leafdir.items() if any(re.search(r"pub struct " + ty + r"\b", open(f, encoding="utf-8").read()) for f in glob.glob(f"{A}/🏅️standards/*/🪆️subsets/*/🧬️schema/🧬️mutations/{d}/**/🦀️.rs", recursive=True))), None)
            if mod:
                variant_of[mod] = v; order.append(mod)
    parent = {l: l for l in order}
    def find(x):
        while parent[x] != x: parent[x] = parent[parent[x]]; x = parent[x]
        return x
    fields = {}
    for l in order:
        d = glob.glob(f"{A}/🏅️standards/*/🪆️subsets/*/🧬️schema/🧬️mutations/{leafdir.get(l, '?')}")
        text = "".join(open(f, encoding="utf-8").read() for dd in d for f in glob.glob(dd + "/**/🦀️.rs", recursive=True) if "🧪️tests" not in f)
        fields[l] = sorted(set(re.findall(r"\bbase\s*\.\s*([a-z_][a-z0-9_]*)", text)))
        for v in re.findall(r"\b[A-Z][A-Za-z0-9]*Mutation::([A-Z][A-Za-z0-9]*)\s*\(", text):
            for mod, vv in variant_of.items():
                if vv == v and mod != l and mod in parent: parent[find(l)] = find(mod)
    seq, seen, groups = [], set(), collections.defaultdict(list)
    for l in order: groups[find(l)].append(l)
    for l in order:
        if find(l) not in seen: seen.add(find(l)); seq.append(groups[find(l)])
    crates, cur = [], []
    for g in seq:
        if len(cur) + len(g) > 10 and cur: crates.append(cur); cur = []
        cur += g
    if cur: crates.append(cur)
    out[art] = {"crates": crates, "fields": fields, "leafdir": leafdir, "mroots": [os.path.relpath(m, REPO) for m in mroots]}
    print(f"== {art}: {len(order)} leaves, {len(mroots)} mutation roots, {len(crates)} chunks")
    for c in crates: print("   ", " ".join(f"{l}[{','.join(fields[l][:2])}]" for l in c))
if "--json" in sys.argv: json.dump(out, open(sys.argv[sys.argv.index("--json") + 1], "w"), ensure_ascii=False, indent=1)
