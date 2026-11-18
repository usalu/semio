import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/vintage-car-1929")
print(d.rects_from_volume(200, "plan", "frame-pair", "frame-pair", ["axle-*", "running-bracket-*", "body-mount-*", "bracket-*", "lamp-stalk-*", "spare-mount", "headlamp-bowl-*", "tail-lamp"], role="detail"))
d.add_shapes(200, "front", "frame", "frame", [{"kind": "ellipse", "cx": 0.55, "cy": 0.99, "rx": 0.11, "ry": 0.11, "id": "headlamp-bowl-l"}, {"kind": "ellipse", "cx": -0.55, "cy": 0.99, "rx": 0.11, "ry": 0.11, "id": "headlamp-bowl-r"}, {"kind": "rect", "x": 0.533, "y": 0.783, "w": 0.035, "h": 0.134, "id": "lamp-stalk-l"}, {"kind": "rect", "x": -0.568, "y": 0.783, "w": 0.035, "h": 0.134, "id": "lamp-stalk-r"}], role="detail")
d.commit()
