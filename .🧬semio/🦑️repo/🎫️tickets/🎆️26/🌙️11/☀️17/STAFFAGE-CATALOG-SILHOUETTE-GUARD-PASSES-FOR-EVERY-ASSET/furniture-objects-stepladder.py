import importlib.util, os, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🪑️objects", "stepladder")
n = d.shape(200, "elevation", "platform"); o = d.obj(d.shape(200, "elevation", "rear-rail"))
o.update({"id": "spreader", "bone": "front-frame", "z": 3.1, "points": [[-0.2535, 0.6389], [0.0935, 0.5189], [0.0805, 0.4811], [-0.2665, 0.6011]], "regions": ["spreaders"]}); o.pop("penMm", None)
d.insert_after(n, o)
d.commit()
