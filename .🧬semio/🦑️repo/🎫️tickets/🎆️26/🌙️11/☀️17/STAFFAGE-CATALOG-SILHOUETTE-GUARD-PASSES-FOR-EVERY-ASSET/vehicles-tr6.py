import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/tram")
for sc in (50, 100, 200, 500):
    try:
        print(sc, d.rects_from_volume(sc, "elevation", "shell-end", "shell-end", ["bogie-side-beam-r-l", "bogie-side-beam-f-l"], role="detail"))
    except KeyError as e:
        print(sc, "no", e)
d.commit()
