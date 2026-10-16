# 🏠️ WP4 re-homing: Appendix B first cut -> consistent DAG (zero-touch helpers lowered to their lowest caller, callers raised over upward calls).
import sys, os, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from callgraph import *

def rehome(verbose=True, force=()):
    mem, f2s, order, s2c, comps = member_info()
    RK = {c: k for k, c in enumerate(RANK)}
    home = {n: infos[0]['crate'] for n, infos in mem.items()}
    orig = dict(home)
    pinned_up = set()
    home['new'] = 'shell-state'
    for leaf in ('debug_log', 'instant_now_ms', 'directory_now_ms'): home[leaf] = 'shell-async'
    for n in force:
        if RK[home[n]] < RK['shell-session']: home[n] = 'shell-session'
    pinned_up |= set(force) | {'new', 'debug_log', 'instant_now_ms', 'directory_now_ms'}
    touch = {n: set().union(*[set(i['touched']) for i in infos]) for n, infos in mem.items()}
    calls = {n: (set().union(*[set(i['calls']) for i in infos]) & set(mem)) - {n} for n, infos in mem.items()}
    callers = collections.defaultdict(set)
    for n, cs in calls.items():
        for c in cs: callers[c].add(n)
    lines = {n: sum(i['lines'] for i in infos) for n, infos in mem.items()}
    S2C = s2c
    for n in home:
        if touch[n]:
            top = max(touch[n], key=lambda s: order.index(s))
            if RK[S2C[top]] > RK[home[n]]: home[n] = S2C[top]
    def lb(n): return max([RK[s2c[s]] for s in touch[n]] + [0 if not calls[n] else -1])
    ch = True; rounds = 0
    while ch:
        ch = False; rounds += 1
        # lower zero-touch helpers to their lowest caller (never below their callees)
        for n in home:
            if touch[n] or not callers[n] or n in pinned_up: continue
            lo = max([RK[home[c]] for c in calls[n]] + [0])
            hi = min(RK[home[x]] for x in callers[n])
            tgt = max(lo, hi)
            if tgt < RK[home[n]]:
                home[n] = RANK[tgt]; ch = True
        # raise: callers over upward calls
        for n in home:
            for c in calls[n]:
                if RK[home[c]] > RK[home[n]]:
                    home[n] = home[c]; ch = True
        if rounds > 50: break
    return mem, f2s, order, s2c, comps, home, orig, touch, calls, callers, lines

if __name__ == '__main__':
    mem, f2s, order, s2c, comps, home, orig, touch, calls, callers, lines = rehome()
    cnt = collections.Counter(); ln = collections.Counter()
    for n in home: cnt[home[n]] += 1; ln[home[n]] += lines[n]
    print('moved', sum(1 for n in home if home[n] != orig[n]))
    for c in RANK: print(c, cnt[c], ln[c])
    json.dump({n: home[n] for n in sorted(home)}, open(HERE + '/../🗑️generated/rwp4-homes.json', 'w'), ensure_ascii=False, indent=0)
