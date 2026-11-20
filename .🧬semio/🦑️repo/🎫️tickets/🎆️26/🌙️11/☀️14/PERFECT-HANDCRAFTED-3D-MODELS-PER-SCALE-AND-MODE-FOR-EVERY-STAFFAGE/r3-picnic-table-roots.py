"""🌱 Round 3, furniture/picnic-table: the root corrections outside `volumes` and the 2D: rig (the A-frames as a vertical bone from the ground to the underside of the top, the ground reference without length, top and benches at their real joints, the bench joints on the pivot bolts, masses from the model), anchors (feet under the real legs, sitting points on the real planks), dimensions, thermal layers and areas, the texts that described benches swinging through the legs, the true-extent symbol, references.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location("r3_picnic_table_" + name, os.path.join(HERE, f"r3-picnic-table-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit")

AREAS = {"table-top": 3.07, "bench-seats": 2.26, "a-frame": 2.32, "hinges": 0.39}


def T(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


def rig(old):
    """🦴 The corrected rig."""
    m = K.masses()
    px, py = K.PIVOT
    bones = []
    for bone in old["bones"]:
        bone = dict(bone)
        bid = bone["id"]
        kg, cx, cy, _ = m.get(bid, (0, 0, 0, 0))
        if bid == "ground":
            bone["lengthM"] = 0
        elif bid == "frame":
            bone.update(x=0, y=0, angleDeg=90, lengthM=K.TOP_Y0)
            bone["mass"] = dict(bone["mass"], kg=kg, comX=round(cy, 3), comY=0.0)
        elif bid == "top":
            bone.update(x=K.TOP_Y0, y=K.TOP_HALF, angleDeg=-90, lengthM=2 * K.TOP_HALF)
            bone["mass"] = dict(bone["mass"], kg=kg, comX=K.TOP_HALF, comY=round(cy - K.TOP_Y0, 3))
        elif bid == "bench-r":
            bone.update(x=py, y=-px, angleDeg=-90, lengthM=K.BENCH_X[1])
            bone["mass"] = dict(bone["mass"], kg=kg, comX=round(cx - px, 3), comY=round(cy - py, 3))
        elif bid == "bench-l":
            bone.update(x=py, y=px, angleDeg=90, lengthM=K.BENCH_X[1])
            bone["mass"] = dict(bone["mass"], kg=kg, comX=round(-cx - px, 3), comY=round(-(cy - py), 3))
        bones.append(bone)
    return dict(old, bones=bones)


def anchors(old):
    """⚓ Feet under the real legs in the vertical frame bone, sitting points on the plank tops at the middle of the seat."""
    out = []
    seat_x = round((K.BENCH_X[0] + K.BENCH_X[1]) / 2, 3)
    for a in old:
        a = dict(a)
        if a["id"].startswith("foot-"):
            sx = -1 if a["id"].startswith("foot-left") else 1
            a.update(x=0, y=-sx * K.FOOT_C, footprintLengthM=round(K.FOOT[1] - K.FOOT[0], 3), footprintWidthM=round(K.LEG_Z[1] - K.LEG_Z[0], 3))
        elif a["id"] == "seat-r":
            a.update(x=seat_x, y=K.BENCH_Y[1])
        elif a["id"] == "seat-l":
            a.update(x=seat_x, y=-K.BENCH_Y[1])
        out.append(a)
    return out


def thermal(th):
    """🌡 Layers at the modelled thicknesses and the outer areas of the modelled parts as `volumes-check.ts --product` measures them on the 1:50 model, each within a factor 1.5 of the old value (C16) except the A-frame timber, which goes to area-review."""
    regions = []
    for r in th["regions"]:
        r = dict(r)
        r["areaM2"] = AREAS[r["id"]]
        if r["id"] == "bench-seats":
            r["characteristicLengthM"] = 0.3
        elif r["id"] == "a-frame":
            r["characteristicLengthM"] = 0.095
            r["layers"] = [dict(r["layers"][0], title=T("Pine member 45 x 95 mm", "Kiefernholz 45 x 95 mm"))]
        elif r["id"] == "hinges":
            r["title"] = T("Steel bench brackets and bearing plates", "Stahlkonsolen der Bänke und Lagerplatten")
            r["layers"] = [dict(r["layers"][0], title=T("Steel plate 8 mm", "Stahlblech 8 mm"), thicknessM=0.008, resistanceM2KW=0.00016, heatCapacityJM2K=28080)]
            r["heatCapacityJM2K"] = 28080
        regions.append(r)
    a = list(th["assumptions"])
    a[2] = T("Gusset brackets, bearing plates and bolts are hot-dip galvanised steel (8 mm plate: 7800 x 450 x 0.008 = 28 kJ/m2K); the stowed bench hangs plumb between the A-frames and keeps its region binding with lower sun.",
             "Knotenblechkonsolen, Lagerplatten und Schrauben sind feuerverzinkter Stahl (8-mm-Blech: 7800 x 450 x 0,008 = 28 kJ/m2K); die eingeklappte Bank hängt lotrecht zwischen den A-Böcken und behält ihre Regionsbindung mit weniger Sonne.")
    return dict(th, regions=regions, assumptions=a)


REFERENCES = [
    {"id": "usda-fs-picnic-table", "citation": "USDA Forest Service / US National Park Service standard A-frame picnic table drawings (6 ft table of 2 x 6 and 2 x 4 lumber: top 28-30 in wide and 29-30 in high, seats 17-18 in high, legs raked about 60 degrees, lapped cleats and seat supports clamped by 3/8 in carriage bolts, diagonal braces to the centre cleat)."},
    {"id": "folding-seat-brackets", "citation": "Manufacturers' data of heavy-duty locking fold-down seat brackets and spring index plungers (galvanised 6-8 mm steel plate, M12 pivot, spring plunger locking in the up and down positions, rated above 150 kg per pair)."},
]


def provenance(p):
    """📚 References and notes."""
    have = {r["id"] for r in p["references"]}
    refs = list(p["references"]) + [r for r in REFERENCES if r["id"] not in have]
    notes = T("End elevation 1.52 x 0.75 m, length 1.80 m. Construction after the classic bolted A-frame picnic table of 45 mm pressure-treated pine: five 132 x 40 mm top planks with 10 mm gaps on two end cleats and a centre cleat, two A-frames 0.10 m from the plank ends with 95 x 45 mm legs raked 60 degrees and lapped 95 x 45 mm cleats and stretchers clamped by M10 carriage bolts, two 70 x 45 mm braces to the centre cleat. The two benches (two 145 x 40 mm planks each, 1.50 m long, seat at 0.44 m) hang between the A-frames on folded 8 mm steel gusset brackets that turn on M12 pivot bolts through leg and stretcher; a spring plunger on each gusset locks into a bearing plate on the leg in the seat position and in the stowed position, where the bench hangs plumb between the frames. Masses from the modelled volumes (pine 500, steel 7800 kg/m3) plus hardware. v3: rig, 2D and 3D rebuilt on this construction; the benches no longer swing through the legs.",
              "Stirnansicht 1,52 x 0,75 m, Länge 1,80 m. Konstruktion nach dem klassischen verschraubten A-Bock-Picknicktisch aus 45 mm kesseldruckimprägnierter Kiefer: fünf Tischbohlen 132 x 40 mm mit 10-mm-Fugen auf zwei Endriegeln und einem Mittelriegel, zwei A-Böcke 0,10 m von den Bohlenenden mit um 60 Grad geneigten Beinen 95 x 45 mm und aufgeblatteten Riegeln und Zangen 95 x 45 mm, mit M10-Schlossschrauben verspannt, zwei Streben 70 x 45 mm zum Mittelriegel. Die zwei Bänke (je zwei Bohlen 145 x 40 mm, 1,50 m lang, Sitz auf 0,44 m) hängen zwischen den A-Böcken an gekanteten 8-mm-Knotenblechkonsolen aus Stahl, die um M12-Drehbolzen durch Bein und Zange schwenken; ein Federrastbolzen an jedem Knotenblech rastet in eine Lagerplatte am Bein, in der Sitzstellung und in der Parkstellung, in der die Bank lotrecht zwischen den Böcken hängt. Massen aus den modellierten Volumen (Kiefer 500, Stahl 7800 kg/m3) plus Beschläge. v3: Rig, 2D und 3D auf dieser Konstruktion neu aufgebaut; die Bänke schwenken nicht mehr durch die Beine.")
    return dict(p, references=refs, notes=notes)


def structure(st):
    """🏗 The structural assumptions."""
    a = list(st["assumptions"])
    a[0] = T("End elevation: the two A-frames (legs, cleats, stretchers, braces) are one rigid body on four foot contacts (frames at z = +-0.8 m, feet at x = +-0.665 m); the top and the two benches are bodies on it, each bench on the pivot bolts of its two gusset brackets (0 to 95 degrees down) and locked by spring plungers.",
             "Stirnansicht: die zwei A-Böcke (Beine, Riegel, Zangen, Streben) sind ein starrer Körper auf vier Fußkontakten (Böcke bei z = +-0,8 m, Füße bei x = +-0,665 m); Platte und beide Bänke sind Körper darauf, jede Bank an den Drehbolzen ihrer zwei Knotenblechkonsolen (0 bis 95 Grad abwärts) und durch Federrastbolzen verriegelt.")
    return dict(st, assumptions=a)


def symbol(sym):
    """🔣 The true-extent symbol on the new construction."""
    sym = dict(sym)
    px, py = K.PIVOT
    y0, y1 = py + K.BENCH_Y[0], py + K.BENCH_Y[1]
    x0, x1 = px + K.BENCH_X[0], px + K.BENCH_X[1]
    s0, s1 = K.STRETCH_Y

    def sh(sid, role, z, filled, tone, **g):
        return dict({"id": sid, "bone": "ground", "role": role, "z": z, "filled": filled, "tone": tone}, **g)

    def rc(x, y, w, h):
        return {"kind": "rect", "x": round(x, 4), "y": round(y, 4), "w": round(w, 4), "h": round(h, 4)}

    legr = [[round(x, 4), round(y, 4)] for x, y in K.leg(False)]
    legl = [[-x, y] for x, y in reversed(legr)]
    bz = K.BEAM_Z
    lz = K.LEG_Z
    sym["description"] = T("The picnic table with deployed benches in all three views: top 0.70 x 1.80 m at 0.75 m, two benches 0.30 x 1.50 m at 0.44 m between two A-frames with cleat and stretcher.",
                           "Der Picknicktisch mit ausgeklappten Bänken in allen drei Ansichten: Platte 0,70 x 1,80 m auf 0,75 m, zwei Bänke 0,30 x 1,50 m auf 0,44 m zwischen zwei A-Böcken mit Riegel und Zange.")
    sym["geometry"] = {
        "elevation": [
            sh("bench-l", "body", 1.6, True, 0.85, **rc(-x1, y0, x1 - x0, y1 - y0)),
            sh("bench-r", "body", 1.6, True, 0.85, **rc(x0, y0, x1 - x0, y1 - y0)),
            sh("leg-l", "body", 2, True, 0.85, kind="poly", points=legl, closed=True),
            sh("leg-r", "body", 2, True, 0.85, kind="poly", points=legr, closed=True),
            sh("beam", "body", 2.2, True, 0.85, **rc(-K.STRETCH_HALF, s0, 2 * K.STRETCH_HALF, s1 - s0)),
            sh("top-beam", "body", 2.5, True, 0.85, **rc(-K.CLEAT_HALF, K.CLEAT_Y0, 2 * K.CLEAT_HALF, K.TOP_Y0 - K.CLEAT_Y0)),
            sh("top", "body", 3, True, 0.85, **rc(-K.TOP_HALF, K.TOP_Y0, 2 * K.TOP_HALF, K.TOP_Y1 - K.TOP_Y0)),
            sh("hinge-l", "detail", 1.5, False, 0.5, **rc(-x1 + 0.02, py - 0.045, x1 - px + 0.01, 0.07)),
            sh("hinge-r", "detail", 1.5, False, 0.5, **rc(px - 0.03, py - 0.045, x1 - px + 0.01, 0.07)),
        ],
        "front": [
            sh("top", "body", 3, True, 0.85, **rc(-K.LENGTH / 2, K.TOP_Y0, K.LENGTH, K.TOP_Y1 - K.TOP_Y0)),
            sh("bench", "body", 2.6, True, 0.85, **rc(-K.BENCH_HALF, y0, 2 * K.BENCH_HALF, y1 - y0)),
            sh("trestle-l", "body", 2, True, 0.85, **rc(lz[0], 0, bz[1] - lz[0], K.TOP_Y0)),
            sh("trestle-r", "body", 2, True, 0.85, **rc(-bz[1], 0, bz[1] - lz[0], K.TOP_Y0)),
            sh("beam-l", "detail", 2.2, False, 0.5, **rc(bz[0], s0, bz[1] - bz[0], s1 - s0)),
            sh("beam-r", "detail", 2.2, False, 0.5, **rc(-bz[1], s0, bz[1] - bz[0], s1 - s0)),
            sh("hinge-l", "detail", 2.7, False, 0.5, **rc(K.GUSSET_Z[0], py - 0.045, lz[0] - K.GUSSET_Z[0], 0.07)),
            sh("hinge-r", "detail", 2.7, False, 0.5, **rc(-lz[0], py - 0.045, lz[0] - K.GUSSET_Z[0], 0.07)),
        ],
        "plan": [
            sh("leg-l-0", "body", 0.8, True, 0.85, **rc(-K.FOOT[1], lz[0], K.FOOT[1] - K.LEG_TOP[0], lz[1] - lz[0])),
            sh("leg-r-0", "body", 0.8, True, 0.85, **rc(K.LEG_TOP[0], lz[0], K.FOOT[1] - K.LEG_TOP[0], lz[1] - lz[0])),
            sh("leg-l-1", "body", 0.8, True, 0.85, **rc(-K.FOOT[1], -lz[1], K.FOOT[1] - K.LEG_TOP[0], lz[1] - lz[0])),
            sh("leg-r-1", "body", 0.8, True, 0.85, **rc(K.LEG_TOP[0], -lz[1], K.FOOT[1] - K.LEG_TOP[0], lz[1] - lz[0])),
            sh("plan-bench-r", "body", 1, True, 0.85, **rc(x0, -K.BENCH_HALF, x1 - x0, 2 * K.BENCH_HALF)),
            sh("plan-bench-l", "body", 1, True, 0.85, **rc(-x1, -K.BENCH_HALF, x1 - x0, 2 * K.BENCH_HALF)),
            sh("plan-top", "body", 2, True, 0.85, **rc(-K.TOP_HALF, -K.LENGTH / 2, 2 * K.TOP_HALF, K.LENGTH)),
        ],
    }
    return sym


def texts(asset):
    """✍ The texts that described the old mechanism."""
    asset["summary"] = T("A 1.80 m softwood picnic table with bolted A-frame ends, braces and two benches for four persons that hang between the frames on steel gusset brackets: locked level as seats or swung down plumb between the frames; end elevation, long side and plan.",
                         "Ein 1,80 m langer Picknicktisch aus Weichholz mit verschraubten A-Böcken, Streben und zwei Bänken für vier Personen, die zwischen den Böcken an Stahl-Knotenblechkonsolen hängen: waagerecht verriegelt als Sitz oder lotrecht zwischen die Böcke abgeschwenkt; Stirnansicht, Längsansicht und Grundriss.")
    for pose in asset["poses"]:
        if pose["id"] == "stowed":
            pose["description"] = T("Both benches swung down on their pivot bolts; they hang plumb between the A-frames, the seat planks vertical, 65 mm clear of the ground.",
                                    "Beide Bänke um ihre Drehbolzen abgeschwenkt; sie hängen lotrecht zwischen den A-Böcken, die Sitzbohlen senkrecht, 65 mm über dem Boden.")
        elif pose["id"] == "bench-proof-load":
            pose["description"] = T("Static test load 1600 N at the centre of the right bench seat.", "Statische Prüflast 1600 N in der Mitte der rechten Sitzfläche.")
    for clip in asset["clips"]:
        if clip["id"] == "fold-benches":
            clip["description"] = T("The plungers are pulled and both benches swing down on their pivot bolts until they hang plumb between the A-frames, in 3 s.",
                                    "Die Rastbolzen werden gezogen und beide Bänke schwenken um ihre Drehbolzen ab, bis sie lotrecht zwischen den A-Böcken hängen, in 3 s.")
        elif clip["id"] == "deploy-benches":
            clip["description"] = T("Both benches are swung up into the seat position until the plungers snap in.", "Beide Bänke werden in die Sitzstellung hochgeschwenkt, bis die Rastbolzen einrasten.")
            for ev in clip["events"]:
                if ev["kind"] == "motion":
                    ev["title"] = T("Benches locked on their brackets", "Bänke auf ihren Konsolen verriegelt")


def apply(asset):
    """🖊 Applies every root correction to the asset in memory."""
    asset["rig"] = rig(asset["rig"])
    asset["anchors"] = anchors(asset["anchors"])
    asset["dimensions"] = dict(asset["dimensions"], massKg=round(sum(v[0] for v in K.masses().values()), 1))
    asset["thermal"] = thermal(asset["thermal"])
    asset["provenance"] = provenance(asset["provenance"])
    asset["structure"] = structure(asset["structure"])
    asset["symbols"] = [symbol(s) if s["id"] == "picnic-table-symbol" else s for s in asset["symbols"]]
    asset["rig"]["bones"][1]["title"] = T("A-frames, cleats, stretchers and braces", "A-Böcke, Riegel, Zangen und Streben")
    texts(asset)
