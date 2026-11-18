from importlib import import_module
L = import_module("animals-appliances-components-c-lib")
d = L.D("🧱️components/sliding-door")
L.dup(d, 50, "plan", "leaf-glass", "leaf-rail-below", x=1.05, y=-0.025, w=0.9, h=0.05, role="body")
L.dup(d, 50, "plan", "leaf-stile-east", "handle-base-cut", x=1.89, y=-0.013, w=0.06, h=0.026)
L.dup(d, 50, "plan", "leaf-stile-east", "handle-grip-cut", x=1.9, y=-0.033, w=0.03, h=0.066)
d.commit()
