"""🏗️ Export exact affected native roof columns, structural slab and beam members."""
import importlib,json
from pathlib import Path
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
exchange=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
model=api.load_model()
samples=[]
for tag in ("10-FIRST","15-FIRST","SL-R","BEAM-PRIMARY-R-4-10","BEAM-PRIMARY-R-10-11","BEAM-PRIMARY-R-10-15","BEAM-PRIMARY-R-34-10","BEAM-PRIMARY-R-14-15"):
    obj=api._find(model,tag)
    physical=api.column_geometry(model,obj) if obj["class"]=="IfcColumn" else api._slab_geometry(model,obj) if obj["class"]=="IfcSlab" else api.solid_geometry(tag,model)
    parts=physical["parts"]
    vertices=[v for p in parts for v in api.part_vertices(p)]
    quantity=api.quantities(tag,model)
    for lang in ("en","de"):
        native=exchange.step_excerpt(tag,model,language=lang)
        samples.append(dict(tag=tag,language=lang,entity=obj["class"].upper(),guid=obj["guid"],name=obj["name"][lang],material_name=next(m for m in model["materials"] if m["tag"]=="MAT-RC")["name"][lang],text="\n".join(native["lines"]),ids=native["ids"],volume=quantity.get("exclusive_volume",quantity["volume"]),full_volume=quantity["volume"],shared_volume=quantity.get("shared_volume",0),finish_volume=quantity.get("finish_volume",0),owned_finish_volume=quantity.get("exclusive_finish_volume",0),shared_finish_volume=quantity.get("shared_finish_volume",0),parts=len(parts),owners=list(dict.fromkeys(obj["geometry"].get("embedded_slabs",[])+obj["geometry"].get("joint_owners",[])+obj["geometry"].get("column_reserves",[])+[p["owner"] for p in physical.get("shared_parts",[]) if "owner" in p])),bounds=[[min(v[i] for v in vertices) for i in range(3)],[max(v[i] for v in vertices) for i in range(3)]],circle_components=[dict(part=i,axis=obj["geometry"]["axis"],radius=p["profile"]["diameter"]["value"]/2,height=p["z1"]-p["z0"]) for i,p in enumerate(parts) if p.get("profile",{}).get("shape")=="circle"]))
Path(__file__).with_name("🗑️generated").joinpath("roof-bearing-native-r43.json").write_text(json.dumps(samples),encoding="utf-8")
print([(s["tag"],s["language"],s["parts"],s["volume"]) for s in samples])
