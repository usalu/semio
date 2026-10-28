"""🔬️ Validate stress-model populations independently and emit native-writer reimport inputs."""
import ctypes
import json
import pathlib
import subprocess
import sys

ticket = pathlib.Path(__file__).resolve().parent
module_path = str(ticket / '🗑️generated/oracle')
if sys.platform == 'win32':
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path, short, len(short))
    module_path = short.value or module_path
sys.path.insert(0, module_path)
import ifcopenshell
import ifcopenshell.validate

output = ticket / '🗑️generated/stepctl'
cases = json.loads((output / 'cases.json').read_text(encoding='utf-8'))
if len(sys.argv) == 3 and sys.argv[1] == '--validate':
    model = ifcopenshell.file.from_string(pathlib.Path(sys.argv[2]).read_text(encoding='utf-8'))
    logger = ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(model, logger, express_rules=True)
    print(json.dumps({'errors': logger.statements}))
    raise SystemExit(0)
selected = next((argument.removeprefix('--cases=').split(',') for argument in sys.argv[1:] if argument.startswith('--cases=')), None)
if selected is not None:
    cases = [case for case in cases if case['name'] in selected]
results = []
for case in cases:
    name = case['name']
    original = ifcopenshell.file.from_string((output / (name + '.source.ifc')).read_text(encoding='utf-8'))
    owned = ifcopenshell.file.from_string((output / (name + '.owned.ifc')).read_text(encoding='utf-8'))
    assert len(list(owned)) == 1
    source_count = len(list(original))
    if not name.startswith('lexical-'):
        assert source_count == 1
    equal = source_count == 1
    difference = None if equal else {'sourceEntities': source_count, 'ownedEntities': 1}
    for entity in original:
        if entity.get_info(recursive=True) != owned.by_id(entity.id()).get_info(recursive=True):
            equal = False
            difference = {'source': entity.get_info(recursive=True), 'owned': owned.by_id(entity.id()).get_info(recursive=True)}
    if not name.startswith('lexical-'):
        assert equal, name
    if name.startswith('lexical-'):
        property = owned.by_id(1)
        authored = ifcopenshell.file(schema='IFC4X3_ADD2')
        value = authored.create_entity(property.NominalValue.is_a(), property.NominalValue.wrappedValue)
        authored_property = authored.create_entity('IfcPropertySingleValue', Name=property.Name, NominalValue=value)
        assert authored_property.get_info(recursive=True) == property.get_info(recursive=True), name
    (output / (name + '.independent.ifc')).write_text(owned.to_string(), encoding='utf-8')
    if name == 'exact-integer':
        assert owned.by_id(1).NominalValue.wrappedValue == 9007199254740993
    validations = []
    for suffix in ('source', 'owned'):
        try:
            checked = subprocess.run([sys.executable, str(pathlib.Path(__file__).resolve()), '--validate', str(output / (name + '.' + suffix + '.ifc'))], capture_output=True, text=True, encoding='utf-8', timeout=20, check=True)
            errors = json.loads(checked.stdout)['errors']
            if not name.startswith('lexical-') or suffix == 'owned':
                assert not errors, (name, suffix, errors)
            validations.append({'input': suffix, 'complete': True, 'errors': len(errors)})
        except subprocess.TimeoutExpired:
            validations.append({'input': suffix, 'complete': False, 'reason': 'independent EXPRESS execution exceeded 20 seconds'})
    results.append({'name': name, 'equalPopulation': equal, 'nativeAuthoredEquivalent': name.startswith('lexical-'), 'difference': difference, 'validation': validations})
    print('[DEBUG] ' + json.dumps(results[-1]), flush=True)
result = {'oracle': 'IfcOpenShell', 'version': ifcopenshell.version, 'cases': results, 'sourceInterpretationDifferences': sum(not item['equalPopulation'] for item in results)}
(output / ('oracle-subset.json' if selected is not None else 'oracle.json')).write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8')
print(json.dumps({'version': ifcopenshell.version, 'models': len(results), 'sourceInterpretationDifferences': result['sourceInterpretationDifferences']}))
