from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/sectional-garage-door")
T = [[-0.05, -0.02], [-0.05, -0.028], [-0.0115, -0.028], [-0.0115, -0.045], [0.0115, -0.045], [0.0115, -0.028], [0.05, -0.028], [0.05, -0.02]]
for s in ("hinge-1", "hinge-2", "hinge-3"):
    n = d.shape(50, "elevation", s); o = d.obj(n); o["points"] = T; d.replace_obj(n, o)
d.commit()
