"""🎼 Drawings, models, rig, poses, clips, physics and the document of the used harmonium, executed into the namespace of author-harmonium.py (which holds the numbers and the part list)."""

exec(open(os.path.join(HERE, "author-harmonium-texts.py"), encoding="utf8").read())


def expanded(parts):
    """🔂 Every part with its repeat unrolled into single parts, for the 2D drawings."""
    out = []
    for p in parts:
        rep = p.extra.get("repeat")
        if not rep or p.kind != "box":
            out.append(p)
            continue
        (x0, y0, z0), (x1, y1, z1) = p.geo
        dz = rep["translation"][2]
        for k in range(rep["count"]):
            out.append(Part(f"{p.id}-{k}", p.bone, "box", ((x0, y0, z0 + k * dz), (x1, y1, z1 + k * dz)), p.material, p.tone, p.region, p.role, p.views, {}, p.pen))
    return out


def stroke(lst, sid, bone, pts, role="detail", region="case", tone=0.25, z=30.0, closed=False, filled=False, material=None):
    """🖋 One open or closed detail or texture stroke."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": "poly", "closed": closed, "points": [[r6(a), r6(b)] for a, b in pts]}
    if region:
        s["region"] = region
    if material:
        s["material"] = material
    lst.append(s)


def circle_pts(cx, cy, r, n=16):
    """⭕ A polygon on a circle."""
    return [(cx + r * math.cos(2 * math.pi * k / n), cy + r * math.sin(2 * math.pi * k / n)) for k in range(n)]


def details_of(scale):
    """🖊 The detail and texture strokes of one scale: key divisions, porcelain stop faces, inlay and veneer seams, the reed pan in the section, and the traces of use."""
    e, f, p = [], [], []
    if scale == 200:
        return e, f, p
    stroke(e, "reed-pan", "case", [(0.03, 0.62), (0.34, 0.62)], tone=0.3)
    stroke(e, "pallet-board", "case", [(0.03, 0.70), (0.34, 0.70)], tone=0.3)
    stroke(e, "reservoir-board", "case", [(0.03, 0.345), (0.36, 0.345)], region="bellows", tone=0.3)
    stroke(f, "nameboard-inlay", "case", [(-0.40, 0.84), (0.40, 0.84), (0.40, 0.89), (-0.40, 0.89)], closed=True, tone=0.3, z=12.0)
    stroke(f, "kneeboard-panel", "case", [(-0.50, 0.40), (0.50, 0.40), (0.50, 0.62), (-0.50, 0.62)], closed=True, tone=0.3, z=12.0)
    stroke(p, "lid-veneer-seam", "lid", [(0.03, 0.0), (0.43, 0.0)], tone=0.3, z=40.0)
    if scale == 50:
        for k in range(1, 36):
            z = -KEY_Z + k * 0.0235
            stroke(f, f"key-gap-{k}", "case", [(z, 0.708), (z, 0.72)], role="texture", region="keys", tone=0.4, z=11.6)
        for k, z in enumerate(STOP_Z):
            bone = "stops" if z in DRAWN else "case"
            stroke(f, f"stop-face-{k}", bone, circle_pts(z, STOPS_AT[1], 0.0075, 12), role="texture", tone=0.95, z=14.0, closed=True)
        for k, (x0, x1) in enumerate(((0.04, 0.09), (0.11, 0.16), (0.18, 0.23), (0.25, 0.30))):
            stroke(e, f"reed-cell-{k}", "case", [(x0, 0.625), (x1, 0.625), (x1, 0.66), (x0, 0.66)], role="texture", tone=0.35, closed=True, z=24.0)
        stroke(e, "side-grain-0", "case", [(0.03, 0.15), (0.21, 0.152), (0.41, 0.149)], role="texture", tone=0.45, z=1.0)
        stroke(e, "side-grain-1", "case", [(0.03, 0.88), (0.22, 0.883), (0.40, 0.878)], role="texture", tone=0.45, z=1.0)
        stroke(e, "carpet-wear-left", "treadle-l", [(0.16, -0.0285), (0.25, -0.0285)], role="texture", region="treadles", tone=0.6, z=26.0)
        stroke(e, "dust-on-bottom", "case", [(0.06, 0.0565), (0.30, 0.0565)], role="texture", tone=0.6, z=26.0)
        stroke(f, "kneeboard-rub-left", "case", [(-0.30, 0.52), (-0.24, 0.535), (-0.18, 0.52)], role="texture", tone=0.6, z=12.5)
        stroke(f, "kneeboard-rub-right", "case", [(0.18, 0.52), (0.24, 0.535), (0.30, 0.52)], role="texture", tone=0.6, z=12.5)
        stroke(f, "inventory-label", "case", [(0.34, 0.43), (0.42, 0.43), (0.42, 0.48), (0.34, 0.48)], role="detail", tone=0.93, z=12.8, closed=True, filled=True, material="paper-label")
        stroke(f, "inventory-label-text", "case", [(0.35, 0.455), (0.41, 0.455)], role="texture", tone=0.3, z=12.9)
        stroke(f, "cheek-chip-right", "case", [(-0.6, 0.70), (-0.589, 0.704), (-0.578, 0.70), (-0.578, 0.69), (-0.6, 0.69)], role="texture", tone=0.7, z=36.0, closed=True)
        stroke(f, "shellac-wear-left", "case", [(0.581, 0.64), (0.584, 0.72)], role="texture", tone=0.6, z=36.0)
        stroke(f, "shellac-wear-right", "case", [(-0.581, 0.64), (-0.584, 0.72)], role="texture", tone=0.6, z=36.0)
        stroke(f, "carpet-tear-left", "treadle-l", [(0.155, 0.19), (0.16, 0.205), (0.175, 0.2)], role="texture", region="treadles", tone=0.6, z=46.0)
        stroke(f, "chipped-key", "case", [(-0.0935, 0.714), (-0.0805, 0.716)], role="texture", region="keys", tone=0.3, z=11.7)
        stroke(p, "water-ring", "lid", circle_pts(0.20, -0.30, 0.045, 18), role="texture", tone=0.55, closed=True, z=40.5)
        for k, (zz, xx) in enumerate(((-0.45, 0.10), (-0.05, 0.32), (0.30, 0.16), (0.48, 0.36))):
            stroke(p, f"lid-craze-{k}", "lid", [(xx, zz), (xx + 0.03, zz + 0.012), (xx + 0.055, zz + 0.004)], role="texture", tone=0.55, z=40.5)
        stroke(p, "lid-front-wear", "lid", [(0.425, -0.5), (0.425, 0.5)], role="texture", tone=0.6, z=40.6)
    return e, f, p


def representation(scale):
    """🖨 The authored representation of one scale: part shapes in the three viewpoints, the section of the side elevation at 1:50 and 1:100, details, pens, hatches and decisions."""
    parts = expanded(parts_of(scale))
    section = scale in (50, 100)
    geo = {"elevation": [], "front": [], "plan": []}
    for part in parts:
        if "elevation" in part.views:
            sh = part.elevation(section, scale)
            if sh is not None and not (section and part.id.startswith("stop") and part.zr()[0] > PLANE + 0.02) and not (scale == 100 and part.id.startswith("cheek-block")):
                geo["elevation"].append(sh)
            if not section and part.id.startswith(("side-left", "handle-left")):
                geo["elevation"].remove(sh)
        if "front" in part.views:
            geo["front"].append(part.front())
        if "plan" in part.views:
            geo["plan"].append(part.plan())
    e, f, p = details_of(scale)
    geo["elevation"] += e
    geo["front"] += f
    geo["plan"] += p
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": None, "ground": 0.5},
            100: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35},
            200: {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25}}[scale]
    hatches = []
    if scale == 50:
        hatches = [{"id": "cut-softwood", "title": T("Cut timber of the case, the action and the boards", "Geschnittenes Holz von Gehäuse, Zungenwerk und Brettern"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.6, "penMm": 0.1},
                   {"id": "cut-other", "title": T("Cut cloth, leather, iron and ebonised parts", "Geschnittenes Tuch, Leder, Eisen und schwarz gebeizte Teile"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}]
    rng = {50: (20, 75), 100: (75, 150), 200: (150, 350)}[scale]
    rep = {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng[0], "maxDenominator": rng[1]}, "pens": pens, "hatches": hatches, "geometry": geo, "decisions": DECISIONS_2D[scale]}
    if section:
        rep["sections"] = {"elevation": {"plane": PLANE, "label": T("A-A", "A-A")}}
    return rep


def joint_pin(jid, member, bone, centre_world, z0, z1, scale, material="iron"):
    """🔩 A hinge pin joint solid along the lateral axis at a world rest point, at least one smallest feature thick."""
    r = max(0.006, FEATURE[scale] * 0.6)
    c = local(bone, centre_world)
    ring = [(c[0] + r * math.cos(2 * math.pi * k / 12), c[1] + r * math.sin(2 * math.pi * k / 12)) for k in range(12)]
    return {"id": jid, "kind": "extrude", "bone": bone, "material": material, "role": "detail", "tone": 0.5, "plane": "xy", "profile": [[r6(a), r6(b)] for a, b in ccw(ring)],
            "from": r6(z0), "to": r6(z1), "member": member, "part": "joint"}


def mass_marker(mid, bone, kg_centre, z, scale):
    """⚖ The centre-of-mass marker of a bone."""
    r = FEATURE[scale]
    c = local(bone, kg_centre)
    return {"id": mid, "kind": "ellipsoid", "bone": bone, "material": "pine", "role": "detail", "tone": 0.5, "radii": [r, r, r], "transform": {"translation": [r6(c[0]), r6(c[1]), r6(z)]}, "member": bone, "part": "mass"}


def emitter_patch(sid, side, scale):
    """💨 The outlet patch of an exhaust valve on the underside of an exhauster: the flap opening where the expelled air leaves."""
    sg = 1 if side == "l" else -1
    et = {50: EXH_T, 100: 0.015, 200: 0.03}[scale]
    d = max(FEATURE[scale], 0.008)
    zc = sg * 0.28
    return {"id": sid, "kind": "box", "bone": f"exhauster-{side}", "material": "rubber-cloth", "role": "detail", "tone": 0.2, "size": [0.06, r6(d), 0.06],
            "transform": {"translation": [0.11, r6(et + d / 2), r6(zc)]}, "region": "bellows", "tags": [f"exhaust-{'left' if sg > 0 else 'right'}"]}


def thermal_list(scale, parts):
    """🌡 The thermal list of one scale: every body part by reference to its region and the two exhaust-valve outlets."""
    out = []
    for part in parts:
        if part.extra.get("texture"):
            continue
        region = part.region if isinstance(part.region, str) else part.region[0]
        out.append({"id": f"th-{part.id}", "of": part.id, "region": region})
    out.append(emitter_patch("th-exhaust-left", "l", scale))
    out.append(emitter_patch("th-exhaust-right", "r", scale))
    return out


def structure_list(scale, parts):
    """🏗 The structural idealisation of one scale: the carcass boards as members of the case, every moving board as the member of its bone, the hinge pins as joints and the centres of mass."""
    names = {p.id for p in parts}
    out = []
    for pid in ("side-right", "side-left", "back", "bottom", "keybed", "reservoir", "action", "stop-rail"):
        if pid in names:
            out.append({"id": f"member-{pid}", "of": pid, "member": "case", "part": "member"})
    for pid, bone in (("treadle-left", "treadle-l"), ("treadle-right", "treadle-r"), ("exhauster-left", "exhauster-l"), ("exhauster-right", "exhauster-r"),
                      ("strap-left", "strap-l"), ("strap-right", "strap-r"), ("fall", "fall"), ("lid", "lid"), ("desk", "desk")):
        out.append({"id": f"member-{pid}", "of": pid, "member": bone, "part": "member"})
    for side, sg in (("l", 1), ("r", -1)):
        tz = sorted((sg * TREADLE_Z[0], sg * TREADLE_Z[1]))
        out.append(joint_pin(f"joint-treadle-{side}", f"treadle-{side}", f"treadle-{side}", TREADLE_HINGE, tz[0], tz[1], scale))
        ez = sorted((sg * EXH_Z[0], sg * EXH_Z[1]))
        out.append(joint_pin(f"joint-exhauster-{side}", f"exhauster-{side}", f"exhauster-{side}", EXH_HINGE, ez[0], ez[1], scale, "leather"))
        sz = sorted((sg * STRAP_Z[0], sg * STRAP_Z[1]))
        out.append(joint_pin(f"joint-strap-{side}", f"strap-{side}", f"strap-{side}", exh_tip(0), sz[0], sz[1], scale, "leather"))
    zi = SIDE_OUT - (0.03 if scale == 200 else 0.022)
    out.append(joint_pin("joint-fall", "fall", "fall", FALL_HINGE, -zi, zi, scale, "iron"))
    out.append(joint_pin("joint-lid", "lid", "lid", (LID_HINGE[0], LID_HINGE[1] - 0.01), -zi, zi, scale, "iron"))
    out.append(joint_pin("joint-desk", "desk", "desk", (LID_HINGE[0] + DESK_AT, LID_HINGE[1] + 0.006), -DESK_Z, DESK_Z, scale, "iron"))
    for bone, (kg, sx, sy) in BONE_MASS.items():
        if bone == "stops" and scale == 200:
            continue
        zc = {"treadle-l": 0.105, "treadle-r": -0.105, "exhauster-l": 0.28, "exhauster-r": -0.28, "strap-l": 0.11, "strap-r": -0.11}.get(bone, 0.0)
        if bone == "stops":
            zc = 0.0
        out.append(mass_marker(f"mass-{bone}", bone, (sx / kg, sy / kg), zc, scale))
    return out


def volume(scale):
    """🧊 The authored volume of one scale: the body from the part list, the thermal and structure lists and the decisions."""
    parts = parts_of(scale)
    body = [p.solid() for p in parts]
    if scale == 50:
        body.append({"id": "inventory-label", "kind": "box", "bone": "case", "material": "paper-label", "role": "detail", "tone": 0.93, "size": [0.0008, 0.05, 0.08],
                     "transform": {"translation": [0.4004, 0.455, 0.38]}, "texture": True, "sheet": True})
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_list(scale, parts), "structure": structure_list(scale, parts), "decisions": DECISIONS_3D[scale]}


def bone_mass(bone, ref="ord-hume-1986"):
    """⚖ The segment mass of a bone from its parts."""
    kg, sx, sy = BONE_MASS[bone]
    c = local(bone, (sx / kg, sy / kg))
    return {"kg": r6(kg), "comX": r6(c[0]), "comY": r6(c[1]), "distribution": "point", "source": "authored-estimate", "reference": ref}


def rig():
    """🦴 The mechanical rig: a massless floor root, the case on a vertical slide for lifting, the two treadles, exhausters and straps, the fall, the lid with the music desk and the slide of the drawn stops."""
    B = [
        {"id": "floor", "parent": None, "title": T("Showroom floor reference (massless, fixed root)", "Bezug Ladenboden (masselos, feste Wurzel)"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.04, "joint": {"type": "fixed"}},
        {"id": "case", "parent": "floor", "title": T("Case with the reed action, the reservoir, the keyboard, the stop rail and the crest (lifted by its two drop handles)", "Gehäuse mit Zungenwerk, Magazinbalg, Klaviatur, Registerleiste und Aufsatz (an den zwei Klappgriffen gehoben)"),
         "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.53, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.2}, "mass": bone_mass("case"),
         "section": {"areaM2": r6(2 * 0.022 * 0.53 + 0.015 * 1.156), "secondMomentM4": r6(2 * 0.022 * 0.53 ** 3 / 12), "extremeFibreM": 0.265, "material": "veneered-softwood", "title": T("Two side panels 22 x 530 mm with the back panel", "Zwei Seitenwände 22 x 530 mm mit der Rückwand")}},
    ]
    for side, name_en, name_de in (("l", "left", "linker"), ("r", "right", "rechter")):
        B.append({"id": f"treadle-{side}", "parent": "case", "title": T(f"Carpeted {name_en} treadle, hinged at the heel", f"Mit Teppich belegter {name_de} Tritt, an der Ferse angeschlagen"), "x": TREADLE_HINGE[0], "y": TREADLE_HINGE[1], "angleDeg": TREADLE_REST, "lengthM": TREADLE_LEN,
                  "joint": {"type": "revolute", "minDeg": -2, "maxDeg": 12}, "mass": bone_mass(f"treadle-{side}"),
                  "section": {"areaM2": r6(0.16 * 0.022), "secondMomentM4": 0.16 * 0.022 ** 3 / 12, "extremeFibreM": 0.011, "material": "beech", "title": T("Beech treadle board 160 x 22 mm", "Buchen-Trittbrett 160 x 22 mm")}})
        B.append({"id": f"exhauster-{side}", "parent": "case", "title": T(f"Moving board of the {name_en} exhauster, hinged at the front edge of the reservoir", f"Bewegliches Brett des {name_de[:-1]}n Schöpfbalgs, an der Vorderkante des Magazinbalgs angeschlagen"), "x": EXH_HINGE[0], "y": EXH_HINGE[1], "angleDeg": 180, "lengthM": EXH_LEN,
                  "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 20}, "mass": bone_mass(f"exhauster-{side}")})
        B.append({"id": f"strap-{side}", "parent": f"exhauster-{side}", "title": T(f"Leather strap from the {name_en} exhauster down to the treadle toe", f"Lederriemen vom {name_de[:-1]}n Schöpfbalg hinab zur Trittspitze"), "x": EXH_LEN, "y": EXH_T, "angleDeg": r6(STRAP_REST), "lengthM": r6(STRAP_LEN),
                  "joint": {"type": "revolute", "minDeg": -40, "maxDeg": 10}, "mass": bone_mass(f"strap-{side}")})
    B += [
        {"id": "fall", "parent": "case", "title": T("Fall board over the keys, hinged at the back of the key well", "Tastenklappe über der Klaviatur, hinten in der Tastenmulde angeschlagen"), "x": FALL_HINGE[0], "y": FALL_HINGE[1], "angleDeg": 0, "lengthM": FALL_LEN,
         "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 95}, "mass": bone_mass("fall")},
        {"id": "lid", "parent": "case", "title": T("Top lid over the action, hinged in front of the crest", "Deckel über dem Werk, vor dem Aufsatz angeschlagen"), "x": LID_HINGE[0], "y": LID_HINGE[1], "angleDeg": 0, "lengthM": LID_LEN,
         "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 85}, "mass": bone_mass("lid")},
        {"id": "desk", "parent": "lid", "title": T("Folding music desk on the lid, hinged at its front edge", "Klappbares Notenpult auf dem Deckel, an seiner Vorderkante angeschlagen"), "x": DESK_AT, "y": 0, "angleDeg": 180, "lengthM": DESK_LEN,
         "joint": {"type": "revolute", "minDeg": -80, "maxDeg": 0}, "mass": bone_mass("desk")},
        {"id": "stops", "parent": "case", "title": T("Slide of the four drawn stop knobs (Diapason 8', Dulciana 8', Principal 4', Forte)", "Schieber der vier gezogenen Registerknöpfe (Diapason 8', Dulciana 8', Principal 4', Forte)"), "x": STOPS_AT[0], "y": STOPS_AT[1], "angleDeg": 0, "lengthM": STOP_L,
         "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.02}, "mass": bone_mass("stops")},
    ]
    return {"kind": "mechanical", "root": "floor", "bones": B}


def anchors():
    """📍 Floor contacts of the four feet, the two drop handles, the treadle and key loads and the two exhaust-valve outlets."""
    A = []
    for k, (x, z, en, de) in enumerate(((0.045, -0.589, "right rear", "rechts hinten"), (0.475, -0.589, "right front", "rechts vorn"), (0.045, 0.589, "left rear", "links hinten"), (0.475, 0.589, "left front", "links vorn"))):
        A.append({"id": f"foot-{k}", "bone": "case", "x": x, "y": 0, "z": z, "kind": "contact", "support": "contact", "footprintLengthM": 0.09, "footprintWidthM": 0.022, "frictionCoefficient": 0.45,
                  "title": T(f"Foot of the side panel {en} on the floor", f"Fuß der Seitenwand {de} auf dem Boden")})
    for side, sg, en, de in (("left", 1, "left", "linken"), ("right", -1, "right", "rechten")):
        A.append({"id": f"grip-{side}", "bone": "case", "x": HANDLE_X, "y": GRIP_Y, "z": r6(sg * 0.612), "kind": "support", "support": "pin",
                  "title": T(f"Drop handle on the {en} side panel: the mover's hand", f"Klappgriff an der {de} Seitenwand: Hand des Trägers")})
    for side, sg, en, de in (("l", 1, "left", "linken"), ("r", -1, "right", "rechten")):
        A.append({"id": f"ball-of-foot-{side}", "bone": f"treadle-{side}", "x": 0.22, "y": -0.028, "z": r6(sg * 0.105), "kind": "load", "title": T(f"Ball of the player's {en} foot on the treadle carpet", f"Fußballen des {de} Fußes auf dem Trittteppich")})
        A.append({"id": f"exhaust-{'left' if sg > 0 else 'right'}", "bone": f"exhauster-{side}", "x": 0.11, "y": r6(EXH_T), "z": r6(sg * 0.28), "kind": "emitter",
                  "title": T(f"Exhaust valve under the {en} exhauster", f"Auslassventil unter dem {de} Schöpfbalg")})
    A.append({"id": "hand-bass", "bone": "case", "x": 0.44, "y": 0.72, "z": -0.22, "kind": "load", "title": T("Player's left hand on the bass keys", "Linke Hand des Spielers auf den Basstasten")})
    A.append({"id": "hand-treble", "bone": "case", "x": 0.44, "y": 0.72, "z": 0.18, "kind": "load", "title": T("Player's right hand on the treble keys", "Rechte Hand des Spielers auf den Diskanttasten")})
    return A


REGIONS = ["case", "keys", "treadles", "bellows"]


def binding(played=False, keys_c=0.0, treadle_c=0.0):
    """🌬 The region binding of a pose: indoors, nothing sunlit; the player's fingers on the keys and soles on the carpets while playing."""
    out = {}
    for r in REGIONS:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if r == "keys" and played:
            b = {"exposedFraction": 0.95, "sunlitFraction": 0, "contact": {"fraction": 0.05, "tempC": 32}}
        if r == "treadles" and played:
            b = {"exposedFraction": 0.7, "sunlitFraction": 0, "contact": {"fraction": 0.3, "tempC": 26}}
        out[r] = b
    return out


def pedal_bones(side):
    """🦵 The offsets of one pressed treadle with its exhauster and strap."""
    return {f"treadle-{side}": {"angleDeg": TREADLE_STROKE}, f"exhauster-{side}": {"angleDeg": r6(THETA_PRESSED)}, f"strap-{side}": {"angleDeg": r6(STRAP_PRESSED - STRAP_REST)}}


OPEN_BONES = {"fall": {"angleDeg": FALL_OPEN}, "desk": {"angleDeg": DESK_UP}, "stops": {"dx": STOP_PULL}}


def play_loads(pressed):
    """🎹 The loads of the playing player: the pressing foot, the resting foot and the two hands."""
    other = "r" if pressed == "l" else "l"
    return [
        {"id": f"foot-press", "kind": "occupant", "title": T("Pressing foot on the treadle, 130 N", "Tretender Fuß auf dem Tritt, 130 N"), "anchor": f"ball-of-foot-{pressed}", "fxN": 0, "fyN": -130, "source": "authored-estimate", "reference": "gellerman-1996"},
        {"id": f"foot-rest", "kind": "occupant", "title": T("Resting foot on the raised treadle, 30 N", "Ruhender Fuß auf dem gehobenen Tritt, 30 N"), "anchor": f"ball-of-foot-{other}", "fxN": 0, "fyN": -30, "source": "authored-estimate", "reference": "gellerman-1996"},
        {"id": "hand-bass", "kind": "occupant", "title": T("Left hand holding a chord, 10 N", "Linke Hand hält einen Akkord, 10 N"), "anchor": "hand-bass", "fxN": 0, "fyN": -10, "source": "authored-estimate", "reference": "keyboard-dimensions"},
        {"id": "hand-treble", "kind": "occupant", "title": T("Right hand playing the melody, 10 N", "Rechte Hand spielt die Melodie, 10 N"), "anchor": "hand-treble", "fxN": 0, "fyN": -10, "source": "authored-estimate", "reference": "keyboard-dimensions"},
    ]


FEET = [f"foot-{k}" for k in range(4)]
GRIPS = ["grip-left", "grip-right"]


def poses():
    """🧍 The named poses stored, carried, opened and played, and the in-between key pose of the right stroke."""
    off = {"pumping": 0}
    quiet = {"exhaust-left": 0, "exhaust-right": 0}
    P = [
        {"id": "stored", "title": T("Stored in the showroom", "Im Laden abgestellt"), "description": T(
            "As the dealer keeps it between viewings: standing on its four feet against the wall, the fall closed over the yellowed keys, the music desk folded flat on the lid, every stop pushed in and both treadles raised by their springs.",
            "So wie der Händler es zwischen zwei Besichtigungen hält: auf seinen vier Füßen an der Wand, die Tastenklappe über den vergilbten Tasten geschlossen, das Notenpult flach auf den Deckel geklappt, alle Register eingeschoben und beide Tritte von ihren Federn gehoben."),
         "bones": {}, "ground": "authored", "contacts": FEET, "thermal": {"operating": off, "regions": binding(), "emitters": quiet}, "loads": []},
        {"id": "carried", "title": T("Carried by two movers", "Von zwei Trägern getragen"), "description": T(
            f"Two movers lift the closed {M_TOTAL:.0f} kg harmonium by the iron drop handles on its side panels and carry it 0.10 m above the floor; the handles sit above the centre of mass, so it hangs level.",
            f"Zwei Träger heben das geschlossene, {M_TOTAL:.0f} kg schwere Harmonium an den eisernen Klappgriffen seiner Seitenwände und tragen es 0,10 m über dem Boden; die Griffe sitzen über dem Schwerpunkt, so hängt es waagrecht."),
         "bones": {"case": {"dy": 0.10}}, "ground": "authored", "contacts": GRIPS, "thermal": {"operating": off, "regions": binding(), "emitters": quiet}, "loads": []},
        {"id": "opened", "title": T("Opened for playing", "Zum Spielen geöffnet"), "description": T(
            "The fall leans back against the name board, the music desk stands up on the lid and four stops are drawn; the treadles still rest raised.",
            "Die Tastenklappe lehnt am Namensbrett, das Notenpult steht auf dem Deckel und vier Register sind gezogen; die Tritte ruhen noch oben."),
         "bones": dict(OPEN_BONES), "ground": "authored", "contacts": FEET, "thermal": {"operating": off, "regions": binding(), "emitters": quiet}, "loads": []},
        {"id": "played", "title": T("Played, left treadle down", "Gespielt, linker Tritt unten"), "description": T(
            f"A seated player holds a chord with both hands and pumps with both feet: the left treadle is pressed {TREADLE_STROKE:.0f} degrees down and pulls its exhauster {THETA_PRESSED:.1f} degrees open by the leather strap, the right treadle has risen and its exhauster closes; the fall is open, the desk up, four stops drawn.",
            f"Ein sitzender Spieler hält mit beiden Händen einen Akkord und tritt mit beiden Füßen: der linke Tritt ist {TREADLE_STROKE:.0f} Grad niedergedrückt und zieht seinen Schöpfbalg über den Lederriemen {THETA_PRESSED:.1f} Grad auf, der rechte Tritt ist gestiegen und sein Schöpfbalg schließt sich; Klappe offen, Pult oben, vier Register gezogen."),
         "bones": dict(OPEN_BONES, **pedal_bones("l")), "ground": "authored", "contacts": FEET,
         "thermal": {"operating": {"pumping": 1}, "regions": binding(True), "emitters": {"exhaust-left": 1, "exhaust-right": 0}}, "loads": play_loads("l")},
        {"id": "played-right", "key": True, "title": T("Played, right treadle down", "Gespielt, rechter Tritt unten"), "description": T(
            "Half a pumping cycle later: the right treadle is down and its exhauster open, the left one has risen and its exhauster expels the air it drew.",
            "Einen halben Tretzyklus später: der rechte Tritt ist unten und sein Schöpfbalg offen, der linke ist gestiegen und sein Schöpfbalg stößt die angesaugte Luft aus."),
         "bones": dict(OPEN_BONES, **pedal_bones("r")), "ground": "authored", "contacts": FEET,
         "thermal": {"operating": {"pumping": 1}, "regions": binding(True), "emitters": {"exhaust-left": 0, "exhaust-right": 1}}, "loads": play_loads("r")},
    ]
    return P


def clips():
    """🎞 The carry clip, the pumping loop and the opening of the instrument."""
    carry = {"id": "carry", "title": T("Lift and carry", "Anheben und tragen"), "description": T(
        "The two movers take the drop handles, lift the harmonium 0.10 m clear of the floor in two seconds and hold it there to carry it away.",
        "Die zwei Träger fassen die Klappgriffe, heben das Harmonium in zwei Sekunden 0,10 m vom Boden ab und halten es so, um es wegzutragen."),
        "durationS": 6, "fps": 25, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.5, "pose": "stored", "ease": "smooth"}, {"t": 3.5, "pose": "carried", "ease": "smooth"}, {"t": 6, "pose": "carried", "ease": "smooth"}],
        "contacts": [{"anchor": a, "fromS": 0, "toS": 1.7} for a in FEET] + [{"anchor": a, "fromS": 1.5, "toS": 6} for a in GRIPS],
        "phases": [{"id": "grip", "fromS": 0, "toS": 1.5, "title": T("Taking the handles", "Griffe fassen")}, {"id": "lift", "fromS": 1.5, "toS": 3.5, "title": T("Lifting", "Anheben")}, {"id": "carry", "fromS": 3.5, "toS": 6, "title": T("Carrying", "Tragen")}],
        "events": [{"t": 1.7, "kind": "contact-off", "title": T("The feet leave the floor", "Die Füße verlassen den Boden"), "target": "case"}],
        "tracks": []}
    pump = {"id": "pumping", "title": T("Pumping the treadles", "Treten der Bälge"), "description": T(
        "The player pumps 50 strokes a minute with each foot: one treadle goes down and pulls its exhauster open while the other rises and its exhauster, closed by its spring, expels the air through its valve; the loop repeats every 1.2 s.",
        "Der Spieler tritt mit jedem Fuß 50 Hübe in der Minute: ein Tritt geht nieder und zieht seinen Schöpfbalg auf, während der andere steigt und sein von der Feder geschlossener Schöpfbalg die Luft durch sein Ventil ausstößt; die Schleife wiederholt sich alle 1,2 s."),
        "durationS": 1.2, "fps": 25, "loop": True, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "played", "ease": "smooth"}, {"t": 0.6, "pose": "played-right", "ease": "smooth"}, {"t": 1.2, "pose": "played", "ease": "smooth"}],
        "contacts": [{"anchor": a, "fromS": 0, "toS": 1.2} for a in FEET],
        "phases": [{"id": "left-rises", "fromS": 0, "toS": 0.6, "title": T("Left treadle rises, right goes down", "Linker Tritt steigt, rechter geht nieder")}, {"id": "right-rises", "fromS": 0.6, "toS": 1.2, "title": T("Right treadle rises, left goes down", "Rechter Tritt steigt, linker geht nieder")}],
        "events": [{"t": 0.6, "kind": "motion", "title": T("Right treadle at the bottom", "Rechter Tritt unten"), "target": "treadle-r"}, {"t": 1.2, "kind": "motion", "title": T("Left treadle at the bottom", "Linker Tritt unten"), "target": "treadle-l"}],
        "tracks": [{"target": "emitter:exhaust-left", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": 0.5, "v": 1}, {"t": 0.6, "v": 0}, {"t": 1.1, "v": 0}, {"t": 1.2, "v": 1}]},
                   {"target": "emitter:exhaust-right", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 0.5, "v": 0}, {"t": 0.6, "v": 1}, {"t": 1.1, "v": 1}, {"t": 1.2, "v": 0}]}]}
    opening = {"id": "open-for-playing", "title": T("Open for playing", "Zum Spielen öffnen"), "description": T(
        "The fall is swung up against the name board, the music desk is raised from the lid and four stops are drawn.",
        "Die Tastenklappe wird gegen das Namensbrett hochgeschwenkt, das Notenpult vom Deckel aufgestellt und vier Register werden gezogen."),
        "durationS": 4, "fps": 25, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.5, "pose": "stored", "ease": "smooth", "bones": {"fall": {"angleDeg": FALL_OPEN}}},
                 {"t": 3, "pose": "stored", "ease": "smooth", "bones": {"fall": {"angleDeg": FALL_OPEN}, "desk": {"angleDeg": DESK_UP}}}, {"t": 4, "pose": "opened", "ease": "smooth"}],
        "contacts": [{"anchor": a, "fromS": 0, "toS": 4} for a in FEET],
        "phases": [{"id": "fall", "fromS": 0, "toS": 1.5, "title": T("Opening the fall", "Klappe öffnen")}, {"id": "desk", "fromS": 1.5, "toS": 3, "title": T("Raising the desk", "Pult aufstellen")}, {"id": "stops", "fromS": 3, "toS": 4, "title": T("Drawing the stops", "Register ziehen")}],
        "events": [{"t": 4, "kind": "motion", "title": T("Ready to play", "Spielbereit"), "target": "stops"}],
        "tracks": []}
    return [carry, pump, opening]


UNAVAILABLE_500 = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature", "reason": T(
    "At 1:500 the harmonium is 2.4 mm wide, 1.1 mm deep and 2.3 mm high on paper, under the 3 mm a recognisable drawing needs, and its 22 mm side panels, the 30 mm keyboard and the 60 mm stop rail shrink to 0.04 to 0.12 mm, below the printable feature of 0.2 mm; draw it at 1:200 or larger.",
    "Bei 1:500 ist das Harmonium auf Papier 2,4 mm breit, 1,1 mm tief und 2,3 mm hoch, unter den 3 mm, die eine erkennbare Zeichnung braucht, und seine 22 mm starken Seitenwände, die 30 mm hohe Klaviatur und die 60 mm hohe Registerleiste schrumpfen auf 0,04 bis 0,12 mm, unter das druckbare Merkmal von 0,2 mm; bei 1:200 oder größer zeichnen."),
    "limitingFeature": T("Stop rail 60 mm high", "Registerleiste 60 mm hoch"), "limitingSizeM": 0.06, "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}


def document():
    """📄 The whole asset document."""
    asset = {
        "id": SLUG, "category": "objects", "title": T("Used Harmonium", "Gebrauchtes Harmonium"), "summary": SUMMARY,
        "tags": ["used", "instrument", "music", "second-hand", "keyboard", "reed-organ", "harmonium", "free-reed", "aerophone", "pedal", "bellows", "walnut-veneer", "wood", "object"],
        "provenance": PROVENANCE,
        "dimensions": {"heightM": 1.15, "widthM": 0.53, "depthM": r6(2 * 0.612), "massKg": r6(M_TOTAL)},
        "materials": MATERIALS + [{"id": "paper-label", "title": T("Dealer's paper inventory label", "Inventaretikett des Händlers aus Papier"), "emissivity": 0.9, "solarAbsorptivity": 0.3, "densityKgM3": 800, "conductivityWmK": 0.12, "specificHeatJKgK": 1300, "source": "literature", "reference": "incropera-2018"}],
        "rig": rig(),
        "volumes": {"50": volume(50), "100": volume(100), "200": volume(200), "500": {"status": "unavailable", "denominator": 500}},
        "anchors": anchors(),
        "representations": {"50": representation(50), "100": representation(100), "200": representation(200), "500": UNAVAILABLE_500},
        "symbols": [], "poses": poses(), "clips": clips(), "thermal": THERMAL, "structure": STRUCTURE,
        "presentation": {"defaultPose": "stored", "defaultClip": "pumping", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf8") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print(f"[DEBUG] harmonium {M_TOTAL:.2f} kg com {COM[0]:.4f} {COM[1]:.4f}; exhauster {THETA_PRESSED:.2f} deg; strap {STRAP_LEN:.4f} m")
