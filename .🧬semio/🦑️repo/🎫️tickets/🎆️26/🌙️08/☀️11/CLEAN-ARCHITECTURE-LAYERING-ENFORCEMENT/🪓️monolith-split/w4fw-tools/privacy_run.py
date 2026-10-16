import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from privacy import *
def run(W, census):
    own = cluster_files(census, W)
    masked = {}; byc = collections.defaultdict(list)
    for p, cs in own.items():
        t = open(W + p, encoding='utf-8').read()
        masked[p] = (t, mask(t))
        for c in cs: byc[c].append(p)
    cm = {c: '\n'.join(masked[p][1] for p in files) for c, files in byc.items()}
    members = collections.defaultdict(list)
    for c in CLUSTERS:
        for p in byc[c]:
            if '🧪️tests' in p: continue
            for m in private_members(W, '', masked[p][0]): members[c].append(m)
    cands = []
    for c, ms in members.items():
        for kind, tname, mname in set(ms):
            for c2, t in cm.items():
                if c2 == c: continue
                if not re.search(r'\b' + re.escape(tname) + r'\b', t): continue
                if kind == 'field' and (re.search(r'\.' + re.escape(mname) + r'\b(?!\s*\()', t) or re.search(r'\b' + re.escape(mname) + r'\s*:', t)): cands.append((c, tname, mname, c2))
                elif kind == 'method' and re.search(r'(\.|::)' + re.escape(mname) + r'\s*[(:<]', t): cands.append((c, tname, mname, c2))
                elif kind == 'tuple' and re.search(r'\b' + re.escape(tname) + r'\s*\(', t): cands.append((c, tname, '.' + mname, c2))
    return members, sorted(set(cands))
if __name__ == '__main__':
    members, cands = run(sys.argv[1], sys.argv[2])
    print({c: len(v) for c, v in members.items()})
    for x in cands: print(x)
