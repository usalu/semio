"""🧬 Validate the complete owned source candidate before a guarded five-product publication."""
import hashlib,importlib.util,json,sys,time
from pathlib import Path
from jsonschema import Draft202012Validator
B=Path(__file__).parent;O=B/'🗑️generated/native-r92';sys.path.insert(0,str(Path.cwd()));started=time.perf_counter()
def module(name,file):
    spec=importlib.util.spec_from_file_location(name,B/file);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
F=module('coal_author','coal-bay-floor-author-r94.py');G=module('grade_author','garage-approach-authoring-r94.py');A=F.A
raw=A._source();m=json.loads(raw);guids={e['tag']:e['guid'] for e in m['elements']};relations=json.dumps(m['relations'],sort_keys=True);floor=json.loads((O/'coal-bay-floor-candidate-r94.json').read_text());grade=json.loads((O/'garage-approach-candidate-r94.json').read_text())
F.apply(m,floor);G.apply_candidate(m,grade)
schema=json.loads((Path(A.__file__).parent/'🧬️schema/🔣️.schema.json').read_text(encoding='utf-8'));errors=list(Draft202012Validator(schema).iter_errors(m))
assert not errors,[(list(e.path),e.message) for e in errors[:20]]
assert {e['tag']:e['guid'] for e in m['elements']}==guids and json.dumps(m['relations'],sort_keys=True)==relations
report={'success':True,'source_sha256':hashlib.sha256(raw.encode('utf-8')).hexdigest(),'floor_candidate_sha256':hashlib.sha256((O/'coal-bay-floor-candidate-r94.json').read_bytes()).hexdigest(),'approach_candidate_sha256':hashlib.sha256((O/'garage-approach-candidate-r94.json').read_bytes()).hexdigest(),'products':len(m['elements']),'schema_errors':0,'guid_identity_and_relations':'unchanged','seconds':time.perf_counter()-started}
(O/'floor-grade-schema-r94.json').write_text(json.dumps(report,indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps(report),flush=True)
