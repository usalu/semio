import heapq,importlib,inspect,json,pickle,time
from pathlib import Path
s=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🧩️surfaces.🐍️')
folder=Path(__file__).parent/'🗑️generated'
data=pickle.loads((folder/'bim-projection-current.pkl').read_bytes())
original=s._regions
def candidate_index(rings,atoms,check):
 starts=sorted((row[2][0],i) for i,row in enumerate(rings))
 ends=[];active=set();pos=0;result={}
 for a,b in sorted(atoms,key=lambda e:((e[0][0]+e[1][0])/2,e)):
  check();x=(a[0]+b[0])/2;y=(a[1]+b[1])/2
  while pos<len(starts) and starts[pos][0]<=x:
   i=starts[pos][1];active.add(i);heapq.heappush(ends,(rings[i][2][1],i));pos+=1
  while ends and ends[0][0]<x:
   _,i=heapq.heappop(ends);active.discard(i)
  result[(a,b)]=[rings[i] for i in sorted(active) if rings[i][2][2]<=y<=rings[i][2][3]]
 return result
text=inspect.getsource(original)
text=text.replace('    for index,(a,b)in enumerate(sorted(atoms)):', '    candidate_map=candidate_index(rings,atoms,check)\n    for index,(a,b)in enumerate(sorted(atoms)):')
text=text.replace('        for ring,sign,box,owned in rings:', '        for ring,sign,box,owned in candidate_map[(a,b)]:')
env=dict(s.__dict__);env['candidate_index']=candidate_index
exec(compile(text,'<temporary in-memory broadphase>','exec'),env)
candidate=env['_regions']
result=[]
for name,fn in (('baseline',original),('candidate',candidate)):
 s._regions=fn;start=time.monotonic()
 try:
  rows=s.projected_material_regions(data['facets'],data['matrix'])
  status='completed';count=len(rows)
 except ValueError as e:status=str(e);count=None
 result.append({'name':name,'seconds':round(time.monotonic()-start,3),'status':status,'regions':count})
print(json.dumps(result),flush=True)
(folder/'bim-surface-index-probe.json').write_text(json.dumps(result,indent=2),encoding='utf-8')
