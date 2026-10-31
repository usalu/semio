"""🔴 Record the unchanged-source RED against the schema-first qualified hall reconstruction contract."""
from pathlib import Path
import importlib,json,sys,hashlib,traceback
from jsonschema import Draft202012Validator
R=Path.cwd();sys.path.insert(0,str(R));P=Path(__file__).parent;D=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests'
for name in ('🏠ground','🚶️hall'):Draft202012Validator(json.loads((D/(name+'.schema.json')).read_text(encoding='utf8'))).validate(json.loads((D/(name+'.json')).read_text(encoding='utf8')))
T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🏠ground');m=T.model.load_model();wall=T.model._find(m,'W-G-SERVICE');assert wall['material']=='LS-PARTITION-15'
error=None
try:T.test_hall_gap_remains_source_bound_without_certifying_800mm_passage()
except AssertionError:error=traceback.format_exc()
assert error and "host['material']==partition['material']" in error
report={'phase':'RED','schema_fixtures_valid':True,'unchanged_actual_material':wall['material'],'expected_material':'LS-PARTITION-08','body_diameters':[row['diameter'] for row in json.loads((D/'🚶️hall.json').read_text(encoding='utf8'))['cases']],'error':error}
(P/'🗑️generated/native-r95-hall/fixture-red.json').write_text(json.dumps(report,indent=2),encoding='utf8');print('[DEBUG] Actual unchanged15cm source RED; both schemas GREEN and required77cm bodies retained')
