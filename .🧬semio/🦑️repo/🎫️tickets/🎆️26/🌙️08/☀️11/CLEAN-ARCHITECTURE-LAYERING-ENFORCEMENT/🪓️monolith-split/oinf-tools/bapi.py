"""Public API of the infinite crate before the carve: per public module path, the names it exports and their origin."""
import sys, os, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from oinf_rs import *
from usetree import parse_use_item
from modtree import path_attr

ITEM_KINDS = ('fn', 'struct', 'enum', 'union', 'trait', 'type', 'const', 'static', 'macro_rules')

class Mod: pass

def build_tree(libfile, infroot):
    mods = {}
    def add(modpath, items, text, base, parent, file, pub):
        m = Mod(); m.path = modpath; m.items = items; m.text = text; m.parent = parent; m.file = file; m.pub = pub
        m.children = {}; mods[modpath] = m
        for it in items:
            if it.kind != 'mod': continue
            pa = path_attr(it.attrs)
            cpub = it.vis == 'pub'
            if it.text.rstrip().endswith('}'):
                bs = text.index('{', it.start)
                inner = text[bs + 1: it.end - 1]
                sub, _ = chunk_file(inner)
                add(modpath + (it.name,), sub, inner, os.path.join(base, pa if pa else it.name), m, None, cpub)
            else:
                child = os.path.normpath(os.path.join(base, pa))
                ct = open(child, encoding='utf-8').read()
                citems, _ = chunk_file(ct)
                add(modpath + (it.name,), citems, ct, os.path.dirname(child), m, child, cpub)
            m.children[it.name] = modpath + (it.name,)
        m.defs = {}
        m.uses = []
        for it in items:
            if it.kind in ITEM_KINDS and it.name: m.defs[it.name] = (it.kind, it.vis, [c for c in it.cfg])
            if it.kind == 'use':
                attrs, vis, ents = parse_use_item(it.text)
                m.uses.append((vis, attrs, ents))
        return m
    t = open(libfile, encoding='utf-8').read()
    items, _ = chunk_file(t)
    add(('crate',), items, t, os.path.dirname(libfile), None, libfile, True)
    return mods

def resolve_path(mods, cur, path):
    """-> ('mod', modpath) | ('item', modpath, name) | ('ext', path) | None"""
    m = mods[cur]
    i = 0; node = None
    first = path[0]
    if first in ('crate', 'infinite'): node = ('crate',); i = 1
    elif first == 'self': node = cur; i = 1
    elif first == 'super': node = cur; 
    else:
        node = None
    while i < len(path) and path[i] == 'super':
        node = mods[node or cur].parent.path; i += 1
    if node == ('crate',) and i < len(path) and path[i] == 'infinite': i += 1
    if node is None:
        if first in m.children: node = m.children[first]; i = 1
        elif first in m.defs: return ('item', cur, first) if len(path) == 1 else None
        else:
            # imported local alias?
            for vis, attrs, ents in m.uses:
                for e in ents:
                    if not e['glob'] and e['local'] == first and tuple(e['path']) != tuple(path[:len(e['path'])]) :
                        pass
            return ('ext', tuple(path))
    while i < len(path):
        seg = path[i]
        mm = mods[node]
        if seg in mm.children and i < len(path):
            node = mm.children[seg]; i += 1; continue
        if seg in mm.defs and i == len(path) - 1:
            return ('item', node, seg)
        if i == len(path) - 1:
            return ('item_any', node, seg)
        return None
    return ('mod', node)

def exports(mods):
    ex = {p: {} for p in mods}   # name -> origin
    for p, m in mods.items():
        for n, (k, vis, cfg) in m.defs.items():
            if vis == 'pub': ex[p][n] = ('item', p, n)
        for n, cp in m.children.items():
            if mods[cp].pub: ex[p][n] = ('mod', cp)
    changed = True
    while changed:
        changed = False
        for p, m in mods.items():
            for vis, attrs, ents in m.uses:
                if vis != 'pub': continue
                for e in ents:
                    if e['glob']:
                        r = resolve_path(mods, p, e['path'])
                        if r and r[0] == 'mod':
                            for n, o in ex[r[1]].items():
                                if n not in ex[p]: ex[p][n] = o; changed = True
                        continue
                    path = list(e['path'])
                    if e.get('self'):
                        pass
                    r = resolve_path(mods, p, path)
                    if r is None: continue
                    if r[0] == 'item_any':
                        o = ex[r[1]].get(r[2])
                        if o is None: continue
                        r = o
                    elif r[0] == 'item':
                        r = ('item', r[1], r[2])
                    local = e['local']
                    if r[0] == 'ext' and len(path) == 1 and path[0] == local: pass
                    if local not in ex[p] or ex[p][local] != r:
                        if local not in ex[p]: ex[p][local] = r; changed = True
    return ex
