"""🔬 Compare all retained IFC4X3_ADD2 declarations with the independent IfcOpenShell parser."""
import hashlib
import json
import pathlib
import sys
import struct

TICKET = pathlib.Path(__file__).resolve().parent
oracle_path = str(TICKET / '🗑️generated/oracle')
if sys.platform == 'win32':
    import ctypes
    buffer = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(oracle_path, buffer, len(buffer))
    if buffer.value:
        oracle_path = buffer.value
sys.path.insert(0, oracle_path)
import ifcopenshell
from ifcopenshell import ifcopenshell_wrapper
import ifcopenshell.validate

ROOT = TICKET.parents[6]
BASE = ROOT / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any'
catalog = json.loads((BASE / '🧬️schema/📚️catalog/🔣️.json').read_text(encoding='utf-8'))
schema = ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2')
failures = []
checks = 0


def equal(actual, expected, context):
    global checks
    checks += 1
    if actual != expected:
        failures.append(dict(context=context, actual=actual, expected=expected))


equal(hashlib.sha256((BASE / '🧬️schema/📚️catalog/IFC4X3_ADD2.exp').read_bytes()).hexdigest(), catalog['source']['sha256'], 'source checksum')
equal(len(schema.entities()), len(catalog['entities']), 'entity count')
for name, declaration in catalog['entities'].items():
    reference = schema.declaration_by_name(name)
    equal(reference.is_abstract(), declaration['abstractEntity'], name + ' abstract')
    parent = reference.supertype()
    equal(parent.name().upper() if parent else None, declaration['supertype'], name + ' supertype')
    attrs = reference.all_attributes()
    equal([attr.name() for attr in attrs], [attr['name'] for attr in declaration['allAttributes']], name + ' attribute order')
    equal([attr.optional() for attr in attrs], [attr['optional'] for attr in declaration['allAttributes']], name + ' attribute optionality')
    equal(list(reference.derived()), [attr['derived'] for attr in declaration['allAttributes']], name + ' derived positions')
for name, declaration in catalog['types'].items():
    reference = schema.declaration_by_name(name)
    if declaration['kind'] == 'enum':
        equal([item.upper() for item in reference.enumeration_items()], declaration['members'], name + ' enum members')
    elif declaration['kind'] == 'select':
        equal([item.name().upper() for item in reference.select_list()], declaration['members'], name + ' select members')


def step_value(value):
    kind = value['kind']
    if kind == 'unset':
        return '$'
    if kind == 'derived':
        return '*'
    if kind == 'integer':
        return value['value']
    if kind == 'real':
        number = struct.unpack('>d', int(value['value']['bits']).to_bytes(8, 'big'))[0]
        return repr(number)
    if kind == 'string':
        return "'" + value['value'].replace("'", "''") + "'"
    if kind == 'enum':
        return '.' + value['value'] + '.'
    if kind == 'reference':
        return '#' + value['value']
    if kind == 'aggregate':
        return '(' + ','.join(step_value(item) for item in value['value']) + ')'
    if kind == 'typedValue':
        return value['value']['name'] + '(' + ','.join(step_value(item) for item in value['value']['items']) + ')'
    raise ValueError(kind)


for case in json.loads((BASE / '🧪️tests/schema-cases.json').read_text(encoding='utf-8')):
    if case['name'] in ['duplicate', 'unknown']:
        continue
    data = '\n'.join('#' + entity['id'] + '=' + entity['name'] + '(' + ','.join(step_value(item) for item in entity['args']) + ');' for entity in case['entities'])
    text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\nFILE_SCHEMA(('IFC4X3_ADD2'));\nENDSEC;\nDATA;\n" + data + '\nENDSEC;\nEND-ISO-10303-21;'
    model = ifcopenshell.file.from_string(text)
    logger = ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(model, logger, express_rules=False)
    equal(not logger.statements, not case['codes'], case['name'] + ' structural validation result')

result = dict(oracle='IfcOpenShell', version=ifcopenshell.version, checks=checks, failures=failures)
(TICKET / '🗑️generated/schema-oracle.json').write_text(json.dumps(result, indent=2), encoding='utf-8')
print(json.dumps(result))
if failures:
    raise SystemExit(1)
