"""🔺️ Pin actual staged triangulation admission and its shared input predicate interface."""
from pathlib import Path
import hashlib
import json

ticket = Path(__file__).parent
base = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
io = base / '🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io'
owned = [io / name for name in ['geometry.ts', 'geometry/triangulated.ts']]
owned += [base / '📐️geometry/🧊️boolean/🟦️.ts']
owned += [io / '🧪️tests' / name for name in ['tfs-admission-fixtures.ts', 'tfs-admission-cases.json', 'tfs-admission.schema.json', 'tfs-admission.test.ts', 'tfs-admission.feature']]
refs = [io / name for name in ['geometry/source.ts', 'geometry/mapping.ts', 'control.ts', 'orientation.ts', 'step-values.ts', 'gltf.ts', '🧪️tests/geometry-progress.test.ts', '🧪️tests/mapping.test.ts', '🧪️tests/surface.test.ts', '🧪️tests/faceted-brep.test.ts', '🧪️tests/faceted-shell.test.ts', '🧪️tests/polygonal-neutral.test.ts', '🧪️tests/gltf.test.ts', '🧪️tests/rectangle-control.test.ts']]
refs += [base / '📐️geometry/🧊️boolean/🧪️tests' / name for name in ['admission.test.ts', 'manifold.test.ts']]
refs += [io.parent / '🧪️fixtures/official' / name for name in ['triangulated-item.ifc', 'slab-tessellated-unique-vertices.ifc', 'beam-straight-i-shape-tessellated.ifc', 'beam-curved-i-shape-tessellated.ifc']]

def row(path):
    """📌️ Observe actual source bytes without copying versioned input into a ticket."""
    return {'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'utf16PathUnits': len(str(path).encode('utf-16-le')) // 2}

result = {'owned': list(map(row, owned)), 'references': list(map(row, refs)), 'nativeExecuted': False, 'scope': 'Declared TFS admission, distinct coordinate/normal domains, exact private Closed geometry, actual world-output nondegeneracy, canonical edits/inverses, STEP/GLB and original controls. Normal transport, native owned execution and full MVD certification remain open.'}
assert all(item['utf16PathUnits'] <= 256 for item in result['owned'] + result['references'])
(ticket / '🗑️generated/tfs-admission-source.json').write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf-8')
