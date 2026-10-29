"""🕳️ Export selected native bore IFC samples for an independent engine, not source copies."""
import importlib,json
from pathlib import Path
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
m=api.load_model()
samples=[]
for tag in ("2","4","2-FIRST","4-FIRST"):
    p=api.column_geometry(m,next(e for e in m["elements"] if e["tag"]==tag))
    vertices=[v for part in p["parts"] for v in api.part_vertices(part)]
    bounds=[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]]
    for language in ("en","de"):
        step=api.step_excerpt(tag,m,language=language)
        samples.append({"tag":tag,"language":language,"text":"\n".join(step["lines"]),"ids":step["ids"],"parts":len(p["parts"]),"hollows":sum(bool(part["holes"]) for part in p["parts"]),"profiles":[{"shape":part["profile"]["shape"],"outer_radius":part["profile"].get("diameter",{}).get("value",0)/2,"radii":[h["diameter"]["value"]/2 for h in part["profile"].get("voids",())]} for part in p["parts"]],"volume":api.quantities(tag,m)["volume"],"bounds":bounds})
from copy import deepcopy
cases=json.loads((Path(api.__file__).parent/"🧪️tests/🕳️profiles.json").read_text())["cases"]
for case in cases:
    trial=deepcopy(m)
    obj=next(e for e in trial["elements"] if e["tag"]=="19")
    next(t for t in trial["types"] if t["tag"]==obj["type"])["profile"]=case["profile"]
    obj["geometry"]={"axis":[0,0],"base_level":0,"top_level":2}
    p=api.column_geometry(trial,obj)
    vertices=[v for part in p["parts"] for v in api.part_vertices(part)]
    bounds=[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]]
    step=api.step_excerpt("19",trial)
    samples.append({"tag":case["name"],"language":"en","text":"\n".join(step["lines"]),"ids":step["ids"],"parts":1,"hollows":1,"volume":case["area"]*2,"bounds":bounds,"profiles":[{"shape":case["profile"]["shape"],"outer_radius":case["profile"].get("diameter",{}).get("value",0)/2,"radii":[h["diameter"]["value"]/2 for h in case["profile"]["voids"]]}]})
Path(__file__).with_name("🗑️generated").joinpath("native-bore-r22.json").write_text(json.dumps(samples),encoding="utf-8")
print("Native bore IFC samples",len(samples))
