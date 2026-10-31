"""🛁 Independently integrate occupied chauffeur-bath sections and list real door clear dimensions."""
import hashlib,importlib,json,sys,time
from pathlib import Path
from scipy.integrate import quad
from shapely.geometry import Polygon,MultiPoint
from shapely.affinity import translate
from shapely.ops import unary_union
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');source=Path(api.__file__).with_name('🔣️.json');raw=source.read_bytes();m=json.loads(raw);ticket=Path(__file__).parent;started=time.monotonic();producer=hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest()
bath=api.service_geometry('BATH-CHAUFFEUR',m);column=api.column_geometry(m,api.element('30',m));parts=bath['parts'];cp=column['parts'];rows=[]
def shape(p,z):
 ring=api.part_outline_at(p,z)
 return Polygon(ring) if len(ring)>=3 else Polygon()
def occupied(z):return unary_union([shape(p,z) for p in cp if p['z0']<=z<=p['z1']])
def collision(dx=0.,dy=0.,details=False):
 total=0.;out=[]
 for i,p in enumerate(parts):
  lo,hi=p['z0'],p['z1'];vs=api.part_vertices(p);b=[min(v[k] for v in vs) for k in range(3)]+[max(v[k] for v in vs) for k in range(3)]
  cb=occupied((lo+hi)/2).bounds
  if not cb or b[3]+dx<=cb[0] or b[0]+dx>=cb[2] or b[4]+dy<=cb[1] or b[1]+dy>=cb[3]:continue
  def area(z):return translate(shape(p,z),xoff=dx,yoff=dy).intersection(occupied(z)).area
  volume,error=quad(area,lo,hi,epsabs=1e-12,epsrel=1e-8,limit=200)
  if volume>1e-13:
   total+=volume;out.append({'index':i,'role':p.get('role'),'z':[lo,hi],'volume_m3':volume,'quadrature_error':error,'section_area_samples':[{'z':z,'area_m2':area(z)} for z in [lo,(lo+hi)/2,hi]],'part':p})
 return (total,out) if details else total
bath_projection_for_search=unary_union([MultiPoint([v[:2] for v in api.part_vertices(p)]).convex_hull for p in parts]);column_projection_for_search=occupied(.37)
def projection_collision(dx,dy):return translate(bath_projection_for_search,xoff=dx,yoff=dy).intersection(column_projection_for_search).area
volume,rows=collision(details=True);candidates=[]
if volume>1e-12:
 for name,vector in [('east',[1.,0.]),('south',[0.,-1.]),('southeast',[2**-.5,-2**-.5])]:
  if projection_collision(vector[0]*.12,vector[1]*.12)>1e-12:continue
  lo,hi=0.,.12
  for _ in range(30):
   mid=(lo+hi)/2
   if projection_collision(vector[0]*mid,vector[1]*mid)>1e-12:lo=mid
   else:hi=mid
  chosen=hi+.001
  candidates.append({'direction':name,'zero_overlap_threshold_m':hi,'candidate_shift':[chosen*vector[0],chosen*vector[1]],'candidate_axis':[api.element('BATH-CHAUFFEUR',m)['geometry']['axis'][k]+chosen*vector[k] for k in range(2)],'candidate_overlap_m3':collision(chosen*vector[0],chosen*vector[1]),'basis':'Inferred geometry-clearance candidate; radial uncertainty must be checked against original registeredcentre, not current already-shiftedaxis.'})
original=[14.58529411764706,19.383823529411764]
from shapely.geometry import MultiPoint,LineString
bath_projection=unary_union([MultiPoint([v[:2] for v in api.part_vertices(p)]).convex_hull for p in parts])
neighbor_shapes={}
for tag in ['BIDET-CHAUFFEUR','FIX-CHAUFFEUR-BASIN']:
 neighbor_shapes[tag]=unary_union([MultiPoint([v[:2] for v in api.part_vertices(p)]).convex_hull for p in api.service_geometry(tag,m)['parts']])
for tag in ['W-CHAUFFEUR-BATH','W-G-E','W-G-N']:
 neighbor_shapes[tag]=unary_union([Polygon(p['outline'],p.get('holes',[])) for p in api.path_wall_geometry(m,api.element(tag,m))['parts'] if p['z0']<.60 and p['z1']>0])
neighbor_shapes['D-CHAUFFEUR-BATH-.60-body']=LineString([[13.65,18.2],[14.55,18.2]]).buffer(.30,cap_style=2)
for c in candidates:
 q=translate(bath_projection,xoff=c['candidate_shift'][0],yoff=c['candidate_shift'][1]);c['radial_offset_from_original_m']=sum((c['candidate_axis'][k]-original[k])**2 for k in range(2))**.5;c['neighbors']=[{'tag':tag,'distance_m':q.distance(s),'projection_overlap_area_m2':q.intersection(s).area} for tag,s in neighbor_shapes.items()]
witness_shape=shape(parts[43],.55).intersection(occupied(.55));witness=list(witness_shape.representative_point().coords)[0]+(.55,)
fragments=[]
for tag in ['BATH-CHAUFFEUR','30']:
 f=api.step_excerpt(tag,m,language='en');fragments.append({'tag':tag,'text':'\n'.join(f['lines']),'ids':f['ids']})
doors=[]
for obj in m['elements']:
 if obj['class']!='IfcDoor':continue
 g=api.filling_geometry(obj['tag'],m);doors.append({k:g.get(k) for k in ['tag','rough_width','rough_height','clear_width','clear_height','clear_sill_z','clear_head_z','operation','frame_width','frame_depth']});doors[-1].update({'tag':obj['tag'],'type':obj.get('type'),'opening':obj['geometry']['opening'],'fails_generic_width_070':g['clear_width']<.70-1e-9,'fails_generic_height_210':g['clear_height']<2.10-1e-9})
record={'native_fragments':fragments,'native_overlap_witness':witness,'source':hashlib.sha256(raw).hexdigest(),'source_after':hashlib.sha256(source.read_bytes()).hexdigest(),'producer_before':producer,'producer_after':hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),'bath_axis':api.element('BATH-CHAUFFEUR',m)['geometry']['axis'],'column_axis':api.element('30',m)['geometry']['axis'],'collision_volume_m3':volume,'colliding_parts':rows,'candidates':candidates,'doors':doors,'seconds':time.monotonic()-started};record['whole_source_stable']=record['source']==record['source_after'];record['producer_file_stable']=record['producer_before']==record['producer_after'];(ticket/'🗑️generated/chauffeur-bath-doors-independent-r92.json').write_text(json.dumps(record,ensure_ascii=False,indent=2),encoding='utf8');print(json.dumps({k:v for k,v in record.items() if k not in ['colliding_parts','native_fragments']}),flush=True);print(json.dumps({'colliding_parts':[{'index':r['index'],'role':r['role'],'z':r['z'],'volume':r['volume_m3']} for r in rows]}),flush=True)
