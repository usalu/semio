"""🚪 Validate the reviewed roof-door field patch without publishing or copying the model source."""
import hashlib,importlib,json,time,unittest
from copy import deepcopy
from pathlib import Path
from shapely.geometry import LineString,Point,Polygon
TICKET=Path(__file__).parent
CLIP=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
progress=TICKET/'🗑️generated/roof-door-candidate-progress-r49.json'
started=time.monotonic()
def phase(name):
    """⏳ Expose candidate material, native host and independent collision phases."""
    progress.write_text(json.dumps({'debug':'[DEBUG] roof door candidate','phase':name,'seconds':time.monotonic()-started}))
phase('capture source and public station axis')
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
source=CLIP/'🧱️model/🔣️.json'
raw_before=hashlib.sha256(source.read_bytes()).hexdigest()
model=json.loads(source.read_text())
patch=json.loads((TICKET/'roof-door-candidate-r49.json').read_text())
axis=LineString(api.represent('W-STAIR-ROOF','axis',model)['primitives'][0]['points'])
register=patch['approved_register']
opening=next(e for e in model['elements'] if e['tag']=='OP-STAIR-ROOF')
stations={key:axis.project(Point(register['x'],register[value])) for key,value in (('start','y_end'),('end','y_start'))}
for key,value in {**stations,'head':register['head']}.items():
    opening['attributes'][key].update(value=value,basis='inferred',source='SAB2024 author roof-plan register R49; existing CMN enclosure; +/-0.15m plan uncertainty')
next(t for t in model['types'] if t['tag']=='TY-STAIR-ROOF-DOOR').update(patch['type_fields'])
phase('native layered door and coherent finite host')
door=api.filling_geometry('D-STAIR-ROOF',model)
void=api.opening_geometry('OP-STAIR-ROOF',model)
wall=api.path_wall_geometry(model,next(e for e in model['elements'] if e['tag']=='W-STAIR-ROOF'))
phase('independent native material collision and room soffit')
maximum_overlap=0.
for leaf in door['parts']:
    a=Polygon(leaf['outline'])
    for host in wall['parts']:
        height=min(leaf['z1'],host['z1'])-max(leaf['z0'],host['z0'])
        if height>0:
            maximum_overlap=max(maximum_overlap,a.intersection(Polygon(host['outline'])).area*height)
assert maximum_overlap<1e-10,maximum_overlap
assert max(p['z1'] for p in door['parts'])==9.03
phase('portable source and synthetic geometry tests on candidate')
namespace=__import__('runpy').run_path(str(CLIP/'🧱️model/🧪️tests/🚪️roof.py'))
original_load=api.load_model
api.load_model=lambda:deepcopy(model)
try:
    class CandidateResult(unittest.TestResult):
        """🧪 Retain each candidate test and all failures beyond an observation timeout."""
        def startTest(self,test):
            super().startTest(test)
            phase('test '+str(test))
        def stopTest(self,test):
            phase('completed '+str(test))
            super().stopTest(test)
    result=CandidateResult()
    unittest.TestSuite(unittest.FunctionTestCase(value) for name,value in namespace.items() if name.startswith('test_')).run(result)
finally:
    api.load_model=original_load
if result.failures or result.errors:
    progress.write_text(json.dumps({'debug':'[DEBUG] failed candidate','failures':[(str(t),m) for t,m in result.failures],'errors':[(str(t),m) for t,m in result.errors]}))
    raise AssertionError('Candidate contracts failed; retained progress contains exact traces')
phase('write derived candidate for independent audit')
geometry=api.building_geometry(model)
dump={'raw_source':raw_before,'door':door,'opening':void,'wall':wall,'stairs':geometry['stairs'],'rails':geometry.get('rails',geometry.get('railings',[])),'roof':next(s for s in geometry['slabs'] if s['tag']=='SL-R')}
(TICKET/'🗑️generated/roof-door-candidate-geometry-r49.json').write_text(json.dumps(dump))
raw_after=hashlib.sha256(source.read_bytes()).hexdigest()
summary={'tests':result.testsRun,'failures':len(result.failures),'errors':len(result.errors),'source_before':raw_before,'source_after':raw_after,'parts':len(door['parts']),'clear_width':door['clear_width'],'clear_height':door['clear_height'],'maximum_wall_overlap':maximum_overlap,'stations':stations,'seconds':time.monotonic()-started}
progress.write_text(json.dumps(summary))
print('ROOF_DOOR_CANDIDATE',json.dumps(summary))
