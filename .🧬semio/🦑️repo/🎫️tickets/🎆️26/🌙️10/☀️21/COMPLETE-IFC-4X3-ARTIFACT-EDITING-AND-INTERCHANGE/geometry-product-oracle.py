"""🌳 Verify actual represented-product eligibility against the independent complete ADD2 hierarchy."""
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
from ifcopenshell import ifcopenshell_wrapper

output = ticket / '🗑️generated' / 'geometry-products'
owned = json.loads((output / 'owned.json').read_text('utf8'))
source = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/geometry-product-cases.json'
cases = json.loads(source.read_text('utf8'))
schema = ifcopenshell_wrapper.schema_by_name(owned['schema'])

def is_product(name):
    try:
        entity = schema.declaration_by_name(name).as_entity()
    except RuntimeError:
        return False
    while entity is not None:
        if entity.name().upper() == 'IFCPRODUCT':
            return True
        entity = entity.supertype()
    return False

for row in owned['entities'] + cases['cases']:
    assert row['isProduct'] == is_product(row['entity']), row
assert len(owned['entities']) == 876
result = {'implementation': 'IfcOpenShell', 'version': ifcopenshell.version, 'schema': owned['schema'], 'entities': len(owned['entities']), 'neutralCases': len(cases['cases']), 'products': sum(row['isProduct'] for row in owned['entities']), 'mismatches': 0, 'nativeExecution': False}
(output / 'oracle.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf8')
print(json.dumps(result))
