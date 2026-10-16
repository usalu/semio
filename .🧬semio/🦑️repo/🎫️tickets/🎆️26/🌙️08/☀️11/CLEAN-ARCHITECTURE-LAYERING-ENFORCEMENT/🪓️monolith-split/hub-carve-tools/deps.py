#!/usr/bin/env python3
"""🕸️ Resolver-based dependency graph: for every file module, the items of other file modules it names (through scope, use bindings and globs)."""
import sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod
from pathlib import Path

BASE = Path(os.environ.get('BASE', '/Users/ueli/Documents/semio/🌎️hub'))

def rel(f):
    try:
        return str(Path(f).relative_to(BASE))
    except Exception:
        return f

def inline_descendants(fm):
    for c in fm.children.values():
        if c.inline:
            yield c
            yield from inline_descendants(c)


def inline_ranges(m, out):
    for c in m.children.values():
        if c.inline:
            a, b = c.item.tok_range
            out.append((a, b, c))
            inline_ranges(c, out)
    return out

def defining_file(res):
    if res is None or res.mod is None:
        return None
    return res.mod.file_root()

def edges_of(root, modmap):
    edges = collections.defaultdict(lambda: collections.defaultdict(lambda: collections.defaultdict(int)))
    for fm in [m for m in hcmod.walk(root) if not m.inline]:
        if getattr(fm, 'missing', False):
            continue
        toks = fm.toks
        ranges = sorted(inline_ranges(fm, []), key=lambda r: (r[0], -r[1]))
        n = len(toks)
        # skip use items' own tokens (their targets are resolved through uses below), but count the leaves as edges
        use_spans = [(u.item.tok_range) for u in fm.uses]
        for m in [fm] + list(inline_descendants(fm)):
            for u in m.uses:
                if u.path and u.path[0] in ('crate', 'self', 'super') or (u.path and hcmod.lookup(m, u.path[0]) is not None):
                    r = hcmod.resolve(m, u.path + ([] if not u.glob else []), None, True)
                    if u.glob:
                        if r is not None and r.kind == 'mod' and r.mod is not None:
                            d = r.mod.file_root()
                            if d is not fm:
                                edges[fm.file][d.file]['*'] += 1
                    elif r is not None and r.mod is not None and r.kind in ('item', 'mod', 'variant'):
                        d = r.mod.file_root() if r.kind != 'mod' else r.mod.file_root()
                        if d is not fm:
                            edges[fm.file][d.file][u.path[-1]] += 1
        i = 0
        skip_to = -1
        us = sorted(use_spans)
        while i < n:
            t = toks[i]
            if t.k != 'id' or t.x in hcmod.KEYWORDS - {'crate', 'self', 'super'}:
                i += 1
                continue
            # inside a use item: skip
            if any(a <= i < b for a, b in us):
                i += 1
                continue
            prev = toks[i - 1].x if i > 0 else ''
            prevk = toks[i - 1].k if i > 0 else ''
            if prev == '.' and prevk == 'p' and not (i > 1 and toks[i - 2].x == '.'):
                i += 1
                continue
            if prev == ':' and i > 1 and toks[i - 2].x == ':' and toks[i - 2].k == 'p':
                i += 1
                continue
            # path starting here
            segs = [t.x]
            j = i + 1
            while j + 2 < n + 1 and j + 1 < n and toks[j].x == ':' and toks[j + 1].x == ':' and j + 2 < n and toks[j + 2].k == 'id':
                segs.append(toks[j + 2].x)
                j += 3
            scope = fm
            for a, b, c in ranges:
                if a <= i < b:
                    scope = c
            r = hcmod.resolve(scope, segs, None, False)
            if r is not None and r.kind in ('item', 'mod', 'variant') and r.mod is not None:
                d = r.mod.file_root()
                if d is not fm and not (segs[0] in ('self', 'super') and False):
                    edges[fm.file][d.file][r.name if r.kind != 'mod' else '(mod ' + r.name + ')'] += 1
            i = j if j > i + 1 else i + 1
    return edges

if __name__ == '__main__':
    root, modmap = hcmod.load_crate(sys.argv[1])
    edges = edges_of(root, modmap)
    out = {rel(f): {rel(g): sorted(v) for g, v in gs.items()} for f, gs in edges.items()}
    files = {}
    for m in hcmod.walk(root):
        if not m.inline:
            files[rel(m.file)] = dict(mod=m.path, cfg=[c for mm in hcmod.modpath_chain(m) for c in mm.cfg])
    json.dump(out, open(sys.argv[2], 'w'), ensure_ascii=False)
    json.dump(files, open(sys.argv[2].replace('.json', '-files.json'), 'w'), ensure_ascii=False)
    print(len(files), 'files', sum(len(v) for v in out.values()), 'file edges')
