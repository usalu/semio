"""🍁 Construct each maple tier's authored pads and recessed shade sprays on their own wind carriers."""
import json
import math
import runpy
import sys
from pathlib import Path
t=Path(__file__).parent;b=runpy.run_path(str(t/'living-botanical-shells.py'));edit=runpy.run_path(str(t/'living-shape-rows.py'))['edit']
out=t/'🗑️generated';manifest=json.loads((out/'living-maple-lobes.json').read_text());frames=json.loads((out/'living-maple-frames.json').read_text())
lobes={k:{v['id']:v for v in row} for k,row in manifest.items()};path=next(b['root'].glob('*/japanese-maple/*.json'))
sections=[(-1,.8),(-.75,.93),(-.35,1),(.15,.97),(.55,.8),(.85,.48),(1,.1)]
def project(s,p):
    f=frames[s['bone']];delta=b['sub'](p,f['o']);return [b['dot'](delta,f[axis]) for axis in ('u','v','w')]
def surfaced(k,m,s):
    if m!='body' or s['id'] not in lobes[k]:return False
    l=lobes[k][s['id']];c=l['world'];a=l['axes'];r=l['radii'];idx=['50','100','200','500'].index(k)
    width=(24,20,16,12)[idx] if s['material'].endswith('sun') else (20,16,12,10)[idx]
    def point(x,y,z):
        p=[c[j]+a[0][j]*r[0]*x+a[1][j]*r[1]*y+a[2][j]*r[2]*z for j in range(3)]
        return project(s,p)
    def ring(h,radius):
        return [point(radius*math.cos(2*math.pi*j/width),h,radius*math.sin(2*math.pi*j/width)) for j in range(width)]
    if l['pad']:rings=[ring(h,radius) for h,radius in sections]
    else:
        lat=((-72,-50,-28,0,28,50,72),(-68,-45,-22,0,22,45,68),(-60,-32,0,32,60),(-52,-22,0,22,52))[idx]
        rings=[[point(0,-1,0)]]+[ring(math.sin(math.radians(h)),math.cos(math.radians(h))) for h in lat]+[[point(0,1,0)]]
    s.pop('transform',None);b['mesh'](s,rings);return True
for p in (path,t/'volumes/vegetation/japanese-maple.json'):edit(p,surfaced)
raw=path.read_text();a=json.loads(raw)['asset'];candidate=json.loads((out/'living-maple-paper.json').read_text())
if a['rig']!=candidate['rig']:raise ValueError('The source rig changed.')
values={k:candidate[k] for k in ('anchors','representations','symbols')};changes=[]
for name,value in values.items():
    marker=raw.index('"'+name+'":')+len(name)+3
    while raw[marker].isspace():marker+=1
    _,n=json.JSONDecoder().raw_decode(raw[marker:]);changes.append((marker,marker+n,json.dumps(value,ensure_ascii=False,separators=(',',':'))))
new=raw
for i,j,v in sorted(changes,reverse=True):new=new[:i]+v+new[j:]
if path.read_text()!=raw:raise RuntimeError(path)
path.write_text(new)
