import sys, importlib
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/city-bicycle")
sc = 50
front = [("fender-r", "=fender-r"), ("rack", "=rack-deck,=rack-rim"), ("tail-lamp", "=tail-lamp"), ("saddle", "=saddle,=seat-lug,=seat-post"), ("chaincase", "=chaincase,=chaincase-rim"), ("down-tube", "=main-tube"), ("seat-tube", "=seat-tube"), ("head-tube", "=head-tube,=head-cup-top,=head-cup-bottom")]
plan = [("fender-r", "=fender-r"), ("rack", "=rack-deck,=rack-rim"), ("tail-lamp", "=tail-lamp"), ("saddle", "=saddle,=seat-lug,=seat-post"), ("chaincase", "=chaincase,=chaincase-rim"), ("head-tube", "=head-tube,=head-cup-top,=head-cup-bottom")]
for sid, ids in front: d.to_poly(sc, "front", sid, ids, "0.004")
for sid, ids in plan: d.to_poly(sc, "plan", sid, ids, "0.004")
d.commit()
