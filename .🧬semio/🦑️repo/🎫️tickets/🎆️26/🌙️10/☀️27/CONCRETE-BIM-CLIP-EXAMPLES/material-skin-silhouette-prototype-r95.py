"""🔬 Measure exact projected material silhouettes of retained actual source facets."""
import importlib,json,sys,time
from pathlib import Path
from fractions import Fraction as F
sys.dont_write_bytecode=True;root=Path.cwd();sys.path.insert(0,str(root));ticket=Path(__file__).parent
s=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🧩️surfaces.🐍️")
facets=json.loads((ticket/"🗑️generated/material-skin-gravel-prototype-facets-r95.json").read_text(encoding="utf8"));facets=[f for f in facets if -f["normal"][0]-f["normal"][1]+f["normal"][2]>1e-6]
rings=[(tuple((F(p[0])-F(p[1]),F(p[0])+F(p[1])+2*F(p[2]))for p in ring),1 if i==0 else -1)for f in facets for i,ring in enumerate(f["rings"])]
started=time.monotonic();last=[-1]
def check():
    """🛑 Respect the operator's retained cooperative cancellation marker."""
    if (ticket/"🗑️generated/material-skin-silhouette-cancel-r95").exists():raise s.SurfaceCancelled()
def progress(phase,completed,total):
    """📣 Publish no more than one progress record per ten seconds."""
    elapsed=int(time.monotonic()-started)
    if elapsed//10!=last[0]:last[0]=elapsed//10;print("[DEBUG]",elapsed,phase,completed,total,flush=True)
out=s._regions(rings,check,progress);out={k:[{"outer":[[float(v)for v in p]for p in r["outer"]],"holes":[[[float(v)for v in p]for p in h]for h in r["holes"]]}for r in v]for k,v in out.items()}
receipt={"seconds":time.monotonic()-started,"facets":len(facets),"regions":out}
(ticket/"🗑️generated/material-skin-silhouette-prototype-r95.json").write_text(json.dumps(receipt),encoding="utf8");print("[DEBUG] complete",receipt["seconds"],[len(v)for v in out.values()],flush=True)
