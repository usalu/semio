"""Board god-object analysis: fields, methods, self-call graph, field closure."""
import sys, os, re, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from oinf_rs import *
from modtree import walk, LIBROOT, INFROOT
from wrewrite import parse_struct_fields

def ctoks(text): return [t for t in lex(text) if t.k != 'comment']

def load_board_mods():
    tree = walk(LIBROOT)
    mods = {}
    for e in tree:
        f = e['file']
        if not f or '🧪️tests' in f or '🧫️fixtures' in f: continue
        rel = f[len(INFROOT):]
        if rel.startswith('🌍️world') or rel == '🦀️.rs' or rel.startswith('📦️'): continue
        text = open(f, encoding='utf-8').read()
        items, _ = chunk_file(text)
        mods[rel] = (text, items)
    return mods

class M: pass

def host_methods(mods, ty):
    """All fns inside inherent `impl ty` blocks."""
    out = []
    fields = None
    for rel, (text, items) in mods.items():
        for it in items:
            if it.kind == 'struct' and it.name == ty:
                fields = (rel, it, parse_struct_fields(text, it))
            if it.kind == 'impl' and it.impl_ty == ty and it.impl_trait is None:
                bs = text.index('{', it.start)
                inner = text[bs + 1: it.end - 1]
                sub, _ = chunk_file(inner)
                for s in sub:
                    if s.kind != 'fn': continue
                    m = M(); m.rel = rel; m.name = s.name; m.item = s; m.impl = it; m.off = bs + 1
                    m.text = inner[s.start:s.end]; m.lines = inner.count('\n', s.start, s.end) + 1
                    m.vis = s.vis; m.cfg = s.cfg + it.cfg
                    toks = ctoks(m.text)
                    m.toks = toks
                    m.has_self = False; m.self_mut = False
                    # params
                    i = 0
                    while toks[i].s != 'fn': i += 1
                    j = i
                    while toks[j].s != '(': j += 1
                    k = j + 1
                    if toks[k].s == '&':
                        k += 1
                        if toks[k].k == 'id' and toks[k].s.startswith("'"): k += 1
                        if toks[k].s == 'mut': m.self_mut = True; k += 1
                        if toks[k].s == 'self': m.has_self = True
                    elif toks[k].s == 'self': m.has_self = True
                    elif toks[k].s == 'mut' and toks[k+1].s == 'self': m.has_self = True; m.self_mut = True
                    out.append(m)
    return fields, out

def self_refs(m, ty):
    fields = collections.Counter(); calls = collections.Counter(); assoc = collections.Counter()
    t = m.toks
    for i, x in enumerate(t):
        if x.k != 'id': continue
        nxt = t[i+1].s if i + 1 < len(t) else ''
        prv = t[i-1].s if i else ''
        if x.s == 'self' and nxt == '.' and i + 2 < len(t):
            nm = t[i+2].s; after = t[i+3].s if i + 3 < len(t) else ''
            if after == '(' : calls[nm] += 1
            else: fields[nm] += 1
        elif x.s in ('Self', ty) and nxt == ':' and t[i+2].s == ':' and i + 3 < len(t):
            nm = t[i+3].s
            assoc[nm] += 1
    return fields, calls, assoc

if __name__ == '__main__':
    ty = sys.argv[1]
    mods = load_board_mods()
    fields, meths = host_methods(mods, ty)
    rel, it, fl = fields
    print(ty, 'fields', len(fl), 'methods', len(meths), 'lines', sum(m.lines for m in meths))
    names = collections.Counter(m.name for m in meths)
    print('dup names', [(n, c) for n, c in names.items() if c > 1])
    fnames = {f['name'] for f in fl}
    nself = sum(1 for m in meths if m.has_self)
    print('with self', nself)

def closure(meths, fnames, ty):
    byname = {m.name: m for m in meths}
    for m in meths:
        m.f, m.c, m.a = self_refs(m, ty)
        m.df = {x for x in m.f if x in fnames}
        m.callees = {x for x in m.c if x in byname} | {x for x in m.a if x in byname}
        m.unk = {x for x in m.f if x not in fnames}
    for m in meths: m.req = set(m.df)
    ch = True
    while ch:
        ch = False
        for m in meths:
            n = set(m.req)
            for c in m.callees: n |= byname[c].req
            if n != m.req: m.req = n; ch = True
    return byname
