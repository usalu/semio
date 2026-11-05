"""🪪️ NumPy independently checks the portable canonical revision visibility cases."""
import json
from pathlib import Path
import numpy as np
root = next(parent for parent in Path(__file__).resolve().parents if (parent / '.git').exists())
fixture = root / '✏️s/🔌️plugins/🀄️wfc/🧪️tests/🧫️preview-revisions/🔣️.json'
cases = json.loads(fixture.read_text(encoding='utf-8'))['cases']
for case in cases:
    source, current = case['sourceRevision'], case['currentRevision']
    visible = current is not None and len(source) == 32 and bool(np.array_equal(np.asarray(source), np.asarray(current)))
    assert visible == case['visible'], case['id']
print(f'NumPy matched {len(cases)} portable preview revision cases')

envelope_file = root / '✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🧫️commit-envelope/🔣️.json'
envelope = json.loads(envelope_file.read_text(encoding='utf-8'))
width, height, depth = envelope['size']
coordinates = np.indices((depth, height, width)).reshape(3, -1).T
assignments = [dict(x=int(x), y=int(y), z=int(z), tileId=envelope['tileId']) for z, y, x in coordinates]
payload = dict(sourceRevision=envelope['sourceRevision'], assignments=assignments, contradiction=False, done=True, trace=[])
encoded = json.dumps(payload, ensure_ascii=False, separators=(',', ':')).encode('utf-8')
assert 65536 < len(encoded) < 262144
if 'expectedPayloadBytes' in envelope:
    assert len(encoded) == envelope['expectedPayloadBytes']
print(f'NumPy constructed {len(assignments)} grid3d assignments, canonical payload {len(encoded)} bytes')
