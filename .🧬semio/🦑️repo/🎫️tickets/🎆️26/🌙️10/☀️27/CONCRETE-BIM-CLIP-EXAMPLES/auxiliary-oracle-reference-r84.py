import json
from shapely.geometry import Polygon,box
from shapely.ops import unary_union
cases=[('ordinary',[box(0,0,4,4)],[box(1,1,3,3),box(10,10,11,11)]),('holes',[Polygon([(0,0),(6,0),(6,6),(0,6)],[[(2,2),(4,2),(4,4),(2,4)]])],[Polygon([(1,1),(5,1),(5,5),(1,5)],[[(2.5,2.5),(3.5,2.5),(3.5,3.5),(2.5,3.5)]])]),('overlaps',[box(0,0,4,3),box(2,1,6,4)],[box(1,0,3,4),box(2,2,5,5),box(-5,-5,-4,-4)])]
rows=[]
for name,raw,primary in cases:
 expected=unary_union(raw).difference(unary_union(primary));pieces=[];selected=0
 for a in raw:
  remainder=a;ab=a.bounds
  for p in primary:
   pb=p.bounds
   if ab[0]>pb[2] or ab[2]<pb[0] or ab[1]>pb[3] or ab[3]<pb[1]:continue
   selected+=1;remainder=remainder.difference(p)
  pieces.append(remainder)
 actual=unary_union(pieces);xor=expected.symmetric_difference(actual).area
 rows.append({'case':name,'expected_area':expected.area,'actual_area':actual.area,'xor':xor,'valid':actual.is_valid,'selected':selected,'possible':len(raw)*len(primary)})
 assert xor<1e-8 and actual.is_valid
print(json.dumps(rows))
