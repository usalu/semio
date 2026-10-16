import os, sys, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import MARK
def region_map(path):
    st = []; out = collections.Counter(); names = collections.Counter()
    for ln in open(path, encoding='utf-8').read().split('\n'):
        m = MARK.match(ln)
        if m:
            if m.group(2):
                if st: st.pop()
            else: st.append(m.group(3)); names[m.group(3)] += 1
            continue
        s = ln.strip()
        if not s or s.startswith('//'): continue
        for r in set(st): out[(r, s.replace('pub(super)', 'pub(crate)'))] += 1
    return out, names
def run(base, base_list, new, new_list):
    B = collections.Counter(); N = collections.Counter(); bn = collections.Counter(); nn = collections.Counter()
    for rel in base_list:
        p = os.path.join(base, rel)
        if rel.endswith('.rs') and os.path.exists(p):
            o, n = region_map(p); B.update(o); bn.update(n)
    for rel in new_list:
        p = os.path.join(new, rel)
        if rel.endswith('.rs') and os.path.exists(p):
            o, n = region_map(p); N.update(o); nn.update(n)
    return B, N, bn, nn
if __name__ == '__main__':
    base, new, bl, census = sys.argv[1:5]
    base_list = [l.strip() for l in open(bl, encoding='utf-8') if l.strip()]
    new_list = sorted({p for r in json.load(open(census)) for p, k, b, l in r.get('list', []) if p.endswith('.rs')} | {'🧰️framework/📦️packages/🦀️rust/🦀️.rs', '🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs', '🧰️framework/🔨️modules/🧬️schema/📽️projection/🧪️tests/🦀️.rs', '🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧩️framework-reserved/🦀️.rs', '🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧰️testkit/🦀️.rs'})
    B, N, bn, nn = run(base, base_list, new, new_list)
    miss = B - N
    byreg = collections.Counter()
    for (r, s), c in miss.items(): byreg[r] += c
    print('regions with missing enclosed code lines:', len(byreg))
    for r, c in byreg.most_common(30): print(' ', c, r)
    print('region names in base not in new:', sorted(set(bn) - set(nn))[:30])
    print('region names in new not in base:', sorted(set(nn) - set(bn))[:30])
    print('total marker lines base', sum(bn.values()), 'new', sum(nn.values()))
