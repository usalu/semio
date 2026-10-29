"""🪟 Verify that catalogue reclassification preserves every physical curtain obstacle."""
import importlib,json,unittest,io
from collections import Counter
from pathlib import Path
parking=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🚗️parking');importlib.reload(parking.api);parking.api._source.cache_clear();importlib.reload(parking)
fixture=json.loads(parking.PATH.read_text(encoding='utf8'));source=parking.api.load_model();actual=parking.api.vehicle_obstacles(source,fixture['car']['z0'],fixture['car']['z0']+fixture['car']['height'])
def freeze(value):
    if isinstance(value,dict):return tuple((k,freeze(v)) for k,v in sorted(value.items()))
    if isinstance(value,list):return tuple(map(freeze,value))
    return value
def canonical(record):return freeze({**record,'kind':'curtain' if record['tag']=='CW-1' else record['kind']})
assert Counter(map(canonical,actual))==Counter(map(canonical,fixture['physical_obstacles'])),'Occupied geometry changed, not only catalogue kind'
assert all(p['kind']=='curtain' for p in actual if p['tag']=='CW-1')
assert all(p['kind']=='fillings' for p in fixture['physical_obstacles'] if p['tag']=='CW-1')
report={'source_signature':parking._signature(source,fixture['car']),'physical_obstacles':actual,'cw_obstacles':sum(p['tag']=='CW-1' for p in actual),'obstacles':len(actual),'same_occupied_multiset':True}
(Path(__file__).parent/'🗑️generated/parking-kind-r36.json').write_text(json.dumps(report),encoding='utf8')
print({k:v for k,v in report.items() if k!='physical_obstacles'})
