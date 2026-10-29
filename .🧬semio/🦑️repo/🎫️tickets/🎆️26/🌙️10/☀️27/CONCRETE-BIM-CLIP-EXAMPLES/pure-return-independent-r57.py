import importlib,json,sys,time
from pathlib import Path
p=Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES');progress=p/'🗑️generated/pure-return-progress-r57.json';progress.write_text(json.dumps({'phase':'import actual pure modules','state':'running'}))
np=importlib.import_module('numpy');pr=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives');core=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.📐️geometry');measure=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.📏️geometry')
issues=[];allowed=(str,int,float,bool,type(None),list,tuple,dict,set,frozenset)
def check(x,path):
 if type(x) not in allowed:issues.append({'path':path,'type':str(type(x))});return
 if type(x) in (list,tuple,set,frozenset):
  for i,y in enumerate(x):check(y,path+'/'+str(i))
 elif type(x) is dict:
  for k,y in x.items():check(k,path+'/key');check(y,path+'/'+str(k))
samples={'P':measure.P(np.array([np.float64(1),np.float64(2)])),'UP':pr.UP,'wall_frame':core.wall_frame({'start':[0.,0.],'end':[4.,0.],'thickness':.2},{})}
for key,value in samples.items():check(value,key)
progress.write_text(json.dumps({'phase':'actual building primitives','state':'running'}));started=time.monotonic();rows=pr.prims()
for i,row in enumerate(rows):
 for slot in row.__slots__:check(getattr(row,slot),'prims/'+str(i)+'/'+slot)
result={'primitive_count':len(rows),'strict_builtin_issues':issues,'manim_loaded':'manim' in sys.modules,'sample_types':{k:str(type(v)) for k,v in samples.items()},'seconds':time.monotonic()-started};(p/'🗑️generated/pure-return-independent-r57.json').write_text(json.dumps(result));progress.write_text(json.dumps({'phase':'complete','state':'PASS' if not issues else 'FAIL'}));print(json.dumps(result))
