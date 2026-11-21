"""🌾 Retain the arch and hanging section of each willow tress and the longitudinal fold of each grass blade."""
import runpy
from pathlib import Path
t=Path(__file__).parent
b=runpy.run_path(str(t/'living-botanical-shells.py'));edit=runpy.run_path(str(t/'living-shape-rows.py'))['edit']
def author(n,k,m,s):
    idx=('50','100','200','500').index(k);material=s.get('material');kind=s.get('kind');id=s['id']
    if n=='weeping-willow':
        if material!='willow-leaf' or m=='structure':return False
        if kind=='ellipsoid':
            lat=((-66,-41,-19,0,22,44,68),(-56,-27,0,29,59),(-49,-20,0,23,52),(-47,0,47))[idx]
            b['oval'](s,s['radii'],(16,12,12,8)[idx],lat)
            return True
        if kind!='loft' or not s.get('capStart') or not s.get('capEnd'):return False
        sides=((12,10,8,8) if 'arch' in id or 'cap' in id else (8,8,6,6))[idx]
        b['loft'](s,sides);return True
    if n=='miscanthus-grass':
        if material not in ('miscanthus-blade','miscanthus-plume') or kind!='loft' or not s.get('capStart') or not s.get('capEnd'):return False
        if material=='miscanthus-plume':
            sides=((8,8,6,6) if 'raceme' in id else (12,10,8,8))[idx]
        else:sides=(8,6,6,6)[idx] if m=='structure' else (8,6,6,4)[idx]
        b['loft'](s,sides);return True
    return False
for n in ('weeping-willow','miscanthus-grass'):
    for p in (next(b['root'].glob('*/'+n+'/*.json')),t/'volumes/vegetation'/f'{n}.json'):
        edit(p,lambda k,m,s:author(n,k,m,s))

