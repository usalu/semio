import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
for sc in (50, 100):
    s = d.shape(sc, "front", "tail-pipe")
    o = d.obj(s)
    sil = d.obj(d.shape(sc, "front", "silencer"))
    extra = {k: sil[k] for k in ("z", "tone", "region", "material") if k in sil}
    new = {k: v for k, v in o.items()}
    for k, v in extra.items(): new.setdefault(k, v)
    d.replace_shape(s, new)
d.commit()
