"""⭕️ Pin represented circle sampling and paired fixed-size numerical source."""
from pathlib import Path
import hashlib
import json

ticket = Path(__file__).parent
base = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
io = base / '🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io'
circle = base / '📐️geometry/⭕️circle'

def row(path):
    """📌️ Observe actual source bytes and the256-unit path bound without copying source."""
    text = str(path)
    return {'path': text, 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'utf16PathUnits': len(text.encode('utf-16-le')) // 2}

owned = [circle / '🟦️.ts', circle / '🦀️.rs', base / '📐️geometry/🦀️.rs', io / 'geometry/circle.ts']
owned += [circle / '🧪️tests' / name for name in ['radius-cases.json', 'radius.schema.json', 'radius.test.ts']]
owned += [io / '🧪️tests' / name for name in ['mapped-circle-cases.json', 'mapped-circle.schema.json', 'mapped-circle.test.ts']]
refs = [io / 'geometry.ts', io / 'geometry/control.ts', io / 'geometry/mapping.ts', io / 'control.ts', io / '🧪️tests/mapping-fixtures.ts', io / '🧪️tests/circle-extrusion.test.ts', io / '🧪️tests/mapping.test.ts', base / '📦️packages/🦀️rust/Cargo.toml', ticket / 'mapped-circle-core-tsconfig.json']
result = {'owned': list(map(row, owned)), 'references': list(map(row, refs)), 'scope': 'TS represented world sampling and paired fixed-size Rust source/test mounting; native owned exchange still has no circle evaluator and compiler/types/runtime are unproved.'}
assert all(item['utf16PathUnits'] <= 256 for item in result['owned'] + result['references'])
(ticket / '🗑️generated/mapped-circle-source.json').write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf-8')
