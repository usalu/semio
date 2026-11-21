"""🌳 Keep individual branch and crown forms inside the per-scene adaptive mesh budget."""
import itertools
import json
import math
import sys
from pathlib import Path

catalog=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
ticket=Path(__file__).parent
species=('apple-tree','english-oak','scots-pine','small-leaved-lime','multi-stem-birch','lombardy-poplar','date-palm','miscanthus-grass')
angles={
    'apple-tree':(-89.5,-80,-66,-51,-34,-17,0,15,30,46,62,77,89.5),
    'english-oak':(-89.5,-78,-64,-50,-34,-18,0,18,34,50,66,80,89.5),
    'scots-pine':(-89.5,-78,-60,-42,-24,-10,0,16,34,53,72,89.5),
    'small-leaved-lime':(-89.5,-73,-56,-38,-19,0,19,37,55,73,89.5),
    'multi-stem-birch':(-89.5,-72,-48,-24,0,24,48,72,89.5),
    'lombardy-poplar':(-89.5,-62,-36,0,32,62,89.5),
}

def crown(s,spacing):
    x,y,z=s.pop('radii')
    s['kind']='loft'
    s['sections']=[{'at':[0,0,round(z*math.sin(math.radians(v)),7)],'rx':round(x*math.cos(math.radians(v)),7),'ry':round(y*math.cos(math.radians(v)),7)} for v in spacing]
    s.update(capStart=True,capEnd=True)

def reduce_sections(s,num):
    sec=s['sections']
    positions=[0]
    for a,b in zip(sec,sec[1:]):
        positions.append(positions[-1]+math.dist(a['at'],b['at']))
    def error(chosen):
        worst=0
        for first,last in zip(chosen,chosen[1:]):
            p,q=sec[first],sec[last]
            for k in range(first+1,last):
                t=(positions[k]-positions[first])/max(1e-8,positions[last]-positions[first])
                mid=[(1-t)*x+t*y for x,y in zip(p['at'],q['at'])]
                dr=max(abs(sec[k][f]-(p[f]*(1-t)+q[f]*t)) for f in ('rx','ry'))
                worst=max(worst,math.dist(mid,sec[k]['at'])+dr)
        return worst
    widest=max(range(len(sec)),key=lambda i:sec[i]['rx']*sec[i]['ry'])
    end=len(sec)-1
    choices=((0,*mid,end) for mid in itertools.combinations(range(1,end),num-2) if widest in (0,*mid,end))
    indices=min(choices,key=error)
    s['sections']=[sec[i] for i in indices]

def petiole_scar(s):
    axial,radial,side=s.pop('size')
    s.pop('cornerRadius')
    s.update(kind='extrude',plane='zx',profile=[
        [0,-axial*.5],[side*.32,-axial*.39],[side*.5,-axial*.12],
        [side*.42,axial*.16],[side*.15,axial*.45],[0,axial*.5],
        [-side*.23,axial*.38],[-side*.47,axial*.09],[-side*.5,-axial*.12],
        [-side*.29,-axial*.39]],taperTo=[0.86,1])
    s['from']=-radial/2
    s['to']=radial/2

def date(s):
    x,y,z=s.pop('radii')
    stations=((-1,.07),(-.79,.57),(-.39,.91),(.08,1),(.53,.86),(.85,.52),(1,.09))
    s.update(kind='loft',sections=[{'at':[round(x*h,7),0,0],'rx':round(y*r,7),'ry':round(z*r,7),'rotation':[.5,.5,.5,.5]} for h,r in stations],capStart=True,capEnd=True)

def lenticel(s):
    x,y,z=s.pop('radii')
    outline=[(-1,0),(-.83,-.47),(-.43,-.82),(0,-1),(.45,-.81),(.82,-.46),(1,0),(.79,.50),(.4,.87),(0,1),(-.42,.82),(-.79,.5)]
    thin=min(range(3),key=[x,y,z].__getitem__)
    plane,u,v,h=(('yz',y,z,x),('zx',z,x,y),('xy',x,y,z))[thin]
    s.update(kind='extrude',plane=plane,profile=[[round(a*u,7),round(b*v,7)] for a,b in outline])
    s['from']=-h
    s['to']=h

def edit(s,name,d,mode):
    kind=s.get('kind')
    mat=s.get('material','')
    if name=='date-palm' and d=='50':
        if kind=='box' and mat=='palm-stem-bark' and 'leaf-bases' in s['id']:petiole_scar(s)
        if kind=='ellipsoid' and mat=='date-fruit': date(s)
        if s.get('kind')=='loft' and mat=='date-fruit' and len(s['sections'])==7 and 'dates-' in s['id']:
            reduce_sections(s,6)
    if name=='date-palm' and d=='100' and kind=='box' and mat=='palm-stem-bark' and 'leaf-bases' in s['id']:
        petiole_scar(s)
        s['profile']=[s['profile'][i] for i in (0,2,3,5,7,8)]
    if name=='multi-stem-birch' and d=='100':
        if kind=='loft' and mat=='birch-leaf' and len(s['sections'])==7:
            reduce_sections(s,5)
        if kind=='ellipsoid' and mat=='birch-bark-base' and s['role']=='texture':
            lenticel(s)
            s['profile']=[s['profile'][i] for i in (0,1,3,5,6,7,9,11)]
    if name=='lombardy-poplar' and d=='100' and kind=='loft' and mat=='poplar-leaf' and len(s['sections'])==8 and '-f' in s['id']:
        reduce_sections(s,4)
    if name in angles and d=='50' and kind=='ellipsoid' and ('leaf' in mat or 'needles' in mat):
        crown(s,angles[name])
    if name=='small-leaved-lime' and d=='100' and kind=='ellipsoid' and 'leaf' in mat:
        crown(s,(-89.5,-72,-53,-31,0,28,49,70,89.5))
    if name=='miscanthus-grass' and d=='500' and mode=='body' and kind=='loft' and mat=='miscanthus-blade' and len(s['sections'])==8:
        reduce_sections(s,6)
    if name=='lombardy-poplar' and d=='50' and kind=='loft' and mat=='poplar-leaf' and len(s['sections'])==8 and '-f' in s['id']:
        reduce_sections(s,4)
    if name=='multi-stem-birch' and d=='50':
        if kind=='loft' and mat=='birch-leaf' and len(s['sections'])==8: reduce_sections(s,5)
        if kind=='ellipsoid' and mat=='birch-bark-base' and s['role']=='texture': lenticel(s)
    if name=='miscanthus-grass' and d=='500' and mode=='thermal' and kind=='loft' and 'transpiration' in s['id'] and len(s['sections'])==6:
        reduce_sections(s,4)

for name in sys.argv[1:] or species:
    for p in (next(catalog.glob(f'*/{name}/*.json')),ticket/'volumes/vegetation'/f'{name}.json'):
        raw=p.read_text()
        doc=json.loads(raw)
        asset=doc.get('asset',doc)
        for scale,model in asset['volumes'].items():
            for mode in ('body','thermal'):
                for s in model[mode]:
                    edit(s,name,scale,mode)
        start=raw.index('{',raw.index('"volumes":'))
        orig,n=json.JSONDecoder().raw_decode(raw[start:])
        if orig==asset['volumes']:continue
        if p.read_text()!=raw:raise SystemExit(f'{name} changed concurrently')
        if raw.startswith('{\n'):
            depth='    ' if 'asset' in doc else '  '
            patch=json.dumps(asset['volumes'],ensure_ascii=False,indent=2).replace('\n','\n'+depth)
        else:patch=json.dumps(asset['volumes'],ensure_ascii=False,separators=(',',':'))
        p.write_text(raw[:start]+patch+raw[start+n:])
        print('[DEBUG]',name,p.name)

