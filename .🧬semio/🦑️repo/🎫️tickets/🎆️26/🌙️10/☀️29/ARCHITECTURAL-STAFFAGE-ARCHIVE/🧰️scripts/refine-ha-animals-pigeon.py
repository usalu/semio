import importlib.util
spec = importlib.util.spec_from_file_location("lib", "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/refine-ha-animals-lib.py")
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
D = lib.Doc(); A = "pigeon"
def setb(o, d):
    for k, v in d.items(): o["bones"][k] = {"angleDeg": v}
    return o
D.edit(A, "anchors", lambda o: o["id"] in ("contact-n", "contact-f"), lambda o: {**o, "y": -0.0095})
D.edit(A, "poses", lambda o: o["id"] == "takeoff-liftoff", lambda o: setb(o, {"humerus-n": 20, "humerus-f": 20, "forearm-n": 130, "forearm-f": 130, "hand-n": -140, "hand-f": -140, "toes-n": -5, "toes-f": -5}))
D.edit(A, "poses", lambda o: o["id"] == "pecking-strike", lambda o: setb(o, {"shank-n": -20, "shank-f": -20, "tarsus-n": 40, "tarsus-f": 40}))
D.edit(A, "poses", lambda o: o["id"] == "airborne-wings-down", lambda o: {**o, "root": {**o["root"], "y": 0.27}})
D.edit(A, "poses", lambda o: o["id"] == "airborne-climb", lambda o: {**o, "root": {**o["root"], "y": 0.34}})
D.save()
