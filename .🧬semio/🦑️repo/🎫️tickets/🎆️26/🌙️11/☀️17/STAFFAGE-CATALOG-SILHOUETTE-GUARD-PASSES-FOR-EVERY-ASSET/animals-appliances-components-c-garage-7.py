from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/sectional-garage-door")
for i, c in enumerate((-1.15, -0.383333, 0.383333, 1.15)):
    n = str(i + 1)
    L.dup(d, 50, "plan", "track-left", "hinge-block-" + n, x=0.02, y=round(c - 0.06, 6), w=0.01, h=0.12, z=1.2)
    L.dup(d, 50, "plan", "track-left", "hinge-knuckle-a-" + n, x=0.03, y=round(c - 0.06, 6), w=0.015, h=0.01, z=1.2)
    L.dup(d, 50, "plan", "track-left", "hinge-knuckle-b-" + n, x=0.03, y=round(c + 0.05, 6), w=0.015, h=0.01, z=1.2)
d.commit()
