"""🔎 Audit exclusive authored solids with independent horizontal sections and GEOS."""
import argparse,hashlib,importlib,json,math,sys,time,traceback,warnings
from collections import Counter,defaultdict
from functools import lru_cache
from pathlib import Path
import numpy as np
from scipy.integrate import quad,IntegrationWarning
from shapely.geometry import Polygon,Point,LineString,box,mapping,shape
from shapely.ops import polygonize,unary_union
from shapely.strtree import STRtree
T=Path(__file__).parent;G=T/'🗑️generated';G.mkdir(exist_ok=True)
def save(name,value):
 (G/name).write_text(json.dumps(value,ensure_ascii=False,indent=2,allow_nan=False),encoding='utf8')
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def circle_ring(ring,c,r):
 total=0.
 for aa,bb in zip(ring,ring[1:]+ring[:1]):
  a=np.asarray(aa,dtype=float)-c;b=np.asarray(bb,dtype=float)-c;d=b-a;A=float(d@d);B=2*float(a@d);C=float(a@a)-r*r;ts=[0.,1.]
  disc=B*B-4*A*C
  if A and disc>0:
   ts += [v for v in ((-B-math.sqrt(disc))/(2*A),(-B+math.sqrt(disc))/(2*A)) if 0<v<1]
  ts=sorted(ts)
  for lo,hi in zip(ts,ts[1:]):
   u=a+lo*d;v=a+hi*d;mid=(u+v)/2
   total+=cross(u,v)/2 if mid@mid<r*r else r*r*math.atan2(cross(u,v),float(u@v))/2
 return abs(total)
def circle_area(poly,c,r):
 if poly.is_empty:return 0.
 if poly.distance(Point(c))>=r:return 0.
 if poly.geom_type=='Polygon':return max(0.,circle_ring(list(poly.exterior.coords)[:-1],c,r)-sum(circle_ring(list(h.coords)[:-1],c,r) for h in poly.interiors))
 return sum(circle_area(q,c,r) for q in poly.geoms)
def topology(p):
 if p.get('holes') and not any(k in p for k in ('mesh','section','top_outline')):
  loops=[p['outline'],*p['holes']];bottom=[[*v,p['z0']] for ring in loops for v in ring];top=[[*v,p['z1']] for ring in loops for v in ring];n=len(bottom);faces=[list(range(len(loops[0]))),list(range(n,n+len(loops[0])))];offset=0
  for ring in loops:
   for i in range(len(ring)):
    a=offset+i;b=offset+(i+1)%len(ring);faces.append([a,b,n+b,n+a])
   offset+=len(ring)
  return bottom+top,faces
 if 'mesh' in p:return p['mesh']['vertices'],p['mesh']['faces']
 if 'section' in p:
  s=p['section'];v=[[u,t,z] if s['plane']=='XZ' else [t,u,z] for t in s['sweep'] for u,z in s['profile']]
 else:v=[[*a,p['z0']] for a in p['outline']]+[[*a,p['z1']] for a in p.get('top_outline',p['outline'])]
 n=len(v)//2;return v,[list(reversed(range(n))),list(range(n,2*n))]+[[i,(i+1)%n,n+(i+1)%n,n+i] for i in range(n)]
class Solid:
 def __init__(self,record):
  self.r=record;self.p=record['geometry'];p=self.p;self.complex=any(k in p for k in ('mesh','section','top_outline'));self.circle=p.get('profile',{}).get('shape')=='circle' and not self.complex
  self.v,self.faces=topology(p);self.v=np.asarray(self.v,dtype=float);self.lo=float(min(self.v[:,2]));self.hi=float(max(self.v[:,2]));self.bounds=(float(min(self.v[:,0])),float(min(self.v[:,1])),float(max(self.v[:,0])),float(max(self.v[:,1])))
  self.foot=box(*self.bounds);self.poly=None if self.complex or self.circle else Polygon(p['outline'],p.get('holes',[]))
  self.edges=sorted({tuple(sorted((a,b))) for f in self.faces for a,b in zip(f,f[1:]+f[:1])});self.planes=[]
  if not self.circle:
   for f in self.faces:
    a=self.v[f[0]];normal=None
    for k in range(1,len(f)-1):
     n=np.cross(self.v[f[k]]-a,self.v[f[k+1]]-a)
     if np.linalg.norm(n)>0:normal=n/np.linalg.norm(n);break
    if normal is None:raise ValueError('Degenerate face')
    residual=max(abs(float((self.v[i]-a)@normal)) for i in f)
    if residual>1e-8:raise ValueError('Nonplanar source face '+str(residual))
    self.planes.append((normal,float(normal@a)))
  if self.circle:
   self.c=np.mean(np.asarray(p['outline']),axis=0);self.radius=p['profile']['diameter']['value']/2
   self.bounds=(self.c[0]-self.radius,self.c[1]-self.radius,self.c[0]+self.radius,self.c[1]+self.radius);self.foot=box(*self.bounds)
 @lru_cache(maxsize=48)
 def section(self,z):
  if not self.lo<z<self.hi:return Polygon()
  if self.poly is not None:return self.poly
  if self.circle:raise ValueError('Analytic circle uses circle-polygon integration')
  if 'mesh' not in self.p and 'section' not in self.p:
   t=(z-self.lo)/(self.hi-self.lo);return Polygon([(a[0]+t*(b[0]-a[0]),a[1]+t*(b[1]-a[1])) for a,b in zip(self.p['outline'],self.p['top_outline'])])
  pts={}
  for i,j in self.edges:
   a,b=self.v[i],self.v[j]
   if (a[2]-z)*(b[2]-z)<0:
    t=(z-a[2])/(b[2]-a[2]);q=a+t*(b-a);pts[(i,j)]=(float(q[0]),float(q[1]))
  lines=[]
  for face in self.faces:
   cut=[pts[tuple(sorted((i,j)))] for i,j in zip(face,face[1:]+face[:1]) if tuple(sorted((i,j))) in pts]
   cut=list(dict.fromkeys(cut))
   if len(cut)<2:continue
   axis=0 if max(q[0] for q in cut)-min(q[0] for q in cut)>max(q[1] for q in cut)-min(q[1] for q in cut) else 1
   cut.sort(key=lambda q:q[axis])
   if len(cut)%2:raise ValueError('Odd face intersections')
   lines += [LineString([a,b]) for a,b in zip(cut[::2],cut[1::2]) if a!=b]
  cells=list(polygonize(lines))
  if lines and not cells:raise ValueError('Open section graph')
  rings=[Polygon(q.exterior) for q in cells];selected=[]
  for q in cells:
   point=q.representative_point();depth=sum(r.contains(point) for r in rings)
   if depth%2:selected.append(q)
  result=unary_union(selected) if selected else Polygon()
  if not result.is_valid:raise ValueError('Invalid independent mesh section')
  return result

def analytic_profile_area(s,poly):
 value=circle_area(poly,s.c,s.radius);angle=math.radians(s.p['profile'].get('rotation',0));cs,sn=math.cos(angle),math.sin(angle)
 for h in s.p['profile'].get('voids',[]):
  x,y=h['offset'];centre=s.c+np.asarray([cs*x-sn*y,sn*x+cs*y]);value-=circle_area(poly,centre,h['diameter']['value']/2)
 return max(0.,value)
def area_pair(a,b,z):
 if a.circle and b.circle:raise ValueError('Two analytic circles pending')
 if a.circle:return analytic_profile_area(a,b.section(z))
 if b.circle:return analytic_profile_area(b,a.section(z))
 return a.section(z).intersection(b.section(z)).area

def events(a,b,lo,hi):
 values={lo,hi}|{float(v[2]) for s in (a,b) for v in s.v if lo<v[2]<hi}
 for s,t in ((a,b),(b,a)):
  if t.circle:continue
  for i,j in s.edges:
   u,v=s.v[i],s.v[j];d=v-u
   for n,c in t.planes:
    den=float(n@d)
    if not den:continue
    f=(c-float(n@u))/den
    if 0<f<1:
     z=float(u[2]+f*d[2])
     if lo<z<hi:values.add(z)
 return sorted(values)
def circle_witness(circle,other,z):
 poly=other.section(z);centre=Point(circle.c)
 def valid(p):
  if not poly.contains(p) or p.distance(centre)>=circle.radius:return False
  angle=math.radians(circle.p['profile'].get('rotation',0));cs,sn=math.cos(angle),math.sin(angle)
  return all(p.distance(Point(circle.c+np.asarray([cs*h['offset'][0]-sn*h['offset'][1],sn*h['offset'][0]+cs*h['offset'][1]])))>h['diameter']['value']/2 for h in circle.p['profile'].get('voids',[]))
 point=poly.representative_point()
 if valid(point):return [point.x,point.y,z]
 from shapely.ops import nearest_points
 edge=nearest_points(poly,centre)[0];end=poly.representative_point()
 for power in range(1,61):
  t=2.**(-power);point=Point(edge.x+t*(end.x-edge.x),edge.y+t*(end.y-edge.y))
  if valid(point):return [point.x,point.y,z]
 return None

def intersection(a,b):
 lo,hi=max(a.lo,b.lo),min(a.hi,b.hi)
 if hi<=lo:return {'volume':0.,'error':0.,'method':'strict_Z_negative'}
 if (a.circle and b.poly is not None) or (b.circle and a.poly is not None):
  v=area_pair(a,b,(lo+hi)/2)*(hi-lo);circle,other=(a,b) if a.circle else (b,a);return {'volume':v,'error':0.,'method':'analytic_circle_prism_exact_height','witness':circle_witness(circle,other,(lo+hi)/2) if v else None}
 if a.poly is not None and b.poly is not None:
  q=a.poly.intersection(b.poly);v=q.area*(hi-lo);pt=q.representative_point() if v else None
  return {'volume':v,'error':0.,'method':'GEOS_prism_exact_height','witness':None if pt is None else [pt.x,pt.y,(lo+hi)/2]}
 ev=events(a,b,lo,hi);volume=0.;error=0.;witness=None;uncertain_bound=0.;intervals=[]
 area_bound=max(0.,min(a.bounds[2],b.bounds[2])-max(a.bounds[0],b.bounds[0]))*max(0.,min(a.bounds[3],b.bounds[3])-max(a.bounds[1],b.bounds[1]))
 for x,y in zip(ev,ev[1:]):
  if y<=x:continue
  mid=(x+y)/2;half=(y-x)/2
  try:
   if mid==x or mid==y:raise ArithmeticError('No representable interior midpoint')
   if a.circle or b.circle:
    with warnings.catch_warnings():
     warnings.simplefilter('error',IntegrationWarning);v,e=quad(lambda z:area_pair(a,b,z),x,y,epsabs=1e-13,epsrel=1e-8,limit=60)
   else:
    u=half/math.sqrt(3);u3=half*math.sqrt(3/5)
    if not x<mid-u3<mid+u3<y:raise ArithmeticError('Quadrature node rounds to event boundary')
    v=half*(area_pair(a,b,mid-u)+area_pair(a,b,mid+u));other=half*(5/9*area_pair(a,b,mid-u3)+8/9*area_pair(a,b,mid)+5/9*area_pair(a,b,mid+u3));e=abs(v-other)
   volume+=v;error+=e
   if witness is None and v>e:
    if a.circle or b.circle:circle,other=(a,b) if a.circle else (b,a);witness=circle_witness(circle,other,mid)
    else:
     cut=a.section(mid).intersection(b.section(mid))
     if cut.area>0:point=cut.representative_point();witness=[point.x,point.y,mid]
  except (ValueError,ArithmeticError,IntegrationWarning) as failure:
   bound=area_bound*(y-x);uncertain_bound+=bound;intervals.append({'z':[x,y],'possible_volume_upper_bound':bound,'reason':str(failure)})
 return {'volume':volume,'error':error+uncertain_bound,'quadrature_difference_estimate':error,'unresolved_interval_upper_bound':uncertain_bound,'unresolved_intervals':intervals,'method':'analytic_circle_polygon_quadrature' if a.circle or b.circle else 'independent_section_event_Gauss2_vs3','events':len(ev),'witness':witness}

def rawsolid(p):return Solid({'geometry':p})
def controls():
 cases=json.loads((T/'whole-model-interference-plan-r92.json').read_text(encoding='utf8'));rows=[]
 for c in cases['controls']:
  if 'a' not in c:continue
  def placed(p):
   if 'box' not in p:return p
   a,b=p['box'];return {'outline':[[a[0],a[1]],[b[0],a[1]],[b[0],b[1]],[a[0],b[1]]],'z0':a[2],'z1':b[2]}
  r=intersection(rawsolid(placed(c['a'])),rawsolid(placed(c['b'])));assert abs(r['volume']-c['expected_volume'])<1e-13,(c,r);rows.append({'id':c['id'],**r})
 c=json.loads((T/'chauffeur-bath-door-cases-r92.json').read_text(encoding='utf8'))['red_control'];column=c.get('column',c.get('column_part',c.get('column_geometry')))
 if column is None:
  column={'outline':[[14.14,20.14],[14.36,20.14],[14.36,20.36],[14.14,20.36]],'z0':.14,'z1':.60}
 results=[intersection(rawsolid(v['part']),rawsolid(column)) for v in c['bath_parts']];v=sum(r['volume'] for r in results);assert abs(v-.00021762299050879686)<1e-12,v;rows.append({'id':'real_bath_column_red','volume':v,'parts':results})
 shift=[.05201579428427681,-.05201579428427681];repaired=0.
 for value in c['bath_parts']:
  p=json.loads(json.dumps(value['part']))
  for key in ('outline','top_outline'):p[key]=[[q[0]+shift[0],q[1]+shift[1]] for q in p[key]]
  repaired+=intersection(rawsolid(p),rawsolid(column))['volume']
 assert repaired==0.;rows.append({'id':'real_bath_repaired','volume':repaired})
 fc=json.loads((T/'floor-beam-owned-cases-r92.json').read_text(encoding='utf8'));parts=fc['red_intersection_prisms'];volume=sum(shape(p['geometry']).area*(p['z'][1]-p['z'][0]) for p in parts);assert abs(volume-.4303719089181796)<1e-12;rows.append({'id':'real_floor_beam_retained_prisms','count':len(parts),'volume':volume})
 loops=[[[0,0],[3,0],[3,3],[0,3]],[[1,1],[2,1],[2,2],[1,2]]];xy=loops[0]+loops[1];vertices=[[*p,z] for z in (0.,1.) for p in xy];faces=[]
 for offset in (0,4):
  for i in range(4):a=offset+i;b=offset+(i+1)%4;faces.append([a,b,8+b,8+a])
 for i in range(4):
  j=(i+1)%4;faces.append([i,j,4+j,4+i]);faces.append([8+i,8+j,12+j,12+i])
 mesh=rawsolid({'outline':loops[0],'z0':0.,'z1':1.,'mesh':{'vertices':vertices,'faces':faces}});hole=rawsolid({'outline':[[1.2,1.2],[1.8,1.2],[1.8,1.8],[1.2,1.8]],'z0':0.,'z1':1.});assert mesh.section(.5).area==8.;r=intersection(mesh,hole);assert r['volume']==0.;rows.append({'id':'closed_nonconvex_hollow_mesh','section_area':mesh.section(.5).area,**r})
 ring=[[math.cos(i*math.tau/96),math.sin(i*math.tau/96)] for i in range(96)];circle=rawsolid({'outline':ring,'z0':0.,'z1':1.,'profile':{'shape':'circle','diameter':{'value':2.}}});quadrant=rawsolid({'outline':[[0,0],[2,0],[2,2],[0,2]],'z0':0.,'z1':1.});r=intersection(circle,quadrant);assert abs(r['volume']-math.pi/4)<1e-12;rows.append({'id':'analytic_circle_quadrant',**r})
 save('whole-interference-controls-r92.json',rows);return rows

def capture():
 root=next(p for p in T.parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
 api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');path=Path(api.__file__).with_name('🔣️.json');raw=path.read_bytes();m=json.loads(raw)
 def hashes():return {str(Path(mod.__file__).relative_to(root)):hashlib.sha256(Path(mod.__file__).read_bytes()).hexdigest() for name,mod in list(sys.modules.items()) if name.startswith('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip') and getattr(mod,'__file__',None) and Path(mod.__file__).is_file()}
 before=hashes();print('[DEBUG] canonicalbuilding capture start',flush=True);started=time.monotonic();g=api.building_geometry(m);print('[DEBUG] canonicalbuilding captured '+str(time.monotonic()-started),flush=True);objects=[]
 for key in ('pilotis','slabs','walls','screens','fillings','stairs','details','furnishings','beams','foundations','railings','services'):objects+=g.get(key,[])
 objects+=g['ramp']['ramps'];objects.append(g['curtain_wall']);seen={o['tag'] for o in objects}
 for e in m['elements']:
  if e['class']=='IfcRampFlight' and e['tag'] not in seen:objects.append(api._ramp_geometry(m,e))
  if e['class']=='IfcCurtainWall' and e['tag'] not in seen:objects.append(api._curtain_filling_geometry(m,e))
 roster={e['tag']:e for e in m['elements']};records=[];inventory=[];seen=set();lost=[]
 structural={'IfcColumn','IfcBeam','IfcSlab','IfcFooting','IfcWall'};mixed={'IfcRamp','IfcRampFlight','IfcStair'}
 for obj in objects:
  if obj['tag'] in seen:raise ValueError('Duplicate product '+obj['tag'])
  seen.add(obj['tag']);e=roster[obj['tag']];parts=obj.get('parts',[])
  inventory.append({'tag':e['tag'],'guid':e['guid'],'class':e['class'],'storey':e['storey'],'owned_parts':len(parts),'excluded_shared_parts':len(obj.get('shared_parts',[])),'excluded_child_parts':len(obj.get('child_parts',[]))})
  for i,p in enumerate(parts):
   role=p.get('role','');mat=p.get('material',e.get('material'));mat=mat.get('tag') if isinstance(mat,dict) else mat
   group='structure' if e['class'] in structural else 'fixture'
   if e['class']=='IfcCovering':group='finish'
   if e['class'] in mixed:group='finish' if any(w in role.lower() for w in ('finish','tile','wearing','paving','bedding')) or mat and any(w in mat for w in ('TILE','MORTAR','FLOOR','BITUMEN','PARQUET')) else 'structure'
   geom={k:v for k,v in p.items() if k in ('outline','holes','top_outline','section','mesh','z0','z1','profile','radius','end_plane','axis')}
   r={'tag':e['tag'],'guid':e['guid'],'class':e['class'],'storey':e['storey'],'index':i,'role':role,'material':mat,'group':group,'geometry':geom};records.append(r)
   if e['class'] in {'IfcColumn','IfcBeam','IfcSlab','IfcWindow','IfcDoor','IfcCurtainWall'} and any(k in p for k in ('holes','top_outline','section','mesh')):lost.append({**{k:r[k] for k in ('tag','index','class','role')},'discarded_keys':[k for k in ('holes','top_outline','section','mesh') if k in p]})
 missing=[{'tag':e['tag'],'class':e['class']} for e in m['elements'] if e['tag'] not in seen and e['class']!='IfcOpeningElement'];after=hashes();changed={k:[before[k],after.get(k)] for k in before if before[k]!=after.get(k)}
 out={'source':hashlib.sha256(raw).hexdigest(),'source_after':hashlib.sha256(path.read_bytes()).hexdigest(),'producers':after,'already_loaded_producer_changes':changed,'inventory':inventory,'missing_products':missing,'records':records,'wall_cutter_discarded_shape_inventory':lost,'seconds':time.monotonic()-started};save('whole-owned-capture-r92.json',out);print('[DEBUG] capture saved '+str(len(records))+' parts',flush=True);return out

def audit(data):
 started=time.monotonic();solids=[];unsupported=[]
 for r in data['records']:
  try:solids.append(Solid(r))
  except Exception as e:unsupported.append({'tag':r['tag'],'index':r['index'],'error':str(e),'group':r['group']})
 targets=[s for s in solids if s.r['group']=='structure'];sources=[s for s in solids if s.r['group']!='structure'];tree=STRtree([s.foot for s in targets]);counts=Counter();positives=[];uncertain=[];pairs=Counter();near=[]
 for n,a in enumerate(sources):
  if n%250==0:
   print('[DEBUG] interference '+str(n)+'/'+str(len(sources))+' positives '+str(len(positives))+' unresolved '+str(len(uncertain)),flush=True)
   save('whole-interference-progress-r92.json',{'completed_sources':n,'sources':len(sources),'positive_parts':len(positives),'uncertain_parts':len(uncertain),'seconds':time.monotonic()-started})
   save('whole-interference-partial-r92.json',{'source':data['source'],'completed_sources':n,'sources':len(sources),'positive_part_pairs':positives,'unresolved_part_pairs':uncertain,'unsupported_parts':unsupported})
  for k in tree.query(a.foot):
   b=targets[int(k)];counts['xy_bbox_candidates']+=1
   if a.hi<=b.lo or b.hi<=a.lo:counts['strict_z_negative']+=1;continue
   aa,bb=a.bounds,b.bounds
   if aa[2]<=bb[0] or bb[2]<=aa[0] or aa[3]<=bb[1] or bb[3]<=aa[1]:counts['strict_xy_negative']+=1;continue
   counts['refined']+=1;key=(a.r['storey'],a.r['group'],b.r['class']);pairs[key]+=1
   ids={'source':{k:a.r[k] for k in ('tag','index','material','role','class','storey')},'target':{k:b.r[k] for k in ('tag','index','material','role','class','storey')}}
   try:
    r=intersection(a,b)
    if r['volume']>max(r['error']*8,1e-14):positives.append({**ids,**r})
    elif r['volume']!=0 or r['error']>0:uncertain.append({**ids,**r,'reason':'Near-numeric volume; no silent pass'})
    else:counts['refined_zero']+=1
   except Exception as e:uncertain.append({**ids,'reason':str(e)})
 groups=defaultdict(lambda:{'part_pairs':0,'sum_pair_volume':0.,'max_error':0.,'witness':None})
 for r in positives:
  key=(r['source']['tag'],r['target']['tag']);v=groups[key];v['part_pairs']+=1;v['sum_pair_volume']+=r['volume'];v['max_error']=max(v['max_error'],r['error']);v['witness']=v['witness'] or r.get('witness')
 out={'status':'POSITIVES_OR_UNRESOLVED' if positives or uncertain or unsupported or data['missing_products'] else 'CHECKED_NO_POSITIVE','source':data['source'],'source_capture_stable':data['source']==data['source_after'] and not data['already_loaded_producer_changes'],'physical_products':len(data['inventory']),'parts':len(solids),'source_parts':len(sources),'structural_target_parts':len(targets),'coverage_counts':dict(counts),'pair_domains':[{'storey':k[0],'source_group':k[1],'target_class':k[2],'refined':v} for k,v in pairs.items()],'unsupported_parts':unsupported,'missing_products':data['missing_products'],'positive_groups':[{'source':k[0],'target':k[1],**v} for k,v in groups.items()],'positive_part_pairs':positives,'unresolved_part_pairs':uncertain,'scope':'Authored static exclusive owned solids, source fixtures/finishes vs structural/wall occupied bodies; no shared/full/child references, no global hull. Not fixture-vs-fixture or operated leaf/body route certificate.','numerical_policy':'Values <=1e-14 are retained unresolved unless exactly zero; event-sliced Gauss2 vs3 error recorded; planar-face residual >1e-8 unsupported, never healed. Circle sections analytic, adaptive quadrature error explicit.','seconds':time.monotonic()-started};save('whole-interference-result-r92.json',out);print(json.dumps({k:v for k,v in out.items() if k not in ('positive_part_pairs','unresolved_part_pairs','unsupported_parts','pair_domains','positive_groups')}),flush=True);return out
if __name__=='__main__':
 parser=argparse.ArgumentParser();parser.add_argument('mode',choices=['controls','capture','audit','all']);args=parser.parse_args()
 if args.mode in ('controls','all'):print(json.dumps(controls()),flush=True)
 if args.mode in ('capture','all'):data=capture()
 if args.mode=='audit':data=json.loads((G/'whole-owned-capture-r92.json').read_text(encoding='utf8'))
 if args.mode in ('audit','all'):audit(data)
