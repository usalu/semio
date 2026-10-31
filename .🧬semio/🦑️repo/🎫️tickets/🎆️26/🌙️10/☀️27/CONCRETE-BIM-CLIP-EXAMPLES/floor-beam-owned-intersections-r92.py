"""🧱 Independently intersect floor finish material with exclusive native beam bodies."""
import hashlib,importlib,json,sys,time
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
from scipy.integrate import quad
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');facade=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🏗️facade.🐍️');ticket=Path(__file__).parent;path=Path(api.__file__).with_name('🔣️.json');raw=path.read_bytes();m=json.loads(raw);started=time.monotonic();producer=hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest();tags=[r['tag'] for r in json.loads((ticket/'floor-load-policy-cases-r92.json').read_text(encoding='utf8'))['per_covering']];floors=[]
for tag in tags:
 for i,p in enumerate(api.service_geometry(tag,m)['parts']):
  assert not any(k in p for k in ['mesh','section','top_outline']),tag
  floors.append({'tag':tag,'index':i,'part':p,'polygon':Polygon(p['outline'],p.get('holes',[]))})
zlo=min(r['part']['z0'] for r in floors);zhi=max(r['part']['z1'] for r in floors);beams=[];considered=[]
for obj in m['elements']:
 if obj['class']!='IfcBeam':continue
 rawg=obj['geometry'];base=api._z(m,rawg['base_level'])
 if 'section_sweep' in rawg:
  rawparts=facade.raw_parts(api,m,obj);intervals=[(p['z0'],p['z1']) for p in rawparts]
 else:
  intervals=[]
  for p in rawg['parts']:
   zs=[v[2] for v in p['mesh']['vertices']] if 'mesh' in p else [v[1] for v in p['section']['profile']] if 'section' in p else [p['z0'],p['z1']]
   intervals.append((base+min(zs),base+max(zs)))
 if not any(hi>zlo+1e-10 and lo<zhi-1e-10 for lo,hi in intervals):
  considered.append({'tag':obj['tag'],'active_owned_parts':0,'stage':'Exactrawheightnegativefilter; ownershipsubtractioncannotcreatenewmaterial','raw_height_intervals':intervals});continue
 print('[DEBUG] resolvingactivebeam '+obj['tag'],flush=True)
 g=api._solid_geometry(m,obj);active=[]
 for i,p in enumerate(g['parts']):
  if p['z1']<=zlo+1e-10 or p['z0']>=zhi-1e-10:continue
  active.append({'index':i,'part':p,'bounds':api.part_footprint(p)})
 considered.append({'tag':obj['tag'],'owned_parts':len(g['parts']),'shared_parts_excluded':len(g.get('shared_parts',[])),'active_owned_parts':len(active)})
 if active:beams.append({'tag':obj['tag'],'parts':active})
print('[DEBUG] actualexclusivebeamcandidatecount '+str(len(beams)),flush=True);rows=[]
for beam in beams:
 for bp in beam['parts']:
  p=bp['part'];foot=Polygon(bp['bounds']);bb=foot.bounds
  for f in floors:
   q=f['part'];lo,hi=max(q['z0'],p['z0']),min(q['z1'],p['z1'])
   if hi-lo<=1e-10:continue
   fb=f['polygon'].bounds
   if fb[2]<=bb[0] or fb[0]>=bb[2] or fb[3]<=bb[1] or fb[1]>=bb[3]:continue
   if any(k in p for k in ['mesh','section','top_outline']):
    def area(z):
     ring=api.part_outline_at(p,z)
     return f['polygon'].intersection(Polygon(ring)).area if len(ring)>=3 else 0.
    volume,error=quad(area,lo,hi,epsabs=1e-12,epsrel=1e-8,limit=100);cut=f['polygon'].intersection(Polygon(api.part_outline_at(p,(lo+hi)/2)))
   else:cut=f['polygon'].intersection(foot);volume=cut.area*(hi-lo);error=0.
   if volume>1e-12:
    xy=list(cut.representative_point().coords)[0];rows.append({'floor':f['tag'],'floor_part':f['index'],'floor_role':q.get('role'),'floor_material':q.get('material'),'beam':beam['tag'],'beam_part':bp['index'],'beam_material':p.get('material',api._material(m,api.element(beam['tag'],m))['tag']),'z':[lo,hi],'volume_m3':volume,'quadrature_error':error,'witness':[xy[0],xy[1],(lo+hi)/2],'intersection_outline':list(cut.exterior.coords) if cut.geom_type=='Polygon' else None})
summary={}
for r in rows:
 key=(r['floor'],r['floor_material'],r['beam']);summary[key]=summary.get(key,0.)+r['volume_m3']
record={'source':hashlib.sha256(raw).hexdigest(),'source_after':hashlib.sha256(path.read_bytes()).hexdigest(),'producer_before':producer,'producer_after':hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),'scope':'Exact17horizontalfirstfloorcoverings and exclusive owned parts ofall sourceIfcBeams; sharedslab/columnreservations excluded','floor_parts':len(floors),'floor_z_range':[zlo,zhi],'considered_beams':considered,'active_beams':[{'tag':b['tag'],'parts':len(b['parts'])} for b in beams],'positive_intersections':rows,'summary':[{'floor':k[0],'material':k[1],'beam':k[2],'volume_m3':v} for k,v in summary.items()],'pair_sum_volume_m3':sum(r['volume_m3'] for r in rows),'seconds':time.monotonic()-started};record['source_stable']=record['source']==record['source_after'];record['producer_stable']=record['producer_before']==record['producer_after'];(ticket/'🗑️generated/floor-beam-owned-intersections-r92.json').write_text(json.dumps(record,ensure_ascii=False,indent=2),encoding='utf8');print(json.dumps({k:v for k,v in record.items() if k not in ['considered_beams','positive_intersections']}),flush=True)
