"""🪟 Project actual native joinery solids into four facade and salon detail views."""
import hashlib,importlib,json,sys,time
from pathlib import Path
from PIL import Image,ImageDraw
root=Path.cwd();ticket=Path(__file__).parent;sys.path.insert(0,str(root));api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
source=api.load_model();source_raw=Path(api.__file__).with_name("🔣️.json").read_bytes();started=time.monotonic();image=Image.new("RGB",(2200,2100),"white");draw=ImageDraw.Draw(image)
colors={"MAT-WOOD":"#926b45","MAT-METAL":"#424d58","MAT-GLASS":"#b4dce6","MAT-TEXTURED-GLASS":"#a7cbd0","MAT-RC":"#a0a5a7"}
windows={e["tag"]:api.filling_geometry(e["tag"],source) for e in source["elements"] if e["class"]=="IfcWindow"}
review=[]
def panel(label,parts,box,axis,bounds):
    """📐 Project source-owned convex prism faces without asserting depth visibility."""
    left,top,right,bottom=box;x0,y0,x1,y1=bounds
    canvas=Image.new("RGB",(right-left,bottom-top),"white");ink=ImageDraw.Draw(canvas)
    draw.text((left,top-20),label,fill="#172b3a")
    for part in parts:
        vertices=api.part_vertices(part);points=[(v[axis],v[2]) if axis in (0,1) else (v[0],v[1]) for v in vertices]
        hull=api._convex_hull(points)
        if len(hull)<3:continue
        projected=[((x-x0)/(x1-x0)*(right-left),(y1-y)/(y1-y0)*(bottom-top)) for x,y in hull]
        ink.polygon(projected,fill=colors.get(part.get("material"),"#d2c7b7"),outline="#56646d")
    image.paste(canvas,(left,top));draw.rectangle(box,outline="#ccd4d8")
    review.append({"label":label,"parts":len(parts),"axis":axis,"bounds":bounds})
for index,(name,host,tags,axis,maximum) in enumerate([("South / Sued","W-S",["RW-S"],0,19.3),("North / Nord","W-N",["RW-N","RW-N-2"],0,19.3),("West","W-W",["RW-W","RW-W-UPPER","RW-W-UPPER-2","RW-W-UPPER-3"],1,21.5),("East / Ost",None,["RW-E"],1,21.5)]):
    parts=[]
    if host:
        wall=api._wall_geometry(source,api._find(source,host));parts=[p for p in wall["parts"] if p["z1"]>4 and p["z0"]<6]
    for tag in tags:parts+=windows[tag]["parts"]
    y=65+index*280;panel(name+" | actual occupied material / echte belegte Koerper",parts,(70,y,2110,y+220),axis,(0,3.8,maximum,6.15))
salon=windows["GL-TERRACE"]["parts"];mechanical=[p for tag in ("SALON-HANDRAIL","SALON-CHAIN-DRIVE","SALON-CRANK") for p in api._solid_geometry(source,api._find(source,tag))["parts"]]
all_parts=salon+mechanical
xmin=min(v[0] for p in all_parts for v in api.part_vertices(p));xmax=max(v[0] for p in all_parts for v in api.part_vertices(p));ymin=min(v[1] for p in all_parts for v in api.part_vertices(p));ymax=max(v[1] for p in all_parts for v in api.part_vertices(p));zmin=min(p["z0"] for p in all_parts);zmax=max(p["z1"] for p in all_parts)
panel("Salon elevation / Ansicht | two assemblies, four panes / zwei Elemente, vier Scheiben",all_parts,(70,1240,1430,1780),0,(xmin-.05,zmin-.05,xmax+.05,zmax+.05))
panel("Plan / Grundriss | normal axis enlarged / Normalachse vergroessert",all_parts,(70,1850,1430,2010),2,(xmin-.05,ymin-.02,xmax+.05,ymax+.02))
panel("End crank / Endkurbel | inferred mechanical dimensions / abgeleitete Beschlagmasse",mechanical,(1510,1240,2110,1775),0,(xmax-.32,zmin-.01,xmax+.01,zmin+.22))
draw.text((1510,1820),"Brown/oak | grey/metal | blue/glass",fill="#253b49");draw.text((70,2050),"Native geometry diagnostic; projection depth ordering is illustrative. Source hierarchy and material intersections are tested independently.",fill="#253b49")
draw.text((70,2075),"Native Geometriediagnose; Tiefenreihenfolge illustrativ. Hierarchie und Materialueberschneidungen separat geprueft.",fill="#253b49")
out=ticket/"🗑️generated";image.save(out/"carpentry-physical-r92.png");report={"source_sha256":hashlib.sha256(source_raw).hexdigest(),"api_sha256":hashlib.sha256(Path(api.__file__).read_bytes()).hexdigest(),"views":review,"seconds":time.monotonic()-started,"limits":["Projection does not certify hidden hardware or historical exact dimensions","East kiosk window removed; no unsupported glazing product exists","Current salon host remains7.802x2.65 rather than asserting a925x320cm registered assembly"]};(out/"carpentry-physical-r92.json").write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding="utf8");print("[DEBUG] "+json.dumps({"seconds":report["seconds"],"views":len(review),"image":str(out/"carpentry-physical-r92.png")}),flush=True)
