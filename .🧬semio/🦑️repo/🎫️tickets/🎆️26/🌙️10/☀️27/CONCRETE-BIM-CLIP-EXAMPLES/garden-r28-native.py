"""🌬️ Export selected source garden products for independent native placed-solid checks."""
import importlib,json
from pathlib import Path
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
exchange=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
model=api.load_model()
samples=[]
for tag in ("16-FIRST","22-FIRST","GARDEN-WEATHER-CAP"):
    source=api._find(model,tag)
    obj=api.element(tag,model)
    physical=api.column_geometry(model,source) if source["class"]=="IfcColumn" else api.solid_geometry(tag,model)
    parts=physical["parts"]
    vertices=[v for p in parts for v in api.part_vertices(p)]
    for lang in ("en","de"):
        native=exchange.step_excerpt(tag,model,language=lang)
        samples.append(dict(tag=tag,language=lang,entity=obj["class"].upper(),name=obj["name"][lang],text="\n".join(native["lines"]),ids=native["ids"],volume=api.quantities(tag,model)["volume"],parts=len(parts),historical=obj["historical_evidence"]["state"],bounds=[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]],roles=sorted({p["role"] for p in parts}),materials=sorted({api.element(p["material"],model)["name"][lang] for p in parts})))
Path(__file__).with_name("🗑️generated").joinpath("garden-native-r28.json").write_text(json.dumps(samples),encoding="utf-8")
print([(s["tag"],s["language"],s["parts"],s["volume"]) for s in samples])
