"""❄️ Round 5, appliances/split-ac-indoor-unit: the 1:50 thermal grille as the real open lattice (the closed flap's inner face is reached by the room air through it, C31), and the 1:500 model authored anew as a crisp massing of the high-wall unit (the true D profile with its 115 mm lower radius and 35 mm top edge in 15-degree steps, the closed curved flap on the radius); rewrites `volumes.500` and the 1:50 `th-grille` of `volumes/appliances/split-ac-indoor-unit.json`.

Usage: python3 r5-split-ac-indoor-unit-500.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("vol", os.path.join(HERE, "r3-split-ac-indoor-unit-volumes.py"))
V = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(V)
g = V.g
COIL_500 = [(0.19, 0.1), (0.19, 0.225), (0.11, 0.276), (0.03, 0.2), (0.03, 0.16), (0.11, 0.232), (0.158, 0.2), (0.158, 0.1)]


SEAT = 0.009


def seat_profile():
    """🅳 The 1:500 side profile: the true D (flat back and top, 35 mm top edge, 115 mm lower radius) in 15-degree steps, with the shallow seat the closed flap lies in, so its outer face runs flush with the radius."""
    pts = [(0.0, 0.0)] + g.arc(g.C[0], g.C[1], g.R, -90, g.LIP_REAR_DEG, 15)
    pts += g.arc(g.C[0], g.C[1], g.R - SEAT, g.LIP_REAR_DEG, g.LIP_FRONT_DEG, 15)
    pts += g.arc(g.C[0], g.C[1], g.R, g.LIP_FRONT_DEG, 0, 15)
    pts += [(g.D, g.H - g.CORNER)] + g.arc(g.D - g.CORNER, g.H - g.CORNER, g.CORNER, 0, 90, 30)[1:] + [(0.0, g.H)]
    return [g.pt(p) for p in g.dedupe(pts)]


def scale500():
    """🗺️ 1:500: one D-profile casing over the full width with the discharge slot, the closed curved flap, the grille plate, the hidden coil and fan, the jet patch; structure: back plate, two side frames, the flap, hook, rest and the mass."""
    body = [
        V.ext("casing", seat_profile(), -g.HALF, g.HALF, tone=0.9),
        V.ext("vane", g.vane_profile(0.008, 0.005, 12), -g.FLAP, g.FLAP, bone="vane", material="abs-grey", tone=0.86),
    ]
    thermal = [
        V.ref("th-casing", "casing", region="casing", layer="shell", sectionFaces=["-u"]),
        V.th(V.box("th-grille", (g.GRILLE[0], g.H, -g.INNER), (g.GRILLE[1], g.H + V.T, g.INNER), material="abs-grey", tone=0.5), "intake-grille", "shell", ["-v"]),
        V.ref("th-vane", "vane", region="discharge-vane", layer="shell"),
        V.th(V.ext("th-coil", COIL_500, -0.36, 0.36, material="aluminium-fin", tone=0.7), "evaporator", "fins"),
        V.th(V.fan_axis("th-fan", [(-0.38, 0.0), (-0.38, 0.048), (0.33, 0.048), (0.33, 0.0)], material="san-gf", tone=0.35), "fan", "blades"),
        V.th(V.ext("th-cold-jet", g.band(g.arc(g.C[0], g.C[1], g.R - SEAT - 0.005, -78, -42, 9), 0.005), -0.38, 0.38, material="abs-grey", role="detail", tone=0.4), "casing", None, ["+u", "-u", "+v", "-v", "+w", "-w"], ["cold-jet"]),
    ]
    structure = [
        V.st(V.box("member-back", (0.0, 0.004, -0.39), (0.02, 0.284, 0.39), material="galvanised-steel", tone=0.7), "unit", "member"),
        V.st(V.ext("member-side-frame", g.scaled(g.outer_d(15), 0.93), 0.37, 0.4, tone=0.75, repeat={"count": 2, "translation": [0, 0, -0.77]}), "unit", "member"),
        V.ref("member-vane", "vane", member="vane", part="member"),
        V.st(V.pin("support-wall-hook", "unit", 0.012, 0.265, -0.06, 0.06, 0.012, material="galvanised-steel", role="ground", tone=0.3), "unit", "support", "wall-hook"),
        V.st(V.pin("support-wall-rest", "unit", 0.012, 0.02, -0.06, 0.06, 0.012, material="abs-white", role="ground", tone=0.3), "unit", "support", "wall-rest"),
        V.st(V.ball("mass-unit", "unit", V.COM, 0.02, material="abs-white", role="detail", tone=0.3), "unit", "mass"),
    ]
    decisions = [
        {"en": "Body: own 1:500 model, a crisp massing of the high-wall unit: one casing of the true D profile (flat back and top, 35 mm top edge, 115 mm lower radius, in 15-degree steps) over the full 0.85 m, and the flap as the closed curved 8 to 5 mm blade lying flush in its seat on the radius; grille bars, panel, display, side-cover rims and louvres merge into the casing.",
         "de": "Körper: eigenes 1:500-Modell, eine klare Massenform des Wandgeräts: ein Gehäuse aus dem wahren D-Profil (flache Rück- und Oberseite, 35-mm-Oberkante, unterer Radius von 115 mm, in 15-Grad-Schritten) über die vollen 0,85 m, und die Klappe als geschlossenes, gewölbtes Blatt von 8 bis 5 mm, bündig in ihrem Sitz auf dem Radius; Gitterstäbe, Blende, Anzeige, Ränder der Seitendeckel und Lamellen gehen im Gehäuse auf."},
        {"en": "Thermal: the casing is its own skin by reference (the back against the wall plate is a contact face), the grille a plate on top, the flap by reference; inside lie the folded coil and the fan drum as hidden solids, and the cold jet leaves through an outlet patch behind the flap seat.",
         "de": "Thermik: das Gehäuse ist als Verweis seine eigene Haut (die Rückseite an der Wandplatte ist eine Anlagefläche), das Gitter eine Platte oben, die Klappe als Verweis; innen liegen der gefaltete Verdampfer und die Lüftertrommel als verborgene Körper, und der kalte Strahl tritt durch eine Auslassfläche hinter dem Klappensitz aus."},
        {"en": "Structure: the wall plate and chassis as one 20 mm back member, two side frames of the D profile, the flap as its own member (its pins merge into the side frames), the hook pin and wall roller and the mass of the unit at its centre.",
         "de": "Tragwerk: Wandplatte und Chassis als ein Rückenstab von 20 mm, zwei Seitenrahmen aus dem D-Profil, die Klappe als eigener Stab (ihre Zapfen gehen in den Seitenrahmen auf), Hakenlager und Wandrolle und die Masse des Geräts in ihrem Schwerpunkt."},
    ]
    return {"status": "authored", "denominator": 500, "body": body, "thermal": thermal, "structure": structure, "decisions": decisions}


def slotted_grille():
    """🧺 1:50 thermal intake grille as the real lattice: the 2 mm plate between the five bars and nine ribs is open (four rows of eight 13.5 x 82.5 mm openings), so the room air reaches the coil, fan and the inner face of the closed flap (C31); both faces count."""
    bars = [round(0.0405 + 0.021 * k, 6) for k in range(5)]
    gaps_x = [(round(bars[k] + 0.00375, 6), round(bars[k + 1] - 0.00375, 6)) for k in range(4)]
    ribs = [round(-0.36 + 0.09 * k, 6) for k in range(9)]
    gaps_z = [(round(ribs[k] + 0.00375, 6), round(ribs[k + 1] - 0.00375, 6)) for k in range(8)]
    holes = [[[z0, x0], [z0, x1], [z1, x1], [z1, x0]] for x0, x1 in gaps_x for z0, z1 in gaps_z]
    s = V.solid("th-grille", "extrude", material="abs-grey", tone=0.5, profile=[[-g.INNER, g.GRILLE[0]], [g.INNER, g.GRILLE[0]], [g.INNER, g.GRILLE[1]], [-g.INNER, g.GRILLE[1]]], holes=holes, plane="zx", **{"from": 0.2865, "to": 0.2885})
    return V.th(s, "intake-grille", "shell")


def main():
    """💾 Replace `volumes.500` and the 1:50 thermal grille in the source."""
    doc = json.load(open(V.SOURCE, encoding="utf-8"))
    doc["volumes"]["500"] = scale500()
    doc["volumes"]["50"]["thermal"] = [slotted_grille() if s["id"] == "th-grille" else s for s in doc["volumes"]["50"]["thermal"]]
    json.dump(doc, open(V.SOURCE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    print("written 1:500", V.SOURCE)


main()
