from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/sectional-garage-door")
L.dup(d, 100, "plan", "track-left", "hinge-band", x=0.02, y=-1.21, w=0.025, h=2.42, z=1.2)
L.dup(d, 50, "plan", "track-left", "hinge-knuckle-band", x=0.022, y=-1.21, w=0.023, h=2.42, z=1.2)
d.commit()
