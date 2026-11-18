from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/sectional-garage-door")
d.delete_item(d.shape(100, "plan", "hinge-band"))
d.delete_item(d.shape(50, "plan", "hinge-knuckle-band"))
d.commit()
d = L.D("🧱️components/sectional-garage-door")
for i, z in enumerate((-1.15, -0.383333, 0.383333, 1.15)):
    L.dup(d, 100, "plan", "track-left", "hinge-block-%d" % (i + 1), x=0.02, y=round(z - 0.06, 6), w=0.025, h=0.12, z=1.2)
d.commit()
