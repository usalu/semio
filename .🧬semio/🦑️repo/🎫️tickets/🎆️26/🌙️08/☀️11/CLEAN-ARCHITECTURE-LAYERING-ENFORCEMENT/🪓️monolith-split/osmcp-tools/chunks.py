import re, sys, collections
sys.path.insert(0, ".")
from agraph import *

REG = re.compile(r'^\s*//#(end)?region\s+(\S.*?)\s*$', re.M)

def region_map(f):
    """Return list of (start_line, end_line, name) leaf regions for the file."""
    stack = []; spans = []
    for m in REG.finditer(f.src):
        ln = f.src.count('\n', 0, m.start()) + 1
        if m.group(1):
            if stack:
                nm, st, depth = stack.pop()
                spans.append((st, ln, nm, depth))
        else:
            stack.append((m.group(2), ln, len(stack)))
    return spans

def region_of(spans, line):
    best = None
    for (s, e, nm, depth) in spans:
        if s <= line <= e:
            if best is None or depth > best[3]: best = (s, e, nm, depth)
    return best[2] if best else "(rest)"

def chunk_of(a, cache={}):
    f = a.file
    if f.path not in cache: cache[f.path] = region_map(f)
    nm = region_of(cache[f.path], a.item.line0)
    short = "/".join(f.modpath[1:]) or "(glue)"
    return f"{short}#{nm}"

if __name__ == "__main__":
    files, allitems, atoms, defs, loose = build()
    edges, amb, flags = atom_edges(files, atoms, defs)
    byid = {id(a): a for a in atoms}
    prod = [a for a in atoms if not flags[id(a)] and a.kind != 'mod']
    ch = {id(a): chunk_of(a) for a in prod}
    csize = collections.Counter()
    for a in prod: csize[ch[id(a)]] += a.text.count("\n") + 1
    cedges = collections.defaultdict(set)
    for s, ts in edges.items():
        if s not in ch: continue
        for t in ts:
            if t in ch and ch[s] != ch[t]: cedges[ch[s]].add(ch[t])
    nodes = sorted(csize)
    comps = tarjan(nodes, cedges)
    print("chunks", len(nodes), "sccs>1:")
    for c in comps:
        if len(c) > 1: print("  ", sorted(c), sum(csize[x] for x in c))
