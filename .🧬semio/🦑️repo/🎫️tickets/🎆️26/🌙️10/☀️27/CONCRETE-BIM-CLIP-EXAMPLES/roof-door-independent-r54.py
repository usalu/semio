import json, math
from pathlib import Path
from shapely.geometry import Polygon,LineString
from shapely.ops import unary_union
p=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES')
d=json.loads((p/'🗑️generated/roof-door-candidate-geometry-r49.json').read_text())
parts=d['door']['parts'];walls=d['wall']['parts']
def volume(a,b):
 ax=[q[0] for q in a['outline']];ay=[q[1] for q in a['outline']];bx=[q[0] for q in b['outline']];by=[q[1] for q in b['outline']]
 if max(ax)<=min(bx) or max(bx)<=min(ax) or max(ay)<=min(by) or max(by)<=min(ay):return 0
 dz=max(0,min(a['z1'],b['z1'])-max(a['z0'],b['z0']))
 return Polygon(a['outline']).intersection(Polygon(b['outline'])).area*dz
ov=sum(volume(a,b) for a in parts for b in walls)
selfov=sum(volume(a,b) for i,a in enumerate(parts) for b in parts[i+1:])
s=next(x for x in d['stairs'] if x['tag']=='ST-FR');last=s['steps'][-1];q=Polygon(last['outline']).centroid
axis=d['door']['axis'];end=(axis[0][0]+.025,(axis[0][1]+axis[1][1])/2)
route=LineString([(q.x,q.y),end]);feet=route.buffer(.04,cap_style=2)
supports=[Polygon(x['outline']) for x in s['steps']+s['landings']+d['roof']['parts']+d['roof']['finish_parts'] if abs(x['z1']-6.6)<1e-7]
support=unary_union(supports)
obstacles=unary_union([Polygon(x['outline']) for x in walls+[p for r in d['rails'] for p in r['parts']] if x['z1']>6.65 and x['z0']<8.4])
result={'source_hash':d['raw_source'],'parts':len(parts),'door_wall_overlap_m3':ov,'door_self_overlap_m3':selfov,'head':max(x['z1'] for x in parts),'clear_leaf':[d['door']['clear_width'],d['door']['clear_height']],'route_assumed':list(route.coords),'route_length':route.length,'route_intersects_wall_or_guard':route.intersection(obstacles).length,'minimum_route_to_wall_guard':route.distance(obstacles),'foot_sample_radius':.04,'unsupported_foot_sample_area':feet.difference(support).area,'materials':sorted(set(x['material'] for x in parts))}
(p/'🗑️generated/roof-door-independent-r54.json').write_text(json.dumps(result));print(json.dumps(result))
