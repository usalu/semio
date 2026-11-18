import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bus")
d.to_poly(500, "elevation", "body-shell", "*,!tyre,!rim,!hub,!stud,!drum,!wheel,!mirror", sys.argv[1])
d.commit()
