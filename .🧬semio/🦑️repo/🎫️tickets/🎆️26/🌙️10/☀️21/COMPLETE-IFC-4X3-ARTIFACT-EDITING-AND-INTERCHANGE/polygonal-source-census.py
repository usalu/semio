"""🔷️ Pin the actual polygon evaluator, source-preserving fixtures and exact vertex closure correction."""
from pathlib import Path
import hashlib
import json

ticket = Path(__file__).parent
base = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
io = base / '🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io'
owned = [io / name for name in ['geometry.ts', 'geometry/polygonal.ts', 'geometry/faceted-brep.ts']]
owned += [io / '🧪️tests' / name for name in ['polygonal-fixtures.ts', 'polygonal-neutral-cases.json', 'polygonal-neutral.schema.json', 'polygonal-neutral.test.ts', 'polygonal-cases.json', 'polygonal.schema.json', 'polygonal.test.ts', 'gltf.test.ts']]
refs = [io / name for name in ['geometry/source.ts', 'geometry/mapping.ts', 'control.ts', 'step-values.ts', 'gltf.ts', '🧪️tests/faceted-brep-fixtures.ts']]
refs += [base / name for name in ['📐️geometry/🕳️rings/🟦️.ts', '📐️geometry/🧊️boolean/🟦️.ts']]

def row(path):
    """📌️ Read a hash in place without duplicating checked source into the ticket."""
    return {'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'utf16PathUnits': len(str(path).encode('utf-16-le')) // 2}

result = {'owned': list(map(row, owned)), 'references': list(map(row, refs)), 'nativeExecuted': False, 'scope': 'Actual TypeScript polygon source/geometry/edit/STEP/GLB/control paths; exact geometric closure correction for planar faceted BRep. Native evaluator, full MVD and precision certificates remain unproved.'}
assert all(item['utf16PathUnits'] <= 256 for item in result['owned'] + result['references'])
(ticket / '🗑️generated/polygonal-source.json').write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf-8')
