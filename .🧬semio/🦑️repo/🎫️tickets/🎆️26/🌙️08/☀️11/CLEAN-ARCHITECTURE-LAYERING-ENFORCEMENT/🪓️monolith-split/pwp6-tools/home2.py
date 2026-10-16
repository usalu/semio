import sys; sys.path.insert(0, '.')
from an6 import *
TIER1 = ['constructor', 'stores', 'roster', 'ingress', 'interaction', 'history', 'hostevent', 'txn', 'children', 'pending', 'tools', 'media', 'ingest']
FOREIGN = {'toolmachine', 'timetravel', 'toolrun'}
TURN_FILES = {'⏪️time-travel/🧵️instance/🦀️.rs': 'time-travel-turns', '⏯️tool-run/🧵️instance/🦀️.rs': 'tool-run-turns'}
MAX_COMPS = 3   # own component + at most two lower components (params)

def home2(model=None):
    recs, fcomp, uniq0, model = analyse(model)
    rk = {c: i for i, c in enumerate(COMP_ORDER)}; USE = len(COMP_ORDER)
    inh = [m for m in recs if m['trait'] != 'PluginApp']
    byn = collections.defaultdict(list)
    for m in inh: byn[m['name']].append(m)
    pa = {m['name']: m for m in recs if m['trait'] == 'PluginApp'}
    uniq = {n: v[0] for n, v in byn.items() if len(v) == 1}
    # resolution of a call name: inherent unique, else trait method (if not shadowed by >1 inherent)
    def callee(n):
        if n in uniq: return uniq[n]
        if n in byn: return None
        return pa.get(n)
    for m in recs:
        m['rank2'] = m['rank'] if m['trait'] != 'PluginApp' else USE + 1
        m['why'] = ''
        # composer-level: Drop impl, Self-constructing
        if m['trait'] == 'Drop': m['rank2'] = USE + 1; m['why'] = 'drop'
        elif re.search(r'\bSelf\s*\{', m['body']) or re.search(r'->\s*Self\b', m['sig']) or re.search(r'->\s*Result<\s*Self\b', m['sig']):
            m['rank2'] = USE + 1; m['why'] = 'constructs-composer'
        if m['home'] in FOREIGN and m['rank2'] < USE: m['rank2'] = USE; m['why'] = 'foreign-type-component'
    def fix():
        ch = True
        while ch:
            ch = False
            for m in recs:
                for cn in m['calls']:
                    t = callee(cn)
                    if t and t is not m and t['rank2'] > m['rank2']: m['rank2'] = t['rank2']; ch = True; m['why'] = m['why'] or 'calls-' + t['name']
    fix()
    memo = {}
    def need(m, stack=()):
        k = id(m)
        if k in memo: return memo[k]
        if k in stack: return set(m['dcomps'])
        s = set(m['dcomps'])
        for cn in m['calls']:
            t = callee(cn)
            if t and t is not m and 0 <= t['rank2'] < USE:
                s.add(COMP_ORDER[t['rank2']]); s |= need(t, stack + (k,))
        memo[k] = s; return s
    for it in range(8):
        memo.clear(); changed = False
        for m in recs:
            if 0 <= m['rank2'] < USE:
                n = need(m) - {'core'}
                if len(n) > MAX_COMPS:
                    m['rank2'] = USE; m['why'] = m['why'] or 'needs-%d-components' % len(n); changed = True
        if not changed: break
        fix()
    memo.clear()
    for m in recs:
        r = m['rank2']
        m['home2'] = COMP_ORDER[r] if 0 <= r < USE else ('free' if r < 0 else 'usecase' if r == USE else 'pluginapp')
        m['need2'] = sorted(need(m), key=lambda c: rk[c]) if 0 <= r < USE else []
    return recs, uniq, callee, model

if __name__ == '__main__':
    recs, uniq, callee, model = home2()
    tot = collections.Counter(); t1 = collections.Counter()
    for m in recs: tot[m['home2']] += m['lines']
    print(dict(tot.most_common()))
    old = collections.Counter(m['home'] for m in recs)
    chg = collections.Counter((m['home'], m['home2']) for m in recs if m['home'] != m['home2'])
    print('changes', sorted(chg.items(), key=lambda kv: -kv[1])[:30])
    print('usecase because:', collections.Counter(m['why'].split('-')[0] + ('-' + m['why'].split('-')[1] if '-' in m['why'] else '') for m in recs if m['home2'] == 'usecase' and m['home'] != 'usecase').most_common(10))
    hist = collections.Counter(len([c for c in m['need2'] if c != m['home2']]) for m in recs if m['home2'] in TIER1)
    print('tier1 params hist', sorted(hist.items()))

from crates6 import HOME_CRATE, FILE_USECASE
def crate_of(m):
    h = m['home2']
    if m['file'] in TURN_FILES: return TURN_FILES[m['file']]
    if h in HOME_CRATE and h not in ('timetravel', 'toolrun'): return HOME_CRATE[h]
    if h in ('timetravel',): return 'time-travel-turns'
    if h in ('toolrun',): return 'tool-run-turns'
    return FILE_USECASE[m['file']]
