"""🛝️ Selected native ramp assemblies and diagonal flight samples for an independent IFC reader."""
import importlib,json
from pathlib import Path
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
exchange=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
samples=[]
def record(tag,model,case):
    obj=api._find(model,tag)
    geometry=api.ramp_geometry(tag,model)
    quantity=api.quantities(tag,model)
    def product(key):
        source=api._find(model,key)
        physical=api.ramp_geometry(key,model)
        vertices=[v for p in physical["parts"] for v in api.part_vertices(p)]
        return dict(tag=key,guid=source["guid"],entity=source["class"].upper(),parts=len(physical["parts"]),volume=api.quantities(key,model)["exclusive_volume"],bounds=[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]])
    for lang in ("en","de"):
        native=exchange.step_excerpt(tag,model,language=lang)
        children=[{**product(c["tag"]),"ids":c["ids"]} for c in native.get("children",[])]
        samples.append({**product(tag),"case":case,"language":lang,"name":obj["name"][lang],"text":"\n".join(native["lines"]),"ids":native["ids"],"children":children,"full_volume":quantity["volume"],"shared_volume":quantity["shared_volume"],"child_volume":quantity["child_volume"],"shared_refs":"; ".join(p["owner_guid"]+"/"+p["tag"] for p in geometry["shared_parts"]),"relation_guid":next((r["guid"] for r in model["relations"] if r["class"]=="IfcRelAggregates" and r["relating"]==tag),None)})
model=api.load_model()
for tag in ("RM-1","RM-2","RF-1A","RF-1B","RF-2A","RF-2B"):
    record(tag,model,"source")
model=api.load_model()
obj=api._find(model,"RF-1A")
obj["geometry"].update(path=[[0,0],[3,4]],from_level=0,to_level=1,portion=[0,1])
obj["geometry"]["thickness"]["value"]=.2
obj["attributes"]["width"]["value"]=2
record("RF-1A",model,"diagonal")
Path(__file__).with_name("🗑️generated").joinpath("ramp-native-r31.json").write_text(json.dumps(samples),encoding="utf-8")
print([(s["tag"],s["case"],s["language"],s["volume"],s["child_volume"]) for s in samples])
