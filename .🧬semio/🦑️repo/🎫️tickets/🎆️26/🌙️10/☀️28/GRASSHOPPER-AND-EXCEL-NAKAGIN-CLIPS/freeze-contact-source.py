"""🧊️ Record the precise numerical-contact and complete-reference amendment."""
from datetime import datetime,timezone
import hashlib
import json
from pathlib import Path

TICKET=Path(__file__).resolve().parent
ROOT=next(path for path in TICKET.parents if (path/'🧰️framework').is_dir())
previous=json.loads((TICKET/'frozen-source-visual-amendment.md').read_text(encoding='utf-8').split('```json\n',1)[1].split('```',1)[0])
base='🎓️teaching/🏛️architecture/🏢️nakagin/'
allowed={base+path for path in ('🎨️visuals/🐍️.py','🧪️cases/🎨️visuals/🔣️.json','🧪️tests/🎨️visuals/🐍️.py','🧪️cases/🎨️visuals/✂️cutaway/🔣️.json','🧪️cases/🎨️visuals/✂️cutaway/🔣️.schema.json','🧪️tests/🎨️visuals/✂️cutaway/🐍️.py','🧪️tests/🎨️visuals/✂️cutaway/🟨️.cjs')}

def snapshot():
    """🔐️ Hash the same complete source set without normalizing bytes."""
    return {name:hashlib.sha256((ROOT/name).read_bytes()).hexdigest() for name in previous['files']}

current=snapshot()
assert current==snapshot()
changed={name for name,old in previous['files'].items() if current[name]!=old}
assert changed==allowed,changed^allowed
assert current[base+'🎨️visuals/🐍️.py']=='4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2'
close=(TICKET/'production-close-list.md').read_text(encoding='utf-8')
listed=json.loads(close.rsplit('```json\n',1)[1].split('```',1)[0])
assert len(current)==len(listed)==118 and set(listed)==set(current)
canonical=json.dumps(current,ensure_ascii=False,sort_keys=True,separators=(',',':')).encode('utf-8')
record={'at':datetime.now(timezone.utc).isoformat(),'base118_sha256':previous['sha256'],'amended_paths':sorted(changed),'files':current,'sha256':hashlib.sha256(canonical).hexdigest(),'unchanged':True}
destination=TICKET/'frozen-source-contact-amendment.md'
if destination.exists():
    saved=json.loads(destination.read_text(encoding='utf-8').split('```json\n',1)[1].split('```',1)[0])
    assert saved['files']==current
else:
    text='# Frozen Source Contact And Hardware Amendment\n\nExactly seven existing paths changed from the prior 118-file visual freeze: the renderer, the root visual cases/test and four dedicated cutaway contract/test files. No physical model, fragment kernel, clip consumer or other source byte changed. The numerical repair reuses the existing per-source 64-ULP boundary equivalence and original endpoint identity, with no new area cutoff or kernel tolerance. Reference capsule views now include all four physical attachment shafts and heads by default; explicit suppression remains available.\n\nPortable contact/positive-remnant controls, all six families with both wall-cut directions, every emitted facet through public fragment preparation, and the previously failing actual shared-geometry render pass. Independent source audit permits the fresh all-family images and complete Visual22 validation to proceed; this byte freeze does not substitute for those pending gates. The exact production close-list remains the same 118 unique paths. Two raw-byte passes agree. Earlier snapshots remain intact with their historical scopes.\n\n```json\n'+json.dumps(record,ensure_ascii=False,indent=2)+'\n```\n'
    destination.write_text(text,encoding='utf-8')
    (TICKET/'🗑️generated/frozen-source-contact-amendment.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] Current118 contact/hardware source freeze SHA256 '+record['sha256'])
