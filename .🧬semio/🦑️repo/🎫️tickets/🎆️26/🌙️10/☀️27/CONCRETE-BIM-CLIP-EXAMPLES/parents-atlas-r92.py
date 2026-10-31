"""🛁 Project actual suite composites and true material cross-sections for construction review."""
import importlib,json,sys,hashlib
from pathlib import Path
from PIL import Image,ImageDraw
sys.path.insert(0,str(Path.cwd()));a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");s=a.load_model();ticket=Path(__file__).parent
image=Image.new("RGB",(1900,1800),"white");ink=ImageDraw.Draw(image)
colors={"MAT-RC":"#a8adb2","MAT-MOSAIC-BLUE":"#6caab5","MAT-MOSAIC-BLACK":"#262c34","MAT-METAL":"#636f78","MAT-WOOD":"#d6c7ac","MAT-SANITARY-CERAMIC":"#eff2ee"}
tags=["FIX-BATH","FIX-CHAISE","FIX-PARENTS-WET-PLATFORM","FIX-DRESSING-STORAGE","BIDET-PARENTS","BASIN-PARENTS","FIX-PARENTS-STORAGE"]
bodies={tag:(a.service_geometry(tag,s) if a._find(s,tag)["class"]=="IfcSanitaryTerminal" else a.solid_geometry(tag,s)) for tag in tags}
def view(label,box,bounds,shapes):
 """📐 Fit native occupied polygons into a labelled diagnostic view."""
 l,t,r,b=box;x0,y0,x1,y1=bounds;scale=min((r-l)/(x1-x0),(b-t)/(y1-y0));ox=((r-l)-(x1-x0)*scale)/2;oy=(b-t)-((b-t)-(y1-y0)*scale)/2;canvas=Image.new("RGB",(r-l,b-t),"white");painter=ImageDraw.Draw(canvas)
 ink.text((l,t-25),label,fill="#183346")
 for ring,mat in shapes:
  if len(ring)>2:painter.polygon([(ox+(x-x0)*scale,oy-(y-y0)*scale) for x,y in ring],fill=colors.get(mat,mat if mat.startswith("#") else "#d7dce0"),outline="#52616b")
 image.paste(canvas,(l,t));ink.rectangle(box,outline="#ccd6dd")
def cut(part,axis,station):
 """🔪 Intersect actual convex material faces with one plane without filling service cavities."""
 v=a.part_vertices(part);faces=a.part_faces(part);points=[];axes=[k for k in range(3) if k!=axis]
 for face in faces:
  for i,j in zip(face,face[1:]+face[:1]):
   p,q=v[i],v[j];d=p[axis]-station;e=q[axis]-station
   if abs(d)<1e-9:points.append([p[k] for k in axes])
   if d*e<0:
    f=d/(d-e);points.append([p[k]+f*(q[k]-p[k]) for k in axes])
 return a._convex_hull(points) if len(points)>2 else []
shapes=[]
for room in s["spaces"]:
 if room["tag"] in ("SP-PARENTS","SP-PARENT-BATH","SP-FIRST-HALL","SP-SON"):
  shapes.append((a._expand(s,room["outline"]),"#edf4f7" if room["tag"]=="SP-FIRST-HALL" else "#f2eee7"))
for body in bodies.values():
 for p in sorted(body["parts"],key=lambda p:p["z1"]):shapes.append((a._convex_hull([[v[0],v[1]] for v in a.part_vertices(p)]),p.get("material","MAT-RC")))
view("Parents programme / Elternbereich | native fixture plan, source-qualified construction",(40,65,1100,1710),(3.2,10.0,10.95,21.25),shapes)
for label,tag,axis,station,box,bounds in [("Thin cantilever and hollow plinth / duenne Auskragung und Hohlsockel","FIX-CHAISE",0,6.085,(1160,80,1850,560),(14.1,3.1,16.8,4.4)),("Separate concrete cabinet / separater Betonschrank","FIX-DRESSING-STORAGE",1,15.35,(1160,650,1850,1150),(6.45,3.1,7.25,5.25)),("Built-in bath and drain void / Einbauwanne und Ablaufhohlraum","FIX-BATH",0,5.1,(1160,1240,1850,1670),(14.3,3.15,16.4,3.8))]:
 shapes=[(cut(p,axis,station),p.get("material","MAT-RC")) for p in bodies[tag]["parts"]]
 view(label,box,bounds,shapes)
ink.text((40,1750),"All cavity/profile/RC dimensions inferred; 5cm blue/black mosaic identity sourced. Cross-sections show actual material, not full silhouette fills.",fill="#183346")
output=ticket/"🗑️generated"/"parents-physical-r92.png";image.save(output)
(ticket/"🗑️generated"/"parents-atlas-r92.json").write_text(json.dumps({"source_sha256":hashlib.sha256(Path(a.__file__).with_name("🔣️.json").read_bytes()).hexdigest(),"tags":tags,"parts":{tag:len(body["parts"]) for tag,body in bodies.items()}},indent=2))
print("[DEBUG] "+str(output),flush=True)
