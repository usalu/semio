"""🧊️ Freeze the exact reviewed visual amendment without replacing earlier source evidence."""
from datetime import datetime,timezone
import hashlib
import json
from pathlib import Path

TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
baseline=json.loads((TICKET/'frozen-source-boundary-amendment.md').read_text(encoding='utf-8').split('```json\n',1)[1].split('```',1)[0])
base='🎓️teaching/🏛️architecture/🏢️nakagin/'
allowed={base+name for name in ('🎨️visuals/🐍️.py','🧪️cases/🎨️visuals/🔣️.json','🧪️tests/🎨️visuals/🐍️.py')}
added=sorted(base+kind+'/🎨️visuals/✂️cutaway/'+name for kind,names in (('🧪️cases',('🔣️.json','🔣️.schema.json')),('🧪️tests',('🐍️.py','🟨️.cjs'))) for name in names)
paths=sorted([*baseline['files'],*added])
assert len(paths)==len(set(paths))==118

def read():
    """🔐️ Hash source bytes without normalizing code or line endings."""
    return {name:hashlib.sha256((ROOT/name).read_bytes()).hexdigest() for name in paths}

current=read()
assert current==read()
changed={name for name,old in baseline['files'].items() if current[name]!=old}
assert changed==allowed,changed^allowed
assert current[base+'🎨️visuals/🐍️.py']=='2aab84498619ba898d3ea581a9ba1ddaea12136ccd0246a173eeec614a582500'
canonical=json.dumps(current,ensure_ascii=False,sort_keys=True,separators=(',',':')).encode('utf-8')
record={'at':datetime.now(timezone.utc).isoformat(),'base114_sha256':baseline['sha256'],'amended_paths':sorted(changed),'added_paths':added,'files':current,'sha256':hashlib.sha256(canonical).hexdigest(),'unchanged':True}
destination=TICKET/'frozen-source-visual-amendment.md'
if destination.exists():
    previous=json.loads(destination.read_text(encoding='utf-8').split('```json\n',1)[1].split('```',1)[0])
    assert previous['files']==current
else:
    destination.write_text('# Frozen Source Visual Amendment\n\nThe inspected current images exposed two bounded rendering defects: centroid-based cutaway face removal and overlapping orthographic annotations. Exactly the renderer and existing visual case/test files changed from the114-path baseline; four dedicated cutaway contract/schema/test files were added. All other source bytes, physical geometry and clip consumers remain equal. Two raw-byte passes agree across all118 paths. Earlier source snapshots and runtime outcomes are retained with their original scopes. This freeze is an identity record, not a substitute for current runtime or image acceptance.\n\n```json\n'+json.dumps(record,ensure_ascii=False,indent=2)+'\n```\n',encoding='utf-8')
    generated=TICKET/'🗑️generated'
    (generated/'frozen-source-visual-amendment.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    close=TICKET/'production-close-list.md'
    old=close.read_text(encoding='utf-8')
    before,rest=old.rsplit('```json\n',1)
    array,after=rest.split('```',1)
    assert set(json.loads(array))==set(baseline['files'])
    note='## Current Visual Amendment:118 Paths\n\nThe current exact close array below contains69 Nakagin,24 Grasshopper,19 Excel and6 shared-kernel paths. Four dedicated cutaway contract/test files extend the prior114 census. See frozen-source-visual-amendment.md for the complete current byte map and the only three amended existing paths. Original census tables below remain chronological.\n\n'
    old=before+'```json\n'+json.dumps(paths,ensure_ascii=False,indent=2)+'\n```'+after
    old=old.replace('# Exact Scoped Production And Test Close-List\n\n','# Exact Scoped Production And Test Close-List\n\n'+note,1)
    close.write_text(old,encoding='utf-8')
print('[DEBUG] Current118 source files unchanged; freeze SHA256 '+record['sha256'])
