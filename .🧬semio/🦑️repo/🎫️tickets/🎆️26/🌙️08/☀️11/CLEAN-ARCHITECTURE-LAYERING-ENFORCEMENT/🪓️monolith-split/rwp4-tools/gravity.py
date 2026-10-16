# 🌍️ WP4 free-item gravity: every free item of the Shell tree goes to the lowest-ranked crate among its users (members, component field types, other free items).
import re, sys, os, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from assign import *
from t1lib import F2S, S2CRATE, COMPS

def run(verbose=True, overrides=None, loaded=None):
    home, files, items = loaded if loaded else analyse(rehomed=True)
    overrides = overrides or {}
    RK = {c: i for i, c in enumerate(RANK)}
    INF = 10**6
    free = [u for u in items if not u.is_member and u.kind not in ('use', 'mod')]
    members = [u for u in items if u.is_member]
    # names
    bytype = collections.defaultdict(list)
    for u in free:
        if u.kind == 'impl': continue
        if u.name and u.name not in GENERIC and len(u.name) > 3 and u.name not in ('_', 'ShellState'): bytype[u.name].append(u)
    impls = collections.defaultdict(list)
    for u in free:
        if u.kind == 'impl' and u.name in bytype: impls[u.name].append(u)
    rx = re.compile(r'(?<![A-Za-z0-9_])(' + '|'.join(sorted(map(re.escape, bytype), key=len, reverse=True)) + r')(?![A-Za-z0-9_])')
    users = collections.defaultdict(set)
    for u in items:
        if not u.text: continue
        for n in set(rx.findall(u.text)):
            users[n].add(u)
    # component field types: text of the ShellState struct per component
    ptext = open(SH + '/🦀️.rs', encoding='utf-8').read()
    psrc = Src(ptext)
    fieldtext = collections.defaultdict(str)
    for it in top_items(psrc):
        if it['kind'] == 'struct' and it['head'].startswith('pub struct ShellState'):
            for i in range(it['first'], it['last'] + 1):
                m = re.match(r'^    (?:pub(?:\([^)]*\))?\s+)?([A-Za-z_]\w*)\s*:(.*)$', psrc.bl[i])
                if m and m.group(1) in F2S: fieldtext[F2S[m.group(1)]] += ' ' + m.group(2)
    struct_users = collections.defaultdict(set)
    for S, tx in fieldtext.items():
        for n in set(rx.findall(tx)): struct_users[n].add(S2CRATE[S])
    h = {}
    for n, us in bytype.items():
        for u in us: h[id(u)] = INF
    def unit_rank(u):
        if u.is_member: return RK[u.crate]
        if u.kind == 'fn' and u.name in overrides: return RK[overrides[u.name]]
        return h.get(id(u), INF)
    for _ in range(40):
        ch = False
        for n, us in bytype.items():
            best = INF
            for usr in users.get(n, ()):
                if usr in us: continue
                r = unit_rank(usr)
                if r < best: best = r
            for c in struct_users.get(n, ()): best = min(best, RK[c])
            for u in us:
                if u.kind == 'fn' and u.name in overrides: h[id(u)] = RK[overrides[u.name]]; continue
                if best < h[id(u)]: h[id(u)] = best; ch = True
        if ch:
            for n, ims in impls.items():
                pass
        if not ch: break
    assignment = {}
    for n, us in bytype.items():
        for u in us:
            assignment[id(u)] = h[id(u)]
    return files, items, bytype, impls, h, RK, users, struct_users

if __name__ == '__main__':
    files, items, bytype, impls, h, RK, users, struct_users = run()
    cnt = collections.Counter(); ln = collections.Counter(); unused = collections.Counter()
    for n, us in bytype.items():
        for u in us:
            r = h[id(u)]
            c = RANK[r] if r < 10**6 else None
            cnt[c] += 1; ln[c] += u.last - u.first + 1
            if c is None: unused[u.f['slug']] += 1
    for c in RANK + [None]: print(c, cnt[c], ln[c])
    print(unused)
