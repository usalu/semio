"""🌲 Refine distal woodland crowns, ears, horns and load-bearing skeletons from the authored species geometry."""
import runpy
from pathlib import Path

helpers=runpy.run_path(str(Path(__file__).with_name("living-articulation.py")))
catalog,shortened,write_scale=(helpers[k] for k in ("catalog","shortened","write_scale"))

def forest(asset,vol):
    den=vol["denominator"]
    if asset["id"]=="ivy-on-trellis":
        src=asset["volumes"]["200"]
        vol["body"]=[shortened(s,[0,2,4] if s["id"].startswith("mat-") else None) for s in src["body"]]
        vol["thermal"]=src["thermal"]
        text=("Four climbing stems spread into upper and lower foliage fans around the timber frame. Openings at the base expose the rails and steel shoes.","Vier Klettertriebe breiten sich am Holzrahmen in obere und untere Blattfächer aus. Öffnungen am Fuß geben Riegel und Stahlschuhe frei.")
    else:
        src=asset["volumes"]["50"]
        crowns=asset["volumes"]["200"]
        stem_names={"stool","stem-l-1","stem-l-2","stem-r-1","stem-r-2","stem-c-1","stem-c-2"}
        wood=[b for b in src["body"] if b["id"] in stem_names]
        branches=[b for b in crowns["body"] if b["id"].startswith("br-")]
        lobes=[b for b in crowns["body"] if b["id"].startswith("lobe-") and (den==200 or b["id"] not in {"lobe-out-s2-l-a","lobe-out-s3-r-a-2","lobe-out-s4-c-a-1"})]
        vol["body"]=[]
        for b in [*wood,*branches,*lobes]:
            n=len(b["sections"])
            ix=sorted({0,n//3,2*n//3,n-1}) if den==500 else sorted({0,n//5,2*n//5,3*n//5,4*n//5,n-1})
            vol["body"].append(shortened(b,ix))
        refs={s["of"]:s for s in [*src["thermal"],*crowns["thermal"]] if s.get("of")}
        vol["thermal"]=[refs[b["id"]] for b in vol["body"] if b["id"] in refs]+[t for t in crowns["thermal"] if "emitter" in t.get("tags",[])]
        if den==500:
            members=[s for s in vol["structure"] if s.get("part")!="mass"]
            vol["structure"]=members+[s for s in crowns["structure"] if s.get("part")=="mass"]
        text=("Three two-part birch stems rise separately from the stool. Each carries its own branch tips and hanging crown, including the sparser gaps between stems.","Drei zweiteilige Birkenstämme steigen einzeln aus dem Stock auf. Jeder trägt eigene Astspitzen und überhängende Kronenteile mit lichten Zwischenräumen.")
    vol["decisions"]=[{"en":text[0],"de":text[1]}]

def detail(asset,vol):
    slug=asset["id"]
    src=asset["volumes"]["100"]
    by_id={s["id"]:s for s in src["body"]}
    parts,drop=[],[]
    for sw,side in [("near","n"),("far","f")]:
        if slug=="roe-deer":
            drop += [sw+"-antler",sw+"-ear"]
            parts+=[(sw+"-ear",[0,2,4],"ears"),(sw+"-antler-beam",[0,2,4,5],None),(sw+"-antler-brow",None,None),(sw+"-antler-back",None,None)]
        elif slug in {"pig","sheep"}:parts.append((sw+"-ear",[0,2,3],"ears"))
        elif slug=="labrador-dog":parts.append(("ear-"+side,[0,2,5,8],"ears"))
        elif slug=="goat":
            drop+=["horn-"+side,"ear-"+side]
            parts += [("horn-"+side,[0,2,4,6],"horns"),("ear-"+side,[0,3,6],"ears")]
    if slug=="pig":parts+=[("tail",[0,2,4,5],"tail"),("snout",None,"snout")]
    if slug=="sheep":parts.append(("tail",[0,2,3],"tail"))
    if slug=="goat":parts.append(("beard",[0,1,2],"coat-head"))
    replace=set(drop)|{p[0] for p in parts}
    vol["body"]=[s for s in vol["body"] if s["id"] not in replace]
    regions={r["id"] for r in asset["thermal"]["regions"]}
    represented={p[2] for p in parts if p[2] in regions and p[2] not in {"coat-head","fleece-trunk"}}
    vol["thermal"]=[t for t in vol["thermal"] if t.get("of") not in replace and t["id"] not in {"th-"+p for p in replace} and (t["region"] not in represented or "emitter" in t.get("tags",[]))]
    for name,rows,region in parts:
        vol["body"].append(shortened(by_id[name],rows))
        if region in regions:
            vol["thermal"].append({"id":"th-"+name,"of":name,"region":region})
    s2=asset["volumes"]["200"]["structure"]
    keep_ribs={"pig":{"2","6","10"},"sheep":{"2","10"},"labrador-dog":{"1","3"},"goat":{"1","3"},"roe-deer":{"1","3"}}[slug]
    result=[]
    for s in s2:
        if s.get("part")=="joint":continue
        if "rib-" in s["id"] and s["id"].split("-")[-1] not in keep_ribs:continue
        if slug=="roe-deer" and s.get("of"):
            if s["of"].endswith("-antler"):
                prefix=s["of"]+"-"
                result.extend({"id":"bone-"+prefix+t,"of":prefix+t,"member":"head","part":"member"} for t in ("beam","brow","back"))
            continue
        n=len(s.get("sections",[]))
        if n>5:
            if "rib-" in s["id"]:ix=sorted({0,n//4,n//2,3*n//4,n-1})
            else:ix=sorted({0,n//3,2*n//3,n-1})
        else:ix=None
        result.append(shortened(s,ix))
    vol["structure"]=result

for slug in ("pig","sheep","goat","roe-deer","labrador-dog"):
    write_scale(next(catalog.glob("*/"+slug+"/*.json")),500,detail)
for slug,scales in [("multi-stem-birch",[500,200]),("ivy-on-trellis",[500])]:
    for scale in scales:write_scale(next(catalog.glob("*/"+slug+"/*.json")),scale,forest)


def rib_loop(asset,vol):
    for item in vol["structure"]:
        if item["id"].startswith("rib-") and item.get("kind")=="revolve" and item["profile"][0]!=item["profile"][-1]:
            item["profile"].append(item["profile"][0][:])
for slug in ("goat","labrador-dog"):
    for scale in [50,100,200,500]:write_scale(next(catalog.glob("*/"+slug+"/*.json")),scale,rib_loop)
