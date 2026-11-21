"""🌿 Follow each curved hosta vein over its own pleated leaf blade."""
import runpy
from pathlib import Path
ticket=Path(__file__).parent
b=runpy.run_path(str(ticket/'living-botanical-shells.py'));edit=runpy.run_path(str(ticket/'living-shape-rows.py'))['edit']
def author(k,m,s):
    if k!='50' or s.get('kind')!='loft' or s.get('material')!='hosta-blade' or not s.get('capStart') or not s.get('capEnd'):return False
    if m=='body' and s.get('texture') and '-vein-' in s['id']:
        b['loft'](s,6)
    elif m=='structure' and '-rib-' in s['id']:
        b['loft'](s,8)
    else:return False
    return True
for p in (next(b['root'].glob('*/hosta-clump/*.json')),ticket/'volumes/vegetation/hosta-clump.json'):edit(p,author)

