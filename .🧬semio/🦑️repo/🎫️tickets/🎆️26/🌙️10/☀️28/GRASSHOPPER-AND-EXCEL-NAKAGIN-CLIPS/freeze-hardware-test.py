"""🧊️ Bind the strengthened full-reference test without changing capture inputs."""
from datetime import datetime,timezone
import hashlib
import json
from pathlib import Path

TICKET=Path(__file__).resolve().parent
ROOT=next(path for path in TICKET.parents if (path/'🧰️framework').is_dir())
previous=json.loads((TICKET/'frozen-source-contact-amendment.md').read_text(encoding='utf-8').split('```json\n',1)[1].split('```',1)[0])

def snapshot():
    """🔐️ Read the same complete 118-path byte set."""
    return {name:hashlib.sha256((ROOT/name).read_bytes()).hexdigest() for name in previous['files']}

files=snapshot()
assert files==snapshot()
changed=[name for name in files if files[name]!=previous['files'][name]]
assert changed==['🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🎨️visuals/🐍️.py']
raw=json.dumps(files,ensure_ascii=False,sort_keys=True,separators=(',',':')).encode()
record={'at':datetime.now(timezone.utc).isoformat(),'base118_sha256':previous['sha256'],'amended_paths':changed,'files':files,'sha256':hashlib.sha256(raw).hexdigest(),'unchanged':True}
destination=TICKET/'frozen-source-hardware-test-amendment.md'
if destination.exists():
    saved=json.loads(destination.read_text(encoding='utf-8').split('```json\n',1)[1].split('```',1)[0])
    assert saved['files']==files
else:
    text='# Frozen Source Hardware Expectation Amendment\n\nThe complete visual suite found one historical test expectation that explicitly excluded attachment hardware from the default reference assembly. The approved default includes that hardware, so the assertion now requires every full physical source part. No production code, model, geometry, schema, capture input or other test byte changed. The focused default-hardware test already verifies all four heads and shafts in six families, their NumPy bounds and explicit suppression.\n\nThe failed first test is not a current pass; the complete Visual22 suite must rerun. The running capture does not declare or consume this test file: its exact 48-input map is unchanged, and its completed actual section worker passed against that map. The same 118 unique source/test paths remain in the close-list, with two matching byte passes below.\n\n```json\n'+json.dumps(record,ensure_ascii=False,indent=2)+'\n```\n'
    destination.write_text(text,encoding='utf-8')
    (TICKET/'🗑️generated/frozen-source-hardware-test-amendment.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] Current118 strengthened hardware-test freeze SHA256 '+record['sha256'])
