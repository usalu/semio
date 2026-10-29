import importlib,json,hashlib,time,traceback
from pathlib import Path
p=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES');gen=p/'🗑️generated';progress=gen/'parking-profile-progress-r62.json';events=[];start=time.monotonic()
def log(phase,**data):
 event={'phase':phase,'elapsed':time.monotonic()-start,**data};events.append(event);progress.write_text(json.dumps(event));(gen/'parking-profile-events-r62.json').write_text(json.dumps(events))
raw=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='c53b7cbe645fb7660d4662799f9e3d79c928dc91d1b3d700bfff7693f5e2e208';log('imports',sha=sha)
mod=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚗️parking');api=mod.api
import scipy.spatial as spatial
import shapely.ops as ops
original_vehicle=api._vehicle_parts;original_hull=spatial.ConvexHull;original_union=ops.unary_union
counts={'hulls':0,'unions':0};timing={'hulls':0.}
def vehicle(*args,**kwargs):
 log('vehicle_parts-start');s=time.monotonic();r=original_vehicle(*args,**kwargs);log('vehicle_parts-end',seconds=time.monotonic()-s,count=len(r));return r
def hull(*args,**kwargs):
 s=time.monotonic();r=original_hull(*args,**kwargs);timing['hulls']+=time.monotonic()-s;counts['hulls']+=1
 if counts['hulls']%10==0:log('hulls',count=counts['hulls'],seconds=timing['hulls'])
 return r
def union(geoms,*args,**kwargs):
 items=list(geoms);counts['unions']+=1;i=counts['unions'];bounds=[list(x.bounds) for x in items];log('union-start',index=i,count=len(items),bounds=bounds)
 (gen/'parking-profile-current-union-r62.json').write_text(json.dumps({'index':i,'geometries':[x.__geo_interface__ for x in items]}));s=time.monotonic();r=original_union(items,*args,**kwargs);log('union-end',index=i,count=len(items),seconds=time.monotonic()-s,area=r.area);return r
api._vehicle_parts=vehicle;spatial.ConvexHull=hull;ops.unary_union=union
try:
 case=mod.ParkingContract('test_physical_oracle_matches_native_height_intersections');case.fixture=json.loads(mod.PATH.read_text());log('oracle-start');case.test_physical_oracle_matches_native_height_intersections();log('oracle-PASS',counts=counts,timing=timing);(gen/'parking-profile-result-r62.json').write_text(json.dumps({'status':'PASS','sha':sha,'seconds':time.monotonic()-start,'counts':counts,'timing':timing}))
except Exception as error:
 log('oracle-FAIL',error=str(error),traceback=traceback.format_exc());(gen/'parking-profile-result-r62.json').write_text(json.dumps({'status':'FAIL','sha':sha,'error':str(error),'traceback':traceback.format_exc()}))
finally:
 api._vehicle_parts=original_vehicle;spatial.ConvexHull=original_hull;ops.unary_union=original_union
