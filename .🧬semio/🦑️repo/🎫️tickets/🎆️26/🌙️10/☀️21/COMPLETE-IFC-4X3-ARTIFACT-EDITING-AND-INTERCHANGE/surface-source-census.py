"""🪟️ Pin current surface admission and the shared declared-face/mesh-predicate factoring."""
from pathlib import Path
import hashlib
import json

ticket = Path(__file__).parent
base = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
io = base / '🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io'
owned = [io / name for name in ['geometry.ts', 'geometry/faces.ts', 'geometry/faceted-brep.ts', 'geometry/surface.ts']]
owned += [base / '📐️geometry/🧊️boolean/🟦️.ts']
owned += [io / '🧪️tests' / name for name in ['surface-fixtures.ts', 'surface-cases.json', 'surface.schema.json', 'surface.test.ts', 'surface.feature']]
refs = [io / name for name in ['geometry/source.ts', 'geometry/mapping.ts', 'control.ts', 'orientation.ts', 'step-values.ts', 'gltf.ts', '🧪️tests/faceted-brep.test.ts', '🧪️tests/faceted-shell.test.ts', '🧪️tests/mapping.test.ts', '🧪️tests/polygonal-neutral.test.ts', '🧪️tests/csg-block.test.ts', '🧪️tests/gltf.test.ts']]
refs += [base / '📐️geometry/🕳️rings/🟦️.ts']
refs += [base / '📐️geometry/🧊️boolean/🧪️tests' / name for name in ['🟦️.test.ts', 'admission.test.ts', 'manifold.test.ts', 'openings.test.ts']]

def row(path):
    """📌️ Observe actual bytes without duplicating checked source into the ticket."""
    return {'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'utf16PathUnits': len(str(path).encode('utf-16-le')) // 2}

result = {'owned': list(map(row, owned)), 'references': list(map(row, refs)), 'nativeExecuted': False, 'scope': 'Actual planar FaceBasedSurfaceModel source/geometry/edit/STEP/GLB/control, shared face factoring and normalized open triangle intersection validator. Native evaluators/full surfaces/all-MVD/output precision remain open.'}
assert all(item['utf16PathUnits'] <= 256 for item in result['owned'] + result['references'])
(ticket / '🗑️generated/surface-source.json').write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf-8')
