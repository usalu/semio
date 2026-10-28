"""🔬 Independent released IFC declaration membership expectations for retained metadata."""
import ctypes
import json
from pathlib import Path
import sys
from typing import Protocol

ticket = Path(__file__).resolve().parent
modules = str(ticket / '🗑️generated' / 'oracle')
if sys.platform == 'win32':
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(modules, short, len(short))
    modules = short.value or modules
sys.path.insert(0, modules)
import ifcopenshell
root = next(parent for parent in ticket.parents if (parent / 'package.json').exists())
contract = json.loads((root / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests/domain-metadata.json').read_text('utf8'))
class SchemaOracle(Protocol):
    def canonical(self, name: str) -> str | None: ...
    def accepts(self, expected: str, actual: str) -> bool: ...
class IfcOpenShellOracle:
    def __init__(self):
        self.schema = ifcopenshell.ifcopenshell_wrapper.schema_by_name(contract['schema'])
        self.names = {value.name().upper(): value for value in self.schema.declarations()}
    def canonical(self, name):
        return name.upper() if name.upper() in self.names else None
    def accepts(self, expected, actual):
        if expected == actual:
            return actual in self.names
        current = self.names[actual].as_entity()
        while current:
            if current.name().upper() == expected:
                return True
            current = current.supertype()
        select = self.names[expected].as_select_type()
        return bool(select and any(self.accepts(member.name().upper(), actual) for member in select.select_list()))
oracle: SchemaOracle = IfcOpenShellOracle()
for row in contract['names']:
    assert oracle.canonical(row['source']) == row['canonical'], row
for row in contract['membership']:
    assert oracle.accepts(row['expected'], row['actual']) == row['admitted'], row
print(json.dumps({'oracle': 'IfcOpenShell ' + ifcopenshell.version, 'canonicalNames': len(contract['names']), 'selectAliasAndSubtypeCases': len(contract['membership']), 'nativeExecution': False, 'originalAccounting': False}))
