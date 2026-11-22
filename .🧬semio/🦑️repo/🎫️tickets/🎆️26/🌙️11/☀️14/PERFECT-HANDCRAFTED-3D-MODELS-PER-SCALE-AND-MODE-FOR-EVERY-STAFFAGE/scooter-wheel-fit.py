"""🛴 Cut rear dropouts into the coarse deck and fit independent front blades, crowns and axles."""
from copy import deepcopy
from pathlib import Path
import json
import re
import sys

T=Path(__file__).resolve().parent
CAT=next(T.parents[6].glob("🎓*/*architecture/*staffage/*catalog/*/child-on-kick-scooter/🔣️.json"))

def author(a):
    fine=a["volumes"]["50"]
    stock={q["id"]:q for q in fine["body"]}
    for volume in a["volumes"].values():
        for support in volume["structure"]:
            if support.get("anchor") not in ("wheel-front-contact","wheel-rear-contact"):
                continue
            contact=next(q for q in a["anchors"] if q["id"]==support["anchor"])
            half=contact["footprintWidthM"]/2
            radius=contact["footprintLengthM"]/2
            support.update(profile=[[-half,0],[-half,radius],[half,radius],[half,0]],origin=[contact["x"],radius,0])
    for key in ("100","200","500"):
        volume=a["volumes"][key]
        body,thermal,structure=(volume[k] for k in ("body","thermal","structure"))
        parts={s["id"]:s for s in body}
        def append(solid,region):
            body.append(solid)
            thermal.append(dict(id="th-"+solid["id"],of=solid["id"],region=region))
            structure.append(dict(id="st-"+solid["id"],of=solid["id"],member="frame",part="member"))
        def metal(id,kind,**kw):
            return dict(id=id,kind=kind,bone="frame",role="body",material="alu-stem",tone=.7,**kw)
        def plate(id,from_,to,points):
            return metal(id,"extrude",plane="xy",**{"from":from_,"to":to,"profile":points})
        for end in ("rear","front"):
            axle=deepcopy(stock["axle-"+end])
            append(axle,"deck" if end=="rear" else "stem")
        if key in ("200","500"):
            rear={"200":-.023,"500":-.020}[key]
            deck=parts["deck"]
            outline=[[rear,-.0275],[.040,-.031],[.085,-.060],[.415,-.060],[.415,.060],[.085,.060],[.04,.031],[rear,.0275],[rear,.0195],[.082,.0195],[.082,-.0195],[rear,-.0195]]
            for field in ("size","transform"):deck.pop(field)
            deck.update(kind="extrude",plane="zx",profile=[[z,x] for x,z in outline],**{"from":.055,"to":.085})
        if key=="200":
            for side in ("l","r"):
                parts["fork-"+side].update(deepcopy(stock["fork-"+side]))
        if key=="500":
            body.remove(parts["fork"]);thermal[:]=[q for q in thermal if q.get("of")!="fork"];structure[:]=[q for q in structure if q.get("of")!="fork"]
            for side in ("l","r"):
                append(deepcopy(stock["fork-"+side]),"stem")
            append(plate("neck-arm",-.016,.016,[[.397,.061],[.416,.059],[.551,.199],[.54,.219],[.523,.211],[.435,.106],[.398,.084]]),"stem")
        for end in ("front","rear"):
            parts["tyre-"+end].update(deepcopy(stock["tyre-"+end]))
            for base in ("rim","spoke","hub"):
                id=base+"-"+end
                if id in parts:
                    parts[id].update(deepcopy(stock[id]))
                else:
                    body.append(deepcopy(stock[id]))
                    thermal.append(deepcopy(next(q for q in fine["thermal"] if q.get("of")==id)))
                    structure.append(deepcopy(next(q for q in fine["structure"] if q.get("of")==id)))
        crown=deepcopy(stock["fork-crown"])
        append(crown,"stem")
        description=volume["decisions"][0]
        description["en"]=description["en"].replace("tyre, core disc and hub","tyre, rim, spokes and hub").replace("deck slab","slotted deck").replace("wheel discs","spoked wheels")
        description["de"]=description["de"].replace("Reifen, Kernscheibe und Nabe","Reifen, Felge, Speichen und Nabe").replace("Trittbrettplatte","geschlitztes Trittbrett").replace("Radscheiben","Speichenräder")
        for index,mode in enumerate(("body","thermal","structure")):
            decision=volume["decisions"][index]
            for lang in ("en","de"):
                decision[lang]=re.sub(r"(1:\d+[^()]*\()\d+",lambda m:m[1]+str(len(volume[mode])),decision[lang],count=1)
        if key=="500":
            d=volume["decisions"][0]
            d["en"]=d["en"].split("Scooter:")[0]+"The scooter has a slotted deck, two axles, separate front blades, a fork crown, a gooseneck and two spoked wheels."
            d["de"]=d["de"].split("Roller:")[0]+"Der Roller hat ein geschlitztes Trittbrett, zwei Achsen, getrennte Gabelblätter, eine Gabelkrone, einen Rahmenhals und zwei Speichenräder."
        details={"en":"The 145 × 30 mm wheels retain the rim, hub and six spokes. The front crown spans the blades above the tread and both axles seat in the dropouts.","de":"Die Räder von 145 × 30 mm behalten Felge, Nabe und sechs Speichen. Die vordere Krone verbindet die Gabelblätter oberhalb des Reifens. Beide Achsen sitzen in den Ausfallenden."}
        volume["decisions"].append(details)
        for view,shapes in a["representations"][key]["geometry"].items():
            if key=="500":
                for shape in shapes:
                    if shape["id"]=="scooter-front-wheel":
                        shape["regions"]=["wheels","stem"]
        if key=="500":
            volume["decisions"].append({"en":"At 1:500 the printed end and plan views combine the wheels and adjacent fork into a legible mark. The digital tyre retains its physical tread width.","de":"Bei 1:500 zeigen die gedruckte Stirnansicht und Draufsicht Rad und angrenzende Gabel als ein lesbares Zeichen. Der digitale Reifen behält seine tatsächliche Laufbreite."})

if __name__=="__main__":
    doc=json.loads(CAT.read_bytes())
    author(doc["asset"])
    Path(sys.argv[1]).write_text(json.dumps(doc,ensure_ascii=False,separators=(",",":")))
