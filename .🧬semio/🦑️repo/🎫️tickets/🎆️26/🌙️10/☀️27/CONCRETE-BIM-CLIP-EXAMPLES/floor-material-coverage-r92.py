import importlib,json,sys,hashlib
from pathlib import Path
from shapely.geometry import Polygon,box
from shapely.ops import unary_union
r=next(p for p in Path(__file__).resolve().parents if (p/'package.json').exists());sys.path.insert(0,str(r));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model();ticket=Path(__file__).parent;data=json.loads((ticket/'🗑️generated/physical-vertical-r92.json').read_text(encoding='utf8'));caps=[];invalid=[]
for solid in data['solids']:
 for face in solid['faces']:
  pts=[solid['vertices'][i] for i in face]
  if all(abs(v[2]-3.14)<1e-7 for v in pts) and sum((pts[i][0]-pts[0][0])*(pts[i+1][1]-pts[0][1])-(pts[i][1]-pts[0][1])*(pts[i+1][0]-pts[0][0]) for i in range(1,len(pts)-1))>1e-12:
   p=Polygon([v[:2] for v in pts])
   if not p.is_valid:invalid.append({'tag':solid['tag'],'points':[v[:2] for v in pts]})
   elif p.area>1e-12:caps.append(p)
slab=unary_union(caps);rooms=unary_union([Polygon(a._expand(m,s['outline'])) for s in m['spaces'] if s['storey']=='first']);plan=next(p for p in data['plans'] if p['z']==3.5);obstacles=[]
for ring in plan['rings']:
 if ring['category'] not in ('walls','pilotis','screens','beams','stairs','ramps','railings'):continue
 p=Polygon(ring['points'])
 if p.is_valid:obstacles.append(p)
 else:invalid.append(ring)
occupied=unary_union(obstacles);unassigned=slab.difference(rooms.union(occupied));parts=list(unassigned.geoms) if hasattr(unassigned,'geoms') else [unassigned];out={'frozen_physical_source':data['source_hash'],'current_room_source':hashlib.sha256(Path(a.__file__).with_name('🔣️.json').read_bytes()).hexdigest(),'plane':3.14,'slab_cap_area':slab.area,'room_union_area':rooms.area,'unassigned_slab_cap_area':unassigned.area,'interior_unassigned_area':unassigned.intersection(box(.3,.3,18.7,21.2)).area,'regions':[{'area':p.area,'centroid':[p.centroid.x,p.centroid.y],'bounds':list(p.bounds),'outline':list(p.exterior.coords),'holes':[list(h.coords) for h in p.interiors]} for p in sorted(parts,key=lambda p:-p.area) if p.area>.01],'interior_regions':[{'area':p.area,'bounds':list(p.bounds),'centroid':[p.centroid.x,p.centroid.y],'outline':list(p.exterior.coords),'holes':[list(h.coords) for h in p.interiors]} for p in (list(unassigned.intersection(box(.3,.3,18.7,21.2)).geoms) if hasattr(unassigned.intersection(box(.3,.3,18.7,21.2)),'geoms') else [unassigned.intersection(box(.3,.3,18.7,21.2))]) if p.area>.005],'invalid_inputs':invalid,'limits':['Uses retained canonical floorcap and wall/post plane cut versus current roompolygons; not a synchronized latest wholemodel freeze.','Structural SL1cap includes exterior edge/floorzones requiring interpretation, not automatically missing internal rooms.']};(ticket/'🗑️generated/floor-material-coverage-r92.json').write_text(json.dumps(out),encoding='utf8');print(json.dumps({k:v for k,v in out.items() if k not in ('invalid_inputs','regions')}));print(json.dumps([{'area':q['area'],'centroid':q['centroid'],'bounds':q['bounds']} for q in out['regions'][:15]]));print('invalid '+str(len(invalid)))
