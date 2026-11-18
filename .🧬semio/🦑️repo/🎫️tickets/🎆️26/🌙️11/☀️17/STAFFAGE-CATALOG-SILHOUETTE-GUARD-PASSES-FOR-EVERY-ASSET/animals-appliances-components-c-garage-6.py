from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/sectional-garage-door")
L.dup(d, 50, "plan", "track-left", "shaft-end-left", x=0.1075, y=1.2, w=0.025, h=0.05, z=1.2)
L.dup(d, 50, "plan", "track-left", "shaft-end-right", x=0.1075, y=-1.25, w=0.025, h=0.05, z=1.2)
d.commit()
