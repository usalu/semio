"""🏗️ Assembles the round-3 wheel loader: reads the catalog asset fresh, replaces rig, anchors, dimensions, poses, clip keys, the 2D shapes of every scale, the symbol and the volumes by the hand-stated model, and writes the candidate document to 🗑️generated/r3-wheel-loader/🔣️.json and the volumes source to volumes/machinery/wheel-loader.json. Nothing in the catalog is written here."""
import copy, json, math, os, sys
import r3_wheel_loader_kin as K
import r3_wheel_loader_model as M
from r3_wheel_loader_lib import r4, rect, circle, poly

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/wheel-loader/🔣️.json"
OUT = os.path.join(T, "🗑️generated", "r3-wheel-loader")
AREAS = json.load(open(os.path.join(T, "r3-wheel-loader-areas.json"))) if os.path.exists(os.path.join(T, "r3-wheel-loader-areas.json")) else {}
LAYER = {"engine-bay": "hood", "counterweight": "iron", "exhaust-stack": "pipe", "cab": "sheet", "cab-glazing": "pane", "front-frame": "plate", "lift-arm": "plate", "bucket": "plate", "hydraulics": "tube", "tyres": "rubber"}
INNER = {"cab-windscreen": ["-u"], "cab-glass-side-l": ["-w"], "cab-glass-side-r": ["+w"], "cab-glass-corner-l": ["-w"], "cab-glass-corner-r": ["+w"], "cab-rear-glass": ["+u"]}
C = M.CARRY
TH_C, BE_C = K.POSES["carry"]
wrap = lambda a: (a + 180) % 360 - 180
PAD = {50: 0.10, 100: 0.15, 200: 0.20, 500: 0.25}


def offsets(theta, beta):
    s = K.solve(theta, beta)
    return {
        "lift-arm": {"angleDeg": r4(theta - TH_C)},
        "bucket": {"angleDeg": r4((beta - theta) - (BE_C - TH_C))},
        "lift-cyl": {"angleDeg": r4(wrap(s["liftAng"] - C["liftAng"]))},
        "lift-rod": {"dx": r4(s["lift"] - C["lift"])},
        "tilt-cyl": {"angleDeg": r4(wrap(s["tiltAng"] - C["tiltAng"]))},
        "tilt-rod": {"dx": r4(s["tilt"] - C["tilt"])},
        "tilt-lever": {"angleDeg": r4(wrap((s["lever"] - theta) - (C["lever"] - TH_C)))},
        "tilt-link": {"angleDeg": r4(wrap((s["link"] - s["lever"]) - (C["link"] - C["lever"])))},
    }


ALL = dict(K.POSES, **K.KEYS)
LOAD_CYCLE = ["dig", "crowd", "rolled", "carry", "raising", "top", "dump-high", "lowering", "dig"]
START_LIFT = ["parked-off", "parked-off", "raising", "top", "dump-high", "lowering", "parked-off"]


def text(en, de):
    return {"en": en, "de": de}


def bone(bid, parent, en, de, x, y, ang, length, joint, kg, cx, cy):
    return {"id": bid, "parent": parent, "title": text(en, de), "x": r4(x), "y": r4(y), "angleDeg": r4(ang), "lengthM": r4(length), "joint": joint, "mass": {"kg": kg, "comX": cx, "comY": cy, "distribution": "point", "source": "authored-estimate"}}


def rig():
    rng = lambda key, field: [o[key][field] for o in (offsets(*v) for v in ALL.values())]
    rev = lambda key, pad=2: {"type": "revolute", "minDeg": math.floor(min(rng(key, "angleDeg")) - pad), "maxDeg": math.ceil(max(rng(key, "angleDeg")) + pad)}
    pri = lambda key: {"type": "prismatic", "axisDeg": 0, "minM": r4(min(rng(key, "dx")) - 0.03), "maxM": r4(max(rng(key, "dx")) + 0.03)}
    f = lambda p: (p[0], p[1])
    armF = K.F_ARM
    return {"kind": "mechanical", "root": "rear-frame", "bones": [
        bone("rear-frame", None, "Rear frame with engine, hood, counterweight and cab", "Hinterwagen mit Motor, Haube, Gegengewicht und Kabine", 0, 0, 0, 3.8, {"type": "fixed"}, 9000, -1.9, 1.45),
        bone("front-frame", "rear-frame", "Front frame with loader tower (the articulation joint is a vertical axis, not an elevation joint)", "Vorderwagen mit Hubgerüst (das Knickgelenk hat eine senkrechte Achse und ist kein Gelenk der Ansicht)", 0, 0, 0, 2.3, {"type": "fixed"}, 2740, 1.2, 1.1),
        bone("lift-arm", "front-frame", "Lift arm (boom)", "Hubarm", K.A[0], K.A[1], TH_C, K.L, rev("lift-arm", 1), 1500, 1.4, 0.2),
        bone("bucket", "lift-arm", "Bucket 3.5 m3", "Schaufel 3,5 m3", K.L, 0, BE_C - TH_C, 1.47, rev("bucket"), 1700, 0.6, 0.1),
        bone("lift-cyl", "front-frame", "Lift cylinder barrel", "Hubzylinder, Rohr", K.E[0], K.E[1], C["liftAng"], M.LIFT_BARREL, rev("lift-cyl"), 230, 0.65, 0),
        bone("lift-rod", "lift-cyl", "Lift cylinder rod", "Hubzylinder, Kolbenstange", M.LIFT_BARREL, 0, 0, C["lift"] - M.LIFT_BARREL, pri("lift-rod"), 140, -0.3, 0),
        bone("tilt-cyl", "front-frame", "Tilt cylinder barrel", "Kippzylinder, Rohr", K.C0[0], K.C0[1], C["tiltAng"], M.TILT_BARREL, rev("tilt-cyl"), 150, 0.8, 0),
        bone("tilt-rod", "tilt-cyl", "Tilt cylinder rod", "Kippzylinder, Kolbenstange", M.TILT_BARREL, 0, 0, C["tilt"] - M.TILT_BARREL, pri("tilt-rod"), 80, 0.3, 0),
        bone("tilt-lever", "lift-arm", "Tilt lever (bell crank of the Z-bar linkage)", "Kipphebel (Umlenkhebel der Z-Kinematik)", armF[0], armF[1], wrap(C["lever"] - TH_C), K.R2, rev("tilt-lever"), 260, 0.05, -0.05),
        bone("tilt-link", "tilt-lever", "Tilt link", "Kippstange", K.R2, 0, wrap(C["link"] - C["lever"]), K.LINK, rev("tilt-link"), 100, 0.45, 0),
        bone("wheel-r", "rear-frame", "Rear wheels", "Hinterräder", -K.AX, K.TYRE_R, 0, K.TYRE_R, {"type": "revolute"}, 2300, 0, 0),
        bone("wheel-f", "front-frame", "Front wheels", "Vorderräder", K.AX, K.TYRE_R, 0, K.TYRE_R, {"type": "revolute"}, 2300, 0, 0),
    ]}


def lists(sc):
    B, sh = M.build(sc)
    f = M.FEATURE[sc]
    ids = {s["id"] for s in B}
    th = [{"id": "th-" + s["id"], "of": s["id"], "region": s["_r"], "layer": LAYER[s["_r"]]} for s in B if s["_r"]]
    for t in th:
        if t["of"] in INNER:
            t["sectionFaces"] = INNER[t["of"]]
    pure = lambda s: {k: v for k, v in s.items() if not k.startswith("_")}
    em = M.RF(material=M.EX, role="detail", tone=0.1)
    e1 = pure(M.cyl("outlet-exhaust-stack-jet", em, "v", (-2.20, 3.30 - f, -0.40), f, 0.057))
    e1.update(region="exhaust-stack", tags=["exhaust-stack-jet"], sectionFaces=["+u", "-u", "+v", "-v", "+w", "-w"])
    e2 = pure(M.box("outlet-radiator-air", em(material=M.PS), (-3.72 - f, 1.60, -0.40), (-3.72, 2.25, 0.40)))
    e2.update(region="engine-bay", tags=["radiator-air"], sectionFaces=["+u", "-u", "+v", "-v", "+w", "-w"])
    th += [e1, e2]
    st = []
    ref = lambda sid, of, member, part="member": st.append({"id": sid, "of": of, "member": member, "part": part}) if of in ids else None
    for sid, of, mb in (("member-rear-rails", "rear-frame", "rear-frame"), ("member-hitch", "hitch", "rear-frame"), ("member-rear-body", "rear-body", "rear-frame"), ("member-rear-crossmember", "rear-crossmember", "rear-frame"), ("member-hitch-bulkhead", "hitch-bulkhead", "rear-frame"), ("member-hitch-upper-r", "hitch-upper-r", "rear-frame"), ("member-hitch-lower-r", "hitch-lower-r", "rear-frame"), ("member-axle-r", "axle-r", "rear-frame"),
                        ("member-tower", "front-frame", "front-frame"), ("member-front-body", "front-body", "front-frame"), ("member-belly-plate", "belly-plate", "front-frame"), ("member-front-bulkhead", "front-bulkhead", "front-frame"), ("member-hitch-upper-f", "hitch-upper-f", "front-frame"), ("member-hitch-lower-f", "hitch-lower-f", "front-frame"), ("member-axle-f", "axle-f", "front-frame"), ("member-lift-pivot", "lift-pivot", "front-frame"), ("member-tilt-crossmember", "tilt-crossmember", "front-frame"),
                        ("member-lift-arm", "lift-arm", "lift-arm"), ("member-torque-tube", "torque-tube", "lift-arm"), ("member-bucket-shell", "bucket-shell", "bucket"), ("member-bucket-side", "bucket-side", "bucket"), ("member-bucket-hinge", "bucket-hinge", "bucket"), ("member-bucket-hinge-o", "bucket-hinge-o", "bucket"), ("member-bucket-hinge-i", "bucket-hinge-i", "bucket"), ("member-tilt-lug", "tilt-lug", "bucket"), ("member-bucket-beam", "bucket-beam", "bucket"),
                        ("member-tilt-lever", "tilt-lever", "tilt-lever"), ("member-tilt-link", "tilt-link", "tilt-link"), ("member-lift-cyl-l", "lift-cyl-l", "lift-cyl"), ("member-lift-cyl-r", "lift-cyl-r", "lift-cyl"), ("member-lift-rod-l", "lift-rod-l", "lift-rod"), ("member-lift-rod-r", "lift-rod-r", "lift-rod"), ("member-tilt-cyl", "tilt-cyl", "tilt-cyl"), ("member-tilt-rod", "tilt-rod", "tilt-rod")):
        ref(sid, of, mb)
    if sc == 500:
        st[:] = [x for x in st if x["id"] != "member-rear-body"]
        beam = pure(M.ext("member-rear-frame", M.RF, M.RAIL, -0.46, 0.46))
        beam.update(member="rear-frame", part="member")
        st.insert(0, beam)
    for k in "rf":
        for side, sg in (("l", 1), ("r", -1)):
            if "rim-%s-%s" % (k, side) in ids:
                ref("member-rim-%s-%s" % (k, side), "rim-%s-%s" % (k, side), "wheel-" + k)
            else:
                s = pure(M.cyl("member-wheel-%s-%s" % (k, side), Ctx("wheel-" + k, M.CI, "detail", 0.4), "w", (0, 0, sg * M.TZ - 0.13), 0.26, 0.36))
                s.update(member="wheel-" + k, part="member")
                st.append(s)
    jc = lambda b: Ctx(b, M.PS, "detail", 0.2)

    def joint(sid, b, u, v, zs, r=0.05, ln=0.12):
        for i, z in enumerate(zs):
            s = pure(M.cyl(sid + ("" if len(zs) == 1 else "-lr"[i:i + 1] or str(i)), jc(b), "w", (u, v, z - ln / 2), ln, r))
            s["id"] = sid + ("" if len(zs) == 1 else ("-l", "-r")[i])
            s.update(member=b, part="joint")
            st.append(s)

    lr, tr_ = C["lift"] - M.LIFT_BARREL, C["tilt"] - M.TILT_BARREL
    if sc <= 200:
        joint("joint-lift-pivot", "lift-arm", 0, 0, (0.595, -0.595))
        joint("joint-bucket-pin", "bucket", 0, 0, (0.595, -0.595))
        joint("joint-lever-pivot", "tilt-lever", 0, 0, (0,))
        for nm, y in (("upper", 1.30), ("lower", 0.59)):
            s = pure(M.cyl("joint-hitch-" + nm, jc("front-frame"), "v", (0, y, 0), 0.19, 0.05))
            s.update(member="front-frame", part="joint")
            st.append(s)
    if sc <= 100:
        joint("joint-lift-cyl-base", "lift-cyl", 0, 0, (0.595, -0.595))
        joint("joint-lift-rod-eye", "lift-rod", lr, 0, (0.595, -0.595))
        joint("joint-tilt-cyl-base", "tilt-cyl", 0, 0, (0,))
        joint("joint-tilt-rod-eye", "tilt-rod", tr_, 0, (0,))
        joint("joint-lever-link", "tilt-link", 0, 0, (0,))
        joint("joint-link-bucket", "tilt-link", K.LINK, 0, (0,))
        joint("joint-axle-r", "wheel-r", 0, 0, (M.TZ, -M.TZ))
        joint("joint-axle-f", "wheel-f", 0, 0, (M.TZ, -M.TZ))
    for an, b, x, z in (("contact-r-l", "rear-frame", -K.AX, M.TZ), ("contact-r-r", "rear-frame", -K.AX, -M.TZ), ("contact-f-l", "front-frame", K.AX, M.TZ), ("contact-f-r", "front-frame", K.AX, -M.TZ)):
        s = pure(M.box("support-" + an, Ctx(b, M.RU, "detail", 0.1), (x - PAD[sc], 0, z - 0.3), (x + PAD[sc], f, z + 0.3)))
        s.update(member=b, part="support", anchor=an, tags=[an])
        st.append(s)
    mc = lambda b: Ctx(b, M.PS, "detail", 0.15)
    for sid, b, at, rad, rep in (("mass-rear", "rear-frame", (-1.9, 1.45, 0), (0.75, 0.42, 0.5), None), ("mass-front", "front-frame", (1.2, 1.1, 0), (0.45, 0.3, 0.3), None), ("mass-wheel-r", "wheel-r", (0, 0, M.TZ), (0.3, 0.3, 0.2), 1), ("mass-wheel-f", "wheel-f", (0, 0, M.TZ), (0.3, 0.3, 0.2), 1)):
        s = pure(M.ell(sid, mc(b), at, rad))
        s.update(member=b, part="mass")
        if rep:
            s["repeat"] = {"count": 2, "translation": [0, 0, -2 * M.TZ]}
        st.append(s)
    return [pure(s) for s in B], th, st, sh.out(sc != 500)


from r3_wheel_loader_lib import Ctx
from r3_wheel_loader_texts import VOLUME_DECISIONS, REP_DECISIONS, SUMMARY, NOTES, DIM_NOTE, SYMBOL_DESC, CHASSIS_TITLE, ASSUMPTION

doc = json.load(open(CATALOG))
a = doc["asset"]
a["summary"] = SUMMARY
a["provenance"]["notes"] = NOTES
refs = a["provenance"]["references"]
if not any(r["id"] == "cat-962m-2017" for r in refs):
    refs.insert(1, {"id": "cat-962m-2017", "citation": "Caterpillar (2017). 962M Wheel Loader, specifications (dealer data sheets, Peterson Cat and Cleveland Brothers listings, read 2026-11-19): wheelbase 3 350 mm, tread 2 140 mm, rear axle centre line to edge of counterweight 2 147 mm, height to top of ROPS 3 444 to 3 458 mm, hinge pin height at carry 669 mm and at maximum lift 4 235 mm, overall length without bucket 7 228 mm, tyres 23.5R25 (1 615 mm x 595 mm)."})
tip = C["tip"]
a["dimensions"] = {"heightM": 3.45, "widthM": r4(tip[0] - M.REAR), "depthM": 2.95, "massKg": 20500}
a["rig"] = rig()
for an in a["anchors"]:
    if an["id"].startswith("contact"):
        an["x"] = r4(-K.AX if "-r-" in an["id"] else K.AX)
        an["z"] = M.TZ if an["id"].endswith("-l") else -M.TZ
        an["footprintLengthM"] = 0.5
    elif an["id"] == "stack-mouth":
        an.update(x=-2.2, y=3.3, z=-0.4)
    elif an["id"] == "radiator-outlet":
        an.update(x=-3.72, y=1.92)
    elif an["id"] == "bucket-cog":
        an.update(x=0.75, y=0.15)
for p in a["poses"]:
    p["bones"] = offsets(*K.POSES[p["id"]])
for clip, seq in zip(a["clips"], (LOAD_CYCLE, START_LIFT)):
    assert len(clip["keys"]) == len(seq), (clip["id"], len(clip["keys"]))
    for key, name in zip(clip["keys"], seq):
        key["bones"] = offsets(*ALL[name])
vol = {}
for sc in (50, 100, 200, 500):
    body, th, st, shapes = lists(sc)
    rep = a["representations"][str(sc)]
    if sc == 500:
        for v in shapes.values():
            for s in v:
                if s["role"] == "detail":
                    s["role"] = "body"
    for v in shapes.values():
        for s in v:
            if s["id"] in ("lift-cyl-r", "lift-rod-r") and v is shapes["elevation"]:
                s["id"] = {"lift-cyl-r": "lift-cyl-barrel", "lift-rod-r": "lift-rod"}[s["id"]]
            if s["id"] == "tilt-cyl" and v is shapes["elevation"]:
                s["id"] = "tilt-cyl-barrel"
    rep["geometry"] = shapes
    rep["decisions"] = REP_DECISIONS[sc]
    vol[str(sc)] = {"status": "authored", "denominator": sc, "body": body, "thermal": th, "structure": st, "decisions": VOLUME_DECISIONS[sc]}
a["volumes"] = vol
sym = a["symbols"][0]
sym["description"] = SYMBOL_DESC
sym["dimensionalNote"] = DIM_NOTE
R, X1 = M.REAR, r4(tip[0])
g = lambda sid, role, z, tone, geom, filled=True: dict(geom, id=sid, bone="rear-frame", role=role, z=z, filled=filled, tone=tone)
sym["geometry"] = {
    "elevation": [g("elevation-tyre-r", "body", 1, 0.15, circle(-K.AX, K.TYRE_R, K.TYRE_R)), g("elevation-tyre-f", "body", 2, 0.15, circle(K.AX, K.TYRE_R, K.TYRE_R)), g("elevation-rim-r", "detail", 3, 0.4, circle(-K.AX, K.TYRE_R, 0.365)), g("elevation-rim-f", "detail", 4, 0.4, circle(K.AX, K.TYRE_R, 0.365)),
                  g("elevation-counterweight", "body", 5, 0.6, rect(R, 0.95, -3.30, 1.52)), g("elevation-hood", "body", 6, 0.5, rect(-3.72, 1.02, -1.42, 2.60)), g("elevation-front-frame", "body", 7, 0.5, rect(-1.42, 0.62, 2.30, 1.46)), g("elevation-cab", "body", 8, 0.45, rect(-1.44, 1.46, 0.32, 3.45)), g("elevation-cab-glass", "detail", 9, 0.9, rect(-1.32, 1.97, 0.14, 3.30), False), g("elevation-exhaust-stack", "detail", 10, 0.3, rect(-2.265, 2.5, -2.135, 3.30)),
                  g("elevation-lift-arm", "body", 11, 0.4, poly([(0.9, 2.3), (1.2, 2.45), (2.75, 1.7), (3.4, 0.7), (3.15, 0.55), (2.3, 1.3)])), g("elevation-bucket", "body", 12, 0.35, poly([(3.0, 0.75), (3.6, 0.63), (X1, 1.41), (3.0, 1.75)]))],
    "front": [g("front-counterweight", "body", 1, 0.6, rect(-1.22, 0.95, 1.22, 1.52)), g("front-hood", "body", 2, 0.5, rect(-0.72, 1.42, 0.72, 2.60)), g("front-front-frame", "body", 3, 0.5, rect(-0.44, 0.58, 0.44, 2.50)), g("front-cab", "body", 4, 0.45, rect(-0.80, 1.85, 0.80, 3.45)), g("front-cab-glass", "detail", 5, 0.9, rect(-0.68, 1.97, 0.68, 3.30), False), g("front-exhaust-stack", "detail", 6, 0.3, rect(-0.465, 2.6, -0.335, 3.30)),
              g("front-tyre-l", "body", 7, 0.15, rect(0.77, 0, 1.37, 1.615)), g("front-tyre-r", "body", 8, 0.15, rect(-1.37, 0, -0.77, 1.615)), g("front-lift-arm-l", "detail", 9, 0.4, rect(0.565, 0.55, 0.625, 2.45)), g("front-lift-arm-r", "detail", 10, 0.4, rect(-0.625, 0.55, -0.565, 2.45)), g("front-bucket", "body", 11, 0.35, rect(-1.475, 0.63, 1.475, 1.75))],
    "plan": [g("plan-counterweight", "body", 1, 0.6, rect(R, -1.22, -3.30, 1.22)), g("plan-hood", "body", 2, 0.5, rect(-3.72, -0.72, -1.42, 0.72)), g("plan-front-frame", "body", 3, 0.5, rect(-1.42, -0.46, 2.30, 0.46)), g("plan-cab", "body", 4, 0.45, rect(-1.44, -0.80, 0.32, 0.80)), g("plan-exhaust-stack", "detail", 5, 0.3, rect(-2.265, -0.465, -2.135, -0.335)),
             g("plan-tyre-fl", "body", 6, 0.15, rect(K.AX - K.TYRE_R, 0.77, K.AX + K.TYRE_R, 1.37)), g("plan-tyre-fr", "body", 7, 0.15, rect(K.AX - K.TYRE_R, -1.37, K.AX + K.TYRE_R, -0.77)), g("plan-tyre-rl", "body", 8, 0.15, rect(-K.AX - K.TYRE_R, 0.77, -K.AX + K.TYRE_R, 1.37)), g("plan-tyre-rr", "body", 9, 0.15, rect(-K.AX - K.TYRE_R, -1.37, -K.AX + K.TYRE_R, -0.77)),
             g("plan-lift-arm-l", "detail", 10, 0.4, rect(0.9, 0.565, 3.4, 0.625)), g("plan-lift-arm-r", "detail", 11, 0.4, rect(0.9, -0.625, 3.4, -0.565)), g("plan-bucket", "body", 12, 0.35, rect(3.0, -1.475, X1, 1.475)), g("plan-articulation-joint", "detail", 13, 0.1, circle(0, 0, 0.25))],
}
for r in a["thermal"]["regions"]:
    if r["id"] == "front-frame":
        r["title"] = CHASSIS_TITLE
    if r["id"] == "tyres":
        r["characteristicLengthM"] = 1.615
    if r["id"] in AREAS:
        r["areaM2"] = AREAS[r["id"]]
a["structure"]["assumptions"] = [ASSUMPTION]
os.makedirs(OUT, exist_ok=True)
json.dump(doc, open(os.path.join(OUT, "🔣️.json"), "w"), ensure_ascii=False, indent=1)
json.dump({"volumes": vol}, open(os.path.join(T, "volumes", "machinery", "wheel-loader.json"), "w"), ensure_ascii=False, indent=1)
print("candidate written; widthM", a["dimensions"]["widthM"], "solids", {k: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for k, v in vol.items()})
for name, v in ALL.items():
    s = K.solve(*v)
    print("  %-11s lift %.3f tilt %.3f tip (%.2f %.2f)" % (name, s["lift"], s["tilt"], *s["tip"]))
