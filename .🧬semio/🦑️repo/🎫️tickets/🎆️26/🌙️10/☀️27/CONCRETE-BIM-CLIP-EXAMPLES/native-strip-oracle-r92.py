"""📏 Independently intersect current declared slab regions and voids with a physical test strip."""
import json,hashlib
from pathlib import Path
from shapely.geometry import Polygon,LineString
p=Path.cwd()/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json';raw=p.read_bytes();m=json.loads(raw);s=next(x for x in m['elements'] if x['tag']=='SL-1');z=next(x['elevation']['value'] for x in m['storeys'] if x['id']=='first');line=LineString([(4.75,6.2),(14.25,6.2)]);voids=[Polygon(x['outline']) for x in s['geometry']['voids']];cells=[]
for r in s['geometry']['regions']:
 shape=Polygon(r['outline'])
 for void in voids:shape=shape.difference(void)
 cut=shape.intersection(line)
 for piece in ([cut] if cut.geom_type=='LineString' else getattr(cut,'geoms',())):
  if piece.geom_type!='LineString' or piece.length==0:continue
  a,b=piece.bounds[0],piece.bounds[2];hi=z+r['top_offset']['value'];lo=hi-r['thickness']['value'];cells.append([a,b,lo,hi])
near=[c for c in cells if c[0]<=8.128<=c[1]];reference=min(near or cells,key=lambda c:abs((c[0]+c[1])/2-8.128));middle=(reference[2]+reference[3])/2
report={'source_sha256':hashlib.sha256(raw).hexdigest(),'row':6.2,'bays':[2,4],'cells':cells,'view_centre':[9.5,6.2,middle/2],'width':.3*(max(c[1] for c in cells)-min(c[0] for c in cells)),'height_full':.3*(max(c[3] for c in cells)-min(c[2] for c in cells)),'height_collapsed':.3*(max((c[2]+c[3])/2 for c in cells)-min((c[2]+c[3])/2 for c in cells))};assert p.read_bytes()==raw;(Path(__file__).parent/'🗑️generated/native-r92/strip-source-oracle.json').write_text(json.dumps(report,indent=2));print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k!='cells'}))
