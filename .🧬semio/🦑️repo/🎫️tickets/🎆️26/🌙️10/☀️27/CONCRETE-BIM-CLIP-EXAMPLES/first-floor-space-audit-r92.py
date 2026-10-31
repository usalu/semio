import importlib,sys,json,hashlib,itertools
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
from shapely.validation import explain_validity
r=next(p for p in Path(__file__).resolve().parents if (p/'package.json').exists());sys.path.insert(0,str(r));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model();rows=[];polys={}
for s in m['spaces']:
 if s['storey']!='first':continue
 p=Polygon(a._expand(m,s['outline']));polys[s['tag']]=p;rows.append({'tag':s['tag'],'area':p.area,'valid':p.is_valid,'validity':explain_validity(p),'kind':s['predefined_type']})
overlap=[{'a':x,'b':y,'area':p.intersection(q).area} for (x,p),(y,q) in itertools.combinations(polys.items(),2) if p.intersection(q).area>1e-9];u=unary_union(list(polys.values()));out={'source':hashlib.sha256(Path(a.__file__).with_name('🔣️.json').read_bytes()).hexdigest(),'spaces':rows,'overlaps':overlap,'union_area':u.area,'sum_area':sum(p.area for p in polys.values()),'limits':['Pairwise programme polygon validity/overlap only; no unsupported claim of complete usable physical floor area or room-boundary historical precision.']};Path(__file__).with_name('🗑️generated').joinpath('first-floor-space-audit-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');print(json.dumps(out))
