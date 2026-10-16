# seeded sensitivity checks of the scans: each mutates ONE thing in a sandbox copy (never in the repo) and the scan must report it
import sys, shutil; sys.path.insert(0, '.')
from lib6 import *
import defscan6 as D, arity6 as AR, orphan6 as O
SB = SCR + '/pwp6-seed/'
SBP = SB + PLUGP
def reset_file(rel):
    shutil.copy2(PLUG + rel, SBP + rel)
def edit(rel, old, new, count=1):
    p = SBP + rel; t = open(p, encoding='utf-8').read()
    assert t.count(old) >= 1, (rel, old)
    open(p, 'w', encoding='utf-8').write(t.replace(old, new, count))

print('== definition scan')
v0 = D.scan(SB, quiet=True)
print('sandbox clean: violations', len(v0))
# a pub(crate) method used by another module becomes private
cand = None
mods = {f: D.parse_module(SB, f) for f in D.family_files(SB)}
for f, m in mods.items():
    for d in m['defs']:
        if d[0] == 'method' and d[2] == 'pub(crate)':
            name = d[1]
            if any(name in D.token_sets(m2['M'])[1] for f2, m2 in mods.items() if f2 != f) and 'staging' in f and 'components' not in f:
                cand = (f, name); break
    if cand: break
f, name = cand
raw = open(SBP + f, encoding='utf-8').read()
mm = re.search(r'(?m)^(\s*)pub\(crate\) ((?:async |const |unsafe )*fn ' + re.escape(name) + r'\b)', raw)
edit(f, mm.group(0), mm.group(1) + mm.group(2))
v1 = D.scan(SB, quiet=True)
print('seed: made', name, 'private in', f.split('/')[-3:-1], '->', len(v1), 'violations', [(x[1]) for x in v1][:3])
assert any(x[1] == name for x in v1), 'definition scan missed the seed'
reset_file(f)

print('== arity scan')
fam = D.family_files(SB)
files = sorted({f_.replace(PLUGP, '') for f_ in plug_files() if os.path.exists(SBP + f_.replace(PLUGP, ''))} | set(fam))
b0, c0 = AR.scan(SB, fam=fam, files=files)
# remove the first argument after the lower-component parameter of a rewritten call
target = None
for f in fam:
    raw = open(SBP + f, encoding='utf-8').read()
    m = re.search(r'self\.stores\.drive_store_constructor_unit\(&mut self\.constructor, ', raw)
    if m: target = (f, m.group(0)); break
f, txt = target
edit(f, txt, 'self.stores.drive_store_constructor_unit(')
b1, c1 = AR.scan(SB, fam=fam, files=files)
kb = collections.Counter(AR.key(x) for x in b0); new = [x for x in b1 if collections.Counter(AR.key(y) for y in b1)[AR.key(x)] > kb.get(AR.key(x), 0)]
print('seed: dropped one lower-component argument of drive_store_constructor_unit ->', len(b0), 'baseline violations,', len(b1), 'after the seed;', [(x[2], x[3], x[4]) for x in b1 if AR.key(x) not in kb][:3])
assert len(b1) > len(b0), 'arity scan missed the seed'
reset_file(f)

print('== orphan scan')
rows0, bad0 = O.scan(verbose=False, root=SBP)
g = [f for f in fam if f.endswith('📜️history/📥️staging/📨️dispatch/🦀️.rs')][0]
raw = open(SBP + g, encoding='utf-8').read()
m = re.search(r'impl<A: ArtifactApp> CommandHistory<A> \{', raw)
edit(g, m.group(0), 'impl<A: ArtifactApp> InstanceStores<A> {')
rows1, bad1 = O.scan(verbose=False, root=SBP)
print('seed: an inherent impl of InstanceStores placed in instance-history ->', len(bad0), 'misplaced before,', len(bad1), 'after')
assert len(bad1) > len(bad0), 'orphan scan missed the seed'
reset_file(g)
print('ALL SEEDS CAUGHT')
