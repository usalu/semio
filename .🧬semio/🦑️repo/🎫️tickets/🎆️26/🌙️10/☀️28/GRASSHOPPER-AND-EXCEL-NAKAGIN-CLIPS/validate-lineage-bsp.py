"""🧪 Validate cached actual BSP work, conserved material and independent nearest rays."""
from pathlib import Path
import hashlib,importlib.util,json,sys,time
import numpy as np
T=Path(__file__).resolve().parent
R=next(p for p in T.parents if (p/'🧰️framework').is_dir())
P=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/✂️fragments/🐍️.py'
I=T/'🗑️generated/largest-component-faces.json'
spec=importlib.util.spec_from_file_location('actual_kernel',P);K=importlib.util.module_from_spec(spec);sys.modules[spec.name]=K;spec.loader.exec_module(K)
data=json.loads(I.read_text());faces=tuple(K.Face(f['source'],f['polygon'],tuple(map(tuple,f['vertices'])),tuple(f['normal'])) for f in data['faces'])
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
before={str(p):sha(p) for p in (P,I,Path(__file__))}
counts={'splits':0,'nodes':0,'classifications':0,'prepared':0}
original_split=K._split
original_prepare=K._prepare
def prepare(*args):
    """🧩 Count actual prepared convex source material."""
    rows=original_prepare(*args);counts['prepared']+=len(rows);return rows
def split(*args):
    """⚔️ Count true material splits independently of progress events."""
    row=original_split(*args);counts['classifications']+=1;counts['splits']+=row[0]==2;return row
def progress(row):
    """📉 Require every BSP node to consume at least one extant material fragment."""
    if row['phase']=='partition':
        counts['nodes']=row['completed']
        if counts['nodes']>counts['prepared']+counts['splits']:
            raise AssertionError('BSP node did not consume material')
def areas(rows):
    """📐 Independently sum local NumPy triangle areas by source boundary."""
    result={}
    for f in rows:
        v=np.asarray(f.vertices);a=float(np.linalg.norm(np.cross(v[1:-1]-v[0],v[2:]-v[0]).sum(axis=0))/2)
        assert len(v)>=3 and a>0,('Nonpositive material boundary',f.source,f.polygon,len(v),a)
        key=(f.source,f.polygon);result[key]=result.get(key,0)+a
    return result
K._split=split;K._prepare=prepare
start=time.monotonic();outcome='failed';error=None;ordered=()
try:
    ordered=K.ordered_fragments(faces,data['towards'],progress=progress)
    a,b=areas(faces),areas(ordered)
    assert a.keys()==b.keys()
    errors={str(k):abs(a[k]-b[k]) for k in a}
    assert max(errors.values())<1e-10
    outcome='completed'
except Exception as exc:error=repr(exc)
after={str(p):sha(p) for p in (P,I,Path(__file__))}
result={'outcome':outcome,'error':error,'elapsed':time.monotonic()-start,'input_faces':len(faces),'output_pieces':len(ordered),'counts':counts,'maximum_area_error':max(errors.values()) if outcome=='completed' else None,'before':before,'after':after,'unchanged':before==after}
folder=T/'🗑️generated/lineage-bsp';folder.mkdir(exist_ok=True)
(folder/(sys.argv[1]+'.json')).write_text(json.dumps(result,indent=2),encoding='utf8')
print('[DEBUG] '+json.dumps(result),flush=True)
raise SystemExit(0 if outcome=='completed' and before==after else 1)
