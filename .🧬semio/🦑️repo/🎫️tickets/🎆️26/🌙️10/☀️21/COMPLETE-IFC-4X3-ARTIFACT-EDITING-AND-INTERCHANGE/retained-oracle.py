"""🔬 Compare retained declaration input with IfcOpenShell and validate the neutral contract."""
import hashlib
import json
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(parent for parent in TICKET.parents if (parent / 'AGENTS.md').exists())
SCHEMA = ROOT / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema'
sys.path.insert(0, str(TICKET / '🗑️generated/ifc083'))
sys.path.insert(0, str(TICKET / '🗑️generated/retained-deps'))
import ifcopenshell.ifcopenshell_wrapper as wrapper
import jsonschema

catalog = json.loads((SCHEMA / '📚️catalog/🔣️.json').read_text(encoding='utf-8'))
native = wrapper.schema_by_name('IFC4X3_ADD2')
count = 0
maximum_chain = 0
for key, expected in catalog['entities'].items():
    actual = native.declaration_by_name(key)
    assert actual.is_abstract() == expected['abstractEntity'], key
    parent = actual.supertype()
    assert (parent.name().upper() if parent else None) == expected.get('supertype'), key
    attrs = actual.all_attributes()
    assert [item.name() for item in attrs] == [item['name'] for item in expected['allAttributes']], key
    assert list(actual.derived()) == [item['derived'] for item in expected['allAttributes']], key
    assert [item.optional() for item in attrs] == [item['optional'] for item in expected['allAttributes']], key
    chain = 1
    while parent:
        chain += 1
        parent = parent.supertype()
    maximum_chain = max(chain, maximum_chain)
    count += len(attrs)
for key, expected in catalog['types'].items():
    actual = native.declaration_by_name(key)
    if expected['kind'] == 'enum':
        assert list(actual.enumeration_items()) == expected['members'], key
    elif expected['kind'] == 'select':
        assert [member.name().upper() for member in actual.select_list()] == expected['members'], key
assert maximum_chain <= 32
contract = json.loads((SCHEMA / '⚖️validation/retained-cases.json').read_text(encoding='utf-8'))
schema = json.loads((SCHEMA / '⚖️validation/retained.json').read_text(encoding='utf-8'))
jsonschema.Draft202012Validator.check_schema(schema)
jsonschema.validate(contract, schema)
result = {'oracle': 'IfcOpenShell 0.8.3', 'entities': len(catalog['entities']), 'types': len(catalog['types']), 'inheritedAttributes': count, 'maximumInheritanceChain': maximum_chain, 'contract': 'valid', 'nativeExecution': False}
output = TICKET / '🗑️generated/retained-express'
output.mkdir(parents=True, exist_ok=True)
(output / 'metadata-oracle.json').write_text(json.dumps(result, indent=2), encoding='utf-8')
print(json.dumps(result))
