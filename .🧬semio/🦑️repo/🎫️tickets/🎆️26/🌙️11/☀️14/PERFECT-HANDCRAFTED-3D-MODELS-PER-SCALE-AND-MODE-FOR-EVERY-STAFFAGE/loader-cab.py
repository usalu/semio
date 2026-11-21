"""🚜 Publish the wheel loader's individually drawn cab fittings without replacing its other authored geometry."""
from pathlib import Path
import json
import runpy
import r3_wheel_loader_model as loader

ticket=Path(__file__).resolve().parent
io=runpy.run_path(str(ticket/"equipment-sections.py"))
live,asset=io["document"]("wheel-loader")
mirror=ticket/"volumes/machinery/wheel-loader.json"

def fitting(item):
    return item["id"].startswith(("cab-door-gasket","cab-wiper-","th-cab-door-gasket","th-cab-wiper-"))

def merge(existing, authored):
    return [s for s in existing if not fitting(s)] + [s for s in authored if fitting(s)]

for path,root in [(live,["asset"]),(mirror,[])]:
    doc=json.loads(path.read_text())
    a=doc["asset"] if root else doc
    edits=[]
    for den in (50,100):
        bodies,sheet=loader.build(den)
        new=[{k:v for k,v in s.items() if not k.startswith("_")} for s in bodies if fitting(s)]
        thermal=[{"id":"th-"+s["id"],"of":s["id"],"region":"cab","layer":"sheet"} for s in new]
        d=str(den)
        for kind,items in [("body",new),("thermal",thermal)]:
            edits.append((root+["volumes",d,kind],merge(a["volumes"][d][kind],items)))
        if root:
            for view,items in sheet.out().items():
                old=a["representations"][d]["geometry"][view]
                edits.append((root+["representations",d,"geometry",view],sorted(merge(old,items),key=lambda s:s["z"])))
    if root:
        i=next(i for i,r in enumerate(a["thermal"]["regions"]) if r["id"]=="cab")
        edits.append((root+["thermal","regions",i,"areaM2"],10.4))
    io["write"](path,edits)
areas=ticket/"r3-wheel-loader-areas.json"
values=json.loads(areas.read_text())
values["cab"]=10.4
areas.write_text(json.dumps(values,ensure_ascii=False)+"\n")
