"""🪞 Compare neutral immediate inverse vectors with an independent graph interpreter and IfcOpenShell operations."""
import copy
import ctypes
import json
import pathlib
import struct
import sys

ticket = pathlib.Path(__file__).resolve().parent
root = ticket.parents[6]
cases_path = root / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests'
fixture = json.loads((cases_path / 'progressive.json').read_text(encoding='utf-8'))
vectors = json.loads((cases_path / 'progressive-inverses.json').read_text(encoding='utf-8'))


def exact(value):
    if isinstance(value, float):
        return ('real-bits', struct.pack('>d', value).hex())
    if isinstance(value, dict):
        return tuple((key, exact(item)) for key, item in value.items())
    if isinstance(value, list):
        return tuple(exact(item) for item in value)
    return value


def apply(model, mutation):
    kind = mutation['mutation']
    inverse = copy.deepcopy(mutation)
    if kind in ('setFileDescription', 'setFileName', 'setFileSchema'):
        key = kind[3].lower() + kind[4:]
        inverse['values'] = copy.deepcopy(model['header'][key])
        model['header'][key] = copy.deepcopy(mutation['values'])
        return inverse
    if kind == 'insertEntity':
        model['entities'].insert(mutation['index'], copy.deepcopy(mutation['entity']))
        return {'mutation': 'removeEntity', 'id': mutation['entity']['id']}
    position = next(index for index, entity in enumerate(model['entities']) if entity['id'] == mutation['id'])
    entity = model['entities'][position]
    if kind == 'removeEntity':
        return {'mutation': 'insertEntity', 'index': position, 'entity': model['entities'].pop(position)}
    if kind in ('setEntityName', 'setEntityComplex'):
        key = 'name' if kind == 'setEntityName' else 'complex'
        inverse[key] = copy.deepcopy(entity[key])
        entity[key] = copy.deepcopy(mutation[key])
    elif kind == 'setEntityArg':
        inverse['value'] = entity['args'][mutation['index']]
        entity['args'][mutation['index']] = copy.deepcopy(mutation['value'])
    elif kind == 'insertEntityArg':
        entity['args'].insert(mutation['index'], copy.deepcopy(mutation['value']))
        inverse = {'mutation': 'removeEntityArg', 'id': mutation['id'], 'index': mutation['index']}
    elif kind == 'removeEntityArg':
        inverse = {'mutation': 'insertEntityArg', 'id': mutation['id'], 'index': mutation['index'], 'value': entity['args'].pop(mutation['index'])}
    else:
        raise AssertionError(kind)
    return inverse


rows = 0
for case, expected in zip(fixture['cases'], vectors['cases'], strict=True):
    model = copy.deepcopy(fixture['snapshot'])
    inverses = []
    for mutation, vector in zip(case['mutations'], expected, strict=True):
        inverse = apply(model, mutation)
        assert exact(inverse) == exact(vector), (case['name'], inverse, vector)
        inverses.append(inverse)
        rows += 1
    for inverse in reversed(inverses):
        apply(model, inverse)
    assert exact(model) == exact(fixture['snapshot']), case['name']

module_path = str(ticket / '🗑️generated/oracle')
if sys.platform == 'win32':
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path, short, len(short))
    module_path = short.value or module_path
sys.path.insert(0, module_path)


class IndependentModel:
    """🔬 Isolate the third-party model behind this test-only interface."""
    def __init__(self):
        import ifcopenshell
        self.model = ifcopenshell.file(schema='IFC4X3_ADD2')

    def organization(self, identity, label):
        return self.model.create_entity('IFCORGANIZATION', id=identity, Name=label)

    def collection(self, values):
        return self.model.create_entity('IFCPROPERTYLISTVALUE', Name='Preimages', ListValues=tuple(self.model.create_entity('IFCREAL', value) for value in values))


oracle = IndependentModel()
entity = oracle.organization(1, 'original')
first = entity.Name
entity.Name = 'first'
second = entity.Name
entity.Name = 'second'
assert first == 'original' and second == 'first'
entity.Name = second
entity.Name = first
assert entity.Name == 'original'
created = oracle.organization(3, 'created')
captured = created.Name
created.Name = 'edited'
assert captured == 'created'
oracle.model.remove(created)
assert {item.id() for item in oracle.model} == {1}
oracle.model.remove(entity)
replacement = oracle.organization(1, 'replacement')
replacement.Name = 'final'
assert replacement.id() == 1
oracle.model.remove(replacement)
entity = oracle.organization(1, 'original')
assert entity.Name == first
values = oracle.collection((-0.0, 2.0, 3.0, 4.0))
original = tuple(values.ListValues)
items = list(values.ListValues)
removed_first = items.pop(0)
removed_second = items.pop(1)
assert removed_first.wrappedValue == -0.0 and removed_second.wrappedValue == 3.0
items.insert(1, removed_second)
items.insert(0, removed_first)
values.ListValues = tuple(items)
assert [struct.pack('>d', item.wrappedValue).hex() for item in values.ListValues] == [struct.pack('>d', item.wrappedValue).hex() for item in original]
oracle.model.header.file_name.name = 'original'
oracle.model.header.file_name.name = 'first'
header_preimage = oracle.model.header.file_name.name
oracle.model.header.file_name.name = 'second'
assert header_preimage == 'first'
print(json.dumps({'neutralProgressiveBatches': len(fixture['cases']), 'exactImmediateInverseRows': rows, 'allUndoRealBitsPreserved': True, 'ifcOpenShellIndependentScenarios': 5, 'nativeCursorExecution': 'not executed', 'physicalComplexMappingConformance': 'not evaluated'}))
