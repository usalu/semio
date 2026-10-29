"""🏗️ Export native occupied beam bodies while preserving source full-member semantics."""
import importlib,json
from pathlib import Path
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
exchange=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
model=api.load_model()
samples=[]
for tag in ("BEAM-PRIMARY-F-2-7","BEAM-PRIMARY-F-4-10","BEAM-PRIMARY-R-2-7","BEAM-PRIMARY-R-4-10","BEAM-PRIMARY-R-6-22","BEAM-PRIMARY-R-20-32"):
    obj=api.element(tag,model)
    physical=api.solid_geometry(tag,model)
    vertices=[v for p in physical["parts"] for v in api.part_vertices(p)]
    for lang in ("en","de"):
        native=exchange.step_excerpt(tag,model,language=lang)
        samples.append(dict(tag=tag,language=lang,entity=obj["class"].upper(),name=obj["name"][lang],text="\n".join(native["lines"]),ids=native["ids"],volume=api.quantities(tag,model)["exclusive_volume"],parts=len(physical["parts"]),full_volume=api.quantities(tag,model)["volume"],shared_volume=api.quantities(tag,model)["shared_volume"],owners=list(dict.fromkeys(obj["geometry"].get("embedded_slabs",[])+obj["geometry"].get("joint_owners",[])+[p["owner"] for p in physical["shared_parts"] if "owner" in p])),bounds=[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]]))
Path(__file__).with_name("🗑️generated").joinpath("joint-native-r30.json").write_text(json.dumps(samples),encoding="utf-8")
print([(s["tag"],s["language"],s["parts"],s["volume"]) for s in samples])
