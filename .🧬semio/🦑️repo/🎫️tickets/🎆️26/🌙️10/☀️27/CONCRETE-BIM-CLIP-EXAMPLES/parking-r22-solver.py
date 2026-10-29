"""🅿️ Replay a bounded two-car static search against current native occupied parts."""
import importlib.util,json,time
from pathlib import Path
import numpy as np
from shapely.geometry import Polygon
from shapely.strtree import STRtree

TICKET=Path(__file__).parent
ROOT=next(p for p in TICKET.parents if (p/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🚗️parking.py').is_file())
TEST=ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🚗️parking.py'
spec=importlib.util.spec_from_file_location('parking',TEST);parking=importlib.util.module_from_spec(spec);spec.loader.exec_module(parking)
config=json.loads((TICKET/'parking-r22-input.json').read_text(encoding='utf-8'))
fixture=json.loads(TEST.with_suffix('.json').read_text(encoding='utf-8'));source=json.loads(TEST.parent.parent.joinpath('🔣️.json').read_text(encoding='utf-8'))
obstacles=parking.api.vehicle_obstacles(parking.api.load_model(),config['vehicle']['z0'],config['vehicle']['z0']+config['vehicle']['height'])
shapes=[Polygon(o['outline']) for o in obstacles];tree=STRtree(shapes)
columns=[Polygon(c['outline']) for c in parking._columns(source,config['vehicle'])]
usable=Polygon(fixture['outline']).buffer(-fixture['clearance']['boundary'])
reserves=[Polygon(r['polygon']) for r in fixture['door_access_reserves']]
first=parking._rectangle(config['fixed_pose'],config['vehicle']);start=time.monotonic();valid=[];evaluated=0;cancelled=False
for x in np.arange(config['candidate_x'][0],config['candidate_x'][1]+.001,config['candidate_x'][2]):
    if time.monotonic()-start>config['maximum_seconds'] or json.loads((TICKET/'parking-r22-input.json').read_text(encoding='utf-8')).get('cancel_requested'):
        cancelled=True
        break
    for y in np.arange(config['candidate_y'][0],config['candidate_y'][1]+.001,config['candidate_y'][2]):
        for angle in config['angles_degrees']:
            pose={'center':[float(x),float(y)],'angle_degrees':angle};car=parking._rectangle(pose,config['vehicle']);evaluated+=1
            if not usable.covers(car) or car.distance(first)<fixture['clearance']['car_pair']:continue
            if any(car.distance(c)<fixture['clearance']['column_surface']-1e-9 for c in columns):continue
            if any(car.intersection(shapes[int(i)]).area>1e-8 for i in tree.query(car)):continue
            if any(car.intersection(r).area>1e-8 for r in reserves):continue
            valid.append(pose)
report={'source_elements':len(source['elements']),'elapsed_seconds':time.monotonic()-start,'evaluated':evaluated,'cancelled':cancelled,'valid_second_poses':valid,'no_maximum_fleet_or_manoeuvre_claim':True}
(TICKET/'🗑️generated').mkdir(exist_ok=True)
(TICKET/'🗑️generated/parking-r22-search.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] parking search',report['evaluated'],len(valid),report['elapsed_seconds'])
