"""🛢️ Author the rigid polished rod and its constant-length twin bridle on the existing walking-beam rig."""
import json
import math
import os
from pathlib import Path

ticket=Path(__file__).resolve().parent
radius=3;gland=1.28;carrier_y=3.18;pivot_y=4.4;links=7
leg=pivot_y-carrier_y+radius*math.radians(20)
section=leg/links

def words(en,de):return {"en":en,"de":de}

def wire_angles(lift):
    def point(s):
        y=carrier_y+lift+s
        if y<=pivot_y:return (-radius,y)
        angle=(y-pivot_y)/radius
        return (-radius*math.cos(angle),pivot_y+radius*math.sin(angle))
    points=[point(i*section) for i in range(links+1)]
    tangents=[math.degrees(math.atan2(b[1]-a[1],b[0]-a[0])) for a,b in zip(points,points[1:])]
    return [a-b for a,b in zip(tangents,[90,*tangents])]
rest=wire_angles(0)

def offsets(bones):
    lift=bones.get("rod",{}).get("dy")
    if lift is None:return
    for i,(a,b) in enumerate(zip(wire_angles(lift),rest),1):bones["bridle-"+str(i)]={"angleDeg":round(a-b,7)}

def rig(a):
    a["rig"]["bones"]=[b for b in a["rig"]["bones"] if not b["id"].startswith("bridle-")]
    for i,angle in enumerate(rest,1):
        a["rig"]["bones"].append({"id":f"bridle-{i}","parent":"rod" if i==1 else f"bridle-{i-1}","title":words(f"Bridle rope, length {i}",f"Seilgehänge, Abschnitt {i}"),"x":0 if i==1 else section,"y":0,"angleDeg":angle,"lengthM":section,"joint":{"type":"revolute","minDeg":-15,"maxDeg":15}})
    for pose in a["poses"]:offsets(pose["bones"])
    for clip in a["clips"]:
        for key in clip["keys"]:
            if "bones" in key:offsets(key["bones"])

def solid(sid,bone,kind,**kw):
    return dict(id=sid,kind=kind,bone=bone,material="painted-steel",role="body",tone=.52,**kw)

def cable_id(i):return f"bridle-rope-{i}"

def volume(v,scale):
    rod=next(s for s in v["body"] if s["id"]=="polished-rod")
    rod.update(bone="rod",axis=[1,0,0],origin=[-3.215,0,0],profile=[[0,0],[0,.025],[3.4,.025],[3.4,0]])
    old=next(s for s in v["structure"] if s["id"]=="member-polished-rod");old["member"]="rod"
    if not any(s["id"]=="well-casing" for s in v["body"]):
        casing=solid("well-casing","base","revolve",axis=[0,1,0],origin=[-3,-1.30,0],profile=[[0,.070],[0,.085],[1.3,.085],[1.3,.070],[0,.070]])
        v["body"].append(casing)
        v["thermal"].append({"id":"th-well-casing","of":"well-casing","region":"wellhead","sectionFaces":["-u","+u","-v","+v","-w","+w"]})
        v["structure"].append({"id":"member-well-casing","of":"well-casing","member":"base","part":"member"})
    for s in v["body"]:
        if s["id"] in ("wellhead-base","wellhead-valve","wellhead-top","wellhead","stuffing-box"):
            pr=s.get("profile")
            if pr and pr[0][1]>.035:
                pr[:]=[[pr[0][0],.0285],*pr,[pr[-1][0],.0285],[pr[0][0],.0285]]
    for i in range(1,links+1):
        base=cable_id(i)
        for mode,prefix in (("body",""),("thermal","th-"),("structure","member-")):
            v[mode]=[s for s in v[mode] if s["id"]!=prefix+base]
        for side,z in (("near",-.23),("far",.23)):
            name=base+"-"+side
            if any(s["id"]==name for s in v["body"]):continue
            s=solid(name,f"bridle-{i}","capsule",p0=[0,0,z],p1=[section,0,z],r0=.0075,r1=.0075,slender=True)
            v["body"].append(s);v["thermal"].append({"id":"th-"+name,"of":name,"region":"steel"})
            v["structure"].append({"id":"member-"+name,"of":name,"member":f"bridle-{i}","part":"member"})
    v["decisions"]=[d for d in v["decisions"] if not d["en"].startswith("Reciprocating polished rod")]
    for d in v["decisions"]:
        for lang in ("en","de"):
            for token in ("; the polished rod and the vertical bridle run are one", "; Polierstange und"):
                if token in d[lang]:d[lang]=d[lang].split(token)[0]+"."
    v["decisions"].append(words("Reciprocating polished rod through the bored stuffing box and a buried casing shown to 1.30 m below grade. The two 15 mm bridle ropes meet the carrier 460 mm apart and wrap the 3 m radius horsehead without changing length.","Bewegliche Polierstange durch die durchbohrte Stopfbuchse und das bis 1,30 m unter Gelände dargestellte Futterrohr. Die beiden 15-mm-Gehängeseile treffen den Träger im Abstand von 460 mm und schmiegen sich bei konstanter Länge an den Pferdekopf mit 3 m Radius."))

def shape(sid,bone,z,region="steel",tone=.55):
    return dict(id=sid,bone=bone,role="body",z=z,filled=True,tone=tone,region=region,material="painted-steel")

def drawings(rep):
    for view,shapes in rep["geometry"].items():
        rod=next(s for s in shapes if s["id"]=="polished-rod")
        rod.clear();rod.update(shape("polished-rod","rod",3.2,tone=.9),kind="member",**{"from":{"bone":"base","x":-3,"y":gland,"z":0},"to":{"bone":"rod","x":.185,"y":0,"z":0},"section":{"kind":"circle","d":.050},"penMm":.20})
        if any(s["id"]=="well-casing" for s in shapes):continue
        if view=="plan":
            casing=dict(shape("well-casing","base",2.9,"wellhead"),kind="ellipse",cx=-3,cy=0,rx=.085,ry=.085)
        else:
            x=-3 if view=="elevation" else 0
            casing=dict(shape("well-casing","base",2.9,"wellhead"),kind="rect",x=x-.085,y=-1.30,w=.170,h=1.30)
        shapes.append(casing)
        for i in range(1,links+1):
            for side,z in (("near",-.23),("far",.23)):
                sh=shape(cable_id(i)+"-"+side,f"bridle-{i}",3.25)
                sh.update(kind="member",**{"from":{"x":0,"y":0,"z":z},"to":{"x":section,"y":0,"z":z},"section":{"kind":"circle","d":.015},"penMm":.20})
                shapes.append(sh)

def polished_steel(a):
    d=.05;length=3.4;cap=math.pi*(d/2)**2;area=math.pi*d*length+cap
    steel=next(r for r in a["thermal"]["regions"] if r["id"]=="steel")
    steel["title"]=words("Post, beam, horsehead, pitman, bridle and guard","Bock, Balancier, Pferdekopf, Schubstange, Seilgehänge und Schutz")
    steel["areaM2"]=46.72
    if not any(r["id"]=="ashrae-2021" for r in a["provenance"]["references"]):
        a["provenance"]["references"].append({"id":"ashrae-2021","citation":"ASHRAE Handbook Fundamentals (2021), Chapter 4 Heat Transfer and Chapter 25 Heat, Air and Moisture Control (surface emissivity and solar absorptivity of painted and bare metals, glass)."})
    if not any(m["id"]=="chrome" for m in a["materials"]):
        a["materials"].append(dict(id="chrome",title=words("Chromed polished rod","Verchromte Polierstange"),emissivity=.12,solarAbsorptivity=.35,densityKgM3=7850,conductivityWmK=50,specificHeatJKgK=460,source="literature",reference="ashrae-2021"))
    if not any(r["id"]=="polished-rod" for r in a["thermal"]["regions"]):
        a["thermal"]["regions"].append(dict(id="polished-rod",title=words("Polished rod above the gland","Polierstange oberhalb der Stopfbuchse"),kind="surface",model="passive",material="chrome",areaM2=round(area,7),characteristicLengthM=d,geometry="cylinder",skyViewFactor=.5,layers=[],heatCapacityJM2K=round(7850*460*cap*length/area,2),source="authored-estimate",reference="incropera-2018"))
    for key,v in a["volumes"].items():
        next(s for s in v["body"] if s["id"]=="polished-rod")["material"]="chrome"
        thermal=next(s for s in v["thermal"] if s["id"]=="th-polished-rod")
        thermal.update(region="polished-rod",sectionFaces=["-u"])
        for shapes in a["representations"][key]["geometry"].values():
            for sh in shapes:
                if sh["id"] in ("polished-rod","polished-rod-axis"):sh.update(region="polished-rod",material="chrome")
    def exposed(dy):return round((math.pi*d*(carrier_y+dy+.185-gland)+cap)/area,7)
    for pose in a["poses"]:
        fraction=exposed(pose["bones"]["rod"]["dy"])
        pose["thermal"]["regions"]["polished-rod"]={"exposedFraction":fraction,"sunlitFraction":.5}
    for clip in a["clips"]:
        target="region:polished-rod:exposedFraction"
        track={"target":target,"ease":"smooth","keys":[{"t":key["t"],"v":exposed(key["bones"]["rod"]["dy"])} for key in clip["keys"]]}
        clip["tracks"]=[tr for tr in clip["tracks"] if tr["target"]!=target]+[track]

def apply(a):
    rig(a)
    for key,v in a["volumes"].items():volume(v,int(key));drawings(a["representations"][key])
    for key,rep in a["representations"].items():
        marks={"200":{"tail-pin"},"500":{"motor-feet","carrier"}}.get(key,set())
        for shapes in rep["geometry"].values():
            for sh in shapes:
                if sh["id"] in marks:sh["penMm"]=.20
        merged=rep["decisions"]["merged"]
        if any(term in merged["en"].lower() for term in ("rope","polished rod","polished-rod","bridle")):
            rep["decisions"]["merged"]=words("The casing head is grouped at the smaller scales. The polished rod, fixed gland and two separate wire ropes retain their individual mounting points through the pump cycle.","Bei kleinen Maßstäben ist der Futterrohrkopf zusammengefasst. Polierstange, feste Stopfbuchse und zwei einzelne Drahtseile behalten ihre Anschlusspunkte im gesamten Pumpzyklus.")
    polished_steel(a)
    a["dimensions"]["heightM"]=6.87
    notes=a["provenance"]["notes"]
    notes["en"]=notes["en"].replace("The rope leaves the horsehead tangentially at its leftmost point, level with the saddle, and hangs vertical, so the polished rod and the rope are drawn as one line and only the carrier bar travels along it.", "Both wire ropes leave the horsehead tangentially at saddle height. The rod clamp and polished rod travel together beneath their carrier.")
    notes["de"]=notes["de"].replace("Das Seil verlässt den Pferdekopf tangential an dessen linkem Punkt auf Sattelhöhe und hängt senkrecht, daher sind Polierstange und Seil als eine Linie gezeichnet, und nur der Gestängeträger wandert an ihr entlang.", "Beide Drahtseile verlassen den Pferdekopf tangential auf Sattelhöhe. Klemme und Polierstange bewegen sich mit ihrem Träger.")
    notes["en"]=notes["en"].replace("(the default pose, heightM)", "(the default pose, above grade)")
    notes["de"]=notes["de"].replace("(Standardstellung, heightM)","(Standardstellung, über Gelände)")
    notes["en"]=notes["en"].split(" The 5.57 m above-grade mid-stroke dimension")[0]
    notes["de"]=notes["de"].split(" Zur Höhe von 5,57 m über Gelände")[0]
    notes["en"]+=" The 5.57 m above-grade mid-stroke dimension plus the 1.30 m below-grade casing gives 6.87 m overall. The polished rod travels in the casing. The hanging and wrapped portions of the two constant-length wire ropes follow the horsehead and carrier."
    notes["de"]+=" Zur Höhe von 5,57 m über Gelände in Hubmitte kommen 1,30 m Futterrohr unter Gelände, insgesamt 6,87 m. Die Polierstange läuft im Futterrohr. Die freien und anliegenden Abschnitte der beiden längenkonstanten Drahtseile folgen Pferdekopf und Träger."

if __name__=="__main__":
    path=next(Path("🎓️teaching").glob("*architecture/*staffage/*catalog/*/oil-pumpjack/*.json"))
    raw=path.read_bytes();doc=json.loads(raw);apply(doc["asset"])
    source=ticket/"volumes/machinery/oil-pumpjack.json"
    source_prior=source.read_bytes();authored=json.loads(source_prior)
    for key,v in authored["volumes"].items():
        volume(v,int(key))
        next(s for s in v["body"] if s["id"]=="polished-rod")["material"]="chrome"
        next(s for s in v["thermal"] if s["id"]=="th-polished-rod").update(region="polished-rod",sectionFaces=["-u"])
    for f,d,old in ((path,doc,raw),(source,authored,source_prior)):
        indent=next((i for i in (1,2,4) if old.startswith(b'{\n'+b' '*i+b'"')),None)
        tmp=f.with_suffix(".partial");tmp.write_text(json.dumps(d,ensure_ascii=False,indent=indent)+('\n' if old.endswith(b'\n') else ''));assert f.read_bytes()==old;os.replace(tmp,f)
