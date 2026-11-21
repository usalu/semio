"""🌳 Divide the drooping willow's curtains into separate, pointed sprays on their own boughs."""
from pathlib import Path
from runpy import run_path
from copy import deepcopy
import math,json
t=Path(__file__).resolve().parent
cat=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/weeping-willow/🔣️.json'))
doc=json.loads(cat.read_text());a=doc['asset']
mesh=run_path(str(t/'living-botanical-shells.py'))['mesh']
def fit(row,u):return [x+(y-x)*u for x,y in zip(*row)]
def point(c,b,d,x,y):
    return [round(c[i]+b[i]*x+d[i]*y,7) for i in range(3)]
def avg(v):return [sum(p[i]for p in v)/len(v) for i in range(3)]
tips=[(.71,.94,.81),(.90,.75,1),(.86,1,.68),(1,.77,.90),(.79,.92,1),(.89,1,.74),(1,.88,.78),(.72,.90,1),(.93,.81,1),(.82,1,.9),(.94,.72,.85),(.85,1,.76)]
fine={q['id']:deepcopy(q)for q in a['volumes']['50']['body'] if q['id'].startswith('tress-')}
for scale,vol in a['volumes'].items():
    new=[];thermal=[]
    for leaf in vol['body']:
        name=leaf['id']
        if not name.startswith('tress-') or name.endswith('-arch'):continue
        original=fine[name];vs=original['vertices'];sides=original['triangles'][0][2];assert (len(vs)-2)%sides==0 and sides>=4
        rings=[vs[j:j+sides]for j in range(0,len(vs)-2,sides)]
        def sample(f):
            pos=min(len(rings)-1,max(0,f*(len(rings)-1)));j=min(int(pos),len(rings)-2);f=pos-j
            ring=[fit((u,v),f) for u,v in zip(rings[j],rings[j+1])]
            c=avg(ring);b=[(x-y)/2 for x,y in zip(ring[0],ring[sides//2])]
            d=[(x-y)/2 for x,y in zip(ring[round(sides/4)],ring[round(sides*3/4)%sides])]
            return c,b,d
        digit=int(name[7:]);family='abc'.index(name[6]);ends=tips[(digit-1+family*3)%len(tips)]
        stations={'50':(0,.11,.23,.37,.51,.64,.76,.85,.92,.97,1),'100':(0,.15,.32,.50,.68,.82,.93,1),'200':(0,.22,.45,.69,.86,1),'500':(0,.30,.58,.82,1)}[scale]
        count={'50':7,'100':7,'200':7,'500':5}[scale]
        for lane,(side,front) in enumerate([(-.55,-.15),(.06,.39),(.58,-.18)]):
            rows=[]
            for u in stations:
                c,b,d=sample(u*ends[lane]);opening=.25+.75*u**.55
                lean=side*opening+.10*math.sin(math.pi*(1.5*u+.2*family+lane*.5))
                depth=front*opening+.11*math.sin(math.pi*(u+digit*.17+lane*.3))
                width=(.58 if lane==1 else .49)*(1-.35*u**1.2)
                thickness=(.62 if lane==1 else .57)*(1-.28*u)
                taper=1 if u<=.82 else max(.035,((1-u)/.18)**.7)
                row=[point(c,b,d,lean+width*taper*math.cos(2*math.pi*h/count),depth+thickness*taper*math.sin(2*math.pi*h/count))for h in range(count)]
                rows.append(row)
            s=deepcopy(leaf);s['id']=name+'-spray-'+str(lane+1);mesh(s,rows)
            new.append(s);thermal.append(dict(id='th-'+s['id'],of=s['id'],region='leaf-curtain'))
    gone={s['id'] for s in vol['body']if s['id'].startswith('tress-')and not s['id'].endswith('-arch')}
    vol['body'][:]=[s for s in vol['body']if s['id']not in gone]+new
    vol['thermal'][:]=[s for s in vol['thermal']if s.get('of')not in gone]+thermal
(t/'🗑️generated/digital-willow-sprigs.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
