import importlib,json,hashlib,time
from pathlib import Path
from shapely.geometry import Polygon
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');base=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');out=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated');sourcepath=base/'🧱️model/🔣️.json';raw=sourcepath.read_bytes();m=json.loads(raw);started=time.monotonic();g=api.building_geometry(m)
case=json.loads((base/'🧱️model/🧪️tests/🛁️details.json').read_text());integrates={c['tag']:c.get('integrates',[]) for c in case['fixtures'] if c.get('integrates')}
structures=[(obj['tag'],'column',i,p) for obj in g['pilotis'] for i,p in enumerate(obj['parts'])]+[(obj['tag'],'wall',i,p) for obj in g['walls'] for i,p in enumerate(obj['parts'])]
shapes=[(tag,kind,i,p,Polygon(p['outline'],p.get('holes',p.get('voids',[])))) for tag,kind,i,p in structures];hits=[];candidates=0;totalpairs=0
for item in g['furnishings']:
 for i,p in enumerate(item['parts']):
  a=Polygon(p['outline'],p.get('holes',p.get('voids',[])));ab=a.bounds
  for tag,kind,j,q,b in shapes:
   totalpairs+=1;lo,hi=max(p['z0'],q['z0']),min(p['z1'],q['z1'])
   if hi<=lo:continue
   bb=b.bounds
   if ab[0]>=bb[2] or ab[2]<=bb[0] or ab[1]>=bb[3] or ab[3]<=bb[1]:continue
   candidates+=1;intersection=a.intersection(b)
   if intersection.area>0:hits.append({'furnishing':item['tag'],'part':i,'structure':tag,'kind':kind,'structure_part':j,'xy_area':intersection.area,'z0':lo,'z1':hi,'volume':intersection.area*(hi-lo),'intersection':intersection.__geo_interface__,'furnishing_outline':p['outline'],'furnishing_holes':p.get('holes',[]),'structure_outline':q['outline'],'structure_holes':q.get('holes',[]),'old_case_integrates':tag in integrates.get(item['tag'],[])})
 (out/'furniture-structure-r89-progress.json').write_text(json.dumps({'completed_furnishing':item['tag'],'hits':len(hits),'candidates':candidates,'seconds':time.monotonic()-started}))
record={'source_before':hashlib.sha256(raw).hexdigest(),'source_after':hashlib.sha256(sourcepath.read_bytes()).hexdigest(),'products':len(m['elements']),'furnishings':len(g['furnishings']),'furnishing_parts':sum(len(o['parts']) for o in g['furnishings']),'column_section_parts':sum(len(o['parts']) for o in g['pilotis']),'wall_parts':sum(len(o['parts']) for o in g['walls']),'total_pairs':totalpairs,'bbox_z_candidates':candidates,'positive_intersections':hits,'integrates_fixture':integrates,'seconds':time.monotonic()-started,'production':{str(p.relative_to(base)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [base/'🧱️model/🐍️.py',base/'🧱️model/🧮️analysis/🐍️.py']}}
(out/'furniture-structure-r89.json').write_text(json.dumps(record));print(json.dumps({k:v for k,v in record.items() if k!='positive_intersections'}));print(json.dumps({'positive_pairs':len(hits),'volume_sum':sum(p['volume'] for p in hits)}))
