"""🔬 Compare language-neutral details declarations and GlobalId encoding with IfcOpenShell."""
import ctypes
import json
import pathlib
import struct
import sys

ticket = pathlib.Path(__file__).resolve().parent
module_path = str(ticket / '🗑️generated/oracle')
if sys.platform == 'win32':
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path, short, len(short))
    module_path = short.value or module_path
sys.path.insert(0, module_path)
import ifcopenshell
import ifcopenshell.guid
import ifcopenshell.validate
from ifcopenshell import ifcopenshell_wrapper

base = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any'
cases = json.loads((base / '✏️editor/tests.json').read_text(encoding='utf-8'))
schema = ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2')
for case in cases['cases']:
    attributes = schema.declaration_by_name(case['entity']).all_attributes()
    assert attributes[case['position']].name() == case['attribute'], case
    if 'optional' in case:
        assert attributes[case['position']].optional() == case['optional'], case
assert 'STANDARD' in schema.declaration_by_name('IFCWALLTYPEENUM').enumeration_items()
for component in cases['complexTypes']:
    declaration = schema.declaration_by_name(component['entity'])
    assert len(declaration.attributes()) == component['attributes'], component
    assert declaration.is_abstract() == component['abstract'], component
switch = cases['typeSwitch']
old_attributes = schema.declaration_by_name(switch['from']).all_attributes()
new_attributes = schema.declaration_by_name(switch['to']).all_attributes()
assert len(old_attributes) == switch['fromAttributes']
assert len(new_attributes) == switch['toAttributes']
assert switch['fromEnum'] in schema.declaration_by_name('IFCWALLTYPEENUM').enumeration_items()
assert switch['fromEnum'] not in schema.declaration_by_name('IFCDOORTYPEENUM').enumeration_items()
assert new_attributes[8].optional() and switch['resetToUnset']
for name in switch['preserved']:
    assert name in [attribute.name() for attribute in old_attributes]
    assert name in [attribute.name() for attribute in new_attributes]
switch_model = ifcopenshell.file(schema='IFC4X3_ADD2')
old = switch_model.create_entity(switch['from'], id=1, GlobalId='0000000000000000000000', Name='Opening', Tag='Element 1')
replacement = switch_model.create_entity(switch['to'], **{name: getattr(old, name) for name in switch['preserved']})
for name in switch['preserved']:
    assert getattr(replacement, name) == getattr(old, name)
assert old.id() == 1
for edit in cases['realEdits']:
    before = struct.unpack('>d', bytes.fromhex(edit['before']))[0]
    after = struct.unpack('>d', bytes.fromhex(edit['after']))[0]
    point = switch_model.create_entity('IFCCARTESIANPOINT', Coordinates=(before,))
    assert struct.pack('>d', point.Coordinates[0]).hex() == edit['before']
    point.Coordinates = (after,)
    assert struct.pack('>d', point.Coordinates[0]).hex() == edit['after']
for vector in cases.get('globalIds', []):
    assert ifcopenshell.guid.compress(vector['hex']) == vector['compressed'], vector
insertion = cases['referenceInsertion']
model = ifcopenshell.file(schema='IFC4X3_ADD2')
group = model.create_entity(insertion['entity'], GlobalId='0000000000000000000000', Name='Reference Set')
properties = [model.create_entity(insertion['itemEntity'], Name=f'Property {entity_id}') for entity_id in range(insertion['firstId'], insertion['lastId'] + 1)]
assert properties[-1].id() == insertion['lastId']
setattr(group, insertion['attribute'], tuple(properties[:insertion['existingLastId'] - insertion['firstId'] + 1]))
target = model.by_id(insertion['targetId'])
setattr(group, insertion['attribute'], getattr(group, insertion['attribute']) + (target,))
assert len(getattr(group, insertion['attribute'])) == 257
assert getattr(group, insertion['attribute'])[-1].id() == insertion['targetId']
logger = ifcopenshell.validate.json_logger()
ifcopenshell.validate.validate(model, logger, express_rules=True)
assert not logger.statements, logger.statements
derived = json.loads((base / '✏️editor/derived.json').read_text(encoding='utf-8'))
for case in derived['cases']:
    declaration = schema.declaration_by_name(case['after'] or 'IFCNAMEDUNIT')
    attributes = declaration.all_attributes()
    index = next(index for index, attribute in enumerate(attributes) if attribute.name() == derived['attribute']['name'])
    assert declaration.derived()[index] == case['derived'], case
unit_model = ifcopenshell.file(schema='IFC4X3_ADD2')
dimensions = unit_model.create_entity('IFCDIMENSIONALEXPONENTS', 1, 0, 0, 0, 0, 0, 0)
si = unit_model.create_entity('IFCSIUNIT', UnitType='LENGTHUNIT', Name='METRE')
factor = unit_model.create_entity('IFCMEASUREWITHUNIT', ValueComponent=unit_model.create_entity('IFCLENGTHMEASURE', 1.0), UnitComponent=si)
conversion = unit_model.create_entity('IFCCONVERSIONBASEDUNIT', Dimensions=dimensions, UnitType='LENGTHUNIT', Name='Custom length', ConversionFactor=factor)
assert '*' in si.to_string()
assert conversion.Dimensions == dimensions
assert dimensions.id() > 0 and conversion.Dimensions.id() == dimensions.id()
unit_logger = ifcopenshell.validate.json_logger()
ifcopenshell.validate.validate(unit_model, unit_logger, express_rules=True)
assert not unit_logger.statements, unit_logger.statements
print(json.dumps({'contextualDerivedCases':len(derived['cases']),'ordinaryUnitExpressErrors':len(unit_logger.statements),'physicalComplexMappingConformance':'not evaluated'}))
retained = json.loads((base / '✏️editor/retained.json').read_text(encoding='utf-8'))
large_model = ifcopenshell.file(schema='IFC4X3_ADD2')
argument_case = retained['argument']
values = tuple(large_model.create_entity('IFCTEXT', 'x' * argument_case['textBytes']) for _ in range(argument_case['count']))
large = large_model.create_entity(argument_case['entity'], Name='Large', ListValues=values)
assert large.ListValues[argument_case['editedIndex']].wrappedValue == 'x' * argument_case['textBytes']
updated = list(large.ListValues)
updated[argument_case['editedIndex']] = large_model.create_entity('IFCTEXT', argument_case['replacement'])
large.ListValues = tuple(updated)
assert large.ListValues[-1].wrappedValue == argument_case['replacement']
text = large_model.create_entity('IFCTEXT', 'x' * retained['typeRows']['existingTextBytes'])
single = large_model.create_entity(retained['typeRows']['entity'], Name='Large text', NominalValue=text)
assert len(single.NominalValue.wrappedValue) == retained['typeRows']['existingTextBytes']
custody = retained['reductionCustody']
old_declaration = schema.declaration_by_name(custody['from']).all_attributes()
new_declaration = schema.declaration_by_name(custody['to']).all_attributes()
old_index = next(index for index, attribute in enumerate(old_declaration) if attribute.name() == custody['attribute'])
new_index = next(index for index, attribute in enumerate(new_declaration) if attribute.name() == custody['attribute'])
assert old_index == 3 and new_index == 2
assert str(old_declaration[old_index].type_of_attribute()) == str(new_declaration[new_index].type_of_attribute())
old_entity = large_model.create_entity(custody['from'], GlobalId=ifcopenshell.guid.compress('00112233445566778899aabbccddeeff'), Name='Wall', Description='x' * custody['textBytes'])
new_entity = large_model.create_entity(custody['to'], Name=old_entity.Name, Description=old_entity.Description)
assert new_entity.Description == old_entity.Description
identity = retained['identityRepair']
empty_entity = large_model.create_entity(identity['entity'], GlobalId=identity['globalId'])
empty_entity.GlobalId = ifcopenshell.guid.compress('00000000000000000000000000000001')
assert len(empty_entity.GlobalId) == 22
empty_entity.GlobalId = identity['globalId']
assert empty_entity.GlobalId == ''
large_model.remove(empty_entity)
print(json.dumps({'custodyAttributePositions':[old_index,new_index],'retainedDescriptionBytes':len(new_entity.Description),'identityRepairAndUndo':True,'nativeCancellationPrefixes':'not executed'}))
request = {'operation':'setValue','path':'/entities/0','value':{'$ifcType':retained['typeRows']['entity']}}
assert len(json.dumps(request, separators=(',', ':')).encode()) < retained['typeRows']['maximumCommandBytes']
large_logger = ifcopenshell.validate.json_logger()
ifcopenshell.validate.validate(large_model, large_logger, express_rules=True)
assert not large_logger.statements, large_logger.statements
print(json.dumps({'largeArgumentValues':argument_case['count'],'largeTextBytes':len(single.NominalValue.wrappedValue),'largeModelExpressErrors':len(large_logger.statements),'nativeCursorExecution':'not executed'}))
wire = json.loads((base / '../../../../🔎️details/🧪️tests/wire.json').resolve().read_text(encoding='utf-8'))
wire_lengths = []
for case in wire['cases']:
    directions = [wire['header'].encode() + json.dumps(case[direction], ensure_ascii=False, separators=(',', ':')).encode() for direction in ['operation', 'inverse']]
    assert all(json.loads(data[len(wire['header']):]) == case[direction] for data, direction in zip(directions, ['operation', 'inverse']))
    wire_lengths.append([len(data) for data in directions])
for text in wire['textRuns']:
    encoded = json.dumps(text, ensure_ascii=False, separators=(',', ':')).encode()
    assert json.loads(encoded) == text
oversized = {'mutation':'setEntityArg','id':1,'index':1,'value':{'kind':'string','value':'x' * wire['largeInverse']['textBytes']}}
assert len(wire['header'].encode() + json.dumps(oversized, separators=(',', ':')).encode()) > wire['largeInverse']['maximumWireBytes']
header_case = wire['headerTuple']
header_model = ifcopenshell.file(schema='IFC4X3_ADD2')
header_model.header.file_description.description = (header_case['before'], 'x' * header_case['commentBytes'])
header_model.header.file_description.description = (header_case['after'], header_model.header.file_description.description[1])
assert header_model.header.file_description.description == (header_case['after'], 'x' * header_case['commentBytes'])
print(json.dumps({'publicationWireBytes':wire_lengths,'jsonEscapingCases':len(wire['textRuns']),'oversizedInverseRefused':True,'nativeWireCursor':'not executed'}))
print(json.dumps({'implementation':'IfcOpenShell','version':ifcopenshell.version,'attributeCases':len(cases['cases']),'globalIds':len(cases.get('globalIds', [])),'status':'passed'}))
print(json.dumps({'referenceInsertion': {'entities': len(list(model)), 'references': len(getattr(group, insertion['attribute'])), 'lastId': target.id(), 'expressErrors': len(logger.statements)}}))
print(ifcopenshell.guid.compress('00112233445566778899aabbccddeeff'))
print(ifcopenshell.guid.compress('ffffffffffffffffffffffffffffffff'))

receiving = json.loads((base / '../../../../🔎️details/🧪️tests/receive.json').resolve().read_text(encoding='utf-8'))
import struct
receiving_model = ifcopenshell.file(schema='IFC4X3_ADD2')
receiving_oracles = 0
for case in receiving['cases']:
    expected = case['expected']
    if expected['kind'] == 'integer':
        independent = receiving_model.create_entity('IFCINTEGER', int(case['input']['value']))
        assert independent.wrappedValue == expected['value']
        receiving_oracles += 1
    elif expected['kind'] == 'string':
        independent = receiving_model.create_entity('IFCTEXT', case['input']['value'])
        assert independent.wrappedValue == expected['value']
        receiving_oracles += 1
    elif 'realBits' in case:
        bits = int(case['realBits'])
        real = struct.unpack('>d', bits.to_bytes(8, 'big'))[0]
        independent = receiving_model.create_entity('IFCREAL', real)
        assert int.from_bytes(struct.pack('>d', independent.wrappedValue), 'big') == bits
        receiving_oracles += 1
    elif expected['kind'] == 'reference':
        assert int(case['input']['value']) == expected['value'] == 18446744073709551615
print(json.dumps({'receivingCases':len(receiving['cases']),'ifcOpenShellScalarOracles':receiving_oracles,'exactU64ReferenceOracle':'Python exact decimal and JSON only; native IFC entity IDs are not used','nativeReceivingExecution':'not executed'}))

value_edits = json.loads((base / '../../../../🔎️details/🧪️tests/value-edit.json').resolve().read_text(encoding='utf-8'))
edit_model = ifcopenshell.file(schema='IFC4X3_ADD2')
def simple_value(value):
    if value['kind'] == 'integer': return edit_model.create_entity('IFCINTEGER', value['value'])
    if value['kind'] == 'string': return edit_model.create_entity('IFCTEXT', value['value'])
    raise AssertionError('Unsupported independent simple value')
for case in value_edits['cases']:
    before, event, after = case['before'], case['event'], case['after']
    if before['kind'] == 'aggregate':
        prop = edit_model.create_entity('IFCPROPERTYLISTVALUE', Name=case['name'], ListValues=tuple(simple_value(value) for value in before['value']))
        values = list(prop.ListValues)
        parts = event['path'].strip('/').split('/')
        if event['kind'] == 'set' and len(parts) == 3:
            values[int(parts[1])].wrappedValue = event['value']
        elif event['kind'] == 'set': values = [simple_value(value) for value in event['value']]
        elif event['kind'] == 'insert': values.insert(int(parts[-1]), simple_value(event['value']))
        elif event['kind'] == 'remove': values.pop(int(parts[-1]))
        elif event['kind'] == 'move': values.insert(int(parts[-1]), values.pop(int(event['from'].split('/')[-1])))
        prop.ListValues = tuple(values)
        assert [value.wrappedValue for value in prop.ListValues] == [value['value'] for value in after['value']]
    elif event['path'] == '/value/name':
        original = edit_model.create_entity(before['value']['name'], before['value']['items'][0]['value'])
        changed = edit_model.create_entity(event['value'], original.wrappedValue)
        assert changed.is_a().upper() == after['value']['name']
        assert changed.wrappedValue == after['value']['items'][0]['value']
    else:
        original = edit_model.create_entity(before['value']['name'], tuple(value['value'] for value in before['value']['items']))
        values = list(original.wrappedValue)
        values.insert(int(event['path'].split('/')[-1]), values.pop(int(event['from'].split('/')[-1])))
        original.wrappedValue = tuple(values)
        assert list(original.wrappedValue) == [value['value'] for value in after['value']['items']]
large_edit = edit_model.create_entity('IFCTEXT', 'before')
large_edit.wrappedValue = 'x' * value_edits['largeInput']['textBytes']
assert len(large_edit.wrappedValue) == value_edits['largeInput']['textBytes']
print(json.dumps({'ifcOpenShellValueEdits':len(value_edits['cases']),'largeIncomingTextBytes':len(large_edit.wrappedValue),'headerCommentBytes':len(header_model.header.file_description.description[1]),'nativeValueEditExecution':'not executed'}))

preimages = json.loads((base / '../../../../🔎️details/🧪️tests/preimages.json').resolve().read_text(encoding='utf-8'))
preimage_flags = {case['name']:case['admitted'] for case in preimages['cases']}
progressive = preimages['progressiveOracle']
preimage_model = ifcopenshell.file(schema='IFC4X3_ADD2')
original_ids = {entity.id() for entity in preimage_model}
created = preimage_model.create_entity(progressive['entity'], Name=progressive['initial'])
assert created.id() not in original_ids
captured_name = created.Name
created.Name = progressive['first']
second_preimage = created.Name
created.Name = progressive['second']
assert second_preimage == progressive['first'] and second_preimage != captured_name
assert created.Name == progressive['second']
assert preimage_flags['duplicate argument slot'] and preimage_flags['insert then edit']
shifted = preimage_model.create_entity('IFCPROPERTYLISTVALUE', Name='Shifted', ListValues=tuple(preimage_model.create_entity('IFCINTEGER', value) for value in (1,2,3)))
captured_second = shifted.ListValues[1].wrappedValue
shifted.ListValues = shifted.ListValues[1:]
immediate_second = shifted.ListValues[1].wrappedValue
assert captured_second == 2 and immediate_second == 3
assert preimage_flags['remove ascending']
preimage_model.header.file_name.name = 'original'
preimage_model.header.file_name.name = 'first'
header_preimage = preimage_model.header.file_name.name
preimage_model.header.file_name.name = 'second'
assert header_preimage == 'first' and preimage_flags['duplicate header tuple']
print(json.dumps({'independentProgressivePreimageScenarios':4,'neutralPreflightCases':len(preimages['cases']),'nativePreflightExecution':'not executed'}))
domain_case = wire['domainInsertion']
domain_model = ifcopenshell.file(schema='IFC4X3_ADD2')
domain_entity = domain_model.create_entity(domain_case['entity']['name'], Name=domain_case['entity']['args'][1]['value'])
assert domain_entity.id() == domain_case['entity']['id'] and len(list(domain_model)) == 1
domain_model.remove(domain_entity)
assert len(list(domain_model)) == 0
assert domain_case['invalidArgumentCount'] < len(schema.declaration_by_name(domain_case['entity']['name']).all_attributes())
invalid_probe = domain_model.create_entity(domain_case['entity']['name'])
try:
    invalid_probe.Name = domain_case['invalidName']['value']
except (TypeError, RuntimeError):
    pass
else:
    raise AssertionError('IfcOpenShell accepted a numeric organization Name')
domain_model.remove(invalid_probe)
print(json.dumps({'ifcOpenShellDomainInsertionAndInverse':True,'nativeDomainCursorExecution':'not executed'}))

structural = json.loads((base / '../../../../🔎️details/🧪️tests/domain-structure.json').resolve().read_text(encoding='utf-8'))
for case in structural['cases']:
    admitted = True
    if case['operation'] == 'insert':
        part = case['part']
        declaration = schema.declaration_by_name(part['name'])
        attributes = declaration.attributes()
        admitted = len(attributes) == len(part['args'])
        probe = domain_model.create_entity(part['name'])
        if admitted:
            for attribute, value in zip(attributes, part['args']):
                if value['kind'] == 'unset':
                    continue
                try:
                    supplied = domain_model.by_id(value['value']) if value['kind'] == 'reference' else value['value']
                    setattr(probe, attribute.name(), supplied)
                except (TypeError, RuntimeError):
                    admitted = False
                    break
        domain_model.remove(probe)
    assert admitted == case['admitted'], case['name']
    if admitted:
        parts = list(case['before'])
        if case['operation'] == 'insert': parts.insert(case['index'], case['part'])
        elif case['operation'] == 'remove': parts.pop(case['index'])
        elif case['operation'] == 'move': parts.insert(case['index'], parts.pop(case['from']))
        derived_context = any(schema.declaration_by_name(part['name']).derived()[0] for part in parts)
        assert derived_context == case['derived'], case['name']
print(json.dumps({'ifcOpenShellStructuralDomainCases':len(structural['cases']),'creationUnsetPolicy':'explicit progressive placeholder','physicalComplexMappingConformance':'not evaluated','nativeStructuralGateExecution':'not executed'}))

owners = json.loads((base / '../../../../🔎️details/🧪️tests/source-owners.json').resolve().read_text(encoding='utf-8'))
borrowed = owners['borrowedWholeReplacement']
body_model = ifcopenshell.file(schema='IFC4X3_ADD2')
body_entity = body_model.create_entity('IFCORGANIZATION', Name='Borrowed preimage', Description='x' * borrowed['textBytes'])
body_preimage = body_entity.Description
body_entity.Description = borrowed['replacement']
assert body_entity.Description == borrowed['replacement'] and len(body_preimage) == borrowed['textBytes']
body_entity.Description = body_preimage
assert len(body_entity.Description) == borrowed['textBytes']
assert borrowed['maximumCapacityBytes'] < borrowed['textBytes']
print(json.dumps({'ifcOpenShellWholeReplacementAndInverseBytes':len(body_entity.Description),'nativeShortLoanBorrowedReplacement':'not executed','nestedContiguousCloneDenial':'native test authored, not executed'}))
