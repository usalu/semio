"""⚖️ Full independent EXPRESS check of actual current rectangle fixture output."""
import ctypes,json,sys,hashlib
from pathlib import Path
t=Path(__file__).resolve().parent;dep=str(t/'🗑️generated/oracle')
if sys.platform=='win32':
 s=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,s,len(s));dep=s.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.validate
out=t/'🗑️generated/rectangle-actual';rows=[]
for row in json.loads((out/'manifest.json').read_text())['rows']:
 path=out/(row['id']+'.ifc');f=ifcopenshell.open(str(path));logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,logger,express_rules=True);diagnostics=[{'level':r.get('level'),'attribute':r.get('attribute'),'message':str(r.get('message'))}for r in logger.statements];rows.append({'id':row['id'],'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'header':list(f.header.file_schema.schema_identifiers),'product':row['product'],'placement':f.by_id(int(row['product'])).ObjectPlacement.id(),'diagnostics':diagnostics})
(out/'independent.json').write_text(json.dumps({'library':ifcopenshell.version,'rows':rows,'nativeExecution':False},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps([{'id':r['id'],'diagnostics':len(r['diagnostics'])}for r in rows]))
