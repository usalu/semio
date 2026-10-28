"""🗺️ Pin shared cooperative solid validation source and evidence inputs."""
from pathlib import Path
import hashlib
import json

ticket = Path(__file__).parent
base = ticket.parents[6] / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc'
solid = base / '📐️geometry/🧊️boolean'

def row(path):
    """📌️ Observe hashes and path lengths without copying controlled source."""
    text = str(path)
    return {'path': text, 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'utf16PathUnits': len(text.encode('utf-16-le')) // 2}

owned = [solid / '🟦️.ts'] + [solid / '🧪️tests' / name for name in ['admission.test.ts', 'admission-cases.json', 'admission.schema.json']]
refs = [solid / '🧪️tests' / name for name in ['🟦️.test.ts', 'manifold.test.ts', 'openings.test.ts']]
refs += [base / '🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/control.ts', ticket / 'cooperative-solid-validation-tsconfig.json']
result = {'owned': list(map(row, owned)), 'references': list(map(row, refs)), 'scope': 'One pure cooperative TS topology validator drives existing synchronous Boolean admission/output and future admitted faceted topology. No native execution proof.'}
assert all(item['utf16PathUnits'] <= 256 for item in result['owned'] + result['references'])
(ticket / '🗑️generated/cooperative-solid-validation-source.json').write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf-8')
