# 🧲️ Orphan/placement scan: every impl of the WP4 files must live in the crate of its type (inherent) or of its type or trait (trait impl).
import sys, collections
from model4 import *
files, items, defs, owner, owner2, text, final1 = load()
LIFTS = ('🧩️extension/🛬️lifted/🏃️runtime/', '🏗️builder/🔌️program/🛬️lifted/🏃️runtime/')
fc = PT.file_crate
SEED = sys.argv[sys.argv.index('--seed') + 1] if '--seed' in sys.argv else None
if SEED:
    moved = [it for it in items if it['kind'] == 'impl' and it.get('target') == SEED and not it.get('trait')]
    for it in moved: it['file'] = '⚛️reactor/🏃️runtime/📦️instances/🏷️metadata-owner/🦀️.rs'
    print('seeded: moved', len(moved), 'inherent impl block(s) of', SEED, 'into another crate')
def crate_of_file(f):
    return fc(f)
bad = []; seen = 0
for it in items:
    if it['kind'] != 'impl' or not it.get('target'): continue
    f = it['file']
    if not (WP4F(f) or f.startswith(LIFTS)): continue
    if it['cfgtest'] or '/🧪️tests/' in f: continue
    seen += 1
    ic = crate_of_file(f)
    tdefs = [j for j in defs.get(it['target'], []) if items[j]['kind'] in ('struct', 'enum', 'union', 'type')]
    tc = {crate_of_file(items[j]['file']) for j in tdefs}
    trait = it.get('trait')
    trdefs = [j for j in defs.get(trait, [])] if trait else []
    trc = {crate_of_file(items[j]['file']) for j in trdefs if items[j]['kind'] == 'trait'}
    if not tdefs:
        continue                     # a foreign type: nothing to check at this level
    if not trait:
        if ic not in tc: bad.append(('inherent', f.split('/')[-2], it['s'], it['target'], ic, sorted(tc)))
    else:
        if ic not in tc and ic not in trc: bad.append(('trait', f.split('/')[-2], it['s'], f"{trait} for {it['target']}", ic, sorted(tc), sorted(trc)))
print('impls checked', seen, 'violations', len(bad))
for b in bad: print(' ', b)

if SEED: sys.exit(0 if bad else 1)
