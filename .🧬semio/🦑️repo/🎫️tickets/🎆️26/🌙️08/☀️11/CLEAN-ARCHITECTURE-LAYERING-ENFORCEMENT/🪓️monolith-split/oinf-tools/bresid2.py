"""🔎 Residual method-call scan: moved host methods still called on a receiver that is not the owning component."""
import sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import bdec
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"

def rs(root):
    out = []
    for dp, dn, fn in os.walk(root):
        if 'node_modules' in dp: continue
        out += [os.path.join(dp, f) for f in fn if f.endswith('.rs')]
    return out

def main(roots):
    comps = {}
    narrow = {}
    for h, mf in (('BoardHost', 'b-meta-board.json'), ('DagHost', 'b-meta-dag.json')):
        for x in json.load(open(f'{SC}/{mf}')):
            if x['kind'] != 'method' or x['parent'] != h: continue
            if x['mode'] == 'comp': comps.setdefault(x['name'], set()).add((h, x['comps'][0]))
            elif x['mode'] in ('narrow', 'free'): narrow.setdefault(x['oldname'], set()).add(h)
    files = [f for r in roots for f in rs(r)]
    defs = collections.Counter()
    deftexts = {}
    for p in files:
        t = open(p, encoding='utf-8').read(); toks = bdec.ctoks(t); deftexts[p] = (t, toks)
    out = []
    for p, (t, toks) in deftexts.items():
        for i, x in enumerate(toks):
            if x.k != 'id' or i < 2 or toks[i-1].s != '.' or toks[i+1].s != '(': continue
            if x.s in narrow:
                out.append(('narrow', p, t.count('\n', 0, x.a) + 1, x.s, t[toks[i-3].a:x.b] if i >= 3 else ''))
            elif x.s in comps:
                hosts = {c for h, c in comps[x.s]}
                prev = toks[i-2].s
                if prev in hosts: continue
                out.append(('comp', p, t.count('\n', 0, x.a) + 1, x.s, t[toks[max(i-4, 0)].a:x.b]))
    return out

if __name__ == '__main__':
    res = main(sys.argv[1:])
    c = collections.Counter((r[0], r[3]) for r in res)
    print(len(res))
    for k, v in sorted(c.items()): print(k, v)
    json.dump(res, open(SC + '/resid2.json', 'w'))
