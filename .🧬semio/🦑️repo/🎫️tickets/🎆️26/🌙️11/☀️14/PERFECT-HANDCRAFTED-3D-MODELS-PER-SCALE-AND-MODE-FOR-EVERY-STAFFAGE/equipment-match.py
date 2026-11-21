"""🚲 Resolve the city bicycle cables and the exposed contour of the slotted glass-balustrade rail."""
import json
from pathlib import Path
import runpy
import sys

ticket=Path(__file__).resolve().parent
io=runpy.run_path(str(ticket/"equipment-sections.py"))

def rail_skin(volumes):
    for d in ("50","100"):
        v=volumes[d]
        bodies={s["id"]:s for s in v["body"]}
        tube=bodies["handrail-tube"]
        cap_ids=("handrail-cap-left","handrail-cap-right") if d=="50" else ("handrail-plug-left","handrail-plug-right")
        arc=tube["profile"][:len(tube["profile"])//2]
        if d=="50":
            gasket=bodies["handrail-gasket"]["profile"]
            arc=arc+[gasket[0],gasket[3]]
        shell={**tube,"id":"th-handrail-exterior","profile":arc,"region":"handrail-steel"}
        caps=[{"id":"th-"+key,"of":key,"region":"handrail-steel"} for key in cap_ids]
        v["thermal"]=[s for s in v["thermal"] if not s["id"].startswith("th-handrail-")]+[shell,*caps]
        text={"en":"The thermal rail follows the exposed arc of the slotted tube. Its slot faces the glass. The end closures terminate the rail and the sealed bore exchanges no heat with outside air.","de":"Die thermische Handlauffläche folgt dem Außenbogen des geschlitzten Rohrs. Sein Schlitz liegt am Glas. Die Endkappen schließen den Handlauf ab. Die geschlossene Innenbohrung steht ohne Luftaustausch zur Außenluft."}
        if text not in v["decisions"]:v["decisions"].append(text)

if __name__=="__main__":
    p,bike=io["document"]("city-bicycle")
    bs=ticket/"volumes/vehicles/city-bicycle.json";bv=json.loads(bs.read_bytes())["volumes"]
    ids={"cable-front","cable-rear-loop","cable-rear"}
    additions=[s for s in bv["50"]["body"] if s["id"] in ids]
    assert len(additions)==3
    current=[s for s in bike["volumes"]["50"]["body"] if s["id"] not in ids]+additions
    assert {s["id"]:s for s in current}=={s["id"]:s for s in bv["50"]["body"]}
    io["write"](p,[ (["asset","volumes","50","body"],bv["50"]["body"]),(["asset","volumes","50","decisions"],bv["50"]["decisions"]),(["asset","volumes","500","decisions"],bv["500"]["decisions"])])
    p,a=io["document"]("glass-balustrade")
    source=ticket/"volumes/components/glass-balustrade.json"
    src=json.loads(source.read_bytes())["volumes"]
    for d,fields in [("50",["body","thermal","decisions"]),("100",["thermal","decisions"])]:
        if d=="50":
            cs={s["id"]:s for s in src[d]["body"] if s["id"] in ("handrail-cap-left","handrail-cap-right")}
            for i,s in enumerate(a["volumes"][d]["body"]):
                if s["id"] in cs:a["volumes"][d]["body"][i]=cs[s["id"]]
        a["volumes"][d]["decisions"]=src[d]["decisions"].copy()
    rail_skin(a["volumes"])
    register=[(d,k) for d,fields in [("50",["body","thermal","decisions"]),("100",["thermal","decisions"])] for k in fields]
    if "--apply" in sys.argv:
        io["write"](p,[(["asset","volumes",d,k],a["volumes"][d][k]) for d,k in register])
        io["write"](source,[(["volumes",d,k],a["volumes"][d][k]) for d,k in register])
    else:
        candidate=ticket/"🗑️generated/equipment-check/glass-exterior.json"
        candidate.write_text(json.dumps({"asset":a},ensure_ascii=False))
