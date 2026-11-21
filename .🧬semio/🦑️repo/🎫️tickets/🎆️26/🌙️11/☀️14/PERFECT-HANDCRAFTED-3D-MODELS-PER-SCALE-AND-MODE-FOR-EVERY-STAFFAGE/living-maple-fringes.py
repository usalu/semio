"""🍁 Retain a spaced sequence of hanging branch tips on the lower edges of the three maple tiers."""
from pathlib import Path
import json
import math
import runpy

t=Path(__file__).parent
b=runpy.run_path(str(t/'living-botanical-shells.py'))
out=t/'🗑️generated'
frames=json.loads((out/'living-maple-frames.json').read_text())
lobes={l['id']:l for l in json.loads((out/'living-maple-lobes.json').read_text())['50']}
tips=('t1-f1','t1-f3','t1-f5','t1-f7','t1-f9','t1-f11','t2-f2','t2-f4','t2-f6','t2-f8','t2-f10','t3-f1','t3-f3','t3-f5','t3-f7')
sections=((-1,.8),(-.35,1),(.15,.97),(.55,.8),(1,.1))
def spray(s,l):
    if not l['pad']:raise ValueError(l['id'])
    c=l['world'];a=l['axes'];r=l['radii'];f=frames[s['bone']]
    def point(h,radius,j):
        theta=j*math.tau/14
        p=[c[k]-f['o'][k]+a[0][k]*r[0]*radius*math.cos(theta)+a[1][k]*r[1]*h+a[2][k]*r[2]*radius*math.sin(theta) for k in range(3)]
        return [b['dot'](p,f[axis]) for axis in ('u','v','w')]
    b['mesh'](s,[[point(h,radius,j) for j in range(14)] for h,radius in sections])
p=next(b['root'].glob('*/japanese-maple/*.json'))
for file in [p,t/'volumes/vegetation/japanese-maple.json']:
    raw=file.read_text();v=raw.index('{',raw.index('"volumes":'));start=raw.index('{',raw.index('"100"',v));scale,n=json.JSONDecoder().raw_decode(raw[start:])
    fine=json.JSONDecoder().raw_decode(raw[raw.index('{',raw.index('"50"',v)):])[0]
    byid={s['id']:s for s in fine['body']}
    built=[]
    for id in tips:
        src=byid[id]
        s={k:src[k] for k in ('id','kind','bone','role','material','tone')}
        spray(s,lobes[id]);built.append(s)
    scale['body']=[s for s in scale['body'] if s['id'] not in tips]+built
    scale['thermal']=[s for s in scale['thermal'] if s.get('of') not in tips]+[dict(id='th-'+id,of=id,region='leaf-sun') for id in tips]
    scale['decisions'][0]=dict(en='The flared root collar carries the curved stems and the secondary limbs retain two stations. Fifty-three broad foliage pads describe the crown. Six hanging spray tips mark its lower tier, five its middle tier and four its upper tier. Their shallow undersides remain visible in elevation.',de='Der ausgestellte Wurzelhals trägt die gekrümmten Stämme. Die Nebenäste behalten zwei Stützpunkte. Dreiundfünfzig breite Laubpolster bilden die Krone. Sechs hängende Triebspitzen säumen die untere, fünf die mittlere und vier die obere Etage. Ihre flachen Unterseiten bleiben in der Ansicht sichtbar.')
    text=raw[:start]+json.dumps(scale,ensure_ascii=False,separators=(',',':'))+raw[start+n:]
    if file.read_text()!=raw:raise RuntimeError(file)
    file.write_text(text)

