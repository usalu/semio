"""🧭️ Freeze actual normal transport, exact cofactors, source domains and original control boundaries."""
from pathlib import Path
import hashlib
import json

ticket = Path(__file__).parent
base = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
io = base / '🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io'
owned = [io / name for name in ['geometry.ts', 'geometry/triangulated.ts', 'orientation.ts', 'gltf.ts', 'gltf-mesh.ts']]
owned += [io / '🧪️tests' / name for name in ['gltf-normals.test.ts', 'normal-matrix.test.ts', 'gltf-authored-normal-cases.json', 'gltf-authored-normal.schema.json', 'normal-matrix-cases.json', 'normal-matrix.schema.json', 'gltf-authored-normal.feature', 'normal-matrix.feature']]
refs = [io / name for name in ['geometry/source.ts', 'geometry/mapping.ts', 'control.ts', 'gltf-source.ts', 'gltf-json.ts', 'gltf-json-reader.ts', 'step-values.ts', '🧪️tests/tfs-admission.test.ts', '🧪️tests/tfs-admission-fixtures.ts', '🧪️tests/mapping-fixtures.ts', '🧪️tests/mapping-cases.json', '🧪️tests/gltf-samples.test.ts', '🧪️tests/gltf-accessor.test.ts', '🧪️tests/gltf.test.ts', '🧪️tests/geometry-progress.test.ts', '🧪️tests/mapping.test.ts', '🧪️tests/surface.test.ts', '🧪️tests/circle-extrusion.test.ts', '🧪️tests/gltf-samples/BoxInterleaved.glb', '🧪️tests/gltf-samples/NegativeScaleTest.glb', '🧪️tests/gltf-samples/provenance.json']]
refs += [base / '📐️geometry/🧊️boolean' / name for name in ['🟦️.ts', '🧪️tests/admission.test.ts']]

def row(path):
    """📌️ Observe current bytes and UTF16 path lengths without duplicating versioned source."""
    return {'path': str(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'utf16PathUnits': len(str(path).encode('utf-16-le')) // 2}

result = {'owned': list(map(row, owned)), 'references': list(map(row, refs)), 'nativeExecuted': False, 'scope': 'Actual authored-normal IFC/GLB interchange, distinct source point/normal domains, exact prepared covectors, byte/logical grants, projected float32 triangle admission and original caller failures. Native execution/full IFC/MVD/GLTF certification remains open.'}
assert all(item['utf16PathUnits'] <= 256 for item in result['owned'] + result['references'])
(ticket / '🗑️generated/gltf-normals-source.json').write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf-8')
