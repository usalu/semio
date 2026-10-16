# 🗺️ Chunk-level crate assignment for the WP4 production files (planner assignment + overrides4.json), printed or returned for the cut engine.
import sys, json, collections
from seg import *
import model4
exec(open(HERE + '/dag4.py').read().split("# adjacency with toks2")[0])   # items, defs, owner2, crate (units), unit()
byfile = collections.defaultdict(list)
for it in items: byfile[it['file']].append(it)

def chunk_crate(f, c):
    its = [it for it in byfile[f] if it['s'] >= c.s - 0 and it['e'] <= c.e and it['kind'] not in ('use',)]
    # the chunk's first-class item is the one starting nearest to the chunk head
    its = [it for it in its if it['kind'] != 'static' or 'body' in it or True]
    if c.kind in ('use', 'moddoc', 'externcrate', 'filler') or c.kind.startswith('region'): return None
    best = None
    for it in sorted(its, key=lambda x: x['s']):
        u = unit(it['id'])
        if u in crate: best = crate[u]; break
    return best

if __name__ == '__main__':
    rel = sys.argv[1]
    chunks = check(PLUG + rel)
    cur = None
    for c in chunks:
        cc = chunk_crate(rel, c)
        print(f'{c.s:5d}-{c.e:5d} {c.kind:12s} {(c.name or c.target or "")[:44]:44s} {cc}')
