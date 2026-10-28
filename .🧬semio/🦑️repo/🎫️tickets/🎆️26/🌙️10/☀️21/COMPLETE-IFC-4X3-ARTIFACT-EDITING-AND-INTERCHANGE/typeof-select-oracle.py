"""🕸️ Verify complete SELECT graph closure with independent IFC metadata and NumPy Boolean transitive closure."""
import ctypes
import json
import pathlib
import re
import sys

ticket = pathlib.Path(__file__).resolve().parent
module_path = str(ticket / '🗑️generated' / 'oracle')
if sys.platform == 'win32':
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path, short, len(short))
    module_path = short.value or module_path
sys.path.insert(0, module_path)
import ifcopenshell
import numpy as np

class SelectGraphOracle:
    """📐 Supply independent direct SELECT edges and Boolean-matrix reachability."""
    def __init__(self, schema_name):
        schema = ifcopenshell.ifcopenshell_wrapper.schema_by_name(schema_name)
        self.names = sorted(declaration.name().upper() for declaration in schema.declarations())
        self.indices = {name: index for index, name in enumerate(self.names)}
        self.reachable = np.zeros((len(self.names), len(self.names)), dtype=np.bool_)
        self.edges = 0
        self.selects = 0
        self.parents = {}
        for declaration in schema.declarations():
            select = declaration.as_select_type()
            if select is None:
                continue
            self.selects += 1
            parent = self.indices[declaration.name().upper()]
            for member in select.select_list():
                name = member.name().upper()
                self.parents.setdefault(name, []).append(declaration.name().upper())
                self.reachable[self.indices[name], parent] = True
                self.edges += 1
        for index in range(len(self.names)):
            self.reachable |= self.reachable[:, index:index+1] & self.reachable[index:index+1, :]

    def close(self, seeds, schema_name):
        prefix = schema_name + '.'
        result = {prefix + seed if seed.startswith('IFC') and '.' not in seed else seed for seed in seeds}
        for seed in seeds:
            name = seed[len(prefix):] if seed.startswith(prefix) else seed
            if name not in self.indices:
                continue
            result.update(prefix + self.names[int(index)] for index in np.flatnonzero(self.reachable[self.indices[name]]))
        return result

output = ticket / '🗑️generated' / 'typeof-select'
owned = json.loads((output / 'owned.json').read_text('utf8'))
oracle = SelectGraphOracle(owned['schema'])
assert len(oracle.names) == 1312
for row in owned['rows']:
    expected = oracle.close(row['seeds'], owned['schema'])
    assert len(row['actual']) == len(set(row['actual'])) and set(row['actual']) == expected, row['id']
validation = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation'
source = (validation / 'select-parents.rs').read_text('utf8')
indexed = [(name, json.loads('[' + parents + ']')) for name, parents in re.findall(r'^\s*\("([^"]+)", &\[(.*)\]\),$', source, re.MULTILINE)]
expected_index = sorted((name, sorted(parents)) for name, parents in oracle.parents.items())
assert indexed == expected_index, 'Immutable direct SELECT parent index differs from independent metadata'
assert len(indexed) == 214 and sum(len(parents) for _, parents in indexed) == 271
constructors = json.loads((validation / 'constructor-cases.json').read_text('utf8'))
rows_by_id = {row['id']: row for row in owned['rows']}
for case in constructors:
    expected = oracle.close(rows_by_id[case['constructor']]['seeds'], owned['schema'])
    assert set(case['schemaTypes']) == {name for name in expected if name.startswith(owned['schema'] + '.')}, case['constructor']
report = {'implementations': {'IfcOpenShellMetadata': ifcopenshell.version, 'NumPyBooleanClosure': np.__version__}, 'schema': owned['schema'], 'namedSeeds': 1312, 'directApiCases': len(owned['rows']) - 1312, 'constructorExpectations': len(constructors), 'selects': oracle.selects, 'directEdges': oracle.edges, 'immutableIndexKeys': len(indexed), 'immutableIndexEdges': sum(len(parents) for _, parents in indexed), 'mismatches': 0, 'nativeExecution': False, 'ifcOpenShellTypeofUsedAsOracle': False}
(output / 'oracle.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf8')
print(json.dumps(report))

