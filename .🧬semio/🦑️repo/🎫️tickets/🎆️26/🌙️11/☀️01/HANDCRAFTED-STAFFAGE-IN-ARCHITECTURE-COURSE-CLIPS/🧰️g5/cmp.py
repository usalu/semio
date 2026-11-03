import sys, json, pathlib, importlib.util
p = pathlib.Path("🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py")
spec = importlib.util.spec_from_file_location("stf", p); m = importlib.util.module_from_spec(spec); sys.modules["stf"]=m; spec.loader.exec_module(m)
root = pathlib.Path(".🧬semio/🎓️teaching/⚡️cache/staffage")
for f in root.glob("*/*/🔣️.json"):
    try: i = json.loads(f.read_text(encoding="utf-8"))
    except Exception: continue
    if i.get("asset") == "external-venetian-blind" and i.get("cast", "").endswith(("fundamentals-blind", "solar-gains-blind")):
        d = m._parts(f.parent, "")
        print(i["cast"], i["view"], i["mmPerMetre"], i["originMm"], {k: d[k].min(axis=0).round(3).tolist()+d[k].max(axis=0).round(3).tolist() for k in ("slat-0","box","bottom-rail","block-sill") if k in d}, f.parent.parent.name)
