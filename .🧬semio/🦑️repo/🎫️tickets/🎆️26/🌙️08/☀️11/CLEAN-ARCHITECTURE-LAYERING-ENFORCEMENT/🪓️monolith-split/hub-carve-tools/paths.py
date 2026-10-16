#!/usr/bin/env python3
"""🧭️ Path-resolved reference graph: crate::/super::/self:: paths (and glob parents) resolved to module-tree files."""
import sys, os, json, collections
from pathlib import Path
sys.path.insert(0, os.path.dirname(__file__))
from rsitems import lex
import modtree
BASE = Path(os.environ.get('BASE', '/Users/ueli/Documents/semio/🌎️hub'))
def rel(f):
    try: return str(Path(f).relative_to(BASE))
    except Exception: return f

def build(root, cratename='crate'):
    out = []
    r = Path(root).resolve()
    modtree.walk(r, 'crate', [], out, {str(r)})
    files = {}
    bymod = {}
    for mp, f, cf, n in out:
        if '(DUP)' in f: continue
        files[f] = dict(mod=mp, cfg=cf)
        bymod[mp] = f
    return files, bymod

def resolve(modpath, segs, bymod):
    base = modpath.split('::')
    i = 0
    cur = None
    if segs[0] == 'crate':
        cur = ['crate']; i = 1
    elif segs[0] == 'self':
        cur = list(base); i = 1
    elif segs[0] == 'super':
        cur = list(base)
        while i < len(segs) and segs[i] == 'super':
            cur = cur[:-1]; i += 1
    else:
        return None
    best = '::'.join(cur) if '::'.join(cur) in bymod else None
    j = i
    tail = []
    while j < len(segs):
        cand = '::'.join(cur + [segs[j]])
        if cand in bymod:
            cur = cur + [segs[j]]; best = cand; j += 1
        else:
            break
    rest = segs[j:]
    return '::'.join(cur), rest

def refs(files, bymod):
    res = collections.defaultdict(lambda: collections.defaultdict(set))
    for f, meta in files.items():
        src = Path(f).read_text()
        toks = [t for t in lex(src) if t[0] != 'comment']
        n = len(toks)
        i = 0
        while i < n:
            k, x = toks[i][0], toks[i][1]
            if k == 'id' and x in ('crate', 'super', 'self') and not (i > 0 and toks[i-1][1] == ':' and toks[i-2][1] == ':'):
                segs = [x]; j = i + 1
                while j + 2 < n + 1 and j + 1 < n and toks[j][1] == ':' and toks[j+1][1] == ':':
                    if j + 2 < n and toks[j+2][0] == 'id':
                        segs.append(toks[j+2][1]); j += 3
                    elif j + 2 < n and toks[j+2][1] == '{':
                        segs.append('{'); break
                    elif j + 2 < n and toks[j+2][1] == '*':
                        segs.append('*'); break
                    else: break
                if len(segs) > 1:
                    r = resolve(meta['mod'], segs, bymod)
                    if r:
                        target, rest = r
                        if target in bymod and bymod[target] != f:
                            res[f][bymod[target]].add('::'.join(rest[:2]) if rest else '')
                i = j
            else:
                i += 1
    return res

if __name__ == '__main__':
    files, bymod = build(sys.argv[1])
    res = refs(files, bymod)
    out = {rel(f): {rel(g): sorted(v) for g, v in gs.items()} for f, gs in res.items()}
    json.dump(out, open('/tmp/hubw/pedges.json', 'w'), ensure_ascii=False)
    json.dump({rel(f): m for f, m in files.items()}, open('/tmp/hubw/files.json', 'w'), ensure_ascii=False)
    print(len(files), 'files', sum(len(v) for v in out.values()), 'edges')
