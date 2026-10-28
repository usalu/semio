"""🔬️ Compare native IfcOpenShell header authoring and owned STEP output for every neutral selection."""
import ctypes
import json
import pathlib
import sys

ticket = pathlib.Path(__file__).resolve().parent
module_path = str(ticket / '🗑️generated/oracle')
if sys.platform == 'win32':
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path, short, len(short))
    module_path = short.value or module_path
sys.path.insert(0, module_path)
import ifcopenshell

base = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any'
cases = json.loads((base / '🧬️schema/👁️mvd/header-cases.json').read_text(encoding='utf-8'))
facts = json.loads((ticket / '🗑️generated/mvd-header-facts.json').read_text(encoding='utf-8'))
assert len(facts) == len(cases['cases'])
for case, fact in zip(cases['cases'], facts, strict=True):
    assert fact['name'] == case['name']
    assert fact['description'] == case['after']
    independent = ifcopenshell.file(schema='IFC4X3_ADD2')
    independent.header.file_description.description = tuple(case['after'])
    independent.header.file_description.implementation_level = '2;1'
    independent.header.file_name.name = 'mvd.ifc'
    independent.header.file_name.time_stamp = '2026-10-21T12:00:00'
    independent.header.file_name.author = ('Author',)
    independent.header.file_name.organization = ('Organisation',)
    independent.header.file_name.preprocessor_version = 'Writer'
    independent.header.file_name.originating_system = 'Application'
    independent.header.file_name.authorization = 'Authorisation'
    parsed = ifcopenshell.file.from_string(fact['step'])
    for name in ('file_description', 'file_name', 'file_schema'):
        assert tuple(getattr(parsed.header, name)) == tuple(getattr(independent.header, name)), (case['name'], name)
    assert tuple(ifcopenshell.file.from_string(independent.to_string()).header.file_description.description) == tuple(case['after'])
for case in cases['declarations']:
    model = ifcopenshell.file(schema='IFC4X3_ADD2')
    model.header.file_description.description = (case['text'],)
    parsed = ifcopenshell.file.from_string(model.to_string())
    assert parsed.header.file_description.description == (case['text'],)
result = {'oracle': 'IfcOpenShell', 'version': ifcopenshell.version, 'selections': len(facts), 'retainedDeclarations': len(cases['declarations']), 'mismatches': 0}
(ticket / '🗑️generated/mvd-header-oracle.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8')
print(json.dumps(result))
