"""🐾 Attach the distal foot, woody crown and grass blades to the members that carry them."""
import copy
import json
import math
from pathlib import Path

catalog = next(Path.cwd().glob("*teaching/*architecture/*staffage/*catalog"))
def write_scale(path, scale, update):
    raw = path.read_text()
    asset = json.loads(raw)["asset"]
    volume = copy.deepcopy(asset["volumes"][str(scale)])
    update(asset, volume)
    if volume == asset["volumes"][str(scale)]: return
    marker = f'"{scale}": '
    start = raw.index(marker,raw.index('"volumes": {'))+len(marker)
    _, length = json.JSONDecoder().raw_decode(raw[start:])
    value = json.dumps(volume, ensure_ascii=False, indent=2, allow_nan=False).replace("\n","\n      ")
    assert path.read_text()==raw
    path.write_text(raw[:start]+value+raw[start+length:])
    print(path.parent.name,scale,*(len(volume[k]) for k in ("body","thermal","structure")))

def shortened(src, stations):
    out = copy.deepcopy(src)
    if stations:
        out["sections"]=[out["sections"][i] for i in stations]
    return out

def body_map(vol):
    return {x["id"]:x for x in vol["body"]}

feet = {
    "horse": ("The four cannons, sloping pasterns and broad hooves pivot through each stride under the barrel and croup.", "Unter Brustkorb und Kruppe folgen die vier Röhrbeine, schrägen Fesseln und breiten Hufe jedem Schritt."),
    "dairy-cow": ("Each hoof and pastern follows its distal joint below the cannon. The udder sits between the hind limbs and the neck carries the heavy muzzle.", "Jede Klaue folgt mit der Fessel dem Endgelenk unter dem Röhrbein. Das Euter sitzt zwischen den Hinterbeinen, der Hals trägt das schwere Flotzmaul."),
    "goat": ("The curved horns and narrow muzzle rise over four tapering cannons. Each foot carries its own short cloven hoof.", "Über den vier schlanken Röhrbeinen erheben sich die gebogenen Hörner und der schmale Kopf. Jeder Fuß trägt seine eigene kurze gespaltene Klaue."),
    "pig": ("The hog rests on four hooves below the hams and forelegs. Each hoof rolls independently of the short cannon.", "Das Schwein steht auf vier Klauen unter Schinken und Vorderbeinen. Jede Klaue rollt unabhängig vom kurzen Mittelfuß ab."),
    "sheep": ("Four bare lower legs emerge below the fleece. The dark cloven hooves move at the toes beneath the pale cannons.", "Vier kurz behaarte Unterbeine treten unter dem Vlies hervor. Die dunklen gespaltenen Klauen bewegen sich an den Zehen unter den hellen Röhrbeinen."),
    "roe-deer": ("The high rump and tucked flank stand over slender cannons, short pasterns and four pointed hooves.", "Unter der hohen Kruppe und der aufgezogenen Flanke stehen schlanke Röhrbeine, kurze Fesseln und vier spitze Schalen."),
    "labrador-dog": ("Four compact paws flex below the carpi and hocks. The pads follow each supporting foot as the retriever shifts its weight.", "Vier kompakte Pfoten beugen sich unter Vorderfußwurzel und Sprunggelenk. Die Ballen folgen der belasteten Pfote bei jeder Gewichtsverlagerung.")
}

def ungulate(asset, vol):
    slug = asset["id"]
    if vol["decisions"][0]["en"]==feet[slug][0]: return
    reference = asset["volumes"]["200"]
    details = body_map(reference)
    for sw,short in [("near","n"),("far","f")]:
        for limb,letter in [("fore","f"),("hind","h")]:
            stem = sw+"-"+limb
            suffix = letter+short
            merging = [stem+"-lower"]
            spec = [(stem+"-cannon", None if slug=="pig" else [0,-1]),(stem+"-hoof",[0,1,-1])]
            if slug=="horse":
                spec = [(stem+"-cannon",[0,1,-1]),(stem+"-pastern",[0,-1]),(stem+"-hoof",None)]
            elif slug=="dairy-cow":
                merging += [stem+"-pastern",stem+"-hoof"]
                spec = [(stem+"-cannon",[0,3,7,8]),(stem+"-pastern",[0,2,5]),(stem+"-hoof",[0,2,5,8,9])]
            elif slug=="goat":
                merging = [limb+"-lower-"+short]
                spec = [("cannon-"+suffix,[0,3,8]),("hoof-"+suffix,[0,2,3])]
            elif slug=="roe-deer":
                spec = [(stem+"-cannon",[0,2,-1]),(stem+"-pastern",None),(stem+"-hoof",None)]
                details[stem+"-hoof"]=body_map(asset["volumes"]["100"])[stem+"-hoof"]
                spec[-1]=(stem+"-hoof",[0,2,4,5])
            elif slug=="labrador-dog":
                merging = [limb+("-foot-" if vol["denominator"]==500 else "-lower-")+short]
                reference = asset["volumes"]["100"]
                details = body_map(reference)
                spec = [(("metatarsus-" if limb=="hind" else "pastern-")+short,[0,4,12,20] if limb=="hind" else [0,3,8]),("paw-"+suffix,[0,5,11,17,20])]
            vol["body"] = [s for s in vol["body"] if s["id"] not in merging]
            names = {name for name,_ in spec}
            vol["thermal"] = [s for s in vol["thermal"] if s.get("of") not in merging and not any(s["id"]=="th-"+n for n in names)]
            for name, stations in spec:
                solid = shortened(details[name],stations)
                vol["body"].append(solid)
                region = next((t["region"] for t in reference["thermal"] if t.get("of")==name), "hooves")
                vol["thermal"].append({"id":"th-"+name,"of":name,"region":region})
            if slug=="horse":
                bone = "hoof-"+suffix
                solid = next(s for s in reference["structure"] if s.get("part")=="member" and s.get("member")==bone)
                vol["structure"].append(shortened(solid,[0,-1]))
            if slug=="labrador-dog":
                pad = next(s for s in reference["thermal"] if s["id"]=="th-pad-"+suffix)
                patch = body_map(reference)[pad["of"]] if "of" in pad else pad
                vol["thermal"] = [s for s in vol["thermal"] if s["id"]!="th-pad-"+suffix]
                vol["thermal"].append({**shortened(patch,None),"id":"th-pad-"+suffix,"region":"pads"})
    en,de = feet[slug]
    vol["decisions"]=[{"en":en,"de":de},{"en":"The contact surfaces follow the hoof or paw joint in both the body and heat-exchange models. The skeleton follows the same ground-contact chain.","de":"Im Körper- und im Wärmeaustauschmodell folgen die Kontaktflächen dem Klauen-, Huf- oder Pfotengelenk. Das Skelett bildet dieselbe Kette bis zum Bodenkontakt ab."}]

def spruce_axes(asset, vol):
    names = {s["id"] for s in vol["structure"]}
    for s in asset["volumes"]["100"]["structure"]:
        if s.get("part") != "member" or not s["member"].startswith("pl-") or s["id"] in names: continue
        vol["structure"].append(shortened(s,[0,2,4] if vol["denominator"]==200 else [0,-1]))

def agave(asset,vol):
    for s in vol["body"]:
        if not (s["id"].startswith("leaf-") and s["id"].endswith("-b")): continue
        tip = s["sections"][-1]
        tip["rx"] = .004 if vol["denominator"]==200 else .006
        tip["ry"] = .003 if vol["denominator"]==200 else .004
    if 'terminal' not in vol["decisions"][-1]["en"]:
        vol["decisions"].append({"en":"The thick blade tapers into a terminal spine at the distal tip of each leaf.","de":"Am Ende jedes Blatts läuft die dicke Spreite in einen Endstachel aus."})

def grass(asset,vol):
    den=vol["denominator"]
    base=asset["volumes"]["50"]
    stations={100:([0,2,4,6],[0,2,4,7,9,10],[0,3,6,10,14,18,21,24]),200:([0,3,6],[0,3,6,9,10],[0,4,9,15,20,24]),500:([0,6],[0,4,7,10],[0,6,15,24])}[den]
    plant = []
    for s in base["body"]:
        name = s["id"]
        if name=="tuft": rows = {100:[0,2,4,6,8],200:[0,2,5,8],500:[0,4,8]}[den]
        elif name.startswith("culm-"): rows=stations[0]
        elif name.startswith("plume-"): rows=stations[1]
        elif name.startswith("blade-"):
            if den==200 and name.endswith("-2"):continue
            if den==500 and not any(t in name for t in ("-1a","-1b")):continue
            rows=stations[2]
        else:continue
        plant.append(shortened(s,rows))
    vol["body"]=plant
    ids={s["id"] for s in plant}
    vol["thermal"]=[s for s in base["thermal"] if s.get("of") in ids or 'emitter' in s.get("tags",[])]
    vol["decisions"]=[{"en":"Four articulated flowering culms stand above the basal clump. The arching blades keep a narrow ribbon section through the bend. Each seed head tapers to its tip.","de":"Vier gegliederte Blütenhalme überragen den Horst. Die bogigen Blätter behalten ihren schmalen bandförmigen Querschnitt. Jede Rispe läuft zur Spitze aus."}]

def lime(asset,vol):
    if any(s["id"]=="wood-ld1" for s in vol["body"]): return
    den=vol["denominator"]
    fine=asset["volumes"]["50"]
    stem=next(s for s in vol["body"] if s["id"]=="stem")
    old=stem["sections"]
    k=next(i for i,s in enumerate(old) if s["at"][0]>2.4)
    a,b=old[k-1:k+1]
    h=(2.4-a["at"][0])/(b["at"][0]-a["at"][0])
    seam={**a,"at":[2.4,*[round(x+(y-x)*h,6) for x,y in zip(a["at"][1:],b["at"][1:])]],**{field:round(a[field]+h*(b[field]-a[field]),6) for field in ["rx","ry"]}}
    stem["sections"]=[*old[:k],seam]
    leader={**stem,"id":"wood-ld1","bone":"ld1","sections":[]}
    joint = next(b for b in asset["rig"]["bones"] if b["id"]=="ld1")
    angle=-math.radians(joint["angleDeg"])
    cs,sn=math.cos(angle),math.sin(angle)
    cw,sz=math.cos(angle/2),math.sin(angle/2)
    for sec in [seam,*old[k:]]:
        x,y,z=sec["at"];x-=joint["x"];y-=joint["y"]
        qx,qy,qz,qw=sec["rotation"]
        leader["sections"].append({**sec,"at":[round(cs*x-sn*y,6),round(sn*x+cs*y,6),z],"rotation":[round(cw*qx-sz*qy,6),round(cw*qy+sz*qx,6),round(cw*qz+sz*qw,6),round(cw*qw-sz*qz,6)]})
    vol["body"].insert(1,leader)
    removed={s["id"] for s in vol["body"] if s["id"].startswith(("fill-crown","crown-"))}
    vol["body"]=[s for s in vol["body"] if s["id"] not in removed]
    vol["thermal"]=[s for s in vol["thermal"] if s.get("of") not in removed and s.get("of")!="stem"]
    vol["thermal"] += [{"id":"th-stem","of":"stem","region":"trunk-bark","sectionFaces":["-u","+u"]},{"id":"th-wood-ld1","of":"wood-ld1","region":"trunk-bark","sectionFaces":["-u"]}]
    if den==500:
        lobes = ["core-low","core-high","top","r1-0","r1-180","r1l-90","r1l-270","r2-0","r2-180","r2l-90","r2l-270","r3-0","r3-180","r4-90","r4-270","epi-e","epi-w"]
        for name in lobes:
            s=next(s for s in fine["body"] if s["id"]=="lobe-"+name)
            n=len(s["sections"])
            vol["body"].append(shortened(s,[0,n//3,2*n//3,n-1]))
            t=next(t for t in fine["thermal"] if t.get("of")==s["id"])
            vol["thermal"].append(t)
    wording = {50:("four tiers of branch-end leaf clusters and the epicormic tufts","vier Etagen von Blattgruppen an den Astenden sowie die Wasserreiser"),100:("the tiered leaf clusters around the middle and upper leaders","die gestuften Blattgruppen am mittleren und oberen Leittrieb"),200:("the main crown lobes and low shoots around the clear stem","die großen Kronenpartien und die niedrigen Austriebe am Schaft"),500:("the rounded outer lobes, shaded crown core and epicormic tufts","die gerundeten äußeren Kronenpartien, den schattigen Kern und die Wasserreiser")}[den]
    vol["decisions"]=[{"en":"The clear stem meets the movable lower leader at its branch collar. The crown follows "+wording[0]+". Sunlit and shaded foliage exchange heat on their own surfaces. Woody members carry their loads to the root plate.","de":"Am Astkragen geht der Schaft in den beweglichen unteren Leittrieb über. Die Krone umfasst "+wording[1]+". Besonnte und beschattete Blätter tauschen über eigene Flächen Wärme aus. Die Holzteile leiten die Lasten zum Wurzelteller."}]

for name in feet:
    path=next(catalog.glob("*/"+name+"/*.json"))
    if name=="labrador-dog":write_scale(path,200,ungulate)
    write_scale(path,500,ungulate)
for name,scales,update in [("small-leaved-lime",[500,200,100,50],lime),("miscanthus-grass",[500,200,100],grass),("agave",[200,500],agave),("scots-pine",[200,500],spruce_axes)]:
    for scale in scales:write_scale(next(catalog.glob("*/"+name+"/*.json")),scale,update)



def bamboo(asset,vol):
    for s in vol["body"]:
        if s.get("material")=="bamboo-leaf" and s.get("role")=="texture" and len(s.get("sections",[]))==8:
            s["sections"]=[s["sections"][i] for i in [0,1,2,4,6,7]]
write_scale(next(catalog.glob("*/bamboo-clump/*.json")),50,bamboo)
