"""🧱 Complete the 1960 wall sample with alternating stretcher and header courses and matching section drawings."""
from pathlib import Path
import json
import runpy

ticket=Path(__file__).resolve().parent
helpers=runpy.run_path(str(ticket/"equipment-sections.py"))
kit=runpy.run_path(str(ticket/"r3-solid-brick-wall-1960-kit.py"))
write=helpers["write"]
document=helpers["document"]
path,asset=document("solid-brick-wall-1960")
volume=asset["volumes"]["50"]
rep=asset["representations"]["50"]
outer=lambda name:next(s for s in volume["body"] if s["id"]==name)
body=[outer("plaster")]
members=[]
mortar="masonry-mortar"
length=(-.5,.5)


def element(name,u,v,w,material,repeat):
    part=kit["box"](name,u,v,w,material,.72 if material==mortar else .49)
    part["size"]=[round(b-a,8) for a,b in (u,v,w)]
    part["transform"]["translation"]=[round((b+a)/2,8) for a,b in (u,v,w)]
    part["repeat"]={"count":repeat,"translation":[0,1/repeat,0]}
    sidecuts=[face for face in kit["cuts"](u,v,w) if face.endswith("w")]
    if sidecuts:
        part["sectionFaces"]=sidecuts
    body.append(part)
    members.append({"id":"compression-"+name,"of":name,"member":"root","part":"member"})


element("bed",(0.015,.38),kit["bed_v"](0),length,mortar,12)
for course,kind in enumerate(("A","B")):
    layout=kit["LAYERS"][kind]
    v=kit["brick_v"](course)
    element("collar-"+kind.lower(),layout["collar"],v,length,mortar,6)
    for row,pitch,origin in [("stretcher",.25,layout["stretcherJoint"]),("header",.125,kit["HEADER_JOINT0"])]:
        joints=kit["joints"](origin,pitch)
        for i,w in enumerate(kit["pieces"](joints)):
            element(f"{kind.lower()}-{row}-{i+1}",layout[row],v,w,"solid-brick",6)
        for i,centre in enumerate(joints):
            element(f"{kind.lower()}-{row}-joint-{i+1}",layout[row],v,(centre-.005,centre+.005),mortar,6)
body.append(outer("render"))
members.append(next(s for s in volume["structure"] if s.get("part")=="support"))
decisions=[
    {"en":"Twelve courses of 71 mm bricks and 12⅓ mm beds fit the metre-high sample. Each course has a 115 mm stretcher row, a 10 mm collar and 240 mm headers through the remaining depth. Header and stretcher faces switch sides at each bed, with offset vertical joints.", "de":"Zwölf Schichten mit 71 mm hohen Ziegeln und 12⅓ mm dicken Lagerfugen ergeben einen Meter. Jede Schicht enthält eine 115 mm breite Läuferreihe, eine 10 mm dicke Längsfuge und 240 mm tiefe Binder. Binder und Läufer wechseln je Schicht die Seite, die Stoßfugen sind versetzt."},
    volume["decisions"][1],
    {"en":"Compression passes through the individual units and mortar joints into the continuous base bearing. The structural section follows the alternating bond through the full 365 mm leaf.", "de":"Die Druckkraft gelangt über die einzelnen Steine und Mörtelfugen in das durchgehende Fußauflager. Der Tragwerksschnitt folgt dem wechselnden Verband durch die gesamte 365-mm-Schale."}
]
thermal=[s.copy() for s in volume["thermal"]]
for skin in thermal:
    if skin["id"]=="th-render":
        skin["sectionFaces"]=["-u","-v","+v","-w","+w"]
vdelta=[(["50","body"],body),(["50","structure"],members),(["50","thermal"],thermal),(["50","decisions"],decisions)]
base=rep["geometry"]["elevation"][:3]
line={k:v for k,v in rep["geometry"]["elevation"][3].items() if k not in ("id","points")}
def stroke(name,points):
    return {**line,"id":name,"material":mortar,"points":[[round(x,6),round(y,6)] for x,y in points]}
elevation=list(base)
for course in range(12):
    y0,y1=kit["bed_v"](course)
    elevation.append(stroke(f"joint-{course+1}",[(.015,(y0+y1)/2),(.38,(y0+y1)/2)]))
    u=sum(kit["LAYERS"][kit["course_kind"](course)]["collar"])/2
    elevation.append(stroke(f"collar-{course+1}",[(u,y1),(u,(course+1)/12)]))
plan=rep["geometry"]["plan"][:3]
layout=kit["LAYERS"]["B"]
u=sum(layout["collar"])/2
plan.append(stroke("collar-plan",[(u,-.5),(u,.5)]))
for row,origin,pitch in [("stretcher",layout["stretcherJoint"],.25),("header",kit["HEADER_JOINT0"],.125)]:
    for i,w in enumerate(kit["joints"](origin,pitch)):
        plan.append(stroke(f"{row}-joint-plan-{i+1}",[(layout[row][0],w),(layout[row][1],w)]))
_,insulated=document("etics-brick-wall-12")
materials=[s for s in asset["materials"] if s["id"]!=mortar]+[next(s for s in insulated["materials"] if s["id"]==mortar)]
updates=[(["asset","volumes"]+keys,val) for keys,val in vdelta]
updates.extend([
    (["asset","materials"],materials),(["asset","representations","50","geometry","elevation"],elevation),
    (["asset","representations","50","geometry","plan"],plan),
    (["asset","summary","en"],asset["summary"]["en"].replace("A uninsulated","An uninsulated"))
])
for scale,representation in asset["representations"].items():
    if representation["status"]!="authored":
        continue
    decisions2d=representation["decisions"].copy()
    decisions2d["silhouette"]={"en":"One metre of rendered solid masonry in section, with the room face on the left and the weather face on the right.","de":"Ein Meter verputztes Vollziegelmauerwerk im Schnitt, innen links und außen rechts."}
    decisions2d["hatching"]={"en":"A diagonal hatch identifies the cut solid brick leaf.","de":"Eine diagonale Schraffur kennzeichnet den geschnittenen Vollziegel."}
    decisions2d["omitted"]={"en":"The sample ends at the cut edges of the wall field.","de":"Der Ausschnitt endet an den Schnittkanten des Wandfelds."}
    if scale=="50":
        decisions2d["retained"]={"en":"Plaster and render enclose the alternating header and stretcher courses. Bed and collar joints cross the vertical section. The top cut shows the staggered head joints.","de":"Innen- und Außenputz schließen die wechselnden Binder- und Läuferschichten ein. Lager- und Längsfugen erscheinen im Vertikalschnitt, die versetzten Stoßfugen in der Aufsicht."}
    else:
        decisions2d["retained"]={"en":"The aggregate masonry section retains its 400 mm overall thickness.","de":"Der zusammengefasste Mauerwerksquerschnitt behält seine Gesamtdicke von 400 mm."}
    updates.append((["asset","representations",scale,"decisions"],decisions2d))
write(path,updates)
write(ticket/"volumes/components/solid-brick-wall-1960.json",[(["volumes"]+keys,val) for keys,val in vdelta])

