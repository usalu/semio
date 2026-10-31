"""🧊️ Record complete interior and cutaway expectations without changing geometry."""
from datetime import datetime,timezone
import hashlib
import json
from pathlib import Path

TICKET=Path(__file__).resolve().parent
ROOT=next(path for path in TICKET.parents if (path/'🧰️framework').is_dir())
previous=json.loads((TICKET/'frozen-source-hardware-test-amendment.md').read_text(encoding='utf-8').split('```json\n',1)[1].split('```',1)[0])

def snapshot():
    """🔐️ Read the unchanged scoped source set and its two revised test inputs."""
    return {name:hashlib.sha256((ROOT/name).read_bytes()).hexdigest() for name in previous['files']}

files=snapshot()
assert files==snapshot()
changed=sorted(name for name in files if files[name]!=previous['files'][name])
expected=sorted('🎓️teaching/🏛️architecture/🏢️nakagin/'+name for name in ('🧪️cases/🎨️visuals/🔣️.json','🧪️tests/🎨️visuals/🐍️.py'))
assert changed==expected
assert files['🎓️teaching/🏛️architecture/🏢️nakagin/🎨️visuals/🐍️.py']=='4d1d855d72e36263b8fa79402de86abb437f869fa61bf93f65b8cca42bc7edc2'
raw=json.dumps(files,ensure_ascii=False,sort_keys=True,separators=(',',':')).encode()
record={'at':datetime.now(timezone.utc).isoformat(),'base118_sha256':previous['sha256'],'amended_paths':changed,'files':files,'sha256':hashlib.sha256(raw).hexdigest(),'unchanged':True}
destination=TICKET/'frozen-source-lining-test-amendment.md'
if destination.exists():
    saved=json.loads(destination.read_text(encoding='utf-8').split('```json\n',1)[1].split('```',1)[0])
    assert saved['files']==files
else:
    text='# Frozen Source Interior Test Amendment\n\nExactly the existing visual case and test changed from the prior 118-file snapshot. No production model, renderer, kernel or clip consumer changed. The old custom-interior assertion measured a roof/wall cutaway but expected the complete uncut lining span, an artifact of the former centroid-based face deletion. The strengthened test now independently verifies full source extents for automatic, explicit and typed interior controls, then verifies the rendered cut extents against the authored 0.15 m removal and lining thickness. Its current focused run passes in 138.044 seconds. No production geometry or tolerance was altered to satisfy a test.\n\nThe failed complete run also detected an unrelated shared drafting dependency update. That dependency lies outside this scoped changed-file census, is preserved, and will be included in the fresh runtime closure rather than rolled back. All prior drifted visual/capture runs remain non-passes. The exact close-list still contains the same 118 scoped paths. Two raw-byte passes agree below.\n\n```json\n'+json.dumps(record,ensure_ascii=False,indent=2)+'\n```\n'
    destination.write_text(text,encoding='utf-8')
    (TICKET/'🗑️generated/frozen-source-lining-test-amendment.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] Current118 lining-test freeze SHA256 '+record['sha256'])
