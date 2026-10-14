#!/usr/bin/env python3
"""🧩️ Plans `semio-framework-plugin` owner crates under the global source admission ceiling.

Input: the module graph of `🧭️facet-imports.py` and the per-module sizes of `📏️unit-size.py`. Strongly connected modules
are condensed first (they can only share one crate); each facet's condensation is then cut into consecutive segments of
one topological order, so every planned crate depends only on earlier crates of its facet and on lower facets, and no
segment exceeds 4,000 active lines / 262,144 bytes / 32 files. A component that alone exceeds a ceiling is reported as
a decomposition obligation (a cycle to break or a god object to split), never packed. It compiles nothing.
Usage: unit-plan.py
"""
import importlib.util, os, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
load = lambda n, f: (lambda s: (s.loader.exec_module(m := importlib.util.module_from_spec(s)), m)[1])(importlib.util.spec_from_file_location(n, os.path.join(HERE, f)))
us = load("unit_size", "📏️unit-size.py")
fi = us.fi
LINES, BYTES, FILES = us.LINES, us.BYTES, us.FILES


def main():
    import io, contextlib, json, tempfile
    tmp = tempfile.mktemp(suffix=".json")
    with contextlib.redirect_stdout(io.StringIO()):
        sys.argv = ["facet-imports", "--json", tmp]; fi.main()
    g = json.load(open(tmp))
    fac, edges = g["facets"], {a: set(b) for a, b in g["edges"].items()}
    fi.modules.clear()
    fi.walk(f"{fi.PLUGIN}/📦️packages/🦀️rust/🦀️.rs", "crate", False, f"{fi.PLUGIN}/📦️packages/🦀️rust")
    files = defaultdict(set)
    for m in fi.modules:
        s = m.path.split("::")
        if not m.test and len(s) >= 3 and m.rng[0] == 0: files[s[2]].add(m.file)
    size = {k: (len(fs), sum(us.active(fi.load(f)[1]) for f in fs), sum(os.path.getsize(f) for f in fs)) for k, fs in files.items()}
    mods = [k for k in size if fac.get(k) in fi.ORDER]
    comps = [c for c in fi.scc(mods, {a: {b for b in bs if b in mods} for a, bs in edges.items()})]
    owner = {m: i for i, c in enumerate(comps) for m in c}
    cedges = defaultdict(set)
    for a, bs in edges.items():
        for b in bs:
            if a in owner and b in owner and owner[a] != owner[b]: cedges[owner[a]].add(owner[b])
    csize = [tuple(sum(size[m][k] for m in c) for k in range(3)) for c in comps]
    order, seen = [], set()

    def visit(i):
        if i in seen: return
        seen.add(i)
        for j in sorted(cedges[i]): visit(j)
        order.append(i)
    for i in sorted(range(len(comps)), key=lambda i: sorted(comps[i])): visit(i)
    units, obligations = [], []
    for f in fi.ORDER:
        cur, tot = [], [0, 0, 0]
        for i in [i for i in order if fac[comps[i][0]] == f]:
            n, l, b = csize[i]
            if n > FILES or l > LINES or b > BYTES:
                obligations.append((f, sorted(comps[i]), csize[i])); continue
            if cur and (tot[0] + n > FILES or tot[1] + l > LINES or tot[2] + b > BYTES):
                units.append((f, cur, tuple(tot))); cur, tot = [], [0, 0, 0]
            cur += sorted(comps[i]); tot = [tot[0] + n, tot[1] + l, tot[2] + b]
        if cur: units.append((f, cur, tuple(tot)))
    for k, (f, ms, (n, l, b)) in enumerate(units, 1):
        print(f"unit {k:2} {f:12} files {n:2} lines {l:5} bytes {b:7}  {', '.join(ms)}")
    print(f"\n{len(units)} crates within the ceiling; {len(obligations)} components need decomposition first:")
    for f, ms, (n, l, b) in obligations:
        print(f"   {f:12} files {n:2} lines {l:6} bytes {b:8}  {', '.join(ms)}")


if __name__ == "__main__":
    main()
