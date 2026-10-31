from pathlib import Path
import importlib,sys,dataclasses,json
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir());sys.path.insert(0,str(ROOT))
M=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🐍️')
base=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin';cases=json.loads((base/'🧪️cases/📦️casing/🔣️.json').read_text(encoding='utf8'))
def bounds(part):
 return tuple(tuple(fn(p[k]for p in part.solid.vertices)for k in range(3))for fn in(min,max))
outputs=[]
for spec in cases['fitting_specs']:
 parameters={**dataclasses.asdict(M.capsule_type('A')),**spec}
 if spec and 'interior_width'not in spec:
  for key in('interior_width','interior_length','interior_height'):parameters.pop(key)
 usable=M.capsule_interior(parameters)
 for family in cases['families']:
  parts=list(M._reference_interior(parameters,family));bb=[bounds(p)for p in parts]
  failures=[parts[i].key for i,(lo,hi)in enumerate(bb)if any(lo[k]<[usable.left,usable.back,usable.bottom][k]-1e-7 or hi[k]>[usable.right,usable.front,usable.top][k]+1e-7 for k in range(3))]
  # Negative broad-phase support test: a disconnected AABB cannot have physical mesh contact.
  anchors={i for i,(lo,hi)in enumerate(bb)if min(abs(lo[2]-usable.bottom),abs(hi[2]-usable.top),abs(lo[0]-usable.left),abs(hi[0]-usable.right),abs(lo[1]-usable.back),abs(hi[1]-usable.front))<1e-7}
  adj=[set()for _ in parts]
  for i in range(len(parts)):
   for j in range(i):
    if all(bb[i][0][k]<=bb[j][1][k]+1e-7 and bb[j][0][k]<=bb[i][1][k]+1e-7 for k in range(3)):adj[i].add(j);adj[j].add(i)
  seen=set(anchors);todo=list(anchors)
  while todo:
   i=todo.pop()
   for j in adj[i]-seen:seen.add(j);todo.append(j)
  disconnected=[parts[i].key for i in range(len(parts))if i not in seen]
  outputs.append({'spec':spec,'family':family,'parts':len(parts),'outside_usable':failures,'broad_phase_unanchored':disconnected})
  if failures or disconnected:print('[DEBUG]',spec,family,'outside',failures,'unanchored',disconnected,flush=True)
OUT=Path(__file__).resolve().parent/'🗑️generated';(OUT/'interior-component-audit.json').write_text(json.dumps(outputs,indent=2));print('[DEBUG] cases',len(outputs),flush=True)
