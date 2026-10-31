"""🚶 Verify full-body routes from frozen exact occupied-band captures, without fixture exclusions."""
import json,math
from pathlib import Path
from shapely.geometry import Polygon,LineString,box
from shapely.ops import unary_union
p=Path(__file__).with_name('🗑️generated');d=json.loads((p/'ground-hall-body-open-rings-r92.json').read_text());rings=[(t,Polygon(q)) for t,q in d['rings']];occupied=unary_union([q for t,q in rings]);support=unary_union([Polygon(q['outline'],q['holes']) for q in d['floor']]);domain=box(7.,11.7,8.15,19.9)

for diameter in (.8,.78):
 free=domain.difference(occupied.buffer(diameter/2+.001,quad_segs=128));bad=[]
 for i in range(5501):
  y=12.2+i*.001
  if free.intersection(LineString([[7,y],[8.15,y]])).is_empty:bad.append(y)
 print('[DEBUG] blocked slices',diameter,(min(bad),max(bad)) if bad else None,flush=True)
 for y in ([13.15] if not bad else [min(bad),sum(bad)/len(bad),max(bad)]):
  line=LineString([[7,y],[8.5,y]]);near=[{'tag':t,'bounds':q.intersection(line).bounds} for t,q in rings if q.intersects(line)];print(json.dumps({'diameter':diameter,'y':y,'sourceintersections':near}),flush=True)
