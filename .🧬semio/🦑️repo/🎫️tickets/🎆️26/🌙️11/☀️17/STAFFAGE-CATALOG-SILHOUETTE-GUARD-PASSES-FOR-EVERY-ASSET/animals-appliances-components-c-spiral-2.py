from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/spiral-stair")
s100 = d.obj(d.field(d.solid(100, "handrail"), "sections"))
L.replace_value(d, d.field(d.solid(200, "handrail"), "sections"), s100)
d.commit()
