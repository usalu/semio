"""🚰 Export test-only service samples through the public physical and native exchange interfaces."""
from copy import deepcopy
import importlib
import json
from pathlib import Path

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
exchange=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
model=api.load_model()
for tag,points in (("TEST-SERVICE-X",[[1,2,3],[4,2,3]]),("TEST-SERVICE-Y",[[2,1,3],[2,4,3]])):
    pipe=deepcopy(api.element("DP-1",model))
    pipe.update(tag=tag,guid=api.global_id(tag),material="MAT-METAL")
    pipe["name"]={"en":tag,"de":tag}
    pipe["geometry"]={"points":points,"tube":{"wall_thickness":{"value":.02,"unit":"m","basis":"inferred"},"segments":96}}
    pipe["attributes"]={"diameter":{"value":.2,"unit":"m","basis":"inferred"}}
    model["elements"].append(pipe)
samples=[]
for tag in ("DP-1","DP-1-OUTLET","HEAT-RISER-6-CASING","SOIL-STAIR","BOILER-FLUE-VISIBLE","BL-1","TEST-SERVICE-X","TEST-SERVICE-Y"):
    obj=api.element(tag,model)
    physical=api.service_geometry(tag,model)
    vertices=[v for p in physical["parts"] for v in api.part_vertices(p)]
    for language in ("en","de"):
        native=exchange.step_excerpt(tag,model,language=language)
        samples.append({"tag":tag,"language":language,"entity":obj["class"].upper(),"name":obj["name"][language],"text":"\n".join(native["lines"]),"ids":native["ids"],"volume":api.quantities(tag,model)["volume"],"parts":len(physical["parts"]),"historical":obj["psets"]["Pset_HistoricalEvidence"]["TargetState"]["value"],"bounds":[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]]})
Path(__file__).with_name("🗑️generated").joinpath("native-services-r21.json").write_text(json.dumps(samples),encoding="utf-8")
print([(s["tag"],s["language"],s["parts"],s["volume"]) for s in samples])
