"""🧭 Retain full raw SDK source diagnostics for deliberately invalid classification source."""
from pathlib import Path
import json,importlib.util
import ifcopenshell,ifcopenshell.validate
p=Path(__file__).parent;spec=importlib.util.spec_from_file_location('reference',p/'issued-express-classification-oracle.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);f=ifcopenshell.file.from_string(m.raw.decode('utf8'));log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True);out={'diagnostics':log.statements,'observations':[{'entityId':str(r['instance'].id())if hasattr(r.get('instance'),'id')else None,'attribute':r.get('attribute'),'message':r.get('message')}for r in log.statements]};(p/'🗑️generated/issued-classification-independent-full-express.json').write_text(json.dumps(out,indent=2,default=str)+'\n',encoding='utf8');print('[DEBUG]',len(log.statements),[(x['entityId'],x['attribute'])for x in out['observations']])
