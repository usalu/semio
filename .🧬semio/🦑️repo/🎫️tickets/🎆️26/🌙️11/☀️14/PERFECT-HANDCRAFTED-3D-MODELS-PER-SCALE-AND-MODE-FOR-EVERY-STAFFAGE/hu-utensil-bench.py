"""🪑 Sums exposed timber stock and the rounded spoon bowl and shank independently of the mesh validator."""
from pathlib import Path
import json,math
ticket=Path(__file__).resolve().parent
root=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
def bench(a):
    total=0
    for p in a['volumes']['50']['body']:
        if not p['id'].startswith('bench-') or p.get('texture'):continue
        if p['kind']=='box':
            w,h,l=p['size']
            total+=2*(w*h+w*l+l*h)
        else:
            q=p['profile'];edges=list(zip(q,q[1:]+q[:1]))
            total+=sum(math.dist(a,b) for a,b in edges)*(p['to']-p['from'])+abs(sum(a[0]*b[1]-b[0]*a[1] for a,b in edges))
    return total-4*2*2*.1*.04-3*2*2*.08*.045-2*2*(.045*.07+.035*.06+.04*.08)-2*2*.035*.06
def spoon(a):
    return math.pi*.0114*.244+2*math.pi*.029*.0205+math.pi*(.029+.0205)*.005
for slug,reg,fn in [('seated-reader','bench',bench),('person-cooking','spoon',spoon)]:
    path=next((root/slug).glob('*'));old=path.read_text();a=json.loads(old)['asset']
    r=next(r for r in a['thermal']['regions'] if r['id']==reg)
    val=round(fn(a),3)
    i=old.index('"id": "'+reg+'"',old.index('"regions": [',old.index('"thermal": {')))
    j=old.index('"areaM2": ',i)+len('"areaM2": ');k=old.index(',',j)
    assert float(old[j:k])==r['areaM2'] and path.read_text()==old
    path.write_text(old[:j]+str(val)+old[k:])
    print(slug,reg,val)
