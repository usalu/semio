"""🗺️ Observe faceted and mapped source custody without copying source files."""
from pathlib import Path
import hashlib
import json

ticket = Path(__file__).parent
base = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
io = base / '🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io'

def row(path):
    """📌️ Record current bytes and platform-independent path bound observations."""
    text = str(path)
    return {'path': text, 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'utf16PathUnits': len(text.encode('utf-16-le')) // 2}

owned = [io / 'geometry.ts'] + [io / 'geometry' / name for name in ['faceted-brep.ts', 'source.ts', 'mapping.ts']]
owned += [io / '🧪️tests' / name for name in ['faceted-brep.test.ts', 'faceted-brep-fixtures.ts', 'faceted-brep-cases.json', 'faceted-brep.schema.json', 'faceted-brep.feature', 'faceted-shell.test.ts', 'faceted-shell-cases.json', 'faceted-shell.schema.json', 'mapping.test.ts', 'mapping-fixtures.ts', 'mapping-cases.json', 'mapping.schema.json']]
refs = [base / '📐️geometry/🧊️boolean/🟦️.ts', base / '📐️geometry/🕳️rings/work.ts', base / '📐️geometry/🕳️rings/🟦️.ts', base / '📚️catalog/🔎️query/🟦️.ts', io / 'control.ts', io / 'orientation.ts', io / 'gltf.ts', ticket / 'faceted-mapping-tsconfig.json']
refs += [io.parent / '🧪️fixtures/official' / name for name in ['brep-model.ifc', 'basin-faceted-brep.ifc']]
result = {'owned': list(map(row, owned)), 'references': list(map(row, refs)), 'scope': '16 owned TS/declarative files: represented faceted solids with voids, 3D mapped leaf traversal and central controlled orientation. Existing Boolean/ring/query code is referenced, native proof remains false.'}
assert all(item['utf16PathUnits'] <= 256 for item in result['owned'] + result['references'])
(ticket / '🗑️generated/faceted-mapping-source.json').write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf-8')
