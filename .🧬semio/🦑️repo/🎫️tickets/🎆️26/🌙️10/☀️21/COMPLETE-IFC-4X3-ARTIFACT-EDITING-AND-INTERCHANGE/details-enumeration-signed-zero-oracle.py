"""🎨 Check actual signed-zero enumeration membership independently of transport-word equality."""
import ctypes,json,sys
from pathlib import Path
path=str(Path(__file__).resolve().parent/'🗑️generated'/'oracle')
if sys.platform=='win32':
 short=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(path,short,len(short));path=short.value or path
sys.path.insert(0,path)
import ifcopenshell,ifcopenshell.validate
file=ifcopenshell.file(schema='IFC4X3_ADD2')
definition=file.create_entity('IfcPropertyEnumeration',Name='Zero',EnumerationValues=(file.create_entity('IfcReal',0.0),))
property=file.create_entity('IfcPropertyEnumeratedValue',Name='Actual',EnumerationValues=(file.create_entity('IfcReal',-0.0),),EnumerationReference=definition)
logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(file,logger,express_rules=True)
result={'oracle':'IfcOpenShell '+ifcopenshell.version,'case':'typed IfcReal negative zero selected from positive zero definition','diagnostics':logger.statements,'nativeExecution':False}
(Path(__file__).resolve().parent/'🗑️generated'/'details-enumeration-signed-zero-oracle.json').write_text(json.dumps(result,default=str,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(result,default=str))

