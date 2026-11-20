"""🌳 Root values of the round 3 scissor lift outside `volumes`: reference, dimensions, the four-stage rig with lift cylinder, anchors, poses, clip keys, thermal lengths and areas, symbol and texts, applied to the parsed asset."""
import copy
import math

REFERENCE = {"id": "genie-gs-1932", "citation": "Genie (Terex) specification sheet Self-Propelled Scissor Lifts GS-1530, GS-1532, GS-1930, GS-1932 (EN, metric), read 2026-11-19 at https://www.colle.eu/media/12834/genie-gs1930_eng.pdf: GS-1932 platform height 5.79 m, platform height stowed 1.00 m, platform 1.63 x 0.74 m, slide-out deck 0.91 m, guardrail 1.10 m, toeboard 0.15 m, height stowed 2.11 m, length 1.83 m, width 0.813 m, wheelbase 1.32 m, ground clearance 0.06 m, solid non-marking tyres 30 x 11 cm, lift capacity 227 kg, raise / lower 16 / 29 s, 24 V (four 6 V 225 Ah batteries), weight 1,503 kg."}
AREAS = {"arms": 7.9, "chassis": 4.9, "battery-box": 0.52, "motor-pump": 0.55, "deck": 3.95, "rails": 6.0, "tyres": 0.97}


def text(en, de):
    return {"en": en, "de": de}


def bones(K):
    def mass(kg, x, y):
        return {"kg": kg, "comX": x, "comY": y, "distribution": "point", "source": "authored-estimate"}

    r = K.r5
    result = [
        {"id": "chassis", "parent": None, "title": text("Chassis with batteries, power unit and drive hubs", "Fahrgestell mit Batterien, Aggregat und Antriebsnaben"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 1.83, "joint": {"type": "fixed"}, "mass": mass(760, 0, 0.24)},
        {"id": "wheel-f", "parent": "chassis", "title": text("Front wheels", "Vorderräder"), "x": K.AXLE_X, "y": K.AXLE_Y, "angleDeg": 0, "lengthM": K.AXLE_Y, "joint": {"type": "revolute"}, "mass": mass(18, 0, 0)},
        {"id": "wheel-r", "parent": "chassis", "title": text("Rear wheels", "Hinterräder"), "x": -K.AXLE_X, "y": K.AXLE_Y, "angleDeg": 0, "lengthM": K.AXLE_Y, "joint": {"type": "revolute"}, "mass": mass(18, 0, 0)},
        {"id": "a1", "parent": "chassis", "title": text("Scissor arm A1 (outer, fixed foot)", "Scherenarm A1 (außen, fester Fuß)"), "x": K.XR, "y": K.BASE_Y, "angleDeg": r(K.THETA0), "lengthM": K.ARM, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 60}, "mass": mass(38, K.ARM / 2, 0)},
        {"id": "slide", "parent": "chassis", "title": text("Rolling foot of arm B1", "Rollenfuß des Arms B1"), "x": r(K.XF), "y": K.BASE_Y, "angleDeg": 0, "lengthM": 0.035, "joint": {"type": "prismatic", "axisDeg": 0, "minM": -0.82, "maxM": 0}, "mass": mass(6, 0, 0)},
        {"id": "b1", "parent": "slide", "title": text("Scissor arm B1 (inner, rolling foot)", "Scherenarm B1 (innen, Rollenfuß)"), "x": 0, "y": 0, "angleDeg": r(180 - K.THETA0), "lengthM": K.ARM, "joint": {"type": "revolute", "minDeg": -60, "maxDeg": 0}, "mass": mass(38, K.ARM / 2, 0)},
    ]
    for k in range(2, K.STAGES + 1):
        result.append({"id": f"a{k}", "parent": f"b{k - 1}", "title": text(f"Scissor arm A{k} (outer)", f"Scherenarm A{k} (außen)"), "x": K.ARM, "y": 0, "angleDeg": r(2 * K.THETA0 - 180), "lengthM": K.ARM, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 120}, "mass": mass(38, K.ARM / 2, 0)})
        result.append({"id": f"b{k}", "parent": f"a{k - 1}", "title": text(f"Scissor arm B{k} (inner)", f"Scherenarm B{k} (innen)"), "x": K.ARM, "y": 0, "angleDeg": r(180 - 2 * K.THETA0), "lengthM": K.ARM, "joint": {"type": "revolute", "minDeg": -120, "maxDeg": 0}, "mass": mass(38, K.ARM / 2, 0)})
    result += [
        {"id": "lift-cylinder", "parent": "b1", "title": text("Lift cylinder barrel", "Hubzylinderrohr"), "x": K.CYL_S1, "y": 0, "angleDeg": r(K.CYL_PHI0 - (180 - K.THETA0)), "lengthM": 0.47, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 130}, "mass": mass(28, 0.24, 0)},
        {"id": "lift-rod", "parent": "lift-cylinder", "title": text("Piston rod of the lift cylinder", "Kolbenstange des Hubzylinders"), "x": r(K.CYL_LEN0), "y": 0, "angleDeg": 0, "lengthM": 0.3, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.31}, "mass": mass(12, -0.25, 0)},
        {"id": "platform", "parent": f"b{K.STAGES}", "title": text("Work platform with guard rails", "Arbeitsbühne mit Geländer"), "x": K.ARM, "y": 0, "angleDeg": r(K.THETA0 - 180), "lengthM": 1.63, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 60}, "mass": mass(270, 0.7275, 0.33)},
        {"id": "deck-ext", "parent": "platform", "title": text("Slide-out deck with its rail frame", "Ausschubbühne mit Geländerrahmen"), "x": r(K.EXT[0] - K.XR), "y": 0, "angleDeg": 0, "lengthM": 0.92, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.91}, "mass": mass(87, -0.42, 0.45)},
    ]
    return result


def anchors(K, old):
    by = {anchor["id"]: copy.deepcopy(anchor) for anchor in old}
    place = {
        "contact-f-l": (K.AXLE_X, 0, K.AXLE_Z), "contact-f-r": (K.AXLE_X, 0, -K.AXLE_Z), "contact-r-l": (-K.AXLE_X, 0, K.AXLE_Z), "contact-r-r": (-K.AXLE_X, 0, -K.AXLE_Z),
        "motor-outlet": (0.915, 0.26, None), "battery-top": (-0.24, 0.3925, None),
        "occupant-1": (0.45, 0.1, None), "occupant-2": (1.15, 0.1, None), "tool-point": (0.8, 0.1, None),
    }
    result = []
    for anchor in old:
        entry = by[anchor["id"]]
        x, y, z = place[anchor["id"]]
        entry["x"], entry["y"] = x, y
        if z is not None:
            entry["z"] = z
        if anchor["id"].startswith("contact"):
            entry["footprintLengthM"], entry["footprintWidthM"] = 0.08, 0.09
        result.append(entry)
    return result


def poses(K, old):
    angles = {"stowed": (K.THETA0, None), "mid-raise": (K.THETA_MID, None), "raised-working": (K.THETA_MAX, 0.91), "lowering": (K.THETA_MID, None)}
    words = {
        "stowed": text("Scissors folded to a 0.44 m stack, platform floor at 1.00 m with its rails at 2.11 m, battery on standby.", "Schere zu einem 0,44 m hohen Paket zusammengefaltet, Bühnenboden bei 1,00 m mit Geländer bei 2,11 m, Batterie in Bereitschaft."),
        "mid-raise": text("Arm angle 30 degrees: platform floor at about 3.5 m, motor and pump at half speed under 220 kg.", "Armwinkel 30 Grad: Bühnenboden bei etwa 3,5 m, Motor und Pumpe mit halber Drehzahl unter 220 kg."),
        "raised-working": text("Arm angle 63 degrees: platform floor at 5.79 m, the deck extended by 0.91 m, two occupants and a tool box on board.", "Armwinkel 63 Grad: Bühnenboden bei 5,79 m, Bühne um 0,91 m ausgeschoben, zwei Personen und ein Werkzeugkasten an Bord."),
        "lowering": text("Key pose between raised and stowed: arm angle 30 degrees, deck retracted.", "Schlüsselpose zwischen gehoben und eingefahren: Armwinkel 30 Grad, Bühne eingefahren."),
    }
    result = []
    for pose in old:
        entry = copy.deepcopy(pose)
        theta, ext = angles[pose["id"]]
        entry["bones"] = K.offsets(theta, ext)
        entry["description"] = words[pose["id"]]
        result.append(entry)
    return result


def clips(K, old):
    """🎞️ The authored key times stay; between stowed and raised the linkage is sampled every half second on a smooth lift curve so that it stays closed in every frame."""
    result = []
    for clip in old:
        entry = copy.deepcopy(clip)
        keys = []
        previous = None
        for key in [authored for authored in clip["keys"] if authored["pose"] in ("stowed", "raised-working")]:
            raised = key["pose"] == "raised-working"
            if previous is not None and (previous["pose"] == "raised-working") != raised:
                t0, t1 = previous["t"], key["t"]
                steps = int(round((t1 - t0) / 0.5))
                for i in range(1, steps):
                    share = i / steps
                    eased = share * share * (3 - 2 * share)
                    lift = eased if raised else 1 - eased
                    theta = K.THETA0 + (K.THETA_MAX - K.THETA0) * lift
                    keys.append({"t": K.r5(t0 + (t1 - t0) * share), "pose": "mid-raise" if raised else "lowering", "ease": "smooth", "bones": K.offsets(theta)})
            ext = 0.91 if "deck-ext" in key.get("bones", {}) else None
            keys.append({"t": key["t"], "pose": key["pose"], "ease": key["ease"], "bones": K.offsets(K.THETA_MAX if raised else K.THETA0, ext)})
            previous = key
        entry["keys"] = keys
        result.append(entry)
    return result


def symbol(old):
    def rect(x, y, w, h):
        return {"kind": "rect", "x": x, "y": y, "w": w, "h": h}

    def disc(cx, cy, r):
        return {"kind": "ellipse", "cx": cx, "cy": cy, "rx": r, "ry": r}

    geometry = {
        "elevation-chassis": rect(-0.915, 0.06, 1.83, 0.34), "elevation-tyre-f": disc(0.66, 0.1525, 0.1525), "elevation-tyre-r": disc(-0.66, 0.1525, 0.1525), "elevation-stack": rect(-0.7725, 0.4, 1.545, 0.46),
        "elevation-deck": rect(-0.815, 0.86, 1.63, 0.14), "elevation-toe-board": rect(-0.815, 1, 1.63, 0.15), "elevation-mid-rail": rect(-0.815, 1.515, 1.63, 0.07), "elevation-top-rail": rect(-0.815, 2.04, 1.63, 0.07),
        "elevation-post-r": rect(-0.815, 1, 0.07, 1.11), "elevation-post-f": rect(0.745, 1, 0.07, 1.11),
        "front-chassis": rect(-0.395, 0.06, 0.79, 0.34), "front-tyre-l": rect(0.295, 0, 0.11, 0.305), "front-tyre-r": rect(-0.405, 0, 0.11, 0.305), "front-stack": rect(-0.32, 0.4, 0.64, 0.46),
        "front-deck": rect(-0.37, 0.86, 0.74, 0.14), "front-toe-board": rect(-0.37, 1, 0.74, 0.15), "front-mid-rail": rect(-0.37, 1.515, 0.74, 0.07), "front-top-rail": rect(-0.37, 2.04, 0.74, 0.07),
        "front-post-l": rect(0.3, 1, 0.07, 1.11), "front-post-r": rect(-0.37, 1, 0.07, 1.11),
        "plan-chassis": rect(-0.915, -0.395, 1.83, 0.79), "plan-tyre-fl": rect(0.5075, 0.295, 0.305, 0.11), "plan-tyre-fr": rect(0.5075, -0.405, 0.305, 0.11), "plan-tyre-rl": rect(-0.8125, 0.295, 0.305, 0.11), "plan-tyre-rr": rect(-0.8125, -0.405, 0.305, 0.11),
        "plan-deck": rect(-0.815, -0.37, 1.63, 0.74), "plan-rail-l": rect(-0.815, 0.3, 1.63, 0.07), "plan-rail-r": rect(-0.815, -0.37, 1.63, 0.07), "plan-rail-f": rect(0.745, -0.37, 0.07, 0.74), "plan-rail-b": rect(-0.815, -0.37, 0.07, 0.74),
    }
    result = copy.deepcopy(old)
    for view, shapes in result["geometry"].items():
        for i, shape in enumerate(shapes):
            kept = {key: value for key, value in shape.items() if key not in ("kind", "x", "y", "w", "h", "cx", "cy", "rx", "ry")}
            shapes[i] = {**geometry[shape["id"]], **kept}
    return result


def apply(asset, K, texts):
    asset["summary"] = text(
        "A battery-electric 19 ft slab scissor lift of the Genie GS-1932 class in side elevation: four scissor stages built from eight tube arms that fold to a 0.44 m stack or rise to a 5.79 m platform, a lift cylinder between the first two stages, a 0.91 m slide-out deck, folding guard rails, and two occupants with a tool box as loads.",
        "Eine batterieelektrische 19-ft-Scherenarbeitsbühne der Klasse Genie GS-1932 in der Seitenansicht: vier Scherenstufen aus acht Rohrarmen, die sich zu einem 0,44 m hohen Paket falten oder zu einer 5,79 m hohen Plattform heben, ein Hubzylinder zwischen den ersten beiden Stufen, eine 0,91 m ausschiebbare Bühnenverlängerung, klappbares Geländer sowie zwei Personen mit Werkzeugkasten als Lasten.")
    references = [reference for reference in asset["provenance"]["references"] if reference["id"] != REFERENCE["id"]]
    asset["provenance"]["references"] = references + [REFERENCE]
    asset["provenance"]["notes"] = text(
        "Dimensions after the Genie GS-1932 data sheet (1.83 x 0.81 m, 1,503 kg, 5.79 m platform height, 1.00 m stowed floor, 2.11 m stowed height, wheelbase 1.32 m, tyres 305 x 110 mm, 227 kg platform load, 1.1 m guard rail per EN 280). Arm length 1.46 m between pins, tube 90 x 50 mm, stage pitch 0.11 m and the cylinder mounting are authored estimates that reproduce the published stowed and raised heights. The scissor stack is a closed linkage built as a tree: arms A_k hang from the end of B_(k-1) and B_k from the end of A_(k-1), the rolling foot and the piston rod are prismatic bones, and the platform is kept level by the pose offsets. 1:500 is unavailable.",
        "Maße nach dem Datenblatt Genie GS-1932 (1,83 x 0,81 m, 1.503 kg, 5,79 m Plattformhöhe, Boden eingefahren 1,00 m, Höhe eingefahren 2,11 m, Radstand 1,32 m, Reifen 305 x 110 mm, 227 kg Plattformlast, 1,1 m Geländer nach EN 280). Armlänge 1,46 m zwischen den Bolzen, Rohr 90 x 50 mm, Stufenabstand 0,11 m und die Zylinderanlenkung sind Schätzungen, die die veröffentlichten Höhen eingefahren und gehoben wiedergeben. Das Scherenpaket ist ein geschlossenes Gestänge, als Baum aufgebaut: Arme A_k hängen am Ende von B_(k-1) und B_k am Ende von A_(k-1), Rollenfuß und Kolbenstange sind Schubknochen, und die Bühne bleibt durch die Posen-Offsets waagerecht. 1:500 ist nicht verfügbar.")
    asset["dimensions"] = {**asset["dimensions"], "heightM": 2.11, "widthM": 1.83, "depthM": 0.81, "massKg": 1503}
    asset["rig"]["bones"] = bones(K)
    asset["anchors"] = anchors(K, asset["anchors"])
    asset["poses"] = poses(K, asset["poses"])
    asset["clips"] = clips(K, asset["clips"])
    asset["symbols"] = [symbol(entry) for entry in asset["symbols"]]
    for region in asset["thermal"]["regions"]:
        region["areaM2"] = AREAS[region["id"]]
        if region["id"] == "arms":
            region["characteristicLengthM"] = K.ARM
        if region["id"] == "tyres":
            region["characteristicLengthM"] = 0.305
    for scale, shapes in texts["shapes"].items():
        asset["representations"][scale]["geometry"] = shapes
        asset["representations"][scale]["decisions"] = texts["decisions"][scale]
    asset["representations"]["500"].update(texts["unavailable"])
    return asset
