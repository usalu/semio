"""🔥 Round 5, appliances/electric-fan-heater: louvres seated inside the outlet cassette at 1:50 and flush on the plate at 1:100 (no slat tips standing out of the frame), smooth 22 mm housing radii at 1:100 and 1:200, and a new crisp 1:500 massing. Rewrites `volumes/appliances/electric-fan-heater.json` (all four scales) from the round-3 definitions plus the overrides stated here.

Usage: python3 r5-electric-fan-heater-scales.py
"""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_src = open(os.path.join(HERE, "r3-electric-fan-heater-model.py"), encoding="utf-8").read()
M = {"__file__": os.path.join(HERE, "r3-electric-fan-heater-model.py"), "__name__": "r3model"}
exec(compile(_src[: _src.rindex("\nmain()")], "r3-electric-fan-heater-model.py", "exec"), M)
K = M["K"]
ABS, STEEL, PP, RUBBER, ZINC, PC = M["ABS"], M["STEEL"], M["PP"], M["RUBBER"], M["ZINC"], M["PC"]
W, YB, YT, YS, R = M["W"], M["YB"], M["YT"], M["YS"], M["R"]
th, st, ref = M["th"], M["st"], M["ref"]
SLAT_IN = (0.0830, 0.0050)
SLAT_OUT = (0.0932, -0.0050)


def louvres50():
    """🪟 Eight 1.5 mm slats sloping down and out, seated between the rails of the cassette (front edge 0.8 mm behind its rim face)."""
    y0, y1, half = K.OUTLET
    bar = K.CASSETTE_BAR
    chord = math.hypot(SLAT_OUT[0] - SLAT_IN[0], SLAT_IN[1] - SLAT_OUT[1])
    angle = math.degrees(math.atan2(SLAT_IN[1] - SLAT_OUT[1], SLAT_OUT[0] - SLAT_IN[0]))
    mid = ((SLAT_IN[0] + SLAT_OUT[0]) / 2, K.SLAT_Y0, 0.0)
    return K.box("louvres", STEEL, "detail", M["T_SLAT"], (chord, K.SLAT_T, 2 * (half - bar) + 0.002), mid, rotation=K.quat_axis(2, -angle), repeat={"count": K.SLAT_COUNT, "translation": [0, K.SLAT_PITCH, 0]}, texture=True)


def warm_air(x_face, thickness):
    """💨 The outlet patch of both warm-air emitters, a disc of the emitter diameter in the cassette opening, flush with its face."""
    return th(K.revolve("th-warm-air", STEEL, "detail", 0.6, [(0, 0), (0, 0.05), (thickness, 0.05), (thickness, 0)], (1, 0, 0), (x_face - thickness, K.AXIS_Y, 0), tags=["warm-air-1", "warm-air-2"]), "grille", "bars")


def body50():
    """🔎 1:50 body: the round-3 mouldings and internals with the louvres seated in the cassette."""
    return [louvres50() if s["id"] == "louvres" else s for s in M["body50"]()]


def thermal50():
    """🌡️ 1:50 thermal: round 3 with the outlet patch flush in the cassette face."""
    return [warm_air(K.CASSETTE_X1, 0.0015) if s["id"] == "th-warm-air" else s for s in M["thermal50"]()]


def body100():
    """🔎 1:100 body: shell and cover masses with five-step 22 mm radii, chamfered caps, the handle with its opening, the outlet plate with four flush slat bars, intake plate, feet and dial."""
    x0, x1 = K.REAR_LIP_X0 + M["CAP100"], K.FRONT_LIP_X1 - M["CAP100"]
    out = [M["feet100"]("foot-back", -K.FOOT_X), M["feet100"]("foot-front", K.FOOT_X)]
    out.append(K.extrude("shell-block", ABS, "body", M["T_CASE"], M["section_low"](5), "yz", x0, x1))
    out.append(K.extrude("cover-block", ABS, "body", M["T_COVER"], M["section_high"](5), "yz", x0, x1))
    face = K.rounded_rect(steps=5, **M["FACE"])
    out.append(K.extrude("front-cap", ABS, "body", M["T_CASE"], face, "yz", x1, K.FRONT_LIP_X1, taper=[1, K.FRONT_LIP_SCALE]))
    out.append(K.extrude("rear-cap", ABS, "body", M["T_CASE"], face, "yz", K.REAR_LIP_X0, x0, taper=[K.REAR_LIP_SCALE, 1]))
    out.append(K.lateral_loft("handle", ABS, "body", 0.75, K.handle_path(bar=0.0075, post=0.0085, bend_steps=2, foot=False, root_y=0.264), n=3.0))
    y0, y1, half = K.OUTLET
    plate = M["PLATE100"]
    out.append(K.box("grille-plate", STEEL, "body", M["T_GRILLE"], (0.015, y1 - y0, 2 * half), (plate - 0.0075, (y0 + y1) / 2, 0), corner=0.004))
    count, pitch, first = M["SLATS100"]
    out.append(K.box("louvres", STEEL, "detail", M["T_SLAT"], (0.003, 0.005, 2 * half - 0.012), (plate + 0.0015, first, 0), repeat={"count": count, "translation": [0, pitch, 0]}, texture=True))
    py0, py1, phalf = K.INTAKE_PANEL
    out.append(K.box("intake-plate", PC, "body", M["T_INTAKE"], (0.015, py1 - py0, 2 * phalf), (K.REAR_X + 0.0075, (py0 + py1) / 2, 0), corner=0.005))
    out.append(K.revolve("dial", PP, "body", M["T_DARK"], [(0, 0), (0, 0.020), (0.002, K.KNOB_R), (0.015, K.KNOB_R), (0.015, 0)], (0, 0, 1), (0, 0, -K.KNOB_OUT), bone="knob"))
    return out


def body200():
    """🔎 1:200 body: the round-3 block with four-step 22 mm radii instead of three."""
    out = M["body200"]()
    for s in out:
        if s["id"] == "block":
            s["profile"] = K.pts(K.rounded_rect(steps=4, **M["FACE"]))
    return out


def body500():
    """🔎 1:500 body: one block with two-step radii, a flush outlet panel, a handle bar block and a thin plinth for the feet."""
    y0, y1, half = K.OUTLET
    return [
        K.box("base", RUBBER, "body", M["T_RUBBER"], (0.146, YB, 0.18), (0, YB / 2, 0)),
        K.extrude("block", ABS, "body", M["T_CASE"], K.rounded_rect(steps=2, **M["FACE"]), "yz", K.REAR_X, K.FRONT_LIP_X1),
        K.box("grille", STEEL, "body", M["T_GRILLE"], (K.FRONT_X - K.FRONT_LIP_X1, y1 - y0, 2 * half), ((K.FRONT_X + K.FRONT_LIP_X1) / 2, (y0 + y1) / 2, 0)),
        K.extrude("handle-block", ABS, "body", M["T_CASE"], [(YT - 0.004, -0.066), (YT - 0.004, 0.066), (0.31, 0.066), (0.31, -0.066)], "yz", -0.013, 0.013),
    ]


def thermal500():
    """🌡️ 1:500 thermal: the block split at the cover seam, outlet panel with the warm-air patch, intake and dial patches, feet plinth, handle."""
    y0, y1, half = K.OUTLET
    py0, py1, phalf = K.INTAKE_PANEL
    return [
        th(K.extrude("th-casing", ABS, "body", M["T_CASE"], M["section_low"](2), "yz", K.REAR_X, K.FRONT_LIP_X1, sectionFaces=["+v"]), "casing"),
        th(ref("th-handle", "handle-block"), "casing"),
        th(K.extrude("th-top-cover", ABS, "body", M["T_COVER"], M["section_high"](2), "yz", K.REAR_X, K.FRONT_LIP_X1, sectionFaces=["-v"]), "top-cover"),
        th(ref("th-grille", "grille", sectionFaces=["-u"]), "grille", "bars"),
        warm_air(K.FRONT_X, 0.003),
        th(K.box("th-intake", PC, "body", M["T_INTAKE"], (0.002, py1 - py0, 2 * phalf), (K.REAR_X - 0.001, (py0 + py1) / 2, 0), sectionFaces=["+u"]), "intake", "inlet"),
        th(K.revolve("th-control", PP, "body", M["T_DARK"], [(0, 0), (0, K.KNOB_R), (0.004, K.KNOB_R), (0.004, 0)], (0, 0, 1), (0, 0, -W - 0.004), bone="knob"), "control", "dial"),
        th(ref("th-feet", "base", sectionFaces=["+v"]), "feet", "foot"),
    ]


def structure500():
    """🏗️ 1:500 structure: the U-shell monocoque with the cover band, the handle, the dial spindle, the motor mass and one rail per foot pair."""
    return [
        st(K.extrude("member-shell", ABS, "body", M["T_CASE"], K.u_profile(YB, YS, W, R, 0.004, 1), "yz", K.REAR_X + 0.006, K.FRONT_LIP_X1 - 0.006), "member"),
        st(K.extrude("member-top-band", ABS, "body", M["T_COVER"], K.u_profile(YS, YT, W, R, 0.004, 1, cap=True), "yz", -0.03, 0.03), "member"),
        st(ref("member-handle", "handle-block"), "member"),
        st(K.revolve("member-knob", ZINC, "detail", 0.25, [(0, 0), (0, 0.008), (0.02, 0.008), (0.02, 0)], (0, 0, 1), (0, 0, -W), bone="knob"), "member", "knob"),
        st(K.box("mass-motor", ZINC, "detail", 0.15, (0.024, 0.046, 0.054), (-0.05, K.AXIS_Y, 0)), "mass"),
        st(K.box("support-foot-back", RUBBER, "detail", 0.25, (0.025, YB, 0.175), (-K.FOOT_X, YB / 2, 0)), "support", anchor="foot-back"),
        st(K.box("support-foot-front", RUBBER, "detail", 0.25, (0.025, YB, 0.175), (K.FOOT_X, YB / 2, 0)), "support", anchor="foot-front"),
    ]


def t(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def decisions(texts):
    """📝 The round-3 decisions with the louvre sentences corrected, and the new 1:500 decisions."""
    d50 = [dict(x) for x in texts["50"]]
    d50[1]["en"] = d50[1]["en"].replace("eight 1.5 mm louvre slats that slope down and outwards and stand 4 mm proud of its rim", "eight 1.5 mm louvre slats that slope down and outwards between its rails, their front edges just behind the rim face")
    d50[1]["de"] = d50[1]["de"].replace("acht 1,5 mm dicken Lamellen, die nach außen abfallen und 4 mm über ihren Rand vorstehen", "acht 1,5 mm dicken Lamellen, die zwischen ihren Holmen nach außen abfallen, die Vorderkanten knapp hinter der Randfläche")
    d100 = [dict(x) for x in texts["100"]]
    d100[0]["en"] = d100[0]["en"].replace("a dark plate with four slats instead of eight", "a dark plate with four flush slat bars instead of eight sloping slats")
    d100[0]["de"] = d100[0]["de"].replace("eine dunkle Platte mit vier statt acht Lamellen", "eine dunkle Platte mit vier bündigen Lamellenstegen statt acht schrägen Lamellen")
    d500 = [
        t("Own 1:500 model, a crisp massing of the box heater: the housing as one block 0.196 x 0.228 x 0.256 m with its 22 mm edges in two steps, the outlet as a dark panel 8 mm proud on the front, the carry handle as one bar block across the cover, and a thin plinth for the four feet; intake, dial and seams are merged into the block.",
          "Eigenes 1:500-Modell, eine klare Massenform des Heizlüfters: das Gehäuse als ein Block von 0,196 x 0,228 x 0,256 m mit seinen 22-mm-Kanten in zwei Stufen, der Auslass als dunkle, 8 mm vorstehende Platte vorne, der Tragegriff als ein Stegblock quer über der Haube und ein flacher Sockel für die vier Füße; Einlass, Regler und Fugen gehen im Block auf."),
        t("Thermal divides the block at the cover seam and adds the outlet with its warm-air patch, the intake and dial as thin patches and the plinth for the feet, so every region keeps a solid; structure is the U-shell with the cover band under the handle, the handle, the dial spindle, the motor mass and one support rail per foot pair.",
          "Die Thermik teilt den Block an der Haubenfuge und ergänzt den Auslass mit seiner Warmluftfläche, Einlass und Regler als dünne Flecken und den Sockel für die Füße, sodass jede Region einen Körper behält; das Tragwerk ist die U-Schale mit dem Haubenband unter dem Griff, der Griff, die Reglerspindel, die Motormasse und je eine Auflagerschiene pro Fußpaar."),
    ]
    return {"50": d50, "100": d100, "200": texts["200"], "500": d500}


def main():
    """💾 Write all four scales."""
    path = os.path.join(HERE, "volumes", "appliances", "electric-fan-heater.json")
    doc = json.load(open(path, encoding="utf-8"))
    texts = json.load(open(os.path.join(HERE, "r3-electric-fan-heater-texts.json"), encoding="utf-8"))["volumes"]
    d = decisions(texts)
    built = {"50": (body50, thermal50, M["structure50"]), "100": (body100, M["thermal100"], M["structure100"]), "200": (body200, M["thermal200"], M["structure200"]), "500": (body500, thermal500, structure500)}
    for key, (b, th_, s) in built.items():
        doc["volumes"][key] = {"status": "authored", "denominator": int(key), "body": b(), "thermal": th_(), "structure": s(), "decisions": d[key]}
    with open(path, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
        f.write("\n")
    print({k: {l: len(v[l]) for l in ("body", "thermal", "structure")} for k, v in doc["volumes"].items()})


main()
