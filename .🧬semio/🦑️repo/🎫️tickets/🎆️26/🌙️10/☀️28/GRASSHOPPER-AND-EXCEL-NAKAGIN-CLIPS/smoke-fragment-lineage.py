"""🧬 Check four original source planes using only the registered standard-library runtime."""
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import time
TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
BASE=ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/✂️fragments'
inputs=(BASE/'🐍️.py',BASE/'🔣️.schema.json',BASE/'🧪️cases/🔣️.json',Path(__file__))
def identities():
    """🔐 Bind exact implementation, schema, portable case and smoke input bytes."""
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in inputs}
before=identities()
spec=importlib.util.spec_from_file_location('stdlib_lineage_smoke',BASE/'🐍️.py')
K=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=K
spec.loader.exec_module(K)
case=json.loads(inputs[2].read_text(encoding='utf-8'))['lineage']
faces=tuple(K.Face(row['source'],row['polygon'],tuple(map(tuple,row['vertices'])),tuple(row['normal'])) for row in case['faces'])
events=[]
started=time.perf_counter()
def progress(event):
    """📣 Retain bounded original-plane partition work without rendering any geometry."""
    events.append(event)
    if event['phase']=='partition' and event['completed']>256:
        raise AssertionError('Four original planes failed to make partition progress')
ordered=K.ordered_fragments(faces,tuple(case['towards']),progress=progress,cancelled=lambda:time.perf_counter()-started>5)
normal_by_id={(face.source,face.polygon):face.normal for face in faces}
if not ordered or any(face.normal!=normal_by_id[(face.source,face.polygon)] for face in ordered):
    raise AssertionError('Source identity or normal lineage changed')
after=identities()
result={'runtime':sys.version,'executable':sys.executable,'input_faces':len(faces),'fragments':len(ordered),'partition_nodes':max((e['completed'] for e in events if e['phase']=='partition'),default=0),'elapsed':time.perf_counter()-started,'source_before':before,'source_after':after,'unchanged':before==after}
(TICKET/'🗑️generated/stdlib-lineage-smoke.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps({k:result[k] for k in ('runtime','input_faces','fragments','partition_nodes','elapsed','unchanged')}),flush=True)
raise SystemExit(0 if before==after else 1)
