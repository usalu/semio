"""📐 Validate every actual immutable capital polygon using local geometric planes only."""
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import numpy as np
TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
BASE=ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/✂️fragments'
INPUT=TICKET/'🗑️generated/capital-after-dependency-order/coverage-items.json'
OUTPUT=TICKET/'🗑️generated/fragment-capital-preparation.json'
FILES=[INPUT,BASE/'🐍️.py',BASE/'🔣️.schema.json',Path(__file__)]
def hashes():
    """🪪 Preserve exact immutable source/export and preparation-interface identities."""
    return {str(path.relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest() for path in FILES}
before=hashes()
spec=importlib.util.spec_from_file_location('capital_preparation_kernel',BASE/'🐍️.py')
K=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=K
spec.loader.exec_module(K)
rows=json.loads(INPUT.read_text(encoding='utf-8'))
errors=[]
count,fragments,raw_rejected=0,0,0
for row in rows:
    cursor=0
    for polygon,projection in enumerate(row['pixel_polygons']):
        vertices=tuple(map(tuple,row['world_vertices'][cursor:cursor+len(projection)]))
        cursor+=len(projection)
        count+=1
        try:
            K._prepare(K.Face(row['index'],polygon,vertices,tuple(row['normal'])),lambda:None)
        except ValueError:
            raw_rejected+=1
        points=np.asarray(vertices)
        vectors=points[1:]-points[0]
        cross=np.cross(vectors[:,None,:],vectors[None,:,:]).reshape(-1,3)
        normal=cross[np.argmax(np.linalg.norm(cross,axis=1))]
        normal/=np.linalg.norm(normal)
        if normal@row['normal']<0:
            normal=-normal
        try:
            prepared=K._prepare(K.Face(row['index'],polygon,vertices,tuple(normal)),lambda:None)
            fragments+=len(prepared)
        except Exception as error:
            errors.append({'source':row['index'],'polygon':polygon,'key':row['key'],'error':str(error)})
after=hashes()
report={'normal_basis':'independent NumPy maximum-area local cross, aligned to source orientation','raw_approximate_normals_rejected':raw_rejected,'polygons':count,'accepted':count-len(errors),'prepared_fragments':fragments,'errors':errors,'before':before,'after':after,'unchanged':before==after,'limits':'Preparation only. No BSP ordering, renderer invocation, Cairo capture or ray query.'}
OUTPUT.write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps({key:value for key,value in report.items() if key not in ('before','after')}),flush=True)
raise SystemExit(0 if not errors and report['unchanged'] else 1)
