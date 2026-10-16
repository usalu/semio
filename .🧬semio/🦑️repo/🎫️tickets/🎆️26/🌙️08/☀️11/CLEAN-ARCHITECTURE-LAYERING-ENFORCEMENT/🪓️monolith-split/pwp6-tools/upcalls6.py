import sys; sys.path.insert(0, '.')
from home2 import *
RANKC = {'instance-stores': 1, 'instance-history': 2, 'instance-children': 3, 'instance-jobs': 4, 'instance-docs': 5, 'instance-commands': 6, 'instance-typed-ops': 6, 'instance-render': 6, 'instance-ladders': 6, 'time-travel-turns': 6, 'tool-run-turns': 6, 'instance-app': 7}
def violations(recs, callee):
    out = []
    for m in recs:
        for cn in m['calls']:
            t = callee(cn)
            if not t or t is m: continue
            a, b = m['crate'], t['crate']
            if a == b: continue
            if RANKC[b] >= RANKC[a] or (RANKC[a] == RANKC[b]):
                out.append((m['file'].split('/')[-2], m['name'], a, cn, b))
    return out
if __name__ == '__main__':
    recs, uniq, callee, model = home2()
    for m in recs: m['crate'] = crate_of(m)
    v = violations(recs, callee)
    print(len(v))
    for x in v[:80]: print(x)
