"""🌳 Trace real validated-parent cuts to the self-plane fragment witness."""
from pathlib import Path
import sys,importlib,json,time,hashlib
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir());sys.path.insert(0,str(root));K=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.✂️fragments.🐍️');G=Path(__file__).parent/'🗑️generated';raw=(G/'largest-component-faces.json').read_bytes();data=json.loads(raw);faces=tuple(K.Face(r['source'],r['polygon'],tuple(map(tuple,r['vertices'])),tuple(r['normal'])) for r in data['faces']);target=next(f for f in faces if(f.source,f.polygon)==(5860,0));paths={target.vertices:[]};original_plane=K._plane;original_split=K._split;state={'first':False,'nodes':0,'classifications':0};result={'kernel_sha256':hashlib.sha256(Path(K.__file__).read_bytes()).hexdigest(),'component_sha256':hashlib.sha256(raw).hexdigest(),'parent':data['faces'][faces.index(target)],'towards':data['towards']}
class Found(Exception):pass
def plane(vertices,normal,check):
 r=original_plane(vertices,normal,check);state['first']=True;return r
def split(face,origin,normal,epsilon):
 first=state['first'];state['first']=False;state['classifications']+=1;r=original_split(face,origin,normal,epsilon)
 if first:
  state['nodes']+=1;state['splitter']={'source':face.source,'polygon':face.polygon,'vertices':face.vertices,'normal':face.normal}
 if(face.source,face.polygon)==(5860,0):
  path=paths.get(face.vertices)
  if path is None:raise RuntimeError('Lost target lineage')
  if first and r[0] in(-1,1):
   result['cuts']=path;result['witness']={'vertices':face.vertices,'normal':face.normal,'origin':origin,'plane_normal':normal,'epsilon':epsilon,'relation':r[0]};raise Found()
  if r[0]==2:
   cut={'origin':origin,'normal':normal,'epsilon':epsilon,'splitter':state['splitter']}
   paths[r[1].vertices]=path+[{**cut,'side':'front'}];paths[r[2].vertices]=path+[{**cut,'side':'back'}]
 return r
K._plane=plane;K._split=split;start=time.monotonic()
try:K.ordered_fragments(faces,data['towards'],cancelled=lambda:time.monotonic()-start>10)
except Found:result['outcome']='ancestry-found'
except K.FragmentCancelled:result['outcome']='deadline'
result['elapsed']=time.monotonic()-start;result['nodes']=state['nodes'];result['classifications']=state['classifications'];(G/'bsp-lineage-witness.json').write_text(json.dumps(result,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in result.items() if k not in('parent','cuts','witness')}));print('[DEBUG] cutters '+str([(r['splitter']['source'],r['splitter']['polygon'],r['side']) for r in result.get('cuts',[])]))
