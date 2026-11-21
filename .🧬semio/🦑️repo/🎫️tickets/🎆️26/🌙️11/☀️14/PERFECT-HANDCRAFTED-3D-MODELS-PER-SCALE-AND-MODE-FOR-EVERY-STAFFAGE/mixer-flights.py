"""🌀 Author twin welded mixing flights and the inclined drum drive of the mixer truck."""
from math import ceil, cos, pi, sin
from pathlib import Path
import json
import runpy

ticket=Path(__file__).resolve().parent
inchannels=[(0.28,0.20),(0.80,0.32),(1.25,0.37),(2.90,0.36),(4.00,0.31),(4.70,0.27),(5.10,0.21),(5.31,0.15)]
leads=[(0.28,0.0),(1.10,0.39),(1.90,0.73),(3.15,1.20),(4.50,1.66),(5.31,1.91)]
carrier="drum"
barrel="drum-spiral"

def interpolate(rows,x):
    for (a,v),(b,w) in zip(rows,rows[1:]):
        if x<=b:
            return v+(w-v)*(x-a)/(b-a)
    return rows[-1][1]

def sections(d,index):
    shell=[(0.18,0.83),(1.10,1.15),(1.90,1.15),(5.40,0.56)]
    gauge={50:.005,100:.005,200:.030,500:.075}[d]
    max_degrees={50:8,100:9,200:16,500:26}[d]
    seating=max(.004,gauge/2)
    xs=[]
    for (start,first),(end,last) in zip(leads,leads[1:]):
        count=ceil(360*(last-first)/max_degrees)
        xs.extend(round(start+(end-start)*i/count,7) for i in range(count))
    xs.append(leads[-1][0])
    out=[]
    for x in xs:
        angle=index*pi-2*pi*interpolate(leads,x)
        width=interpolate(inchannels,x)-(gauge-.005)+(seating-.004)
        radius=interpolate(shell,x)-gauge+seating-width/2
        out.append({"at":[round(x,6),round(radius*cos(angle),7),round(radius*sin(angle),7)],
                    "rx":{50:.006,100:.008,200:.0155,500:.038}[d],"ry":round(width/2,7),"exponent":{50:6,100:6,200:4,500:4}[d],
                    "rotation":[round(sin(angle/2),9),0,0,round(cos(angle/2),9)]})
    return out

def pressed_flight(den,phase):
    vertices=[]
    triangles=[]
    ring_size=6 if den==50 else 4
    for row in sections(den,phase):
        x,y,z=row["at"]
        halfaxial,halfradial=row["rx"],row["ry"]
        radius=(y*y+z*z)**.5
        inner,outer=radius-halfradial,radius+halfradial
        outline=[(-halfaxial,inner),(halfaxial,inner),(halfaxial,outer),(-halfaxial,outer)]
        if den==50:
            bevel=.0015
            outline=[(-halfaxial+bevel,inner),(halfaxial-bevel,inner),(halfaxial,inner+bevel),(halfaxial,outer),(-halfaxial,outer),(-halfaxial,inner+bevel)]
        offset=len(vertices)
        vertices.extend([[round(x+dx,8),round(y*r/radius,8),round(z*r/radius,8)] for dx,r in outline])
        if offset:
            for j in range(ring_size):
                k=(j+1)%ring_size
                triangles.extend([[offset-ring_size+j,offset-ring_size+k,offset+j],[offset-ring_size+k,offset+k,offset+j]])
    last=len(vertices)-ring_size
    for j in range(1,ring_size-1):
        triangles.extend([[0,j+1,j],[last,last+j,last+j+1]])
    signed=sum(a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]) for a,b,c in ([vertices[i] for i in face] for face in triangles))
    if signed<0:triangles=[face[::-1] for face in triangles]
    return {"kind":"polyhedron","vertices":vertices,"triangles":triangles}

def update_volumes(volumes):
    for den,volume in volumes.items():
        d=int(den)
        for kind in ["body","thermal","structure"]:
            for part in volume[kind]:
                key=part["id"]
                if key in ["drum-blade-1","drum-blade-2"]:
                    for field in ("sections","capStart","capEnd"):part.pop(field,None)
                    part.update(pressed_flight(d,int(key[-1])-1))
                if part.get("bone")==carrier and (key.startswith("drum-") or key=="th-drum-skin"):
                    part["bone"]=barrel
        if d in (100,200) and not any(s["id"]=="drum-hatch" for s in volume["body"]):
            hatch={"id":"drum-hatch","kind":"box","bone":barrel,"material":"drum-paint","role":"detail","tone":.36,"size":[.46,.36,.04],"transform":{"translation":[1.5,0,-1.142]}}
            if d==100:hatch["cornerRadius"]=.016
            volume["body"].append(hatch)
            volume["thermal"].append({"id":"th-drum-hatch","of":"drum-hatch","region":"drum","layer":"shell"})
        if d==500:
            for index,member in enumerate(volume["structure"]):
                if member.get("kind") in ("loft","polyhedron") and member["id"] in ("drum-blade-1","drum-blade-2"):
                    volume["body"].append({k:v for k,v in member.items() if k not in ("part","member")})
                    volume["structure"][index]={"id":"member-"+member["id"],"of":member["id"],"member":barrel,"part":"member"}
        rotating={s["id"] for s in volume["body"] if s.get("bone")==barrel}
        for part in volume["structure"]:
            if (part.get("of") in rotating or part.get("bone")==barrel) and part.get("part")=="member":part["member"]=barrel
            if part["id"]=="mass-drum":part.update(bone=barrel,member=barrel)
            if part["id"]=="joint-drum-bearing":part["member"]=barrel
        if not any(p["id"]=="member-drive-bearing-housing" for p in volume["structure"]):
            volume["structure"].append({"id":"member-drive-bearing-housing","of":"drive-gearbox" if d==50 else "drive-housing","member":carrier,"part":"member"})
        if d==50:
            for part in volume["body"]:
                if part["id"] in ("wheel-disc-1-l","wheel-disc-1-r","wheel-disc-2-l","wheel-disc-2-r"):
                    from math import hypot
                    radius=hypot(*part["profile"][0])
                    if radius<.28-1e-6:part["profile"]=[[round(.28*x/radius,6),round(.28*y/radius,6)] for x,y in part["profile"]]
        notes=volume["decisions"]
        for n in notes:
            n["en"]=n["en"].replace("two blades of thirteen sections","two full-length mixing flights").replace("two blades of seven sections","two full-length mixing flights")
            n["de"]=n["de"].replace("zwei Wendeln aus dreizehn Schnitten","zwei durchgehende Mischwendeln").replace("zwei Wendeln aus sieben Schnitten","zwei durchgehende Mischwendeln")
        note={"en":"Twin welded flights run 5.03 m from the drive dome to the discharge cone. They widen through the mixing belly and taper to clear the fixed inlet spout. Drum skin, tracks and blades turn on their common longitudinal axis.","de":"Zwei angeschweißte Wendeln führen über 5,03 m vom Antriebsboden zum Auslaufkonus. Im Mischbauch sind sie höher. Am festen Einfüllstutzen laufen sie schmal aus. Trommelmantel, Laufringe und Wendeln drehen sich um dieselbe Längsachse."}
        if note not in notes:notes.append(note)

    runpy.run_path(str(ticket/"mixer-wheels.py"))["update_wheels"](volumes)

def update_rig(asset):
    bearing=next(b for b in asset["rig"]["bones"] if b["id"]==carrier)
    rotor=next(b for b in asset["rig"]["bones"] if b["id"]==barrel)
    bearing["title"]={"en":"Inclined drum bearing axis, stationary drive and feed spout","de":"Geneigte Trommellagerachse mit festem Antrieb und Einfüllstutzen"}
    rotor.update(title={"en":"Rotating steel barrel, running rings and twin welded mixing flights","de":"Drehende Stahltrommel mit Laufringen und zwei angeschweißten Mischwendeln"},lengthM=5.40,joint={"type":"revolute","continuousTwist":True})
    if "mass" in bearing:rotor["mass"]=bearing.pop("mass")
    for clip in asset["clips"]:
        if clip["id"]=="agitate-at-rest":
            clip["description"]={"en":"The drum and its welded flights turn once in ten seconds at idle. The engine and radiator keep emitting throughout the cycle.","de":"Trommel und angeschweißte Mischwendeln drehen sich bei Leerlauf in zehn Sekunden einmal. Motor und Kühler bleiben über den gesamten Umlauf aktiv."}
        for key in clip["keys"]:
            t=key["t"]
            if t in (6.9667,9.9667,11.9667):key["ease"]="linear"
            spin=36*t if clip["id"]=="agitate-at-rest" else 360-72*max(0,min(12,t-2))
            key.setdefault("bones",{})[barrel]={"twistDeg":round(spin,5)}

def update_text(asset):
    notes=asset["provenance"]["notes"]
    notes["en"]=notes["en"].replace("13.5 t unladen, 32 t gross", "13.5 t unladen, 35.1 t with the modelled 21.6 t charge")
    notes["de"]=notes["de"].replace("13,5 t leer, 32 t Gesamtgewicht", "13,5 t leer, 35,1 t mit der modellierten 21,6-t-Füllung")
    for assumption in asset["structure"]["assumptions"]:
        assumption["en"]=assumption["en"].replace("The drum rotation is not an elevation joint; the flight marks are a drawing pattern.", "A fixed bearing supports the 3.6 t rotating steel drum and its welded flights. Their centre of mass lies 2.6 m along the rotation axis. The fresh concrete load acts independently at its own axial centre.")
        assumption["de"]=assumption["de"].replace("Die Trommeldrehung ist kein Gelenk der Ansicht; die Wendelmarken sind ein Zeichenmuster.", "Das feste Lager trägt die 3,6 t schwere rotierende Stahltrommel mit ihren eingeschweißten Wendeln. Ihr Schwerpunkt liegt 2,6 m entlang der Drehachse. Die Frischbetonlast greift unabhängig davon in ihrem eigenen axialen Schwerpunkt an.")
    for key,old,new,deold,denew in [
        (asset["summary"],"whose spiral flight marks drift with its rotation","with twin welded flights that rotate with the shell","deren Wendelmarken mit der Drehung wandern","deren angeschweißte Mischwendeln mit dem Mantel umlaufen"),
        (asset["provenance"]["notes"],"The drum rotation is shown by a flight pattern that drifts along the drum axis (the rotation axis lies in the drawing plane)","The skin and twin internal flights rotate on the inclined longitudinal axis. Flight pitch and blade depth are estimates for this body class","Die Trommeldrehung wird durch ein längs der Trommelachse wanderndes Wendelmuster gezeigt (die Drehachse liegt in der Zeichenebene)","Trommelmantel und beide Mischwendeln drehen sich gemeinsam um die geneigte Längsachse. Steigung und Schaufelhöhe sind Schätzwerte für diese Baugröße"),
        (asset["representations"]["50"]["decisions"]["retained"],"five flight marks that drift along the drum axis","five diagrammatic flight marks carried by the rotating barrel","fünf Wendelmarken, die längs der Trommelachse wandern","fünf schematische Wendelmarken auf der drehenden Trommel")]:
        key["en"]=key["en"].replace(old,new)
        key["de"]=key["de"].replace(deold,denew)

def update_ink(asset):
    for scale,volume in asset["volumes"].items():
        profiles={s["bone"]:s for s in volume["body"] if s["id"] in ("chute-trough","chute-ext-trough")}
        for shapes in asset["representations"][scale]["geometry"].values():
            for shape in shapes:
                if shape["id"] in ("chute-main","chute-ext-folded"):
                    trough=profiles[shape["bone"]]
                    for key in ("x","y","w","h"):shape.pop(key,None)
                    shape.update(kind="poly",closed=True,points=trough["profile"],solid={"kind":"extruded","sweep":"u","z0":trough["from"],"z1":trough["to"]})
    for view in ("front","plan"):
        for shape in asset["representations"]["50"]["geometry"][view]:
            if shape["id"].startswith(("tyre-3-left","tyre-3-right","tyre-4-left","tyre-4-right")):
                shape["penMm"]=.30
    side=asset["representations"]["50"]["geometry"]["elevation"]
    ladder=next(s for s in side if s["id"]=="rear-ladder")
    style={k:v for k,v in ladder.items() if k not in ("id","kind","closed","points","x0","x1","y0","y1","r0","r1")}
    side[:]=[s for s in side if s["id"] not in ("rear-ladder","ladder-guard-post")]
    for identifier,key in [("rear-ladder","ladder-stile-1"),("ladder-guard-post","guard-post-1")]:
        rail=next(s for s in asset["volumes"]["50"]["body"] if s["id"]==key)
        side.append({**style,"id":identifier,"kind":"capsule","filled":True,"x0":rail["p0"][0],"y0":rail["p0"][1],"x1":rail["p1"][0],"y1":rail["p1"][1],"r0":rail["r0"],"r1":rail["r1"],"penMm":.16})
    def ring(r,reverse=False):
        direction=0 if reverse else 1
        return f"M {r} 0 A {r} {r} 0 1 {direction} {-r} 0 A {r} {r} 0 1 {direction} {r} 0 Z"
    def polygon(points):
        return "M "+" L ".join(f"{x} {y}" for x,y in points)+" Z"
    for axle in range(1,5):
        disc=next(s for s in asset["volumes"]["50"]["body"] if s["id"]==f"wheel-disc-{axle}-r"+("-outer" if axle>=3 else ""))
        for key,r,opening in [(f"tyre-{axle}",.52,.288),(f"rim-{axle}",.31,.275)]+([(f"rim-well-{axle}",.29,.262)] if axle>=3 else []):
            sh=next(sh for sh in side if sh["id"]==key)
            for field in ("cx","cy","rx","ry","rotationDeg"):sh.pop(field,None)
            sh.update(kind="path",d=ring(r)+" "+ring(opening,True))
        faceid=f"steel-wheel-disc-{axle}"
        side[:]=[s for s in side if s["id"]!=faceid and not s["id"].startswith(f"handhole-{axle}-")]
        rim=next(sh for sh in side if sh["id"]==f"rim-{axle}")
        face={**rim,"id":faceid,"z":4.17,"d":polygon(disc["profile"])+" "+" ".join(polygon(list(reversed(hole))) for hole in disc["holes"])}
        side.append(face)
        nuts=next(s for s in asset["volumes"]["50"]["body"] if s["id"]==f"wheel-nuts-{axle}-r"+("-outer" if axle>=3 else ""))
        wrench_ring=nuts["vertices"][48:60:2]
        outlines=[]
        for stud in range(nuts["repeat"]["count"]):
            angle=stud*nuts["repeat"]["rotationRad"]
            outlines.append(polygon([(round(x*cos(angle)-y*sin(angle),7),round(x*sin(angle)+y*cos(angle),7)) for x,y,z in wrench_ring]))
        fasteners=f"wheel-fasteners-{axle}"
        side[:]=[sh for sh in side if sh["id"]!=fasteners]
        hub=next(sh for sh in side if sh["id"]==f"hub-{axle}")
        side.append({**{k:v for k,v in hub.items() if k not in ("cx","cy","rx","ry","rotationDeg")},"id":fasteners,"kind":"path","d":" ".join(outlines),"material":nuts["material"],"z":4.31,"penMm":.13,"tone":.50})
    side.sort(key=lambda s:s["z"])
    for scale in ["50","100","200"]:
        rep=asset["representations"][scale]
        for view,shapes in rep["geometry"].items():
            shapes[:]=[sh for sh in shapes if not (sh["bone"]==barrel and sh["id"].startswith("flight-")) and sh["id"]!="drum-hatch"]
            points=[[1.286,-.180],[1.714,-.180],[1.730,-.164],[1.730,.164],[1.714,.180],[1.286,.180],[1.270,.164],[1.270,-.164]] if scale!="200" else [[1.27,-.18],[1.73,-.18],[1.73,.18],[1.27,.18]]
            hatch={"id":"drum-hatch","bone":barrel,"role":"detail","z":3.7,"filled":False,"tone":.36,"kind":"poly","points":points,"closed":True,"region":"drum","material":"drum-paint","solid":{"kind":"extruded","sweep":"w","z0":-1.162,"z1":-1.122}}
            shapes.append(hatch)
            shapes.sort(key=lambda sh:sh["z"])
        retained=rep["decisions"]["retained"]
        for lang,replacements in {
            "en":[("five diagrammatic flight marks carried by the rotating barrel, seams and the hatch","the barrel seams and rotating access hatch"),("five flight marks","the rotating access hatch"),("drum with flights and seam","drum with its access hatch and seam"),("drum with flights","drum with its access hatch"),("visor and hatch","and visor")],
            "de":[("fünf schematische Wendelmarken auf der drehenden Trommel, Nähte und das Mannloch","Trommelnähte und das umlaufende Mannloch"),("fünf Wendelmarken","das umlaufende Mannloch"),("Trommel mit Wendeln und Naht","Trommel mit Mannloch und Naht"),("Trommel mit Wendeln","Trommel mit Mannloch"),("Blende und Mannloch","und Blende")]
        }.items():
            for old,new in replacements:retained[lang]=retained[lang].replace(old,new)
    for shapes in asset["representations"]["100"]["geometry"].values():
        for shape in shapes:
            if shape["id"]=="air-filter":
                shape["penMm"]=.22

if __name__=="__main__":
    io=runpy.run_path(str(ticket/"equipment-sections.py"))
    path,asset=io["document"]("concrete-mixer-truck")
    update_volumes(asset["volumes"])
    update_rig(asset)
    update_text(asset)
    update_ink(asset)
    edits=[(["asset","volumes",den,kind],volume[kind]) for den,volume in asset["volumes"].items() for kind in ("body","thermal","structure","decisions")]
    edits.extend([(["asset","summary"],asset["summary"]),(["asset","structure","assumptions"],asset["structure"]["assumptions"]),(["asset","provenance","notes"],asset["provenance"]["notes"]),(["asset","rig","bones"],asset["rig"]["bones"]),(["asset","clips"],asset["clips"])])
    edits.extend((["asset","representations",sc,k],asset["representations"][sc][k]) for sc in ("50","100","200","500") for k in ("geometry","decisions"))
    io["write"](path,edits)
    source=ticket/"volumes/machinery/concrete-mixer-truck.json"
    volumes=json.loads(source.read_text())["volumes"]
    update_volumes(volumes)
    io["write"](source,[(["volumes",den,kind],v[kind]) for den,v in volumes.items() for kind in ("body","thermal","structure","decisions")])
