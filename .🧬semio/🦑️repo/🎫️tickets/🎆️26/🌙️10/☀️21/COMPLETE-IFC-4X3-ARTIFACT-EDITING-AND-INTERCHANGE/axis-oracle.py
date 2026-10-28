"""🧭 Compare placement admission and frames with independent EXPRESS validation and arbitrary precision."""
import ctypes
import json
import math
import pathlib
import sys

ticket = pathlib.Path(__file__).resolve().parent
module_path = str(ticket / '🗑️generated' / 'oracle')
if sys.platform == 'win32':
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path, short, len(short))
    module_path = short.value or module_path
sys.path.insert(0, module_path)
sys.path.insert(0, str(ticket / '🗑️generated' / 'determinant'))
import ifcopenshell
import ifcopenshell.validate
from ifcopenshell.util.placement import get_axis2placement
import mpmath

mpmath.mp.prec = 4096
output = ticket / '🗑️generated' / 'axis'
owned = json.loads((output / 'owned.json').read_text('utf8'))
results = []

def unit(vector):
    values = mpmath.matrix([mpmath.mpf(float(n)) for n in vector])
    if len(values) == 2:
        values = mpmath.matrix([*values, 0])
    return values / mpmath.sqrt(sum(n*n for n in values))

def cross(a, b):
    return mpmath.matrix([a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]])

for row in owned['rows']:
    model = ifcopenshell.open(str(output / (row['id'] + '.ifc')))
    logger = ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(model, logger, express_rules=True)
    errors = len(logger.statements)
    if 'error' in row:
        assert errors > 0 and row['actualError'] == row['error'], (row['id'], logger.statements)
        results.append({'id': row['id'], 'independentRefusal': True, 'expressErrors': errors})
        continue
    z = unit(row['axis']['ratios']) if row['axis'] else mpmath.matrix([0,0,1])
    x = unit(row['ref']['ratios']) if row['ref'] else mpmath.matrix([1,0,0])
    y = unit(list(cross(z, x)))
    x = unit(list(cross(y, z)))
    origin = row['origin']['ratios'] + ([0] if row['dimension'] == 2 else [])
    high = [float(n) for axis in [x,y,z] for n in [*axis,0]] + origin + [1]
    for a, b, prescribed in zip(row['actual'], high, row['matrix']):
        assert math.isclose(a,b,rel_tol=1e-14,abs_tol=1e-14) and math.isclose(prescribed,b,rel_tol=1e-14,abs_tol=1e-14), row['id']
    independent_matrix = get_axis2placement(model.by_id(row['placementId']))
    flat = [float(independent_matrix[r,c]) for c in range(4) for r in range(4)]
    finite = all(math.isfinite(n) for n in flat)
    if finite:
        assert all(math.isclose(a,b,rel_tol=1e-14,abs_tol=1e-14) for a,b in zip(flat,high)), row['id']
    if row['id'] not in ['large-finite','subnormal']:
        assert finite and errors == 0, (row['id'], logger.statements)
    results.append({'id': row['id'], 'mpmathEqual': True, 'ifcOpenShellFiniteMatrix': finite, 'expressErrors': errors})
report = {'implementations': {'IfcOpenShell': ifcopenshell.version, 'mpmath': mpmath.__version__}, 'precisionBits': mpmath.mp.prec, 'cases': results, 'mismatches': 0, 'nativeExecution': False}
(output / 'oracle.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf8')
print(json.dumps(report))

