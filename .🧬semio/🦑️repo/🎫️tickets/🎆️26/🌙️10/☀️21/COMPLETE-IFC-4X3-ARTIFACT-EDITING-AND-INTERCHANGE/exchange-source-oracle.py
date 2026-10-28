"""🔬 Validate generated captured sources independently and serialize them for reciprocal projection."""
import ctypes
import json
import pathlib
import sys

ticket = pathlib.Path(__file__).resolve().parent
module_path = str(ticket / '🗑️generated' / 'oracle')
if sys.platform == 'win32':
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path, short, len(short))
    module_path = short.value or module_path
sys.path.insert(0, module_path)
import ifcopenshell
import ifcopenshell.validate

output = ticket / '🗑️generated' / 'exchange-source'
owned = json.loads((output / 'owned.json').read_text('utf8'))
rows = []
for row in owned['rows']:
    model = ifcopenshell.open(str(output / (row['id'] + '.ifc')))
    logger = ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(model, logger, express_rules=True)
    assert not logger.statements, (row['id'], logger.statements)
    (output / (row['id'] + '.independent.ifc')).write_text(model.to_string(), encoding='utf8')
    rows.append({'id': row['id'], 'schema': model.schema_identifier, 'entities': len(list(model)), 'expressErrors': len(logger.statements)})
result = {'implementation': 'IfcOpenShell', 'version': ifcopenshell.version, 'rows': rows, 'nativeExecution': False}
(output / 'oracle.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf8')
print(json.dumps(result))

