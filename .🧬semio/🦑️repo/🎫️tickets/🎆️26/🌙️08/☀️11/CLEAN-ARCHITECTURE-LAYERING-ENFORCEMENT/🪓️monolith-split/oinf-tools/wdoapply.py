import sys; sys.path.insert(0,'.')
import wrun4
from wapply import *
crates, T, files, P, libtexts, testtexts = wrun4.main4()
OUT = wrun4.wrun3.OUT
ok, a, b = check_unchanged(SCR + '/snapshot-infinite')
print('live world unchanged since snapshot:', ok)
if not ok: raise SystemExit('live tree changed; regenerate')
name2crate = {}
for tg in T['targets']:
    for k in tg['tests']: name2crate[T['pool'][k].it.name] = CRATES[tg['crate']][0]
copy_world(OUT, crates)
print('world copied')
reg_members(crates); reg_owners(crates); reg_launch(crates)
patch_facade_manifest(crates)
print('consumers', apply_consumers())
print('readers', apply_readers())
print('routes', rewrite_routes(name2crate))
