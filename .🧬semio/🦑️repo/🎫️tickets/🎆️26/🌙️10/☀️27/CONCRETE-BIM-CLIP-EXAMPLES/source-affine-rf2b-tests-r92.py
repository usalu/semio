"""🪜 Validate the captured native RF2B publication after original-affine construction."""
import importlib,sys,json,time,hashlib
from pathlib import Path
from scipy.spatial import ConvexHull
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");start=time.monotonic();m=a.load_model();out=[]
for tag in ('RF-1A','RF-1B','RF-2A','RF-2B'):
 g=a.ramp_geometry(tag,m);parts=g['parts'];volume=0.;minimum=float('inf')
 for i,p in enumerate(parts):
  a._polyhedron_part(p['mesh']['vertices'],p['mesh']['faces']);v=a._polyhedron_volume(p);assert v>0,(tag,i,v);volume+=v;minimum=min(minimum,v)
 out.append({'tag':tag,'parts':len(parts),'volume':volume,'minimum':minimum});print('[DEBUG] '+json.dumps(out[-1]),flush=True)
json.dumps(out);(Path(__file__).parent/'🗑️generated/source-affine-ramp-green-r92.json').write_text(json.dumps({'rows':out,'seconds':time.monotonic()-start,'source':hashlib.sha256(Path(a.__file__).with_name('🔣️.json').read_bytes()).hexdigest()},indent=2),encoding='utf8');print('[DEBUG] source-affine native publication PASS',flush=True)
