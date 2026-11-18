"""Tower crane 1:200 and 1:100: the winch house vent stack (0.4 m wide, 0.2 m high on the roof) is in the volume but in no drawing; elevation and front draw it as the extruded rectangle of the winch bone, the plan as the 0.4 m circle at the winch position (x = -8 m)."""
from mvlib import *
d = D("🏗️machinery/tower-crane")
z = {200: 1.9, 100: 3.4}
for sc in (200, 100):
    for view in ("elevation", "front", "plan"):
        last = d.at("asset", "representations", str(sc), "geometry", view)["items"][-1]
        if view == "plan":
            o = {"id": "winch-vent-stack", "bone": "winch", "role": "detail", "z": z[sc], "filled": True, "tone": 0.3, "kind": "ellipse", "cx": -8, "cy": 0, "rx": 0.2, "ry": 0.2, "material": "steel-s355"}
        else:
            o = {"id": "winch-vent-stack", "bone": "winch", "role": "detail", "z": z[sc], "filled": True, "tone": 0.3, "kind": "rect", "x": -0.2, "y": 1.8, "w": 0.4, "h": 0.2, "material": "steel-s355", "solid": {"kind": "extruded", "sweep": "w", "z0": -0.2, "z1": 0.2}}
        d.insert_after(last, o)
d.commit()
