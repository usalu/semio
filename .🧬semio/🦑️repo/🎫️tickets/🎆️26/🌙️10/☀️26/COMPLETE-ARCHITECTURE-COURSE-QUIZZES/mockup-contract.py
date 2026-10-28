from pathlib import Path
import json

root = Path('🧰️framework/🛍️products/📚️course')
schema_path = root / '🧬️schema/🔣️.json'
schema = json.loads(schema_path.read_text(encoding='utf-8'))
defs = schema['$defs']
ref = lambda name: {'$ref': '#/$defs/' + name}
arr = lambda item, minimum=0, maximum=16: {'type': 'array', 'minItems': minimum, 'maxItems': maximum, 'items': item}
integer = {'type': 'integer', 'minimum': 0, 'maximum': 31}
obj = lambda props, optional=(): {'type': 'object', 'additionalProperties': False, 'required': [key for key in props if key not in optional], 'properties': props}
enum = lambda values: {'type': 'string', 'enum': values.split('|')}
defs['MockupText'] = obj({'en': {'type': 'string', 'maxLength': 256}, 'de': {'type': 'string', 'maxLength': 256}})
defs['MockupSelection'] = obj({'row': integer, 'column': integer, 'rowEnd': integer, 'columnEnd': integer}, ['rowEnd', 'columnEnd'])
defs['ExcelTarget'] = enum('cell|range|row|column|worksheet|workbook|formula|name-box')
defs['ExcelMockup'] = obj({'kind': {'const': 'excel'}, 'caption': ref('Text'), 'workbook': ref('Text'), 'sheets': arr(ref('Text'), 1, 8), 'activeSheet': integer, 'columns': arr({'type': 'string', 'pattern': '^[A-Z]{1,2}$'}, 1, 16), 'rows': arr(arr(ref('MockupText'), 1, 16), 1, 32), 'selection': ref('MockupSelection'), 'formula': ref('MockupText'), 'target': ref('ExcelTarget')})
defs['MockupPort'] = obj({'id': ref('Slug'), 'label': ref('Text')})
defs['MockupNode'] = obj({'id': ref('Slug'), 'label': ref('Text'), 'inputs': arr(ref('MockupPort'), 0, 12), 'outputs': arr(ref('MockupPort'), 0, 12)})
defs['MockupEndpoint'] = obj({'node': ref('Slug'), 'port': ref('Slug')})
defs['MockupWire'] = obj({'from': ref('MockupEndpoint'), 'to': ref('MockupEndpoint')})
defs['MockupBranch'] = obj({'path': {'type': 'string', 'pattern': '^\\{[0-9]+(?:;[0-9]+)*\\}$', 'maxLength': 64}, 'values': arr(ref('MockupText'), 0, 16)})
defs['GrasshopperTargetKind'] = enum('node|input|output|wire|tree|branch|item')
defs['MockupTarget'] = obj({'kind': ref('GrasshopperTargetKind'), 'index': integer, 'node': ref('Slug'), 'port': ref('Slug')}, ['node', 'port'])
defs['GrasshopperMockup'] = obj({'kind': {'const': 'grasshopper'}, 'caption': ref('Text'), 'nodes': arr(ref('MockupNode'), 1, 8), 'wires': arr(ref('MockupWire'), 0, 24), 'tree': arr(ref('MockupBranch'), 0, 12), 'target': ref('MockupTarget')})
defs['Mockup'] = {'description': 'A solution-free software workspace preview. Only given reference data belongs here; grading truth stays on the authored item.', 'oneOf': [ref('ExcelMockup'), ref('GrasshopperMockup')]}
for name in ['ClassificationItem', 'SortingItem', 'MatchingItem', 'SheetItem']:
    defs[name]['properties']['mockup'] = ref('Mockup')
defs['WireVersion']['const'] = 9
schema_path.write_text(json.dumps(schema, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')

text = lambda value: {'en': value, 'de': value}
excel = {'kind': 'excel', 'caption': text('Reference cells'), 'workbook': text('Studio.xlsx'), 'sheets': [text('Areas'), text('Costs')], 'activeSheet': 0, 'columns': ['A', 'B', 'C'], 'rows': [[text('Room'), text('Area'), text('Factor')], [text('Hall'), text('12'), text('2')], [text('Office'), text('8'), text('3')]], 'selection': {'row': 1, 'column': 2}, 'formula': text('=B2*2'), 'target': 'formula'}
gh = {'kind': 'grasshopper', 'caption': text('Given source tree'), 'nodes': [{'id': 'source', 'label': text('Numbers'), 'inputs': [], 'outputs': [{'id': 'numbers', 'label': text('N')}]}, {'id': 'operation', 'label': text('Operation'), 'inputs': [{'id': 'data', 'label': text('D')}], 'outputs': [{'id': 'result', 'label': text('R')}]}], 'wires': [{'from': {'node': 'source', 'port': 'numbers'}, 'to': {'node': 'operation', 'port': 'data'}}], 'tree': [{'path': '{0;0}', 'values': [text('1'), text('2')]}, {'path': '{0;1}', 'values': [text('3')]}], 'target': {'kind': 'wire', 'index': 0}}
cases = [{'id': 'excel', 'mockup': excel, 'valid': True, 'schemaValid': True}, {'id': 'grasshopper', 'mockup': gh, 'valid': True, 'schemaValid': True}]
from copy import deepcopy
def invalid(base, name, change, structural=True):
    value = deepcopy(base)
    change(value)
    cases.append({'id': name, 'mockup': value, 'valid': False, 'schemaValid': not structural})
invalid(excel, 'missing-language', lambda value: value['caption'].pop('de'))
invalid(excel, 'unknown-kind', lambda value: value.update(kind='browser'))
invalid(excel, 'solution-key', lambda value: value.update(expected=24))
invalid(excel, 'unknown-target', lambda value: value.update(target='ribbon'))
invalid(excel, 'selection-outside-grid', lambda value: value['selection'].update(row=10), False)
invalid(excel, 'nonrectangular-grid', lambda value: value['rows'][0].pop(), False)
invalid(gh, 'unknown-wire-port', lambda value: value['wires'][0]['to'].update(port='missing'), False)
invalid(gh, 'unknown-target-node', lambda value: value.update(target={'kind': 'node', 'index': 0, 'node': 'missing'}), False)
invalid(gh, 'duplicate-nodes', lambda value: value['nodes'].append(deepcopy(value['nodes'][0])), False)
invalid(gh, 'invalid-tree-path', lambda value: value['tree'][0].update(path='0:1'))
fixture = root / '🧫️fixtures/🖥️mockups/🔣️.json'
fixture.parent.mkdir(parents=True, exist_ok=True)
fixture.write_text(json.dumps(cases, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
