"""🌳 Give each species conserved transverse sections along its own authored crown and skeleton."""
import runpy
import sys
from pathlib import Path

ticket=Path(__file__).parent
argv=sys.argv;sys.argv=[argv[0]]
b=runpy.run_path(str(ticket/'living-botanical-shells.py'))
sys.argv=argv
edit=runpy.run_path(str(ticket/'living-shape-rows.py'))['edit']
def style(name,m,s,k):
    part=s.get('part');mat=s.get('material','');ident=s['id']
    if part in ('joint','mass'):return (8,8,8,6)[k]
    if name=='agave':
        if mat=='agave-leaf':return ((12,10,8,6) if '-wing-' in ident else (16,12,10,8))[k]
        if mat=='agave-caudex':return (16,12,10,8)[k]
    if name=='english-oak':
        if 'oak-leaf' in mat:return ((16,12,12,8) if 'sun' in mat else (12,12,8,8))[k]
        if 'oak-wood'==mat or 'oak-bark'==mat:return ((20,16,12,12) if 'trunk' in ident else (12,10,8,8))[k]
    if name=='lombardy-poplar':
        if mat=='poplar-leaf':return (12,10,8,8)[k]
        if mat in ('poplar-wood','poplar-bark'):return ((20,16,12,12) if 'trunk' in ident else (10,8,8,6))[k]
    if name=='multi-stem-birch':
        if mat=='birch-leaf':return ((8,6,6,6) if '-v' in ident or 'tress' in ident else (12,8,8,6))[k]
        if mat in ('birch-wood','birch-bark','birch-bark-base'):return ((16,12,12,8) if any(x in ident for x in ('stem-','base-','stool')) else (8,8,6,6))[k]
    if name=='scots-pine':
        if 'pine-needles' in mat:return ((16,12,10,8) if 'sun' in mat else (12,10,8,8))[k]
        if mat.startswith(('pine-bark','pine-wood')):return ((20,16,12,10) if 'trunk' in ident else (12,10,8,8))[k]
    if name=='small-leaved-lime':
        if 'lime-leaf' in mat:return ((8,8,6,6) if 'epicormic' in mat else (16,12,10,8))[k]
        if mat in ('lime-bark','lime-wood','lime-twig','lime-bark-young'):return ((20,16,12,10) if 'stem' in ident or 'trunk' in ident else (10,8,8,6))[k]
    if name=='japanese-maple':
        if 'maple-leaf' in mat:return ((16,12,10,8) if 'sun' in mat else (12,10,8,8))[k]
        if 'maple-' in mat:return ((16,12,10,8) if 'stem' in ident else (10,8,8,6))[k]
    if name=='london-plane-pollarded':
        if 'plane-leaf' in mat:return ((12,12,8,8) if 'shade' in mat else (16,12,10,8))[k]
        if mat.startswith(('plane-bark','plane-wood','plane-twig')):return ((20,16,12,10) if 'head' in ident or 'knuckle' in ident or 'trunk' in ident else (10,8,8,6))[k]

def process(name,k,m,s):
    idx=('50','100','200','500').index(k)
    n=style(name,m,s,idx)
    if not n:return False
    if s.get('kind')=='loft' and s.get('capStart') and s.get('capEnd'):
        sections=s['sections']
        small=max(q['rx']*q['ry'] for q in sections)*.0005
        pointed=sections[0]['rx']*sections[0]['ry']<=small and sections[-1]['rx']*sections[-1]['ry']<=small and len(sections)>=3
        b['loft'](s,n,ends=pointed)
    elif s.get('kind')=='ellipsoid':
        if s.get('part') in ('mass','joint'):lat=(-52,0,52)
        elif name in ('japanese-maple','english-oak','small-leaved-lime'):lat=((-70,-48,-24,0,24,48,70),(-62,-30,0,32,63),(-52,-24,0,26,55),(-49,0,49))[idx]
        elif name=='scots-pine':lat=((-68,-43,-20,0,22,47,69),(-61,-28,0,31,63),(-53,-19,0,28,56),(-48,0,48))[idx]
        else:lat=((-67,-42,-21,0,24,47,68),(-60,-28,0,30,63),(-50,-21,0,28,54),(-48,0,48))[idx]
        b['oval'](s,s['radii'],n,lat)
    else:return False
    return True

for name in argv[1:]:
    for path in (next(b['root'].glob('*/'+name+'/*.json')),ticket/'volumes/vegetation'/f'{name}.json'):
        edit(path,lambda k,m,s:process(name,k,m,s))

