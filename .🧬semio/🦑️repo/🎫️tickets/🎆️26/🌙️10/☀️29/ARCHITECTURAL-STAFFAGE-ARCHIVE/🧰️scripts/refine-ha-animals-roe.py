import importlib.util, json
T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/"
spec = importlib.util.spec_from_file_location("lib", T + "🧰️scripts/refine-ha-animals-lib.py")
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
D = lib.Doc(); A = "roe-deer"
EN = " Dynamic snapshot: the centre of mass is outside the support, so the moment is not statically balanced."
DE = " Dynamische Momentaufnahme: Der Schwerpunkt liegt außerhalb der Unterstützung, daher ist das Moment nicht statisch im Gleichgewicht."
def dyn(o):
    if EN.strip() not in o["description"]["en"]:
        o["description"] = {"en": o["description"]["en"] + EN, "de": o["description"]["de"] + DE}
    if o["id"] == "bounding-pushoff": o["bones"]["hoof-hf"] = {"angleDeg": 2}
    return o
print(D.edit(A, "poses", lambda o: o["id"] in ("bounding-gathered", "bounding-pushoff") or o["id"].startswith("bound-phase-"), dyn))
D.save()
D = lib.Doc()
D.edit(A, "poses", lambda o: o["id"] == "bound-phase-5", lambda o: {**o, "bones": {**o["bones"], "cannon-hf": {"angleDeg": 50}}}); D.save()
