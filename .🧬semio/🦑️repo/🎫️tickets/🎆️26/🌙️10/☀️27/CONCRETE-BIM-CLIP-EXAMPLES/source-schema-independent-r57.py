import json,hashlib,importlib.metadata,time
from pathlib import Path
from jsonschema import Draft202012Validator
p=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES');root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model')
progress=p/'🗑️generated/source-schema-progress-r57.json';progress.write_text(json.dumps({'phase':'read JSON + validate','state':'running'}))
started=time.monotonic();raw=(root/'🔣️.json').read_bytes();source=json.loads(raw);schema=json.loads((root/'🧬️schema/🔣️.schema.json').read_text());Draft202012Validator.check_schema(schema);errors=[{'path':list(e.absolute_path),'schema_path':list(e.absolute_schema_path),'message':e.message} for e in Draft202012Validator(schema).iter_errors(source)]
result={'sha256':hashlib.sha256(raw).hexdigest(),'elements':len(source['elements']),'jsonschema':importlib.metadata.version('jsonschema'),'draft':'2020-12','schema_valid':True,'errors':errors,'seconds':time.monotonic()-started};(p/'🗑️generated/source-schema-independent-r57.json').write_text(json.dumps(result));progress.write_text(json.dumps({'phase':'complete','state':'PASS' if not errors else 'FAIL'}));print(json.dumps(result))
