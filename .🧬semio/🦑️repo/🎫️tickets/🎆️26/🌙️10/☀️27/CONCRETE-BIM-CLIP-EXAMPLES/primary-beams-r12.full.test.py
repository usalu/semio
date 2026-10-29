import json, math
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
p=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES')
a=json.loads((p/'🗑️generated/primary-beams-r12.full.actual.json').read_text())
for slab in a['slabs']:
 old=unary_union([Polygon(r['outline']) for r in slab['before']]);new=unary_union([Polygon(r['outline']) for r in slab['regions']])
 assert old.symmetric_difference(new).area<1e-7
 assert math.isclose(sum(Polygon(r['outline']).area for r in slab['regions']),new.area,abs_tol=1e-7)
 for r in slab['regions']:
  if 'member' in r: assert r['material']=='MAT-RC' and r['member'].startswith('BEAM-PRIMARY-')
 print(slab['tag'],len(slab['regions']),new.area,'regional coverage preserved; no overlap')
assert len(a['members'])==59
print('59 primary full-section references, exclusive parts and embedded member ownership prepared')

from shapely.geometry import box
slabs=[]
for slab in a['slabs']:
 ref=slab['reference'];mask=Polygon(ref['outline']).difference(unary_union([Polygon(v['outline']) for v in ref['voids']]))
 used=Polygon()
 for r in slab['before']:
  poly=Polygon(r['outline']).difference(used).intersection(mask);used=used.union(Polygon(r['outline']))
  top=ref['elevation']+r['top_offset']['value'];slabs.append((poly,top-r['thickness']['value'],top))
full=[]
for m in a['members']:
 s=m['spec'];v=s['axis'];w=s['width']/2
 if abs(v[1][0]-v[0][0])<abs(v[1][1]-v[0][1]): x=(v[0][0]+v[1][0])/2;poly=box(x-w,min(v[0][1],v[1][1]),x+w,max(v[0][1],v[1][1]))
 else:y=(v[0][1]+v[1][1])/2;poly=box(min(v[0][0],v[1][0]),y-w,max(v[0][0],v[1][0]),y+w)
 full.append((poly,*s['full_z']))
actual=[(box(q[0],q[1],q[3],q[4]),q[2],q[5]) for m in a['members'] for q in m['boxes']]
zz=sorted(set(round(x,9) for _,lo,hi in full+slabs+actual for x in (lo,hi)))
expected=got=0
for lo,hi in zip(zz,zz[1:]):
 mid=(lo+hi)/2;f=unary_union([p for p,l,h in full if l<mid<h]);s=unary_union([p for p,l,h in slabs if l<mid<h]);g=unary_union([p for p,l,h in actual if l<mid<h]);want=f.difference(s)
 assert want.symmetric_difference(g).area<1e-7,(lo,hi,want.symmetric_difference(g).area)
 assert g.intersection(s).area<1e-7
 expected+=want.area*(hi-lo);got+=g.area*(hi-lo)
assert math.isclose(expected,got,abs_tol=1e-7)
print('Independent Shapely full section minus occupied slabs volume',expected,'exclusive volume',got,'m3; no slab intersection')
