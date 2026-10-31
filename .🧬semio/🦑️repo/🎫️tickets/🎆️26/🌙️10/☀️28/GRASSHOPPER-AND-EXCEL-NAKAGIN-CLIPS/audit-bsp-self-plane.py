"""🧷 Retain the first actual splitter face that cannot leave its own partition."""
from pathlib import Path
import sys,importlib,json,time,hashlib
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir());sys.path.insert(0,str(root));K=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.✂️fragments.🐍️')
G=Path(__file__).parent/'🗑️generated';data=(G/(sys.argv[1] if len(sys.argv)>1 else 'largest-component-faces.json')).read_bytes();source=json.loads(data);faces=tuple(K.Face(r['source'],r['polygon'],tuple(map(tuple,r['vertices'])),tuple(r['normal'])) for r in source['faces']);original_plane=K._plane;original_split=K._split;state={'first':False,'nodes':0,'classifications':0,'kernel_sha256':hashlib.sha256(Path(K.__file__).read_bytes()).hexdigest(),'component_sha256':hashlib.sha256(data).hexdigest()}
class Found(Exception):pass
def plane(vertices,normal,check):
 row=original_plane(vertices,normal,check);state['first']=True;return row
def split(face,origin,normal,epsilon):
 state['classifications']+=1;first=state['first'];state['first']=False;row=original_split(face,origin,normal,epsilon)
 if first:
  state['nodes']+=1
  if row[0] in (-1,1):
   state['witness']={'source':face.source,'polygon':face.polygon,'vertices':face.vertices,'source_normal':face.normal,'splitter_origin':origin,'splitter_normal':normal,'epsilon':epsilon,'distances':[K._dot(K._sub(p,origin),normal) for p in face.vertices],'relation':row[0],'returned_face_unchanged':row[1] is face};raise Found()
 return row
K._plane=plane;K._split=split;began=time.monotonic()
try:K.ordered_fragments(faces,source['towards'],cancelled=lambda:time.monotonic()-began>10)
except Found:state['outcome']='self-plane-no-progress-found'
except K.FragmentCancelled:state['outcome']='deadline'
state['elapsed']=time.monotonic()-began;state.pop('first',None);(G/(sys.argv[2] if len(sys.argv)>2 else 'bsp-self-plane-witness.json')).write_text(json.dumps(state,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps(state))
