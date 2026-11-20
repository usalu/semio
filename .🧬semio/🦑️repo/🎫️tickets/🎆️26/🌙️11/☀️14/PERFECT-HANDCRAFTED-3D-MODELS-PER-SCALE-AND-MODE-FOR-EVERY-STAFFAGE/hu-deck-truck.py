"""🛹 Independently estimates exposed passive assemblies from their stock, castings and component interfaces."""
from pathlib import Path
import json, math, sys
ticket=Path(__file__).resolve().parent
def stock(p):
    if p['kind']=='box':
        x,y,z=p['size']
        return 2*(x*y+y*z+x*z)
    if p['kind']=='extrude':
        q=p['profile']
        edges=list(zip(q,q[1:]+q[:1]))
        return (p['to']-p['from'])*sum(math.dist(a,b) for a,b in edges)+abs(sum(a*d-b*c for (a,b),(c,d) in edges))
    if p['kind']=='capsule':
        r,R=p['r0'],p['r1']
        return math.pi*(r+R)*math.hypot(math.dist(p['p0'],p['p1']),r-R)+2*math.pi*(r*r+R*R)
    s=p['sections']
    return sum(math.pi*(q['rx']+q['ry']+w['rx']+w['ry'])/2*math.dist(q['at'],w['at']) for q,w in zip(s,s[1:]))+math.pi*sum(q['rx']*q['ry'] for q in [s[0],s[-1]])
for slug,reg in [('skateboarder','trucks'),('child-on-kick-scooter','deck')]:
    path=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/'+slug+'/*'))
    text=path.read_text()
    a=json.loads(text)['asset']
    v=a['volumes']['50']
    ids={p['of'] for p in v['thermal'] if p['region']==reg}
    outer=sum(stock(p) for p in v['body'] if p['id'] in ids and not p.get('texture'))
    if reg=='trucks':
        per_assembly=2*math.pi*.004*.132+2*math.pi*.0045*.042+math.pi*(.0135**2+.012**2)*2+math.pi*.006*.016+math.pi*(.004**2+.0045**2)*2
        outer-=2*per_assembly
    true_area=round(outer,3)
    region=next(r for r in a['thermal']['regions'] if r['id']==reg)
    if '--write' in sys.argv:
        i=text.index('"id": "'+reg+'"',text.index('"regions": [',text.index('"thermal": {')))
        j=text.index('"areaM2": ',i)+len('"areaM2": ')
        end=text.index(',',j)
        assert float(text[j:end])==region['areaM2']
        assert path.read_text()==text
        path.write_text(text[:j]+str(true_area)+text[end:])
    else:assert abs(true_area-region['areaM2'])<=.001,(slug,region['areaM2'],true_area)
    print(slug,reg,true_area)
