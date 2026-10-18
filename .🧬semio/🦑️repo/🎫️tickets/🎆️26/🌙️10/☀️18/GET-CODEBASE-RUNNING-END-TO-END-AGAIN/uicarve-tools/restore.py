import json, os, shutil, sys
sys.path.insert(0, 'lib')
import carve
l = carve._ledger()
only = sys.argv[1:] 
for rel, name in l['backed'].items():
    if only and not any(rel.startswith(o) for o in only): continue
    shutil.copy2(carve.ORIG + '/' + name, carve.R + '/' + rel)
for rel in l['created']:
    if only and not any(rel.startswith(o) for o in only): continue
    p = carve.R + '/' + rel
    if os.path.exists(p): os.remove(p)
print('restored', len(l['backed']), 'removed', len(l['created']))
