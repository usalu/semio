import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/delivery-van")
for sc in (50, 100, 200):
    print(sc, d.rects_from_volume(sc, "front", "body-shell", "body-shell", ["slider-track-*", "handle-slider"], role="detail"))
d.commit()
