"""🍳 Audit actual lower wall faces, counter material intersection and foot support without a full scene."""
import hashlib,importlib,json,sys,time
from pathlib import Path
from shapely.geometry import Polygon,LineString,box
from shapely.ops import unary_union
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');path=Path(api.__file__).with_name('🔣️.json');raw=path.read_bytes();m=json.loads(raw);start=time.monotonic();before=hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest();counter=api.service_geometry('FIX-KITCHEN-SINK-COUNTER',m);walls={tag:api._wall_geometry(m,api.element(tag,m)) for tag in ['W-W','W-S']};rows=[]
for tag,g in walls.items():
 intersections=[]
 for i,p in enumerate(counter['parts']):
  poly=Polygon(p['outline'],p.get('holes',[]))
  for j,q in enumerate(g['parts']):
   lo,hi=max(p['z0'],q['z0']),min(p['z1'],q['z1'])
   if hi-lo<=1e-10:continue
   area=poly.intersection(Polygon(q['outline'],q.get('holes',[]))).area
   if area*(hi-lo)>1e-14:intersections.append({'counter_part':i,'role':p.get('role'),'wall_part':j,'layer':q.get('layer'),'material':q.get('material'),'z':[lo,hi],'volume':area*(hi-lo)})
 sections=[]
 for z in [3.20,3.60,4.029,4.031,4.067,4.071,4.50]:
  shape=unary_union([Polygon(p['outline'],p.get('holes',[])) for p in g['parts'] if p['z0']+1e-10<z<p['z1']-1e-10]);line=LineString([[-.5,.4566],[.5,.4566]]) if tag=='W-W' else LineString([[1.5,-.5],[1.5,.5]]);cut=shape.intersection(line);sections.append({'z':z,'axis':'X' if tag=='W-W' else 'Y','bounds':list(cut.bounds) if not cut.is_empty else [],'parts':[{'material':p.get('material'),'layer':p.get('layer'),'bounds':list(Polygon(p['outline']).bounds)} for p in g['parts'] if p['z0']<z<p['z1'] and Polygon(p['outline']).intersects(line)]})
 rows.append({'tag':tag,'source_parts':len(g['parts']),'counter_overlap_volume':sum(x['volume'] for x in intersections),'intersections':intersections,'sections':sections})
floors={o['tag']:api.service_geometry(o['tag'],m)['parts'] for o in m['elements'] if o['class']=='IfcCovering' and o.get('predefined_type')=='FLOORING' and o['storey']=='first'};feet=[]
for i,p in enumerate(counter['parts']):
 if p.get('role')!='inferred_counter_leg':continue
 foot=Polygon(p['outline']);owners=[]
 for tag,ps in floors.items():
  shape=unary_union([Polygon(q['outline'],q.get('holes',[])) for q in ps if abs(q['z1']-p['z0'])<1e-9]);area=shape.intersection(foot).area
  if area>1e-14:owners.append({'tag':tag,'area':area})
 feet.append({'counter_part':i,'foot_outline':p['outline'],'base_z':p['z0'],'area':foot.area,'support_owners':owners,'unsupported_area':foot.area-sum(x['area'] for x in owners)})
record={'source':hashlib.sha256(raw).hexdigest(),'source_after':hashlib.sha256(path.read_bytes()).hexdigest(),'producer_before':before,'producer_after':hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),'room_outline':api.element('SP-KITCHEN',m)['outline'],'counter_bounds':{'min':[min(v[k] for p in counter['parts'] for v in api.part_vertices(p)) for k in range(3)],'max':[max(v[k] for p in counter['parts'] for v in api.part_vertices(p)) for k in range(3)]},'walls':rows,'foot_support':feet,'seconds':time.monotonic()-start};record['source_stable']=record['source']==record['source_after'];record['producer_stable']=record['producer_before']==record['producer_after'];(Path(__file__).parent/'🗑️generated/kitchen-west-interface-r92.json').write_text(json.dumps(record,ensure_ascii=False,indent=2),encoding='utf8');print(json.dumps({k:v for k,v in record.items() if k!='walls'}),flush=True);print(json.dumps({'walls':[{'tag':r['tag'],'overlap':r['counter_overlap_volume'],'sections':[{'z':x['z'],'axis':x['axis'],'bounds':x['bounds']} for x in r['sections']]} for r in rows]}),flush=True)
