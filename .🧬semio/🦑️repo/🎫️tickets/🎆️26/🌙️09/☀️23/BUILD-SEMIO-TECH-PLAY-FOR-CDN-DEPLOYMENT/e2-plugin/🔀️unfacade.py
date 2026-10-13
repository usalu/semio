#!/usr/bin/env python3
"""🔀️ Rewrites the crate's internal uses of its public `app` vocabulary to the owning modules (wave E2, step 1).

`app` composes the crate's public surface for consumers. Inside the crate every path through it
(`crate::app::X`, `crate::component::app::X`, `super::app::X` from a component-level module) hides the real
dependency, so this tool rewrites each one to the module that owns `X` (or to the foreign path `app` re-exports).
`use` trees are split by owner; globs over `app` (test support) and the crate's own public re-exports stay.
Usage: unfacade.py [--write]
"""
import importlib.util, json, os, re, sys
from collections import OrderedDict, defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
def load(name, file):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, file)); m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m); return m
ri = load("rust_items", "🧱️rust_items.py")
im = load("imports", "🧹️imports.py")
fi = load("facet_imports", "🧭️facet-imports.py")
PLUGIN = im.PLUGIN
ROOT = os.path.join(PLUGIN, "🦀️.rs")


def facade_foreign():
    raw = open(ROOT, encoding="utf-8").read(); cl = ri.clean(raw)
    top, _ = ri.items(raw, cl, 0, len(cl))
    app = next(it for it in top if it["kind"] == "mod" and it["name"] == "app")
    ob = cl.index("{", app["head"]); cb = ri.skip_group(cl, ob) - 1
    out = {}
    for it in ri.items(raw, cl, ob + 1, cb)[0]:
        if it["kind"] != "use": continue
        m = re.match(r"pub\s+use\s+(.*);$", re.sub(r"\s+", " ", cl[it["head"]:it["end"]].strip()))
        if not m or m.group(1).endswith("::*"): continue
        for p, a in im.parse_tree(m.group(1)):
            b = im.bound(p, a)
            if not b: continue
            p = re.sub(r"^super::", "crate::", p)
            out[b] = p + (f" as {a}" if a and a != p.split("::")[-1] else "")
    return out


def resolve(leaf, owners, foreign):
    segs = leaf.split("::")
    x = segs[0]
    if x in foreign:
        base = foreign[x]
        if " as " in base: return base if len(segs) == 1 else None
        return base + ("::" + "::".join(segs[1:]) if len(segs) > 1 else "")
    if x in owners: return owners[x] + "::" + leaf
    return None


def main():
    owners = im.owner_index()
    foreign = facade_foreign()
    fi.walk(f"{PLUGIN}/📦️packages/🦀️rust/🦀️.rs", "crate", False, f"{PLUGIN}/📦️packages/🦀️rust")
    files = {}
    for mod in fi.modules:
        segs = mod.path.split("::")
        files.setdefault(mod.file, set()).add(len(segs) == 3)
    unresolved, changed = defaultdict(int), 0
    for f, levels in files.items():
        if f == ROOT or "/📦️packages/" in f: continue
        raw = open(f, encoding="utf-8").read()
        cl = ri.clean(raw)
        edits = []
        for m in re.finditer(r"\b(pub(?:\s*\([^)]*\))?\s+)?use\s+(?:crate::(?:component::)?app|super::app)::([^;]*);", cl):
            if m.group(0).startswith("super") and True not in levels: continue
            body = raw[m.start(2):m.end(2)]
            if body.strip() == "*": continue
            vis = m.group(1) or ""
            leaves = im.parse_tree(body)
            groups = OrderedDict()
            ok = True
            for p, a in leaves:
                r = resolve(p, owners, foreign)
                if r is None: unresolved[p.split("::")[0]] += 1; ok = False; break
                if " as " in r:
                    r, a2 = r.split(" as "); a = a or a2
                owner, last = r.rsplit("::", 1)
                groups.setdefault(owner, []).append(last + (f" as {a}" if a and a != last else ""))
            if not ok: continue
            stmts = [f"{vis}use {o}::{ls[0]};" if len(ls) == 1 else f"{vis}use {o}::{{{', '.join(ls)}}};" for o, ls in groups.items()]
            indent = raw[raw.rfind("\n", 0, m.start()) + 1:m.start()]
            edits.append((m.start(), m.end(), ("\n" + indent).join(stmts)))
        taken = [(a, b) for a, b, _ in edits]
        for m in re.finditer(r"\b(?:crate::(?:component::)?app|super::app)::(\w+)", cl):
            if any(a <= m.start() < b for a, b in taken): continue
            if m.group(0).startswith("super") and True not in levels: continue
            r = resolve(m.group(1), owners, foreign)
            if r is None or " as " in r: unresolved[m.group(1)] += 1; continue
            edits.append((m.start(), m.end(), r))
        if not edits: continue
        for a, b, t in sorted(edits, reverse=True):
            raw = raw[:a] + t + raw[b:]
        changed += 1
        if "--write" in sys.argv: open(f, "w", encoding="utf-8").write(raw)
    print("files changed", changed)
    print("unresolved", sorted(unresolved.items(), key=lambda x: -x[1])[:40])


if __name__ == "__main__":
    main()
