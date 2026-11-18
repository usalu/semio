import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-car")
d.sub(d.field(d.shape(50, "elevation", "body-shell"), "d"), "0.25,1.46 C 0.1,1.5 -0.4,1.5", "0.25,1.46 L 0.22,1.466 L 0.22,1.488 C 0.1,1.5 -0.4,1.5")
d.sub(d.field(d.shape(50, "elevation", "roof-band"), "d"), "M 0.25,1.46 C 0.1,1.5 -0.4,1.5", "M 0.25,1.46 L 0.22,1.466 L 0.22,1.488 C 0.1,1.5 -0.4,1.5")
d.commit()
