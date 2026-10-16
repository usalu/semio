import re, sys, collections
sys.path.insert(0, ".")
from igraph import *

def is_test_atom(a, files):
    if is_test_file(a.file, files): return True
    return any('cfg(test)' in c.replace(' ', '') for c in a.item.cfg)

import functools
@functools.lru_cache(None)
def local_mod_names():
    files = walk(LIBROOT, ["crate"])
    names = {"crate", "super", "self"}
    for f in files.values():
        names.add(f.modpath[-1])
        for (n, _, _, _) in f.children: names.add(n)
        for m in re.finditer(r'\bmod\s+([A-Za-z_]\w*)', f.code): names.add(m.group(1))
    return frozenset(names)

def tokens_of(text, own):
    out = set()
    for m in TOK.finditer(text):
        e = m.end(1)
        k = e
        while k < len(text) and text[k] in ' \t\n': k += 1
        follow = text[k:k+2]
        s = m.start(1)
        name = m.group(1)
        # preceded by '.'
        j = s - 1
        qual = None
        while j >= 0 and text[j] in ' \t\n': j -= 1
        if j >= 0 and text[j] == '.' and not text.startswith('..', max(0, j-1)): continue
        if text[max(0, j-1):j+1] == 'fn' and (j < 2 or not (text[j-2].isalnum() or text[j-2] == '_')): continue
        if j >= 1 and text[j] == ':' and text[j-1] == ':':
            # qualifier
            q = text[max(0, j-1-80):j-1]
            qm = re.search(r'([A-Za-z_]\w*)\s*(?:<[^<>]*>)?\s*$', q)
            if qm: qual = qm.group(1)
            if qm and qm.group(1)[0].isupper(): continue
            if qm and qm.group(1) not in local_mod_names(): continue
        out.add((name, follow, qual))
    return out

_imp_cache = {}
def import_map(f):
    if f.path in _imp_cache: return _imp_cache[f.path]
    from uses import file_uses
    m = {}
    for vis, p, alias, off, pre in file_uses(f):
        if p[0] in ('crate', 'super', 'self') and len(p) > 2:
            m[p[-1]] = p[-2] if p[1] != p[-1] else p[1]
            if p[0] == 'crate': m[p[-1]] = p[1]
    _imp_cache[f.path] = m
    return m

def is_ancestor(anc, f):
    cur = f.parent
    while cur is not None:
        if cur is anc: return True
        cur = cur.parent
    return False

def atom_edges(files, atoms, defs, want_test=False):
    edges = collections.defaultdict(set)
    amb = collections.defaultdict(set)
    flags = {id(a): is_test_atom(a, files) for a in atoms}
    for a in atoms:
        if flags[id(a)] and not want_test: continue
        toks = tokens_of(a.text, a.name)
        fileimports = import_map(a.file)
        for (t, follow, qual) in toks:
            if t == a.name: continue
            cands = [d for d in defs.get(t, []) if (want_test or not flags[id(d)]) and d.kind != 'mod']
            if cands and all(d.kind == 'fn' for d in cands):
                if not (follow.startswith('(') or follow.startswith('::') or follow.startswith('<') or follow[:1] in ',;)' ):
                    continue
                if follow[:1] in ',;)' and t[0].islower() and len(t) < 8: continue
            cands = [d for d in cands if d is not a]
            cands = [d for d in cands if d.item.vis or d.file is a.file or is_ancestor(d.file, a.file)]
            if not cands: continue
            if len(cands) > 1:
                same = [d for d in cands if d.file is a.file]
                if same: cands = same
                else:
                    q = []
                    if qual and qual not in ('crate', 'super', 'self'):
                        q = [d for d in cands if qual in d.file.modpath]
                    if not q and t in fileimports:
                        q = [d for d in cands if fileimports[t] in d.file.modpath]
                    if q: cands = q
                    else:
                        sm = [d for d in cands if topmod(d.file) == topmod(a.file)]
                        if sm: cands = sm
            if len(cands) > 1:
                amb[id(a)].add(t)
            for d in cands:
                edges[id(a)].add(id(d))
    return edges, amb, flags

def tarjan(nodes, edges):
    idx = {}; low = {}; st = []; on = set(); res = []; c = [0]
    sys.setrecursionlimit(100000)
    def sc(v):
        idx[v] = low[v] = c[0]; c[0] += 1; st.append(v); on.add(v)
        for w in edges.get(v, ()):
            if w not in idx: sc(w); low[v] = min(low[v], low[w])
            elif w in on: low[v] = min(low[v], idx[w])
        if low[v] == idx[v]:
            comp = []
            while True:
                w = st.pop(); on.discard(w); comp.append(w)
                if w == v: break
            res.append(comp)
    for v in nodes:
        if v not in idx: sc(v)
    return res

if __name__ == "__main__":
    files, allitems, atoms, defs, loose = build()
    edges, amb, flags = atom_edges(files, atoms, defs)
    byid = {id(a): a for a in atoms}
    prod = [id(a) for a in atoms if not flags[id(a)] and a.kind != 'mod']
    comps = tarjan(prod, edges)
    big = [c for c in comps if len(c) > 1]
    print("prod atoms", len(prod), "sccs>1:", len(big), "sizes", sorted(len(c) for c in big))
    for c in sorted(big, key=len, reverse=True)[:5]:
        mods = collections.Counter(topmod(byid[x].file) for x in c)
        print(len(c), dict(mods))
