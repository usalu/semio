"""🛞 Author pass 1 (second round) on machinery/concrete-mixer-truck: the running gear gets its real kinematics. The four `wheel-N` bones (one rigid bone for axle beam and both wheel ends) become `axle-N` (beam or drive axle, fixed to the springs), `kingpin-N-l|r` on the two steered axles (vertical king pins 1.72 m apart, steering by twist) and `wheel-N-l|r` (one spinning wheel end each, twin wheels as one). King pins, knuckles, steering arms, tie rods, steering box, pitman arm, drag links and relay lever are modelled by hand; every solid, structure entry, thermal reference and 2D shape is bound to the new bones.

Usage: python3 rework-concrete-mixer-truck-rig.py candidate|publish
  candidate: writes 🗑️generated/r3-concrete-mixer-truck/🔣️.json (check with r3-validate.ts --file, r3-sheet.ts --file)
  publish:   re-reads the catalog file, applies the same edit, writes the catalog file and the volumes source
"""
import json, os, re, sys, copy

CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/concrete-mixer-truck/🔣️.json"
SOURCE = "volumes/machinery/concrete-mixer-truck.json"
CANDIDATE = "🗑️generated/r3-concrete-mixer-truck/🔣️.json"
P, C = "paint-steel", "cast-iron"
AXLE_X = {1: 3.025, 2: 1.23, 3: -1.275, 4: -2.675}
STEERED = {1: 42, 2: 30}
KINGPIN_Z, KINGPIN_DROP, KINGPIN_LEN = 0.86, 0.12, 0.24
FRONT_WHEEL_Z, TWIN_Z = 1.02, 0.92
SIDES = (("l", 1), ("r", -1))
SIDE_NAME = {"l": ("left", "links"), "r": ("right", "rechts")}


def bones(old):
    out = []
    for b in old:
        m = re.fullmatch(r"wheel-(\d)", b["id"])
        if not m:
            if b["id"] == "chute-swivel":
                b["joint"] = {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -100, "twistMaxDeg": 100}
            out.append(b)
            continue
        n = int(m.group(1))
        drive = n > 2
        out.append({"id": f"axle-{n}", "parent": "chassis",
                    "title": {"en": f"Drive axle {n} with differential" if drive else f"Steered front axle beam {n}", "de": f"Antriebsachse {n} mit Differential" if drive else f"Gelenkter Vorderachskörper {n}"},
                    "x": AXLE_X[n], "y": 0.52, "angleDeg": 0, "lengthM": 0.3, "joint": {"type": "fixed"},
                    "mass": {"kg": 380, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate"}})
        for s, k in SIDES:
            parent, x, y, z, angle = f"axle-{n}", 0, 0, k * TWIN_Z, 0
            if not drive:
                out.append({"id": f"kingpin-{n}-{s}", "parent": f"axle-{n}",
                            "title": {"en": f"King pin of axle {n} ({SIDE_NAME[s][0]})", "de": f"Achsschenkelbolzen der Achse {n} ({SIDE_NAME[s][1]})"},
                            "x": 0, "y": -KINGPIN_DROP, "z": k * KINGPIN_Z, "angleDeg": 90, "lengthM": KINGPIN_LEN,
                            "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -STEERED[n], "twistMaxDeg": STEERED[n]}})
                parent, x, y, z, angle = f"kingpin-{n}-{s}", KINGPIN_DROP, 0, k * round(FRONT_WHEEL_Z - KINGPIN_Z, 4), -90
            out.append({"id": f"wheel-{n}-{s}", "parent": parent,
                        "title": {"en": f"{'Twin wheels' if drive else 'Wheel'} of axle {n} ({SIDE_NAME[s][0]})", "de": f"{'Zwillingsräder' if drive else 'Rad'} der Achse {n} ({SIDE_NAME[s][1]})"},
                        "x": x, "y": y, "z": z, "angleDeg": angle, "lengthM": 0.52, "joint": {"type": "revolute"},
                        "mass": {"kg": 110, "comX": 0, "comY": 0, "distribution": "point", "source": "authored-estimate"}})
    return out


def side_of(sid):
    m = re.search(r"-(l|r)(-|$)", sid)
    return m.group(1) if m else None


def shift(solid, d):
    kind = solid["kind"]
    if kind == "revolve":
        solid["origin"][2] = round(solid["origin"][2] + d, 4)
    elif kind in ("box", "ellipsoid"):
        solid["transform"]["translation"][2] = round(solid["transform"]["translation"][2] + d, 4)
    elif kind == "extrude":
        solid["from"], solid["to"] = round(solid["from"] + d, 4), round(solid["to"] + d, 4)
    elif kind == "polyhedron":
        solid["vertices"] = [[x, y, round(z + d, 5)] for x, y, z in solid["vertices"]]
        if "repeat" in solid:
            solid["repeat"]["origin"][2] = round(solid["repeat"]["origin"][2] + d, 4)
    else:
        raise SystemExit(f"no shift for {kind} {solid['id']}")


def rebind(solid):
    n = int(solid["bone"][-1])
    s = side_of(solid["id"])
    if s is None or solid.get("part") == "support":
        solid["bone"] = f"axle-{n}"
        return f"wheel-{n}-{s}" if s else f"axle-{n}"
    k = 1 if s == "l" else -1
    solid["bone"] = f"wheel-{n}-{s}"
    shift(solid, -k * (FRONT_WHEEL_Z if n <= 2 else TWIN_Z))
    return solid["bone"]


def box(sid, bone, size, at, tone=0.3, material=P, role="detail"):
    return {"id": sid, "kind": "box", "bone": bone, "material": material, "role": role, "tone": tone, "size": size, "transform": {"translation": at}}


def rod(sid, bone, p0, p1, r, tone=0.3, material=P):
    return {"id": sid, "kind": "capsule", "bone": bone, "material": material, "role": "detail", "tone": tone, "p0": p0, "p1": p1, "r0": r, "r1": r}


def steering(scale):
    """🧭 Hand-authored steering parts per scale: (solid, structure part) pairs."""
    out = []
    for n in (1, 2):
        for s, k in SIDES:
            kp = f"kingpin-{n}-{s}"
            if scale in (50, 100):
                out.append(({"id": f"kingpin-{n}-{s}", "kind": "revolve", "bone": kp, "material": C, "role": "detail", "tone": 0.4, "profile": [[0, 0.026], [KINGPIN_LEN, 0.026]], "axis": [1, 0, 0], "origin": [0, 0, 0]}, "joint"))
            if scale == 50:
                out.append((box(f"knuckle-{n}-{s}", kp, [0.2, 0.11, 0.06], [0.12, 0, k * 0.065], material=C), "member"))
                out.append((box(f"knuckle-lug-{n}-{s}-top", kp, [0.045, 0.09, 0.09], [0.1975, 0, k * 0.01], material=C), "member"))
                out.append((box(f"knuckle-lug-{n}-{s}-bottom", kp, [0.045, 0.09, 0.09], [0.0425, 0, k * 0.01], material=C), "member"))
            elif scale in (100, 200):
                out.append((box(f"knuckle-{n}-{s}", kp, [0.2, 0.11, 0.1], [0.12, 0, k * 0.045], material=C), "member"))
            else:
                out.append((box(f"knuckle-{n}-{s}", kp, [0.24, 0.11, 0.1], [0.12, 0, k * 0.045], material=C), "member"))
            if scale != 500:
                out.append((rod(f"steering-arm-{n}-{s}", kp, [0.06, 0.03, k * 0.05], [0.06, 0.24, k * -0.03], 0.02), "member"))
        if scale != 500:
            out.append((rod(f"tie-rod-{n}", f"axle-{n}", [-0.24, -0.06, 0.83], [-0.24, -0.06, -0.83], 0.02, tone=0.4), "member"))
        if scale in (50, 100):
            out.append((rod(f"drag-arm-{n}-l", f"kingpin-{n}-l", [0.2, 0, 0], [0.2, -0.2, -0.2], 0.02), "member"))
    if scale in (50, 100):
        out += [(box("steering-box", "chassis", [0.2, 0.22, 0.13], [3.72, 0.8, 0.44], material=C), "member"),
                (rod("pitman-arm", "chassis", [3.72, 0.72, 0.52], [3.66, 0.6, 0.52], 0.022), "member"),
                (rod("drag-link-1", "chassis", [3.66, 0.6, 0.52], [3.225, 0.6, 0.66], 0.02, tone=0.4), "member"),
                (rod("relay-rod", "chassis", [3.7, 0.7, 0.545], [1.75, 0.72, 0.5], 0.02, tone=0.4), "member"),
                (box("relay-lever", "chassis", [0.06, 0.26, 0.14], [1.75, 0.74, 0.445]), "member"),
                (rod("drag-link-2", "chassis", [1.75, 0.63, 0.5], [1.43, 0.6, 0.66], 0.02, tone=0.4), "member")]
    return out


DECISIONS = {
    50: ("The running gear follows the real kinematics: each axle is a bone of its own that stays with the springs, the two front axles carry king pins 1.72 m apart with forked knuckles, steering arms and a tie rod behind the beam, and each wheel end spins on a bone of its own at the wheel centre; steering box, pitman arm, drag link, relay rod with lever and the second drag link bring the motion to both steered axles. The oil sump sits behind the first axle beam so the beam passes in front of it.",
         "Das Fahrwerk folgt der wirklichen Kinematik: Jede Achse ist ein eigener Knochen, der bei den Federn bleibt, die beiden Vorderachsen tragen Achsschenkelbolzen im Abstand von 1,72 m mit Gabel-Achsschenkeln, Lenkhebeln und einer Spurstange hinter dem Achskörper, und jedes Rad dreht auf einem eigenen Knochen in der Radmitte; Lenkgetriebe, Lenkstockhebel, Lenkschubstange, Zwischenstange mit Umlenkhebel und die zweite Schubstange bringen die Bewegung auf beide gelenkten Achsen. Die Ölwanne sitzt hinter dem ersten Achskörper, sodass dieser vor ihr durchläuft."),
    100: ("Axles, king pins and wheel ends are separate bones as at 1:50; the knuckle is one 100 mm block around the 52 mm king pin, the steering arms, tie rods, drag links and relay rod stay as 40 mm rods because they are above the 15 mm feature.",
          "Achsen, Achsschenkelbolzen und Räder sind wie bei 1:50 eigene Knochen; der Achsschenkel ist ein 100-mm-Block um den 52-mm-Bolzen, Lenkhebel, Spurstangen, Schubstangen und Zwischenstange bleiben 40-mm-Stangen, weil sie über dem kleinsten Merkmal von 15 mm liegen."),
    200: ("Axles, king pins and wheel ends are separate bones; the steering is reduced to the knuckle block, the steering arm and the 40 mm tie rod of each front axle, the steering box and drag links are below what reads at 30 mm and are left out.",
          "Achsen, Achsschenkelbolzen und Räder sind eigene Knochen; die Lenkung ist auf Achsschenkelblock, Lenkhebel und 40-mm-Spurstange je Vorderachse reduziert, Lenkgetriebe und Schubstangen liegen unter dem bei 30 mm Lesbaren und entfallen."),
    500: ("Axles, king pins and wheel ends are separate bones so the wheels still steer and spin; of the steering only the knuckle block between axle end and wheel remains, every rod is below the 75 mm feature.",
          "Achsen, Achsschenkelbolzen und Räder sind eigene Knochen, damit die Räder weiterhin lenken und drehen; von der Lenkung bleibt nur der Achsschenkelblock zwischen Achsende und Rad, jede Stange liegt unter dem kleinsten Merkmal von 75 mm."),
}


def volumes(vol, scale):
    by_id = {}
    for solid in vol["body"]:
        if solid["bone"].startswith("wheel-"):
            rebind(solid)
        by_id[solid["id"]] = solid
    for n in (1, 2):
        beam = by_id.get(f"axle-beam-{n}")
        if beam:
            beam["size"][2] = 1.78
    if "oil-sump" in by_id:
        by_id["oil-sump"]["transform"]["translation"][0] = 3.42
    for entry in vol["structure"]:
        if "of" in entry:
            if str(entry.get("member", "")).startswith("wheel-"):
                entry["member"] = by_id[entry["of"]]["bone"]
        elif str(entry.get("bone", "")).startswith("wheel-"):
            entry["member"] = rebind(entry)
    for solid, part in steering(scale):
        vol["body"].append(solid)
        vol["thermal"].append({"id": "th-" + solid["id"], "of": solid["id"], "region": "running-gear"})
        vol["structure"].append({"id": "st-" + solid["id"], "of": solid["id"], "member": solid["bone"], "part": part})
    en, de = DECISIONS[scale]
    vol["decisions"].append({"en": en, "de": de})


def rect(sid, bone, z, x, y, w, h, tone=0.3):
    return {"id": sid, "bone": bone, "role": "detail", "z": z, "filled": True, "tone": tone, "kind": "rect", "x": x, "y": y, "w": w, "h": h, "region": "running-gear", "material": P}


def drawings(rep, scale):
    g = rep["geometry"]
    far = []
    seen = set()
    for shape in g["elevation"]:
        if shape["bone"].startswith("wheel-"):
            n = shape["bone"][-1]
            if n not in seen:
                seen.add(n)
                twin = copy.deepcopy(shape)
                twin.update(id=shape["id"] + "-far", bone=f"wheel-{n}-l", z=round(shape["z"] - 0.5, 2))
                far.append(twin)
            shape["bone"] = f"wheel-{n}-r"
    g["elevation"] = far + g["elevation"]
    for view in ("front", "plan"):
        for shape in g[view]:
            if shape["bone"].startswith("wheel-"):
                n = shape["bone"][-1]
                shape["bone"] = f"wheel-{n}-l" if "left" in shape["id"] else f"wheel-{n}-r" if "right" in shape["id"] else f"axle-{n}"
    if scale in (50, 100):
        for n in (1, 2):
            for s, k in SIDES:
                name = SIDE_NAME[s][0]
                lo = 0.855 if k > 0 else -0.955
                g["elevation"].insert(0, rect(f"knuckle-{n}-{name}", f"kingpin-{n}-{s}", 3.4, 0.02, -0.055, 0.2, 0.11))
                g["front"].insert(0, rect(f"knuckle-{n}-{name}", f"kingpin-{n}-{s}", 2.4 - 0.05 * n, lo, 0.42, 0.1, 0.2))
                g["plan"].insert(0, rect(f"knuckle-{n}-{name}", f"kingpin-{n}-{s}", 0.5, AXLE_X[n] - 0.055, lo, 0.11, 0.1))
    if scale == 50:
        for n in (1, 2):
            g["front"].insert(0, rect(f"tie-rod-{n}", f"axle-{n}", 2.3 - 0.05 * n, -0.83, 0.44, 1.66, 0.04, tone=0.4))
            g["plan"].insert(0, rect(f"tie-rod-{n}", f"axle-{n}", 0.45, round(AXLE_X[n] - 0.26, 3), -0.83, 0.04, 1.66, tone=0.4))


def apply(doc):
    asset = doc["asset"]
    assert any(b["id"] == "wheel-1" for b in asset["rig"]["bones"]), "already applied"
    asset["rig"]["bones"] = bones(asset["rig"]["bones"])
    for scale in (50, 100, 200, 500):
        volumes(asset["volumes"][str(scale)], scale)
        drawings(asset["representations"][str(scale)], scale)
    return doc


mode = sys.argv[1]
doc = apply(json.load(open(CATALOG)))
if mode == "candidate":
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(doc, open(CANDIDATE, "w"), ensure_ascii=False, indent=1)
elif mode == "publish":
    json.dump(doc, open(CATALOG, "w"), ensure_ascii=False, indent=1)
    json.dump({"volumes": doc["asset"]["volumes"]}, open(SOURCE, "w"), ensure_ascii=False, indent=1)
print(mode, "written;", sum(len(doc["asset"]["volumes"][s]["body"]) for s in doc["asset"]["volumes"]), "body solids")
