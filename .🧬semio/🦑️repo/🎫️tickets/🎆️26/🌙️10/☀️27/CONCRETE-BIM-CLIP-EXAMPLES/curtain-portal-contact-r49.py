"""🪟 Independently verify native curtain portal material contact after finite end-plane clipping."""
import hashlib,importlib,json,time
from pathlib import Path
from shapely.geometry import Polygon
ROOT=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
TICKET=Path(__file__).parent
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
model=json.loads((ROOT/'🧱️model/🔣️.json').read_text())
curtain=api.filling_geometry('CW-1',model)
def overlap(a,b):
    """📏 Integrate exact GEOS planar occupied overlap over the shared physical height."""
    height=min(a['z1'],b['z1'])-max(a['z0'],b['z0'])
    if height<=0: return 0.
    p,q=Polygon(a['outline'],a.get('holes',[])),Polygon(b['outline'],b.get('holes',[]))
    u,v=p.bounds,q.bounds
    if u[2]<=v[0] or v[2]<=u[0] or u[3]<=v[1] or v[3]<=u[1]: return 0.
    return p.intersection(q).area*height
rows=[]
for tag in ('D-ENTRY','D-SERVICE'):
    filling=api.filling_geometry(tag,model)
    values=[(overlap(a,b),i,j) for i,a in enumerate(curtain['parts']) for j,b in enumerate(filling['parts'])]
    maximum=max(values)
    positive=[row for row in values if row[0]>1e-12]
    rows.append({'tag':tag,'curtain_parts':len(curtain['parts']),'filling_parts':len(filling['parts']),'maximum_overlap':maximum,'positive_material_pairs':positive})
control=curtain['parts'][0]
assert overlap(control,control)>0
summary={'raw_source':hashlib.sha256((ROOT/'🧱️model/🔣️.json').read_bytes()).hexdigest(),'rows':rows,'self_overlap_control':overlap(control,control)}
(TICKET/'🗑️generated/curtain-portal-contact-r49.json').write_text(json.dumps(summary))
print('CURTAIN_PORTAL_CONTACT',json.dumps(summary))
assert all(not row['positive_material_pairs'] for row in rows)
