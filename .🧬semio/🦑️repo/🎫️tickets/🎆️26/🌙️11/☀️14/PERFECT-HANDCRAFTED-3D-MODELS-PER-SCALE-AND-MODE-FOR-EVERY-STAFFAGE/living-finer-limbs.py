"""🦊 Expose the fox's upper limbs and the wings and beaks of the two birds at the intermediate scale."""
import runpy
from pathlib import Path
h=runpy.run_path(str(Path(__file__).with_name("living-articulation.py")))
def additions(a,v):
    specs={
        "red-fox":[("near-thigh",100,[0,1,2,3,4],"coat-legs"),("near-shoulder",100,[0,1,2,3,4],"coat-legs"),("near-upper-arm",50,[0,2,5,9,12],"coat-legs")],
        "mallard-duck":[("near-primaries",100,[0,2,4],"wing-plumage"),("rump",100,[0,2,4,6],"body-plumage")],
        "pigeon":[("beak",100,[0,2,3],"beak"),("cere",100,[0,2],"cere"),("near-hallux",50,[0,2,4],"legs-feet")]
    }[a["id"]]
    for k,scale,indices,region in specs:
        for name in (k,k.replace("near-","far-")) if k.startswith("near-") else (k,):
            if name in {x["id"] for x in v["body"]}:continue
            part=next(x for x in a["volumes"][str(scale)]["body"] if x["id"]==name)
            v["body"].append(h["shortened"](part,indices))
            if name in {"beak","cere"}:
                v["thermal"]=[s for s in v["thermal"] if s["region"]!=region or 'emitter' in s.get("tags",[])]
            v["thermal"].append({"id":"th-"+name,"of":name,"region":region})
for name in ["pigeon","mallard-duck","red-fox"]:
    h["write_scale"](next(h["catalog"].glob("*/"+name+"/*.json")),200,additions)

