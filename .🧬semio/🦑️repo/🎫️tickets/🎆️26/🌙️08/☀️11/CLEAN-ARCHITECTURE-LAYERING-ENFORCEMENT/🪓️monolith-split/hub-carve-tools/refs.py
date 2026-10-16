#!/usr/bin/env python3
"""🕸️ Name-based file reference graph for a crate's module tree: edge F->G when F mentions a top-level item name defined in G (and not defined in F)."""
import sys, os, json, collections
from pathlib import Path
sys.path.insert(0, os.path.dirname(__file__))
from rsitems import lex, items, name_kind
import modtree

BASE = Path(os.environ.get('BASE', '/Users/ueli/Documents/semio/🌎️hub'))

def load(root):
    out = []
    modtree.walk(Path(root).resolve(), 'crate', [], out, {str(Path(root).resolve())})
    files = {}
    for mp, f, cf, n in out:
        if '(DUP)' in f: continue
        files[f] = dict(mod=mp, cfg=cf)
    return files

def analyse(files):
    defs = collections.defaultdict(list)
    info = {}
    for f, meta in files.items():
        src = Path(f).read_text()
        toks = lex(src)
        ids = collections.Counter(t[1] for t in toks if t[0] == 'id')
        strs = ' '.join(t[1] for t in toks if t[0] == 'str')
        its = items(src)
        mine = {}
        for it in its:
            try:
                v, k, nm = name_kind(it['toks'])
            except Exception:
                continue
            if nm and k not in ('impl', 'use', 'mod', 'include'):
                mine[nm] = (v, k)
        info[f] = dict(ids=ids, defs=mine, strs=strs)
        for nm in mine:
            defs[nm].append(f)
    return defs, info

def rel(f):
    try: return str(Path(f).relative_to(BASE))
    except Exception: return f

if __name__ == '__main__':
    root = sys.argv[1]
    files = load(root)
    defs, info = analyse(files)
    edges = collections.defaultdict(lambda: collections.defaultdict(set))
    for f, d in info.items():
        for nm, c in d['ids'].items():
            if nm in d['defs']: continue
            if nm in defs and len(nm) > 3:
                for g in defs[nm]:
                    if g != f:
                        edges[f][g].add(nm)
    json.dump({rel(f): {rel(g): sorted(n) for g, n in gs.items()} for f, gs in edges.items()}, open('/tmp/hubw/edges.json', 'w'), ensure_ascii=False)
    json.dump({rel(f): dict(mod=files[f]['mod'], cfg=files[f]['cfg'], defs={k: v for k, v in info[f]['defs'].items()}) for f in files}, open('/tmp/hubw/files.json', 'w'), ensure_ascii=False)
    print(len(files), 'files', sum(len(v) for v in edges.values()), 'edges')
