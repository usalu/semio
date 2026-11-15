import sys, json, math; sys.path.insert(0, ".")
import importlib; L = importlib.import_module("author-13-lib")
globals().update({k: getattr(L, k) for k in dir(L) if not k.startswith("_")})
SEED = json.load(open("author-13-seed-rack.json"))
RS, MS, CH, BL, CR = "rack-steel", "mesh-steel", "chassis", "blank-steel", "caster-rubber"
SRV_H = 0.0879
SY = [0.32225 + 0.0889 * k for k in range(6)]
def J(x): return json.loads(json.dumps(x))
def sf(d, faces): d["sectionFaces"] = faces; return d

def rect(u0, u1, v0, v1): return [(u0, v0), (u1, v0), (u1, v1), (u0, v1)]
def wall(side, vents, thick=0.03):
    u0 = -0.3 if side < 0 else 0.3 - thick
    holes = [rect(1.62 + 0.05 * k - 0.006, 1.62 + 0.05 * k + 0.006, 0.22, 0.42) for k in range(7)] if vents and side > 0 else None
    return ext(f"side-wall-{'l' if side < 0 else 'r'}", "frame", RS, "body", 0.5, rect(0.07, 2.0, -0.475, 0.5), "yz", u0, u0 + thick, holes=holes)
def door(holes, z0=-0.4985, z1=-0.4735, sid="door-field"):
    return ext(sid, "door", MS, "body", 0.18, rect(-0.27, 0.27, 0.1, 1.967), "xy", z0, z1, holes=holes)
def holes_door():
    h = []
    for k in range(42):
        y = 0.1222 + 0.0444 * k; x0 = -0.255 if k % 2 == 0 else -0.237
        for j in range(14 if k % 2 == 0 else 14): h.append(rect(x0 + 0.036 * j, x0 + 0.036 * j + 0.012, y - 0.002, y + 0.002))
    return h
def caster(id, x, z, r=0.015, h=0.07):
    return rev(id, "feet", CR, "body", 0.2, [(0, 0.011), (0.004, r), (h - 0.012, r), (h - 0.004, 0.011), (h, 0)], axis=(0, 1, 0), at=(x, 0, z))
def casters(r=0.015, h=0.07):
    return [caster(f"caster-{a}{b}", sx * 0.25, sz * 0.4, r, h) for a, sx in (("l", -1), ("r", 1)) for b, sz in (("f", -1), ("r", 1))]
def yoke(a, b, sx, sz, w=0.045):
    return box(f"caster-yoke-{a}{b}", "feet", RS, "detail", 0.4, (w, 0.012, 0.05), (sx * 0.25, 0.0, sz * 0.4 - 0.0)) if False else None

def equip50():
    b = []
    b.append(box("storage", "equipment", CH, "body", 0.4, (0.4826, 0.1768, 0.72), (0, 0.1889, -0.1)))
    for r_, y in ((0, 0.155), (1, 0.2105)):
        b.append(box(f"storage-bay-{r_}", "equipment", CH, "detail", 0.3, (0.052, 0.045, 0.008), (-0.164, y, -0.4585), repeat={"count": 6, "translation": [0.0645, 0, 0]}))
    b.append(ell("storage-led", "equipment", CH, "glazing", 0.1, (0.0065, 0.0065, 0.002), (0.215, 0.189, -0.46)))
    b.append(box("server", "equipment", CH, "body", 0.45, (0.4826, SRV_H, 0.72), (0, SY[0], -0.1), repeat={"count": 6, "translation": [0, 0.0889, 0]}))
    for k, x in enumerate((0.0415, 0.097, 0.1525, 0.208)):
        b.append(box(f"server-bay-{k}", "equipment", CH, "detail", 0.3, (0.043, 0.06, 0.008), (x, 0.3238, -0.4585), repeat={"count": 6, "translation": [0, 0.0889, 0]}))
    b.append(box("server-vent", "equipment", CH, "texture", 0.3, (0.162, 0.0585, 0.003), (-0.124, 0.32305, -0.4615), repeat={"count": 6, "translation": [0, 0.0889, 0]}))
    b.append(ell("server-led", "equipment", CH, "glazing", 0.1, (0.0065, 0.0065, 0.002), (-0.216, 0.32225, -0.46), repeat={"count": 6, "translation": [0, 0.0889, 0]}))
    b.append(box("patch-40", "equipment", BL, "body", 0.4, (0.4826, 0.04345, 0.1), (0, 1.8558, -0.41)))
    b.append(box("brush-41", "equipment", BL, "body", 0.4, (0.4826, 0.04345, 0.1), (0, 1.9002, -0.41)))
    b.append(box("switch-42", "equipment", CH, "body", 0.45, (0.4826, 0.04345, 0.4), (0, 1.9447, -0.26)))
    for r_, y in ((0, 1.93775), (1, 1.95175)):
        b.append(box(f"switch-port-{r_}", "equipment", CH, "detail", 0.3, (0.0155, 0.0105, 0.008), (-0.18225, y, -0.4585), repeat={"count": 12, "translation": [0.0245, 0, 0]}))
    b.append(ell("switch-led", "equipment", CH, "glazing", 0.1, (0.0065, 0.0065, 0.002), (0.215, 1.94468, -0.46)))
    return b

def build50():
    b = [wall(-1, False), wall(1, True), J(SEED["roof"]), box("base-rail", "frame", RS, "body", 0.45, (0.54, 0.03, 1), (0, 0.085, 0)),
         box("rear-panel", "frame", RS, "body", 0.4, (0.54, 1.867, 0.012), (0, 1.0335, 0.494)),
         door(holes_door()), box("door-header", "door", MS, "body", 0.2, (0.54, 0.033, 0.025), (0, 1.9835, -0.4870)), box("door-handle", "door", MS, "detail", 0.2, (0.022, 0.13, 0.008), (0.243, 1.015, -0.5025), r=0.003)]
    b += casters()
    return b + equip50()

ALLB = ["+w", "+u", "-u", "+v", "-v"]
def plate(id, region, layer, mat, size, at, faces, tone=0.4, repeat=None):
    d = box(id, "equipment", mat, "detail", tone, size, at, region=region, layer=layer, sectionFaces=faces)
    if repeat: d["repeat"] = repeat
    return d
def cabinet_env(th=0.008, sc=50):
    z0, z1, h = -0.475, 0.5, 1.93
    zc, dz = (z0 + z1) / 2, z1 - z0
    def pl(id, size, at, keep):
        faces = [f for f in ("+u", "-u", "+v", "-v", "+w", "-w") if f != keep]
        return box(id, "frame", RS, "body", 0.45, size, at, region="cabinet", layer="cabinet-sheet", sectionFaces=faces)
    return [pl("th-cabinet-left", (th, h, dz), (-0.3 + th / 2 - 0.0005, 1.035, zc), "-u"), pl("th-cabinet-right", (th, h, dz), (0.3 - th / 2 + 0.0005, 1.035, zc), "+u"),
            pl("th-cabinet-roof", (0.6, th, dz), (0, 2.0 - th / 2 + 0.0005, zc), "+v"), pl("th-cabinet-rear", (0.6, h, th), (0, 1.035, 0.5 - th / 2 + 0.0005), "+w")]
def th_door(thick, z0=-0.4985):
    return box("th-door", "door", MS, "body", 0.18, (0.54, 1.867, thick), (0, 1.0335, z0 + thick / 2), region="door", layer="door-sheet", sectionFaces=["+w"])
def emitter(h):
    return rev("th-exhaust-air", "frame", RS, "detail", 0.4, [(2.0 - h - 0.005, 0.25), (1.995, 0.25), (1.995, 0.0)] if False else [(1.995 - h, 0.25), (1.995, 0.25), (1.995, 0.0)], axis=(0, 1, 0), at=(0, 0, 0.1), region="cabinet", tags=["exhaust-air"], sectionFaces=["+u", "-u", "+v", "-v", "+w", "-w"])
def thermal50():
    t = cabinet_env(0.008) + [th_door(0.025), emitter(0.09),
         plate("th-servers-storage", "servers", "front-bezel", CH, (0.4826, 0.1768, 0.008), (0, 0.1889, -0.4668), ALLB),
         plate("th-servers", "servers", "front-bezel", CH, (0.4826, SRV_H, 0.008), (0, SY[0], -0.4668), ALLB, repeat={"count": 6, "translation": [0, 0.0889, 0]}),
         plate("th-switch", "switch", "switch-front", CH, (0.4826, 0.04345, 0.008), (0, 1.9447, -0.4668), ALLB),
         plate("th-blanks", "blanks", "blank-sheet", BL, (0.4826, 0.0869, 0.008), (0, 1.878, -0.4668), ALLB, tone=0.3)]
    for k, (x, z) in enumerate(((-0.25, -0.4), (-0.25, 0.4), (0.25, -0.4), (0.25, 0.4))):
        d = caster(f"th-caster-{k}", x, z, 0.016); d["bone"] = "feet"; d["region"] = "feet"; d["layer"] = "caster-body"; d["sectionFaces"] = ["-v", "+v"]; t.append(d)
    return t
def structure(sc):
    big = sc == 500; t30 = sc >= 200
    w = 0.0765 if big else (0.0315 if t30 else 0.03)
    rw = 0.0765 if big else (0.0315 if t30 else 0.02)
    s = []
    for i, (x, z) in enumerate(((-0.285, -0.46), (0.285, -0.46), (-0.285, 0.485), (0.285, 0.485))):
        if big: x, z = (-0.25 if x < 0 else 0.25), (-0.44 if z < 0 else 0.44)
        s.append(st(box(f"member-frame-post-{i}", "frame", RS, "body", 0.5, (w, 1.93 if not big else 1.9, w), (x, 1.035, z)), "frame", "member"))
    if sc == 50:
        s.append(st(ext("member-frame-base", "frame", RS, "body", 0.5, rect(-0.3, 0.3, -0.47, 0.5), "zx", 0.07, 0.1, holes=[rect(-0.27, 0.27, -0.44, 0.47)]) if False else ext("member-frame-base", "frame", RS, "body", 0.5, rect(-0.47, 0.5, -0.3, 0.3), "zx", 0.07, 0.1, holes=[rect(-0.44, 0.47, -0.27, 0.27)]), "frame", "member"))
        s.append(st(ext("member-frame-top", "frame", RS, "body", 0.5, rect(-0.47, 0.5, -0.3, 0.3), "zx", 1.967, 2.0, holes=[rect(-0.44, 0.47, -0.27, 0.27)]), "frame", "member"))
    elif sc in (100, 200):
        t_ = 0.03 if sc == 100 else 0.0315
        s.append(st(ext("member-frame-ring", "frame", RS, "body", 0.5, rect(-0.47, 0.5, -0.3, 0.3), "zx", 1.967 - (0 if sc == 100 else 0.0), 2.0 + 0.0 if False else 2.0, holes=[rect(-0.4, 0.43, -0.22, 0.22)]) if False else ext("member-frame-ring", "frame", RS, "body", 0.5, rect(-0.47, 0.5, -0.3, 0.3), "zx", 2.0 - max(t_, 0.033), 2.0, holes=[rect(-0.4, 0.43, -0.22, 0.22)]), "frame", "member"))
    nrail = 4 if sc <= 100 else 2
    pos = [(-0.2513, -0.455), (0.2513, -0.455), (-0.2513, 0.26), (0.2513, 0.26)] if nrail == 4 else [(-0.2513, -0.1), (0.2513, -0.1)]
    if big: pos = [(-0.2, -0.1), (0.2, -0.1)]
    for i, (x, z) in enumerate(pos):
        s.append(st(box(f"member-equipment-rail-{i}", "equipment", RS, "body", 0.5, (rw, 1.8, rw), (x, 1.03, z)), "equipment", "member"))
    dt = 0.025 if sc <= 100 else (0.0315 if sc == 200 else 0.0765)
    inner = 0.04 if sc <= 100 else (0.05 if sc == 200 else 0.075)
    s.append(st(ext("member-door", "door", MS, "body", 0.5, rect(-0.27, 0.27, 0.1, 1.967), "xy", -0.4985, -0.4985 + dt, holes=[rect(-0.245, 0.245, 0.1 + inner, 1.965)] if sc in (100, 200) else [rect(-0.27 + inner, 0.27 - inner, 0.1 + inner, 1.967 - inner)]), "door", "member"))
    pt = 0.008 if sc == 50 else (0.0155 if sc == 100 else (0.0315 if sc == 200 else 0.0765))
    s.append(st(ext("support-base-contact", "feet", CR, "ground", 0.1, rect(-0.42, 0.42, -0.27, 0.27), "zx", -pt, 0.0, holes=[rect(-0.36, 0.36, -0.2, 0.2)]), "feet", "support"))
    s.append(st(ell("mass-equipment", "equipment", CH, "detail", 0.15, (0.2, 0.5, 0.3), (0, 0.8, -0.1)), "equipment", "mass"))
    return s

def body_coarse(sc):
    big = sc == 500
    if big:
        return [ext("cabinet", "frame", RS, "body", 0.5, rect(-0.3, 0.3, 0.07, 2.0), "xy", -0.5, 0.5, holes=[rect(-0.25, 0.25, 0.145, 1.967)]),
                box("rear-panel", "frame", RS, "body", 0.4, (0.6, 1.93, 0.0765), (0, 1.035, 0.46175)),
                box("server-block", "equipment", CH, "body", 0.42, (0.4, 0.7112, 0.61), (0, 0.4556, -0.045)),
                box("top-group", "equipment", CH, "body", 0.4, (0.4, 0.13335, 0.5), (0, 1.9, -0.1))]
    t = 0.0315 if sc == 200 else 0.03
    roofh = 0.06 if sc == 200 else 0.04
    hole = [rect(0.43, 0.46, -0.15, 0.15)] if sc == 100 else None
    b = [wall(-1, False, t), wall(1, False, t),
         ext("roof", "frame", RS, "body", 0.45, rect(-0.475, 0.5, -0.3, 0.3), "zx", 2.0 - roofh, 2.0, holes=hole),
         box("base-rail", "frame", RS, "body", 0.45, (0.54, 0.034 if sc == 100 else 0.05, 1), (0, 0.07 + (0.017 if sc == 100 else 0.025), 0)),
         box("rear-panel", "frame", RS, "body", 0.4, (0.54, 1.867, 0.0315 if sc == 200 else 0.0155), (0, 1.0335, 0.5 - (0.01575 if sc == 200 else 0.00775))),
         ext("door-field", "door", MS, "body", 0.18, rect(-0.27, 0.27, 0.1, 1.967), "xy", -0.4985, -0.4985 + (0.03 if sc == 100 else 0.0315), holes=[rect(-0.245, 0.245, 0.1 + (0.03 if sc == 100 else 0.05), 1.967 - 0.002)])]
    if sc == 100: b.append(box("door-handle", "door", MS, "detail", 0.2, (0.026, 0.16, 0.0155), (0.241, 1.01, -0.4905), r=0.006))
    b += casters(0.015 if sc == 100 else 0.0155, 0.07)
    if sc == 100:
        b += [box("storage", "equipment", CH, "body", 0.4, (0.4826, 0.1778, 0.72), (0, 0.1889, -0.1)),
              box("server", "equipment", CH, "body", 0.45, (0.4826, 0.0879, 0.72), (0, SY[0], -0.1), repeat={"count": 6, "translation": [0, 0.0889, 0]}),
              box("patch-brush", "equipment", BL, "body", 0.4, (0.4826, 0.0869, 0.1), (0, 1.878, -0.41)),
              box("switch", "equipment", CH, "body", 0.45, (0.4826, 0.04345, 0.4), (0, 1.9447, -0.26))]
    else:
        b += [box("server-block", "equipment", CH, "body", 0.42, (0.4826, 0.7112, 0.72), (0, 0.4556, -0.1)),
              box("top-group", "equipment", CH, "body", 0.4, (0.4826, 0.13335, 0.6), (0, 1.9, -0.16))]
    return b
def thermal_coarse(sc):
    big = sc == 500; th = {100: 0.0155, 200: 0.0315, 500: 0.0765}[sc]
    dz = -0.4985 + {100: 0.03, 200: 0.0315, 500: 0.0765}[sc]
    t = cabinet_env(th) + [th_door(th), emitter(0.09 if not big else 0.12)]
    t += [plate("th-servers", "servers", "front-bezel", CH, (0.4826, 0.7112, th), (0, 0.4556, dz + 0.001 + th / 2), ALLB),
          plate("th-switch", "switch", "switch-front", CH, (0.28 if big else 0.4826, 0.0765 if big else 0.04345, th), (0, 1.9447, dz + 0.001 + th / 2), ALLB),
          plate("th-blanks", "blanks", "blank-sheet", BL, (0.4826, 0.0765 if big else 0.0869, th), (0, 1.878, dz + 0.001 + th / 2), ALLB, tone=0.3)]
    if big:
        t.append(box("th-feet", "feet", CR, "body", 0.2, (0.0765, 0.0765, 0.0765), (0, 0.03825, 0), region="feet", layer="caster-body", sectionFaces=["-v", "+v"]))
    else:
        r = 0.0162 if sc == 100 else 0.0168
        for k, (x, z) in enumerate(((-0.25, -0.4), (-0.25, 0.4), (0.25, -0.4), (0.25, 0.4))):
            d = rev(f"th-caster-{k}", "feet", CR, "body", 0.2, [(0, 0.011), (0.004, r), (0.066, r), (0.07, 0)] if sc == 200 else [(0, 0.011), (0.004, r), (0.058, r), (0.066, 0.011), (0.07, 0)], axis=(0, 1, 0), at=(x, 0, z), region="feet", layer="caster-body", sectionFaces=["-v", "+v"]); t.append(d)
    for d in t[4:]:
        if d.get("kind") == "box": d["size"][0] -= 0.0004; d["transform"]["translation"][1] += 0.0003
    return t
def th_ref(id, of, region, layer): return {"id": id, "of": of, "region": region, "layer": layer}

DEC = {
"50": (["Body: a rack of steel side walls (the vented right wall cut with seven louvre slots), a roof plate with its two fan openings and cable slot, base rail, rear panel, and a mesh door cut with 588 real slots in 42 staggered rows plus its pull handle and header; one storage chassis and six servers with bays, vent grilles and LEDs, two blank panels and a switch with 24 ports on the equipment bone, four rounded caster stems. Depths behind the drawn front are estimates (equipment 0.72 m deep, rear panel, 15 mm door gap).",
        "Thermal: every region has one skin with its single layer: the cabinet is the outer envelope (floor and door sides are section faces), the door is a 25 mm plate whose inner side is a section face, servers, switch and blanks are 7.5 mm front bezels whose other faces are section faces (exposed area = front faces), the feet reuse the caster stems and the exhaust-air emitter is a 0.5 m duct under the roof inside the cabinet envelope.",
        "Structure: four corner posts, a base and a top frame, four mounting rails, the door frame ring and a ground contact frame under the casters as the base-contact support, plus the equipment mass block; the fixed joints of the rack are not modelled as joint solids."],
       ["Körper: ein Rack aus Stahlseitenwänden (die rechte mit sieben Lüftungsschlitzen), einer Dachplatte mit zwei Lüfteröffnungen und Kabelschlitz, Fußschiene, Rückwand und einer Gittertür mit 588 echten Schlitzen in 42 versetzten Reihen samt Griff und Kopfleiste; ein Storage-Gehäuse und sechs Server mit Schächten, Lüftungsgittern und LEDs, zwei Blindplatten und ein Switch mit 24 Ports am Knochen equipment, vier gerundete Rollenstützen. Tiefen hinter der gezeichneten Front sind Schätzungen (Einbauten 0,72 m tief, Rückwand, 15 mm Türspalt).",
        "Thermik: jede Region hat eine Haut mit ihrer einen Schicht: der Schrank ist die äußere Hülle (Boden- und Türseite sind Schnittflächen), die Tür eine 25-mm-Platte, deren Innenseite Schnittfläche ist, Server, Switch und Blenden sind 7,5-mm-Frontblenden, deren übrige Flächen Schnittflächen sind (freie Fläche = Frontflächen), die Füße verwenden die Rollenstützen und der Emitter Abluft ist ein 0,5-m-Kanal unter dem Dach innerhalb der Schrankhülle.",
        "Tragwerk: vier Eckpfosten, ein Boden- und ein Dachrahmen, vier Montageschienen, der Türrahmenring und ein Bodenkontaktrahmen unter den Rollen als Auflager base-contact sowie der Massenblock der Einbauten; die festen Gelenke des Racks sind nicht als Gelenkkörper modelliert."]),
"100": (["Body: modelled anew against the 1:100 drawings: 30 mm walls, one roof plate with the cable slot, base rail 34 mm, plain door plate (the 588 slots are merged into its tone) with a 15 mm flush pull, storage and six servers as plain 0.72 m deep chassis, one blank block for patch and brush panels, the switch, 40 mm caster stems; bays, vents, ports and LEDs are left out (15 mm smallest feature).",
         "Thermal: same six regions with one solid each: envelope, 15 mm door plate, and the servers as one 0.4826 x 0.7112 m front plate (storage and six servers merged), switch and blank plates, casters reusing the body, emitter duct.",
         "Structure: posts and a single frame ring (base and top merged), the four rails, door ring, contact frame and the mass."],
        ["Körper: neu gegen die Zeichnungen 1:100 gebaut: 30-mm-Wände, eine Dachplatte mit Kabelschlitz, Fußschiene 34 mm, glatte Türplatte (die 588 Schlitze gehen im Farbton auf) mit bündigem 15-mm-Griff, Storage und sechs Server als schlichte 0,72 m tiefe Gehäuse, ein Blindblock für Patch- und Bürstenblende, der Switch, Rollenstützen von 40 mm; Schächte, Lüftungsgitter, Ports und LEDs entfallen (Mindestmaß 15 mm).",
         "Thermik: dieselben sechs Regionen mit je einem Körper: Hülle, 15-mm-Türplatte und die Server als eine Frontplatte von 0,4826 x 0,7112 m (Storage und sechs Server zusammengefasst), Switch- und Blindplatte, Rollen aus dem Körper, Emitterkanal.",
         "Tragwerk: Pfosten und ein einziger Rahmenring (Boden und Dach zusammengefasst), die vier Schienen, Türring, Kontaktrahmen und die Masse."]),
"200": (["Body: walls, roof, rear panel and base rail at 31-60 mm, one 50 mm door plate, one server block (storage and servers merged), one top group (blanks and switch merged), four 50 mm casters; handle, slots and every detail below 30 mm are left out.",
         "Thermal: six regions, one solid each at 31 mm plates and the caster stems; emitter duct inside the envelope.",
         "Structure: four 31 mm posts, one frame ring, two merged rails, 31 mm door ring, 31 mm contact frame and the mass block."],
        ["Körper: Wände, Dach, Rückwand und Fußschiene mit 31 bis 60 mm, eine 50-mm-Türplatte, ein Serverblock (Storage und Server zusammengefasst), eine Obergruppe (Blenden und Switch zusammengefasst), vier 50-mm-Rollen; Griff, Schlitze und jedes Detail unter 30 mm entfallen.",
         "Thermik: sechs Regionen mit je einem Körper aus 31-mm-Platten und den Rollenstützen; Emitterkanal in der Hülle.",
         "Tragwerk: vier 31-mm-Pfosten, ein Rahmenring, zwei zusammengefasste Schienen, 31-mm-Türring, 31-mm-Kontaktrahmen und der Massenblock."]),
"500": (["Body: the cabinet is one 0.6 x 2 x 1 m block, the equipment two blocks (server block and top group); door, feet and walls are merged into the block as drawn at 1:500.",
         "Thermal: six regions with 75 mm solids: envelope, 75 mm door and front plates, the feet as one 75 mm block at the contact anchor (the four casters cannot be modelled at 75 mm, the exposed area stays within 30 % of areaM2), emitter duct.",
         "Structure: four 75 mm posts, two 75 mm rails, a 75 mm door frame, a 75 mm contact frame and the equipment mass."],
        ["Körper: der Schrank ist ein Block von 0,6 x 2 x 1 m, die Einbauten zwei Blöcke (Serverblock und Obergruppe); Tür, Füße und Wände gehen wie in der Zeichnung 1:500 im Block auf.",
         "Thermik: sechs Regionen mit 75-mm-Körpern: Hülle, 75-mm-Tür- und Frontplatten, die Füße als ein 75-mm-Block am Kontaktanker (die vier Rollen sind bei 75 mm nicht abbildbar, die freie Fläche bleibt innerhalb von 30 % von areaM2), Emitterkanal.",
         "Tragwerk: vier 75-mm-Pfosten, zwei 75-mm-Schienen, ein 75-mm-Türrahmen, ein 75-mm-Kontaktrahmen und die Masse der Einbauten."]),
}
def volumes():
    v = {"50": scale_vol(50, build50(), thermal50(), structure(50), *DEC["50"])}
    for sc in (100, 200, 500): v[str(sc)] = scale_vol(sc, body_coarse(sc), thermal_coarse(sc), structure(sc), *DEC[str(sc)])
    return v
if __name__ == "__main__":
    write("appliances/server-rack", volumes())
    print(run("check", "appliances/server-rack"))
