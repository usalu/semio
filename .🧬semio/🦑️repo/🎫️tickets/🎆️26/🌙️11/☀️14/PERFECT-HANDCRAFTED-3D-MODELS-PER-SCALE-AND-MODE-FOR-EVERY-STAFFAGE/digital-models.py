"""🗿 Hand-laid digital miniatures in millimetres, in each asset's own joint frames."""
from copy import deepcopy
from pathlib import Path
import json
import math
import re
import sys

TICKET = Path(__file__).resolve().parent
CATALOG = next(Path.cwd().glob("🎓️teaching/🏛️architecture/*staffage/🗂️catalog"))
IDENTITY = ("id", "bone", "material", "tone", "role", "tags", "region", "layer", "member", "anchor", "part", "sectionFaces", "repeat")

def mm(values):
    return [v / 1000 for v in values]

def box(size, center):
    return dict(kind="box", size=mm(size), cornerRadius=0, transform=dict(translation=mm(center)))

def turn(axis, origin, points):
    return dict(kind="revolve", origin=mm(origin), axis=axis, profile=[mm(p) for p in points])

def prism(plane, low, high, profile, holes=()):
    result=dict(kind="extrude", plane=plane, **{"from":low/1000,"to":high/1000}, profile=[mm(p) for p in profile])
    if holes:
        result["holes"]=[[mm(p) for p in ring] for ring in holes]
    return result

def rect(x0,y0,x1,y1):
    return [(x0,y0),(x1,y0),(x1,y1),(x0,y1)]

def octagon(x,y,r):
    return [(-x+r,-y),(x-r,-y),(x,-y+r),(x,y-r),(x-r,y),(-x+r,y),(-x,y-r),(-x,-y+r)]

def disc(r,n=8):
    return [(round(r*math.cos(i*math.tau/n),3),round(r*math.sin(i*math.tau/n),3)) for i in range(n)]

def roundbar(axis,origin,a,b,r):
    return turn(axis,origin,[(a,0),(a,r),(b,r),(b,0)])

def beam(p,q,r,s):
    return dict(kind="capsule",p0=mm(p),p1=mm(q),r0=r/1000,r1=s/1000)

X,Y,Z=(1,0,0),(0,1,0),(0,0,1)
O=(0,0,0)

def author(slug,scale,source):
    lists=[{s["id"]:deepcopy(s) for s in source[k]} for k in ("body","thermal","structure")]
    body,thermal,structure=lists
    def put(items,id,shape):
        old=items[id]
        if "of" in old:old={**body[old["of"]],**old}
        items[id]={**{k:old[k] for k in IDENTITY if k in old}, **shape}
    def omit(items,*ids):
        for id in ids:
            del items[id]
    def surface(items,id,target):
        old=items[id]
        items[id]={k:old[k] for k in ("id","region","layer","member","part","anchor") if k in old}
        items[id]["of"]=target
    def b(id,shape):put(body,id,shape)
    def h(id,shape):put(thermal,id,shape)
    def f(id,shape):put(structure,id,shape)

    if slug=="ceiling-diffuser":
        b("flange",prism("zx",-9,0,rect(-297.5,-297.5,297.5,297.5),[rect(-245,-245,245,245)]))
        b("cone",prism("zx",-70,-13,rect(-252,-252,252,252)))
        body["cone"]["taperTo"]=[.54,1]
        b("collar",turn(Y,O,[(0,123),(50,123),(50,124.5),(0,124.5),(0,123)]))
        for side,x in [("left",-254),("right",254)]:
            h("jet-"+side,box((5,4,405),(x,-12,0)))
            f("support-ceiling-hanger-"+side,roundbar(Y,(-280 if x<0 else 280,0,0),-5,9,4))
        note=("The square face and dished, tapered central plate leave a perimeter discharge slot. The circular neck stays hollow.","Quadratische Front und konischer Mittelteller lassen einen umlaufenden Auslass frei. Der runde Hals bleibt offen.")
    elif slug=="circulation-pump":
        b("casing",turn(Z,(105.5,-3,0),[(8,0),(8,41),(22,48),(53,48),(67,37),(67,0)]))
        b("pipe",roundbar(X,(-6,0,35),0,220,20))
        b("motor-head",turn(Z,O,[(-67,0),(-67,35),(-58,42),(8,42),(8,0)]))
        h("th-rotor-window",roundbar(Z,O,-67.6,-66.8,21))
        f("member-terminal",box((39,45,8),(13,0,-27)))
        note=("The transverse motor meets the pipe at a stepped volute. A separate terminal box retains the electrical connection.","Der querliegende Motor trifft an der gestuften Spirale auf das Rohr. Der eigene Klemmenkasten markiert den Stromanschluss.")
    elif slug in ("duct-fan","decentralised-hrv-fan"):
        hrv=slug.startswith("decentralised")
        half,outer,inner,front,back=(130,98,82,-120,120) if hrv else (110,100,96,-45,45)
        b("plate",prism("xy",front,front+4,octagon(half,half,8),[disc(inner,16)]))
        b("ring",turn(Z,O,[(front+4,inner),(back,inner),(back,outer),(front+4,outer),(front+4,inner)]))
        for i in range(3):
            angle=i*math.tau/3
            blade=[(27,-6),(outer-13,4),(outer-17,26),(35,14)] if hrv else [(29,-10),(87,-12),(94,8),(45,15)]
            points=[(round(x*math.cos(angle)-y*math.sin(angle),2),round(x*math.sin(angle)+y*math.cos(angle),2)) for x,y in blade]
            b("blade-"+str(i+1),prism("xy",-99 if hrv else -28,-94 if hrv else -23,points))
        if hrv:
            h("th-exhaust-out",roundbar(Z,O,59,60,80))
            note=("The wall sleeve surrounds the ceramic core and reversible three-vane impeller. Both air ports remain distinct.","Das Wandrohr umfasst Keramikkern und umkehrbares Dreiflügelrad. Beide Luftanschlüsse bleiben getrennt.")
        else:
            h("th-extract",turn(Z,O,[(-3,44),(-3,89),(-2,89),(-2,44),(-3,44)]))
            note=("The recessed three-vane rotor leaves an open annulus within the square mounting plate.","Das zurückgesetzte Dreiflügelrad lässt einen offenen Ring innerhalb der quadratischen Montageplatte.")
        f("support-wall-screw-top",roundbar(Z,(-108 if hrv else -92,108 if hrv else 92,0),front-1,front+5,3))
    elif slug=="desktop-monitor":
        b("panel",prism("yz",-167,156,[(-40,-270),(-40,270),(-26,270),(-5,184),(-5,-184),(-26,-270)]))
        b("neck",prism("xy",-32,32,[(-4,-6),(247,-6),(256,11),(0,30)]))
        b("base-plate",prism("zx",0,12,[(-124,12),(-88,0),(88,0),(124,12),(112,210),(-112,210)]))
        h("th-front-stack",box((323,2,536),(-6,-39,0)))
        f("member-panel-rail",box((21,7,482),(-20,-18,0)))
        note=("The tapered rear case clears the pivot. The wide foot, rising column and panel remain separate moving assemblies.","Das rückseitig verjüngte Gehäuse lässt das Gelenk frei. Breiter Fuß, Säule und Bildschirm bleiben getrennte Baugruppen.")
    elif slug=="electric-fan-heater":
        b("block",prism("yz",-91,87,[(16,-92),(30,-118),(216,-120),(269,-84),(269,84),(216,120),(30,118),(16,92)]))
        b("grille",box((7,171,186),(94,145,0)))
        h("th-grille",box((2,170,184),(97,145,0)))
        f("member-shell",prism("yz",-82,82,[(22,-99),(223,-105),(251,-81),(251,81),(223,105),(22,99)], [rect(34,-87,214,87)]))
        note=("The outlet sits flush in the inclined case. The grip above it remains open and the feet bear below it.","Der Auslass sitzt bündig im geneigten Gehäuse. Der Griff darüber bleibt offen, die Füße tragen von unten.")
    elif slug=="electric-kettle":
        b("jug",turn(Y,O,[(0,0),(0,76),(34,88),(117,81),(175,69),(175,0)]))
        b("base",turn(Y,O,[(0,0),(0,86),(6,90),(30,89),(30,0)]))
        b("spout",prism("xy",-14,14,[(-52,118),(-52,151),(-114,189),(-100,163)]))
        b("handle",prism("xy",-12,12,[(-5,65),(30,60),(59,35),(59,-28),(30,-57),(-4,-62),(-4,-47),(20,-42),(40,-23),(40,28),(25,42),(-5,48)]))
        h("th-water",turn(Y,O,[(0,0),(0,72),(75,79),(75,0)]))
        surface(structure,"member-handle","handle")
        note=("The tapered jug and pouring lip oppose an open grip. The water remains attached to its own rig bone.","Dem verjüngten Kessel und Ausguss liegt ein offener Bügel gegenüber. Das Wasser bleibt seinem eigenen Riggelenk zugeordnet.")
    elif slug=="electricity-meter":
        b("base",box((212,132,25),(50,0,31.5)))
        b("cover",prism("xy",-74,18,[(-2,-48),(8,-59),(134,-59),(148,-44),(148,44),(134,59),(8,59),(-2,48)]))
        omit(body,"works")
        h("th-plate",box((62,82,1),(82,0,-66)))
        f("member-spindle",roundbar(X,O,-4,34,2))
        note=("The meter cover and terminal block sit on one mounting base. The visible disc keeps its separate spindle.","Zählerhaube und Klemmenblock sitzen auf einer Montageplatte. Die sichtbare Scheibe behält ihre eigene Spindel.")
    elif slug=="gas-boiler":
        b("casing",prism("yz",-220,220,[(50,-148),(63,-167),(735,-167),(770,-135),(770,175),(50,175)]))
        b("collar",turn(Y,(0,0,40),[(0,53),(50,53),(50,62),(0,62),(0,53)]))
        h("th-vessel",dict(kind="ellipsoid",radii=mm((155,155,39)),transform=dict(translation=mm((0,400,105)))))
        f("member-connections",box((335,9,37),(0,49,70)))
        note=("A wall-hung casing encloses the burner and expansion vessel. The concentric flue rises separately above the shell.","Im wandhängenden Mantel liegen Brenner und Ausdehnungsgefäß. Der konzentrische Abgasstutzen ragt über die Hülle.")
    elif slug=="gas-hob":
        b("plate",prism("zx",0,40,rect(-260,-150,260,150)))
        b("burner",turn(Y,(0,0,30),[(0,0),(0,37),(17,37),(19,52),(25,52),(25,0)]))
        b("flame-crown",turn(Y,(0,0,30),[(0,46),(0,51),(22,48),(0,46)]))
        h("th-plate",box((300,3,520),(0,38,0)))
        f("member-plate",box((282,4,484),(0,36,4)))
        note=("The burner cap overhangs the flame ring and grate. The gas control still turns independently at the front.","Der Brennerdeckel kragt über den Flammenring unter dem Rost. Der Gasregler an der Vorderkante bleibt drehbar.")
    elif slug=="gas-meter":
        b("housing",prism("xy",-67,67,[(-76,-100),(76,-100),(96.5,-69),(96.5,60),(75,95),(-75,95),(-96.5,60),(-96.5,-69)]))
        omit(body,"counter","dial")
        b("index",box((130,66,26),(0,39,-77)))
        h("th-unit",dict(kind="ellipsoid",radii=mm((74,57,44)),transform=dict(translation=mm((0,-30,0)))))
        f("member-unit",box((134,8,72),(0,-49,0)))
        note=("Clipped corners describe the pressed diaphragm case. The forward index and twin threaded connections define its orientation.","Abgesetzte Ecken zeichnen das gepresste Membrangehäuse. Vorgebautes Zählwerk und zwei Gewindestutzen zeigen die Einbaulage.")
    elif slug=="kitchen-extractor-hood":
        b("canopy",prism("xy",-450,450,[(0,0),(500,0),(500,57),(400,200),(0,200)]))
        omit(body,"end-l","end-r")
        omit(structure,"member-canopy-end-l","member-canopy-end-r")
        h("th-filter",box((430,2,810),(248,1,0)))
        f("support-wall-rest",box((8,25,280),(4,40,0)))
        note=("The folded canopy spans both end caps. Its grease filter faces down and the telescopic flue remains articulated.","Der gefaltete Schirm schließt die Stirnkappen ein. Der Fettfilter zeigt nach unten, der Teleskopschacht bleibt beweglich.")
    elif slug=="laptop":
        b("base",prism("xy",-156.5,156.5,[(0,0),(218,2.8),(218,9),(200,12.5),(0,12.5)]))
        b("lid",box((218,6.5,313),(104.5,1.25,0)))
        h("th-deck",prism("xy",-153,153,[(0,11.3),(199,11.3),(216,8.2),(216,9.3),(199,12.5),(0,12.5)]))
        f("lid-frame",prism("zx",-1,1,rect(-151,-2,151,210),[rect(-143,5,143,202)]))
        note=("The tapered keyboard deck and thin display meet at the actual hinge. Opening the lid keeps both slabs in register.","Tastaturkeil und dünner Bildschirm treffen an der Scharnierachse zusammen. Beim Öffnen bleiben die Kanten ausgerichtet.")
    elif slug=="mvhr-unit":
        b("casing",box((600,600,298),(300,0,1)))
        omit(body,"cover")
        omit(thermal,*(s["id"] for s in thermal.values() if s.get("of")=="cover"))
        b("window",roundbar(Z,(300,0,0),-150,-148,127))
        h("th-extract-fan",roundbar(Z,(500,165,0),0,104,70))
        f("member-back",box((565,570,3),(300,0,146)))
        note=("The enclosure reads as one cabinet around the exchanger core, access window and two duct pairs.","Der Schrank fasst Tauscherkern, Wartungsfenster und zwei Kanalpaare in einem Gehäuse zusammen.")
    elif slug=="pendant-lamp":
        b("shade",turn(X,O,[(-30,0),(-30,22),(7,41),(139,133),(139,131),(8,39),(0,0)]))
        b("bulb",turn(X,O,[(-37,0),(-37,11),(25,15),(47,30),(72,29),(102,10),(105,0)]))
        h("th-bulb",turn(X,O,[(24,0),(29,17),(51,29),(74,28),(105,0)]))
        f("member-cord",roundbar(X,O,0,602,2.8))
        note=("The spun shade retains its hollow throat. Cord, holder and bulb remain axial through the swing.","Der Blechschirm behält seine hohle Kehle. Kabel, Fassung und Kolben liegen beim Pendeln auf einer Achse.")
    elif slug=="room-thermostat":
        if scale==200:
            b("housing",prism("xy",-1.5,13.5,[(-15,-40),(-10,-45),(70,-45),(75,-40),(75,40),(70,45),(-10,45),(-15,40)]))
            b("dial",turn(Z,O,[(-13.5,0),(-13.5,18),(-11,21),(1.5,21),(1.5,0)]))
            h("th-display",box((21,36,0.7),(56.5,0,-1.4)))
            f("member-housing",box((74,60,2),(30,0,12)))
            note=("Clipped corners locate the faceplate, with a rectangular display and a bevel on the projecting rotary dial.","Gebrochene Ecken fassen die Frontplatte mit Rechteckanzeige und abgeschrägtem Drehknopf.")
        else:
            b("housing",box((90,90,15),(30,0,6)))
            b("dial",prism("xy",-13.5,0,disc(20.5,8)))
            omit(body,"display")
            h("th-display",box((20,35,0.5),(56.5,0,-1.5)))
            h("th-casing-front",box((90,90,1),(30,0,-1)))
            omit(thermal,"th-casing-wall-top","th-casing-wall-left","th-casing-wall-bottom","th-casing-wall-right")
            f("member-housing",box((76,28,1.5),(30,0,12.5)))
            note=("The square enclosure and projecting knob identify the thermostat. The display remains a separate thermal surface.","Quadratgehäuse und vorstehender Knopf kennzeichnen den Regler. Die Anzeige bleibt eine eigene thermische Fläche.")
    elif slug=="shower-head":
        b("tube",roundbar(Y,(60,0,0),0,810,12))
        b("head",turn((.83867,-.54464,0),(39,140,0),[(0,0),(0,55),(5,67),(48,66),(48,0)]))
        for n,p,q in [(0,(10,-4,0),(135,-89,-46)),(2,(135,-89,-46),(200,-260,-105)),(4,(200,-260,-105),(130,-337.5,-129)),(6,(130,-337.5,-129),(90,-220,-105))]:
            id="hose-%02d"%n
            b(id,beam(p,q,7.5,7.5))
            h("th-"+id,beam(p,q,7.5,7.5))
            omit(body,"hose-%02d"%(n+1))
            omit(thermal,"th-hose-%02d"%(n+1))
        f("member-rail",roundbar(Y,(60,0,0),95,790,10))
        note=("The hose passes through four laid-out bends beneath a round riser. The inclined head remains on its slider.","Unter der runden Stange läuft der Schlauch durch vier gesetzte Bögen. Der geneigte Kopf bleibt am Schieber.")
    elif slug=="split-ac-indoor-unit":
        b("casing",prism("xy",-425,425,[(0,0),(140,0),(192,43),(210,105),(205,239),(165,285),(0,290)]))
        omit(body,"end-cap","grille-field")
        b("vane",prism("xy",-392,392,[(0,-3),(57,15),(66,21),(5,4)]))
        h("th-front",prism("xy",-419,419,[(192,43),(210,105),(205,239),(202,239),(207,105),(189,43)]))
        f("member-back",box((3,260,755),(2,145,0)))
        note=("The intake crown and curved nose share one casing. The vane rotates independently on the discharge lip.","Ansaugkrone und gerundete Nase bilden ein Gehäuse. Die Klappe dreht unabhängig an der Austrittskante.")
    elif slug=="thermostatic-radiator-valve":
        if scale==200:
            b("body",turn(Y,O,[(-75,0),(-75,15),(-61,15),(-57,21),(-40,21),(-33,15),(-11,15),(0,19),(12,19),(12,0)]))
            b("head",turn(Z,O,[(-75,0),(-75,25),(-68,30),(-25,30),(-14,22),(18,18),(18,0)]))
            h("th-knob",turn(Z,O,[(-75,0),(-75,23),(-69,29),(-50,29),(-50,0)]))
            f("member-head",roundbar(Z,O,-52,16,9))
            note=("The stepped union and tapered thermostatic head meet at right angles. The control cap covers the sensing end.","Die gestufte Verschraubung und der verjüngte Thermostatkopf treffen rechtwinklig aufeinander. Die Kappe schließt das Fühlerende.")
        else:
            b("body",turn(Y,O,[(-75,0),(-75,17),(-33,17),(-10,15),(12,18),(12,0)]))
            b("head",turn(Z,O,[(-75,0),(-75,27),(-32,30),(18,18),(18,0)]))
            h("th-knob",roundbar(Z,O,-75,-51,27))
            f("member-head",roundbar(Z,O,-55,12,7))
            note=("A tapered axial head crosses the short valve union. Heat exchange stays on the casing and sensing cap.","Ein konischer Axialkopf kreuzt den kurzen Ventilanschluss. Gehäuse und Fühlerkappe bleiben getrennte Wärmeflächen.")
    elif slug=="toaster":
        b("casing",prism("xy",-84,84,[(-136,0),(136,0),(150,23),(148,149),(126,185),(-126,185),(-148,149),(-150,23)]))
        b("top-plate",prism("zx",180,188,rect(-75,-128,75,128),[rect(-51,-92,-14,92),rect(14,-92,51,92)]))
        omit(body,"pan")
        omit(thermal,*(s["id"] for s in thermal.values() if s.get("of")=="pan"))
        h("th-slot-air",box((173,3,29),(0,183,-32)))
        f("member-slices-carriage",box((145,4,95),(0,40,0)))
        note=("The shoulders roll into the two-slot top. A separate slice still rides the carriage through its travel.","Die Schultern laufen zum Doppelschlitzdeck aus. Eine eigene Scheibe fährt mit dem Hubkorb.")
    elif slug=="tower-server":
        b("case",prism("xy",-100,100,[(-225,12),(211,12),(225,26),(225,437),(212,450),(-225,450)]))
        b("plinth",box((395,12,164),(0,6,0)))
        for suffix,x,z in [("fl",195,75),("fr",195,-75),("rl",-195,75),("rr",-195,-75)]:
            h("th-feet-"+suffix,box((20,12,18),(x,6,z)))
            f("member-case-post-"+suffix,box((12,417,12),(x,227,z)))
        h("th-front-bezel",box((6,419,194),(222,230,0)))
        h("th-rear-panel",box((2,434,196),(-224,231,0)))
        h("th-psu-vent-rear",box((2,143,154),(-225,373,0)))
        h("th-cpu-grille",box((166,122,1),(15,267,100)))
        note=("The chamfered front and recessed plinth establish the case orientation. Four distinct foot patches coincide with the corner posts.","Die gefaste Front und der zurückgesetzte Sockel zeigen die Ausrichtung. Vier getrennte Fußflächen liegen unter den Eckpfosten.")
    elif slug=="cooking-pot":
        b("wall",turn(Y,O,[(0,0),(0,77),(6,85),(100,86),(100,85.2),(6,84.2),(5,77),(5,0)]))
        b("water",roundbar(Y,O,5,50,84))
        for side,sign in [("left",-1),("right",1)]:
            pts=[(sign*x,y) for x,y in [(81,0),(117,0),(125,8),(125,18),(119,24),(82,24),(82,18),(115,18),(118,14),(115,7),(81,7)]]
            b("handle-"+side,prism("xy",-10,10,pts))
        h("th-base",roundbar(Y,O,0,5,81))
        h("th-wall",turn(Y,O,[(5,84),(100,85.2),(100,86),(5,85),(5,84)]))
        f("support-base-contact",roundbar(Y,O,0,0.8,77))
        note=("Open folded grips flank the thin-walled vessel. The five-millimetre sandwich base bears on the hob beneath the water.","Offene Bügel flankieren den dünnwandigen Topf. Der fünf Millimeter starke Sandwichboden trägt das Wasser auf dem Kochfeld.")
    elif slug=="tealight":
        b("cup-shell",turn(Y,O,[(0,0),(0,16),(15.6,19),(15.6,18.85),(.15,15.85),(.15,0)]))
        b("wax-wick",turn(Y,O,[(.15,0),(.15,15.8),(13,18.3),(13,0)]))
        b("wax-top",roundbar(Y,O,-1,4.5,1))
        b("flame",turn(Y,O,[(0,0),(3,3.3),(6,3.9),(11,3.2),(22,0)]))
        h("th-wick",roundbar(Y,O,-1,4.5,1))
        f("support-base-contact",roundbar(Y,O,0,.15,16))
        note=("The pressed foil cup remains 38 mm wide. A millimetre-radius wick separates the wax pool from the tapered flame.","Der gezogene Folienbecher bleibt 38 mm breit. Ein Docht mit einem Millimeter Radius trennt Wachsspiegel und Flamme.")
    else:
        raise ValueError(slug)
    if slug=="ceiling-diffuser":
        h("th-cone",prism("zx",-70,-13,rect(-252,-252,252,252)))
        thermal["th-cone"].update(taperTo=[.54,1],sectionFaces=["+v"])
        h("th-collar",dict(kind="loft",sections=[dict(at=mm((0,y,0)),rx=.124,ry=.124,exponent=2) for y in (0,50)],capStart=False,capEnd=False))
    elif slug=="circulation-pump":
        b("casing",prism("xy",8,67,[(105.5+x,-3+y) for x,y in disc(48,10)]))
        b("pipe",prism("yz",-6,214,[(y,35+z) for y,z in disc(20,10)], [[(y,35+z) for y,z in disc(13,10)]]))
        f("member-rotor",prism("xy",-55,34,disc(12,8)))
    elif slug=="electric-fan-heater":
        b("handle-block",prism("yz",-9,9,[(262,-65),(262,-50),(296,-50),(296,50),(262,50),(262,65),(301,65),(310,54),(310,-54),(301,-65)]))
        h("th-warm-air",prism("yz",96,98,[(145+y,z) for y,z in disc(48,8)]))
    elif slug=="electric-kettle":
        f("member-lid",prism("zx",0,10,disc(50,8)))
    elif slug=="electricity-meter":
        body["cover"]["holes"]=[[[.028,-.046],[.120,-.046],[.120,.046],[.028,.046]]]
        b("window",box((94,94,2),(74,0,-73.8)))
    elif slug=="gas-boiler":
        b("panel",box((400,145,8),(0,137.5,-168)))
        h("th-vessel",prism("xy",66,144,[(x,400+y) for x,y in octagon(155,155,60)]))
        f("member-collar",prism("zx",0,50,[(40+z,x) for z,x in disc(56,8)], [[(40+z,x) for z,x in disc(53,8)]]))
        omit(thermal,"th-hx-collector")
    elif slug=="gas-hob":
        f("member-knob-spindle",prism("xy",-257,-225,disc(5,6)))
    elif slug=="gas-meter":
        h("th-unit",prism("xy",-52,52,[(-77,-81),(-50,-88),(-50,-60),(-13,-60),(-13,-87),(13,-87),(13,-60),(50,-60),(50,-88),(77,-81),(77,27),(-77,27)]))
        f("member-pointer-spindle",prism("xy",-92,-68,disc(2,6)))
    elif slug=="kitchen-extractor-hood":
        h("th-chimney-upper",prism("zx",0,370,rect(-148,1,148,298)))
    elif slug=="pendant-lamp":
        h("th-shade",dict(kind="loft",sections=[dict(at=mm((x,0,0)),rx=r/1000,ry=r/1000,exponent=2) for x,r in [(-30,18),(6,42),(55,95),(139,133)]],capStart=False,capEnd=False))
    elif slug=="room-thermostat":
        if scale==200:
            h("th-casing-wall-top",box((2,80,12),(74,0,6)))
            h("th-casing-wall-bottom",box((2,80,12),(-14,0,6)))
            b("dial",prism("xy",-13.5,1.5,disc(21,12)))
        f("member-dial",prism("xy",-13.5,1.5,disc(5,8 if scale==200 else 6)))
    elif slug=="shower-head":
        h("th-tube",dict(kind="loft",sections=[dict(at=mm((60,y,0)),rx=.012,ry=.012,exponent=2) for y in (95,790)],capStart=False,capEnd=False))
        thermal["spray-film"]={**thermal["th-spray"],"id":"spray-film",**roundbar((.83867,-.54464,0),(79,114,0),0,0.8,53)}
        thermal["spray-film"]={k:v for k,v in thermal["spray-film"].items() if k not in ("sections","capStart","capEnd","tags")}
        f("member-rail",prism("zx",95,790,[(z,60+x) for z,x in disc(10,8)]))
        f("mass-shower",box((23,20,39),(30,80,0)))
    elif slug=="thermostatic-radiator-valve":
        if scale==200:
            b("body",turn(Y,O,[(-75,0),(-75,15),(-57,21),(-40,21),(-33,15),(12,18),(12,0)]))
            h("th-knob",prism("xy",-75,-50,disc(27,12)))
        f("member-body",prism("zx",-75,12,disc(8.5,10 if scale==200 else 6),[disc(6.5,10 if scale==200 else 6)]))
    elif slug=="toaster":
        f("member-base-frame",prism("xy",-58,58,[(-128,12),(128,12),(128,20),(-128,16)]))
    elif slug=="tower-server":
        f("mass-case",box((124,158,55),(-20,200,0)))
        h("th-case-plume",prism("xy",-40,40,[(-142,370),(-58,370),(-100,432)]))
        h("th-feet-fl",box((20,12,18),(195,6,75)))
        thermal["th-feet-fl"]["repeat"]=dict(count=2,translation=mm((0,0,-150)))
        omit(thermal,"th-feet-fr")
        note=(note[0]+" The thermal model retains both front feet as a repeated contact.","Die thermische Darstellung enthält beide vorderen Füße als wiederholte Kontaktfläche. "+note[1])
    elif slug=="cooking-pot":
        b("wall",prism("zx",0,100,disc(86,12),[disc(85.2,12)]))
        b("water",prism("zx",5,50,disc(84,12)))
        for side,sign in [("left",-1),("right",1)]:
            b("handle-"+side,prism("xy",-10,10,[(sign*x,y) for x,y in [(81,0),(125,0),(125,24),(81,24),(81,18),(118,18),(118,6),(81,6)]]))
        h("th-wall",dict(kind="loft",sections=[dict(at=mm((0,y,0)),rx=.0856,ry=.0856,exponent=2) for y in (5,100)],capStart=False,capEnd=False))
        h("th-base",prism("zx",0,5,disc(80,10)))
        f("member-surface",prism("zx",-1,0,disc(67,8)))
    return dict(status="authored",denominator=scale,body=list(body.values()),thermal=list(thermal.values()),structure=list(structure.values()),decisions=[dict(en=note[0],de=note[1])])

def replace_scale(path,scale,volume,root):
    raw=path.read_text()
    start=raw.index('"volumes"')
    pattern=r'(?m)^( +)"'+str(scale)+r'"\s*:\s*(?=\{)'
    section=re.search(pattern,raw[start:])
    pos=start+section.end()
    current,size=json.JSONDecoder().raw_decode(raw[pos:])
    if current.get('denominator')!=scale: raise ValueError((path,scale))
    pad=section.group(1)
    shape=json.dumps(volume,ensure_ascii=False,indent=2)
    result=raw[:pos]+shape.replace('\n','\n'+pad)+raw[pos+size:]
    before=json.loads(raw);after=json.loads(result)
    target=before["asset"] if root else before
    target["volumes"][str(scale)]=json.loads(json.dumps(volume))
    assert before==after
    if path.read_text()!=raw:raise RuntimeError(f"Concurrent edit to {path}. Retry.")
    path.write_text(result)

def main():
 for asset_file in sorted(CATALOG.glob('*/*/🔣️.json')):
    a=json.loads(asset_file.read_bytes())['asset']
    if a['category'] not in ('appliances','objects'): continue
    if sys.argv[1:] and a['id'] not in sys.argv[1:]:continue
    for d in (200,500):
        if a["representations"][str(d)]["status"]=="authored":continue
        ref=a["volumes"]["200" if a["representations"]["200"]["status"]=="authored" else "100"]
        volume=author(a["id"],d,ref)
        for key in ("thermal","structure"):
            assert all("of" not in s or s["of"] in {b["id"] for b in volume["body"]} for s in volume[key]),(a["id"],key)
        source=TICKET/"volumes"/a["category"]/(a["id"]+".json")
        replace_scale(source,d,volume,False)
        replace_scale(asset_file,d,volume,True)
        print(a["id"],d,*(len(volume[k]) for k in ("body","thermal","structure")))

if __name__=="__main__":
    main()
