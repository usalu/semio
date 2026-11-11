import importlib,inspect,json,pickle,time
from pathlib import Path
s=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🧩️surfaces.🐍️')
folder=Path(__file__).parent/'🗑️generated'
data=pickle.loads((folder/'bim-projection-current.pkl').read_bytes())
stats={'rings':0,'atoms':0,'bounds_hits':0,'x_hits':0,'phases':{},'source':'retained 177-facet pre-fix float capture'}
text=inspect.getsource(s._regions)
needle='    for index,(a,b)in enumerate(sorted(atoms)):\n'
text=text.replace(needle,"    stats['rings']=len(rings);stats['atoms']=len(atoms)\n"+needle)
text=text.replace('            if not(box[0]<=mid[0]<=box[1]and box[2]<=mid[1]<=box[3]):continue',"            if box[0]<=mid[0]<=box[1]:stats['x_hits']+=1\n            if not(box[0]<=mid[0]<=box[1]and box[2]<=mid[1]<=box[3]):continue")
text=text.replace('            location=_r._location(mid,ring)',"            stats['bounds_hits']+=1\n            location=_r._location(mid,ring)")
env=dict(s.__dict__);env['stats']=stats
exec(compile(text,'<temporary in-memory instrumentation>','exec'),env)
s._regions=env['_regions']
start=time.monotonic()
def progress(event):
 stats['phases'].setdefault(event['phase'],{'first':round(time.monotonic()-start,3),'last':0,'completed':0,'total':0})
 row=stats['phases'][event['phase']];row.update(last=round(time.monotonic()-start,3),completed=event['completed'],total=event['total'])
try:
 rows=s.projected_material_regions(data['facets'],data['matrix'],progress=progress)
 stats['result']='completed';stats['regions']=len(rows)
except ValueError as e:stats['result']=str(e)
stats['seconds']=round(time.monotonic()-start,3)
(folder/'bim-surface-microprobe.json').write_text(json.dumps(stats,indent=2),encoding='utf-8')
print(json.dumps(stats),flush=True)
