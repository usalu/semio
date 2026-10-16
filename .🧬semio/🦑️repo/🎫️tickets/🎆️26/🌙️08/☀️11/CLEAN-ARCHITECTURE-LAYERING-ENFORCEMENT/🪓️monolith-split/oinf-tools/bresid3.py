"""🔎 Per-host residual scan: a moved host method still called on a receiver that is neither its component nor a known component-internal `self`."""
import sys, os, json, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bdec
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"

def rs(root):
    out = []
    for dp, dn, fn in os.walk(root):
        out += [os.path.join(dp, f) for f in fn if f.endswith('.rs')]
    return out

def host_sets(h, mf):
    meta = json.load(open(f'{SC}/{mf}'))
    comp = collections.defaultdict(set); narrow = {}; owner = set(); allc = set()
    for x in meta:
        if x['parent'] != h and x['kind'] != 'ext': continue
        if x['mode'] == 'owner': owner.add(x['oldname'])
        elif x['kind'] == 'method' and x['mode'] == 'comp': comp[x['name']].add(x['comps'][0])
        elif x['kind'] == 'method': narrow[x['oldname']] = x['name']
    return comp, narrow, owner

def scan(h, mf, files):
    comp, narrow, owner = host_sets(h, mf)
    defined = collections.Counter()
    for p in files:
        for m in re.finditer(r'\bfn\s+([A-Za-z_][A-Za-z0-9_]*)', open(p, encoding='utf-8').read()): defined[m.group(1)] += 1
    out = []
    for p in files:
        t = open(p, encoding='utf-8').read()
        for m in re.finditer(r'([A-Za-z_][A-Za-z0-9_]*|\)|\])\s*\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*\(', t):
            rid, n = m.group(1), m.group(2)
            if n in owner: continue
            if n in comp and rid not in comp[n] and rid != 'self': out.append(('comp', p, t.count('\n', 0, m.start()) + 1, rid, n))
            elif n in narrow: out.append(('narrow', p, t.count('\n', 0, m.start()) + 1, rid, n))
    return out

if __name__ == '__main__':
    inf = sys.argv[1]
    nfiles = [p for p in rs(inf) if '➕️normal' in p]
    dfiles = [p for p in rs(inf) if '🕸️dag' in p]
    for h, mf, fl in (('BoardHost', 'b-meta-board.json', nfiles), ('DagHost', 'b-meta-dag.json', dfiles)):
        res = scan(h, mf, fl)
        print(h, len(res))
        c = collections.Counter((k, r, n) for k, p, l, r, n in res)
        for key, v in sorted(c.items(), key=lambda x: -x[1])[:60]: print(' ', key, v)
