"""🚶 Independently audit actual ground hall body clearance and floor support through sparse native geometry."""
import hashlib,importlib,json,sys,math,time
from pathlib import Path
from shapely.geometry import Polygon,LineString,box
from shapely.ops import unary_union
r=next(p for p in Path(__file__).resolve().parents if (p/'package.json').exists());sys.path.insert(0,str(r));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model();sp=Path(a.__file__).with_name('🔣️.json');before=sp.read_bytes();m=json.loads(before);started=time.monotonic();rings=[];floor=[];errors=[];captured=[];operated=[]
producer_paths=[q for q in Path(a.__file__).parent.rglob('*.py') if '🧪️tests' not in str(q)];producer_before={str(q.relative_to(r)):hashlib.sha256(q.read_bytes()).hexdigest() for q in producer_paths}
services={'IfcPipeSegment','IfcCovering','IfcChimney','IfcWasteTerminal','IfcBoiler','IfcSpaceHeater','IfcSanitaryTerminal','IfcLightFixture'}
for o in m['elements']:
 c=o['class'];g=o.get('geometry',{})
 if c=='IfcOpeningElement':continue
 if c in ('IfcDoor','IfcWindow','IfcCurtainWall') and o['storey'] not in ('ground','cellar'):continue
 if 'base_level' in g and a._z(m,g['base_level'])>=1.85:continue
 if c=='IfcSlab' and o['storey'] not in ('ground','cellar'):continue
 try:
  if c=='IfcColumn':v=a.column_geometry(m,o)
  elif c=='IfcWall':v=a._wall_geometry(m,o)
  elif c=='IfcSlab':v=a._slab_geometry(m,o)
  elif c=='IfcStair':v=a.stair_geometry(o['tag'],m)
  elif c in ('IfcRamp','IfcRampFlight'):v=a.ramp_geometry(o['tag'],m)
  elif c in ('IfcDoor','IfcWindow','IfcCurtainWall'):v=a.filling_geometry(o['tag'],m)
  elif 'parts' in g or g.get('kind')=='ramp_finish':v=a.solid_geometry(o['tag'],m)
  elif c in services:v=a.service_geometry(o['tag'],m)
  else:errors.append({'tag':o['tag'],'error':'unhandledphysicalclass '+c});continue
 except Exception as ex:errors.append({'tag':o['tag'],'error':str(ex)});continue
 ps=v['parts']+v.get('child_parts',[]);captured.append(o['tag'])
 if o['tag']=='D-LAUNDRY':
  leaves=[q for q in ps if 'frame' not in q.get('role','').lower()]
  verts=[v for q in leaves for v in a.part_vertices(q)];hx=max(v[0] for v in verts);hy=max(v[1] for v in verts)
  for q in leaves:
   for key in ('outline','top_outline'):
    if key in q:q[key]=[[hx+v[1]-hy,hy-v[0]+hx] for v in q[key]]
  operated.append({'tag':o['tag'],'angle_degrees':-90,'hinge':[hx,hy],'parts':len(leaves),'inferred_operation':True})
 for i,p in enumerate(ps):
  if (c=='IfcSlab' or c=='IfcCovering' and o.get('predefined_type')=='FLOORING') and abs(p['z1'])<1e-8:
   q=Polygon(p.get('top_outline',p['outline']),p.get('holes',()))
   if q.is_valid:floor.append(q)
  if p['z1']<=.05 or p['z0']>=1.85:continue
  vs=a.part_vertices(p);xx=[q[0] for q in vs];yy=[q[1] for q in vs]
  if max(xx)<6.4 or min(xx)>8.9 or max(yy)<11.5 or min(yy)>20.1:continue
  for ring in a.part_band_footprints(p,max(.05,p['z0']),min(1.85,p['z1'])):
   q=Polygon(ring)
   if not q.is_valid:errors.append({'tag':o['tag'],'part':i,'error':'invalidoccupiedbandring'});continue
   if q.area>0:rings.append((o['tag']+('|operatedleaf' if o['tag']=='D-LAUNDRY' and 'frame' not in p.get('role','').lower() else ''),q))
Path(__file__).with_name('🗑️generated').joinpath('ground-hall-final-rings-r92.json').write_text(json.dumps({'rings':[[t,list(q.exterior.coords)] for t,q in rings],'floor':[{'outline':list(q.exterior.coords),'holes':[list(h.coords) for h in q.interiors]} for q in floor],'operated':operated,'source':hashlib.sha256(before).hexdigest()}),encoding='utf8')
print('[DEBUG] Actual sparse occupied geometry captured '+str(len(captured))+' products/'+str(len(rings))+' nearby rings',flush=True)
occupied=unary_union([p for t,p in rings]);support=unary_union(floor);domain=box(7.0,11.7,8.15,19.9);start=LineString([[7,12.2],[8.15,12.2]]);end=LineString([[7,19.4],[8.15,19.4]])
def component(d):
 free=domain.difference(occupied.buffer(d/2+.001,quad_segs=64));parts=list(free.geoms) if hasattr(free,'geoms') else [free];return next((p for p in parts if p.area>0 and p.intersects(start) and p.intersects(end)),None)
def witness(d):
 p=component(d)
 if p is None:return {'diameter':d,'connected':False}
 points=[]
 for i in range(7201):
  y=12.2+i*.001;s=p.intersection(LineString([[7,y],[8.15,y]]));pieces=list(s.geoms) if hasattr(s,'geoms') else [s];intervals=[q.bounds for q in pieces if q.geom_type=='LineString' and q.length>2e-6]
  if not intervals:return {'diameter':d,'connected':True,'witness':False,'empty_slice_y':y}
  interval=min(intervals,key=lambda b:abs(max(b[0],min(7.52,b[2]))-7.52));x=max(interval[0]+1e-6,min(7.52,interval[2]-1e-6));points.append([x,y])
 line=LineString(points);body=line.buffer(d/2+.00004,quad_segs=64);hits=[{'tag':t,'area':q.intersection(body).area} for t,q in rings if q.intersects(body) and q.intersection(body).area>1e-10];missing=body.difference(support).area;return {'diameter':d,'connected':True,'witness':not hits and missing<1e-8,'points':points,'collision_area':sum(h['area'] for h in hits),'collisions':hits,'floor_missing_area':missing,'body_vertical_band':[.05,1.85],'all_door_leaves_retained':True,'operated':operated}
low,high=.5,1.1
for i in range(16):
 mid=(low+high)/2
 if component(mid) is not None:low=mid
 else:high=mid
checks=[witness(d) for d in (.8,.79,.78,.77,.76,.75,.7,.6)];old=LineString([[7.6,12.2],[7.6,19.4]]).buffer(.40004,quad_segs=64);oldhits=[{'tag':t,'area':p.intersection(old).area} for t,p in rings if p.intersects(old) and p.intersection(old).area>1e-10];after=sp.read_bytes();out={'source_before':hashlib.sha256(before).hexdigest(),'source_after':hashlib.sha256(after).hexdigest(),'source_stable':before==after,'producer_hashes':producer_before,'producers_stable':producer_before=={str(q.relative_to(r)):hashlib.sha256(q.read_bytes()).hexdigest() for q in producer_paths},'captured_products':captured,'nearby_occupied_rings':len(rings),'errors':errors,'operated':operated,'operated_leaf_projection_contacts':[{'tag':t,'area':q.intersection(unary_union([v for name,v in rings if name.endswith('|operatedleaf')])).area} for t,q in rings if not t.endswith('|operatedleaf') and q.intersection(unary_union([v for name,v in rings if name.endswith('|operatedleaf')])).area>1e-10],'largest_west_aisle_connectivity_diameter_interval':[low,high],'witnesses':checks,'old_080_straight_hits':oldhits,'seconds':time.monotonic()-started,'limits':['West-side groundhall segmentY12.2..19.4; nofixture/post displacement. Sourcepositions/sizes have original raster uncertainty.','Actual closed door leaves/frames,stairs,ramps,columns,services and fixtures retained. Witness requires both obstacle clearance and ground slab support.','Connectivity radius includes1mm margin; returned bodybuffer has0.04mm extra, polygonarcerror below0.031mm. Largest diameter is nominalmodel only.']};Path(__file__).with_name('🗑️generated').joinpath('ground-hall-final-r92.json').write_text(json.dumps(out),encoding='utf8');print(json.dumps({'source':out['source_before'],'stable':out['source_stable'],'errors':errors,'max':out['largest_west_aisle_connectivity_diameter_interval'],'checks':[{k:v for k,v in q.items() if k not in ('points','collisions')} for q in checks],'seconds':out['seconds']}),flush=True)

assert not errors,errors
assert out["source_stable"] and out["producers_stable"],"Hall physical source changed"
assert any(q.get("witness") for q in checks),"No positive supported body route"
