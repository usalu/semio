"""🌿 Preserve all 22 midrib-to-margin stations of each fine hosta half-blade and the paired petioles."""
import runpy
from pathlib import Path
ticket=Path(__file__).parent
b=runpy.run_path(str(ticket/'living-botanical-shells.py'));edit=runpy.run_path(str(ticket/'living-shape-rows.py'))['edit']
def author(k,m,s):
    id=s['id'];idx=('50','100','200','500').index(k);kind=s.get('kind');material=s.get('material')
    if m=='body' and kind=='loft' and s.get('capStart') and s.get('capEnd') and material=='hosta-blade' and s.get('role')=='body':
        b['loft'](s,(16,12,8,8)[idx]);return True
    if m=='thermal' and kind=='ellipsoid' and material=='hosta-blade' and id.startswith('th-tp-'):
        b['oval'](s,s['radii'],(16,12,12,8)[idx],((-70,-46,-22,0,22,46,70),(-60,-28,0,30,62),(-50,-19,0,22,54),(-48,0,48))[idx]);return True
    if m=='body' and kind=='loft' and material=='hosta-petiole' and id.startswith('petiole-') and s.get('capStart') and s.get('capEnd'):
        b['loft'](s,(12,10,8,8)[idx]);return True
    return False
for p in (next(b['root'].glob('*/hosta-clump/*.json')),ticket/'volumes/vegetation/hosta-clump.json'):edit(p,author)

