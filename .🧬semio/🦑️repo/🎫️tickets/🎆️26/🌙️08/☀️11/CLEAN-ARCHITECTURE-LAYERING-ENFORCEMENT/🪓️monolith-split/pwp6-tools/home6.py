import sys; sys.path.insert(0, '.')
from methods6 import *
COMP_ORDER = pt.COMP_ORDER
def analyse():
    fs = parse_fields(); fcomp = {f['name']: comp_of(f['name']) for f in fs}; fn = set(fcomp)
    ms = extract()
    by = collections.defaultdict(list)
    for m in ms: by[m['name']].append(m)
    uniq = {n: v[0] for n, v in by.items() if len(v) == 1}
    rk = {c: i for i, c in enumerate(COMP_ORDER)}; USE = len(COMP_ORDER)
    for m in ms:
        m['lines'] = m['end'] - m['attr_start']
        m['fields'] = sorted(set(re.findall(r'\bself\.(\w+)', m['body'])) & fn)
        m['calls'] = sorted(set(re.findall(r'\b(?:self|Self)(?:\.|::)(\w+)\s*(?:\(|::<)', m['body'])))
        m['dcomps'] = sorted({fcomp[f] for f in m['fields']}, key=lambda c: rk[c])
        cs = set(m['dcomps']); base = max([rk[c] for c in cs], default=-1)
        if len(cs - {'core'}) >= 4: base = USE
        if m['trait'] == 'PluginApp': base = USE + 1
        m['rank'] = base
    ch = True
    while ch:
        ch = False
        for m in ms:
            for cn in m['calls']:
                t = uniq.get(cn)
                if t and t is not m and t['rank'] > m['rank']: m['rank'] = t['rank']; ch = True
    for m in ms:
        r = m['rank']; m['home'] = COMP_ORDER[r] if 0 <= r < USE else ('free' if r < 0 else 'usecase' if r == USE else 'pluginapp')
    # transitive component need
    memo = {}
    def need(m, stack=()):
        k = id(m)
        if k in memo: return memo[k]
        if k in stack: return set(m['dcomps'])
        s = set(m['dcomps'])
        for cn in m['calls']:
            t = uniq.get(cn)
            if t and t is not m:
                if t['home'] in COMP_ORDER: s.add(t['home'])
                s |= need(t, stack + (k,))
        memo[k] = s; return s
    for m in ms: m['need'] = sorted(need(m), key=lambda c: rk[c])
    return ms, fcomp, uniq
if __name__ == '__main__':
    ms, fcomp, uniq = analyse()
    tot = collections.Counter(); 
    for m in ms: tot[m['home']] += m['lines']
    print(dict(tot.most_common()))
    # param count for tier-1 homed
    hist = collections.Counter()
    for m in ms:
        if m['home'] in COMP_ORDER:
            np_ = len([c for c in m['need'] if c != m['home']])
            hist[np_] += 1
    print('param histogram (tier1 homed):', sorted(hist.items()))
    hist2 = collections.Counter()
    for m in ms:
        if m['home'] in COMP_ORDER:
            hist2[len(set(m['need']))] += m['lines']
    print('lines by |need|:', sorted(hist2.items()))
    unresolved = collections.Counter()
    for m in ms:
        for cn in m['calls']:
            if cn not in uniq: unresolved[cn] += 1
    print('unresolved self-calls:', unresolved.most_common(40))
