# adds to the six tier-1 manifests the downward first-party edges the staged code needs (param edges are certain, name evidence is probable)
import sys; sys.path.insert(0, '.')
from lib6 import *
import edges6 as E, orphan6 as O, defscan6 as D
import t2plan
from comps6 import COMPONENTS
CR = E.CR
TIER1 = ['instance-stores', 'instance-history', 'instance-children', 'instance-jobs', 'instance-docs']

def needed():
    model, methods, table, known = t2plan.build_world()
    certain = {c: set() for c in TIER1}
    for (rel, pi, mi), (info, r) in table.items():
        if not info.cf: continue
        own = COMPONENTS[info.cf][1]
        for p in info.params:
            if p in ('app', 'mounted_policy'): continue
            q = COMPONENTS[p][1]
            if q != own: certain[own].add(q)
    # component field types and constructor inputs reference items of lower crates: name evidence below
    idx = E.index()
    prob = {c: collections.Counter() for c in TIER1}
    texts = collections.defaultdict(str)
    for f in D.family_files():
        c = O.crate_of_file(f)
        if c in TIER1: texts[c] += mask(open(PLUG + f, encoding='utf-8').read()) + '\n'
    for c in TIER1:
        words = set(w_ for w_ in re.findall(r'(?<![\w.])([A-Za-z_]\w{5,})\b', texts[c]) if re.match(r'[A-Z]', w_))
        for w_ in words:
            cs = idx.get(w_)
            if cs and len(cs) == 1:
                d = next(iter(cs))
                if d != c and d in CR and CR[d]['rank'] < CR[c]['rank'] and not d.startswith(('instance-', 'tests-', 'test-support', 'builder', 'testing-')): prob[c][d] += 1
    return certain, prob

def manifest_path(c): return PLUG + CR[c]['pkg'].replace('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/', '') + '/Cargo.toml'

def rel_dep(c, d):
    return os.path.relpath(PLUG + CR[d]['pkg'].replace('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/', ''), os.path.dirname(manifest_path(c)))

def main(write=False):
    certain, prob = needed()
    report = {}
    for c in TIER1:
        p = manifest_path(c); t = open(p, encoding='utf-8').read()
        have = set(re.findall(r'^semio-framework-plugin-([\w-]+) = ', t, re.M))
        want = set(certain[c])
        add = sorted((d for d in want if d not in have), key=lambda d: CR[d]['rank'])
        report[c] = dict(add=add, certain=sorted(certain[c] - have))
        if write and add:
            lines = ['semio-framework-plugin-' + d + ' = { path = "' + rel_dep(c, d) + '" }' for d in add]
            last = [m for m in re.finditer(r'(?m)^semio-framework-plugin-[\w-]+ = [^\n]*\n', t)][-1]
            t = t[:last.end()] + '\n'.join(lines) + '\n' + t[last.end():]
            open(p, 'w', encoding='utf-8').write(t)
    for c, r in report.items(): print(c.ljust(18), 'add', r['add'], '| certain', r['certain'])
    return report

if __name__ == '__main__':
    main(write='--write' in sys.argv)
