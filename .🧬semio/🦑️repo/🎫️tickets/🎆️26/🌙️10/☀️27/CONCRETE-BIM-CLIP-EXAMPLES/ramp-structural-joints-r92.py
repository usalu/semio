"""🔬 Prove all eight exclusive ramp and wearing products clear actual slab and column material."""
import importlib,json,sys,hashlib
from pathlib import Path
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");o=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️interfaces");m=a.load_model();rows=[]
source_path=Path(a.__file__).with_name('🔣️.json');semantic=lambda value:hashlib.sha256(json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest();loaded_source=semantic(m)
products={tag:[(i,p,a._material_bounds(p)) for i,p in enumerate(o.occupied(m,tag))] for tag in ('RM-1','RM-2','RF-1A','RF-1B','RF-2A','RF-2B','FIN-RAMP-INTERIOR','FIN-RAMP-EXTERIOR')}
def check(q,owner,metadata):
    bounds=a._material_bounds(q)
    for tag,parts in products.items():
        for index,p,box in parts:
            if not all(lo<hi2 and lo2<hi for (lo,hi),(lo2,hi2) in zip(box,bounds)):continue
            volume=o.common_volume(p,q)
            if volume>1e-10:rows.append({'product':tag,'source_part':index,'owner':owner,'volume':volume,'parts':[p,q],**metadata})
for tag in ('SL-1','SL-R'):
    slab=a._slab_geometry_entry(m,a._find(m,tag))
    for field in ('parts','shared_parts'):
        for index,q in enumerate(slab[field]):check(q,q.get('owner',q.get('member',tag)),{'aggregate':tag,'field':field,'target_part':index,'region':q.get('region')})
    print('[DEBUG] actual structural slab reviewed '+tag,flush=True)
for obj in m['elements']:
    if obj['class']=='IfcColumn':
        for q in a.column_geometry(m,obj)['parts']:check(q,obj['tag'],{'aggregate':obj['tag']})
(Path(__file__).parent/'🗑️generated/ramp-structural-joints-r92.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2),encoding='utf8')
for row in rows:print('[DEBUG] '+json.dumps({k:v for k,v in row.items() if k!='parts'}),flush=True)
print('[DEBUG] structural source '+hashlib.sha256(Path(a.__file__).with_name('🔣️.json').read_bytes()).hexdigest(),flush=True)
current_source=semantic(json.loads(source_path.read_text(encoding='utf8')))
(Path(__file__).parent/'🗑️generated/ramp-structural-joints-meta-r92.json').write_text(json.dumps({'loaded_semantic_sha256':loaded_source,'current_semantic_sha256':current_source,'stable_source':loaded_source==current_source,'positive_pairs':len(rows),'api_file_sha256':hashlib.sha256(Path(a.__file__).read_bytes()).hexdigest()},indent=2),encoding='utf8')
assert not rows,[(r['product'],r['owner'],r['volume']) for r in rows]
print('[DEBUG] all eight actual ramp/finish structural interfaces PASS',flush=True)
