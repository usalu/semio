"""🏛️ Independently validate actual BIM frames and their one-sided-direction counterexamples."""
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
import ifcopenshell
import ifcopenshell.validate
from ifcopenshell.util.placement import get_axis2placement

output = ticket / '🗑️generated' / 'bim-axis'
contract = json.loads((output / 'owned.json').read_text('utf8'))
rows = []
for row in contract['cases']:
    model = ifcopenshell.open(str(output / (row['id'] + '.ifc')))
    logger = ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(model, logger, express_rules=True)
    assert not logger.statements, (row['id'], logger.statements)
    building = model.by_type('IfcBuilding')[0]
    frame = get_axis2placement(building.ObjectPlacement.RelativePlacement)
    angle = math.atan2(float(frame[1,0]), float(frame[0,0]))
    assert math.isclose(angle, row['angle'], abs_tol=1e-14)
    assert [float(frame[k,3]) for k in range(3)] == [1.,2.,4.]
    prior_errors = None
    if row['angle'] != 0:
        prior = ifcopenshell.open(str(output / (row['id'] + '.one-sided.ifc')))
        prior_logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(prior, prior_logger, express_rules=True)
        prior_errors = len(prior_logger.statements)
        assert prior_errors > 0 and any('AxisAndRefDirProvision' in str(message) for message in prior_logger.statements), (row['id'], prior_logger.statements)
    (output / (row['id'] + '.independent.ifc')).write_text(model.to_string(), encoding='utf8')
    rows.append({'id': row['id'], 'expressErrors': 0, 'rotation': angle, 'priorOneSidedErrors': prior_errors})
report = {'implementation': 'IfcOpenShell', 'version': ifcopenshell.version, 'rows': rows, 'mismatches': 0, 'nativeExecution': False}
(output / 'oracle.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf8')
print(json.dumps(report))

