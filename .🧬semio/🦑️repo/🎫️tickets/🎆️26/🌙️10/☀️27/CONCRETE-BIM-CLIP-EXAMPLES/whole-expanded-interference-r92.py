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
ROOT=next(p for p in T.parents if (p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(ROOT));sys.dont_write_bytecode=True
CONTRACT=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🧾️capture')
SEMANTIC_ORACLE=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️solids.➖️difference.🧪️tests.🔬️scipy')
CAPTURE_NAME='whole-expanded-owned-capture-r92.json'
CLIP=ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip';SOURCE=CLIP/'🧱️model/🔣️.json'
def save(name,value):
 if not name.startswith('whole-expanded-'):raise ValueError('Expanded audit outputs require their explicit namespace')
 (G/name).write_text(json.dumps(value,ensure_ascii=False,indent=2,allow_nan=False),encoding='utf8')
def producer_hashes():
 suffixes={'.py','.json','.mjs','.js','.ts'};excluded={'🧪️tests','tests','__pycache__','node_modules','🗑️generated'}
 paths={p for p in CLIP.rglob('*')if p.is_file()and p.suffix in suffixes and not any(k in excluded for k in p.relative_to(CLIP).parts)}
 paths.update((Path(__file__),Path(CONTRACT.__file__),Path(SEMANTIC_ORACLE.__file__)))
 return {p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest()for p in sorted(paths)}
def loaded_hashes():
 result={}
 for module in list(sys.modules.values()):
  path=getattr(module,'__file__',None)
  if not path:continue
  file=Path(path).resolve()
  try:key=file.relative_to(ROOT).as_posix()
  except ValueError:continue
  if file.is_file()and file.suffix=='.py':result[key]=hashlib.sha256(file.read_bytes()).hexdigest()
 return result
def current_identity():return hashlib.sha256(SOURCE.read_bytes()).hexdigest(),producer_hashes()
def capture_issues(data):return CONTRACT.capture_issues(data,*current_identity())
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
def constant_prism_proof(vertices,faces):
 """📐 Certify a closed stationary two-height section using exact authored coordinates and oriented edges."""
 from fractions import Fraction
 if not vertices or any(len(p)!=3 or any(not math.isfinite(v)for v in p)for p in vertices):return False
 points=[tuple(Fraction(v)for v in p)for p in vertices];levels={p[2]for p in points}
 if len(levels)!=2:return False
 directed=Counter();caps=set()
 for face in faces:
  if len(face)<3 or any(type(i)is not int or not 0<=i<len(points)for i in face):return False
  ring=[points[i]for i in face]
  if len({p[2]for p in ring})==1:caps.add(ring[0][2])
  else:
   a=ring[0];b=next((p for p in ring[1:]if p[:2]!=a[:2]),None)
   if b is None:return False
   if any((b[0]-a[0])*(p[1]-a[1])-(b[1]-a[1])*(p[0]-a[0])!=0 for p in ring):return False
  for a,b in zip(ring,ring[1:]+ring[:1]):
   if a==b:return False
   if a[2]!=b[2]and a[:2]!=b[:2]:return False
   directed[(a,b)]+=1
 return caps==levels and all(count==1 and directed.get((b,a))==1 for (a,b),count in directed.items())

def analytic_circle_bounds(axis,radius):
 """🪐 Enclose the true dyadic axis/radius extrema with directed binary64 rounding."""
 from fractions import Fraction
 if len(axis)!=2 or not all(math.isfinite(v)for v in axis)or not math.isfinite(radius)or radius<=0:raise ValueError('Invalid analytic circular placement')
 cx,cy=(Fraction.from_float(float(v))for v in axis);r=Fraction.from_float(float(radius));exact=(cx-r,cy-r,cx+r,cy+r);out=[]
 for index,value in enumerate(exact):
  published=float(value)
  if not math.isfinite(published):raise ValueError('Analytic extent exceeds finite binary64 bounds')
  if index<2 and Fraction.from_float(published)>value:published=math.nextafter(published,-math.inf)
  if index>=2 and Fraction.from_float(published)<value:published=math.nextafter(published,math.inf)
  out.append(published)
 return tuple(out)

def _native_semantic_operand(part):
 """🧾 Preserve an original native profile frame or exact finite stock as a semantic operand."""
 if 'csg'in part:return part['csg']
 profile=part.get('profile',{});shape=profile.get('shape');complex=any(k in part for k in('mesh','section','top_outline'))
 if not complex and shape in('circle','ellipse','rectangle','polygon')and'axis'in part:
  p={'shape':shape,'voids':[{'offset':h['offset'],'radius':h['diameter']['value']/2}for h in profile.get('voids',[])]}
  if shape=='circle':p['radius']=profile['diameter']['value']/2
  elif shape=='ellipse':p['radii']=[profile[k]['value']/2 for k in('width','depth')]
  elif shape=='rectangle':p.update(width=profile['width']['value'],depth=profile['depth']['value'])
  else:p['outline']=profile['outline']
  return{'kind':'profile_extrusion','axis':part['axis'],'rotation':profile.get('rotation',0),'profile':p,'z0':part['z0'],'z1':part['z1']}
 if complex:
  vertices,faces=topology(part);return{'kind':'polyhedron','vertices':vertices,'faces':faces}
 return{'kind':'prism','outline':part['outline'],'holes':part.get('holes',[]),'z0':part['z0'],'z1':part['z1']}

def _semantic_stock(operand):
 """🪵 Retain the outer stock envelope without treating it as owned material."""
 while operand.get('operator')=='difference':operand=operand['stock']
 return operand

def _semantic_radius(profile):
 """🫧 Bound an original profile by a conservative true circle without replacing its geometry."""
 from fractions import Fraction
 q=lambda v:Fraction.from_float(float(v))
 if profile['shape']=='circle':return profile['radius']
 if profile['shape']=='ellipse':return max(profile['radii'])
 if profile['shape']=='rectangle':squared=(q(profile['width'])**2+q(profile['depth'])**2)/4
 else:squared=max(q(x)**2+q(y)**2 for x,y in profile['outline'])
 radius=math.sqrt(float(squared))
 while q(radius)**2<squared:radius=math.nextafter(radius,math.inf)
 return radius

def _semantic_envelope(operand):
 """📦 Provide only conservative support geometry, with analytic profile identity retained separately."""
 stock=_semantic_stock(operand)
 if stock['kind']=='prism':return{k:stock[k]for k in('outline','holes','z0','z1')}
 if stock['kind']=='polyhedron':return{'mesh':{'vertices':stock['vertices'],'faces':stock['faces']}}
 radius=_semantic_radius(stock['profile']);x,y=stock['axis'];return{'axis':[x,y],'outline':[[x+radius,y],[x,y+radius],[x-radius,y],[x,y-radius]],'profile':{'shape':'circle','diameter':{'value':2*radius}},'z0':stock['z0'],'z1':stock['z1']}

def _semantic_key(operand,heights=True):
 """🔐 Compare geometry alone; ownership identifiers cannot certify physical exclusion."""
 if operand.get('operator')=='difference':return('difference',_semantic_key(operand['stock'],heights),tuple(_semantic_key(c['operand'],heights)for c in operand['cutters']))
 def freeze(value):
  if isinstance(value,dict):return tuple((key,freeze(item))for key,item in sorted(value.items()))
  if isinstance(value,(list,tuple)):return tuple(freeze(item)for item in value)
  if isinstance(value,(int,float))and not isinstance(value,bool):return('binary64',float(value).hex())
  return value
 return freeze({k:v for k,v in operand.items()if heights or k not in('z0','z1')})

def _semantic_contains(query,container,domain,band):
 """🧠 Certify sufficient set containment within the actual common height interval."""
 from fractions import Fraction
 if _semantic_key(query)==_semantic_key(container):return True
 if query.get('operator')=='difference'and _semantic_contains(query['stock'],container,domain,band):return True
 if container.get('operator')=='difference':
  if len(container['cutters'])==1:
   inner=container['cutters'][0]['operand']
   if inner.get('operator')=='difference'and len(inner['cutters'])==1 and _semantic_key(container['stock'])==_semantic_key(inner['stock']):
    return _semantic_contains(domain,container['stock'],domain,band)and _semantic_contains(query,inner['cutters'][0]['operand'],domain,band)
  return False
 if container['kind']=='polyhedron':return False
 if not(container['z0']<=band[0]and band[1]<=container['z1']):return False
 if query.get('operator')!='difference'and query['kind']!='polyhedron'and _semantic_key(query,False)==_semantic_key(container,False):return True
 if container['kind']!='profile_extrusion'or container['profile']['shape']!='circle'or container['profile']['voids']:return False
 q=lambda v:Fraction.from_float(float(v));cx,cy=(q(v)for v in container['axis']);radius=q(container['profile']['radius'])
 if query.get('operator')=='difference':return False
 if query['kind']=='profile_extrusion':
  r=q(_semantic_radius(query['profile']));dx,dy=(q(query['axis'][k])-v for k,v in enumerate((cx,cy)));return radius>=r and dx*dx+dy*dy<=(radius-r)**2
 points=query['vertices']if query['kind']=='polyhedron'else query['outline'];return all((q(p[0])-cx)**2+(q(p[1])-cy)**2<=radius**2 for p in points)

def _semantic_exclusion(left,right,band):
 """🛡️ Prove actual Boolean material exclusion without a tag or GUID waiver."""
 def side(a,b):
  if a.get('operator')!='difference':return None
  for cutter in a['cutters']:
   if _semantic_contains(b,cutter['operand'],a['stock'],band):return{'rule':'actual_geometric_cutter_contains_query','geometry_sha256':hashlib.sha256(repr(_semantic_key(cutter['operand'])).encode()).hexdigest(),'height':list(band)}
  return side(a['stock'],b)
 return side(left,right)or side(right,left)

@lru_cache(maxsize=512)
def _validated_semantic(key):
 """🧬 Reject non-finite or malformed operands before any geometric set certificate."""
 from jsonschema import Draft202012Validator
 from jsonschema.exceptions import ValidationError
 node=json.loads(key)
 def finite(value):
  if isinstance(value,dict):return all(finite(v)for v in value.values())
  if isinstance(value,list):return all(finite(v)for v in value)
  return not isinstance(value,(int,float))or math.isfinite(value)
 if not finite(node):raise ValueError('Semantic geometry requires finite numeric authority')
 schema=json.loads(Path(SEMANTIC_ORACLE.__file__).parents[1].joinpath('🧬️.schema.json').read_text(encoding='utf8'));schema={'$defs':schema['$defs'],'$ref':'#/$defs/operand'}
 try:Draft202012Validator(schema).validate(node)
 except ValidationError as error:raise ValueError('Invalid semantic operand: '+error.message)from error
 for leaf in SEMANTIC_ORACLE._leaves(node):
  low,high=SEMANTIC_ORACLE._z_extent(leaf)
  if not low<high:raise ValueError('Semantic extrusion needs positive finite height')
  if leaf['kind']=='polyhedron':SEMANTIC_ORACLE._polyhedron(json.dumps(leaf,sort_keys=True))
  elif leaf['kind']=='prism':SEMANTIC_ORACLE._polygon(json.dumps(leaf,sort_keys=True))
  elif leaf['profile']['shape']in('rectangle','polygon'):SEMANTIC_ORACLE._profile_polygon(json.dumps(leaf,sort_keys=True))

def _semantic_validate(operand):
 """🔐 Bind cached independent validity to the complete unchanged geometry expression."""
 _validated_semantic(json.dumps(operand,sort_keys=True,allow_nan=False))

def _semantic_result(a,b,lo,hi,cancel=None,progress=None):
 """⚖️ Use exact geometric exclusion or independent real analytic sections, retaining numerical zero uncertainty."""
 left=a.semantic if a.semantic is not None else _native_semantic_operand(a.p);right=b.semantic if b.semantic is not None else _native_semantic_operand(b.p);_semantic_validate(left);_semantic_validate(right);proof=_semantic_exclusion(left,right,(lo,hi))
 if proof:return{'volume':0.,'error':0.,'method':'exact_semantic_exclusion','proof':proof}
 value,error=SEMANTIC_ORACLE.intersection_volume(left,right,cancel=cancel,progress=progress)
 if value<0 or error<0:raise ValueError('Negative independent semantic measure')
 bound=max(0.,min(a.bounds[2],b.bounds[2])-max(a.bounds[0],b.bounds[0]))*max(0.,min(a.bounds[3],b.bounds[3])-max(a.bounds[1],b.bounds[1]))*(hi-lo)
 if value==0:
  if bound<=0:raise ArithmeticError('Unproved semantic zero has no representable positive uncertainty bound')
  error=max(error,bound)
 elif value>bound:error=max(error,value+bound)
 return{'volume':value,'error':error,'method':'independent_real_semantic_sections','error_kind':'estimated_absolute_error_plus_unproved_zero_envelope','witness':None}

class Solid:
 def __init__(self,record):
  self.r=record;self.p=record['geometry'];p=self.p;self.semantic=p.get('csg')
  if self.semantic is not None and any(k in p for k in('outline','mesh','section','top_outline','holes','z0','z1','radius','profile','display_mesh')):raise ValueError('Semantic solid cannot also declare physical tessellation authority')
  if self.semantic is None and not any(k in p for k in('mesh','section','top_outline'))and'axis'in p and p.get('profile',{}).get('shape')in('ellipse','rectangle','polygon'):self.semantic=_native_semantic_operand(p)
  if self.semantic is not None:
   _semantic_validate(self.semantic);self._support_envelope=Solid({'geometry':_semantic_envelope(self.semantic)})
   for key in('v','faces','edges','planes','lo','hi','bounds','foot'):setattr(self,key,getattr(self._support_envelope,key))
   self.poly=None;self.circle=False;self.complex=True;self.prism_proof=None;return
  self.complex=any(k in p for k in ('mesh','section','top_outline'));self.circle=p.get('profile',{}).get('shape')=='circle' and not self.complex
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
   if not isinstance(p.get('axis'),(list,tuple))or len(p['axis'])!=2 or not all(math.isfinite(v)for v in p['axis']):raise ValueError('Analytic native profiles require their explicit source axis')
   self.c=np.asarray(p['axis'],dtype=float);self.radius=p['profile']['diameter']['value']/2
   self.bounds=analytic_circle_bounds(self.c,self.radius);self.foot=box(*self.bounds)
  self.prism_proof=None
  if self.poly is None and not self.circle and constant_prism_proof(self.v.tolist(),self.faces):
   from fractions import Fraction
   mid=float((Fraction(self.lo)+Fraction(self.hi))/2)
   if self.lo<mid<self.hi:
    polygon=self.section(mid)
    if polygon.is_valid and not polygon.is_empty:self.poly=polygon;self.prism_proof="closed_2Z_vertical_stationary_edges_exact"
 @lru_cache(maxsize=48)
 def section(self,z):
  if self.semantic is not None:raise ValueError('Semantic geometry requires real analytic sections, never its support envelope')
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

def _exact_support_axis(values):
 """🧭 Canonicalize an exact direction without changing its support ordering."""
 if not any(values):return None
 if all(isinstance(v,int)for v in values):axis=tuple(values)
 else:
  from fractions import Fraction
  values=tuple(Fraction(v)for v in values);denominator=math.lcm(*(v.denominator for v in values));axis=tuple(v.numerator*(denominator//v.denominator)for v in values)
 divisor=math.gcd(*axis);axis=tuple(v//divisor for v in axis);return tuple(-v for v in axis)if next(v for v in axis if v)<0 else axis

def _exact_support_cross(a,b):
 """✖️ Preserve the actual binary64 edge and face products as exact rational directions."""
 return(a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])

def _exact_support_data(s):
 """🧱 Retain each binary64 vertex on one exact integer lattice, with every original face and edge direction."""
 if hasattr(s,'_support_data'):return s._support_data
 ratios=tuple(tuple(float(x).as_integer_ratio()for x in p)for p in s.v);denominator=max(d for p in ratios for n,d in p);points=tuple(tuple(n*(denominator//d)for n,d in p)for p in ratios);directions=set();normals=set()
 for i,j in s.edges:
  axis=_exact_support_axis(tuple(points[j][k]-points[i][k]for k in range(3)))
  if axis:directions.add(axis)
 for face in s.faces:
  origin=points[face[0]]
  for i in range(1,len(face)-1):
   a=tuple(points[face[i]][k]-origin[k]for k in range(3));b=tuple(points[face[i+1]][k]-origin[k]for k in range(3));axis=_exact_support_axis(_exact_support_cross(a,b))
   if axis:normals.add(axis)
 s._support_data=(points,tuple(sorted(normals)),tuple(sorted(directions)),denominator);return s._support_data

def exact_support_separation(a,b):
 """↔️ Certify only exact support negatives, retaining every unproved concave, hollow or circular pair."""
 from fractions import Fraction
 a=getattr(a,'_support_envelope',a);b=getattr(b,'_support_envelope',b)
 if a.hi<=b.lo or b.hi<=a.lo:return{'kind':'exact_support','axis':['0','0','1'],'rule':'height_interval'}
 def cylinder(s):
  """🪈 Resolve the authoritative analytic axis without a centroid or display-ring estimate."""
  axis=s.p.get('axis');diameter=s.p.get('profile',{}).get('diameter',{}).get('value')
  if not isinstance(axis,(list,tuple))or len(axis)!=2 or not all(math.isfinite(v)for v in axis)or not isinstance(diameter,(int,float))or not math.isfinite(diameter)or diameter<=0:return None
  return tuple(Fraction.from_float(float(v))for v in axis),Fraction.from_float(float(diameter))/2
 circles=[cylinder(s)if s.circle else None for s in(a,b)]
 if any(s.circle and c is None for s,c in zip((a,b),circles)):return None
 if a.circle and b.circle:
  (ca,ra),(cb,rb)=circles;distance=sum((ca[k]-cb[k])**2 for k in range(2))
  return{'kind':'exact_support','rule':'analytic_outer_disks_squared'}if distance>=(ra+rb)**2 else None
 data=[None if s.circle else _exact_support_data(s)for s in(a,b)];axes={(1,0,0),(0,1,0),(0,0,1)}
 for value in data:
  if value is not None:axes.update(value[1])
 def interval(value,axis):
  """📐 Bound every point of a finite material body by its exact vertex supports."""
  values=[sum(v[k]*axis[k]for k in range(3))for v in value[0]];return min(values),max(values)
 def check(axis):
  """🔬 Compare finite intervals or square a nonnegative analytic radial support gap exactly."""
  if not a.circle and not b.circle:
   alo,ahi=interval(data[0],axis);blo,bhi=interval(data[1],axis);return ahi*data[1][3]<=blo*data[0][3]or bhi*data[0][3]<=alo*data[1][3]
  index=0 if a.circle else 1;s=(a,b)[index];(center,radius)=circles[index];value=data[1-index];low,high=interval(value,axis);low,high=Fraction(low,value[3]),Fraction(high,value[3]);base=sum(center[k]*axis[k]for k in range(2));heights=[axis[2]*Fraction.from_float(float(z))for z in(s.lo,s.hi)];radial=radius**2*(axis[0]**2+axis[1]**2);gaps=(low-base-max(heights),base+min(heights)-high);return any(gap>=0 and gap**2>=radial for gap in gaps)
 seen=set()
 for axis in sorted(axes):
  seen.add(axis)
  if check(axis):return{'kind':'exact_support','axis':[str(v)for v in axis],'rule':'analytic_radial_squared'if a.circle or b.circle else'vertex_support_interval'}
 if not a.circle and not b.circle:
  for left in data[0][2]:
   for right in data[1][2]:
    axis=_exact_support_axis(_exact_support_cross(left,right))
    if axis and axis not in seen:
     seen.add(axis)
     if check(axis):return{'kind':'exact_support','axis':[str(v)for v in axis],'rule':'edge_cross_support_interval'}
 return None

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

def disk_pair_area(a,b):
 d=float(np.linalg.norm(a.c-b.c));r,s=a.radius,b.radius
 if d>=r+s:return 0.
 if d<=abs(r-s):return math.pi*min(r,s)**2
 u=max(-1.,min(1.,(d*d+r*r-s*s)/(2*d*r)));v=max(-1.,min(1.,(d*d+s*s-r*r)/(2*d*s)))
 return r*r*math.acos(u)+s*s*math.acos(v)-.5*math.sqrt(max(0.,(-d+r+s)*(d+r-s)*(d-r+s)*(d+r+s)))

def analytic_result(a,b,volume,lo,hi,method,witness=None):
 """🔎 Retain a numerical zero without an exact negative certificate as unresolved analytic material."""
 width=max(0.,min(a.bounds[2],b.bounds[2])-max(a.bounds[0],b.bounds[0]));depth=max(0.,min(a.bounds[3],b.bounds[3])-max(a.bounds[1],b.bounds[1]));bound=width*depth*(hi-lo);error=0.;reason=None
 if volume==0 and bound>0:error=bound;reason='Analytic numerical zero has no exact separation certificate'
 elif volume>bound:error=volume+bound;reason='Analytic numerical volume exceeds its conservative bounding volume'
 return{'volume':volume,'error':error,'method':method,'witness':witness,**({'reason':reason,'possible_volume_upper_bound':bound}if reason else{})}

def intersection(a,b,cancel=None,progress=None):
 if cancel and cancel():raise InterruptedError('Whole-model intersection cancelled')
 lo,hi=max(a.lo,b.lo),min(a.hi,b.hi)
 if hi<=lo:return {'volume':0.,'error':0.,'method':'strict_Z_negative'}
 if a.semantic is not None or b.semantic is not None:return _semantic_result(a,b,lo,hi,cancel=cancel,progress=progress)
 if a.poly is None or b.poly is None:
  proof=exact_support_separation(a,b)
  if proof:return {'volume':0.,'error':0.,'method':'exact_binary64_support_negative','proof':proof}
 if a.circle and b.circle:
  if a.p['profile'].get('voids') or b.p['profile'].get('voids'):raise ValueError('Analytic circle/circle with finite voids requires independent disk set integration')
  return analytic_result(a,b,disk_pair_area(a,b)*(hi-lo),lo,hi,'analytic_disk_lens_exact_height')
 if (a.circle and b.poly is not None) or (b.circle and a.poly is not None):
  v=area_pair(a,b,(lo+hi)/2)*(hi-lo);circle,other=(a,b) if a.circle else (b,a);return analytic_result(a,b,v,lo,hi,'analytic_circle_prism_exact_height',circle_witness(circle,other,(lo+hi)/2)if v else None)
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
 ring=[[math.cos(i*math.tau/96),math.sin(i*math.tau/96)] for i in range(96)];circle=rawsolid({'axis':[0.,0.],'outline':ring,'z0':0.,'z1':1.,'profile':{'shape':'circle','diameter':{'value':2.}}});quadrant=rawsolid({'outline':[[0,0],[2,0],[2,2],[0,2]],'z0':0.,'z1':1.});r=intersection(circle,quadrant);assert abs(r['volume']-math.pi/4)<1e-12;rows.append({'id':'analytic_circle_quadrant',**r})
 r=intersection(circle,circle);assert abs(r['volume']-math.pi)<1e-12;rows.append({'id':'analytic_identical_disks',**r})
 shifted=rawsolid({'axis':[2.,0.],'outline':[[p[0]+2,p[1]] for p in ring],'z0':0.,'z1':1.,'profile':{'shape':'circle','diameter':{'value':2.}}});r=intersection(circle,shifted);assert r['volume']<1e-14;rows.append({'id':'analytic_tangent_disks',**r})
 save('whole-expanded-interference-controls-r92.json',rows);return rows

def capture():
 before=producer_hashes();loaded_before=loaded_hashes();raw=SOURCE.read_bytes();m=json.loads(raw)
 api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');path=SOURCE
 print('[DEBUG] canonicalbuilding capture start',flush=True);started=time.monotonic();g=api.building_geometry(m);print('[DEBUG] canonicalbuilding captured '+str(time.monotonic()-started),flush=True);objects=[]
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
   geom={k:v for k,v in p.items() if k in ('outline','holes','top_outline','section','mesh','z0','z1','profile','radius','end_plane','axis','csg')}
   r={'tag':e['tag'],'guid':e['guid'],'class':e['class'],'storey':e['storey'],'index':i,'role':role,'material':mat,'group':group,'geometry':geom};records.append(r)
   if e['class'] in {'IfcColumn','IfcBeam','IfcSlab','IfcWindow','IfcDoor','IfcCurtainWall'} and any(k in p for k in ('holes','top_outline','section','mesh')):lost.append({**{k:r[k] for k in ('tag','index','class','role')},'preserved_shape_keys':[k for k in ('holes','top_outline','section','mesh') if k in p]})
 missing=[{'tag':e['tag'],'class':e['class']} for e in m['elements'] if e['tag'] not in seen and e['class']!='IfcOpeningElement'];after=producer_hashes();loaded_after=loaded_hashes();changed={k:[v,loaded_after.get(k)]for k,v in loaded_before.items()if v!=loaded_after.get(k)};untracked=sorted(set(loaded_after)-set(before));snapshot=hashlib.sha256(raw).hexdigest();source_after=hashlib.sha256(path.read_bytes()).hexdigest()
 out={'capture_kind':CONTRACT.CAPTURE_KIND,'source':snapshot,'source_after':source_after,'producers_before':before,'producers_after':after,'loaded_producers_before':loaded_before,'loaded_producers_after':loaded_after,'loaded_producer_changes':changed,'untracked_loaded_producers':untracked,'capture_stable':snapshot==source_after and snapshot==before.get(SOURCE.relative_to(ROOT).as_posix())and before==after and not changed and not untracked,'inventory':inventory,'missing_products':missing,'records':records,'native_shape_inventory':lost,'seconds':time.monotonic()-started};save(CAPTURE_NAME,out);print('[DEBUG] capture saved '+str(len(records))+' parts',flush=True);return out

def audit(data,diagnostic=False):
 if data.get('capture_kind')!=CONTRACT.CAPTURE_KIND:raise ValueError('A fresh expanded capture with complete v2 provenance is required')
 identity_before=current_identity();freshness_before=CONTRACT.capture_issues(data,*identity_before)
 if freshness_before and not diagnostic:raise ValueError('Expanded capture is stale or incomplete: '+','.join(freshness_before))
 started=time.monotonic();solids=[];unsupported=[]
 for r in data['records']:
  try:solids.append(Solid(r))
  except Exception as e:unsupported.append({'tag':r['tag'],'index':r['index'],'error':str(e),'group':r['group']})
 targets=solids;sources=solids;tree=STRtree([s.foot for s in targets]);counts=Counter();positives=[];uncertain=[];pairs=Counter();near=[]
 cancel=G/'whole-expanded-cancel-r92'
 cancel_state=[0.,False];progress_state=[0.]
 def cancelled():
  now=time.monotonic()
  if now-cancel_state[0]>=.1:cancel_state[:]=[now,cancel.exists()]
  return cancel_state[1]
 def pair_progress(event):
  now=time.monotonic()
  if now-progress_state[0]>=1:
   progress_state[0]=now;save('whole-expanded-semantic-progress-r95.json',{'source':data['source'],'source_index':n,'sources':len(sources),'pair':ids,'event':event,'seconds':now-started})
 for n,a in enumerate(sources):
  if cancel.exists():
   save('whole-expanded-interference-cancelled-r92.json',{'completed_sources':n,'source':data['source'],'reason':'cooperative audit cancellation; partial results retained'});return None
  if n%250==0:
   print('[DEBUG] interference '+str(n)+'/'+str(len(sources))+' positives '+str(len(positives))+' unresolved '+str(len(uncertain)),flush=True)
   save('whole-expanded-interference-progress-r92.json',{'completed_sources':n,'sources':len(sources),'positive_parts':len(positives),'uncertain_parts':len(uncertain),'seconds':time.monotonic()-started})
   save('whole-expanded-interference-partial-r92.json',{'source':data['source'],'completed_sources':n,'sources':len(sources),'positive_part_pairs':positives,'unresolved_part_pairs':uncertain,'unsupported_parts':unsupported})
  for k in tree.query(a.foot):
   b=targets[int(k)]
   if int(k)<=n:continue
   if a.r['tag']==b.r['tag']:
    counts['same_product_component_pairs_excluded']+=1;continue
   counts['xy_bbox_candidates']+=1
   if a.hi<=b.lo or b.hi<=a.lo:counts['strict_z_negative']+=1;continue
   aa,bb=a.bounds,b.bounds
   if aa[2]<=bb[0] or bb[2]<=aa[0] or aa[3]<=bb[1] or bb[3]<=aa[1]:counts['strict_xy_negative']+=1;continue
   counts['refined']+=1;key=(a.r['storey'],a.r['group'],b.r['class']);pairs[key]+=1
   ids={'source':{k:a.r[k] for k in ('tag','index','material','role','class','storey')},'target':{k:b.r[k] for k in ('tag','index','material','role','class','storey')}}
   try:
    r=intersection(a,b,cancel=cancelled,progress=pair_progress)
    if r['volume']>max(r['error']*8,1e-14):positives.append({**ids,**r})
    elif r['volume']!=0 or r['error']>0:uncertain.append({**ids,**r,'reason':'Near-numeric volume; no silent pass'})
    else:
     counts['refined_zero']+=1
     if r.get('method')=='exact_binary64_support_negative':counts['exact_support_negative']+=1
   except InterruptedError:
    save('whole-expanded-interference-cancelled-r92.json',{'completed_sources':n,'source':data['source'],'pair':ids,'reason':'cooperative cancellation during actual semantic integration; partial results retained'});save('whole-expanded-interference-partial-r92.json',{'source':data['source'],'completed_sources':n,'sources':len(sources),'positive_part_pairs':positives,'unresolved_part_pairs':uncertain,'unsupported_parts':unsupported});return None
   except Exception as e:uncertain.append({**ids,'reason':str(e)})
 groups=defaultdict(lambda:{'part_pairs':0,'sum_pair_volume':0.,'max_error':0.,'witness':None})
 for r in positives:
  key=(r['source']['tag'],r['target']['tag']);v=groups[key];v['part_pairs']+=1;v['sum_pair_volume']+=r['volume'];v['max_error']=max(v['max_error'],r['error']);v['witness']=v['witness'] or r.get('witness')
 out={'status':'POSITIVES_OR_UNRESOLVED' if positives or uncertain or unsupported or data['missing_products'] else 'CHECKED_NO_POSITIVE','source':data['source'],'source_capture_stable':data['capture_stable'],'physical_products':len(data['inventory']),'parts':len(solids),'source_parts':len(sources),'structural_target_parts':len(targets),'coverage_counts':dict(counts),'pair_domains':[{'storey':k[0],'source_group':k[1],'target_class':k[2],'refined':v} for k,v in pairs.items()],'unsupported_parts':unsupported,'missing_products':data['missing_products'],'positive_groups':[{'source':k[0],'target':k[1],**v} for k,v in groups.items()],'positive_part_pairs':positives,'unresolved_part_pairs':uncertain,'scope':'All distinct-product authored static exclusive owned solids: fixture/fixture, structure/structure, finish/finish and cross-domain pairs. Only same-product component pairs and duplicate ordering excluded; no tag-based interface waivers. Shared/full/child references excluded at capture, no global hull. Analytic circle/circle combinations not yet supported remain unresolved, not polygonized or passed. Not an operated leaf/body-route certificate.','numerical_policy':'Values <=1e-14 are retained unresolved unless exactly zero; event-sliced Gauss2 vs3 error recorded; exact support negatives use native face/edge axes and analytic cylinder supports; missing analytic source axes are unsupported; planar-face residual >1e-8 unsupported, never healed. Circle sections analytic, adaptive quadrature error explicit.','seconds':time.monotonic()-started};identity_after=current_identity();freshness_after=CONTRACT.capture_issues(data,*identity_after);out.update(capture_kind=data['capture_kind'],capture_producers_before=data['producers_before'],capture_producers_after=data['producers_after'],audit_source_before=identity_before[0],audit_source_after=identity_after[0],audit_producers_before=identity_before[1],audit_producers_after=identity_after[1],freshness_issues_before=freshness_before,freshness_issues_after=freshness_after)
 if freshness_before or freshness_after:out['status']='DIAGNOSTIC_STALE_OR_MOVING_CAPTURE'
 save('whole-expanded-interference-result-r92.json',out);print(json.dumps({k:v for k,v in out.items() if k not in ('positive_part_pairs','unresolved_part_pairs','unsupported_parts','pair_domains','positive_groups')}),flush=True);return out
if __name__=='__main__':
 parser=argparse.ArgumentParser();parser.add_argument('mode',choices=['controls','capture','audit','all']);parser.add_argument('--diagnostic',action='store_true');args=parser.parse_args()
 if args.mode in ('controls','all'):print(json.dumps(controls()),flush=True)
 if args.mode in ('capture','all'):data=capture()
 if args.mode=='audit':data=json.loads((G/CAPTURE_NAME).read_text(encoding='utf8'))
 if args.mode in ('audit','all'):audit(data,diagnostic=args.diagnostic)
                                                                                   