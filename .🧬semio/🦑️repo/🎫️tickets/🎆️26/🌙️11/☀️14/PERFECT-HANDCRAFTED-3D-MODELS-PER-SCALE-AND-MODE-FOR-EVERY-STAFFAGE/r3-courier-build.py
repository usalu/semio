"""🏗️ Writes the handcrafted volumes of `humans/courier-with-hand-truck` to the author source and to the ad-hoc source the tools read with the candidate document: python3 r3-courier-build.py. Scales without a module yet keep what the source holds."""
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
paths = [os.path.join(T, "volumes", "humans", "courier-with-hand-truck.json"), os.path.join(T, "volumes", "adhoc", "r3-courier-with-hand-truck.json")]
doc = json.load(open(paths[0], encoding="utf-8"))
for scale in (50, 100, 200, 500):
    file = os.path.join(T, f"r3-courier-s{scale}.py")
    if not os.path.exists(file):
        continue
    spec = importlib.util.spec_from_file_location(f"s{scale}", file)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    doc["volumes"][str(scale)] = module.build()
    v = doc["volumes"][str(scale)]
    print(scale, len(v["body"]), len(v["thermal"]), len(v["structure"]))
for path in paths:
    json.dump(doc, open(path, "w", encoding="utf-8"), ensure_ascii=False)
