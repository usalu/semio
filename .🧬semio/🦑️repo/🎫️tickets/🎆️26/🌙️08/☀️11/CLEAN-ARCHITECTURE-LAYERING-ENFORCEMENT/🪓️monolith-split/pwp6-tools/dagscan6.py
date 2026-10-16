# crate DAG scan of the instance family: a member/function of crate X may call only members/functions of crates below X.
import sys; sys.path.insert(0, '.')
from lib6 import *
import defscan6 as D, orphan6 as O
RANK = {'instance-stores': 1, 'instance-history': 2, 'instance-children': 3, 'instance-jobs': 4, 'instance-docs': 5, 'instance-commands': 6, 'instance-typed-ops': 6, 'instance-render': 6, 'instance-ladders': 6, 'time-travel-turns': 6, 'tool-run-turns': 6, 'instance-app': 7}

def scan():
    fam = D.family_files()
    mods = {f: D.parse_module(ROOT, f) for f in fam}
    from comps6 import COMPONENTS
    cfs = '|'.join(list(COMPONENTS) + ['self', 'app'])
    rx = re.compile(r'(?<![\w])(?:(?:self|app)\s*\.\s*(?:(?:' + '|'.join(COMPONENTS) + r')\s*\.\s*)?|(?:' + '|'.join(COMPONENTS) + r')\s*\.\s*|(?:Self|VcsArtifactApp)\s*::\s*|[A-Z]\w*\s*::\s*<[^>]*>\s*::\s*)(\w+)\s*(?:::<[^()]*>)?\(')
    for f, m in mods.items(): m['words'], m['calls'] = D.token_sets(m['M']); m['calls'] = set(rx.findall(m['M']))
    defs = collections.defaultdict(list)
    for f, m in mods.items():
        c = O.crate_of_file(f)
        for d in m['defs']: defs[d[1]].append((f, c, d))
    viol = []
    for f, m in mods.items():
        cu = O.crate_of_file(f)
        for name in m['calls']:
            ds = defs.get(name)
            if not ds or len(ds) != 1: continue
            df, cd, d = ds[0]
            if df == f or cd == cu or cd is None or cu is None: continue
            if d[0] == 'method' and False: pass
            if RANK[cd] >= RANK[cu]: viol.append((f, cu, name, df, cd))
    return viol

if __name__ == '__main__':
    v = scan()
    tier1 = {'instance-stores', 'instance-history', 'instance-children', 'instance-jobs', 'instance-docs'}
    t1 = [x for x in v if x[1] in tier1]
    print('crate DAG scan: non-downward references', len(v), '| from tier-1 crates (defects):', len(t1))
    for x in t1: print('  TIER1', x[1], '->', x[4], x[2], x[0].split('/')[-3:-1])
    c = collections.Counter((x[1], x[4]) for x in v if x[1] not in tier1)
    for k, n in sorted(c.items(), key=lambda kv: -kv[1]): print('  ', n, k)
