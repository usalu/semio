import sys; sys.path.insert(0, '.')
import home2 as H
from home2 import *
from upcalls6 import RANKC
H.TURN_FILES.clear()
FIELD_CRATE = {'constructor': 'instance-stores', 'stores': 'instance-stores', 'roster': 'instance-stores', 'ingress': 'instance-stores', 'interaction': 'instance-stores',
 'history': 'instance-history', 'hostevent': 'instance-history', 'txn': 'instance-history', 'children': 'instance-children',
 'pending': 'instance-jobs', 'tools': 'instance-jobs', 'toolmachine': 'instance-jobs', 'media': 'instance-docs', 'ingest': 'instance-docs',
 'maintenance': 'instance-ladders', 'close': 'instance-ladders', 'timetravel': 'time-travel-turns', 'toolrun': 'tool-run-turns', 'core': 'instance-app'}
DEF_NAME = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:(?:async|const|unsafe|extern\s+"[^"]*")\s+)*(?:fn|struct|enum|trait|type|static|const|union)\s+(\w+)')
def tail_rank(c): return RANKC[c]

def assign():
    recs, uniq, callee, model = H.home2()
    for m in recs: m['crate'] = H.crate_of(m)
    # free (no-self) associated functions: lowest caller crate
    callers = collections.defaultdict(list)
    for m in recs:
        for cn in m['calls']:
            t = callee(cn)
            if t and t is not m: callers[id(t)].append(m)
    for it in range(6):
        changed = False
        for m in recs:
            if m['home2'] != 'free': continue
            cs = {c['crate'] for c in callers[id(m)]}
            if not cs: new = 'instance-history'
            else:
                t1 = [c for c in cs if RANKC[c] <= 5]
                if t1: new = min(t1, key=RANKC.get)
                elif len(cs) == 1: new = next(iter(cs))
                else: new = 'instance-docs'
                # never above one of its callers
            if new != m['crate']: m['crate'] = new; changed = True
        if not changed: break
    return recs, uniq, callee, model, callers

if __name__ == '__main__':
    recs, uniq, callee, model, callers = assign()
    tot = collections.Counter()
    for m in recs: tot[m['crate']] += m['lines']
    print(dict(tot))
    from upcalls6 import violations
    v = violations(recs, callee)
    print(len(v), 'cross-crate non-downward calls')
    c = collections.Counter((x[2], x[4]) for x in v)
    for k, n in sorted(c.items(), key=lambda kv: -kv[1])[:12]: print(n, k)
    for x in v:
        if RANKC[x[2]] <= 5 or x[4] in ('instance-stores','instance-history','instance-children','instance-jobs','instance-docs'): print('TIER1-RELATED', x)
