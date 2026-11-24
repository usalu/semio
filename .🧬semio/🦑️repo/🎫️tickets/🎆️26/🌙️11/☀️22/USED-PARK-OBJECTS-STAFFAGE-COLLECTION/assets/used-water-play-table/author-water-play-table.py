"""💦 Authoring script of `used-water-play-table` (run it to regenerate `🔣️.json`): rig, anchors, poses, clips, thermal and structural model, traces of use and the four authored scales of a used sand-water play table with an Archimedes screw.

@see ./author_geometry.py — the hand-chosen numbers
@see ./author_scales.py — the parts of every scale
@see ./author_physics.py — masses, materials, sections, references
@see ./author_texts.py — the decisions of every scale
"""
import json
import math
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))

from author_geometry import (
    APRON_X, APRON_Y, AXIS_DEG, POST_Z, BONES, BS_CX, BS_FLOOR, BS_WATER, DRAIN, FOOT_R, FOOT_Y0, GATE_OPEN, GATE_X, GRIP_UV, MUD, P, PLINTH, POOL_IDS, POSTS,
    POST_R, POST_Y1, SHAFT, SWING, TR_FLOOR, TR_TOP, TUBE, WATER_D, grip_top, local, r5,
)
from author_parts import FEATURE, Scale, box, ell, path, poly, rev, shape, tx
from author_physics import REFERENCES, materials, masses, sections
from author_scales import CONC, PE, STEEL, WATER, TIMBER_G, basin, bracket_and_chute, gates, mud, posts_and_footings, screw_assembly, table_frame, trough, water
from author_texts import PENS, BANDS, rep_decisions, vol_decisions
from author_thermal import area_estimates, binding, regions

HERE = pathlib.Path(__file__).parent
G = 9.80665


def de(value, digits):
    """🇩🇪 A number in German notation with `digits` decimals."""
    return f"{value:.{digits}f}".replace(".", ",")


def build(s):
    """🏗️ All shapes and solids of scale 1:s."""
    k = Scale(s)
    posts_and_footings(k)
    table_frame(k)
    trough(k)
    gates(k)
    water(k)
    basin(k)
    mud(k)
    bracket_and_chute(k)
    screw_assembly(k)
    return k


def traces(k):
    """🩹 Traces of use and weather as detail and texture strokes: drying checks and graffiti on the greyed aprons, screw heads of the replaced board, limescale tide lines in the pools and on the chute, chewing gum on the flange, scratches on the clouded tube and moss at the shaded post feet."""
    s = k.s
    if s > 100:
        return
    tg = ("timber-greyed", TIMBER_G)
    k.elev.append(shape("apron-checks", "frame", "texture", 3.1, False, 0.4, path("M -0.74 0.62 L -0.42 0.623 M -0.30 0.50 L 0.05 0.497 M 0.22 0.64 L 0.61 0.642 M -0.6 0.47 L -0.48 0.471"), *tg))
    k.elev.append(shape("apron-wear", "frame", "detail", 3.1, False, 0.4, poly([(-0.6, 0.672), (0.6, 0.672)], False), *tg))
    if s == 50:
        k.elev.append(shape("graffiti", "frame", "texture", 3.15, False, 0.3, path("M 0.08 0.52 C 0.11 0.6 0.14 0.48 0.17 0.56 C 0.2 0.62 0.23 0.5 0.27 0.58 M 0.12 0.5 L 0.25 0.505"), *tg))
        k.elev.append(shape("sticker", "frame", "texture", 3.15, False, 0.6, path("M -0.18 0.58 L -0.12 0.58 L -0.12 0.62 L -0.18 0.62 Z"), *tg))
        for bone in ("post-rr", "post-fr"):
            k.elev.append(shape("post-checks-" + bone, bone, "texture", 1.7, False, 0.35, path("M 0.66 0.01 L 0.95 0.012 M 0.72 -0.02 L 0.9 -0.021"), "posts-robinia", "robinia-weathered"))
            k.elev.append(shape("moss-" + bone, bone, "texture", 1.7, False, 0.3, path("M 0.61 -0.05 L 0.64 -0.04 M 0.61 0.0 L 0.65 0.01 M 0.62 0.04 L 0.64 0.05"), "posts-robinia", "robinia-weathered"))
        k.front.append(shape("head-board-checks", "frame", "texture", 3.2, False, 0.4, path("M -0.2 0.62 L 0.05 0.618 M -0.1 0.5 L 0.18 0.502"), *tg))
        k.front.append(shape("head-board-moss", "frame", "texture", 3.2, False, 0.3, path("M 0.06 0.45 L 0.12 0.455 M 0.15 0.448 L 0.22 0.452 M 0.18 0.46 L 0.24 0.465"), *tg))
        k.plan.append(shape("replaced-screws", "frame", "detail", 2.65, False, 0.5, path("M -0.805 -0.2 L -0.795 -0.2 M -0.805 0.2 L -0.795 0.2 M -0.805 0.0 L -0.795 0.0"), "timber-fresh", "douglas-fresh"))
        k.plan.append(shape("gum-spots", "frame", "texture", 3.0, False, 0.2, path("M -0.31 0.262 L -0.305 0.262 M 0.12 -0.27 L 0.125 -0.27 M 0.47 0.258 L 0.475 0.258 M -0.55 -0.262 L -0.545 -0.262"), "trough-steel", STEEL))
    for pid in POOL_IDS:
        x0 = BONES[pid]["origin"][0]
        k.plan.append(shape("scale-line-" + pid, pid, "texture", 2.95, False, 0.95, path(f"M {r5(x0 - 0.22)} 0.235 L {r5(x0 + 0.22)} 0.235 M {r5(x0 - 0.22)} -0.235 L {r5(x0 + 0.22)} -0.235"), "water-pools", WATER))
    k.elev.append(shape("chute-scale", "frame", "texture", 3.95, False, 0.95, path("M 0.71 0.705 L 0.86 0.735"), "screw-steel", STEEL))
    k.elev.append(shape("tube-scratches", "screw-mount", "texture", 3.65, False, 0.6, path("M 0.5 0.06 L 0.62 0.05 M 0.66 -0.04 L 0.8 -0.05 M 0.3 0.065 L 0.38 0.06"), "tube-pc", "polycarbonate-clouded"))


def structure_extras(k, total):
    """🏗️ Mass markers at the centres of mass of every bone and the support blocks of scale k at the footing bases and under the plinth."""
    s, f = k.s, FEATURE[k.s]
    mr = {50: 0.02, 100: 0.025, 200: 0.03, 500: 0.04}[s]
    mats = {"frame": TIMBER_G, "grip": PE, "screw-mount": STEEL, "screw": STEEL, "gate-1": STEEL, "gate-2": STEEL, "basin-water": WATER}
    for bone, (m, cx, cy) in total.items():
        u, v = local(bone, cx, cy)
        r = mr
        if bone in POOL_IDS:
            r = min(mr, WATER_D / 2 - 0.001) if s < 500 else 0.0375
        if bone == "grip":
            r = min(mr, 0.016) if s < 500 else 0.0375
        k.structure.append(ell("mass-" + bone, bone, mats.get(bone, WATER if bone in POOL_IDS else CONC), [r, r, r], [u, v, 0], member=bone, part="mass"))
    for bone in POSTS:
        k.structure.append(box("support-" + bone, bone, CONC, "detail", 0.1, [f, 2 * FOOT_R, 2 * FOOT_R], [f / 2, 0, 0], member=bone, part="support", anchor="footing-" + bone))
    x0, x1, y0, y1, zp = PLINTH
    k.structure.append(box("support-plinth", "basin", CONC, "detail", 0.1, [x1 - x0, f, 2 * zp], [(x0 + x1) / 2 - BS_CX, y0 + f / 2, 0], member="basin", part="support", anchor="plinth-base"))


def rig(total):
    """🦴 The mechanical rig: the fixed frame, four posts, two sliding sluice gates, three pool slabs and the basin water that drain downwards, the swinging screw mount, the continuously turning screw and its grip."""
    sec = sections()

    def mass(bone, distribution="point"):
        m, cx, cy = total[bone]
        u, v = local(bone, cx, cy)
        src = "literature" if bone not in ("grip",) else "authored-estimate"
        ref = {"frame": "en-338", "grip": "iso-10456-2007", "screw-mount": "iso-10456-2007", "screw": "en-iso-1127"}.get(bone, "en-1991-1-1")
        return {"kg": round(m, 3), "comX": u, "comY": v, "distribution": distribution, "source": src, "reference": ref}

    bones = [
        {"id": "frame", "parent": None, "title": tx("Table frame: aprons, bearers, deck, stainless channel, bracket and chute; at the channel axis on the sand", "Tischrahmen: Zargen, Riegel, Boden, Edelstahlrinne, Halter und Rutsche; auf der Rinnenachse im Sand"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": APRON_X, "joint": {"type": "fixed"}, "mass": mass("frame"), "section": sec["frame"]},
    ]
    bones.append({"id": "basin", "parent": "frame", "title": tx("Lower basin on its precast plinth, bedded in the sand", "Unteres Becken auf seinem Fertigteilsockel, im Sand gebettet"), "x": BS_CX, "y": 0, "angleDeg": 0, "lengthM": 0.05, "joint": {"type": "fixed"}, "mass": mass("basin")})
    for bone in POSTS:
        ox, oy = BONES[bone]["origin"]
        side = {"rl": ("outlet end, left", "Auslaufseite, links"), "rr": ("outlet end, right", "Auslaufseite, rechts"), "fl": ("screw end, left", "Schneckenseite, links"), "fr": ("screw end, right", "Schneckenseite, rechts")}[bone[5:]]
        bones.append({"id": bone, "parent": "frame", "title": tx(f"Robinia post in its point footing, {side[0]}, from the footing base up to the bearer", f"Robinienpfosten im Punktfundament, {side[1]}, von der Fundamentsohle bis zum Riegel"), "x": ox, "y": oy, "z": BONES[bone]["z"], "angleDeg": 90, "lengthM": round(POST_Y1 - FOOT_Y0, 4), "joint": {"type": "fixed"}, "mass": mass(bone, "point"), "section": {**sec["post"]}})
    for gid, xg in GATE_X.items():
        bones.append({"id": gid, "parent": "frame", "title": tx(f"Sluice gate {gid[-1]}, sliding up in its guides from the channel floor", f"Schieber {gid[-1]}, in seinen Führungen vom Rinnenboden aufwärts gleitend"), "x": xg, "y": TR_FLOOR, "angleDeg": 90, "lengthM": 0.06, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": GATE_OPEN}, "mass": mass(gid)})
    for pid in POOL_IDS:
        ox, oy = BONES[pid]["origin"]
        n = pid[-1]
        bones.append({"id": pid, "parent": "frame", "title": tx(f"Standing water of pool {n} on the channel floor; drained, it sinks into the deck below", f"Stehendes Wasser von Becken {n} auf dem Rinnenboden; abgelassen sinkt es in den Boden darunter"), "x": r5(ox), "y": oy, "angleDeg": 0, "lengthM": 0.03, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -DRAIN, "maxM": 0}, "mass": mass(pid)})
    bones.append({"id": "basin-water", "parent": "frame", "title": tx("Water of the lower basin; drained, it sinks into the plinth below", "Wasser des unteren Beckens; abgelassen sinkt es in den Sockel darunter"), "x": BS_CX, "y": BS_FLOOR, "angleDeg": 0, "lengthM": 0.05, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -(BS_WATER + 0.002), "maxM": 0}, "mass": mass("basin-water")})
    bones.append({"id": "screw-mount", "parent": "frame", "title": tx("Screw tube on its top pivot pins, down the screw axis to the foot bearing in the basin", "Schneckenrohr an den oberen Gelenkbolzen, entlang der Schneckenachse bis zum Fußlager im Becken"), "x": P[0], "y": P[1], "angleDeg": AXIS_DEG, "lengthM": TUBE[1], "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 50}, "mass": mass("screw-mount", "uniform"), "section": sec["screw-mount"]})
    bones.append({"id": "screw", "parent": "screw-mount", "title": tx("Screw shaft with its flight and crank, turning about the screw axis", "Schneckenwelle mit Wendel und Kurbel, um die Schneckenachse drehend"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": SHAFT[1], "joint": {"type": "revolute", "continuousTwist": True}, "mass": mass("screw", "uniform"), "section": sec["screw"]})
    bones.append({"id": "grip", "parent": "screw", "title": tx("Crank grip, from the crank arm outwards", "Kurbelgriff, vom Kurbelarm nach außen"), "x": GRIP_UV[0], "y": GRIP_UV[1], "angleDeg": 180, "lengthM": 0.11, "joint": {"type": "fixed"}, "mass": mass("grip")})
    return {"kind": "mechanical", "root": "frame", "bones": bones}


def anchors():
    """📍 The fixed supports at the footing bases, the soil contact under the plinth and the load points of the poses."""
    out = []
    for bone in POSTS:
        out.append({"id": "footing-" + bone, "bone": bone, "x": 0, "y": 0, "kind": "support", "title": tx("Footing base held by the soil (fixed)", "Fundamentsohle im Boden gehalten (eingespannt)"), "support": "fixed", "footprintLengthM": 0.3, "footprintWidthM": 0.3})
    out.append({"id": "plinth-base", "bone": "basin", "x": 0, "y": PLINTH[2], "kind": "support", "title": tx("Plinth bedded in the sand (soil contact)", "Sockel im Sand gebettet (Bodenkontakt)"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.52, "footprintWidthM": 0.52, "frictionCoefficient": 0.6})
    out += [
        {"id": "crank-grip", "bone": "grip", "x": 0.06, "y": 0, "kind": "load", "title": tx("A child's hand on the crank grip", "Kinderhand am Kurbelgriff")},
        {"id": "lean-right", "bone": "frame", "x": 0.35, "y": APRON_Y[1], "z": -0.27, "kind": "load", "title": tx("A child leaning on the right apron", "Ein Kind lehnt an der rechten Zarge")},
        {"id": "lean-left", "bone": "frame", "x": -0.35, "y": APRON_Y[1], "z": 0.27, "kind": "load", "title": tx("A child leaning on the left apron", "Ein Kind lehnt an der linken Zarge")},
        {"id": "screw-hand", "bone": "screw-mount", "x": 0.84, "y": 0, "kind": "load", "title": tx("The depot worker's hand under the lower end of the screw tube", "Die Hand des Bauhofmitarbeiters unter dem unteren Ende des Schneckenrohrs")},
        {"id": "snow-table", "bone": "frame", "x": 0, "y": TR_TOP, "kind": "load", "title": tx("Centre of the table top under snow", "Mitte der Tischfläche unter Schnee")},
        {"id": "snow-basin", "bone": "basin", "x": 0, "y": 0.3415, "kind": "load", "title": tx("Centre of the basin under snow", "Mitte des Beckens unter Schnee")},
    ]
    return out


def poses(total):
    """🧍 In service with standing water, in use with a child cranking, the depot's maintenance with the screw swung out, and drained for winter under snow."""
    base = {"ground": "authored", "contacts": ["footing-" + b for b in POSTS] + ["plinth-base"]}
    m_screw = sum(total[b][0] for b in ("screw-mount", "screw", "grip"))
    cx = sum(total[b][0] * total[b][1] for b in ("screw-mount", "screw", "grip")) / m_screw
    lever = cx - P[0]
    hand_x = 0.84 * math.cos(math.radians(AXIS_DEG + SWING))
    hand = round(m_screw * G * (lever * math.cos(math.radians(AXIS_DEG + SWING)) / math.cos(math.radians(AXIS_DEG))) / hand_x, 1)
    use_loads = [
        {"id": "crank-turn", "kind": "occupant", "title": tx("A child turning the crank (0.08 kN at the grip, DIN 33402-2 child of 20 kg)", "Ein Kind dreht die Kurbel (0,08 kN am Griff, Kind von 20 kg nach DIN 33402-2)"), "anchor": "crank-grip", "fxN": 40, "fyN": -70, "source": "authored-estimate", "reference": "din-33402-2"},
        {"id": "child-leaning-right", "kind": "occupant", "title": tx("A child leaning on the right apron (0.2 kN down, 0.05 kN sideways)", "Ein Kind lehnt an der rechten Zarge (0,2 kN abwärts, 0,05 kN seitlich)"), "anchor": "lean-right", "fxN": 0, "fyN": -200, "fzN": 50, "source": "authored-estimate", "reference": "din-en-1176-1"},
        {"id": "child-leaning-left", "kind": "occupant", "title": tx("A child kneeling on the left apron (0.2 kN down, 0.05 kN sideways)", "Ein Kind kniet an der linken Zarge (0,2 kN abwärts, 0,05 kN seitlich)"), "anchor": "lean-left", "fxN": 0, "fyN": -200, "fzN": -50, "source": "authored-estimate", "reference": "din-en-1176-1"},
    ]
    hand_load = [{"id": "worker-hand", "kind": "carried", "title": tx(f"The depot worker holds the swung-out screw at its lower end ({hand / 1000:.3f} kN up, the rest hangs on the pivot pins)", f"Der Bauhofmitarbeiter hält die ausgeschwenkte Schnecke am unteren Ende ({de(hand / 1000, 3)} kN aufwärts, der Rest hängt an den Gelenkbolzen)"), "anchor": "screw-hand", "fxN": 0, "fyN": hand, "source": "authored-estimate", "reference": "din-en-1176-7"}]
    snow = [
        {"id": "snow-on-table", "kind": "snow", "title": tx("Snow on the table, 0.8 x 0.85 kN/m2 over 1.64 x 0.58 m", "Schnee auf dem Tisch, 0,8 x 0,85 kN/m² über 1,64 x 0,58 m"), "anchor": "snow-table", "fxN": 0, "fyN": -round(0.8 * 850 * 1.64 * 0.58), "source": "literature", "reference": "en-1991-1-3"},
        {"id": "snow-on-basin", "kind": "snow", "title": tx("Snow on the basin, 0.8 x 0.85 kN/m2 over 0.49 x 0.49 m", "Schnee auf dem Becken, 0,8 x 0,85 kN/m² über 0,49 x 0,49 m"), "anchor": "snow-basin", "fxN": 0, "fyN": -round(0.8 * 850 * 0.49 * 0.49), "source": "literature", "reference": "en-1991-1-3"},
    ]
    drained = {pid: {"dy": -DRAIN} for pid in POOL_IDS}
    gates_up = {g: {"dy": GATE_OPEN} for g in GATE_X}
    return [
        {"id": "in-service", "title": tx("In service", "Im Einsatz"), "description": tx("A summer evening in the sand play area: the three pools hold water behind the closed sluices, the basin is filled from the pump, the screw rests in its foot bearing; the posts stand in their point footings, the plinth is bedded in the sand.", "Ein Sommerabend im Sandspielbereich: die drei Becken halten Wasser hinter den geschlossenen Schiebern, das untere Becken ist von der Pumpe gefüllt, die Schnecke ruht im Fußlager; die Pfosten stehen in ihren Punktfundamenten, der Sockel ist im Sand gebettet."), "bones": {}, **base, "thermal": binding("in-service"), "loads": []},
        {"id": "in-use", "title": tx("In use", "In Benutzung"), "description": tx("A child turns the crank and lifts water from the basin into pool 1, the first sluice is pulled up 60 mm and the water runs on, two more children lean and kneel at the aprons with their sand moulds.", "Ein Kind dreht die Kurbel und hebt Wasser aus dem Becken in Becken 1, der erste Schieber ist 60 mm hochgezogen und das Wasser läuft weiter, zwei weitere Kinder lehnen und knien mit ihren Sandförmchen an den Zargen."), "bones": {"screw": {"twistDeg": 120}, "gate-1": {"dy": 0.06}, "basin-water": {"dy": -0.03}}, **base, "thermal": binding("in-use"), "loads": use_loads},
        {"id": "maintenance", "title": tx("Maintenance by the depot", "Wartung durch den Bauhof"), "description": tx(f"The depot drains the play table for descaling: both sluices pulled to their stops, the pools and the basin empty, the screw swung up {SWING:.0f} degrees about its top pivot pins out of the basin and held at its lower end while the tube is rinsed and the foot bearing checked.", f"Der Bauhof lässt den Spieltisch zum Entkalken ab: beide Schieber bis zum Anschlag gezogen, Becken und Unterbecken leer, die Schnecke um {SWING:.0f} Grad an ihren oberen Gelenkbolzen aus dem Becken geschwenkt und am unteren Ende gehalten, während das Rohr gespült und das Fußlager geprüft wird."), "bones": {**drained, **gates_up, "basin-water": {"dy": -(BS_WATER + 0.002)}, "screw-mount": {"angleDeg": SWING}}, **base, "thermal": binding("maintenance"), "loads": hand_load},
        {"id": "winter-drained", "title": tx("Drained for winter", "Für den Winter entleert"), "description": tx("From October to April the feed is shut, the pools and the basin are drained so that no ice can burst the welds, both sluices stand open and snow lies on the table and the basin.", "Von Oktober bis April ist der Zulauf abgesperrt, Becken und Unterbecken sind entleert, damit kein Eis die Nähte sprengt, beide Schieber stehen offen und Schnee liegt auf Tisch und Becken."), "bones": {**drained, **gates_up, "basin-water": {"dy": -(BS_WATER + 0.002)}}, **base, "thermal": binding("winter-drained"), "loads": snow},
    ]


def clips():
    """🎞 The depot's draining and swing-out, and a child playing the water from the basin through the three pools into the sand."""
    contacts = [{"anchor": a, "fromS": 0, "toS": 10} for a in ["footing-" + b for b in POSTS] + ["plinth-base"]]
    play_contacts = [{"anchor": a, "fromS": 0, "toS": 8} for a in ["footing-" + b for b in POSTS] + ["plinth-base"]]
    d = -DRAIN
    play_keys = [
        {"t": 0, "pose": "in-service", "ease": "linear", "bones": {"screw": {"twistDeg": 0}, "pool-1": {"dy": d}, "pool-2": {"dy": d}, "pool-3": {"dy": d}}},
        {"t": 2.5, "pose": "in-service", "ease": "linear", "bones": {"screw": {"twistDeg": 450}, "pool-1": {"dy": 0}, "pool-2": {"dy": d}, "pool-3": {"dy": d}, "basin-water": {"dy": -0.012}}},
        {"t": 3.5, "pose": "in-service", "ease": "linear", "bones": {"screw": {"twistDeg": 630}, "gate-1": {"dy": 0.06}, "pool-1": {"dy": 0}, "pool-2": {"dy": d}, "pool-3": {"dy": d}, "basin-water": {"dy": -0.017}}},
        {"t": 5.5, "pose": "in-service", "ease": "linear", "bones": {"screw": {"twistDeg": 990}, "gate-1": {"dy": 0.06}, "pool-1": {"dy": 0}, "pool-2": {"dy": 0}, "pool-3": {"dy": d}, "basin-water": {"dy": -0.026}}},
        {"t": 6.5, "pose": "in-service", "ease": "linear", "bones": {"screw": {"twistDeg": 1170}, "gate-1": {"dy": 0.06}, "gate-2": {"dy": 0.06}, "pool-1": {"dy": 0}, "pool-2": {"dy": 0}, "pool-3": {"dy": d}, "basin-water": {"dy": -0.03}}},
        {"t": 8, "pose": "in-service", "ease": "linear", "bones": {"screw": {"twistDeg": 1440}, "gate-1": {"dy": 0.06}, "gate-2": {"dy": 0.06}, "pool-1": {"dy": 0}, "pool-2": {"dy": 0}, "pool-3": {"dy": 0}, "basin-water": {"dy": -0.03}}},
    ]
    drained = {pid: {"dy": d} for pid in POOL_IDS}
    gates_up = {g: {"dy": GATE_OPEN} for g in GATE_X}
    return [
        {"id": "maintenance", "title": tx("Draining and swinging out the screw", "Ablassen und Ausschwenken der Schnecke"), "description": tx("The depot worker pulls both sluices, the pools run dry and the basin is emptied; then he lifts the screw out of the basin on its top pivot pins and holds it up for rinsing.", "Der Bauhofmitarbeiter zieht beide Schieber, die Becken laufen leer und das Unterbecken wird geleert; dann hebt er die Schnecke an ihren oberen Gelenkbolzen aus dem Becken und hält sie zum Spülen hoch."), "durationS": 10, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "in-service", "ease": "smooth"}, {"t": 3.5, "pose": "in-service", "ease": "smooth", "bones": {**drained, **gates_up, "basin-water": {"dy": -(BS_WATER + 0.002)}}}, {"t": 7, "pose": "maintenance", "ease": "smooth"}, {"t": 10, "pose": "maintenance", "ease": "smooth"}],
         "contacts": contacts,
         "phases": [{"id": "drain", "fromS": 0, "toS": 3.5, "title": tx("Pulling the sluices and draining", "Schieber ziehen und ablassen")}, {"id": "swing-out", "fromS": 3.5, "toS": 7, "title": tx("Swinging the screw out", "Schnecke ausschwenken")}, {"id": "rinse", "fromS": 7, "toS": 10, "title": tx("Holding it up for rinsing", "Zum Spülen hochhalten")}],
         "events": [{"t": 0.3, "kind": "motion", "title": tx("Sluices pulled", "Schieber gezogen")}, {"t": 3.5, "kind": "motion", "title": tx("Pools and basin empty", "Becken und Unterbecken leer")}, {"t": 7, "kind": "load-on", "title": tx("Screw held out of the basin", "Schnecke aus dem Becken gehalten"), "target": "worker-hand"}],
         "tracks": [{"target": "load:worker-hand", "ease": "smooth", "keys": [{"t": 3.5, "v": 0}, {"t": 7, "v": 1}]}, {"target": "region:water-pools:exposedFraction", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": 3.5, "v": 0.05}]}]},
        {"id": "water-play", "title": tx("Water play", "Wasserspiel"), "description": tx("A child cranks the screw at half a turn a second; water rises through the clear tube, pours down the chute and fills pool 1; the first sluice is pulled and pool 2 fills, then the second, and pool 3 spills through the spout into the sand hollow.", "Ein Kind kurbelt die Schnecke mit einer halben Umdrehung je Sekunde; Wasser steigt im klaren Rohr, läuft über die Rutsche und füllt Becken 1; der erste Schieber wird gezogen und Becken 2 füllt sich, dann der zweite, und Becken 3 läuft durch den Auslauf in die Sandmulde über."), "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": play_keys, "contacts": play_contacts,
         "phases": [{"id": "lift", "fromS": 0, "toS": 2.5, "title": tx("Lifting into pool 1", "Heben in Becken 1")}, {"id": "second-pool", "fromS": 2.5, "toS": 5.5, "title": tx("First sluice open, pool 2 fills", "Erster Schieber offen, Becken 2 füllt sich")}, {"id": "spill", "fromS": 5.5, "toS": 8, "title": tx("Second sluice open, spilling into the sand", "Zweiter Schieber offen, Überlauf in den Sand")}],
         "events": [{"t": 0, "kind": "load-on", "title": tx("Child starts cranking", "Kind beginnt zu kurbeln"), "target": "crank-turn"}, {"t": 3.5, "kind": "motion", "title": tx("First sluice pulled", "Erster Schieber gezogen")}, {"t": 6.5, "kind": "motion", "title": tx("Second sluice pulled", "Zweiter Schieber gezogen")}],
         "tracks": [{"target": "load:crank-turn", "ease": "smooth", "keys": [{"t": 0, "v": 1}, {"t": 8, "v": 1}]}, {"target": "region:wet-sand:wettedness", "ease": "smooth", "keys": [{"t": 6.5, "v": 0.7}, {"t": 8, "v": 1}]}]},
    ]


def document():
    """📄 The whole asset document."""
    parts, total = masses()
    area = area_estimates()
    reps, vols = {}, {}
    for s in (50, 100, 200, 500):
        k = build(s)
        traces(k)
        structure_extras(k, total)
        hatches = [{"id": "concrete", "title": tx("Concrete stipple", "Betonpunktierung"), "pattern": "dots", "angleDeg": 0, "spacingMm": 0.8 if s == 50 else 0.6, "penMm": 0.13}, {"id": "timber", "title": tx("Timber grain", "Holzmaserung"), "pattern": "lines", "angleDeg": 0, "spacingMm": 1.2 if s == 50 else 0.9, "penMm": 0.13}] if s <= 100 else []
        reps[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": BANDS[s][0], "maxDenominator": BANDS[s][1]}, "pens": PENS[s], "hatches": hatches, "geometry": {"elevation": k.elev, "front": k.front, "plan": k.plan}, "decisions": rep_decisions(s)}
        vols[str(s)] = {"status": "authored", "denominator": s, "body": k.body, "thermal": k.thermal, "structure": k.structure, "decisions": vol_decisions(s)}
    mass_total = round(sum(t[0] for t in total.values()), 2)
    x_min = MUD[0] - MUD[2]
    x_max = PLINTH[1]
    asset = {
        "id": "used-water-play-table",
        "category": "objects",
        "title": tx("Used Water Play Table", "Gebrauchter Wasserspieltisch"),
        "summary": tx("A sand-water play table of the municipal playground type after many summers: a 1.55 m stainless channel at 0.68 m working height on greyed Douglas aprons and four Robinia posts in concrete point footings, two slide sluices, a clear polycarbonate Archimedes screw with a hand crank lifting water from a lower basin on its plinth, limescale tide lines in the pools, silted wet sand at the outlet, a replaced end board in fresh wood and a splash hollow in the sand under the spout; drained by the depot every winter.", "Ein Sand-Wasser-Spieltisch der kommunalen Spielplatzbauart nach vielen Sommern: eine 1,55 m lange Edelstahlrinne auf 0,68 m Arbeitshöhe auf vergrauten Douglasienzargen und vier Robinienpfosten in Betonpunktfundamenten, zwei Schieber, eine klare Archimedische Schnecke aus Polycarbonat mit Handkurbel, die Wasser aus einem unteren Becken auf seinem Sockel hebt, Kalkränder in den Becken, eingeschwemmter nasser Sand am Auslauf, ein in frischem Holz ersetztes Stirnbrett und eine Spritzmulde im Sand unter dem Auslauf; jeden Winter vom Bauhof entleert."),
        "tags": ["used", "park", "public-space", "play", "water", "water-play", "sand-play", "archimedes-screw", "sluice", "stainless-steel", "timber", "robinia", "douglas-fir", "polycarbonate", "concrete-footing", "playground"],
        "provenance": {"author": "Semio staffage content (used park objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES], "notes": tx(f"Catalogue-typical sand-water play table: stainless 1.4301 channel 1.55 x 0.49 x 0.14 m in 1.5 mm sheet at a rim height of 0.68 m, two 3 mm slide sluices, Douglas C24 aprons 40 x 240 mm, Robinia posts 120 mm set 0.5 m into C20/25 point footings 0.30 x 0.60 m, a polycarbonate screw tube 160 x 5 mm, 0.88 m long at 45 degrees with a stainless shaft 42.4 x 2 and a 2 mm flight of 160 mm pitch, a lower basin 0.42 x 0.42 x 0.19 m on a precast plinth. Masses from section x density: frame {total['frame'][0]:.1f} kg, each post with footing {total['post-rl'][0]:.1f} kg, screw tube {total['screw-mount'][0]:.2f} kg, screw {total['screw'][0]:.2f} kg, water {sum(total[p][0] for p in POOL_IDS) + total['basin-water'][0]:.1f} kg. Nothing is measured.", f"Katalogtypischer Sand-Wasser-Spieltisch: Edelstahlrinne 1.4301 1,55 x 0,49 x 0,14 m aus 1,5-mm-Blech auf 0,68 m Randhöhe, zwei 3-mm-Schieber, Douglasienzargen C24 40 x 240 mm, Robinienpfosten 120 mm, 0,5 m tief in Punktfundamenten C20/25 0,30 x 0,60 m, ein Schneckenrohr aus Polycarbonat 160 x 5 mm, 0,88 m lang unter 45 Grad mit Edelstahlwelle 42,4 x 2 und 2-mm-Wendel mit 160 mm Steigung, ein unteres Becken 0,42 x 0,42 x 0,19 m auf einem Fertigteilsockel. Massen aus Querschnitt x Dichte: Rahmen {de(total['frame'][0], 1)} kg, je Pfosten mit Fundament {de(total['post-rl'][0], 1)} kg, Schneckenrohr {de(total['screw-mount'][0], 2)} kg, Schnecke {de(total['screw'][0], 2)} kg, Wasser {de(sum(total[p][0] for p in POOL_IDS) + total['basin-water'][0], 1)} kg. Nichts ist gemessen.")},
        "dimensions": {"heightM": round(grip_top() - FOOT_Y0, 4), "widthM": round(x_max - x_min, 4), "depthM": round(2 * (POST_Z + FOOT_R), 4), "massKg": mass_total},
        "materials": materials(),
        "rig": rig(total),
        "volumes": vols,
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(total),
        "clips": clips(),
        "thermal": {
            "minimumAirPassageM": 0.005,
            "environment": {"airTempC": 28, "meanRadiantTempC": 34, "skyTempC": 14, "groundTempC": 38, "relativeHumidity": 0.4, "windMS": 1.0, "windDirectionDeg": 180, "solarWM2": 850, "diffuseFraction": 0.15, "pressurePa": 101325},
            "passengers": [], "operating": [], "emitters": [],
            "regions": regions(area),
            "assumptions": [
                tx("Passive surfaces on a summer afternoon in a sand play area: absorbed sun, long-wave exchange with sky and hot sand, convection, evaporation from the water and the wet sand, and the heat capacity of each layer; the play table draws no power (0 W) and carries no rating.", "Passive Flächen an einem Sommernachmittag im Sandspielbereich: absorbierte Sonne, Langwellenaustausch mit Himmel und heißem Sand, Konvektion, Verdunstung von Wasser und nassem Sand und die Wärmekapazität jeder Schicht; der Spieltisch nimmt keine Leistung auf (0 W) und trägt keine Nennleistung."),
                tx("Weathered surfaces differ from new ones: greyed Douglas absorbs about 0.70 of the sun (fresh, planed 0.55), stainless steel with a limescale film and scratches emits about 0.35 (brushed new about 0.15), the clouded polycarbonate tube absorbs 0.20 instead of about 0.10 when clear (Duffie & Beckman, Incropera; tube and grip authored estimates).", "Verwitterte Flächen unterscheiden sich von neuen: vergraute Douglasie absorbiert etwa 0,70 der Sonne (frisch gehobelt 0,55), Edelstahl mit Kalkschleier und Kratzern strahlt etwa 0,35 (gebürstet neu etwa 0,15), das getrübte Polycarbonatrohr absorbiert 0,20 statt etwa 0,10 im klaren Zustand (Duffie & Beckman, Incropera; Rohr und Griff geschätzt)."),
                tx("The thermal story is the sun-heated stainless rim against the evaporatively cooled water and wet sand: the flange and the dry floor reach the high forties in the July sun while the shallow pools stay near air temperature. Fractions are scenario data.", "Die thermische Geschichte ist der sonnenheiße Edelstahlrand gegen das verdunstungsgekühlte Wasser und den nassen Sand: Flansch und trockener Boden erreichen in der Julisonne gut 45 °C, während die flachen Becken nahe der Lufttemperatur bleiben. Die Anteile sind Szenariodaten."),
            ],
        },
        "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": ["footing-" + b for b in POSTS] + ["plinth-base"], "assumptions": [
            tx("Each Robinia post is clamped in its point footing and the soil holds the footing: the restraint is one fixed support at each footing base; the basin plinth bears on the sand as a contact support. The frame is one rigid body, so the solver shares the loads over the five supports by equal stiffness.", "Jeder Robinienpfosten ist im Punktfundament eingespannt und das Erdreich hält das Fundament: die Einspannung ist ein festes Auflager je Fundamentsohle; der Beckensockel liegt als Kontaktauflager auf dem Sand. Der Rahmen ist ein starrer Körper, daher verteilt der Löser die Lasten nach gleicher Steifigkeit auf die fünf Auflager."),
            tx("The screw tube hangs on two pivot pins in the bracket plates and stands in its foot bearing on the basin floor; swung out for maintenance its lower end is held by the worker, an upward load from the moment balance about the pins.", "Das Schneckenrohr hängt an zwei Gelenkbolzen in den Halterplatten und steht im Fußlager auf dem Beckenboden; zur Wartung ausgeschwenkt hält der Mitarbeiter das untere Ende, eine aufwärts gerichtete Last aus dem Momentengleichgewicht um die Bolzen."),
            tx("User loads are authored estimates after DIN EN 1176-1 for children of 3 to 6 years (DIN 33402-2: 14 to 23 kg): 0.2 kN per leaning child, 0.08 kN on the crank; snow after DIN EN 1991-1-3 zone 2 (0.8 x 0.85 kN/m2) in the drained winter pose; water and wet sand from DIN EN 1991-1-1 densities.", "Nutzerlasten sind Schätzungen nach DIN EN 1176-1 für Kinder von 3 bis 6 Jahren (DIN 33402-2: 14 bis 23 kg): 0,2 kN je anlehnendem Kind, 0,08 kN an der Kurbel; Schnee nach DIN EN 1991-1-3 Zone 2 (0,8 x 0,85 kN/m²) in der entleerten Winterpose; Wasser und nasser Sand nach den Wichten von DIN EN 1991-1-1."),
        ], "clinical": False},
        "presentation": {"defaultPose": "in-service", "defaultClip": "water-play", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    out = HERE / "🔣️.json"
    out.write_text(json.dumps(document(), ensure_ascii=False, indent=1) + "\n")
    print("[DEBUG] wrote", out)


if __name__ == "__main__":
    # the silhouette repair of batch SO, the last step: ops/used-water-play-table.json of the umbrella ticket of 2026-11-22
    import sys as _sys
    from pathlib import Path as _Path
    _sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/🧰️silhouette/SO")
    from lib import repair_file as _repair_file
    _repair_file(_Path(__file__).resolve().parent / "🔣️.json", "used-water-play-table")
