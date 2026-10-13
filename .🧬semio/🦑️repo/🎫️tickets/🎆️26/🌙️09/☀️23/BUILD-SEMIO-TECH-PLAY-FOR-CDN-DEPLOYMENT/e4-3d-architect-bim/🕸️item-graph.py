#!/usr/bin/env python3
"""🕸️ Item and facet dependency graph of one `#[path]`-tree crate (wave E4), generalised from E2's `🕸️plugin-module-graph.py`.

Walks the mounted module tree from the lib root (`#[path]`, inline `mod {}`, `include!`). Every module larger than
`--split` lines is split into its top-level items with E1's `🧱️rust_items.py` (generic-brace fix); `impl` blocks join
their local self type (or local trait). Every node gets a facet from the facet rules (item-name rules, then file regex
rules on the path relative to the crate base directory). References are resolved by unique name (types, functions,
macros, constants), as in E2. It reports:

- lines per facet (prod, test), the facet x facet symbol matrix and edges against `allow`, facet SCCs, node SCCs
- `--edge A B`: the symbols behind a facet edge, node by node
- `--nodes`: every node with facet, kind, lines and location
- `--json OUT`: the whole graph

It compiles nothing.
usage: 🕸️item-graph.py <facets.json> [--split N] [--edge A B] [--nodes] [--json OUT]
facets.json: {"root": <repo-relative lib root>, "base": <repo-relative dir that file rules are relative to>,
              "order": [facet...], "allow": {facet: [facet...]}, "names": [[label-regex, facet]...],
              "files": [[path-regex, facet]...]}
"""
import importlib.util, json, os, re, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rust_items", os.path.join(HERE, "..", "e1-os-kernel", "🧱️rust_items.py"))
ri = importlib.util.module_from_spec(spec); spec.loader.exec_module(ri)

REPO = "/Users/ueli/Documents/semio"
CFG = json.load(open(sys.argv[1], encoding="utf-8"))
ROOT = os.path.join(REPO, CFG["root"])
BASE = os.path.join(REPO, CFG["base"])

MOD_RE = re.compile(r"(?<![\w:])(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+(r#)?(\w+)\s*([;{])")
INC_RE = re.compile(r'include!\s*\(\s*"([^"]+)"\s*\)')
DEF_RE = re.compile(r"(?<![\w])(?:struct|enum|trait|union|type|fn|const|static)\s+(\w+)|macro_rules!\s*(\w+)")
IDENT_RE = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
COMMON = {"new", "default", "from", "into", "clone", "fmt", "drop", "len", "is_empty", "get", "step", "main", "tests", "T", "Self", "Ok", "Err", "Some", "None", "A", "M", "S", "P", "C", "D", "E", "V", "K", "N"}

cache = {}


def load(f):
    if f not in cache:
        raw = open(f, encoding="utf-8").read()
        cache[f] = (raw, ri.clean(raw))
    return cache[f]


def attrs_before(raw, cl, pos):
    attrs, j = [], pos
    while True:
        k = j - 1
        while k >= 0 and cl[k].isspace(): k -= 1
        if k < 0 or cl[k] != "]": break
        depth, s = 0, k
        while s >= 0:
            if cl[s] == "]": depth += 1
            elif cl[s] == "[":
                depth -= 1
                if depth == 0: break
            s -= 1
        if s <= 0 or cl[s - 1] != "#": break
        attrs.append(raw[s - 1:k + 1]); j = s - 1
    return attrs


def cfg_kind(attrs, inherited):
    k = inherited
    for a in attrs:
        if a.startswith("#[cfg("):
            body = a[6:-2]
            if re.search(r"\btest\b", body) and "not(test" not in body and "not (test" not in body: k = "test"
    return k


class Mod:
    def __init__(s, path, file, kind, rng):
        s.path, s.file, s.kind, s.rng, s.children, s.includes = path, file, kind, rng, [], []


mods = []


def walk_file(f, modpath, kind):
    raw, cl = load(f)
    root = Mod(modpath, f, kind, (0, len(cl)))
    mods.append(root)
    walk_region(f, raw, cl, 0, len(cl), modpath, kind, os.path.dirname(f), root)
    return root


def walk_region(f, raw, cl, a, b, modpath, kind, base_dir, parent):
    i = a
    inline = []
    while True:
        m = MOD_RE.search(cl, i, b)
        if not m: break
        name, term = m.group(2), m.group(3)
        attrs = attrs_before(raw, cl, m.start())
        k = cfg_kind(attrs, kind)
        p = None
        for at in attrs:
            pm = re.match(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]', at)
            if pm: p = pm.group(1)
        if term == ";":
            target = os.path.normpath(os.path.join(base_dir, p)) if p else None
            if not target:
                for cand in (os.path.join(base_dir, name + ".rs"), os.path.join(base_dir, name, "mod.rs")):
                    if os.path.exists(cand): target = cand; break
            if target and os.path.exists(target):
                parent.children.append(walk_file(target, f"{modpath}::{name}", k))
            else:
                print(f"[warn] unresolved mod {modpath}::{name} -> {target}", file=sys.stderr)
            i = m.end()
        else:
            ob = m.end() - 1
            cb = ri.skip_group(cl, ob) - 1
            d = os.path.normpath(os.path.join(base_dir, p if p is not None else name))
            child = Mod(f"{modpath}::{name}", f, k, (ob, cb))
            mods.append(child); parent.children.append(child); inline.append((ob, cb))
            walk_region(f, raw, cl, ob + 1, cb, child.path, k, d, child)
            i = cb + 1
    for m in INC_RE.finditer(cl, a, b):
        if any(x < m.start() < y for x, y in inline): continue
        inc = os.path.normpath(os.path.join(os.path.dirname(f), raw[m.start(1):m.end(1)]))
        parent.includes.append((m.start(), m.end()))
        if os.path.exists(inc):
            parent.children.append(walk_file(inc, modpath + "#include:" + os.path.relpath(inc, BASE), cfg_kind(attrs_before(raw, cl, m.start()), kind)))


def own_spans(m):
    a, b = m.rng
    holes = sorted([c.rng for c in m.children if c.file == m.file and c.rng != (0, len(load(m.file)[1]))] + m.includes)
    spans, cur = [], a
    for ha, hb in holes:
        if ha >= cur: spans.append((cur, ha)); cur = hb + 1
    spans.append((cur, b))
    return spans


class Node:
    def __init__(s, nid, file, kind, spans, defs, label):
        s.id, s.file, s.kind, s.spans, s.defs, s.label = nid, file, kind, spans, set(defs), label


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


def build(split):
    walk_file(ROOT, "crate", "prod")
    nodes = {}
    pending = []
    for m in mods:
        raw, cl = load(m.file)
        spans = own_spans(m)
        size = sum(cl.count("\n", a, b) for a, b in spans)
        if size <= split or m.kind == "test":
            names = set()
            dm = depth_map(m.file)
            depth_base = dm[m.rng[0]] + (1 if cl[m.rng[0]:m.rng[0] + 1] == "{" else 0)
            for a, b in spans:
                for d in DEF_RE.finditer(cl, a, b):
                    if dm[d.start()] == depth_base: names.add(d.group(1) or d.group(2))
            nodes[m.path] = Node(m.path, m.file, m.kind, spans, names, m.path)
            continue
        type_node = {}
        impls = []
        for a, b in spans:
            its, _ = ri.items(raw, cl, a + (1 if cl[a:a + 1] == "{" else 0), b)
            for it in its:
                k = cfg_kind(attrs_before(raw, cl, it["head"]), m.kind)
                line = ri.line_of(raw, it["head"])
                if it["kind"] == "impl":
                    impls.append((it, k, line)); continue
                nid = f"{m.path}@{it['name'] or it['kind']}:{line}"
                n = Node(nid, m.file, k, [(it["start"], it["end"])], [it["name"]] if it["name"] and it["kind"] not in ("macro_call", "use") else [], f"{m.path}::{it['name'] or it['kind']}")
                n.line, n.item = line, it["kind"]
                nodes[nid] = n
                if it["kind"] in ("struct", "enum", "trait", "type", "union") and it["name"]:
                    type_node.setdefault(it["name"], nid)
        pending.append((m, impls, type_node))
    global_types = {}
    for (_, _, tn) in pending:
        for k, v in tn.items(): global_types.setdefault(k, v)
    for m, impls, type_node in pending:
        for it, k, line in impls:
            owner = type_node.get(it.get("self")) or global_types.get(it.get("self")) or global_types.get(it.get("trait"))
            if owner and owner in nodes and k == nodes[owner].kind:
                nodes[owner].spans.append((it["start"], it["end"]))
                nodes[owner].impl_files = getattr(nodes[owner], "impl_files", set()) | {m.file}
                continue
            nid = f"{m.path}@impl {it.get('trait') or ''} {it.get('self')}:{line}"
            n = Node(nid, m.file, k, [(it["start"], it["end"])], [], f"{m.path}::impl {it.get('trait') or ''} for {it.get('self')}")
            n.line, n.item, n.attach = line, "impl", owner
            nodes[nid] = n
    for n in nodes.values():
        n.texts = []
    return nodes


def facet_of(n):
    rel = os.path.relpath(n.file, BASE)
    if n.kind == "test": return "test"
    for rx, f in CFG.get("names", []):
        if re.search(rx, n.label): return f
    for rx, f in CFG["files"]:
        if re.search(rx, rel): return f
    return "other"


def scc(nodes, adj):
    index, low, onstack, stack, out, counter = {}, {}, set(), [], [], [0]
    sys.setrecursionlimit(1000000)

    def visit(v):
        index[v] = low[v] = counter[0]; counter[0] += 1
        stack.append(v); onstack.add(v)
        for w in adj.get(v, ()):
            if w not in index: visit(w); low[v] = min(low[v], low[w])
            elif w in onstack: low[v] = min(low[v], index[w])
        if low[v] == index[v]:
            comp = []
            while True:
                w = stack.pop(); onstack.discard(w); comp.append(w)
                if w == v: break
            out.append(comp)
    for v in nodes:
        if v not in index: visit(v)
    return out


def text_of(n):
    out = []
    for a, b in n.spans:
        for f in [n.file] + sorted(getattr(n, "impl_files", ())):
            cl = load(f)[1]
            if b <= len(cl): out.append(cl[a:b]); break
    return "\n".join(out)


def main():
    args = sys.argv[2:]
    split = int(args[args.index("--split") + 1]) if "--split" in args else 300
    nodes = build(split)
    defs = defaultdict(set)
    text = {}
    for n in nodes.values():
        text[n.id] = text_of(n)
        for d in n.defs: defs[d].add(n.id)
    edges = defaultdict(lambda: defaultdict(set))
    for n in nodes.values():
        t = text[n.id]
        toks = {x for x in IDENT_RE.findall(t) if not x[0].islower() and x[0] != "_"}
        toks |= set(re.findall(r"(?:::|(?<![.\w]))([a-z_][a-z0-9_]*)\s*(?:\(|!|::<)", t))
        toks |= set(re.findall(r"::([a-z_][a-z0-9_]*)\b", t))
        for x in toks:
            if x in COMMON or x in n.defs: continue
            owners = defs.get(x)
            if owners and len(owners) == 1:
                o = next(iter(owners))
                if o != n.id: edges[n.id][o].add(x)
        if getattr(n, "attach", None) and n.attach in nodes: edges[n.id][n.attach].add("impl-of")
    facet = {n.id: facet_of(n) for n in nodes.values()}
    lines = {n.id: sum(1 for l in text[n.id].split("\n") if l.strip()) for n in nodes.values()}
    if "--nodes" in args:
        for nid in sorted(nodes, key=lambda x: (facet[x], nodes[x].file, getattr(nodes[x], "line", 0))):
            n = nodes[nid]
            print(f"{facet[nid]:14} {n.kind:5} {lines[nid]:6} {getattr(n, 'line', '-')} {n.label}  [{os.path.relpath(n.file, BASE)}]")
        return
    fl = defaultdict(lambda: [0, 0])
    for nid in nodes: fl[facet[nid]][0 if nodes[nid].kind == "prod" else 1] += lines[nid]
    fe = defaultdict(lambda: defaultdict(int))
    fe_syms = defaultdict(lambda: defaultdict(set))
    for a, tos in edges.items():
        if nodes[a].kind != "prod": continue
        for b, syms in tos.items():
            if nodes[b].kind != "prod": continue
            if facet[a] != facet[b]:
                fe[facet[a]][facet[b]] += len(syms)
                fe_syms[(facet[a], facet[b])][(a, b)] |= syms
    if "--edge" in args:
        i = args.index("--edge"); A, B = args[i + 1], args[i + 2]
        for (a, b), syms in sorted(fe_syms[(A, B)].items(), key=lambda x: -len(x[1])):
            print(f"{nodes[a].label} [{lines[a]}] -> {nodes[b].label}: {', '.join(sorted(syms)[:30])}")
        return
    order = CFG["order"]; allow = CFG.get("allow", {})
    print("## facet non-blank lines (prod, test)")
    for f_, (a, b) in sorted(fl.items(), key=lambda x: -x[1][0]):
        print(f"{f_:16} {a:7} {b:7}")
    print("\n## facet edges, prod -> prod (distinct node-pair symbols); '!' marks an edge `allow` forbids")
    names = sorted(fl, key=lambda f: order.index(f) if f in order else 99)
    bad = 0
    for a in names:
        row = {b: fe[a].get(b, 0) for b in names if fe[a].get(b, 0)}
        if row:
            marks = []
            for b, n in sorted(row.items(), key=lambda x: -x[1]):
                forbidden = a in allow and b not in allow[a]
                bad += n if forbidden else 0
                marks.append(f"{'!' if forbidden else ''}{b}:{n}")
            print(f"{a:16} -> " + ", ".join(marks))
    print(f"\nforbidden symbol edges: {bad}")
    print("\n## facet SCCs (prod)")
    adj = defaultdict(set)
    for a, tos in fe.items():
        for b, v in tos.items():
            if v: adj[a].add(b)
    for comp in scc(names, adj):
        if len(comp) > 1: print("cycle:", " <-> ".join(sorted(comp, key=lambda f: order.index(f) if f in order else 99)))
    print("\n## node SCCs (prod, size > 1)")
    nadj = defaultdict(set)
    for a, tos in edges.items():
        if nodes[a].kind != "prod": continue
        for b in tos:
            if nodes[b].kind == "prod": nadj[a].add(b)
    for comp in sorted(scc([x for x in nodes if nodes[x].kind == "prod"], nadj), key=len, reverse=True)[:10]:
        if len(comp) > 1:
            fs = defaultdict(int)
            for x in comp: fs[facet[x]] += lines[x]
            print(f"SCC {len(comp)} nodes, {sum(lines[x] for x in comp)} lines: " + ", ".join(f"{k}:{v}" for k, v in sorted(fs.items(), key=lambda x: -x[1])))
            if "--scc-members" in args:
                for x in sorted(comp): print("   ", nodes[x].label, f"[{facet[x]}]")
    if "--json" in args:
        out = args[args.index("--json") + 1]
        json.dump({"nodes": [{"id": x, "label": nodes[x].label, "facet": facet[x], "kind": nodes[x].kind, "lines": lines[x], "line": getattr(nodes[x], "line", None), "file": os.path.relpath(nodes[x].file, REPO)} for x in nodes],
                   "edges": [{"from": a, "to": b, "symbols": sorted(s)} for a, tos in edges.items() for b, s in tos.items()]}, open(out, "w"), ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
