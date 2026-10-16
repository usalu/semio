#!/usr/bin/env python3
"""🧪️ Bin test file analysis: tests vs helpers, helper reach per test, contiguous-partition cost."""
import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import itemdeps

PATH = '/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs'

def is_test(items):
    return any(any(a.replace(' ', '').startswith(('#[test', '#[tokio::test')) for a in it.attrs) for it in items)

def load(path=PATH):
    byname, deps, sizes = itemdeps.analyse(path)
    order = list(byname)
    tests = [k for k in order if is_test(byname[k])]
    return byname, deps, sizes, order, tests

def reach(deps, start, tests):
    seen = set(); st = list(deps.get(start, ()))
    tset = set(tests)
    while st:
        v = st.pop()
        if v in seen or v in tset:
            continue
        seen.add(v); st.extend(deps.get(v, ()))
    return seen

if __name__ == '__main__':
    byname, deps, sizes, order, tests = load()
    tl = sum(sizes[t] for t in tests)
    helpers = [k for k in order if k not in set(tests)]
    hl = sum(sizes[h] for h in helpers)
    print('tests', len(tests), tl, 'lines; helpers', len(helpers), hl, 'lines')
    users = collections.defaultdict(set)
    for t in tests:
        for h in reach(deps, t, tests):
            users[h].add(t)
    multi = [h for h in helpers if len(users[h]) > 1]
    print('helpers used by >1 test:', len(multi), sum(sizes[h] for h in multi), 'lines;  by >5:', sum(1 for h in helpers if len(users[h]) > 5), sum(sizes[h] for h in helpers if len(users[h]) > 5))
    print('unused helpers:', [h for h in helpers if not users[h]][:20])
    for K in (3, 4, 5, 6):
        # contiguous partition by test file order into K groups of equal test lines
        pos = {t: i for i, t in enumerate(order)}
        ts = sorted(tests, key=lambda t: pos[t])
        target = tl / K
        groups = [[]]; acc = 0
        for t in ts:
            if acc >= target and len(groups) < K:
                groups.append([]); acc = 0
            groups[-1].append(t); acc += sizes[t]
        gh = []
        for g in groups:
            hs = set()
            for t in g: hs |= users_for(t) if False else reach(deps, t, tests)
            gh.append(hs)
        cnt = collections.Counter(h for hs in gh for h in hs)
        shared = {h for h, n in cnt.items() if n > 1}
        sl = sum(sizes[h] for h in shared)
        print(f'K={K}: shared helper lines {sl}; groups', [(len(g), sum(sizes[t] for t in g), sum(sizes[h] for h in gh[i] - shared)) for i, g in enumerate(groups)])


CHILD_FILES = {
    'hostile_generative': '/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🎲️hostile-generative/🦀️.rs',
    'count_lease_tests': '/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🪶️count-lease/🦀️.rs',
}
PSEUDO = ('quick', 'long', 'hostile_generative', 'count_lease_tests')


def full(path=PATH):
    """Like load(), with the child-file mounts' own dependencies on the parent's helpers and the pseudo tests."""
    byname, deps, sizes, order, tests = load(path)
    import carve
    for name, f in CHILD_FILES.items():
        its, toks = carve.parse_items(open(f).read())
        ids = set()
        for it in its:
            for t in it.toks_all:
                if t.k == 'id':
                    ids.add(t.x)
        deps[name] = sorted(x for x in ids if x in byname and x != name)
        sizes[name] = sizes.get(name, 0) + sum(1 for l in open(f) if l.strip())
    return byname, deps, sizes, order, tests


def plan(cuts, path=PATH):
    """cuts: list of lists of test names (file order partition); the pseudo tests go to the group named by PSEUDO_GROUP."""
    byname, deps, sizes, order, tests = full(path)
    pseudo = [p for p in PSEUDO if p in byname]
    tset = set(tests) | set(pseudo)
    groups = [list(g) for g in cuts]
    assigned = {t for g in groups for t in g}
    missing = [t for t in tests if t not in assigned]
    return byname, deps, sizes, order, tests, groups, missing
