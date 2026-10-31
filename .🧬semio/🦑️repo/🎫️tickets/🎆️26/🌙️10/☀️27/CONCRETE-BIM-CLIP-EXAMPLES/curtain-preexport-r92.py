import importlib,sys,json,hashlib,math
from pathlib import Path
from shapely.geometry import Polygon
from shapely.validation import explain_validity
r=next(p for p in Path(__file__).resolve().parents if (p/'package.json').exists());sys.path.insert(0,str(r));a=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=a.load_model();g=a.filling_geometry('CW-1',m);ps=g['parts'];rows=[]
for i in [12,48,212,213,263,425,426]:
 p=ps[i];poly=Polygon(p['outline']);rect=list(poly.minimum_rotated_rectangle.exterior.coords);edges=[math.dist(x,y) for x,y in zip(rect,rect[1:])];rows.append({'part':i,'source_part':p,'validity':explain_validity(poly),'area':poly.area,'minimum_rectangle_width':min(edges),'minimum_vertex_edge':min(math.dist(x,y) for x,y in zip(p['outline'],p['outline'][1:]+p['outline'][:1]))})
source=Path(a.__file__).with_name('🔣️.json');out={'source':hashlib.sha256(source.read_bytes()).hexdigest(),'part_count':len(ps),'cases':rows};Path(__file__).with_name('🗑️generated').joinpath('curtain-preexport-r92.json').write_text(json.dumps(out,ensure_ascii=False),encoding='utf8');print(json.dumps(out,ensure_ascii=False))
