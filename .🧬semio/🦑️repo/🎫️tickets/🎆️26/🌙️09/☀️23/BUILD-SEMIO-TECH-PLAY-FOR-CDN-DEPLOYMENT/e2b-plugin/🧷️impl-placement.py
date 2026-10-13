#!/usr/bin/env python3
"""🧷️ Checks that every `impl` of `semio-framework-plugin` could stay where it is once each facet is its own crate.

An inherent `impl X` must sit in the facet that defines `X`. A trait impl must sit in the facet of its trait or of its
self type (the orphan rule, per future facet crate); one whose trait and self type are both external is legal only
because the plugin crate itself defines neither, which the split cannot change. Names resolve through the module-level
definitions of the crate, a definition in the impl's own module first (`🕸️item-graph.py`); production modules only. It compiles nothing.
Usage: impl-placement.py
"""
import importlib.util, os, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("item_graph", os.path.join(HERE, "🕸️item-graph.py"))
ig = importlib.util.module_from_spec(spec); spec.loader.exec_module(ig)
fi, ri = ig.fi, ig.ri


def main():
    fi.walk(f"{fi.PLUGIN}/📦️packages/🦀️rust/🦀️.rs", "crate", False, f"{fi.PLUGIN}/📦️packages/🦀️rust")
    top = {}
    for m in fi.modules:
        s = m.path.split("::")
        if len(s) == 3 and s[2] not in top: top[s[2]] = m
    fac = {k: fi.facet_of(v) for k, v in top.items()}
    defs, impls = defaultdict(set), []
    for m in fi.modules:
        if m.test: continue
        s = m.path.split("::")
        if len(s) < 3 or s[2] == "app": continue
        raw, cl = fi.load(m.file)
        a, b = m.rng
        a += 1 if cl[a:a + 1] == "{" else 0
        holes = [c.rng for c in fi.modules if c.file == m.file and c is not m and a <= c.rng[0] < b]
        its, _ = ri.items(raw, cl, a, b)
        for it in its:
            if any(x <= it["head"] <= y for x, y in holes): continue
            head = raw[max(0, it["head"] - 300):it["head"]]
            if fi.re.search(r"#\[\s*cfg\s*\(\s*test\s*\)\s*\]\s*$", head): continue
            if it["kind"] in ("struct", "enum", "trait", "type", "union") and it["name"]: defs[it["name"]].add(s[2])
            if it["kind"] == "impl": impls.append((s[2], it.get("self"), it.get("trait"), os.path.relpath(m.file, fi.PLUGIN), ri.line_of(raw, it["head"])))
    owner = lambda n, here_mod: (here_mod if here_mod in defs.get(n, ()) else next(iter(defs[n])) if n in defs and len(defs[n]) == 1 else None)
    bad = []
    for mod, ty, tr, f, line in impls:
        here = fac.get(mod)
        tm, rm = owner(ty, mod) if ty else None, owner(tr, mod) if tr else None
        tf, rf = fac.get(tm) if tm else None, fac.get(rm) if rm else None
        ok = (tf == here) if not tr else (tf == here or rf == here or (tm is None and rm is None))
        if not ok: bad.append(f"{here}:{mod} impl {tr + ' for ' if tr else ''}{ty}  (type: {tf}:{tm}, trait: {rf}:{rm})  {f}:{line}")
    for b in sorted(bad): print("  ", b)
    print(f"{len(bad)} misplaced impls of {len(impls)}")


if __name__ == "__main__":
    main()
