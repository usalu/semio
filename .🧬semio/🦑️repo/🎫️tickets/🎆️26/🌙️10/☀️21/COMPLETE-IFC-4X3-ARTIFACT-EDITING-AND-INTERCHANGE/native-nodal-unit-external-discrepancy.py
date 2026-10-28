"""📏 Diagnose an external generated-rule discrepancy without changing native expectations."""
import ast,ctypes,inspect,json,sys
from pathlib import Path
p=Path(__file__).parent;dep=str(p/'🗑️generated/oracle')
if sys.platform=='win32':
 b=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,b,len(b));dep=b.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.validate
from ifcopenshell.express.rules import IFC4X3_ADD2 as r
f=ifcopenshell.file(schema='IFC4X3_ADD2');u=f.create_entity('IfcSIUnit',UnitType='FORCEUNIT',Name='NEWTON');log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True);text=inspect.getsource(r.IfcCorrectDimensions);result=dict(version=ifcopenshell.version,unit=str(u),dimensions=str(u.Dimensions),correctDimensions=r.IfcCorrectDimensions(u.UnitType,u.Dimensions),diagnostics=log.statements,functionSource=text);(p/'🗑️generated/native-nodal-unit-external-discrepancy.json').write_text(json.dumps(result,indent=2,default=str)+'\n',encoding='utf-8');print('[DEBUG] '+json.dumps({k:v for k,v in result.items()if k!='functionSource'},default=str))
