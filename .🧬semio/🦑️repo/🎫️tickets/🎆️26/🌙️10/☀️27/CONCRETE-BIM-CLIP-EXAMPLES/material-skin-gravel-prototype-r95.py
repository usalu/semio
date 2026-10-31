"""🧪 Measure source gravel surface extraction before production integration."""
import importlib,json,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
root=Path.cwd();p=next(root.glob(".🧬semio/🦑️repo/🎫️tickets/🎆*26/🌙*10/☀*27/CONCRETE-BIM-CLIP-EXAMPLES"))
sys.path.insert(0,str(root));base="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip"
s=importlib.import_module(base+".🎨️visuals.📐️drafting.🧩️surfaces.🐍️")
a=importlib.import_module(base+".🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")
m=json.loads((root/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json").read_text(encoding="utf8"));e=next(e for e in m["elements"]if e["tag"]=="FIN-PILOTIS-GRAVEL")
facets=[{"rings":f[1],"normal":f[2],"material":f[3]}for part in e["geometry"]["parts"]for f in a._part_faces(part,"gravel")]
start=time.monotonic();last=[-1]
def progress(event):
    """📣 Retain finite phase progress without excessive console output."""
    phase=event["phase"];elapsed=int(time.monotonic()-start)
    if elapsed//10!=last[0]:
        last[0]=elapsed//10;print("[DEBUG]",elapsed,event,flush=True)
out=s.material_skin(facets,progress=progress)
receipt={"source_cells":len(e["geometry"]["parts"]),"input_facets":len(facets),"output_facets":len(out),"seconds":time.monotonic()-start}
(p/"🗑️generated"/"material-skin-gravel-prototype-r95.json").write_text(json.dumps(receipt,indent=2),encoding="utf8")
(p/"🗑️generated"/"material-skin-gravel-prototype-facets-r95.json").write_text(json.dumps(out),encoding="utf8")
print(receipt,flush=True)
