"""🚶 Identify actual physical owners of the published skirting aisle bottleneck."""
from pathlib import Path
import json
from shapely.geometry import Polygon,LineString,Point
from shapely.ops import unary_union
P=Path(__file__).parent;O=P/'🗑️generated'
r=json.loads((O/'ground-hall-final-rings-r92.json').read_text(encoding='utf8'));audit=json.loads((O/'ground-hall-final-r92.json').read_text(encoding='utf8'))
assert r['source']==audit['source_before'] and audit['source_stable'] and audit['producers_stable'] and not audit['errors']
rings=[(tag,Polygon(points)) for tag,points in r['rings']];occupied=unary_union([shape for _,shape in rings]);route=next(row['points'] for row in audit['witnesses'] if row['diameter']==.75)
best=None
for x,y in route:
 free=LineString([[7,y],[8.15,y]]).difference(occupied)
 segments=list(free.geoms) if hasattr(free,'geoms') else [free]
 for part in segments:
  if part.geom_type!='LineString' or part.bounds[0]>x or part.bounds[2]<x:continue
  width=part.bounds[2]-part.bounds[0]
  if best is None or width<best['width']:
   left,right=Point(part.bounds[0],y),Point(part.bounds[2],y)
   best={'width':width,'y':y,'x_interval':[left.x,right.x],'left_owners':[tag for tag,shape in rings if shape.distance(left)<1e-9],'right_owners':[tag for tag,shape in rings if shape.distance(right)<1e-9]}
report={'source':r['source'],'route_diameter':.75,'horizontal_bottleneck':best,'scope':'Actual native occupied horizontal cross sections along the existing supported .75m witness; connectivity proof remains the complete hall audit.'}
(O/'native-hall-bottleneck-r95.json').write_text(json.dumps(report,indent=2),encoding='utf8')
(P/'native-hall-bottleneck-r95.md').write_text('# Actual Ground Hall Bottleneck R95\n\nSource '+r['source']+'; the complete actual native-body .75m witness crosses the narrowest horizontal free interval '+str(best['x_interval'])+' at Y'+str(best['y'])+', width '+str(best['width'])+'m. Left physical owners: '+', '.join(best['left_owners'])+'. Right physical owners: '+', '.join(best['right_owners'])+'. This is an explanatory horizontal sample along the already proven route; it does not replace body connectivity or floor-support validation.\n',encoding='utf8')
print('[DEBUG] '+json.dumps(report),flush=True)
