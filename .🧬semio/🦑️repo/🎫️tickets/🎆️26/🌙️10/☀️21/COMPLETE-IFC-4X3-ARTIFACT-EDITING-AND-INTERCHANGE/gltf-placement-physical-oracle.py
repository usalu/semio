"""🧭 Actual GLB import physical IFC full EXPRESS check."""
import ctypes,json,sys
from pathlib import Path
t=Path(__file__).resolve().parent;dep=str(t/'🗑️generated/oracle')
if sys.platform=='win32':
 s=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,s,len(s));dep=s.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.validate
out=t/'🗑️generated/gltf-placement';f=ifcopenshell.open(str(out/'box-import.ifc'));logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,logger,express_rules=True);rows=[{'level':r.get('level'),'attribute':r.get('attribute'),'message':str(r.get('message'))}for r in logger.statements];result={'library':ifcopenshell.version,'header':list(f.header.file_schema.schema_identifiers),'products':[{'id':p.id(),'placement':p.ObjectPlacement.id()if p.ObjectPlacement else None}for p in f.by_type('IfcProduct')],'diagnostics':rows,'nativeExecution':False};(out/'independent.json').write_text(json.dumps(result,indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps({'products':len(result['products']),'diagnostics':len(rows)}))
