import importlib.util, os, json
spec = importlib.util.spec_from_file_location("l", os.path.join(os.path.dirname(os.path.abspath(__file__)), "furniture-objects-lib.py")); l = importlib.util.module_from_spec(spec); spec.loader.exec_module(l)
d = l.doc("🛋️furniture", "operating-table")
n = d.solid(200, "col"); o = d.obj(n)
rot = [0.5, 0.5, 0.5, -0.5]
new = {k: v for k, v in o.items() if k not in ("size", "transform", "cornerRadius")}
new["kind"] = "loft"
new["sections"] = [{"at": [0, 0.0, 0], "rx": 0.16, "ry": 0.16, "exponent": 12, "rotation": rot}, {"at": [0, 0.62, 0], "rx": 0.11, "ry": 0.11, "exponent": 12, "rotation": rot}]
new["capStart"] = True; new["capEnd"] = True
d.replace_obj(n, new)
d.commit()
