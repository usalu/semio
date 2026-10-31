"""🚶 Test a source-qualified thinner service partition without altering fixed fixtures or publishing source."""
import hashlib,importlib,json,sys,time,copy
from pathlib import Path
from shapely.geometry import Polygon,LineString,box
from shapely.ops import unary_union
from shapely import constrained_delaunay_triangles
from scipy.spatial import ConvexHull
import numpy as np
from fractions import Fraction
R=Path.cwd();P=Path(__file__).parent;O=P/'🗑️generated';sys.path.insert(0,str(R));A=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
source=Path(A.__file__).with_name('🔣️.json');before=source.read_bytes();m=json.loads(before);started=time.monotonic()
cache=json.loads((O/'ground-hall-final-rings-r92.json').read_text(encoding='utf8'));assert cache['source']==hashlib.sha256(before).hexdigest(),'Fresh physical capture required'
original=copy.deepcopy(A._find(m,'W-G-SERVICE'));wall=A._find(m,'W-G-SERVICE');assert original['geometry']['path']==[[7.05,11.9],[7.05,18.2]]
assert A._find(m,'LS-PARTITION-08')['layers'][1]['thickness']['value']==.06
wall['material']='LS-PARTITION-08';wall['type']='TY-PARTITION-08';wall['attributes']['thickness']={'value':.08,'unit':'m','basis':'inferred','source':'SAB2024-p45-registered-service-partition;p96-thin-partition-family'}
wall['geometry']['path']=[[7.015,y] for _,y in original['geometry']['path']]
assert Fraction('7.015')-Fraction('.08')/2==Fraction('7.05')-Fraction('.15')/2
changed={'W-G-SERVICE':A._wall_geometry(m,wall)}
for tag in ('D-SERVANT-1','D-SERVANT-2'):changed[tag]=A.filling_geometry(tag,m)
print('[DEBUG] Candidate physical wall and both full servant doors resolved',flush=True)
stock=copy.deepcopy(A._find(m,'SKIRT-W-G-SERVICE-ground'));moved=0
for part in stock['geometry']['parts']:
 vertices=part['mesh']['vertices']
 if min(v[0] for v in vertices)>=7.125-1e-9:
  for v in vertices:v[0]-=.07
  moved+=1
changed[stock['tag']]=A._solid_geometry(m,stock)
assert moved>0
rings=[(tag,Polygon(points)) for tag,points in cache['rings'] if tag.split('|')[0] not in changed]
for tag,body in changed.items():
 for part in body['parts']:
  if part['z1']<=.05 or part['z0']>=1.85:continue
  rings.extend((tag,Polygon(ring)) for ring in A.part_band_footprints(part,max(.05,part['z0']),min(1.85,part['z1'])))
old=unary_union([Polygon(points) for tag,points in cache['rings'] if tag=='W-G-SERVICE']);new=unary_union([shape for tag,shape in rings if tag=='W-G-SERVICE'])
support=unary_union([Polygon(row['outline'],row['holes']) for row in cache['floor']]);exposed=old.difference(new).difference(support)
patches=[poly for poly in constrained_delaunay_triangles(exposed).geoms if exposed.covers(poly)] if not exposed.is_empty else []
assert abs(sum(poly.area for poly in patches)-exposed.area)<1e-9
caps=unary_union([Polygon(ring) for tag in ('SL-GROUND','SL-GROUND-CELLAR') for part in A._slab_geometry(m,A._find(m,tag))['full_parts'] if abs(part['z1']+.04)<1e-9 for ring in A.part_regions(part,part['z1'])])
assert exposed.difference(caps).area<1e-8,('Unsupported newly exposed hall flooring',exposed.difference(caps).area)
floor_delta=[]
for poly in patches:
 outline=[list(v) for v in poly.exterior.coords[:-1]]
 for z0,z1,role,material in ((-.04,-.015,'bedding','MAT-MORTAR-PUTTY'),(-.015,0.,'wearing','MAT-GRES-WHITE')):
  floor_delta.append({'outline':outline,'z0':z0,'z1':z1,'role':role,'material':material})
support=support.union(exposed)
volumes=[]
for tag,body in changed.items():
 for part in body['parts']:
  points=np.asarray(A.part_vertices(part),float);span=np.ptp(points,axis=0);assert np.all(span>0)
  native=A._polyhedron_volume(part);independent=ConvexHull((points-points.min(axis=0))/span).volume*float(np.prod(span));assert native>0 and abs(native-independent)<=max(1e-24,independent*1e-8)
  volumes.append([tag,native,independent])
occupied=unary_union([shape for _,shape in rings]);domain=box(7,11.7,8.15,19.9);free=domain.difference(occupied.buffer(.77/2+.001,quad_segs=64));components=list(free.geoms) if hasattr(free,'geoms') else [free];start=LineString([[7,12.2],[8.15,12.2]]);end=LineString([[7,19.4],[8.15,19.4]])
component=next((poly for poly in components if poly.area>0 and poly.intersects(start) and poly.intersects(end)),None);assert component is not None,'Existing .77m body connectivity must recover'
points=[]
for i in range(7201):
 y=12.2+i*.001;section=component.intersection(LineString([[7,y],[8.15,y]]));pieces=list(section.geoms) if hasattr(section,'geoms') else [section];intervals=[line.bounds for line in pieces if line.geom_type=='LineString' and line.length>2e-6];assert intervals
 interval=min(intervals,key=lambda b:abs(max(b[0],min(7.52,b[2]))-7.52));points.append([max(interval[0]+1e-6,min(7.52,interval[2]-1e-6)),y])
body=LineString(points).buffer(.77/2+.00004,quad_segs=64);hits=[{'tag':tag,'area':shape.intersection(body).area} for tag,shape in rings if shape.intersection(body).area>1e-10];missing=body.difference(support).area
report={'success':not hits and missing<1e-8,'candidate_only':True,'publishable':False,'source_before':hashlib.sha256(before).hexdigest(),'source_after':hashlib.sha256(source.read_bytes()).hexdigest(),'source_stable':before==source.read_bytes(),'wall_delta':wall,'floor_parts_to_add':floor_delta,'new_floor_area':exposed.area,'moved_proud_stock_cells':moved,'stock_candidate':stock,'volume_cells':volumes,'body_diameter':.77,'body_band':[.05,1.85],'route':points,'collisions':hits,'floor_missing_area':missing,'seconds':time.monotonic()-started,'limits':['Unpublished read-only feasibility candidate. Fixed basin/post axes and every original door panel retained.','Cabinet-side finishX6.975 preserved;8cmtype is existing inferred thin-partition family, not a measured historic dimension.','This intermediate stock moves complete east-facing cells only; all-host contour/joint regeneration, source schema/contact and bilingual native SDK proof are still required before source publication.']}
(O/'native-hall-candidate-r95.json').write_text(json.dumps(report,ensure_ascii=False),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k not in ('wall_delta','stock_candidate','floor_parts_to_add','volume_cells','route')}),flush=True);assert report['success'] and report['source_stable']
