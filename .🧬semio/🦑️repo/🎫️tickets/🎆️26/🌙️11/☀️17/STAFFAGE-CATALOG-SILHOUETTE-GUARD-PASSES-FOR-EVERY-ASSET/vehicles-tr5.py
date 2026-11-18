import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/tram")
for sc in (50, 100, 200, 500):
    print(sc, d.rects_from_volume(sc, "front", "shell-end", "shell-end", ["axle-ws-r1", "brake-disc-ws-r1-l", "brake-disc-ws-r1-r"], role="detail"))
d.commit()
