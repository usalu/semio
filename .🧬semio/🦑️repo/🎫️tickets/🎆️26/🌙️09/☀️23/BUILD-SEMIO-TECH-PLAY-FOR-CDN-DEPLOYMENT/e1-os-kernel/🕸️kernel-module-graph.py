#!/usr/bin/env python3
"""🕸️ Module and facet dependency graph of `semio-framework-os-kernel` (wave E1).

Walks the mounted module tree from the lib root (`#[path]`, inline `mod {}`, `include!`), extracts
module-level item definitions, resolves identifier references against them, and reports:

- per-module own lines (test / non-test), facet assignment
- facet x facet reference matrix and module-level strongly connected components
- for a selected facet pair, the concrete symbols behind an edge (`--edge A B`)

It compiles nothing. Usage:
  kernel-module-graph.py [--json OUT] [--edge FROM TO] [--modules] [--items FILE_SUBSTR]
"""
import json, os, re, sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"
ROOT = f"{REPO}/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs"


def clean(src):
    out = list(src)
    i, n = 0, len(src)
    def blank(a, b):
        for k in range(a, b):
            if out[k] != "\n":
                out[k] = " "
    while i < n:
        c = src[i]
        if c == "/" and i + 1 < n and src[i + 1] == "/":
            j = src.find("\n", i)
            j = n if j < 0 else j
            blank(i, j); i = j; continue
        if c == "/" and i + 1 < n and src[i + 1] == "*":
            depth, j = 1, i + 2
            while j < n and depth:
                if src.startswith("/*", j): depth += 1; j += 2
                elif src.startswith("*/", j): depth -= 1; j += 2
                else: j += 1
            blank(i, j); i = j; continue
        m = re.match(r'b?r(#*)"', src[i:i + 20]) if c in "rb" and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")) else None
        if m:
            hashes = m.group(1)
            start = i + m.end()
            end = src.find('"' + hashes, start)
            end = n if end < 0 else end
            blank(start, end); i = end + 1 + len(hashes); continue
        if c == '"' or (c == "b" and i + 1 < n and src[i + 1] == '"' and not (i and (src[i-1].isalnum() or src[i-1] == "_"))):
            j = i + (2 if c == "b" else 1)
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            blank(i + (2 if c == "b" else 1), j); i = j + 1; continue
        if c == "'":
            m = re.match(r"'(\\.[^']*|[^'\\])'", src[i:i + 12])
            if m:
                blank(i + 1, i + m.end() - 1); i += m.end(); continue
        i += 1
    return "".join(out)


def match_brace(text, i):
    depth = 0
    for j in range(i, len(text)):
        ch = text[j]
        if ch == "{": depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0: return j
    return len(text) - 1


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


MOD_RE = re.compile(r"(?<![\w:])(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+(r#)?(\w+)\s*([;{])")
INC_RE = re.compile(r'include!\s*\(\s*"([^"]+)"\s*\)')


def cfg_kind(attrs, inherited):
    k = inherited
    for a in attrs:
        if a.startswith("#[cfg("):
            body = a[6:-2]
            if re.fullmatch(r"\s*test\s*", body): k = "test"
            elif re.search(r"\btest\b", body) and k != "test": k = "laws"
    return k


class Mod:
    def __init__(s, path, file, kind, rng):
        s.path, s.file, s.kind, s.rng, s.children = path, file, kind, rng, []


mods = []
file_cache = {}


def load(f):
    if f not in file_cache:
        raw = open(f, encoding="utf-8").read()
        file_cache[f] = (raw, clean(raw))
    return file_cache[f]


def walk_file(f, modpath, kind, base_dir):
    raw, cl = load(f)
    root = Mod(modpath, f, kind, (0, len(cl)))
    mods.append(root)
    walk_region(f, raw, cl, 0, len(cl), modpath, kind, base_dir, root)
    for m in INC_RE.finditer(cl):
        rawname = raw[m.start(1):m.end(1)]
        inc = os.path.normpath(os.path.join(os.path.dirname(f), rawname))
        walk_file(inc, modpath + "#include", cfg_kind(attrs_before(raw, cl, m.start()), kind), os.path.dirname(inc))
    return root


def walk_region(f, raw, cl, a, b, modpath, kind, base_dir, parent):
    i = a
    while True:
        m = MOD_RE.search(cl, i, b)
        if not m: return
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
                child = walk_file(target, f"{modpath}::{name}", k, os.path.dirname(target))
                parent.children.append(child)
            else:
                print(f"[warn] unresolved mod {modpath}::{name} -> {target}", file=sys.stderr)
            i = m.end()
        else:
            ob = m.end() - 1
            cb = match_brace(cl, ob)
            d = os.path.normpath(os.path.join(base_dir, p if p is not None else name))
            child = Mod(f"{modpath}::{name}", f, k, (ob, cb))
            mods.append(child); parent.children.append(child)
            walk_region(f, raw, cl, ob + 1, cb, child.path, k, d, child)
            i = cb + 1


DEF_RE = re.compile(r"(?<![\w])(?:struct|enum|trait|union|type|fn|const|static)\s+(\w+)|macro_rules!\s*(\w+)")
IDENT_RE = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")


def own_spans(m):
    a, b = m.rng
    holes = sorted(c.rng for c in m.children if c.file == m.file and c.rng != (0, len(load(m.file)[1])))
    spans, cur = [], a
    for ha, hb in holes:
        if ha >= cur: spans.append((cur, ha)); cur = hb + 1
    spans.append((cur, b))
    return spans


def depth_map(cl):
    d, out = 0, bytearray(len(cl))
    for i, ch in enumerate(cl):
        if ch == "}": d -= 1
        out[i] = max(0, min(d, 255))
        if ch == "{": d += 1
    return out


FACETS = [
    ("laws", r"🧪️tests|🧪️testing|🔮️oracles|🧫️fixtures"),
    ("canonical-edit", r"🏪️store/🧵️canonical-edit"),
    ("history", r"🏪️store/(🕰️history|📜️space-history|🔁️replay|🧾️document/📜️history)"),
    ("retirement", r"🏪️store/(♻️retirement|🔗️read|📦️prepared|🔗️backbone/♻️retirement)|/♻️retirement/"),
    ("store-composition", r"🏪️store/🧩️composition"),
    ("store-sync", r"🏪️store/(🔄️sync|👷️worker)"),
    ("store", r"🏪️store"),
    ("spr", r"📡️spr"),
    ("dsl", r"🗣️dsl"),
    ("pack", r"🎒️pack"),
    ("vcs", r"🌿️vcs"),
    ("directory", r"📇️directory|🔌️plugin/📇️registry"),
    ("io", r"/🔨️modules/🚪️io"),
    ("inference", r"💡️inference"),
    ("semio", r"🧬️semio"),
    ("extension", r"🧩️extension"),
    ("identity", r"🪪️identity"),
    ("list-delta", r"🪡️list-delta"),
    ("root", r"📦️packages/🦀️rust/🦀️.rs$"),
]


def facet_of(m):
    rel = m.file
    if m.kind in ("test", "laws"): return "laws"
    for name, rx in FACETS:
        if re.search(rx, rel): return name
    return "other"


def main():
    args = sys.argv[1:]
    walk_file(ROOT, "crate", "prod", os.path.dirname(ROOT))
    defs = defaultdict(set)
    mod_defs = {}
    mod_text = {}
    for m in mods:
        raw, cl = load(m.file)
        dm = depth_map(cl)
        base = dm[m.rng[0] + 1] if m.rng[0] < len(cl) and cl[m.rng[0]] == "{" else 0
        names = set()
        txt = []
        for a, b in own_spans(m):
            seg = cl[a:b]
            txt.append(seg)
            for dmatch in DEF_RE.finditer(cl, a, b):
                if dm[dmatch.start()] == base:
                    names.add(dmatch.group(1) or dmatch.group(2))
        mod_defs[m.path] = names
        mod_text[m.path] = "\n".join(txt)
        for nm in names: defs[nm].add(m.path)
    common = {"new", "default", "from", "into", "clone", "fmt", "drop", "len", "is_empty", "get", "step", "close_step", "main", "tests", "T", "Self", "Ok", "Err", "Some", "None"}
    edges = defaultdict(lambda: defaultdict(set))
    for m in mods:
        txt = mod_text[m.path]
        toks = {t for t in IDENT_RE.findall(txt) if not t[0].islower() and t[0] != "_"}
        toks |= set(re.findall(r"(?:::|\b)([a-z_][a-z0-9_]*)\s*(?:\(|!|::<)", txt))
        toks |= set(re.findall(r"::([a-z_][a-z0-9_]*)\b", txt))
        for t in toks:
            if t in common or t in mod_defs[m.path]: continue
            owners = defs.get(t)
            if owners and len(owners) == 1:
                o = next(iter(owners))
                if o != m.path: edges[m.path][o].add(t)
    lines = {}
    for m in mods:
        lines[m.path] = len([l for l in mod_text[m.path].split("\n") if l.strip()])
    facet = {m.path: facet_of(m) for m in mods}
    kinds = {m.path: m.kind for m in mods}
    files = {m.path: m.file for m in mods}
    fl = defaultdict(lambda: [0, 0])
    for p in lines:
        fl[facet[p]][0 if kinds[p] == "prod" else 1] += lines[p]
    fe = defaultdict(lambda: defaultdict(int))
    fe_syms = defaultdict(lambda: defaultdict(set))
    for a, tos in edges.items():
        for b, syms in tos.items():
            if facet[a] != facet[b]:
                fe[facet[a]][facet[b]] += len(syms)
                fe_syms[(facet[a], facet[b])][(a, b)] |= syms
    if "--edge" in args:
        i = args.index("--edge"); A, B = args[i + 1], args[i + 2]
        for (a, b), syms in sorted(fe_syms[(A, B)].items()):
            print(f"{a}  ->  {b}: {', '.join(sorted(syms)[:40])}")
        return
    if "--modules" in args:
        for p in sorted(lines, key=lambda p: (facet[p], p)):
            print(f"{facet[p]:18} {kinds[p]:5} {lines[p]:7} {p}  [{os.path.relpath(files[p], REPO)}]")
        return
    print("## facet lines (prod, test)")
    for f_, (a, b) in sorted(fl.items(), key=lambda x: -x[1][0]):
        print(f"{f_:18} {a:7} {b:7}")
    print("\n## facet edges (distinct symbols referenced)")
    names = sorted(fl)
    for a in names:
        row = {b: fe[a][b] for b in names if fe[a][b]}
        if row: print(f"{a:18} -> " + ", ".join(f"{b}:{n}" for b, n in sorted(row.items(), key=lambda x: -x[1])))
    print("\n## facet SCCs (prod edges only)")
    prod_fe = defaultdict(set)
    for a, tos in edges.items():
        if kinds[a] != "prod": continue
        for b in tos:
            if kinds[b] == "prod" and facet[a] != facet[b]: prod_fe[facet[a]].add(facet[b])
    for comp in scc(names, prod_fe):
        if len(comp) > 1: print("cycle:", " <-> ".join(sorted(comp)))
    print("\n## module SCCs (prod modules, size > 1)")
    prod_me = defaultdict(set)
    for a, tos in edges.items():
        if kinds[a] != "prod": continue
        for b in tos:
            if kinds[b] == "prod": prod_me[a].add(b)
    pm = [p for p in lines if kinds[p] == "prod"]
    for comp in scc(pm, prod_me):
        if len(comp) > 1:
            fs = defaultdict(int)
            for p in comp: fs[facet[p]] += lines[p]
            print(f"SCC {len(comp)} modules, {sum(lines[p] for p in comp)} lines: " + ", ".join(f"{k}:{v}" for k, v in sorted(fs.items(), key=lambda x: -x[1])))
    if "--json" in args:
        out = args[args.index("--json") + 1]
        json.dump({"modules": [{"path": p, "facet": facet[p], "kind": kinds[p], "lines": lines[p], "file": os.path.relpath(files[p], REPO)} for p in lines],
                   "edges": [{"from": a, "to": b, "symbols": sorted(s)} for a, tos in edges.items() for b, s in tos.items()]}, open(out, "w"), ensure_ascii=False, indent=1)


def scc(nodes, adj):
    index, low, onstack, stack, out, counter = {}, {}, set(), [], [], [0]
    sys.setrecursionlimit(100000)
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
