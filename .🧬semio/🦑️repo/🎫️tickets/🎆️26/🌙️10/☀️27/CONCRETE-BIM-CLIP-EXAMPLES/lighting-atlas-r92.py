"""💡 Project original native light bodies for physical construction review."""
import hashlib,importlib,json,sys
from pathlib import Path
from PIL import Image,ImageDraw
from scipy.spatial import ConvexHull
sys.path.insert(0,str(Path.cwd()))
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
source=api.load_model();ticket=Path(__file__).parent
case=json.loads((Path(api.__file__).parent/"🧪️tests/💡️lights.json").read_text())
image=Image.new("RGB",(1800,2400),"white");draw=ImageDraw.Draw(image);manifest=[]
for index,row in enumerate(case["fixtures"]):
    body=api.service_geometry(row["tag"],source);verts=[v for p in body["parts"] for v in api.part_vertices(p)]
    axis=0;bounds=[min(v[axis] for v in verts),min(v[2] for v in verts),max(v[axis] for v in verts),max(v[2] for v in verts)]
    width,height=bounds[2]-bounds[0],bounds[3]-bounds[1];scale=min(790/max(width,1e-5),290/max(height,1e-5))
    left=40+(index%2)*900;top=60+(index//2)*390;ox=left+395-width*scale/2;oy=top+180+height*scale/2
    draw.text((left,top-30),row["tag"]+" | inferred dimensions / abgeleitete Masse",fill="#142c3e")
    for part in body["parts"]:
        vertices=api.part_vertices(part);hull=api._convex_hull([[v[axis],v[2]] for v in vertices]);points=[(ox+(x-bounds[0])*scale,oy-(z-bounds[1])*scale) for x,z in hull]
        draw.polygon(points,fill="#c6e1eb" if "GLASS" in part.get("material","") else "#aab3b9",outline="#354652")
    draw.rectangle((left-5,top-5,left+825,top+330),outline="#d7dfe5")
    manifest.append({"tag":row["tag"],"parts":len(body["parts"]),"bounds":bounds})
output=ticket/"🗑️generated"/"lighting-physical-r92.png";image.save(output)
(ticket/"🗑️generated"/"lighting-atlas-r92.json").write_text(json.dumps({"source_sha256":hashlib.sha256(Path(api.__file__).with_name("🔣️.json").read_bytes()).hexdigest(),"views":manifest},indent=2))
print("[DEBUG] "+str(output),flush=True)
