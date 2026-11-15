import os, math, sys, copy
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-20-lib.py")).read())

SEED = json.load(open(os.path.join(T, "🗑️generated", "author-20", "stepladder.seed.json")))["volumes"]["50"]
BODY = {d["id"]: d for d in SEED["body"]}
REGION = {t["of"]: t["region"] for t in SEED["thermal"]}
FF, RF = "front-frame", "rear-frame"
sc = []
MAT = BODY["front-rail-l"]["material"]


def derive(id, **ch):
    d = copy.deepcopy(BODY[id])
    for k, v in ch.items():
        if v is None:
            d.pop(k, None)
        else:
            d[k] = v
    return d


def rail(id, exponent, rx=None, ry=None):
    d = copy.deepcopy(BODY[id])
    for s in d["sections"]:
        s["exponent"] = exponent
        if rx:
            s["rx"] = rx
        if ry:
            s["ry"] = ry
    return d


def fit(s, ids, den):
    for i in ids:
        s.b(i)
    for d in s.body:
        r = REGION.get(d["id"])
        if r:
            s.thof("th-" + d["id"], d["id"], r)


def structure(s, mem, joints, masses_k, pad, hinge_real=None):
    for i in mem:
        s.stof("member-" + i, i, FF if i.startswith(("front", "tread", "platform", "spreader-front", "top")) else RF, "member")
    for i in joints:
        s.stof("joint-" + i, i, RF if i in ("hinge",) else FF, "joint")
    if hinge_real:
        s.st(capsule("joint-hinge", RF, MAT, 0.5, [0, 0, -0.15], [0, 0, 0.15], hinge_real, role="body"), "rear-frame", "joint")
    s.st(ellipsoid("mass-front", FF, MAT, 0.5, [masses_k] * 3, [-0.2, 0.75, 0.15]), "front-frame", "mass")
    s.st(ellipsoid("mass-rear", RF, MAT, 0.5, [masses_k] * 3, [0.18, -0.6, 0.15]), "rear-frame", "mass")
    for nm, bone, x, y, z in (("foot-front-left", FF, -0.365, 0, 0.2), ("foot-front-right", FF, -0.365, 0, -0.2), ("foot-rear-left", RF, 0.36, -1.2, 0.175), ("foot-rear-right", RF, 0.36, -1.2, -0.175)):
        s.st(box(f"support-{nm}", bone, BODY["foot-front-l"]["material"], 0.1, [pad, pad if pad > 0.05 else pad / 2, pad], [x, y + (pad if pad > 0.05 else pad / 2) / 2, z]), bone, "support")



# ---- 1:50: the reviewed seed body (23 solids), thermal as references, structure added
s = Scale(50)
fit(s, [copy.deepcopy(d) for d in SEED["body"]], 50)
structure(s, ["front-rail-l", "front-rail-r", "rear-rail-l", "rear-rail-r", "tread-0", "tread-1", "tread-2", "tread-3", "platform", "spreader-front-l", "spreader-front-r", "spreader-rear-l", "spreader-rear-r", "rear-brace-low", "rear-brace-high"], ["hinge", "spreader-knuckle-l", "spreader-knuckle-r"], 0.03, 0.06)
s.dec("DRAFT, not reviewed, not spliced: derived from the 1:50 seed body of the previous drafter (rails, treads, platform, spreaders, feet) with thermal references and a first structure; silhouette, thermal areas and structure containment still fail the check (see author-20.md).", "ENTWURF, nicht geprüft, nicht eingefügt: aus dem 1:50-Startkörper des früheren Entwurfs abgeleitet (Holme, Stufen, Plattform, Spreizen, Füße) mit thermischen Verweisen und einem ersten Tragwerk; Silhouette, thermische Flächen und Lage des Tragwerks im Körper bestehen die Prüfung noch nicht (siehe author-20.md).")
sc.append(s)

# ---- 1:100
s = Scale(100)
ids = [rail(n, 12) for n in ("front-rail-l", "front-rail-r", "rear-rail-l", "rear-rail-r")]
ids += [derive(n, cornerRadius=None) for n in ("foot-front-l", "foot-front-r", "foot-rear-l", "foot-rear-r", "tread-0", "tread-1", "tread-2", "tread-3", "platform")]
ids += [derive(n) for n in ("spreader-front-l", "spreader-front-r", "spreader-rear-l", "spreader-rear-r", "hinge", "rear-brace-low", "rear-brace-high")]
fit(s, ids, 100)
structure(s, ["front-rail-l", "front-rail-r", "rear-rail-l", "rear-rail-r", "tread-0", "tread-1", "tread-2", "tread-3", "platform", "spreader-front-l", "spreader-front-r", "spreader-rear-l", "spreader-rear-r", "rear-brace-low", "rear-brace-high"], ["hinge"], 0.0155, 0.032)
s.dec("DRAFT, not reviewed, not spliced: derived from the 1:50 seed body of the previous drafter (rails, treads, platform, spreaders, feet) with thermal references and a first structure; silhouette, thermal areas and structure containment still fail the check (see author-20.md).", "ENTWURF, nicht geprüft, nicht eingefügt: aus dem 1:50-Startkörper des früheren Entwurfs abgeleitet (Holme, Stufen, Plattform, Spreizen, Füße) mit thermischen Verweisen und einem ersten Tragwerk; Silhouette, thermische Flächen und Lage des Tragwerks im Körper bestehen die Prüfung noch nicht (siehe author-20.md).")
sc.append(s)

# ---- 1:200
s = Scale(200)
ids = [rail(n, 16) for n in ("front-rail-l", "front-rail-r", "rear-rail-l", "rear-rail-r")]
ids += [derive(n, cornerRadius=None) for n in ("foot-front-l", "foot-front-r", "foot-rear-l", "foot-rear-r", "tread-0", "tread-1", "tread-2", "tread-3", "platform")]
ids += [derive(n, p0=[BODY[n]["p0"][0], BODY[n]["p0"][1], BODY[n]["p0"][2]]) for n in ("spreader-front-l", "spreader-front-r")]
for d in ids:
    if d["kind"] == "capsule":
        d["r0"] = d["r1"] = 0.0155
fit(s, ids, 200)
structure(s, ["front-rail-l", "front-rail-r", "rear-rail-l", "rear-rail-r", "tread-0", "tread-1", "tread-2", "tread-3", "platform", "spreader-front-l", "spreader-front-r"], [], 0.031, 0.062, hinge_real=0.0155)
s.dec("DRAFT, not reviewed, not spliced: derived from the 1:50 seed body of the previous drafter (rails, treads, platform, spreaders, feet) with thermal references and a first structure; silhouette, thermal areas and structure containment still fail the check (see author-20.md).", "ENTWURF, nicht geprüft, nicht eingefügt: aus dem 1:50-Startkörper des früheren Entwurfs abgeleitet (Holme, Stufen, Plattform, Spreizen, Füße) mit thermischen Verweisen und einem ersten Tragwerk; Silhouette, thermische Flächen und Lage des Tragwerks im Körper bestehen die Prüfung noch nicht (siehe author-20.md).")
sc.append(s)

# ---- 1:500
s = Scale(500)
ids = [rail("front-rail-l", 16, 0.04, 0.069), rail("front-rail-r", 16, 0.04, 0.069), rail("rear-rail-l", 16, 0.04, 0.073), rail("rear-rail-r", 16, 0.04, 0.073)]
ids += [derive(n, cornerRadius=None, size=[BODY[n]["size"][0], 0.08 if n.startswith("tread") else (BODY[n]["size"][1]), BODY[n]["size"][2]]) for n in ("tread-0", "tread-1", "tread-2", "tread-3")]
ids += [derive("platform", cornerRadius=None, size=[0.27, 0.1, 0.33]), derive("foot-front-l", cornerRadius=None, size=[0.09, 0.09, 0.075]), derive("foot-front-r", cornerRadius=None, size=[0.09, 0.09, 0.075]), derive("foot-rear-l", cornerRadius=None, size=[0.09, 0.09, 0.075]), derive("foot-rear-r", cornerRadius=None, size=[0.09, 0.09, 0.075])]
fit(s, ids, 500)
structure(s, ["front-rail-l", "front-rail-r", "rear-rail-l", "rear-rail-r", "tread-0", "tread-1", "tread-2", "tread-3", "platform"], [], 0.0755, 0.076, hinge_real=0.0755)
s.dec("DRAFT, not reviewed, not spliced: derived from the 1:50 seed body of the previous drafter (rails, treads, platform, spreaders, feet) with thermal references and a first structure; silhouette, thermal areas and structure containment still fail the check (see author-20.md).", "ENTWURF, nicht geprüft, nicht eingefügt: aus dem 1:50-Startkörper des früheren Entwurfs abgeleitet (Holme, Stufen, Plattform, Spreizen, Füße) mit thermischen Verweisen und einem ersten Tragwerk; Silhouette, thermische Flächen und Lage des Tragwerks im Körper bestehen die Prüfung noch nicht (siehe author-20.md).")
sc.append(s)
save("objects", "stepladder", sc)
