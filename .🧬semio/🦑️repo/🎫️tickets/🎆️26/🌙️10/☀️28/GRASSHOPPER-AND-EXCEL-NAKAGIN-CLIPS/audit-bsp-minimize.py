"""🔬 Remove original inputs only while preserving the exact public-path sliver witness."""
from pathlib import Path
import sys,importlib,json,time,hashlib
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir());sys.path.insert(0,str(root));K=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.✂️fragments.🐍️');G=Path(__file__).parent/'🗑️generated';d=json.loads((G/'bsp-minimal-five-faces.json').read_text(encoding='utf8'));target=json.loads((G/'bsp-self-plane-witness.json').read_text(encoding='utf8'))['witness']['vertices'];op,os=K._plane,K._split
class Found(Exception):pass
def reproduces(rows):
 state={'first':False,'found':False};start=time.monotonic()
 def plane(*args):
  r=op(*args);state['first']=True;return r
 def split(face,*args):
  first=state['first'];state['first']=False;r=os(face,*args)
  if first and r[0] in(-1,1):state['found']=face.source==5860 and list(map(list,face.vertices))==target;raise Found()
  return r
 K._plane,K._split=plane,split
 try:K.ordered_fragments(tuple(K.Face(r['source'],r['polygon'],tuple(map(tuple,r['vertices'])),tuple(r['normal'])) for r in rows),d['towards'],cancelled=lambda:time.monotonic()-start>.1)
 except (Found,K.FragmentCancelled):pass
 finally:K._plane,K._split=op,os
 return state['found']
rows=d['faces'];attempts=[];changed=True
while changed:
 changed=False
 for i in range(len(rows)):
  candidate=rows[:i]+rows[i+1:];yes=reproduces(candidate);attempts.append({'removed_source':rows[i]['source'],'remaining':len(candidate),'same_witness':yes})
  if yes:rows=candidate;changed=True;break
out={'towards':d['towards'],'faces':rows};(G/'bsp-minimal-public-faces.json').write_text(json.dumps(out,indent=2),encoding='utf8');(G/'bsp-minimal-public-result.json').write_text(json.dumps({'kernel_sha256':hashlib.sha256(Path(K.__file__).read_bytes()).hexdigest(),'remaining_faces':len(rows),'attempts':attempts,'same_witness':reproduces(rows)},indent=2),encoding='utf8');print('[DEBUG]',len(rows),[(r['source'],len(r['vertices'])) for r in rows])
