#!/usr/bin/env python3
"""🕸️ Item and facet dependency graph of `semio-framework-plugin` (wave E2).

Walks the mounted module tree from the lib root (`#[path]`, inline `mod {}`, `include!`). Every module
whose own span is larger than `--split` lines is split into its top-level items; `impl` blocks join
their local self type (or local trait). The inherent `impl VcsArtifactApp` blocks are split once more
into one node per method, so the god impl can be assigned method by method. Every node gets a facet
from `🗺️facets.json` (file regex rules, then item names, then line ranges of the root file) and the
tool reports:

- lines per facet (prod, test), the facet x facet symbol matrix, facet SCCs (prod)
- `--edge A B`: the symbols behind a facet edge, node by node
- `--nodes`: every node with facet, kind, lines and location
- `--unassigned`: nodes that fell through to `other`
- `--json OUT`: the whole graph

It compiles nothing. Usage: plugin-module-graph.py [--split N] [--edge A B] [--nodes] [--unassigned] [--json OUT]
"""
import importlib.util, json, os, re, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rust_items", os.path.join(HERE, "🧱️rust_items.py"))
ri = importlib.util.module_from_spec(spec); spec.loader.exec_module(ri)

REPO = "/Users/ueli/Documents/semio"
PLUGIN = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
ROOT = f"{PLUGIN}/📦️packages/🦀️rust/🦀️.rs"
FACETS = json.load(open(os.path.join(HERE, "🗺️facets.json"), encoding="utf-8"))

MOD_RE = re.compile(r"(?<![\w:])(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+(r#)?(\w+)\s*([;{])")
INC_RE = re.compile(r'include!\s*\(\s*"([^"]+)"\s*\)')
DEF_RE = re.compile(r"(?<![\w])(?:struct|enum|trait|union|type|fn|const|static)\s+(\w+)|macro_rules!\s*(\w+)")
IDENT_RE = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
COMMON = {"new", "default", "from", "into", "clone", "fmt", "drop", "len", "is_empty", "get", "step", "close_step", "main", "tests", "T", "Self", "Ok", "Err", "Some", "None", "A", "M", "S", "P", "C", "D", "E"}

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
            if re.fullmatch(r"\s*test\s*", body): k = "test"
            elif re.search(r"\btest\b", body) and "not(test" not in body and k != "test": k = "laws"
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
        parent.children.append(walk_file(inc, modpath + "#include:" + os.path.relpath(inc, PLUGIN), cfg_kind(attrs_before(raw, cl, m.start()), kind)))


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
        s.methods = set()


def item_kind(raw, cl, it, inherited):
    return cfg_kind(attrs_before(raw, cl, it["head"]), inherited)


def build(split):
    walk_file(ROOT, "crate", "prod")
    nodes = {}
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
        pending_impls = []
        for a, b in spans:
            its, _ = ri.items(raw, cl, a + (1 if cl[a:a + 1] == "{" else 0), b)
            for it in its:
                k = item_kind(raw, cl, it, m.kind)
                line = ri.line_of(raw, it["head"])
                if it["kind"] == "impl":
                    pending_impls.append((it, k, line)); continue
                nid = f"{m.path}@{it['name'] or it['kind']}:{line}"
                n = Node(nid, m.file, k, [(it["start"], it["end"])], [it["name"]] if it["name"] and it["kind"] not in ("macro_call", "use") else [], it["name"] or it["kind"])
                n.line, n.item = line, it["kind"]
                nodes[nid] = n
                if it["kind"] in ("struct", "enum", "trait", "type", "union") and it["name"]:
                    type_node.setdefault(it["name"], nid)
        build.pending.append((m, raw, cl, pending_impls, type_node))
    global_types = {}
    for (_, _, _, _, tn) in build.pending:
        for k, v in tn.items(): global_types.setdefault(k, v)
    for m, raw, cl, pending_impls, type_node in build.pending:
        for it, k, line in pending_impls:
            owner = global_types.get(it.get("self")) or global_types.get(it.get("trait"))
            if it.get("self") == "VcsArtifactApp" and not it.get("trait"):
                ob = cl.index("{", it["head"]); cb = ri.skip_group(cl, ob) - 1
                subs, _ = ri.items(raw, cl, ob + 1, cb)
                for s in subs:
                    mk = item_kind(raw, cl, s, k)
                    nid = f"VcsArtifactApp::{s['name']}:{ri.line_of(raw, s['head'])}"
                    n = Node(nid, m.file, mk, [(s["start"], s["end"])], [], f"VcsArtifactApp::{s['name']}")
                    n.line, n.item, n.method = ri.line_of(raw, s["head"]), "method", s["name"]
                    nodes[nid] = n
                continue
            if owner and owner in nodes and k == nodes[owner].kind:
                nodes[owner].spans.append((it["start"], it["end"]))
                continue
            nid = f"{m.path}@impl {it.get('trait') or ''} {it.get('self')}:{line}"
            n = Node(nid, m.file, k, [(it["start"], it["end"])], [], f"impl {it.get('trait') or ''} for {it.get('self')}")
            n.line, n.item = line, "impl"
            n.attach = owner
            nodes[nid] = n
    return nodes


build.pending = []


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


def facet_of(n):
    rel = os.path.relpath(n.file, PLUGIN)
    if n.kind in ("test", "laws"): return "laws"
    for rule in FACETS["names"]:
        if re.fullmatch(rule[0], n.label): return rule[1]
    if getattr(n, "method", None):
        for rule in FACETS["methods"]:
            if re.fullmatch(rule[0], n.method): return rule[1]
        return "other"
    if hasattr(n, "line") and rel == "🦀️.rs":
        for a, b, f in FACETS["lines"]:
            if a <= n.line <= b: return f
    for rx, f in FACETS["files"]:
        if re.search(rx, rel): return f
    return "other"


def main():
    args = sys.argv[1:]
    split = int(args[args.index("--split") + 1]) if "--split" in args else 1500
    nodes = build(split)
    defs = defaultdict(set)
    methods = defaultdict(set)
    text = {}
    for n in nodes.values():
        raw, cl = load(n.file)
        text[n.id] = "\n".join(cl[a:b] for a, b in n.spans)
        for d in n.defs: defs[d].add(n.id)
        if getattr(n, "method", None): methods[n.method].add(n.id)
    fn_count = defaultdict(int)
    for f in list(cache):
        for d in re.finditer(r"\bfn\s+(\w+)", cache[f][1]): fn_count[d.group(1)] += 1
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
        for mname in set(re.findall(r"\.([a-z_][a-z0-9_]*)\s*(?:\(|::<)", t)) | set(re.findall(r"Self::([a-z_][a-z0-9_]*)\b", t)):
            if fn_count.get(mname, 0) != len(methods.get(mname, ())): continue
            for o in methods.get(mname, ()):
                if o != n.id: edges[n.id][o].add("." + mname)
        if getattr(n, "attach", None) and n.attach in nodes: edges[n.id][n.attach].add("impl-of")
    facet = {n.id: facet_of(n) for n in nodes.values()}
    lines = {n.id: sum(1 for l in text[n.id].split("\n") if l.strip()) for n in nodes.values()}
    if "--unassigned" in args:
        for nid in sorted((x for x in nodes if facet[x] == "other"), key=lambda x: (nodes[x].file, getattr(nodes[x], "line", 0))):
            n = nodes[nid]
            print(f"{lines[nid]:6} {getattr(n, 'line', '-')} {n.label}  [{os.path.relpath(n.file, PLUGIN)}]")
        return
    if "--nodes" in args:
        for nid in sorted(nodes, key=lambda x: (facet[x], nodes[x].file, getattr(nodes[x], "line", 0))):
            n = nodes[nid]
            print(f"{facet[nid]:14} {n.kind:5} {lines[nid]:6} {getattr(n, 'line', '-')} {n.label}  [{os.path.relpath(n.file, PLUGIN)}]")
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
    order = FACETS["order"]
    print("## facet lines (prod, test)")
    for f_, (a, b) in sorted(fl.items(), key=lambda x: -x[1][0]):
        print(f"{f_:16} {a:7} {b:7}")
    print("\n## facet edges, prod -> prod (distinct node-pair symbols); '!' marks an edge against the target order")
    names = sorted(fl, key=lambda f: order.index(f) if f in order else 99)
    for a in names:
        row = {b: fe[a].get(b, 0) for b in names if fe[a].get(b, 0)}
        if row:
            print(f"{a:16} -> " + ", ".join(f"{'!' if a in order and b in order and order.index(b) > order.index(a) else ''}{b}:{n}" for b, n in sorted(row.items(), key=lambda x: -x[1])))
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
    for comp in sorted(scc([x for x in nodes if nodes[x].kind == "prod"], nadj), key=len, reverse=True)[:8]:
        if len(comp) > 1:
            fs = defaultdict(int)
            for x in comp: fs[facet[x]] += lines[x]
            print(f"SCC {len(comp)} nodes, {sum(lines[x] for x in comp)} lines: " + ", ".join(f"{k}:{v}" for k, v in sorted(fs.items(), key=lambda x: -x[1])))
    if "--json" in args:
        out = args[args.index("--json") + 1]
        json.dump({"nodes": [{"id": x, "label": nodes[x].label, "facet": facet[x], "kind": nodes[x].kind, "lines": lines[x], "line": getattr(nodes[x], "line", None), "file": os.path.relpath(nodes[x].file, REPO)} for x in nodes],
                   "edges": [{"from": a, "to": b, "symbols": sorted(s)} for a, tos in edges.items() for b, s in tos.items()]}, open(out, "w"), ensure_ascii=False, indent=1)


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


if __name__ == "__main__":
    main()
