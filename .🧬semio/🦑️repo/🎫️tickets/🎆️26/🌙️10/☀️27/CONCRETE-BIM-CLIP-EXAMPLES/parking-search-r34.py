"""🚗 Search static positive controls against the independently registered garage."""
import importlib,json,math,time,hashlib
from pathlib import Path
import numpy as np
from shapely.geometry import Polygon
from shapely.strtree import STRtree
parking=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚗️parking')
source=parking.api.load_model();fixture=json.loads(parking.PATH.read_text(encoding='utf8'))
fixture['outline']=next(s['outline'] for s in source['spaces'] if s['tag']==fixture['garage'])
registry={e['tag']:e for e in source['elements']+source['types']}
for reserve in fixture['door_access_reserves']:
    op=registry[reserve['opening']];wall=registry[op['geometry']['wall']];a,b=wall['geometry']['path'];length=math.dist(a,b);u=[(b[i]-a[i])/length for i in (0,1)]
    filling=next(e for e in source['elements'] if e['geometry'].get('opening')==op['tag']);w=registry[filling['type']]['filling']['frame_width']['value']
    points=[[a[i]+u[i]*station for i in (0,1)] for station in [op['attributes']['start']['value']+w,op['attributes']['end']['value']-w]]
    n=[-u[1]*reserve['depth'],u[0]*reserve['depth']]
    candidates=[points+[[p[0]+sign*n[0],p[1]+sign*n[1]] for p in points[::-1]] for sign in (1,-1)]
    reserve['polygon']=max(candidates,key=lambda ring:Polygon(ring).intersection(Polygon(fixture['outline'])).area)
fixture['physical_obstacles']=parking.api.vehicle_obstacles(source,0,1.65)
physical=[Polygon(o['outline']) for o in fixture['physical_obstacles'] if o['kind']!='column'];tree=STRtree(physical)
columns=[Polygon(o['outline']) for o in parking._columns(source,fixture['car'])];garage=Polygon(fixture['outline']).buffer(-fixture['clearance']['boundary']);reserves=[Polygon(r['polygon']) for r in fixture['door_access_reserves']]
second=parking._rectangle(fixture['verified_control']['poses'][1],fixture['car']);valid=[];evaluated=0;started=time.monotonic();cancelled=False
for x in np.arange(13.2,14.51,.1):
    if time.monotonic()-started>30 or (Path(__file__).parent/'🗑️generated/parking-r34.cancel').exists():cancelled=True;break
    for y in np.arange(9.5,11.51,.1):
        pose={'center':[round(float(x),3),round(float(y),3)],'angle_degrees':90};car=parking._rectangle(pose,fixture['car']);evaluated+=1
        if not garage.covers(car) or car.distance(second)<fixture['clearance']['car_pair']:continue
        if any(car.distance(c)<fixture['clearance']['column_surface']-1e-9 for c in columns):continue
        if any(car.intersection(physical[int(i)]).area>1e-8 for i in tree.query(car)):continue
        if any(car.intersection(r).area>1e-8 for r in reserves):continue
        valid.append(pose)
report={'source_hash':hashlib.sha256(parking.PATH.parent.parent.joinpath('🔣️.json').read_bytes()).hexdigest(),'evaluated':evaluated,'seconds':time.monotonic()-started,'cancelled':cancelled,'valid':valid,'fixture':fixture}
(Path(__file__).parent/'🗑️generated/parking-search-r34.json').write_text(json.dumps(report),encoding='utf8')
print(json.dumps({k:v for k,v in report.items() if k not in ('valid','fixture')}|{'valid':valid[:10],'count':len(valid)}))
