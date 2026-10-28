"""🔬 Inspect the independent IfcOpenShell property-template distribution."""
import ctypes
import json
import pathlib
import sys
from collections import Counter
ticket = pathlib.Path(__file__).resolve().parent
module_path = str(ticket / '🗑️generated/oracle')
if sys.platform == 'win32':
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path, short, len(short))
    module_path = short.value or module_path
sys.path.insert(0, module_path)
import ifcopenshell
from ifcopenshell.util.pset import get_template
from ifcopenshell import ifcopenshell_wrapper
catalog = get_template('IFC4X3_ADD2')
schema = ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2')
fixture = json.loads((ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/🏷️templates/cases.json').read_text(encoding='utf-8'))
for case in fixture['applicability']:
    try:
        applicable = catalog.get_applicable(case['entity'], predefined_type=case.get('predefinedType', ''))
        admitted = case['template'] in [item.Name for item in applicable]
        assert admitted == (case['state'] == 'applicable'), case
    except RuntimeError:
        assert case['state'] == 'unknownEntity', case
rows = []
for model in catalog.templates:
    for template in model.by_type('IfcPropertySetTemplate'):
        rows.append({'name': template.Name, 'kind': template.TemplateType, 'applicable': template.ApplicableEntity, 'entries': [{'name': item.Name, 'kind': item.TemplateType, 'type': item.PrimaryMeasureType, 'secondary': item.SecondaryMeasureType, 'enum': [str(value.wrappedValue) for value in item.Enumerators.EnumerationValues] if item.Enumerators else None} for item in template.HasPropertyTemplates]})
        for entry in rows[-1]['entries']:
            if entry['kind'].startswith('Q_'):
                quantity = schema.declaration_by_name('IfcQuantity' + entry['kind'][2:].title())
                attributes = [attribute for attribute in quantity.all_attributes() if attribute.name().endswith('Value')]
                assert len(attributes) == 1
                entry['type'] = attributes[0].type_of_attribute().declared_type().name()
result = {'implementation': 'IfcOpenShell', 'version': ifcopenshell.version, 'templates': rows}
for template in rows:
    for selector in template['applicable'].split(','):
        assert schema.declaration_by_name(selector.split('/')[0])
    for entry in template['entries']:
        assert schema.declaration_by_name(entry['type'])
        if entry['secondary']:
            assert schema.declaration_by_name(entry['secondary'])
(ticket / '🗑️generated/property-sets/ifcopenshell.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
print(json.dumps({'implementation': 'IfcOpenShell', 'version': ifcopenshell.version, 'sets': len(rows), 'kinds': dict(Counter(row['kind'] for row in rows)), 'entries': sum(len(row['entries']) for row in rows)}))
