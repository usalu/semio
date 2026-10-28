"""🧭 Corroborate the actual zero-direction rule with independent released EXPRESS validation."""
import sys,ctypes,json,hashlib
from pathlib import Path
t=Path(__file__).resolve().parent;dep=str(t/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.validate
p=t/'🗑️generated/express-zero-direction.ifc';f=ifcopenshell.open(str(p));log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True);rows=[{'attribute':r.get('attribute'),'message':str(r.get('message'))}for r in log.statements];result={'library':ifcopenshell.version,'header':list(f.header.file_schema.schema_identifiers),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'direction':list(f.by_id(1).DirectionRatios),'diagnostics':rows,'nativeExecution':False};(t/'🗑️generated/express-zero-direction-oracle.json').write_text(json.dumps(result,indent=2));print('[DEBUG] '+json.dumps({'diagnostics':[r['attribute']for r in rows]}))