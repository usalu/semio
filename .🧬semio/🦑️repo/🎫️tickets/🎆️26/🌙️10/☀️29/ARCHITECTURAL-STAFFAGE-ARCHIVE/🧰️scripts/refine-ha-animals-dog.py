import importlib.util
spec = importlib.util.spec_from_file_location("lib", "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🧰️scripts/refine-ha-animals-lib.py")
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
D = lib.Doc(); A = "labrador-dog"
rumps = D.get(A, "geometry", lambda o: o["id"] == "rump")
rump = rumps[0] if rumps else None
def shape(base, id, bone, z, d):
    return {**base, "id": id, "bone": bone, "z": z, "d": d}
hind = shape(rump, "hindbody", "pelvis", 3.0, "M -0.13 -0.01 C -0.135 0.045 -0.085 0.078 -0.01 0.078 C 0.08 0.08 0.22 0.076 0.34 0.076 L 0.36 -0.17 C 0.28 -0.14 0.2 -0.11 0.12 -0.115 C 0.04 -0.12 -0.07 -0.1 -0.11 -0.055 C -0.125 -0.035 -0.13 -0.02 -0.13 -0.01 Z")
chest = shape(rump, "chest", "thorax", 3.2, "M -0.05 0.078 C 0.05 0.08 0.13 0.084 0.185 0.076 C 0.235 0.068 0.268 0.03 0.272 -0.03 C 0.276 -0.12 0.235 -0.2 0.165 -0.218 C 0.1 -0.232 0.03 -0.205 -0.05 -0.165 Z")
D.edit(A, "geometry", lambda o: o["id"] == "rump", lambda o: hind)
D.edit(A, "geometry", lambda o: o["id"] == "chest", lambda o: chest)
D.edit(A, "geometry", lambda o: o["id"] in ("loin", "belly-tuck"), lambda o: None)
D.edit(A, "geometry", lambda o: o["id"] in ("near-thigh", "far-thigh"), lambda o: {**o, "d": o["d"].replace("C 0.0 0.095 0.07 0.09 0.12 0.06", "C 0.01 0.078 0.07 0.074 0.12 0.056")})
def fur(o):
    if o["id"] == "fur-neck":
        return [{**o, "id": "fur-neck-1", "points": [[0.03, 0.045], [0.08, 0.04], [0.13, 0.042]]}, {**o, "id": "fur-neck-2", "points": [[0.04, -0.01], [0.09, -0.018], [0.15, -0.02]]}]
    return [{**o, "id": "fur-trunk-1", "points": [[0.02, 0.05], [0.08, 0.046], [0.15, 0.05]]}, {**o, "id": "fur-trunk-2", "points": [[0.0, -0.08], [0.06, -0.09], [0.12, -0.085]]}]
D.edit(A, "geometry", lambda o: o["id"] in ("fur-neck", "fur-trunk"), fur)
D.edit(A, "anchors", lambda o: o["id"].startswith("contact-") and o["id"] != "contact-rump", lambda o: {**o, "y": -0.0255 if o["id"] in ("contact-fn", "contact-ff") else -0.02})
D.save()
