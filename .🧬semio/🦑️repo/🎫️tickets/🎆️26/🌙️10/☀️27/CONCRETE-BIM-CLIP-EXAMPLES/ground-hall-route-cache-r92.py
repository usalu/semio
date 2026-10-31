"""🚶 Verify full-body routes from frozen exact occupied-band captures, without fixture exclusions."""
import json,math
from pathlib import Path
from shapely.geometry import Polygon,LineString,box
from shapely.ops import unary_union
p=Path(__file__).with_name('🗑️generated');d=json.loads((p/'ground-hall-body-open-rings-r92.json').read_text());rings=[(t,Polygon(q)) for t,q in d['rings']];occupied=unary_union([q for t,q in rings]);support=unary_union([Polygon(q['outline'],q['holes']) for q in d['floor']]);domain=box(7.,11.7,8.15,19.9)
def audit(diameter,end):
 free=domain.difference(occupied.buffer(diameter/2+.001,quad_segs=128));parts=list(free.geoms) if hasattr(free,'geoms') else [free];start=LineString([[7,12.2],[8.15,12.2]]);finish=LineString([[7,end],[8.15,end]]);part=next((q for q in parts if q.area>0 and q.intersects(start) and q.intersects(finish)),None)
 if part is None:return {'diameter':diameter,'end_y':end,'connected':False}
 points=[]
 for i in range(round((end-12.2)*1000)+1):
  y=12.2+i*.001;s=part.intersection(LineString([[7,y],[8.15,y]]));pieces=list(s.geoms) if hasattr(s,'geoms') else [s];lines=[q.bounds for q in pieces if q.geom_type=='LineString' and q.length>2e-6]
  if not lines:return {'diameter':diameter,'end_y':end,'connected':True,'witness':False,'empty_slice':y}
  b=min(lines,key=lambda b:abs(max(b[0],min(7.52,b[2]))-7.52));points.append([max(b[0]+1e-6,min(7.52,b[2]-1e-6)),y])
 line=LineString(points).simplify(1e-12);body=line.buffer(diameter/2+.00004,quad_segs=128);hits=[{'tag':t,'area':q.intersection(body).area} for t,q in rings if q.intersection(body).area>1e-10];missing=body.difference(support).area
 return {'diameter':diameter,'end_y':end,'connected':True,'witness':not hits and missing<1e-8,'points':list(line.coords),'collisions':hits,'floor_missing_area':missing,'body_vertical_band':[.05,1.85]}
out={'frozen_source':d['source'],'operated':d['operated'],'all_actual_leaf_and_frame_parts_retained':True,'checks':[audit(w,end) for end in (17.7,19.4) for w in (.80,.79,.78,.77,.76)],'inference':'High-X hinge and inward -90deg operated door pose inferred; no source dimensions, fixture positions or body exclusions changed.'};(p/'ground-hall-route-cache-r92.json').write_text(json.dumps(out));print(json.dumps({**out,'checks':[{k:v for k,v in q.items() if k!='points'} for q in out['checks']]}),flush=True)
