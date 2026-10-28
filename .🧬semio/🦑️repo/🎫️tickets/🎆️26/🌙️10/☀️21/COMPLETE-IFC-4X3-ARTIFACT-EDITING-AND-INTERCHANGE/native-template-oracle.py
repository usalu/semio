"""🧪 Re-author the permanent native template goldens through the existing independent test interface."""
import json
import pathlib
import subprocess
import sys

ticket = pathlib.Path(__file__).resolve().parent
root = ticket.parents[6]
templates = root / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates'
catalog = json.loads((templates / '🔣️.json').read_text('utf8'))
fixtures = json.loads((templates / '✍️apply/native-cases.json').read_text('utf8'))
directory = ticket / '🗑️generated' / 'native-template-oracle'
directory.mkdir(parents=True, exist_ok=True)
cases = []
for fixture in fixtures['cases']:
    name, application = fixture['name'], fixture['application']
    declaration = next(template for template in catalog['templates'] if template['name'] == application['template'])
    cases.append({'name': name, 'application': application, 'template': declaration})
    (directory / f'{name}-source.ifc').write_text(fixture['source'], 'utf8')
    (directory / f'{name}-semio.ifc').write_text(fixture['expected'], 'utf8')
(directory / 'cases.json').write_text(json.dumps(cases, ensure_ascii=False, indent=2), 'utf8')
if '--select-only' not in sys.argv:
    subprocess.run([sys.executable, str(ticket / 'template-authoring-oracle.py'), str(directory)], check=True)
sys.path.insert(0, str(ticket / '🗑️generated' / 'ifc083'))
import ifcopenshell
import ifcopenshell.validate
manifest = json.loads((templates / '✍️apply/native-select.json').read_text('utf8'))
model = ifcopenshell.open(str(directory / f"{manifest['sourceCase']}-semio.ifc"))
relation = model.by_type(manifest['relation'])[0]
definition = model.create_entity(manifest['type'], (relation.RelatingPropertyDefinition,))
relation.RelatingPropertyDefinition = definition
logger = ifcopenshell.validate.json_logger()
ifcopenshell.validate.validate(model, logger, express_rules=True)
assert not logger.statements, logger.statements
model.write(str(directory / 'select.ifc'))
subprocess.run(['bun', str(ticket / 'native-template-select.ts')], cwd=root, check=True)
print(json.dumps({'select': manifest['type'], 'expressDiagnostics': len(logger.statements), 'ownDuplicateTemplate': manifest['refusal']}))
