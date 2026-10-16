#!/usr/bin/env python3
"""🧭️ Path-resolved reference graph v2: expands `use` trees and path expressions (crate::/super::/self::) to (target file, item) pairs."""
import sys, os, json, collections
from pathlib import Path
sys.path.insert(0, os.path.dirname(__file__))
from rsitems import lex
import modtree
BASE = Path(os.environ.get('BASE', '/Users/ueli/Documents/semio/🌎️hub'))
def rel(f):
    try: return str(Path(f).relative_to(BASE))
    except Exception: return f

def build(root):
    out = []
    r = Path(root).resolve()
    modtree.walk(r, 'crate', [], out, {str(r)})
    files, bymod = {}, {}
    for mp, f, cf, n in out:
        if '(DUP)' in f: continue
        files[f] = dict(mod=mp, cfg=cf)
        bymod[mp] = f
    return files, bymod

def parse_use(toks, i):
    """toks[i] is 'use'; return (list of paths (list of segs, possibly ending '*'), next index)."""
    n = len(toks)
    paths = []
    def tree(i, prefix):
        # parse one use tree starting at i; returns next index
        segs = list(prefix)
        while i < n:
            t = toks[i][1]
            if t == '{':
                i += 1
                while i < n and toks[i][1] != '}':
                    i = tree(i, segs)
                    if i < n and toks[i][1] == ',': i += 1
                return i + 1
            if t == '*':
                paths.append(segs + ['*']); return i + 1
            if toks[i][0] == 'id':
                segs.append(t); i += 1
                if i + 1 < n and toks[i][1] == ':' and toks[i+1][1] == ':':
                    i += 2; continue
                if i < n and toks[i][1] == 'as' and toks[i][0] == 'id':
                    i += 2
                paths.append(list(segs)); return i
            if t == ':':
                i += 1; continue
            i += 1
        return i
    i += 1
    i = tree(i, [])
    return paths, i

def resolve(modpath, segs, bymod):
    base = modpath.split('::')
    if segs[0] == 'crate': cur = ['crate']; i = 1
    elif segs[0] == 'self': cur = list(base); i = 1
    elif segs[0] == 'super':
        cur = list(base); i = 0
        while i < len(segs) and segs[i] == 'super':
            cur = cur[:-1]; i += 1
    else: return None
    j = i
    while j < len(segs):
        cand = '::'.join(cur + [segs[j]])
        if cand in bymod: cur = cur + [segs[j]]; j += 1
        else: break
    return '::'.join(cur), segs[j:]

def refs(files, bymod):
    res = collections.defaultdict(lambda: collections.defaultdict(set))
    for f, meta in files.items():
        src = Path(f).read_text()
        toks = [t for t in lex(src) if t[0] != 'comment']
        n = len(toks); i = 0
        while i < n:
            k, x = toks[i][0], toks[i][1]
            if k == 'id' and x == 'use' and not (i > 0 and toks[i-1][1] in ('.',)):
                paths, j = parse_use(toks, i)
                for p in paths:
                    if p and p[0] in ('crate', 'self', 'super'):
                        r = resolve(meta['mod'], p, bymod)
                        if r and r[0] in bymod and bymod[r[0]] != f:
                            res[f][bymod[r[0]]].add('::'.join(r[1]) or '(module)')
                i = j; continue
            if k == 'id' and x in ('crate', 'super', 'self') and not (i > 0 and toks[i-1][1] == ':'):
                segs = [x]; j = i + 1
                while j + 2 < n and toks[j][1] == ':' and toks[j+1][1] == ':' and toks[j+2][0] == 'id':
                    segs.append(toks[j+2][1]); j += 3
                if len(segs) > 1:
                    r = resolve(meta['mod'], segs, bymod)
                    if r and r[0] in bymod and bymod[r[0]] != f:
                        res[f][bymod[r[0]]].add('::'.join(r[1][:1]) or '(module)')
                i = j; continue
            i += 1
    return res

if __name__ == '__main__':
    files, bymod = build(sys.argv[1])
    res = refs(files, bymod)
    out = {rel(f): {rel(g): sorted(v) for g, v in gs.items()} for f, gs in res.items()}
    json.dump(out, open(sys.argv[2], 'w'), ensure_ascii=False)
    json.dump({rel(f): m for f, m in files.items()}, open(sys.argv[2].replace('.json', '-files.json'), 'w'), ensure_ascii=False)
    print(len(files), 'files', sum(len(v) for v in out.values()), 'edges')
