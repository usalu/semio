"""☀️ Export registered rooflight samples for independent native IFC verification."""
import importlib,json
from pathlib import Path
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
exchange=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
model=api.load_model()
obj=api.element("ROOFLIGHT-1",model)
parts=api.solid_geometry("ROOFLIGHT-1",model)["parts"]
vertices=[v for p in parts for v in api.part_vertices(p)]
samples=[]
for lang in ("en","de"):
    native=exchange.step_excerpt("ROOFLIGHT-1",model,language=lang)
    samples.append(dict(tag="ROOFLIGHT-1",language=lang,entity="IFCBUILDINGELEMENTPROXY",name=obj["name"][lang],text="\n".join(native["lines"]),ids=native["ids"],volume=api.quantities("ROOFLIGHT-1",model)["volume"],parts=len(parts),historical=obj["historical_evidence"]["state"],bounds=[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]],roles=sorted({p["role"] for p in parts}),materials=sorted({api.element(p["material"],model)["name"][lang] for p in parts})))
Path(__file__).with_name("🗑️generated").joinpath("rooflight-native-r27.json").write_text(json.dumps(samples),encoding="utf-8")
print([(s["language"],s["parts"],s["volume"]) for s in samples])
