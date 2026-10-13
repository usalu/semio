#!/usr/bin/env python3
"""🧭️ Module and facet graph of `semio-framework-plugin` from its import paths (wave E2, re-run for E2b).

After the step-1 decomposition every facet module names the crate items it uses through explicit paths
(`use crate::<module>::…`, `crate::<module>::…`, `super::<module>::…`, and `crate::app::<item>` through the
public vocabulary). This tool walks every production module of the crate (test-only modules and
`#[cfg(test)]`-gated imports excluded), resolves every crate path to the module that owns the named item, and
reports the module x module and facet x facet edges, the edges against the facet order, and the cycles.

Facets: destination modules carry theirs (`🗂️destinations.json`); every other module gets one from the file
rules of `🗺️facets.json`; its `modules` map (E2b) states the facet of a module whose directory rule would mislead. It compiles nothing.
E2b changes against `e2-plugin/`: the splitter is E1's generic-brace-fixed `e1-os-kernel/🧱️rust_items.py`; `mod` heads
inside `macro_rules!` bodies are expanded in the invoking crate, not here, so they are skipped; the owner index only
holds module-level definitions (methods and locals no longer shadow extern aliases such as `crate::store`); every
attribute stacked before a `mod` is read, `#[cfg(test)]` items with `async`/`const`/`unsafe` heads are stripped too, so `#[cfg(test)] #[path = …] mod …;` test modules are excluded (E2's reader saw
only the attribute next to the head).
Usage: facet-imports.py [--edges] [--back] [--modules] [--json OUT]
"""
import importlib.util, json, os, re, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rust_items", os.path.join(os.path.dirname(HERE), "e1-os-kernel", "🧱️rust_items.py"))
ri = importlib.util.module_from_spec(spec); spec.loader.exec_module(ri)

REPO = "/Users/ueli/Documents/semio"
PLUGIN = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
DEST = json.load(open(os.path.join(HERE, "🗂️destinations.json"), encoding="utf-8"))
RULES = json.load(open(os.path.join(HERE, "🗺️facets.json"), encoding="utf-8"))
ORDER = ["ownership", "window", "declaration", "publication", "jobs", "reactor", "instance"]
MOD_RE = re.compile(r"(?<![\w:])(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+(\w+)\s*([;{])")
INC_RE = re.compile(r'include!\s*\(\s*"([^"]+)"\s*\)')
PATH_RE = re.compile(r"\b(crate|super)((?:::\w+)+)")
DEF_RE = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+|async\s+|unsafe\s+)*(?:fn|struct|enum|trait|type|const|static|union|mod)\s+(\w+)", re.M)


def attrs_before(raw, pos):
    out, head = [], raw[:pos]
    while True:
        m = re.search(r"#\[((?:[^\[\]]|\[[^\[\]]*\])*)\]\s*$", head[-2000:])
        if not m: return out
        out.append(m.group(1)); head = head[:len(head) - (len(head[-2000:]) - m.start())]


def gated(attrs):
    return any(re.match(r"\s*cfg\s*\(.*\btest\b", a) and "not(test" not in a for a in attrs)


class Module:
    def __init__(s, path, file, test, rng):
        s.path, s.file, s.test, s.rng = path, file, test, rng


modules = []
cache = {}


def load(f):
    if f not in cache:
        raw = open(f, encoding="utf-8").read()
        cache[f] = (raw, ri.clean(raw))
    return cache[f]


def walk(f, path, test, base):
    raw, cl = load(f)
    modules.append(Module(path, f, test, (0, len(cl))))
    region(f, raw, cl, 0, len(cl), path, test, base)


def macro_spans(cl, a, b):
    out = []
    for m in re.finditer(r"macro_rules!\s*\w+\s*\{", cl[a:b]):
        ob = a + m.end() - 1
        out.append((ob, ri.skip_group(cl, ob)))
    return out


def region(f, raw, cl, a, b, path, test, base):
    i = a
    skip = macro_spans(cl, a, b)
    inline = []
    while True:
        m = MOD_RE.search(cl, i, b)
        if not m: break
        hole = next((y for x, y in skip if x <= m.start() < y), None)
        if hole: i = hole; continue
        name, term = m.group(1), m.group(2)
        attrs = attrs_before(raw, m.start())
        t = test or gated(attrs)
        p = next((re.match(r'\s*path\s*=\s*"([^"]*)"', x).group(1) for x in attrs if re.match(r'\s*path\s*=', x)), None)
        if term == ";":
            target = os.path.normpath(os.path.join(base, p)) if p else None
            if target and os.path.exists(target):
                walk(target, f"{path}::{name}", t, os.path.dirname(target))
            i = m.end()
        else:
            ob = m.end() - 1; cb = ri.skip_group(cl, ob) - 1
            inline.append((ob, cb))
            modules.append(Module(f"{path}::{name}", f, t, (ob, cb)))
            region(f, raw, cl, ob + 1, cb, f"{path}::{name}", t, os.path.normpath(os.path.join(base, p if p is not None else name)))
            i = cb + 1
    for m in INC_RE.finditer(cl, a, b):
        if any(x <= m.start() <= y for x, y in inline + skip): continue
        inc = os.path.normpath(os.path.join(os.path.dirname(f), raw[m.start(1):m.end(1)]))
        if os.path.exists(inc):
            walk(inc, path, test or gated(attrs_before(raw, m.start())), os.path.dirname(inc))


depth_cache = {}


def depth_map(f):
    if f not in depth_cache:
        cl = load(f)[1]
        d, out = 0, [0] * (len(cl) + 1)
        for i, ch in enumerate(cl):
            if ch == "}": d -= 1
            out[i] = d
            if ch == "{": d += 1
        depth_cache[f] = out
    return depth_cache[f]


def facet_of(mod):
    rel = os.path.relpath(mod.file, PLUGIN)
    top = mod.path.split("::")
    if len(top) >= 3 and top[1] == "component" and top[2] in RULES.get("modules", {}): return RULES["modules"][top[2]]
    for d in DEST["destinations"]:
        if rel == f"{d['dir']}/🦀️.rs": return d["facet"]
    top = mod.path.split("::")
    if len(top) >= 3 and top[1] == "component":
        for d in DEST["destinations"]:
            if top[2] == d["module"]: return d["facet"]
    for rx, f in RULES["files"]:
        if re.search(rx, rel): return f
    if rel == "🦀️.rs": return {"wasip2": "reactor", "owned_abi": "reactor", "component_persistent_local": "ownership", "plugin_app_close_prelude": "reactor", "__subset_registration": "declaration"}.get(top[2] if len(top) >= 3 else "", "root")
    return "other"


def main():
    args = sys.argv[1:]
    walk(f"{PLUGIN}/📦️packages/🦀️rust/🦀️.rs", "crate", False, f"{PLUGIN}/📦️packages/🦀️rust")
    owner = {}
    top_mods = {}
    for mod in modules:
        segs = mod.path.split("::")
        if len(segs) == 3 and segs[2] not in top_mods: top_mods[segs[2]] = mod
    for mod in modules:
        if mod.test: continue
        raw, cl = load(mod.file)
        segs = mod.path.split("::")
        key = segs[2] if len(segs) >= 3 else "root"
        if key == "app": continue
        dm = depth_map(mod.file)
        base_depth = dm[mod.rng[0]] + (1 if cl[mod.rng[0]:mod.rng[0] + 1] == "{" else 0)
        for m in DEF_RE.finditer(cl, mod.rng[0], mod.rng[1]):
            if dm[m.start()] == base_depth: owner.setdefault(m.group(1), set()).add(key)
    edges = defaultdict(lambda: defaultdict(set))
    for mod in modules:
        if mod.test: continue
        raw, cl = load(mod.file)
        segs = mod.path.split("::")
        src = segs[2] if len(segs) >= 3 else "root"
        if src == "app": continue
        text = cl[mod.rng[0]:mod.rng[1]]
        for m in re.finditer(r"#\[\s*cfg\s*\([^\]]*\btest\b[^\]]*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:(?:const|async|unsafe)\s+)*(?:use|mod|fn|impl|struct|enum|const|static|type|trait)\b[^;{]*(?:;|\{)", text):
            start = mod.rng[0] + m.start()
            end = start + len(m.group(0)) if m.group(0).endswith(";") else ri.skip_group(cl, start + len(m.group(0)) - 1)
            text = text[:start - mod.rng[0]] + " " * (end - start) + text[end - mod.rng[0]:]
        for um in re.finditer(r"\b(crate|super)::((?:\w+::)*)\{([^;]*)\};", text):
            if um.group(1) == "super" and len(segs) != 3: continue
            pre = [x for x in um.group(2).split("::") if x]
            if pre and pre[0] == "component": pre = pre[1:]
            leaves = []
            def expand(prefix, body):
                depth, cur, parts = 0, "", []
                for ch in body:
                    if ch == "{": depth += 1
                    if ch == "}": depth -= 1
                    if ch == "," and depth == 0: parts.append(cur); cur = ""
                    else: cur += ch
                parts.append(cur)
                for p_ in parts:
                    p_ = p_.strip()
                    if not p_: continue
                    mm = re.match(r"((?:\w+::)*)\{(.*)\}$", p_, re.S)
                    if mm: expand(prefix + [x for x in mm.group(1).split("::") if x], mm.group(2)); continue
                    leaves.append(prefix + p_.split(" as ")[0].strip().split("::"))
            expand(pre, um.group(3))
            for path in leaves:
                if not path or path[-1] in ("self", "*"): continue
                if path[0] in ("app", "plugin_app_close_prelude"):
                    tos = owner.get(path[-1], set())
                    if len(tos) == 1 and next(iter(tos)) != src: edges[src][next(iter(tos))].add(path[-1])
                elif path[0] in top_mods and path[0] != src:
                    edges[src][path[0]].add(path[-1] if len(path) > 1 else path[0])
        for pm in PATH_RE.finditer(text):
            root, rest = pm.group(1), pm.group(2).strip(":").split("::")
            if root == "super" and len(segs) != 3: continue
            if rest and rest[0] == "component": rest = rest[1:]
            if not rest: continue
            if rest[0] in ("app", "plugin_app_close_prelude"):
                if len(rest) < 2: continue
                name = rest[1]
                tos = owner.get(name, set())
                if len(tos) == 1: dst = next(iter(tos))
                else: continue
            elif rest[0] in top_mods:
                dst = rest[0]; name = rest[1] if len(rest) > 1 else rest[0]
            else:
                tos = owner.get(rest[0], set())
                if len(tos) != 1: continue
                dst = next(iter(tos)); name = rest[0]
            if dst != src: edges[src][dst].add(name)
        for um in re.finditer(r"\buse\s+(crate|super)::\{([^;]*)\};", text):
            if um.group(1) == "super" and len(segs) != 3: continue
            for leaf in re.split(r"[,{}]", um.group(2)):
                leaf = leaf.strip().split("::")[0].split(" as ")[0].strip()
                if leaf in top_mods and leaf != src: edges[src][leaf].add(leaf)
    fac = {}
    for mod in modules:
        segs = mod.path.split("::")
        k = segs[2] if len(segs) >= 3 else "root"
        if k not in fac: fac[k] = facet_of(top_mods.get(k, mod))
    if "--modules" in args:
        for k in sorted(fac, key=lambda k: (ORDER.index(fac[k]) if fac[k] in ORDER else 9, k)): print(f"{fac[k]:12} {k}")
        return
    fe = defaultdict(lambda: defaultdict(int))
    back = []
    for a, tos in edges.items():
        for b, names in tos.items():
            fa, fb = fac.get(a, "other"), fac.get(b, "other")
            if fa != fb: fe[fa][fb] += len(names)
            if fa in ORDER and fb in ORDER and ORDER.index(fb) > ORDER.index(fa):
                back.append((fa, a, fb, b, sorted(names)))
    print("## facet -> facet (distinct imported names); '!' = against the order " + " < ".join(ORDER))
    for fa in ORDER + ["root", "other"]:
        row = fe.get(fa, {})
        if row: print(f"{fa:12} -> " + ", ".join(f"{'!' if fa in ORDER and fb in ORDER and ORDER.index(fb) > ORDER.index(fa) else ''}{fb}:{n}" for fb, n in sorted(row.items(), key=lambda x: -x[1])))
    adj = {fa: {fb for fb in row} for fa, row in fe.items()}
    print("\n## facet cycles")
    for comp in scc(list(fe), adj):
        if len(comp) > 1: print("  ", " <-> ".join(sorted(comp, key=lambda f: ORDER.index(f) if f in ORDER else 9)))
    madj = {a: set(tos) for a, tos in edges.items()}
    print("\n## module cycles (strongly connected modules)")
    for comp in sorted(scc(list(fac), madj), key=len, reverse=True):
        if len(comp) > 1: print(f"   {len(comp)}: " + ", ".join(sorted(comp)))
    if "--back" in args:
        print("\n## edges against the order")
        for fa, a, fb, b, names in sorted(back):
            print(f"   {fa}:{a} -> {fb}:{b}  [{len(names)}] {', '.join(names[:12])}")
    if "--edges" in args:
        print("\n## module edges")
        for a in sorted(edges):
            for b in sorted(edges[a]):
                print(f"   {a} -> {b}: {', '.join(sorted(edges[a][b])[:10])}")
    if "--json" in args:
        json.dump({"facets": fac, "edges": {a: {b: sorted(n) for b, n in tos.items()} for a, tos in edges.items()}}, open(args[args.index("--json") + 1], "w"), ensure_ascii=False, indent=1)


def scc(nodes, adj):
    sys.setrecursionlimit(100000)
    index, low, on, st, out, c = {}, {}, set(), [], [], [0]

    def visit(v):
        index[v] = low[v] = c[0]; c[0] += 1; st.append(v); on.add(v)
        for w in adj.get(v, ()):
            if w not in index: visit(w); low[v] = min(low[v], low[w])
            elif w in on: low[v] = min(low[v], index[w])
        if low[v] == index[v]:
            comp = []
            while True:
                w = st.pop(); on.discard(w); comp.append(w)
                if w == v: break
            out.append(comp)
    for v in nodes:
        if v not in index: visit(v)
    return out


if __name__ == "__main__":
    main()
