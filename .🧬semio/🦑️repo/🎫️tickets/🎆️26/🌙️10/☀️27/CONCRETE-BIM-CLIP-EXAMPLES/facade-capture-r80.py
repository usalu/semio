import importlib,json,time
from pathlib import Path
from shapely.geometry import Polygon
from shapely.validation import explain_validity
ticket=Path("/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES");gen=ticket/'🗑️generated';api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
parts=json.loads((gen/'facade-profile-parts-r80.json').read_text());z=6.140000000000001;rows=[];invalid=[];started=time.monotonic()
for index,p in enumerate(parts['primary']):
 if p['z0']<z<p['z1']:
  ring=api.part_outline_at(p,z)
  if ring:
   holes=p.get('holes',[]);polygon=Polygon(ring,holes);rows.append({'source_part':index,'rings':[ring,*holes]})
   if not polygon.is_valid or polygon.area<=0:invalid.append({'source_part':index,'reason':explain_validity(polygon),'area':polygon.area})
assert len(rows)==813
record={'z':z,'source':parts['source'],'count':len(rows),'invalid':invalid,'seconds':time.monotonic()-started,'polygons':rows}
(gen/'facade-union813-input-r80.json').write_text(json.dumps(record),encoding='utf8');print({k:v for k,v in record.items() if k!='polygons'})
