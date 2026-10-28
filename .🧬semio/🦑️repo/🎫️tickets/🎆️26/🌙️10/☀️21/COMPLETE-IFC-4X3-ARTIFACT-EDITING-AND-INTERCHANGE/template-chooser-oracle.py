"""🔬 Check the complete released selector roster against independent IfcOpenShell models."""
import ctypes
import json
import pathlib
import sys
from collections import Counter

ticket = pathlib.Path(__file__).resolve().parent
module_path = str(ticket / '🗑️generated' / 'oracle')
if sys.platform == 'win32':
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path, short, len(short))
    module_path = short.value or module_path
sys.path.insert(0, module_path)
import ifcopenshell
from ifcopenshell.util.pset import get_template

source = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates'
contract = json.loads((source / '✍️apply' / 'chooser.json').read_text('utf8'))
owned = json.loads((source / '🔣️.json').read_text('utf8'))['templates']
independent = [row for model in get_template(contract['schema']).templates for row in model.by_type('IfcPropertySetTemplate')]
actual = [(row['name'], row['kind']) for row in owned]
expected = [(row.Name, row.TemplateType) for row in independent]
assert len(set(name for name, _ in actual)) == contract['choices']
assert sorted(actual) == sorted(expected)
assert dict(Counter(kind for _, kind in expected)) == contract['kinds']
result = {'implementation': 'IfcOpenShell', 'version': ifcopenshell.version, 'schema': contract['schema'], 'choices': len(expected), 'mismatches': 0, 'kinds': dict(Counter(kind for _, kind in expected)), 'nativeExecution': False}
output = ticket / '🗑️generated' / 'template-chooser'
output.mkdir(parents=True, exist_ok=True)
(output / 'oracle.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf8')
print(json.dumps(result, ensure_ascii=False))
