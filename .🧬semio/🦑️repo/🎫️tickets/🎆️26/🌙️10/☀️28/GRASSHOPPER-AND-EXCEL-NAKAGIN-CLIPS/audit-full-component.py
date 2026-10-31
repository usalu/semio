"""🔎 Census exact detailed cutaway graph components without BSP or drawing."""
from pathlib import Path
import sys,importlib,json,hashlib,time,collections
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir());sys.path.insert(0,str(root))
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
T=Path(__file__).parent;out=T/'🗑️generated/full-component-census.json';began=time.monotonic();deadline=began+120
paths=[Path(V.__file__),Path(V._model.__file__),Path(V._model.__file__).with_name('🔣️.json'),Path(V._fragments.__file__)]
hashes=lambda:{p.relative_to(root).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
result={'source_start':hashes(),'components':[],'progress':[],'partition_called':False,'paint_called':False}
def save():
    result['elapsed']=time.monotonic()-began;out.write_text(json.dumps(result,indent=2),encoding='utf8')
class Stop(Exception):pass
original=V._section_order
def progress(row):
    result['progress'].append(dict(row));save();print('[DEBUG] '+json.dumps(row),flush=True)
def census(faces,towards,progress=None,cancelled=None):
    row={'faces':len(faces),'source_items':len({f.source for f in faces}),'source_polygons':len({(f.source,f.polygon) for f in faces}),'vertices':sum(len(f.vertices) for f in faces),'max_vertices':max(map(lambda f:len(f.vertices),faces),default=0),'sources':sorted({f.source for f in faces})};result['components'].append(row);save();print('[DEBUG] component '+json.dumps({k:v for k,v in row.items() if k!='sources'}),flush=True);return faces
V._fragments.ordered_fragments=census
V._restore_order=lambda items,recipes:[]
def section(items,view,*args,**kwargs):
    result['input_items']=len(items);result['input_polygons']=sum(len(i.polygons or (i.verts,)) for i in items);result['input_vertices']=sum(len(i.verts) for i in items);save()
    original(items,view,progress=progress,cancelled=lambda:time.monotonic()>deadline)
    result['graph_completed']=True;raise Stop()
V._section_order=section
try:V.NakaginDrawing().capsule_axon(V._model.capsule_types()[0],cutaway=True,bolts=True)
except Stop:result['outcome']='graph-census-complete-no-partition'
except V._fragments.FragmentCancelled:result['outcome']='deadline-before-complete'
except Exception as error:result['outcome']='error';result['error']=repr(error)
result['source_end']=hashes();result['unchanged']=result['source_start']==result['source_end'];save();print('[DEBUG] '+json.dumps({k:v for k,v in result.items() if k not in ('components','progress','source_start','source_end')}),flush=True)
