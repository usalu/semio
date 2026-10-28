"""🧱️ Pin the exact CSG dispatcher, owner-qualified primitive and original route contracts."""
from pathlib import Path
import hashlib
import json

ticket = Path(__file__).parent
base = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
io = base / '🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io'
owned = [io / name for name in ['geometry.ts', 'geometry/csg.ts']]
owned += [io / '🧪️tests' / name for name in ['csg-block-fixtures.ts', 'csg-block-cases.json', 'csg-block.schema.json', 'csg-block.test.ts', 'csg-block.feature', 'gltf.test.ts']]
refs = [io / name for name in ['geometry/source.ts', 'geometry/mapping.ts', 'control.ts', 'orientation.ts', 'step-values.ts', 'gltf.ts']]
refs += [base / '📐️geometry/🧱️primitive/🟦️.ts', io.parent / '🧪️fixtures/official/csg-primitive.ifc']

def row(path):
    """📌️ Read actual source hashes in place and enforce the cross-platform path bound."""
    return {'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'utf16PathUnits': len(str(path).encode('utf-16-le')) // 2}

result = {'owned': list(map(row, owned)), 'references': list(map(row, refs)), 'nativeExecuted': False, 'scope': 'Exact TypeScript IfcCsgSolid→IfcBlock public conversion/edit/STEP/GLB/control; existing shared paired primitive kernel. General CSG/Boolean trees and native owned evaluation remain open.'}
assert all(item['utf16PathUnits'] <= 256 for item in result['owned'] + result['references'])
(ticket / '🗑️generated/csg-block-source.json').write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf-8')
