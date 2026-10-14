#!/usr/bin/env python3
"""🔍️ Heuristic orphan-rule scan over a crate family: every `impl Trait for Type` whose trait and type are both
defined in other crates of the family (or the type in another family crate and the trait outside every family crate).

usage: 🔍️orphans.py <workspace Cargo.toml>...
"""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__)); REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
g = {}
exec(compile(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], "mg", "exec"), g)
crates = {}
for ws in sys.argv[1:]:
    wsd = os.path.dirname(os.path.join(REPO, ws))
    wtext = open(os.path.join(REPO, ws), encoding="utf-8").read()
    for m in re.findall(r'"([^"]+)"', re.search(r'members\s*=\s*\[(.*?)\]', wtext, re.S).group(1)):
        t = open(os.path.join(wsd, m, "Cargo.toml"), encoding="utf-8").read()
        name = re.search(r'^name\s*=\s*"([^"]+)"', t, re.M).group(1)
        lp = re.search(r'\[lib\][^\[]*?path\s*=\s*"([^"]+)"', t, re.S)
        lib = os.path.normpath(os.path.join(wsd, m, lp.group(1) if lp else "src/lib.rs"))
        try: crates[name] = g["Crate"](lib)
        except Exception as e: print("skip", m, e)
types, traits = {}, {}
for n, c in crates.items():
    for f in c.files:
        s = c.text[f]
        for m in re.finditer(r'\b(?:struct|enum|type|union)\s+([A-Z]\w*)', s): types.setdefault(m.group(1), set()).add(n)
        for m in re.finditer(r'\btrait\s+([A-Z]\w*)', s): traits.setdefault(m.group(1), set()).add(n)
IMPL = re.compile(r'\bimpl\s*(?:<[^{;]*?>)?\s*([:\w]+)\s*(?:<[^{;]*?>)?\s+for\s+([:\w]+)')
for n, c in sorted(crates.items()):
    for f in c.files:
        if "🧪️tests" in f: continue
        for m in IMPL.finditer(c.text[f]):
            tr, ty = m.group(1).split("::")[-1], m.group(2).split("::")[-1]
            tyc, trc = types.get(ty, set()), traits.get(tr, set())
            if tyc and n not in tyc and n not in trc:
                print(f"{n}: impl {tr} for {ty}  (type in {sorted(tyc)}, trait in {sorted(trc) or 'external'})  {os.path.relpath(f, REPO)}")
