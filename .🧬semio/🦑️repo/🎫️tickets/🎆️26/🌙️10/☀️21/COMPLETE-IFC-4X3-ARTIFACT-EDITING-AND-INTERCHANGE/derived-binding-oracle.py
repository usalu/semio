"""🔮 Full IfcOpenShell rule validation for inherited effective derived SELF attributes."""
import ctypes,json,pathlib,sys
root=pathlib.Path(__file__).resolve().parent
module_path=str(root/'🗑️generated'/'oracle')
if sys.platform=='win32':
 short=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(module_path,short,len(short));module_path=short.value or module_path
sys.path.insert(0,module_path)
import ifcopenshell,ifcopenshell.validate
base=pathlib.Path.cwd()/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation'
cases=json.loads((base/'derived-binding-cases.json').read_text(encoding='utf-8'))
header=json.loads((base/'models.json').read_text(encoding='utf-8'))
rows=[]
for case in cases:
 model=ifcopenshell.file.from_string(header['header']+case['body']+header['footer'])
 structural=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,structural,express_rules=False)
 logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,logger,express_rules=True)
 diagnostics=[{key:value.id() if isinstance(value,ifcopenshell.entity_instance) else str(value) for key,value in row.items()} for row in logger.statements]
 north=sorted(str(row.get('instance').id()) for row in logger.statements if 'north2d' in str(row.get('attribute','')).lower())
 rows.append({'name':case['name'],'version':ifcopenshell.version,'structuralErrors':len(structural.statements),'northFailures':north,'diagnostics':diagnostics})
output=root/'🗑️generated'/'derived-binding-oracle.json';output.write_text(json.dumps(rows,indent=2),encoding='utf-8')
print('[DEBUG]',json.dumps(rows))
for actual,expected in zip(rows,cases):
 assert actual['structuralErrors']==0,actual
 assert actual['northFailures']==sorted(expected['northFailures']),actual