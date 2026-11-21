"""🪶 Draw the terminal primary vane on each hand and keep six short site-scale bone shafts independently articulated."""
import json
import math
import runpy
from pathlib import Path
t=Path(__file__).parent;g=runpy.run_path(str(t/'living-botanical-shells.py'));edit=runpy.run_path(str(t/'living-shape-rows.py'))['edit']
path=next(g['root'].glob('*/mallard-duck/*.json'))
def rods(k,m,s):
    if k!='500' or m!='structure' or s.get('kind')!='capsule' or s['id'].split('-',1)[0] not in ('near','far') or s['id'].split('-',1)[1] not in ('humerus','forearm-bones','manus'):return False
    a,b=s['p0'],s['p1'];u=g['unit'](g['sub'](b,a));v=g['unit'](g['cross'](u,[0,0,1]));w=g['cross'](u,v)
    def ring(p,r):return [g['add'](p,g['add'](g['mul'](v,r*math.cos(math.tau*j/8)),g['mul'](w,r*math.sin(math.tau*j/8)))) for j in range(8)]
    r0,r1=s.pop('r0'),s.pop('r1');s.pop('p0');s.pop('p1')
    g['mesh'](s,[[g['sub'](a,g['mul'](u,r0))],ring(a,r0),ring(b,r1),[g['add'](b,g['mul'](u,r1))]])
    return True
for p in (path,t/'volumes/animals/mallard-duck.json'):edit(p,rods)
raw=path.read_text();a=json.loads(raw)['asset'];r=a['representations'];e=r['200']['geometry']['elevation']
for side,name,sign in (('n','near',-1),('f','far',1)):
    primary=next(s for s in a['volumes']['200']['body'] if s['id']==name+'-primaries')
    rows=primary['sections'];base=rows[1]['at'];shaft=rows[2]['at'];tip=rows[3]['at']
    border=[[base[0],base[1]-.001],[shaft[0]-.012,shaft[1]+.0021],[tip[0],tip[1]+.0018],[tip[0]+.0015,tip[1]],[shaft[0]+.001,shaft[1]-.0026],[base[0]+.008,base[1]-.0041]]
    depth=sign*.051+shaft[2]
    shape=dict(id=name+'-outer-primary',bone='hand-'+side,role='body',z=5.1 if side=='n' else -4.2,filled=True,tone=.46,region='wing-plumage',kind='poly',points=[[round(x,7),round(y,7)] for x,y in border],closed=True,solid=dict(kind='extruded',sweep='w',z0=round(depth-.0004,7),z1=round(depth+.0004,7)))
    if any(q['id']==shape['id'] for q in e):raise ValueError('Primary already drawn')
    e.append(shape)
start=raw.index('{',raw.index('"representations":'));_,n=json.JSONDecoder().raw_decode(raw[start:]);new=raw[:start]+json.dumps(r,ensure_ascii=False,separators=(',',':'))+raw[start+n:]
if path.read_text()!=raw:raise RuntimeError(path)
path.write_text(new)
